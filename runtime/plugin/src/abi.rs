//! Canonical ff_plugin_* C-ABI — frozen at ABI_VERSION = 1.
//!
//! Every cdylib plugin MUST export these four symbols in this exact order:
//!
//! ```text
//! ff_plugin_info()     → *const FfPluginInfo  (called first; validates ABI)
//! ff_plugin_init()     → i32                  (called once after load; 0 = ok)
//! ff_plugin_tick()     → i32                  (called every tick; 0 = ok)
//! ff_plugin_shutdown() → i32                  (called once before unload)
//! ```
//!
//! Use [`export_plugin!`] to generate all four symbols from a single macro call.
//! See `docs/PLUGIN_ABI.md` for the full specification.

/// ABI version — must match the host's `PLUGIN_ABI_VERSION`.
/// Increment when any symbol signature changes (breaking change).
pub const ABI_VERSION: u32 = types::plugin::PLUGIN_ABI_VERSION;

/// Host context passed to `ff_plugin_init`.
#[repr(C)]
pub struct FfPluginCtx {
    /// Opaque host pointer — plugins MUST NOT dereference.
    pub host:      *const std::ffi::c_void,
    /// `WorldTick.0` at the moment the plugin was loaded.
    pub load_tick: u64,
}

/// Static metadata returned by `ff_plugin_info`.
#[repr(C)]
pub struct FfPluginInfo {
    /// Null-terminated plugin ID string.
    pub id:          *const std::ffi::c_char,
    /// Null-terminated SemVer version string.
    pub version:     *const std::ffi::c_char,
    /// Null-terminated human-readable name.
    pub name:        *const std::ffi::c_char,
    /// Must equal [`ABI_VERSION`]; host refuses to load otherwise.
    pub abi_version: u32,
}

// SAFETY: static string pointers are read-only; the host never writes through them.
unsafe impl Send for FfPluginInfo {}
unsafe impl Sync for FfPluginInfo {}

// ── Function-pointer type aliases (for host-side dlsym) ───────────────────────
pub type InfoFn     = unsafe extern "C" fn() -> *const FfPluginInfo;
pub type InitFn     = unsafe extern "C" fn(*const FfPluginCtx) -> i32;
pub type TickFn     = unsafe extern "C" fn(u64) -> i32;
pub type ShutdownFn = unsafe extern "C" fn() -> i32;

// ── export_plugin! ─────────────────────────────────────────────────────────────

/// Generate the four canonical `ff_plugin_*` symbols from a cdylib crate.
///
/// # Parameters
/// - `id`       — plugin ID literal  (e.g. `"forgefabrik.my-plugin"`)
/// - `version`  — SemVer literal     (e.g. `"1.0.0"`)
/// - `name`     — display name literal
/// - `init`     — `unsafe fn(*const FfPluginCtx) -> i32`
/// - `tick`     — `unsafe fn(u64) -> i32`
/// - `shutdown` — `unsafe fn() -> i32`
///
/// # Example
/// ```rust,ignore
/// use plugin::{abi::FfPluginCtx, export_plugin};
///
/// unsafe fn init(_: *const FfPluginCtx) -> i32 { 0 }
/// unsafe fn tick(_: u64)               -> i32 { 0 }
/// unsafe fn shutdown()                 -> i32 { 0 }
///
/// export_plugin!(
///     id: "forgefabrik.my-plugin", version: "1.0.0", name: "My Plugin",
///     init: init, tick: tick, shutdown: shutdown,
/// );
/// ```
#[macro_export]
macro_rules! export_plugin {
    (
        id:       $id:literal,
        version:  $v:literal,
        name:     $n:literal,
        init:     $init:ident,
        tick:     $tick:ident,
        shutdown: $sd:ident $(,)?
    ) => {
        static _FF_INFO: $crate::abi::FfPluginInfo = $crate::abi::FfPluginInfo {
            id:          concat!($id, "\0").as_ptr() as *const ::std::ffi::c_char,
            version:     concat!($v,  "\0").as_ptr() as *const ::std::ffi::c_char,
            name:        concat!($n,  "\0").as_ptr() as *const ::std::ffi::c_char,
            abi_version: $crate::abi::ABI_VERSION,
        };

        #[no_mangle]
        pub extern "C" fn ff_plugin_info() -> *const $crate::abi::FfPluginInfo {
            &_FF_INFO
        }

        #[no_mangle]
        pub extern "C" fn ff_plugin_init(ctx: *const $crate::abi::FfPluginCtx) -> i32 {
            // SAFETY: host guarantees ctx is valid for the call duration.
            unsafe { $init(ctx) }
        }

        #[no_mangle]
        pub extern "C" fn ff_plugin_tick(tick: u64) -> i32 {
            // SAFETY: host guarantees sequential, non-concurrent tick calls.
            unsafe { $tick(tick) }
        }

        #[no_mangle]
        pub extern "C" fn ff_plugin_shutdown() -> i32 {
            // SAFETY: no further host calls after this returns.
            unsafe { $sd() }
        }
    };
}

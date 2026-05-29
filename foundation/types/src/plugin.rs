//! Canonical plugin types — identical ABI required in all repos.
//!
//! ## Plugin vs Driver rule (frozen)
//! - `plugins/`         = domain-behaviour extensions  (pure, no I/O)
//! - `runtime/drivers/` = infrastructure I/O adapters  (HTTP, storage, …)
//!
//! These are **not interchangeable**.

use serde::{Deserialize, Serialize};

/// ABI version — increment when C-symbol signatures change.
/// All repos must reject plugins compiled against a different ABI_VERSION.
pub const PLUGIN_ABI_VERSION: u32 = 1;

/// Capability token declared in `Plugin.toml`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PluginCapability(pub String);

impl std::fmt::Display for PluginCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Parsed `Plugin.toml` manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub version: String,
    pub name: String,
    pub description: String,
    /// Capabilities this plugin adds when loaded.
    pub provides: Vec<PluginCapability>,
    /// Capabilities that must already be registered before this plugin loads.
    pub requires: Vec<PluginCapability>,
    /// Relative path to the compiled `.so` / `.dylib` / `.dll`.
    pub lib: String,
}

/// Runtime record of a loaded plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeFabrikPlugin {
    pub manifest: PluginManifest,
    pub lib_path: String,
    pub abi_version: u32,
}

// ── Canonical C-ABI symbols (all cdylib plugins must export these) ────────────
//
// Symbol            Signature                                  Called when
// ─────────────────────────────────────────────────────────────────────────────
// ff_plugin_info    unsafe extern "C" fn() -> *const PluginInfo    on load
// ff_plugin_init    unsafe extern "C" fn(ctx: *const PluginCtx)    on load
//                       -> i32
// ff_plugin_tick    unsafe extern "C" fn(tick: u64) -> i32         each tick
// ff_plugin_shutdown unsafe extern "C" fn() -> i32                 on unload
//
// Return value convention:  0 = success,  non-zero = error.
//
// Repos that currently use different symbol names (e.g. plugin_info / plugin_id)
// must migrate to ff_plugin_* in their next breaking-change release.
// See docs/PLUGIN_ABI.md for the migration guide.

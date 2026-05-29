//! Canonical ForgeFabrik plugin template.
//!
//! ## Checklist
//! - [ ] `crate-type = ["cdylib"]` in `Cargo.toml`
//! - [ ] `export_plugin!` called exactly once
//! - [ ] `Plugin.toml` shipped alongside the compiled `.so`
//! - [ ] All three lifecycle functions return `0` on success
//! - [ ] `tick` completes in < 1 ms; no I/O inside `tick`

use plugin::{abi::FfPluginCtx, export_plugin};

unsafe fn init(_: *const FfPluginCtx) -> i32 { 0 /* TODO: initialise */ }
unsafe fn tick(_: u64)               -> i32 { 0 /* TODO: per-tick work */ }
unsafe fn shutdown()                 -> i32 { 0 /* TODO: cleanup */ }

export_plugin!(
    id:       "forgefabrik.example-v1",
    version:  "0.1.0",
    name:     "ForgeFabrik Example Plugin",
    init:     init,
    tick:     tick,
    shutdown: shutdown,
);

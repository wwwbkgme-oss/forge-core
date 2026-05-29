//! `plugin` — ForgeFabrik runtime plugin host.
//!
//! **Layer:** `runtime` — may perform I/O (filesystem, dlopen).
//!
//! - [`abi`]: Canonical C-ABI types, ABI version, function-pointer aliases, `export_plugin!`.
//! - [`manifest`]: `Plugin.toml` parser.

pub mod abi;
pub mod manifest;

pub use abi::{ABI_VERSION, FfPluginCtx, FfPluginInfo, InfoFn, InitFn, ShutdownFn, TickFn};
pub use manifest::parse_manifest;

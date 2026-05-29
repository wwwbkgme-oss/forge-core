# ForgeFabrik — Canonical Plugin ABI Specification

**Status:** frozen (ABI_VERSION = 1)  
**Applies to:** forge-core · ff-one · ff-two · ff-three

---

## Overview

Every `cdylib` plugin in the ForgeFabrik federation must export exactly four
C-ABI symbols. This document defines their signatures, calling conventions,
and contract.

---

## C-ABI Symbols

### `ff_plugin_info`

```c
FfPluginInfo* ff_plugin_info(void);
```

Returns a pointer to statically-allocated plugin metadata.
Called immediately after the library is loaded to validate ABI compatibility.

```rust
#[repr(C)]
pub struct FfPluginInfo {
    pub id:          *const std::ffi::c_char,  // plugin ID, null-terminated
    pub version:     *const std::ffi::c_char,  // semver string, null-terminated
    pub name:        *const std::ffi::c_char,  // human name, null-terminated
    pub abi_version: u32,                       // must equal PLUGIN_ABI_VERSION
}
```

If `abi_version` does not match the host's `PLUGIN_ABI_VERSION`, the host
must refuse to load the plugin and return `PluginLoadError`.

---

### `ff_plugin_init`

```c
int32_t ff_plugin_init(const FfPluginCtx* ctx);
```

Called once after `ff_plugin_info` succeeds.

```rust
#[repr(C)]
pub struct FfPluginCtx {
    pub host:      *const std::ffi::c_void,  // opaque host pointer
    pub load_tick: u64,                       // WorldTick at load time
}
```

Return value: `0` = success, any other value = error (plugin is unloaded).

---

### `ff_plugin_tick`

```c
int32_t ff_plugin_tick(uint64_t tick);
```

Called once per world tick.  `tick` is the current `WorldTick` value.

**Rules:**
- Must return quickly (< 1 ms recommended).
- Must not perform I/O (HTTP, filesystem, DB).
- Must not block the calling thread.
- Return value: `0` = success, non-zero = warning (host logs but continues).

---

### `ff_plugin_shutdown`

```c
int32_t ff_plugin_shutdown(void);
```

Called when the plugin is unloaded (server shutdown or explicit unload).
Return value is ignored by the host but should be `0` on clean shutdown.

---

## Plugin.toml Manifest

Every plugin must ship a `Plugin.toml` alongside its compiled `.so` file:

```toml
[plugin]
id          = "forgefabrik.my-plugin"   # reverse-domain style
version     = "0.1.0"                   # semver
name        = "My Plugin"
description = "What this plugin does."

[capabilities]
provides = ["my-capability"]            # capabilities added
requires = ["agent"]                    # must be loaded first

[entry]
lib = "libmy_plugin.so"                 # relative to Plugin.toml
```

The host validates `requires` before calling `ff_plugin_init`.

---

## Macro helper (Rust)

Use the `export_plugin!` macro (defined in `runtime/plugin` crates) to avoid
writing the boilerplate:

```rust
use plugin::{abi::FfPluginCtx, export_plugin};

fn init(_: *const FfPluginCtx) -> i32 { 0 }
fn tick(_: u64)                -> i32 { 0 }
fn shutdown()                  -> i32 { 0 }

export_plugin!(
    id:       "forgefabrik.my-plugin",
    version:  "0.1.0",
    name:     "My Plugin",
    init:     init,
    tick:     tick,
    shutdown: shutdown,
);
```

---

## Migration: ff-two divergent ABI

`ff-two` currently exports `plugin_info()` / `plugin_id()` instead of
`ff_plugin_info()`.  Migration path:

1. Add `ff_plugin_info`, `ff_plugin_init`, `ff_plugin_tick`, `ff_plugin_shutdown`
   exports alongside the existing symbols.
2. Deprecate old symbols with a `#[deprecated]` note.
3. Remove old symbols in the next breaking release (bump ABI_VERSION to 2).

---

## Plugin vs Driver boundary

```
plugins/         = domain-behaviour extensions  (pure, no I/O)
runtime/drivers/ = infrastructure I/O adapters  (HTTP, storage, …)
```

LLM provider adapters are **drivers**, not plugins.
See `foundation/types/src/plugin.rs` for the full rationale and ADR reference.

---

## ABI version history

| Version | Changes |
|---|---|
| 1 (current) | Initial canonical ABI: `ff_plugin_info/init/tick/shutdown` |

# ForgeFabrik Unified Sync Contract (v0.1)

**Status:** active  
**Applies to:** forge-core · ff-one · ff-two · ff-three

---

## Repositories

| Repo | Role |
|---|---|
| `forge-core` | Canonical definitions / shared kernel contracts (this repo) |
| `ff-one` | Runtime MMO — world engine, voxel simulation |
| `ff-two` | DevStudio — multi-agent software dev / orchestration |
| `ff-three` | Academy — event-sourced educational simulation |

---

## Goal

All repositories remain **autark** (independently runnable) but are structurally
and semantically synchronized on a common architecture.

> ForgeFabrik = a modular multi-system of compatible but independent Rust workspaces.
> No repo may import another repo to run.

---

## Global Architecture Model (mandatory)

All systems must follow this 4-layer model:

```
foundation → domain → runtime → plugins
```

| Layer | Rules |
|---|---|
| `foundation` | Pure types, events, errors. NO I/O. |
| `domain` | Deterministic logic. NO HTTP, NO DB, NO filesystem. |
| `runtime` | I/O boundary — HTTP, CLI, sandbox, drivers, plugins. |
| `plugins` | Optional dynamic `cdylib` extensions. No I/O. |

---

## Synchronisation Rules

### 1. Event-First System

- State is always a projection of events.
- No direct mutation of persistent state outside a Reducer.

### 2. Single Mutation Path

```
Command → Event → Reducer → State Projection
```

### 3. Determinism (mandatory in domain + foundation)

Forbidden:
- `Utc::now()` — use `TickContext.tick`
- Unseeded randomness (`thread_rng()`) — use `DeterministicRng`
- Global mutable state
- Thread-dependent logic

### 4. Replay-Safety

```
assert_eq!(replay(events), snapshot.state_hash)
```

---

## Canonical Names (frozen)

| Concept | Name |
|---|---|
| Agent | `Agent` |
| World state | `WorldState` |
| Event | `WorldEvent` |
| Simulation clock | `WorldTick` |
| Snapshot | `WorldSnapshot` |
| Tick context | `TickContext` |
| Plugin record | `ForgeFabrikPlugin` |
| Error | `ForgeError` |

---

## Module Compatibility

Repos may have their own implementations but must honour the same type
semantics. If conversion is needed, implement an adapter in `runtime/`.

---

## Plugin System (unified)

All repos use the canonical plugin ABI:

```
cdylib plugin
Plugin.toml manifest
ff_plugin_init(ctx)   → i32
ff_plugin_tick(tick)  → i32
ff_plugin_shutdown()  → i32
ff_plugin_info()      → *const PluginInfo
```

Plugins must:
- Have no mandatory dependency on core implementation crates.
- Be loadable optionally (missing plugin ≠ crash).
- Carry a version field.

See `docs/PLUGIN_ABI.md` for C-symbol details.

---

## Repo-Specific Roles (no overlap)

| Repo | Domain |
|---|---|
| `forge-core` | Canonical type definitions, trait contracts, ADRs |
| `ff-one` | Voxel world simulation, MMO runtime, BLAKE3 consensus |
| `ff-two` | Multi-agent software creation, deployment pipelines |
| `ff-three` | Character simulation, GOAP planner, educational RPG |

---

## Integration Rule

Each repo:
- **May** have its own implementations.
- **Must** implement compatible type names and event semantics.
- **Must NOT** import another federated repo as a Cargo dependency.
- **Must NOT** introduce incompatible event systems.

If differences exist: implement adapters in `runtime/`, not in `domain/`.

---

## Testing Requirements (mandatory)

Every repo must have:

| Test | Requirement |
|---|---|
| Deterministic replay | `replay(events) == snapshot.state_hash` |
| Event equality | Equal events produce equal state |
| Snapshot roundtrip | Serialize → deserialize → assert_eq! |

Reference implementation: `domain/kernel/tests/mandatory.rs` in this repo.
Copy or adapt for each federated repo.

---

## CQRS Contracts (`domain/kernel`)

forge-core ships the canonical CQRS trait contracts in `domain/kernel`:

| Trait | Purpose |
|---|---|
| `Reducer<S,E>` | Pure `(state, event) → state`. Deterministic. |
| `CommandHandler<S,C,E>` | Validate command, emit events. |
| `AggregateRoot` | Combines both; provides `replay()`. |
| `DeterministicHash` | Hex state hash — feeds `WorldSnapshot.state_hash`. |

Each federated repo re-implements these traits locally (no import from forge-core).

---

## Plugin ABI (`runtime/plugin`)

forge-core ships the canonical plugin runtime in `runtime/plugin`:

| Item | Description |
|---|---|
| `ABI_VERSION = 1` | Frozen — increment only on breaking change. |
| `FfPluginCtx` / `FfPluginInfo` | C-ABI structs (`#[repr(C)]`). |
| `export_plugin!` | Generates `ff_plugin_info/init/tick/shutdown`. |
| `parse_manifest()` | Parses `Plugin.toml` → `PluginManifest`. |

See `plugins/plugin-example/` for a complete cdylib reference template.

---

## Sync Strategy

- `forge-core` is the reference specification.
- Other repos adapt to `forge-core`.
- Breaking changes require versioned events and a migration guide.

---

## Final Instruction to Agents

> "Treat this repository as part of a federated ForgeFabrik system.
> Maintain compatibility with the canonical event + domain model.
> Do not introduce incompatible abstractions."

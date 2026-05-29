# AGENTS.md — ForgeFabrik Federated System

**Treat this repository as part of a federated ForgeFabrik system.**

Maintain compatibility with the canonical event + domain model defined here.
Do not introduce incompatible abstractions.

---

## What this means in practice

### 1. Type names are canonical

The following names are frozen across all repos. Use them exactly:

| Concept | Canonical name | Defined in |
|---|---|---|
| Agent | `Agent` | `foundation/types/src/agent.rs` |
| World state | `WorldState` | `foundation/types/src/world.rs` |
| World snapshot | `WorldSnapshot` | `foundation/types/src/world.rs` |
| Simulation clock | `WorldTick` | `foundation/types/src/tick.rs` |
| Tick context | `TickContext` | `foundation/types/src/tick.rs` |
| Event wrapper | `EventEnvelope<E>` | `foundation/events/src/envelope.rs` |
| Plugin record | `ForgeFabrikPlugin` | `foundation/types/src/plugin.rs` |
| Error type | `ForgeError` | `foundation/types/src/error.rs` |

### 2. Layer rules are hard constraints

```
foundation → domain → runtime → plugins
```

| Layer | Allowed | Forbidden |
|---|---|---|
| `foundation/` | types, traits, errors, events | I/O, randomness, business logic |
| `domain/` | deterministic logic, reducers | HTTP, DB, filesystem, `Utc::now()`, `thread_rng()` |
| `runtime/` | I/O, HTTP, processes, plugins | domain business logic outside trait impls |
| `plugins/` | domain-behaviour extensions | I/O, HTTP, DB |

### 3. Event-First is mandatory

```
Events are truth. State is projection.
Command → Event → Reducer → State Projection
```

- Every state mutation emits at least one `WorldEvent`.
- Reducers are pure functions: `(state, event) → state`.
- Same events + same seed → same state hash (Replay-Safety).

### 4. Determinism rules

Forbidden in `domain/` and `foundation/`:

- `chrono::Utc::now()` — use `TickContext.tick` instead
- `rand::thread_rng()` — use `DeterministicRng::new(rng_seed)` instead
- Global mutable state (`static mut`, `OnceLock` with mutable content)
- Thread-dependent behaviour

### 5. Plugin vs Driver boundary

```
plugins/         = domain-behaviour extensions  (pure, no I/O)
runtime/drivers/ = infrastructure I/O adapters  (HTTP, storage, …)
```

These are not interchangeable. See `docs/PLUGIN_ABI.md`.

### 6. No cross-repo imports

No repo in the federation may list another federated repo as a Cargo dependency.
Each repo re-implements the canonical types independently.
Adapter mapping between repos belongs in the `runtime/` layer.

---

## Reference

- `docs/SYNC_CONTRACT.md` — full federation sync contract
- `docs/PLUGIN_ABI.md` — canonical plugin ABI specification
- `foundation/types/` — canonical type definitions (this repo)
- `foundation/events/` — canonical event envelope + EventStore trait
- `foundation/contracts/` — canonical trait definitions

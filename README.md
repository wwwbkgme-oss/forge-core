# forge-core

> Canonical shared kernel for the ForgeFabrik federated system.

`forge-core` defines the canonical types, trait contracts, and event primitives
that all ForgeFabrik systems implement compatibly.

**No other repo imports this crate.** Each federated system re-implements the
canonical types with the same names and semantics.  `forge-core` is the
reference specification and the sanity-check build target.

---

## What's in here

```
foundation/
  types/       WorldTick, TickContext, WorldState, WorldSnapshot,
               Agent, AgentKind, FreeProvider, ForgeFabrikPlugin,
               DeterministicRng, ForgeError
  events/      WorldEvent, EventEnvelope<E>, EventStore<E> trait
  contracts/   AgentDriver, WorldSimulator, PluginHost, ConsensusCoordinator
docs/
  SYNC_CONTRACT.md    Federation-wide synchronisation contract
  PLUGIN_ABI.md       Canonical C-ABI specification for plugins
AGENTS.md             Instructions for AI agents working in the federation
```

---

## Federated repos

| Repo | Role |
|---|---|
| **forge-core** | This repo — canonical definitions |
| [ff-one](https://github.com/wwwbkgme-oss/ff-one) | Runtime MMO — voxel world engine |
| [ff-two](https://github.com/wwwbkgme-oss/ff-two) | DevStudio — multi-agent software creation |
| [ff-three](https://github.com/wwwbkgme-oss/ff-three) | Academy — event-sourced educational simulation |

---

## Architecture (all repos must follow)

```
foundation → domain → runtime → plugins
```

| Layer | Rules |
|---|---|
| `foundation` | Pure types, events, errors. NO I/O. |
| `domain` | Deterministic logic. NO HTTP, NO DB, NO `Utc::now()`. |
| `runtime` | I/O boundary. |
| `plugins` | Domain-behaviour extensions. No I/O. |

See [`AGENTS.md`](AGENTS.md) for the full layer rules and canonical type table.

---

## Build

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

---

## Key canonical types

### WorldTick

```rust
// Monotonic simulation counter — replaces all wall-clock usage in domain/.
let tick = WorldTick(42);
let next = tick.advance(1);
```

### DeterministicRng

```rust
// Use instead of rand::thread_rng() in domain/.
let mut rng = DeterministicRng::new(seed ^ tick.0 ^ entity_id_hash);
let roll = rng.next_bounded(100);
```

### EventEnvelope

```rust
// Every event must be wrapped before storage.
let envelope = EventEnvelope::new(tick, WorldEvent::AgentSpawned { … });
```

### FreeProvider / AgentKind

```rust
// New free providers: add to FreeProvider, not to AgentKind.
let kind = AgentKind::Free(FreeProvider::Groq);
```

---

## License

MIT OR Apache-2.0

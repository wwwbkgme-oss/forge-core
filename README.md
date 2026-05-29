# forge-core

> Canonical shared kernel for the ForgeFabrik federated system.

`forge-core` defines the canonical types, trait contracts, event primitives,
CQRS kernel, and plugin ABI that all ForgeFabrik systems implement compatibly.

**No other repo imports this crate.** Each federated system re-implements the
canonical types with the same names and semantics.  `forge-core` is the
reference specification, the sanity-check build target, and the source of
conformance test vectors.

---

## Workspace layout

```
foundation/
  types/       WorldTick, TickContext, WorldState, WorldSnapshot,
               Agent, AgentKind, FreeProvider, ForgeFabrikPlugin,
               DeterministicRng, ForgeError, CONFORMANCE_VERSION
  events/      WorldEvent, EventEnvelope<E>, EventStore<E> trait
  contracts/   AgentDriver, WorldSimulator, PluginHost, ConsensusCoordinator
domain/
  kernel/      Reducer, CommandHandler, AggregateRoot, DeterministicHash
               assert_deterministic! macro
               tests/mandatory.rs — 7 canonical CQRS conformance tests
runtime/
  plugin/      ff_plugin_* C-ABI (ABI_VERSION=1), Plugin.toml parser,
               export_plugin! macro
plugins/
  plugin-example/  Reference cdylib template + Plugin.toml
conformance/   Test-vector suite — copy vectors/ into federated repos
docs/
  SYNC_CONTRACT.md    Federation-wide synchronisation rules
  PLUGIN_ABI.md       Canonical ff_plugin_* C-ABI specification
  EVENT_ORDERING.md   9 formal event-ordering rules
AGENTS.md      Instructions for AI agents in the federation
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
| `runtime` | I/O boundary — HTTP, CLI, sandbox, drivers, plugins. |
| `plugins` | Domain-behaviour extensions. No I/O. cdylib. |

See [`AGENTS.md`](AGENTS.md) for the full layer rules and canonical type table.

---

## Build & test

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

---

## CQRS pattern (`domain/kernel`)

```rust
use kernel::{AggregateRoot, CommandContext, DeterministicHash, Reducer};

// Command → handle() → [Events] → apply() → NewState
impl AggregateRoot for MyAggregate {
    type Event = MyEvent; type Command = MyCmd; type Error = String;

    fn handle(&self, cmd: MyCmd, ctx: &CommandContext)
        -> Result<Vec<MyEvent>, String> { … }

    fn apply(state: Self, event: &MyEvent) -> Self { … }
}

impl DeterministicHash for MyAggregate {
    fn state_hash(&self) -> String { /* hex-encoded BLAKE3 */ }
}

// Canonical assertion — must hold in every repo:
assert_eq!(MyAggregate::replay(initial, &events).state_hash(), snapshot.state_hash);
```

---

## Plugin ABI (`runtime/plugin`)

```rust
use plugin::{abi::FfPluginCtx, export_plugin};

unsafe fn init(_: *const FfPluginCtx) -> i32 { 0 }
unsafe fn tick(_: u64)               -> i32 { 0 }
unsafe fn shutdown()                 -> i32 { 0 }

export_plugin!(
    id: "forgefabrik.my-plugin", version: "1.0.0", name: "My Plugin",
    init: init, tick: tick, shutdown: shutdown,
);
```

Ship a `Plugin.toml` alongside the compiled `.so`:

```toml
[plugin]
id          = "forgefabrik.my-plugin"
version     = "1.0.0"
name        = "My Plugin"
description = "What it does."

[capabilities]
provides = ["agent"]
requires = []

[entry]
lib = "libmy_plugin.so"
```

---

## Key canonical types

### WorldTick

```rust
let tick = WorldTick(42);
let next = tick.advance(1);          // WorldTick(43)
let display = tick.to_string();      // "tick(42)"
```

### DeterministicRng

```rust
// Use instead of rand::thread_rng() in domain/.
let mut rng = DeterministicRng::new(seed ^ tick.0 ^ entity_hash);
let roll = rng.next_bounded(100);
```

### WorldSnapshot

```rust
let snap = WorldSnapshot::new(tick, state_hash_hex);
// Canonical assertion:
assert_eq!(replay(events).state_hash(), snap.state_hash);
```

---

## License

MIT OR Apache-2.0

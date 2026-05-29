# Contributing to forge-core

`forge-core` is the **canonical reference spec** for the ForgeFabrik federated
system.  Changes here propagate semantics to all four repos.

---

## What lives here

- `foundation/types` — canonical primitive types (`WorldTick`, `TickContext`,
  `DeterministicRng`, `WorldSnapshot`, `ForgeFabrikPlugin`, `Agent`, …)
- `foundation/events` — `WorldEvent`, `EventEnvelope<E>`, `EventStore<E>` trait
- `foundation/contracts` — trait contracts (`AgentDriver`, `WorldSimulator`, …)
- `domain/kernel` — conformance vectors (serialisation, tick, snapshot, rng)
- `runtime/plugin` — plugin host ABI (`FfPluginCtx`, manifest loader)
- `plugins/plugin-example` — minimal reference plugin
- `conformance/` — cross-repo compatibility test vectors
- `docs/SYNC_CONTRACT.md` — federation-wide synchronisation contract (frozen)
- `docs/PLUGIN_ABI.md` — canonical C-ABI for cdylib plugins

---

## Ground rules

1. **forge-core is a reference spec, not a runtime.**
   No business logic, no database, no HTTP lives here.

2. **No other repo imports this crate.**
   Other repos copy patterns locally.  Compatibility is semantic, not
   compile-time.

3. **`docs/SYNC_CONTRACT.md` is frozen** — changes need explicit consensus
   from all four repos (open an issue tagged `sync-contract`).

4. **Breaking changes to canonical types require a new version** in
   `CHANGELOG.md` and an issue notifying all federated repos.

---

## Build

```bash
cargo build --workspace
cargo test  --workspace
cargo clippy --workspace -- -D warnings
cargo fmt   --check
```

All four commands must pass before a PR is merged.  CI enforces them.

---

## Adding a canonical type

1. Add the type in `foundation/types/src/` (or `foundation/events/src/`).
2. Re-export from `foundation/types/src/lib.rs`.
3. Add a conformance vector in `conformance/src/vectors/` (if serialisable).
4. Update `docs/SYNC_CONTRACT.md` (§3 Canonical naming table).
5. Update `README.md` (Key canonical types section).
6. Update `CHANGELOG.md`.
7. Open issues in ff-one, ff-two, ff-three to adopt the new name.

---

## Changing the plugin ABI

1. Bump `api_version` in `docs/PLUGIN_ABI.md`.
2. Update `runtime/plugin/src/abi.rs`.
3. Update `plugins/plugin-example/src/lib.rs` to match.
4. Update `CHANGELOG.md`.
5. Notify all federated repos (breaking change).

---

## Commit message format

```
type(scope): short description

Optional body.

Co-Authored-By: Name <email>
```

Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`

---

## License

MIT OR Apache-2.0

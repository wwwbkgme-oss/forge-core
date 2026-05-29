# NEXT — forge-core Roadmap

---

## v0.2 — Planned

### Conformance suite expansion
- Add conformance vector for `EventStore::append` OCC semantics
- Add cross-repo serialisation compatibility test (ff-one ↔ ff-three `WorldEvent`)
- Script to run all four repos' conformance suites in CI

### EventStore contract
- Formalise `EventStore<E>` semantics in `docs/SYNC_CONTRACT.md` §5
- Add `ExpectedVersion::Exact` OCC to the contract (currently informally described)
- Reference `PgEventStore` (ff-three) as the canonical Postgres implementation

### Plugin ABI v2
- Switch from raw `FfPluginCtx *` to versioned `FfPluginCtxV2` struct
- Add `ff_plugin_capabilities()` → `u64` bitfield for capability advertisement
- Update `plugins/plugin-example` and notify all repos

### WorldEvent unification
- Define an adapter pattern in `docs/SYNC_CONTRACT.md` for domain-specific
  events (e.g. ff-three `CharacterEvent`) wrapped in a `WorldEvent::Domain`
  variant for cross-repo event buses
- ff-three: add `WorldEvent` umbrella wrapper in `foundation/events`
- ff-two: map `TaskEvent` / `AgentEvent` into `WorldEvent` adapter

### TickContext broadcast
- Define `TickBroadcast` — a signed `TickContext` that all realm participants
  receive, enabling multi-node determinism verification

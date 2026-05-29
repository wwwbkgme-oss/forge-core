# Changelog — forge-core

All notable changes to the canonical ForgeFabrik kernel specification.

Format: [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
Versions track the `SYNC_CONTRACT.md` contract version.

---

## [0.1.0] — 2026-05

### Added
- `foundation/types` — canonical primitives:
  `WorldTick`, `TickContext`, `DeterministicRng`, `WorldSnapshot`,
  `ForgeFabrikPlugin`, `Agent`, `AgentKind`, `FreeProvider`, `ForgeError`
- `foundation/events` — `WorldEvent`, `EventEnvelope<E>`, `EventStore<E>` trait
- `foundation/contracts` — trait contracts:
  `AgentDriver`, `WorldSimulator`, `PluginHost`, `ConsensusCoordinator`
- `domain/kernel` — conformance test vectors
  (serialisation, tick ordering, snapshot roundtrip, RNG determinism)
- `runtime/plugin` — plugin host ABI:
  `FfPluginCtx` (`#[repr(C)]`), `FfPluginInfo`, manifest loader
- `plugins/plugin-example` — minimal reference cdylib plugin
- `conformance/` — cross-repo compatibility test vectors
- `docs/SYNC_CONTRACT.md` — federation-wide sync contract v0.1
- `docs/PLUGIN_ABI.md` — canonical C-ABI specification
- `docs/EVENT_ORDERING.md` — event ordering and causal consistency rules
- `AGENTS.md` — instructions for AI agents operating in this repo
- CI: `cargo check`, `cargo clippy -D warnings`, `cargo test`, `rustfmt`

### Design decisions
- `AgentKind::Free(FreeProvider)` grouping — prevents provider explosion
  as new free LLM providers are added.
- `DeterministicRng` uses BLAKE3 seeding over xorshift XOR-mixing for
  better avalanche properties across entity IDs.
- Plugin ABI uses `#[repr(C)]` + extern "C" functions, not `dyn Trait`,
  to avoid vtable ABI instability across Rust versions.

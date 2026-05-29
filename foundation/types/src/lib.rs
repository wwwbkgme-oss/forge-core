//! # forge-core — canonical foundation types
//!
//! **Layer:** `foundation` — no I/O, no randomness, no business logic.
//!
//! These types are the Single Source of Truth for the entire ForgeFabrik
//! federation.  Every federated system (ff-one, ff-two, ff-three) must
//! implement types with identical names, semantics, and serialisation
//! behaviour.  They do not import this crate — they re-implement it.
//!
//! ## Sync rule
//! Any change here that alters a type's semantics or wire format is a
//! **breaking change** and requires a versioned event migration in all repos.

pub mod agent;
pub mod error;
pub mod plugin;
pub mod rng;
pub mod tick;
pub mod world;

/// Conformance version — bump when any canonical type, serialisation format,
/// or RNG algorithm changes.  All federated repos must match this version in
/// their conformance test runs.
pub const CONFORMANCE_VERSION: u32 = 1;

// ── Flat re-exports ───────────────────────────────────────────────────────────
pub use agent::{Agent, AgentCapability, AgentKind, AgentState, FreeProvider};
pub use error::{ForgeError, ForgeResult};
pub use plugin::{ForgeFabrikPlugin, PluginCapability, PluginManifest};
pub use rng::DeterministicRng;
pub use tick::{TickContext, WorldTick, DAY_LENGTH_TICKS};
pub use world::{WorldSnapshot, WorldState};

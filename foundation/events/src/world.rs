//! Canonical `WorldEvent` — the federation-wide event taxonomy.
//!
//! Each repo extends this with domain-specific events.
//! The variants listed here MUST be present and semantically identical
//! in every federated system.

use serde::{Deserialize, Serialize};
use types::{agent::AgentState, tick::WorldTick};
use uuid::Uuid;

/// All events that can occur in a ForgeFabrik world.
///
/// **Sync rule:** adding a variant is non-breaking.
/// Renaming or removing a variant is breaking and requires a versioned migration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorldEvent {
    // ── Agent lifecycle (mandatory in all repos) ──────────────────────────
    AgentSpawned {
        agent_id: Uuid,
        name: String,
        tick: WorldTick,
    },
    AgentStateChanged {
        agent_id: Uuid,
        new_state: AgentState,
        tick: WorldTick,
    },
    AgentDied {
        agent_id: Uuid,
        reason: String,
        tick: WorldTick,
    },

    // ── Simulation ────────────────────────────────────────────────────────
    TickAdvanced {
        tick: WorldTick,
    },
    EpochStarted {
        epoch: u64,
        seed: u64,
    },
    EpochEnded {
        epoch: u64,
        dominant_faction: Option<String>,
    },

    // ── Consensus ─────────────────────────────────────────────────────────
    ConsensusRoundStarted {
        tick: WorldTick,
    },
    ConsensusRoundFinalised {
        tick: WorldTick,
        state_hash: String,
    },

    // ── Repo-specific (opaque payload, not interpreted cross-repo) ────────
    /// Domain-specific event from a federated system.
    /// Use this variant to carry events that have no canonical equivalent.
    DomainEvent {
        source_repo: String,
        event_type: String,
        payload: serde_json::Value,
    },
}

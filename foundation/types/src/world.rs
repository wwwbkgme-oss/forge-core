//! Canonical world state and snapshot types.

use crate::tick::WorldTick;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Canonical world snapshot — replay-safe hash of state at a given tick.
///
/// All repos must implement this type with identical serialisation.
/// The hash algorithm is BLAKE3 over sorted, deterministic state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub id: Uuid,
    pub tick: WorldTick,
    /// BLAKE3 hex digest of deterministic world state.
    pub state_hash: String,
    /// Snapshot metadata (engine-specific, not used for equality).
    pub label: Option<String>,
}

impl WorldSnapshot {
    pub fn new(tick: WorldTick, state_hash: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            tick,
            state_hash: state_hash.into(),
            label: None,
        }
    }

    /// Snapshots are equal when they share the same tick and hash.
    /// `id` and `label` are metadata and do not affect equality.
    pub fn semantically_eq(&self, other: &Self) -> bool {
        self.tick == other.tick && self.state_hash == other.state_hash
    }
}

/// Minimal world state header — each repo extends this with domain fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub tick: WorldTick,
    pub seed: u64,
    pub state_hash: String,
}

impl WorldState {
    pub fn new(seed: u64) -> Self {
        Self {
            tick: WorldTick::ZERO,
            seed,
            state_hash: String::new(),
        }
    }
}

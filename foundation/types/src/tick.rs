//! Deterministic simulation time — the only time primitive allowed in domain/.
//!
//! **Rule:** `chrono::Utc::now()` is FORBIDDEN in `domain/` and `foundation/`.
//! All non-determinism enters exclusively through [`TickContext`].

use serde::{Deserialize, Serialize};

/// Monotonically increasing simulation counter.
/// One tick = one logical step; wall-clock duration is runtime-defined.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct WorldTick(pub u64);

impl WorldTick {
    pub const ZERO: Self = Self(0);

    pub fn advance(self, delta: u64) -> Self {
        Self(self.0 + delta)
    }
    pub fn elapsed_since(self, earlier: Self) -> u64 {
        self.0.saturating_sub(earlier.0)
    }
}

impl std::fmt::Display for WorldTick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "tick({})", self.0)
    }
}

/// Number of ticks in one in-world day (runtime sets actual wall-clock rate).
pub const DAY_LENGTH_TICKS: u64 = 2400;

/// Passed to every domain tick function — the only source of non-determinism.
///
/// **Rule:** domain code must not call `Utc::now()`, `thread_rng()`, or any
/// other non-deterministic primitive.  Use `rng_seed` and [`crate::DeterministicRng`].
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TickContext {
    /// Absolute simulation counter.
    pub tick: WorldTick,
    /// Deterministic RNG seed for this tick: `hash(global_seed, tick, entity_id)`.
    pub rng_seed: u64,
    /// Normally 1; larger during catch-up replay.
    pub delta_ticks: u64,
}

impl TickContext {
    pub fn new(tick: WorldTick, rng_seed: u64) -> Self {
        Self {
            tick,
            rng_seed,
            delta_ticks: 1,
        }
    }
}

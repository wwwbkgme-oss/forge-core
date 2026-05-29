//! Deterministic PRNG — the canonical way to get randomness inside domain/.
//!
//! Seeded from `(global_seed, tick, entity_id)` so every run with the same
//! inputs produces the same outputs.  **Never** call `rand::thread_rng()` in
//! `domain/` or `foundation/`.

/// Xorshift64* PRNG — fast, deterministic, no `std`/`rand` dependency needed.
pub struct DeterministicRng(u64);

impl DeterministicRng {
    /// Seed: `hash(global_seed XOR tick XOR entity_id)` recommended.
    pub fn new(seed: u64) -> Self {
        // Ensure non-zero state.
        Self(if seed == 0 {
            0xcafe_babe_dead_beef
        } else {
            seed
        })
    }

    /// Next pseudo-random `u64`.
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x.wrapping_mul(0x9e37_79b9_7f4a_7c15)
    }

    /// Uniformly distributed `u64` in `[0, bound)`.
    pub fn next_bounded(&mut self, bound: u64) -> u64 {
        if bound == 0 {
            return 0;
        }
        self.next_u64() % bound
    }

    /// Uniformly distributed `f64` in `[0.0, 1.0)`.
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }
}

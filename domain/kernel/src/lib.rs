//! `kernel` — ForgeFabrik domain kernel.
//!
//! **Layer:** `domain` — deterministic logic only; no I/O, no HTTP, no DB.
//!
//! Defines the CQRS / event-sourcing contracts every ForgeFabrik aggregate must
//! implement:
//!
//! ```text
//! Command → CommandHandler → [Events] → Reducer → State
//! ```
//!
//! ## Determinism rules (mandatory for all implementations)
//!
//! - `chrono::Utc::now()` is **forbidden**.  Use `TickContext.tick`.
//! - `rand::thread_rng()` is **forbidden**.  Use `types::DeterministicRng`.
//! - Global mutable state is **forbidden**.
//! - Thread-dependent behaviour is **forbidden** (breaks cross-node replay).

pub mod traits;

pub use traits::{AggregateRoot, CommandContext, CommandHandler, DeterministicHash, Reducer};
// DeterministicRng lives in foundation/types; re-export for ergonomic access.
pub use types::DeterministicRng;

/// Assert that two independent applications of the same events yield equal results.
///
/// Both `$state` clones must produce the same output via `$reducer`.
/// Requires `Clone + PartialEq + Debug` on the state type.
#[macro_export]
macro_rules! assert_deterministic {
    ($state:expr, $events:expr, $reducer:ty) => {{
        let r1 = $events.iter().fold($state.clone(), |s, e| <$reducer>::apply(s, e));
        let r2 = $events.iter().fold($state.clone(), |s, e| <$reducer>::apply(s, e));
        assert_eq!(r1, r2, "Reducer::apply must be deterministic (same inputs → same outputs)");
    }};
}

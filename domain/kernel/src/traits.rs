//! CQRS / event-sourcing trait contracts.

use types::tick::{TickContext, WorldTick};

// ── CommandContext ─────────────────────────────────────────────────────────────

/// Deterministic context injected into every command handler.
///
/// Wraps [`TickContext`] so command handlers receive time + RNG seed without
/// direct access to wall-clock or thread-local state.
#[derive(Debug, Clone)]
pub struct CommandContext {
    /// Deterministic simulation time and RNG seed.
    pub tick_ctx: TickContext,
}

impl CommandContext {
    /// Construct a minimal context for unit tests.
    pub fn test_context(tick: WorldTick) -> Self {
        Self { tick_ctx: TickContext::new(tick, tick.0) }
    }
    /// Current simulation tick.
    pub fn tick(&self) -> WorldTick { self.tick_ctx.tick }
}

// ── Reducer ────────────────────────────────────────────────────────────────────

/// Apply a single event to a state, producing the next state.
///
/// ## Contract
/// 1. **Deterministic** — same `(state, event)` always produces the same output.
/// 2. **Side-effect free** — no I/O, no randomness, no wall-clock time.
/// 3. **Total** — must handle every variant without panicking.
pub trait Reducer<S, E> {
    fn apply(state: S, event: &E) -> S;
}

// ── CommandHandler ─────────────────────────────────────────────────────────────

/// Validate a command and emit zero or more domain events.
///
/// Deterministic and side-effect free. I/O belongs in the runtime layer.
pub trait CommandHandler<S, C, E> {
    type Error;
    fn handle(&self, state: &S, command: C, ctx: &CommandContext)
        -> Result<Vec<E>, Self::Error>;
}

// ── AggregateRoot ──────────────────────────────────────────────────────────────

/// Combines command handling and state projection for a domain aggregate.
///
/// ```text
/// Command → handle() → [Events] → apply() → NewState
/// ```
pub trait AggregateRoot: Sized {
    type Event;
    type Command;
    type Error;

    /// Validate `command` and emit events. Must be deterministic and pure.
    fn handle(&self, command: Self::Command, ctx: &CommandContext)
        -> Result<Vec<Self::Event>, Self::Error>;

    /// Apply a single event, returning the next state.
    fn apply(state: Self, event: &Self::Event) -> Self;

    /// Replay a sequence of events from an initial state.
    ///
    /// **Invariant**: `replay(initial, events) == sequential apply(initial, events)`.
    fn replay(initial: Self, events: &[Self::Event]) -> Self {
        events.iter().fold(initial, |s, e| Self::apply(s, e))
    }
}

// ── DeterministicHash ──────────────────────────────────────────────────────────

/// Every aggregate participating in replay must expose a deterministic hash.
///
/// ## Contract
/// - No wall-clock time, no randomness, no platform-dependent ordering.
/// - Return value is a hex-encoded BLAKE3 or SHA-256 digest — matches the
///   format of [`types::WorldSnapshot::state_hash`].
/// - Same state → same hash, always, on any node.
pub trait DeterministicHash {
    /// Hex-encoded deterministic hash of the current state.
    fn state_hash(&self) -> String;
}

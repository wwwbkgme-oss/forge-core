//! # forge-core — canonical WorldEvent + EventStore
//!
//! **Layer:** `foundation` — no I/O, no randomness, no business logic.
//!
//! ## Event-First rule (federation-wide)
//!
//! ```text
//! Events are truth. State is projection.
//! Command → Event → Reducer → State Projection
//! ```
//!
//! All federated repos must implement this contract.

pub mod envelope;
pub mod store;
pub mod world;

pub use envelope::{EventEnvelope, EventMeta};
pub use store::{EventStore, ExpectedVersion, StoreError, StoredEvent, StreamId};
pub use world::WorldEvent;

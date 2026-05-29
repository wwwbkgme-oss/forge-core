//! `EventEnvelope` — canonical wrapper for all events.
//!
//! Every event in a federated system MUST be wrapped in an `EventEnvelope`
//! before being stored or transmitted.  This provides correlation,
//! ordering, and schema versioning.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use types::tick::WorldTick;
use uuid::Uuid;

/// Mandatory metadata attached to every event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMeta {
    /// Unique event ID.
    pub event_id: Uuid,
    /// Logical simulation time when the event occurred.
    pub tick: WorldTick,
    /// Wall-clock time — for logging only, never used in reducers.
    pub timestamp: DateTime<Utc>,
    /// Groups causally related events (e.g., one user command).
    pub correlation_id: Option<Uuid>,
    /// Schema version of the event payload.
    pub schema_version: u32,
}

impl EventMeta {
    pub fn new(tick: WorldTick) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            tick,
            timestamp: Utc::now(),
            correlation_id: None,
            schema_version: 1,
        }
    }
}

/// Canonical event envelope — wrap every domain event in this.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<E> {
    pub meta: EventMeta,
    pub payload: E,
}

impl<E> EventEnvelope<E> {
    pub fn new(tick: WorldTick, payload: E) -> Self {
        Self {
            meta: EventMeta::new(tick),
            payload,
        }
    }
}

//! Canonical `EventStore` trait — replay-safe append-only log.
//!
//! **Rule:** Same events → same state → same hash (Replay-Safety invariant).

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Identifies an event stream (one per aggregate instance).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StreamId(pub String);

impl StreamId {
    pub fn new(aggregate_type: &str, id: Uuid) -> Self {
        Self(format!("{aggregate_type}-{id}"))
    }
}

impl std::fmt::Display for StreamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Optimistic concurrency guard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpectedVersion {
    /// Stream must not exist yet.
    NoStream,
    /// Stream must be at exactly this version.
    Exact(u64),
    /// No concurrency check.
    Any,
}

/// A persisted event with sequence metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredEvent<E> {
    pub stream_id: StreamId,
    /// Monotonically increasing within the stream.
    pub version: u64,
    /// Global sequence number across all streams.
    pub global_seq: u64,
    pub payload: E,
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("stream not found: {0}")]
    StreamNotFound(StreamId),
    #[error("version conflict: expected {expected:?}, got {actual}")]
    VersionConflict {
        expected: ExpectedVersion,
        actual: u64,
    },
    #[error("store error: {0}")]
    Backend(String),
}

/// Canonical event store interface.
///
/// Implementations: `InMemoryEventStore` (tests), `PgEventStore` (production).
#[async_trait]
pub trait EventStore<E>: Send + Sync {
    /// Append events to a stream with optimistic concurrency.
    async fn append(
        &self,
        stream: &StreamId,
        expected: ExpectedVersion,
        events: Vec<E>,
    ) -> Result<u64, StoreError>;

    /// Load all events for a stream from `from_version`.
    async fn load(
        &self,
        stream: &StreamId,
        from_version: u64,
    ) -> Result<Vec<StoredEvent<E>>, StoreError>;

    /// Current version of a stream (`0` if it does not exist).
    async fn version(&self, stream: &StreamId) -> Result<u64, StoreError>;
}

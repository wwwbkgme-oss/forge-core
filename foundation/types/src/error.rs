//! Canonical error type for the ForgeFabrik federation.
//!
//! Each repo may extend this with domain-specific variants; the core
//! variants listed here must be implemented identically.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ForgeError {
    // ── Agent errors ─────────────────────────────────────────────────────────
    #[error("agent not found: {0}")]
    AgentNotFound(String),
    #[error("agent is dead: {0}")]
    AgentDead(String),
    #[error("capability denied — entity: {0}, capability: {1}")]
    CapabilityDenied(String, String),

    // ── World errors ─────────────────────────────────────────────────────────
    #[error("chunk not found: {0}")]
    ChunkNotFound(String),

    // ── Consensus / replay ───────────────────────────────────────────────────
    #[error("consensus disputed at tick {0}")]
    ConsensusDisputed(u64),
    #[error("replay hash mismatch at tick {0}: expected {1}, got {2}")]
    ReplayHashMismatch(u64, String, String),

    // ── Plugin ───────────────────────────────────────────────────────────────
    #[error("plugin load error: {0}")]
    PluginLoadError(String),
    #[error("plugin not found: {0}")]
    PluginNotFound(String),
    #[error("unsatisfied plugin dependency: {0} requires {1}")]
    UnsatisfiedDependency(String, String),

    // ── Execution / sandbox ──────────────────────────────────────────────────
    #[error("sandbox not found: {0}")]
    SandboxNotFound(String),
    #[error("execution failed: {0}")]
    ExecutionFailed(String),

    // ── I/O (runtime only — not for use in domain/) ──────────────────────────
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    // ── Catch-all ────────────────────────────────────────────────────────────
    #[error("{0}")]
    Other(String),
}

pub type ForgeResult<T> = Result<T, ForgeError>;

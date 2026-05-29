//! # forge-core — canonical trait contracts
//!
//! **Layer:** `foundation` — trait definitions only; no implementations.
//!
//! Implementations live in `domain/` (deterministic) or `runtime/` (I/O).
//! The layer an implementation belongs to is determined by whether it
//! performs I/O.
//!
//! ## Implementation rules
//!
//! | Trait              | Implements in | Allowed I/O |
//! |--------------------|---------------|-------------|
//! | `WorldSimulator`   | `domain/`     | none        |
//! | `AgentDriver`      | `runtime/`    | HTTPS       |
//! | `SandboxExecutor`  | `runtime/`    | subprocess  |
//! | `PluginHost`       | `runtime/`    | dlopen      |

use async_trait::async_trait;
use types::{
    agent::{Agent, AgentCapability},
    error::ForgeResult,
    plugin::{ForgeFabrikPlugin, PluginCapability, PluginManifest},
    tick::WorldTick,
    world::WorldState,
};
use uuid::Uuid;

// ── AgentDriver ───────────────────────────────────────────────────────────────

/// Connection to an AI backend.
/// Implemented in `runtime/drivers` — may perform I/O.
#[async_trait]
pub trait AgentDriver: Send + Sync {
    /// Unique driver name — must match `AgentKind::to_string()` for the driver
    /// to be looked up correctly by `AgentManager`.
    fn name(&self) -> &str;
    fn capabilities(&self) -> Vec<AgentCapability>;
    async fn complete(&self, agent: &Agent, prompt: &str) -> ForgeResult<String>;
    async fn generate_code(&self, agent: &Agent, task: &str, lang: &str) -> ForgeResult<String>;
    async fn is_available(&self) -> bool;
}

// ── WorldSimulator ────────────────────────────────────────────────────────────

/// Deterministic world simulator.
/// Implemented in `domain/world` — no I/O allowed.
#[async_trait]
pub trait WorldSimulator: Send + Sync {
    async fn tick(&mut self, world: &mut WorldState) -> ForgeResult<Vec<events::WorldEvent>>;
    fn compute_hash(&self, world: &WorldState) -> String;
}

// ── PluginHost ────────────────────────────────────────────────────────────────

/// Plugin lifecycle manager.
/// Implemented in `runtime/plugin` — uses dlopen/libloading.
#[async_trait]
pub trait PluginHost: Send + Sync {
    async fn load(&mut self, manifest_path: &str) -> ForgeResult<ForgeFabrikPlugin>;
    async fn unload(&mut self, plugin_id: &str) -> ForgeResult<()>;
    fn get(&self, plugin_id: &str) -> Option<&ForgeFabrikPlugin>;
    fn list(&self) -> Vec<&ForgeFabrikPlugin>;
    fn available_capabilities(&self) -> Vec<PluginCapability>;
    fn validate_manifest(&self, manifest: &PluginManifest) -> ForgeResult<()>;
}

// ── ConsensusCoordinator ──────────────────────────────────────────────────────

/// BLAKE3 world-state consensus.
/// Implemented in `domain/consensus` — no I/O.
#[async_trait]
pub trait ConsensusCoordinator: Send + Sync {
    async fn start_round(&self, tick: WorldTick, required_witnesses: u32) -> ForgeResult<()>;
    async fn submit_witness(
        &self,
        tick: WorldTick,
        agent_id: Uuid,
        hash: String,
    ) -> ForgeResult<()>;
    async fn await_consensus(&self, tick: WorldTick, timeout_ms: u64) -> ForgeResult<String>;
}

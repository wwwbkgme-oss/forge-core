//! Canonical agent types — identical semantics required in all repos.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Free-tier LLM providers — no credit card required.
/// Grouped to prevent provider explosion in [`AgentKind`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FreeProvider {
    Groq,
    SambaNova,
    Ollama,
    OpenRouter,
    Cerebras,
}

impl std::fmt::Display for FreeProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Must match AgentDriver::name() in runtime/drivers.
        let s = match self {
            Self::Groq => "Groq",
            Self::SambaNova => "SambaNova",
            Self::Ollama => "Ollama",
            Self::OpenRouter => "OpenRouter",
            Self::Cerebras => "Cerebras",
        };
        write!(f, "{s}")
    }
}

/// Which AI backend drives this agent.
///
/// Variants represent agent semantics, not infrastructure config.
/// Free-tier backends are grouped under `Free(FreeProvider)` to keep this
/// enum stable as new providers are added.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentKind {
    // ── Paid / cloud backends ─────────────────────────────────────────────
    Claude,
    OpenCode,
    Codex,
    // ── Free-tier (grouped — add new providers to FreeProvider, not here) ─
    Free(FreeProvider),
    // ── Escape hatch ─────────────────────────────────────────────────────
    Custom { name: String, endpoint: String },
}

impl std::fmt::Display for AgentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Free(p) => write!(f, "{p}"),
            Self::Custom { name, .. } => write!(f, "{name}"),
            _ => write!(f, "{self:?}"),
        }
    }
}

/// Lifecycle phase — `Dead` is terminal.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentState {
    #[default]
    Idle,
    Active {
        current_task: String,
        started_at: DateTime<Utc>,
    },
    Executing,
    Resting {
        until_tick: u64,
    },
    Dead {
        reason: String,
    },
}

/// Permissions an agent has been granted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentCapability {
    BuildAndMine,
    ExecuteCode,
    Combat,
    Trade,
    Witness,
    QuestGiver,
}

/// The canonical agent record — Single Source of Truth.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub kind: AgentKind,
    pub state: AgentState,
    pub capabilities: Vec<AgentCapability>,
    pub spawned_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
}

impl Agent {
    pub fn new(name: impl Into<String>, kind: AgentKind) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            kind,
            state: Default::default(),
            capabilities: vec![
                AgentCapability::BuildAndMine,
                AgentCapability::ExecuteCode,
                AgentCapability::Trade,
                AgentCapability::Witness,
            ],
            spawned_at: now,
            last_active: now,
        }
    }

    pub fn is_alive(&self) -> bool {
        !matches!(self.state, AgentState::Dead { .. })
    }
}

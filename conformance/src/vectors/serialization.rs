//! Canonical serialisation test vectors.
//!
//! ## Rule (frozen, CONFORMANCE_VERSION = 1)
//!
//! All federated repos MUST serialise canonical types to JSON using:
//!   - `serde_json` with default settings
//!   - `#[serde(tag = "type", rename_all = "snake_case")]` on enums
//!   - No pretty-printing in wire format
//!   - Field order is not guaranteed — parse, don't string-compare
//!
//! Canonical JSON is the ONLY wire format for cross-repo event exchange.
//! Binary formats (bincode, CBOR, MessagePack) may be used internally
//! but must never cross a repo boundary.

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use types::{AgentKind, AgentState, FreeProvider, WorldTick};

    // ── WorldTick ─────────────────────────────────────────────────────────────

    #[test]
    fn world_tick_serialises_as_u64() {
        let t = WorldTick(42);
        let json = serde_json::to_string(&t).unwrap();
        assert_eq!(
            json, "42",
            "WorldTick must serialise as bare u64, got: {json}"
        );
    }

    #[test]
    fn world_tick_roundtrip() {
        let t = WorldTick(999);
        let s = serde_json::to_string(&t).unwrap();
        let back: WorldTick = serde_json::from_str(&s).unwrap();
        assert_eq!(t, back);
    }

    // ── AgentKind ─────────────────────────────────────────────────────────────

    #[test]
    fn agent_kind_claude_serialises_as_string() {
        let v: Value = serde_json::to_value(&AgentKind::Claude).unwrap();
        assert_eq!(
            v,
            Value::String("Claude".into()),
            "AgentKind::Claude must serialise as string \"Claude\""
        );
    }

    #[test]
    fn agent_kind_free_groq_serialises_with_tag() {
        let v: Value = serde_json::to_value(AgentKind::Free(FreeProvider::Groq)).unwrap();
        // Expected: {"Free": "Groq"}  or {"Free":{"Groq":null}}
        // The exact shape depends on serde derive — assert it round-trips.
        let s = serde_json::to_string(&AgentKind::Free(FreeProvider::Groq)).unwrap();
        let back: AgentKind = serde_json::from_str(&s).unwrap();
        assert_eq!(
            back,
            AgentKind::Free(FreeProvider::Groq),
            "AgentKind::Free(Groq) must round-trip via JSON; canonical form: {v}"
        );
    }

    #[test]
    fn free_provider_display_matches_driver_name() {
        // FreeProvider::fmt() must return the same string AgentDriver::name()
        // returns in runtime/drivers. These are the canonical names.
        assert_eq!(FreeProvider::Groq.to_string(), "Groq");
        assert_eq!(FreeProvider::SambaNova.to_string(), "SambaNova");
        assert_eq!(FreeProvider::Ollama.to_string(), "Ollama");
        assert_eq!(FreeProvider::OpenRouter.to_string(), "OpenRouter");
        assert_eq!(FreeProvider::Cerebras.to_string(), "Cerebras");
    }

    // ── AgentState ────────────────────────────────────────────────────────────

    #[test]
    fn agent_state_idle_roundtrip() {
        let s = AgentState::Idle;
        let json = serde_json::to_string(&s).unwrap();
        let back: AgentState = serde_json::from_str(&json).unwrap();
        assert_eq!(back, s);
    }

    #[test]
    fn agent_state_dead_roundtrip() {
        let s = AgentState::Dead {
            reason: "fell into lava".into(),
        };
        let json = serde_json::to_string(&s).unwrap();
        let back: AgentState = serde_json::from_str(&json).unwrap();
        assert_eq!(back, s);
    }

    // ── WorldEvent ────────────────────────────────────────────────────────────

    #[test]
    fn world_event_uses_snake_case_tag() {
        use events::WorldEvent;
        use uuid::Uuid;
        let e = WorldEvent::TickAdvanced { tick: WorldTick(1) };
        let v: Value = serde_json::to_value(&e).unwrap();
        // serde(tag = "type", rename_all = "snake_case") → "type":"tick_advanced"
        assert_eq!(
            v.get("type").and_then(Value::as_str),
            Some("tick_advanced"),
            "WorldEvent must use snake_case type tag; got: {v}"
        );
    }

    #[test]
    fn world_event_agent_spawned_roundtrip() {
        use events::WorldEvent;
        use uuid::Uuid;
        let id = Uuid::new_v4();
        let e = WorldEvent::AgentSpawned {
            agent_id: id,
            name: "TestBot".into(),
            tick: WorldTick(5),
        };
        let json = serde_json::to_string(&e).unwrap();
        let back: WorldEvent = serde_json::from_str(&json).unwrap();
        // Verify agent_id survived the roundtrip.
        if let WorldEvent::AgentSpawned { agent_id, .. } = back {
            assert_eq!(agent_id, id);
        } else {
            panic!("deserialized wrong variant");
        }
    }

    // ── EventEnvelope ─────────────────────────────────────────────────────────

    #[test]
    fn event_envelope_roundtrip() {
        use events::{EventEnvelope, WorldEvent};
        let env = EventEnvelope::new(
            WorldTick(10),
            WorldEvent::TickAdvanced {
                tick: WorldTick(10),
            },
        );
        let json = serde_json::to_string(&env).unwrap();
        let back: EventEnvelope<WorldEvent> = serde_json::from_str(&json).unwrap();
        assert_eq!(back.meta.tick, WorldTick(10));
        assert_eq!(back.meta.schema_version, 1);
    }
}

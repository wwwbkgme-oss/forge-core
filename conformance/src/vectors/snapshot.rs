//! Canonical WorldSnapshot test vectors.
//!
//! ## Replay-Safety invariant (federation-wide)
//!
//! ```text
//! assert_eq!(replay(events_up_to(tick)), snapshot.state_hash)
//! ```
//!
//! This must hold in ALL federated repos.  A snapshot mismatch between repos
//! after replaying the same events is a conformance failure.

#[cfg(test)]
mod tests {
    use types::{WorldSnapshot, WorldTick};
    use uuid::Uuid;

    #[test]
    fn new_snapshot_has_correct_tick() {
        let snap = WorldSnapshot::new(WorldTick(42), "abc123");
        assert_eq!(snap.tick, WorldTick(42));
        assert_eq!(snap.state_hash, "abc123");
        assert!(snap.label.is_none());
    }

    #[test]
    fn semantic_equality_ignores_id_and_label() {
        let tick = WorldTick(10);
        let hash = "deadbeef";
        let mut a = WorldSnapshot::new(tick, hash);
        let mut b = WorldSnapshot::new(tick, hash);
        // Give them different IDs and labels.
        b.id = Uuid::new_v4();
        a.label = Some("snap-a".into());
        b.label = Some("snap-b".into());
        // They are still semantically equal.
        assert!(
            a.semantically_eq(&b),
            "snapshots with same tick+hash must be semantically equal"
        );
    }

    #[test]
    fn different_hashes_not_equal() {
        let a = WorldSnapshot::new(WorldTick(10), "aaa");
        let b = WorldSnapshot::new(WorldTick(10), "bbb");
        assert!(!a.semantically_eq(&b));
    }

    #[test]
    fn different_ticks_not_equal() {
        let a = WorldSnapshot::new(WorldTick(10), "hash");
        let b = WorldSnapshot::new(WorldTick(11), "hash");
        assert!(!a.semantically_eq(&b));
    }

    #[test]
    fn snapshot_roundtrip_json() {
        let original = WorldSnapshot::new(WorldTick(99), "cafebabe");
        let json = serde_json::to_string(&original).unwrap();
        let back: WorldSnapshot = serde_json::from_str(&json).unwrap();
        assert!(
            original.semantically_eq(&back),
            "snapshot must be semantically equal after JSON roundtrip"
        );
        assert_eq!(original.id, back.id, "id must be preserved in JSON");
    }
}

//! Mandatory ForgeFabrik conformance tests — domain/kernel tier.
//!
//! Every ForgeFabrik system must pass:
//!
//! 1. **Deterministic replay** — `replay(events) == sequential apply(events)`
//! 2. **Event equality**       — same command on equal states → equal events
//! 3. **Snapshot roundtrip**   — `WorldSnapshot` hash survives JSON round-trip
//!
//! Canonical spec assertion: `assert_eq!(replay(events), snapshot.state_hash)`

use kernel::{AggregateRoot, CommandContext, DeterministicHash, Reducer};
use types::{WorldSnapshot, WorldTick};

// ── Minimal aggregate ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
struct Counter { value: i64, version: u64 }

impl Counter { fn new() -> Self { Self { value: 0, version: 0 } } }

#[derive(Debug, Clone, PartialEq, Eq)]
enum CounterEvent { Incremented { delta: i64 }, Reset }

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum CounterCmd { Increment { delta: i64 }, Reset }

struct CR;
impl Reducer<Counter, CounterEvent> for CR {
    fn apply(mut s: Counter, e: &CounterEvent) -> Counter {
        s.version += 1;
        match e {
            CounterEvent::Incremented { delta } => s.value += delta,
            CounterEvent::Reset                 => s.value  = 0,
        }
        s
    }
}

impl AggregateRoot for Counter {
    type Event = CounterEvent; type Command = CounterCmd; type Error = String;
    fn handle(&self, cmd: CounterCmd, _: &CommandContext) -> Result<Vec<CounterEvent>, String> {
        match cmd {
            CounterCmd::Increment { delta } => {
                if delta == 0 { return Err("delta=0".into()); }
                Ok(vec![CounterEvent::Incremented { delta }])
            }
            CounterCmd::Reset => Ok(vec![CounterEvent::Reset]),
        }
    }
    fn apply(s: Counter, e: &CounterEvent) -> Counter { CR::apply(s, e) }
}

impl DeterministicHash for Counter {
    fn state_hash(&self) -> String { format!("{:016x}{:016x}", self.value as u64, self.version) }
}

fn ctx(t: u64) -> CommandContext { CommandContext::test_context(WorldTick(t)) }

// ── Test 1: Deterministic replay ─────────────────────────────────────────────

#[test]
fn replay_matches_sequential() {
    let init   = Counter::new();
    let events = vec![
        CounterEvent::Incremented { delta: 3 }, CounterEvent::Incremented { delta: 7 },
        CounterEvent::Reset, CounterEvent::Incremented { delta: 1 },
    ];
    let seq      = events.iter().fold(init.clone(), |s, e| Counter::apply(s, e));
    let replayed = Counter::replay(init, &events);
    assert_eq!(seq, replayed);
    assert_eq!(seq.state_hash(), replayed.state_hash(),
        "assert_eq!(replay(events), snapshot.state_hash)");
}

#[test]
fn replay_deterministic_across_runs() {
    let init   = Counter::new();
    let events = vec![CounterEvent::Incremented { delta: 5 }];
    assert_eq!(Counter::replay(init.clone(), &events), Counter::replay(init, &events));
}

// ── Test 2: Event equality ───────────────────────────────────────────────────

#[test]
fn same_command_produces_equal_events() {
    let s1 = Counter::new(); let s2 = Counter::new();
    let e1 = s1.handle(CounterCmd::Increment { delta: 10 }, &ctx(1)).unwrap();
    let e2 = s2.handle(CounterCmd::Increment { delta: 10 }, &ctx(1)).unwrap();
    assert_eq!(e1, e2);
}

#[test]
fn equal_events_equal_hashes() {
    let init   = Counter::new();
    let events = vec![CounterEvent::Incremented { delta: 4 }, CounterEvent::Reset];
    let h1 = Counter::replay(init.clone(), &events).state_hash();
    let h2 = Counter::replay(init, &events).state_hash();
    assert_eq!(h1, h2);
}

#[test]
fn different_events_different_hashes() {
    let init = Counter::new();
    let h_inc   = Counter::replay(init.clone(), &[CounterEvent::Incremented { delta: 1 }]).state_hash();
    let h_reset = Counter::replay(init,          &[CounterEvent::Reset]).state_hash();
    assert_ne!(h_inc, h_reset);
}

// ── Test 3: Snapshot roundtrip ───────────────────────────────────────────────

#[test]
fn snapshot_roundtrip_preserves_hash() {
    let init   = Counter::new();
    let events = vec![CounterEvent::Incremented{delta:10}, CounterEvent::Incremented{delta:5}, CounterEvent::Reset];
    let final_state = Counter::replay(init, &events);
    let hash        = final_state.state_hash();

    let snap = WorldSnapshot::new(WorldTick(events.len() as u64), &hash);
    let json: String = serde_json::to_string(&snap).expect("must serialise");
    let back: WorldSnapshot = serde_json::from_str(&json).expect("must deserialise");

    assert_eq!(snap.state_hash, back.state_hash, "hash must survive JSON roundtrip");
    assert!(snap.semantically_eq(&back));

    // Canonical assertion
    let recomputed = Counter::replay(Counter::new(), &events).state_hash();
    assert_eq!(back.state_hash, recomputed,
        "assert_eq!(replay(events), snapshot.state_hash)");
}

#[test]
fn snapshot_wrong_hash_rejected() {
    let snap = WorldSnapshot::new(WorldTick(1), "correct_hash");
    let back: WorldSnapshot = serde_json::from_str(&serde_json::to_string(&snap).unwrap()).unwrap();
    assert_eq!(back.state_hash, "correct_hash");
    assert!(!snap.semantically_eq(&WorldSnapshot::new(WorldTick(1), "wrong")));
}

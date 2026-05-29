# ForgeFabrik — Event Ordering Specification

**Status:** accepted  
**Version:** 1  
**Applies to:** forge-core · ff-one · ff-two · ff-three

---

## One-sentence rule

> Within a tick, events are totally ordered by emission sequence.
> Across ticks, the tick counter is the causal clock.

---

## Definitions

| Term | Meaning |
|---|---|
| **Tick** | Atomic unit of simulation time — `WorldTick(u64)` |
| **Event** | An immutable fact wrapped in `EventEnvelope` |
| **Stream** | Ordered sequence of events for one aggregate (identified by `StreamId`) |
| **Global sequence** | Monotonically increasing integer assigned by the `EventStore` |
| **Causal order** | Event A causally precedes B if A's effects are visible to B's reducer |

---

## Rule 1 — Tick is the causal clock

Two events with different ticks are causally ordered by their tick:

```
tick(A) < tick(B)  →  A causally precedes B
```

**Implication:** reducers must process all events for tick N before
processing any event for tick N+1.  Violations produce divergent state.

---

## Rule 2 — Within a tick: emission sequence

Within a single tick, events are ordered by the sequence in which the
domain function emitted them (first-emitted = first-applied).

```
tick(A) == tick(B)  →  order determined by emission position in the call stack
```

The canonical emission order within a tick:

```
1. Command validation events (if any)
2. Domain mutation events (e.g. BlockPlaced, AgentMoved)
3. Derived events (e.g. ConsensusRoundStarted triggered by tick advance)
4. TickAdvanced (always last within a tick)
```

`TickAdvanced` MUST be the final event emitted for every tick.
A replay is complete when `TickAdvanced { tick: T }` is seen.

---

## Rule 3 — Per-stream version monotonicity

Within one `StreamId`, event versions form a strictly increasing sequence:

```
version(event[n+1]) == version(event[n]) + 1
```

Gaps are forbidden.  A gap indicates a lost event and is a hard error.

The `EventStore::append` call with `ExpectedVersion::Exact(v)` enforces
this as an optimistic concurrency guard.

---

## Rule 4 — Global sequence across streams

The `EventStore` assigns a monotonically increasing `global_seq` to every
appended event across all streams.  This is the total order for cross-stream
event replay.

```
global_seq(A) < global_seq(B)  →  A was appended before B
```

Global sequence is storage-assigned, not domain-assigned.
Two events in different streams with the same tick may have any relative
global_seq ordering — the tick is the causal truth; global_seq is the
storage truth.

---

## Rule 5 — Causality via correlation_id

Events that belong to the same causal chain (one user command → multiple
domain events) share the same `EventMeta.correlation_id`.

```
correlation_id(A) == correlation_id(B)  →  A and B are causally related
```

Reducers MUST NOT use `correlation_id` to affect state.
It is metadata for debugging and audit trails only.

---

## Rule 6 — No future-tick events

An event with `tick T` may not be emitted by a domain function while the
world clock is at tick `T' < T`.

```
event.meta.tick  ≤  current_world_tick   (invariant)
```

The runtime enforces this by passing only the current `TickContext` to
domain functions.

---

## Rule 7 — Conflict resolution (multi-writer)

When two agents submit conflicting mutations for the same resource in the
same tick (e.g. both place a block at the same position):

1. The `EventStore` serialises the writes via optimistic concurrency.
2. The first writer wins (lower `global_seq`).
3. The second writer receives a `VersionConflict` error and must retry.
4. The retry executes in a new tick (`current_tick + 1`).

**Implication:** the domain must never assume a command will succeed
without checking the store's version.  Reducers are pure — conflict
resolution lives in the command handler, not the reducer.

---

## Rule 8 — Replay completeness

A full replay of a stream is complete when:

1. All events from `version=0` to the latest version have been applied.
2. The final computed `state_hash` equals `WorldSnapshot.state_hash`.

```rust
// Canonical replay assertion (all federated repos must pass this form):
assert_eq!(
    replay_hash(events),
    snapshot.state_hash,
    "replay-safety invariant violated at tick {}",
    snapshot.tick,
);
```

---

## Rule 9 — DomainEvent (cross-repo events)

When a federated system emits a `WorldEvent::DomainEvent { source_repo, event_type, payload }`,
the receiving system treats it as an opaque fact.  Ordering rules 1–8 still apply.

The receiving system MUST NOT reject a `DomainEvent` it does not recognise.
It MUST store it and advance the stream version.  This is the forward-compatibility
contract of the federation.

---

## Forbidden patterns

### ❌ Ordering by wall-clock timestamp

```rust
// WRONG: timestamps are logging metadata, not causal order
events.sort_by_key(|e| e.meta.timestamp);
```

`EventMeta.timestamp` is set at emission time and must not be used to
determine processing order.  Use `meta.tick` and `stream_version` instead.

---

### ❌ Inter-tick event dependency without tick boundary

```rust
// WRONG: reading state written in the same tick before TickAdvanced
let x = world.get_block(pos);  // written two events ago, same tick
```

Domain functions receive immutable state at the start of a tick.
Mid-tick reads of mutable state are forbidden — all reads see the state
at the beginning of the tick.

---

### ❌ Reducer that depends on global_seq

```rust
// WRONG: reducer branches on storage ordering
if event.global_seq % 2 == 0 { ... }
```

Reducers must be pure functions of `(state, event.payload)`.
`global_seq` is invisible to reducers.

---

## Ordering invariants summary

| Invariant | Rule |
|---|---|
| `tick(A) < tick(B)` → A before B | Rule 1 |
| Within a tick: emission order | Rule 2 |
| `TickAdvanced` is final per tick | Rule 2 |
| Per-stream version strictly increases | Rule 3 |
| `global_seq` is total storage order | Rule 4 |
| `correlation_id` is metadata only | Rule 5 |
| No future-tick events | Rule 6 |
| Conflicts resolved by first-writer-wins | Rule 7 |
| Replay completeness = hash equality | Rule 8 |
| `DomainEvent` must not be rejected | Rule 9 |

---

## Conformance test requirement

Every federated repo must pass a test of the form:

```rust
#[test]
fn event_ordering_tick_causality() {
    let mut events = vec![
        make_event(WorldTick(2), "b"),
        make_event(WorldTick(1), "a"),
        make_event(WorldTick(3), "c"),
    ];
    events.sort_by_key(|e| e.meta.tick);
    assert_eq!(events[0].meta.tick, WorldTick(1));
    assert_eq!(events[1].meta.tick, WorldTick(2));
    assert_eq!(events[2].meta.tick, WorldTick(3));
}
```

This verifies that sorting by `meta.tick` produces causal order.

//! Canonical test vectors for the ForgeFabrik federation.
//!
//! Each sub-module covers one area of canonical behaviour.
//! All values are computed once from the forge-core reference implementation
//! and are **immutable** — they are the law for the federation.

pub mod rng;
pub mod serialization;
pub mod snapshot;
pub mod tick;

//! # forge-core conformance test suite
//!
//! This crate defines canonical test vectors for the ForgeFabrik federation.
//! Every federated repo must pass **all** tests in this module against its
//! own local implementation of the canonical types.
//!
//! ## How to use in a federated repo
//!
//! 1. Copy `conformance/src/vectors/` into your repo (e.g. `tests/conformance/`).
//! 2. Replace the `use types::` imports with your own `foundation/types` path.
//! 3. Run `cargo test conformance` — all tests must pass without modification.
//! 4. Any failure = conformance drift that must be fixed before the next release.
//!
//! ## Conformance version
//!
//! The current conformance version is in `types::CONFORMANCE_VERSION`.
//! When any canonical behaviour changes (RNG, serialisation, tick arithmetic),
//! the version is bumped and all repos must re-run their conformance tests.

pub mod vectors;

/// Assert that the local implementation's conformance version matches forge-core.
///
/// Call this at the top of your integration test suite:
/// ```ignore
/// conformance::assert_version(my_crate::CONFORMANCE_VERSION);
/// ```
pub fn assert_version(local_version: u32) {
    assert_eq!(
        local_version,
        types::CONFORMANCE_VERSION,
        "Conformance version mismatch: local={local_version}, forge-core={}. \
         Run conformance tests and fix all failures before releasing.",
        types::CONFORMANCE_VERSION,
    );
}

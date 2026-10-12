//! The contract's facts, as constants.
//!
//! Generated at build time by `build.rs` from the pinned release of the
//! conformance suite, which carries the contract inside it. This module exists
//! so the rest of the program can name a limit without any part of this
//! repository deciding what it is.

include!(concat!(env!("OUT_DIR"), "/contract.rs"));

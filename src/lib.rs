//! The Martian Robots CLI, as a library.
//!
//! The binary is a thin wrapper so every part of the implementation can be
//! exercised directly as well as through the process boundary. The conformance
//! suite grades the process; these modules are what it grades.
//!
//! The split between them follows what the contract can and cannot say.
//! `contract` is generated from the published schema: the limits, the
//! separators, the two vocabularies. `read` follows the grammar's productions
//! and cites the rulings that govern what it refuses. `simulate` is §2.2,
//! which the contract states only in prose, so it is hand-written and the
//! suite is what proves it right. `write` is the output format.

pub mod contract;
pub mod mission;
pub mod read;
pub mod simulate;
pub mod write;

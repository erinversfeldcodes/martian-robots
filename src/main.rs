//! The Martian Robots CLI.
//!
//! What this program must do is not written down in this repository. It is
//! written down in the contract the conformance suite carries, which this
//! repository pins and does not own; `martian-robots-verify --contract`
//! prints it.
//!
//! Nothing is implemented yet. This refuses every input with a diagnostic that
//! says so, rather than crashing or exiting silently, so that the suite grades
//! a program that is incomplete instead of one that is broken.

use std::io::Read;
use std::process::ExitCode;

use martian_robots::contract;

fn main() -> ExitCode {
    // Read stdin and discard it. Leaving it unread would make the behaviour
    // depend on how much the pipe happens to hold, which is a second defect
    // on top of the one below.
    let mut input = Vec::new();
    let _ = std::io::stdin().read_to_end(&mut input);

    eprintln!(
        "not implemented: this program does not yet read a mission (contract {})",
        contract::VERSION
    );
    ExitCode::FAILURE
}

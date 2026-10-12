//! The answer, as the contract wants it written.
//!
//! One line per robot, in input order. What a line looks like — the field
//! order, the separator, the suffix for a lost robot, the line ending — is the
//! contract's to say, so `contract::push_line` is generated from it and this
//! module is the loop that calls it.

use crate::contract;
use crate::simulate::Report;

pub fn answer(reports: &[Report]) -> String {
    let mut written = String::new();
    for report in reports {
        contract::push_line(
            &mut written,
            report.x,
            report.y,
            report.facing.as_char(),
            report.lost,
        );
    }
    written
}

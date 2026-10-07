//! The answer, as the contract wants it written.
//!
//! One line per robot, in input order, each `x y orientation` with ` LOST`
//! appended when the robot walked off. Numbers are canonical decimal: no
//! leading zeros, no signs (R16). The only line ending emitted is `\n`, even
//! when the input used CRLF (R11).

use std::fmt::Write as _;

use crate::simulate::Report;

pub fn answer(reports: &[Report]) -> String {
    let mut written = String::new();
    for report in reports {
        let _ = writeln!(
            written,
            "{} {} {}{}",
            report.x,
            report.y,
            report.facing.as_char(),
            if report.lost { " LOST" } else { "" }
        );
    }
    written
}

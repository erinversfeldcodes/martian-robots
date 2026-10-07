//! The Martian Robots CLI.
//!
//! What this program must do is not written down in this repository. It is in
//! the contract the conformance suite carries, which this repository pins and
//! does not own; `martian-robots-verify --contract` prints it.

use std::ffi::OsString;
use std::io::{Read, Write};
use std::process::ExitCode;

use martian_robots::{contract, read, simulate, write};

const USAGE: &str = "\
Usage: martian-robots < mission

  A mission on stdin, the robots' final positions on stdout. Diagnostics go to
  stderr and the exit code is non-zero when the input is refused.

  -h, --help     print this message and exit
      --version  print the contract version this implements and exit
";

fn main() -> ExitCode {
    // Arguments are decided before stdin is touched. R20 and R21 both say a
    // flag leaves stdin unread, and a program that drains it first cannot
    // honour that however carefully it ignores what it read.
    match std::env::args_os()
        .skip(1)
        .collect::<Vec<OsString>>()
        .as_slice()
    {
        [] => simulate_stdin(),
        [only] if only == "--help" || only == "-h" => {
            print!("{USAGE}");
            ExitCode::SUCCESS
        }
        [only] if only == "--version" => {
            // R26 asks for the major; printing the whole version reports it.
            println!("martian-robots, contract {}", contract::VERSION);
            ExitCode::SUCCESS
        }
        // Anything else, including a flag alongside another argument (R24) and
        // an argument that is not text at all, which is why these arrive as
        // OsString rather than String: `args()` panics on the latter instead
        // of diagnosing it.
        _ => {
            eprintln!("(R21) that is not something this program knows how to be asked");
            eprint!("{USAGE}");
            ExitCode::FAILURE
        }
    }
}

fn simulate_stdin() -> ExitCode {
    let mut input = Vec::new();
    if std::io::stdin().read_to_end(&mut input).is_err() {
        eprintln!("line 1: stdin could not be read (R22)");
        return ExitCode::FAILURE;
    }

    match read::read(&input) {
        Ok(mission) => {
            // Nothing reaches stdout until the whole input has been read and
            // accepted: a rejection must produce no stdout, and a program that
            // answers as it goes cannot take that back.
            let answer = write::answer(&simulate::run(&mission));
            let _ = std::io::stdout().write_all(answer.as_bytes());
            ExitCode::SUCCESS
        }
        Err(problems) => {
            for problem in &problems {
                match problem.line {
                    Some(line) => {
                        eprintln!("line {line}: {} ({})", problem.what, problem.ruling);
                    }
                    None => eprintln!("{} ({})", problem.what, problem.ruling),
                }
            }
            ExitCode::FAILURE
        }
    }
}

//! Read a mission, or say why it is not one.
//!
//! The productions this follows are the grammar's, and the separators, line
//! endings and vocabularies it checks against are the derived constants, so
//! this cannot admit something the published grammar forbids without the
//! constants changing first.
//!
//! Diagnostics are collected rather than returned at the first problem: R25
//! asks for every violation found in one pass, each line that breaks a rule on
//! its own diagnosed whatever is wrong elsewhere.

use crate::contract;
use crate::mission::{Instruction, Mission, Orientation, Robot};

/// One thing wrong, the ruling that governs it, and the line it sits on.
pub struct Problem {
    pub ruling: &'static str,
    pub line: Option<usize>,
    pub what: String,
}

impl Problem {
    fn at(ruling: &'static str, line: usize, what: impl Into<String>) -> Self {
        Self {
            ruling,
            line: Some(line),
            what: what.into(),
        }
    }

    fn nowhere(ruling: &'static str, what: impl Into<String>) -> Self {
        Self {
            ruling,
            line: None,
            what: what.into(),
        }
    }
}

pub fn read(input: &[u8]) -> Result<Mission, Vec<Problem>> {
    let text = match std::str::from_utf8(input) {
        Ok(text) => text,
        Err(bad) => {
            // The line is where the bytes stop being text, counted in the
            // part that was readable. Counted once, on input already refused;
            // a dependency to do it faster would cost more than it saves.
            #[expect(
                clippy::naive_bytecount,
                reason = "one count on input that is already refused"
            )]
            let line = 1 + input[..bad.valid_up_to()]
                .iter()
                .filter(|byte| **byte == b'\n')
                .count();
            return Err(vec![Problem::at("R22", line, "this is not text")]);
        }
    };

    let lines = split(text)?;
    let mut problems = Vec::new();

    let mut at = skip_blank(&lines, 0);
    let Some(grid) = lines.get(at) else {
        // Nothing to blame a line for: the input has no grid line at all.
        return Err(vec![Problem::nowhere("R12", "there is no grid line")]);
    };
    let grid_at = at + 1;
    at += 1;

    let mut world = (0, 0);
    match numbers(grid, 2) {
        Ok(found) => {
            for value in &found {
                if !crate::mission::within_limit(*value) {
                    problems.push(Problem::at(
                        "R5",
                        grid_at,
                        format!("{value} is past the limit of {}", contract::MAX_COORDINATE),
                    ));
                }
            }
            world = (found[0], found[1]);
        }
        Err(what) => problems.push(Problem::at("R12", grid_at, what)),
    }

    let robots = read_robots(&lines, at, world, &mut problems);

    if problems.is_empty() {
        Ok(Mission {
            max_x: world.0,
            max_y: world.1,
            robots,
        })
    } else {
        Err(problems)
    }
}

fn read_robots(
    lines: &[&str],
    from: usize,
    world: (u32, u32),
    problems: &mut Vec<Problem>,
) -> Vec<Robot> {
    let mut robots = Vec::new();
    let mut at = from;

    loop {
        at = skip_blank(lines, at);
        if at >= lines.len() {
            break;
        }

        let position = lines[at];
        let position_at = at + 1;

        // The line after a position line is that robot's instruction line even
        // when blank, so a missing one is a robot that never says what to do
        // (R13) rather than a block separator.
        let Some(instructions) = lines.get(at + 1) else {
            problems.push(Problem::at(
                "R13",
                position_at,
                "this robot never says what to do",
            ));
            break;
        };
        at += 2;

        match robot(position, instructions) {
            Ok(robot) => {
                for (value, axis) in [(robot.x, "x"), (robot.y, "y")] {
                    if !crate::mission::within_limit(value) {
                        problems.push(Problem::at(
                            "R5",
                            position_at,
                            format!("{axis} of {value} is past the limit"),
                        ));
                    }
                }
                if robot.x > world.0 || robot.y > world.1 {
                    problems.push(Problem::at(
                        "R1",
                        position_at,
                        "this robot starts off the world",
                    ));
                }
                if !crate::mission::instructions_within_limit(robot.instructions.len()) {
                    problems.push(Problem::at(
                        "R6",
                        position_at + 1,
                        format!(
                            "{} instructions, and the limit is {}",
                            robot.instructions.len(),
                            contract::MAX_INSTRUCTIONS
                        ),
                    ));
                }
                robots.push(robot);
            }
            Err((ruling, offset, what)) => {
                problems.push(Problem::at(ruling, position_at + offset, what));
            }
        }
    }

    robots
}

/// Lines, with the grammar's endings, R14's implicit final one and R19's guard.
fn split(text: &str) -> Result<Vec<&str>, Vec<Problem>> {
    let mut lines = Vec::new();
    let mut rest = text;
    let mut number = 1;

    while !rest.is_empty() {
        if let Some(at) = rest.find('\n') {
            let line = &rest[..at];
            let line = line.strip_suffix('\r').unwrap_or(line);
            if line.contains('\r') {
                return Err(vec![Problem::at(
                    "R19",
                    number,
                    "a carriage return that ends nothing",
                )]);
            }
            lines.push(line);
            rest = &rest[at + 1..];
            number += 1;
        } else {
            // R14: end of input ends a line, but only a line that is there.
            // R19 lets a trailing carriage return complete one, and R18 stops
            // either from inventing one.
            let line = rest.strip_suffix('\r').unwrap_or(rest);
            if line.contains('\r') {
                return Err(vec![Problem::at(
                    "R19",
                    number,
                    "a carriage return that ends nothing",
                )]);
            }
            if line.is_empty() {
                return Err(vec![Problem::at(
                    "R19",
                    number,
                    "a carriage return cannot make a line out of nothing",
                )]);
            }
            lines.push(line);
            rest = "";
        }
    }

    Ok(lines)
}

/// A line of only whitespace counts as blank (R15), and blank lines between
/// blocks are ignored (R4, R17).
fn blank(line: &str) -> bool {
    line.trim_matches(separators()).is_empty()
}

fn skip_blank(lines: &[&str], mut at: usize) -> usize {
    while at < lines.len() && blank(lines[at]) {
        at += 1;
    }
    at
}

fn separators() -> &'static [char] {
    &contract::SEPARATORS
}

fn robot(position: &str, instructions: &str) -> Result<Robot, (&'static str, usize, String)> {
    let tokens = tokens(position);
    let [x, y, facing] = tokens.as_slice() else {
        return Err((
            "R12",
            0,
            format!("{} token(s) where three belong", tokens.len()),
        ));
    };

    // A token of the right count and the wrong width is Q5's territory: R12
    // and R7 both fit and the contract does not settle which, so this cites
    // the one about the vocabulary.
    let facing = {
        let mut letters = facing.chars();
        match (letters.next(), letters.next()) {
            (Some(only), None) => Orientation::from_char(only)
                .ok_or_else(|| ("R7", 0usize, format!("{only:?} is not a way to face")))?,
            _ => return Err(("R7", 0, format!("{facing:?} is not an orientation"))),
        }
    };

    let trimmed = instructions.trim_matches(separators());
    let mut steps = Vec::with_capacity(trimmed.len());
    for letter in trimmed.chars() {
        let step = Instruction::from_char(letter)
            .ok_or_else(|| ("R7", 1usize, format!("{letter:?} is not something to do")))?;
        steps.push(step);
    }

    Ok(Robot {
        x: number(x).map_err(|what| ("R12", 0, what))?,
        y: number(y).map_err(|what| ("R12", 0, what))?,
        facing,
        instructions: steps,
    })
}

fn numbers(line: &str, want: usize) -> Result<Vec<u32>, String> {
    let tokens = tokens(line);
    if tokens.len() != want {
        return Err(format!("{} token(s) where {want} belong", tokens.len()));
    }
    tokens.iter().map(|token| number(token)).collect()
}

/// A number is digits and nothing else: no sign, no leading `+`. Leading zeros
/// are a spelling rather than a different number (R16).
fn number(token: &str) -> Result<u32, String> {
    if token.is_empty() || !token.chars().all(|letter| letter.is_ascii_digit()) {
        return Err(format!("{token:?} is not a number"));
    }
    token
        .parse()
        .map_err(|_| format!("{token:?} is past anything this contract can mean"))
}

/// Split on runs of the grammar's separators, which is what `ws` admits.
fn tokens(line: &str) -> Vec<&str> {
    line.split(separators())
        .filter(|part| !part.is_empty())
        .collect()
}

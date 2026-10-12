//! §2.2, which no part of the contract states in a machine-readable way.
//!
//! The grammar fixes the alphabet, the limits fix the numbers, and the
//! semantics fix what each instruction does — all of that is generated. What
//! is left is the part the problem statement only says in prose: that a robot
//! which would leave the world is lost and stops, that it leaves a scent where
//! it stood, and that a later robot standing on a scent ignores the move and
//! carries on. So this is hand-written, and the conformance suite is what
//! proves it right.
//!
//! The dispatch is on an instruction's *effect*, not on its letter. A command
//! added to the contract is therefore already handled if it turns or advances,
//! and fails to compile if it does neither — which is the failure one wants.

use crate::mission::{Effect, Mission, Orientation, Robot};

/// Where a robot ended up, and whether it is still on the world.
pub struct Report {
    pub x: u32,
    pub y: u32,
    pub facing: Orientation,
    pub lost: bool,
}

/// Run a mission. The mission must already be valid; nothing here diagnoses.
pub fn run(mission: &Mission) -> Vec<Report> {
    // A scent belongs to the cell a robot was lost from, not to the direction
    // it was facing (R9). The problem statement says "the same grid point",
    // and a direction-based reading passes its published sample and fails at a
    // corner.
    let mut scented: Vec<(u32, u32)> = Vec::new();
    let mut reports = Vec::with_capacity(mission.robots.len());

    for robot in &mission.robots {
        reports.push(run_one(mission, robot, &mut scented));
    }

    reports
}

/// What one grid point of movement came to.
enum Step {
    /// The robot moved here.
    Taken(u32, u32),
    /// The move would have left the world, and a scent said not to make it.
    Ignored,
    /// The move left the world.
    Lost,
}

fn run_one(mission: &Mission, robot: &Robot, scented: &mut Vec<(u32, u32)>) -> Report {
    let (mut x, mut y) = (robot.x, robot.y);
    let mut facing = robot.facing;

    for instruction in &robot.instructions {
        match instruction.effect() {
            Effect::Turn(degrees) => facing = facing.turned(degrees),
            Effect::Advance(points) => {
                // One grid point at a time, so a move of more than one is lost
                // at the edge it crosses rather than over it. `F` advances by
                // one, which makes this a single pass.
                for _ in 0..points {
                    match step(mission, scented, x, y, facing) {
                        Step::Taken(ahead_x, ahead_y) => (x, y) = (ahead_x, ahead_y),
                        // Ignored means skipped rather than fatal: the robot
                        // carries on with the rest of its instructions (R9,
                        // R10). Every remaining point of this same move would
                        // be ignored for the same reason.
                        Step::Ignored => break,
                        // Off the world, so there is nothing left for it to do.
                        Step::Lost => {
                            return Report {
                                x,
                                y,
                                facing,
                                lost: true,
                            };
                        }
                    }
                }
            }
        }
    }

    Report {
        x,
        y,
        facing,
        lost: false,
    }
}

fn step(
    mission: &Mission,
    scented: &mut Vec<(u32, u32)>,
    x: u32,
    y: u32,
    facing: Orientation,
) -> Step {
    let (ahead_x, ahead_y) = facing.ahead(x, y);
    let leaves = ahead_x < 0
        || ahead_y < 0
        || ahead_x > i64::from(mission.max_x)
        || ahead_y > i64::from(mission.max_y);

    if !leaves {
        return match (u32::try_from(ahead_x), u32::try_from(ahead_y)) {
            (Ok(ahead_x), Ok(ahead_y)) => Step::Taken(ahead_x, ahead_y),
            // Unreachable: a cell on the world has unsigned coordinates.
            _ => Step::Ignored,
        };
    }

    if scented.contains(&(x, y)) {
        return Step::Ignored;
    }

    scented.push((x, y));
    Step::Lost
}

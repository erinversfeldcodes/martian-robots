//! §2.2, which no part of the contract states in a machine-readable way.
//!
//! The grammar fixes the alphabet and the limits fix the numbers, but what `L`
//! does, and what a scent is, are prose in the problem statement. So this is
//! hand-written, and the conformance suite is what proves it right.

use crate::mission::{Instruction, Mission, Orientation, Robot};

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

fn run_one(mission: &Mission, robot: &Robot, scented: &mut Vec<(u32, u32)>) -> Report {
    let (mut x, mut y) = (robot.x, robot.y);
    let mut facing = robot.facing;

    for instruction in &robot.instructions {
        match instruction {
            Instruction::Left => facing = facing.turned(3),
            Instruction::Right => facing = facing.turned(1),
            Instruction::Forward => {
                let (ahead_x, ahead_y) = facing.ahead(x, y);
                let leaves = ahead_x < 0
                    || ahead_y < 0
                    || ahead_x > i64::from(mission.max_x)
                    || ahead_y > i64::from(mission.max_y);

                if !leaves {
                    x = u32::try_from(ahead_x).unwrap_or(x);
                    y = u32::try_from(ahead_y).unwrap_or(y);
                    continue;
                }

                // A world-leaving move from a scented cell is ignored, and
                // ignored means skipped rather than fatal: the robot carries
                // on with the rest of its instructions (R9, R10).
                if scented.contains(&(x, y)) {
                    continue;
                }

                scented.push((x, y));
                // Off the grid, so there is nothing left for it to do.
                return Report {
                    x,
                    y,
                    facing,
                    lost: true,
                };
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

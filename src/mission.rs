//! A mission, and what the contract says one is.
//!
//! The shapes here come from the grammar: a grid line, then a robot block per
//! robot. The vocabularies — which letters name an orientation or an
//! instruction, which way each one faces, what each one does — are generated
//! from the contract's semantics at build time and re-exported here, so a
//! letter added to the contract arrives as a variant rather than as something
//! a `_ => None` arm quietly rejects.
//!
//! What the contract cannot say — what a scent does, that a lost robot stops —
//! is in `simulate`, because the problem statement says it in prose.

use crate::contract;

pub use crate::contract::{Effect, Instruction, Orientation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Robot {
    pub x: u32,
    pub y: u32,
    pub facing: Orientation,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mission {
    pub max_x: u32,
    pub max_y: u32,
    pub robots: Vec<Robot>,
}

impl Mission {
    /// Whether every robot starts somewhere the world has (R1).
    pub fn every_robot_starts_on_the_grid(&self) -> bool {
        self.robots
            .iter()
            .all(|robot| robot.x <= self.max_x && robot.y <= self.max_y)
    }
}

/// Whether a coordinate is within the contract's declared limit (R5).
pub fn within_limit(value: u32) -> bool {
    value <= contract::MAX_COORDINATE
}

/// Whether an instruction string is within the contract's declared limit (R6).
pub fn instructions_within_limit(count: usize) -> bool {
    count <= contract::MAX_INSTRUCTIONS
}

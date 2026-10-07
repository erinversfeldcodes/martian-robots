//! A mission, and what the contract says one is.
//!
//! The shapes here come from the grammar: a grid line, then a robot block per
//! robot. What the grammar cannot say lives in `contract` as derived
//! constants, and what neither can say — which way `L` turns, what a scent
//! does — is in `simulate`, because the problem statement says it in prose and
//! no part of the contract is machine-readable about it.

use crate::contract;

/// Which way a robot faces.
///
/// The letters are the grammar's, derived at build time. The *order* is not:
/// the grammar lists them as it pleases, and this is the compass, which only
/// the problem statement fixes — north is the direction from `(x, y)` to
/// `(x, y + 1)`. Turning is rotation through this sequence, so it has to be
/// rotational rather than however the grammar happened to list them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    const COMPASS: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    pub fn from_char(letter: char) -> Option<Self> {
        match letter {
            'N' => Some(Self::North),
            'E' => Some(Self::East),
            'S' => Some(Self::South),
            'W' => Some(Self::West),
            _ => None,
        }
    }

    pub fn as_char(self) -> char {
        match self {
            Self::North => 'N',
            Self::East => 'E',
            Self::South => 'S',
            Self::West => 'W',
        }
    }

    /// Three quarter-turns right is one left, which keeps the arithmetic
    /// unsigned and the two turns one expression.
    #[must_use]
    pub fn turned(self, quarters: usize) -> Self {
        let at = Self::COMPASS
            .iter()
            .position(|point| *point == self)
            .expect("every orientation is on the compass");
        Self::COMPASS[(at + quarters) % 4]
    }

    /// The cell ahead, as signed numbers, because a robot facing south at
    /// `y = 0` steps to `-1` and the contract wants that detected rather than
    /// wrapped.
    pub fn ahead(self, x: u32, y: u32) -> (i64, i64) {
        let (x, y) = (i64::from(x), i64::from(y));
        match self {
            Self::North => (x, y + 1),
            Self::East => (x + 1, y),
            Self::South => (x, y - 1),
            Self::West => (x - 1, y),
        }
    }
}

/// One instruction. `L` and `R` turn; `F` moves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Left,
    Right,
    Forward,
}

impl Instruction {
    pub fn from_char(letter: char) -> Option<Self> {
        match letter {
            'L' => Some(Self::Left),
            'R' => Some(Self::Right),
            'F' => Some(Self::Forward),
            _ => None,
        }
    }
}

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

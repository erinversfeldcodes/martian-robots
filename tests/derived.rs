//! What the build derived from the pinned contract, written down.
//!
//! None of this is this repository's to choose: it comes out of the release in
//! `Cargo.toml`. Writing it down here does not duplicate the contract, it pins
//! what a given pin produced — so bumping the suite shows up as a reviewable
//! diff rather than as constants and match arms that quietly changed under an
//! implementation built against the old ones.

use martian_robots::contract;
use martian_robots::mission::{Effect, Instruction, Orientation};

#[test]
fn the_pinned_contract_states_these_facts() {
    assert_eq!(contract::VERSION, "2.3.0");
    assert_eq!(contract::MAX_COORDINATE, 50);
    assert_eq!(contract::MAX_INSTRUCTIONS, 99);
    assert_eq!(contract::ORIENTATIONS, ['N', 'E', 'S', 'W']);
    assert_eq!(contract::INSTRUCTIONS, ['L', 'R', 'F']);
    assert_eq!(contract::SEPARATORS, [' ', '\t']);
}

#[test]
fn the_pinned_contract_means_these_things() {
    assert_eq!(
        Orientation::COMPASS,
        [
            Orientation::N,
            Orientation::E,
            Orientation::S,
            Orientation::W
        ]
    );
    assert_eq!(Orientation::PER_POINT, 90);
    assert_eq!(Instruction::L.effect(), Effect::Turn(-90));
    assert_eq!(Instruction::R.effect(), Effect::Turn(90));
    assert_eq!(Instruction::F.effect(), Effect::Advance(1));

    let mut written = String::new();
    contract::push_line(&mut written, 1, 1, 'E', false);
    contract::push_line(&mut written, 3, 3, 'N', true);
    assert_eq!(written, "1 1 E\n3 3 N LOST\n");
}

#[test]
fn the_vocabularies_do_not_overlap() {
    // Not a restatement of the contract but a consequence of it that this
    // implementation will rely on: a character read where an instruction
    // belongs can never also be an orientation.
    for instruction in contract::INSTRUCTIONS {
        assert!(
            !contract::ORIENTATIONS.contains(&instruction),
            "{instruction:?} is in both vocabularies"
        );
    }
}

#[test]
fn the_compass_turns_the_way_the_steps_do() {
    // The compass order is derived from the steps, and `turned` walks it. So
    // the two have to agree: turning one point clockwise must land on the
    // orientation whose step is this one's step rotated the same way. A
    // contract whose `dx`/`dy` did not describe a compass would fail here
    // rather than producing a program that turns in a circle of its own.
    for facing in Orientation::COMPASS {
        let (dx, dy) = step(facing);
        let (turned_x, turned_y) = step(facing.turned(Orientation::PER_POINT));
        assert_eq!(
            (turned_x, turned_y),
            (dy, -dx),
            "turning from {facing:?} does not rotate its step"
        );
    }
}

#[test]
fn turning_all_the_way_round_changes_nothing() {
    let full_turn = Orientation::PER_POINT * i32::try_from(Orientation::COMPASS.len()).unwrap();
    for facing in Orientation::COMPASS {
        assert_eq!(facing.turned(full_turn), facing);
        assert_eq!(facing.turned(-full_turn), facing);
    }
}

/// The step an orientation takes, read back out of `ahead`.
fn step(facing: Orientation) -> (i64, i64) {
    let origin = i64::from(contract::MAX_COORDINATE);
    let (x, y) = facing.ahead(contract::MAX_COORDINATE, contract::MAX_COORDINATE);
    (x - origin, y - origin)
}

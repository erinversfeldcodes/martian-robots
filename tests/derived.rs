//! What the build derived from the pinned contract, written down.
//!
//! These constants are not this repository's to choose: they come out of the
//! release in `Cargo.toml`. Writing them down here does not duplicate the
//! contract, it pins what a given pin produced — so bumping the suite shows up
//! as a reviewable diff rather than as constants that quietly changed under an
//! implementation built against the old ones.

use martian_robots::contract;

#[test]
fn the_pinned_contract_states_these_facts() {
    assert_eq!(contract::VERSION, "2.1.0");
    assert_eq!(contract::MAX_COORDINATE, 50);
    assert_eq!(contract::MAX_INSTRUCTIONS, 99);
    assert_eq!(contract::ORIENTATIONS, ['N', 'S', 'E', 'W']);
    assert_eq!(contract::INSTRUCTIONS, ['L', 'R', 'F']);
    assert_eq!(contract::SEPARATORS, [' ', '\t']);
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

//! One suite version, stated in three places, checked against itself.
//!
//! `Cargo.toml` pins the tag it derives the contract's facts from, the
//! workflow pins the tag it installs the grader from, and the README shows a
//! reader the same command. Any two of those drifting apart is silent and
//! wrong in a particular way: the program would be built against one contract
//! and graded against another.

use std::path::Path;

fn read(relative: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("{relative} to exist"))
}

/// Every `vX.Y.Z` that follows `marker` in `text`.
fn tags_after(text: &str, marker: &str) -> Vec<String> {
    text.match_indices(marker)
        .map(|(at, _)| {
            text[at + marker.len()..]
                .chars()
                .skip_while(|character| !character.is_ascii_digit() && *character != 'v')
                .take_while(|character| character.is_ascii_alphanumeric() || *character == '.')
                .collect()
        })
        .collect()
}

#[test]
fn the_manifest_the_workflow_and_the_readme_pin_one_version() {
    let workflow = read(".github/workflows/ci.yml");
    let pinned = workflow
        .lines()
        .find_map(|line| line.trim().strip_prefix("SUITE_TAG:"))
        .expect("SUITE_TAG in the workflow")
        .trim()
        .to_string();

    // The workflow states the version once, as SUITE_TAG, and refers to it by
    // name everywhere else; these two spell it out, so these two can drift.
    let mut found = 0;
    for (what, text, marker) in [
        ("the manifest derives from", read("Cargo.toml"), "tag = "),
        ("the README installs", read("README.md"), "--tag "),
    ] {
        let tags = tags_after(&text, marker);
        assert!(!tags.is_empty(), "{what} no version at all");
        for tag in tags {
            assert_eq!(tag, pinned, "{what} {tag}, and CI pins {pinned}");
            found += 1;
        }
    }

    assert!(
        found >= 2,
        "only {found} tag(s) checked, so this test is watching almost nothing"
    );
}

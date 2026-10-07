# martian-robots

An implementation of the Martian Robots CLI: missions on stdin, results on stdout, diagnostics on stderr, and an exit code that means something.

## What correct means

Not this repository. The problem statement, the contract derived from it, and the conformance suite that enforces the contract all live in [martian-robots-tests](https://github.com/erinversfeldcodes/martian-robots-tests). This repository holds an implementation and nothing else.

The split is deliberate. A specification kept beside the code that implements it drifts towards the code, and the drift is silent: nobody writes down that the parser now accepts something the contract forbids. Keeping the definition somewhere that has never seen this source makes a disagreement between them a fact rather than an opinion. The dependency runs one way, and the suite depends on nothing.

## Building and grading it

The suite is installed from a pinned release, and it carries the contract inside it, so one install provides both the definition of correct and the thing that checks it:

```
cargo install --git https://github.com/erinversfeldcodes/martian-robots-tests \
  --tag v2.2.0 --locked --bin martian-robots-verify
cargo build --release
martian-robots-verify --bin ./target/release/martian-robots
```

`martian-robots-verify --version` reports the release and the contract version it enforces, `--contract` writes the contract out as one document, and `--cases` lists every case by id so a single one can be rerun with `--case <id>`.

## The contract is a build dependency, not a copy

This crate does not restate the limits or the vocabularies. `build.rs` reads them out of the pinned release and generates `contract::MAX_COORDINATE`, `contract::INSTRUCTIONS` and the rest, so there is no second place for them to live and nothing to keep in step. None of the suite is linked into the binary.

`tests/derived.rs` writes down what a given pin produced. That is not a duplicate of the contract: it means bumping the pin arrives as a reviewable diff instead of as constants that changed quietly under an implementation built against the old ones.

The same version is named by `Cargo.toml`, by `SUITE_TAG` in the workflow, and by the command above. `tests/pin.rs` fails if they disagree, because a program built against one contract and graded against another would otherwise look merely wrong.

## Status

The implementation is being ported in slices, and `ENFORCED` in `.github/workflows/ci.yml` says how far it has got. It lists the case groups CI holds this program to, and each slice adds the groups it completes, so the diff of that one line is the record of what has been claimed. Groups not yet listed are still graded on every run, in the ungated step before the gate, which keeps what is left to do in the log rather than in this file.

Nothing is enforced yet. A stub that refuses every input passes 7 of the 63 cases by accident, because for those no stdout plus a non-zero exit plus any diagnostic is the whole demand. That is the baseline the first slice of the port has to beat.

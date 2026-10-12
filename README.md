# martian-robots

An implementation of the Martian Robots CLI: missions on stdin, results on stdout, diagnostics on stderr, and an exit code that means something.

## What correct means

Not this repository. The problem statement, the contract derived from it, and the conformance suite that enforces the contract all live in [martian-robots-tests](https://github.com/erinversfeldcodes/martian-robots-tests). This repository holds an implementation and nothing else.

The split is deliberate. A specification kept beside the code that implements it drifts towards the code, and the drift is silent: nobody writes down that the parser now accepts something the contract forbids. Keeping the definition somewhere that has never seen this source makes a disagreement between them a fact rather than an opinion. The dependency runs one way, and the suite depends on nothing.

## Building and grading it

The suite is installed from a pinned release, and it carries the contract inside it, so one install provides both the definition of correct and the thing that checks it:

```
cargo install --git https://github.com/erinversfeldcodes/martian-robots-tests \
  --tag v2.3.0 --locked --bin martian-robots-verify
cargo build --release
martian-robots-verify --bin ./target/release/martian-robots
```

`martian-robots-verify --version` reports the release and the contract version it enforces, `--contract` writes the contract out as one document, and `--cases` lists every case by id so a single one can be rerun with `--case <id>`.

## The contract is a build dependency, not a copy

This crate does not restate the limits, the vocabularies, or the shape of an answer. `build.rs` reads them out of the pinned release and generates them: the constants, the `Orientation` and `Instruction` types, what each instruction does, and the code that writes a robot's line. So there is no second place for them to live and nothing to keep in step. None of the suite is linked into the binary.

`tests/derived.rs` writes down what a given pin produced. That is not a duplicate of the contract: it means bumping the pin arrives as a reviewable diff instead of as constants that changed quietly under an implementation built against the old ones.

The same version is named by `Cargo.toml`, by `SUITE_TAG` in the workflow, and by the command above. `tests/pin.rs` fails if they disagree, because a program built against one contract and graded against another would otherwise look merely wrong.

## How it is built

The split between modules follows what the contract can and cannot say.

`contract` is generated, and generating it is worth more than not retyping six numbers. `Orientation`'s variants are the letters the contract admits and their order is the compass, derived from the step each one takes rather than from the order the contract lists them in. `Instruction` carries what each instruction *does* — a turn of so many degrees, or a move of so many grid points — so the simulator dispatches on the effect and a command added to the contract is already handled if it turns or advances. The hand-written version of this had a `_ => None` arm, which meant a new command grew the vocabulary constant and was rejected anyway.

`read` follows the grammar's productions and cites the ruling that governs whatever it refuses. It collects every violation rather than stopping at the first, because R25 asks for every one found in one pass — which is the hardest part of the contract and the part a parser generator would be worst at.

`simulate` is what is left of §2.2 once the vocabulary is generated: that a robot which would leave the world is lost and stops, that it leaves a scent where it stood, and that a later robot standing on a scent ignores the move and carries on. The contract states those in prose, so they are hand-written and the suite is what proves them right. The judgement that matters is that a scent belongs to the cell a robot was lost from rather than to the direction it was facing — a reading the problem statement's own sample cannot distinguish.

`write` is a loop. The line itself — field order, separator, the suffix for a lost robot, the line ending — is a template in the contract, compiled at build time into the pushes that fill it in.

## Status

Conforming. The suite's 63 cases pass, and all four generators report nothing across the seeds tried. CI enforces the whole suite rather than a list of groups, and also runs `--seed random` so a defect the fixed seed happens not to reach still has somewhere to surface.

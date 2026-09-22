# martian-robots

An implementation of the Martian Robots CLI: missions on stdin, results on
stdout, diagnostics on stderr, an exit code that means something.

## What correct means

Not this repository. The contract, and the conformance suite that enforces it,
live in [martian-robots-tests](https://github.com/erinversfeldcodes/martian-robots-tests).
This program is graded by them:

```
martian-robots-verify --bin ./target/release/martian-robots
```

The split is deliberate. A specification kept beside the code that implements
it drifts toward the code, and the drift is silent — nobody writes down "the
parser now accepts something the document forbids". Keeping the definition
somewhere that has never seen this source makes a disagreement between them a
fact rather than an opinion.

The dependency runs one way: this repository pins a version of the suite, and
the suite depends on nothing.

## Status

Nothing is implemented yet. The suite's version, and the contract version it
enforces, get pinned here with the first slice of the implementation.

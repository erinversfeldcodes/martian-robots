# The Martian Robots CLI — the brief, the I/O contract, and the ambiguity rulings

This document is the complete interface between the problem and every
implementation and test of it. §1 restates the brief; §2 is the formal I/O
contract the command-line program implements; §3 rules on every ambiguity we
have found in the brief; §4 names the questions we have deliberately left open.

It is written to be read by two audiences who never read each other's work: the
people implementing the program, and the people writing the acceptance suite
that grades it. The suite is authored from this document and the original
problem statement — never from an implementation's source. That is the whole
point of writing it down: where the two sides disagree, the disagreement is a
defect in this document, and it gets fixed here rather than in whichever side
happened to be written second.

**Contract version: 1.0.0.** This version covers the command-line surface only:
text on stdin, text on stdout, diagnostics on stderr, an exit code. Other ways
of reaching the same simulation — an HTTP API, agent tooling, a browser
playground — are out of scope and would arrive as a later version of this
document, not as an implementation detail of this one.

---

## 1. The brief (paraphrased)

Mars is a bounded rectangular grid. Robots occupy integer coordinates with an
orientation (N, S, E, W) and execute instruction strings: `L` (turn left 90°),
`R` (turn right 90°), `F` (move one cell forward). North is from `(x, y)`
toward `(x, y+1)`. Provision should be made for additional instruction types in
future.

A robot moving off the grid is lost forever, but leaves a *scent* at the last
on-grid cell it occupied; an instruction that would move a later robot off the
world from a scented cell is ignored. Robots execute sequentially — each
finishes before the next begins.

Input: the first line is the upper-right coordinate of the world (lower-left is
`0 0`); then two lines per robot (initial position and orientation; instruction
string). Coordinates are at most 50; instruction strings are shorter than 100
characters. Output: each robot's final position and orientation, with ` LOST`
appended if it fell off.

Sample input → output, verbatim from the brief:

```
5 3            1 1 E
1 1 E          3 3 N LOST
RFRFRFRF       2 3 S

3 2 N
FRRFLLFFRRFLL

0 3 W
LLFFFLFLFL
```

## 2. The I/O contract

### 2.1 Grammar

ASCII text on stdin. `eol` is LF or CRLF (R11); `ws` is a run of spaces or
tabs. Blank lines are permitted between robot blocks and ignored (R4), with one
disambiguation rule: **the line immediately following a position line is always
that robot's instruction line, even when blank** (a blank there means zero
instructions, R2). Without this rule, R2 and R4 would be indistinguishable.

The final `eol` in the input may be omitted: end-of-file acts as an implicit
end-of-line (R14) — but only for a non-empty, unterminated final line; the
implicit eol never manufactures a phantom blank line after a terminated one, so
R13 is unaffected (R18). A bare CR at end of input is read as half a CRLF
completed by that implicit eol, but only when it terminates a non-empty line; a
CR not followed by LF anywhere else is invalid (R19).

The unifying principle: **implicit endings never create content** — neither
R14's implicit eol nor R19's CR-completion may manufacture a line that was not
visibly present.

```ebnf
input            = { blank-line } , grid-line , { blank-line } , { robot-block } ;
blank-line       = ows , eol ;
grid-line        = ows , number , ws , number , ows , eol ;
robot-block      = position-line , instruction-line , { blank-line } ;
position-line    = ows , number , ws , number , ws , orientation , ows , eol ;
instruction-line = ows , { instruction } , ows , eol ;      (* 0 to 99 instructions *)
orientation      = "N" | "S" | "E" | "W" ;
instruction      = "L" | "R" | "F" ;
number           = digit , { digit } ;                       (* no sign, no leading + *)
ows              = [ ws ] ;                                  (* optional whitespace *)
```

Semantic constraints, checked after parse: each `number` ≤ 50 (R5); robot start
positions lie on the grid (R1); instruction count ≤ 99 (R6).

### 2.2 Semantics

- The world is the inclusive cell set `(0..=max_x, 0..=max_y)`.
- `L` and `R` rotate 90° in place; `F` translates one cell along the
  orientation.
- An `F` that would leave the world: if the current cell is scented, the
  instruction is ignored (see R9 for the scent model); otherwise the robot is
  lost — it vanishes, the cell becomes scented, and its remaining instructions
  are discarded.
- **Blocking world-leaving moves is scent's only effect (R23).** Moving onto or
  through a scented cell is unremarkable, and a robot may start on one; scent
  never rejects input, alters movement that stays on-grid, or marks a robot.
  Its protection is by **occupancy, not provenance**: a robot that was placed on
  a scented cell is protected exactly as one that walked there.
- Robots run strictly sequentially, in input order.

### 2.3 Output

For each robot, in input order, one line to stdout: `<x> <y> <orientation>` —
the final on-grid position — with ` LOST` appended if the robot was lost
(position and orientation are those held at the moment of loss). Single ASCII
spaces between tokens, LF line endings, no trailing whitespace, no blank lines.
Numbers are canonical decimal — no leading zeros, no signs (R16). The sample in
§1 is normative.

### 2.4 Failure

Invalid input — a grammar violation or a semantic-constraint violation —
produces diagnostics on stderr naming the offending line and rule, every
violation found in one pass (R25), **no output on stdout, and a non-zero exit
code**. Framing is positional (§2.1), so a missing or extra line shifts the
blocks after it, and the lines it shifts are diagnosed where they fall.

Execution is all-or-nothing. Robot blocks are coupled by scent: running "just
the valid robots" could silently change the outcomes of the robots that *are*
valid, because a robot that was skipped might have scented a cell the next one
walks off. An incomplete answer would not be incomplete; it would be wrong.

Valid input exits 0.

### 2.5 Diagnostics content

A diagnostic names its input line as `line N` (1-based, physical) **when the
violation is attributable to a physical line**. Violations with no such line —
empty or blank-only input, which is missing its grid line — carry no line
reference, and a missing instruction line anchors to the position line that
demanded it (R13).

Where one or more numbered rulings govern a rejection, **at least one**
governing tag appears as `(R#)`. Overlapping rulings need not all be
enumerated: an off-grid coordinate that is also greater than 50 may cite either.

Exact prose is implementation-chosen. The line reference and the ruling tag are
contract; the sentence around them is not.

### 2.6 Invocation

The program is a stdin/stdout filter when invoked bare. Its argument surface:

- `--help` or `-h`, as the sole argument, prints usage to **stdout**
  (non-empty, containing the string `Usage`), exits 0, and reads no stdin (R20).
- Any other argument — including a help flag given together with another
  argument (R24) — prints a brief usage error to **stderr** (containing
  `Usage`), exits non-zero, produces no stdout, and reads no stdin (R21).
- Input that is not valid UTF-8 is invalid input under §2.4 (R22): a diagnostic
  on stderr, empty stdout, a non-zero exit.

### 2.7 Versioning

This contract carries a semver version, and an implementation reports the
version it implements.

- A change that alters what input is accepted, or what output a given input
  produces, is a **major** bump. Implementations and test suites pinned to the
  previous major stay correct against it.
- A ruling that settles a question §4 leaves open, without changing the answer
  any conforming implementation already gives, is a **minor** bump.
- Extending the contract to a surface it does not yet cover is a **minor** bump
  for the new surface's own section, not a rewrite of this one.

## 3. Ambiguity rulings

Every ruling below answers a question the brief leaves open, and every one of
them is testable. An implementation that disagrees with a ruling is wrong; a
*reader* who disagrees with a ruling has found a defect in this document, and
the loop that fixes it — finding, ruling, revision, cases — is how the contract
gets hard enough to build against.

| ID | Question the brief leaves open | Ruling |
|---|---|---|
| R1 | Robot's initial position off-grid? | Invalid input (§2.4). Validation happens at the boundary; the simulation never sees an off-grid robot. |
| R2 | Empty instruction line? | Valid: zero instructions; the robot reports its initial state. |
| R3 | Grid line `0 0`? | Valid: a 1×1 world consisting of cell (0,0). |
| R4 | Whitespace tolerance? | Runs of spaces and tabs separate tokens; leading and trailing whitespace on a line is ignored; blank lines between robot blocks are ignored. The brief's own sample input contains them. |
| R5 | Coordinate greater than 50, in the grid line or a position line? | Invalid input. |
| R6 | Instruction string of 100 or more characters? | Invalid input — the brief says "less than 100", so 99 is the maximum valid length. |
| R7 | Characters outside `L R F` in an instruction string, or outside `N S E W` as an orientation? | Invalid input, lowercase included: the vocabulary is strict-uppercase until a future command type says otherwise. |
| R8 | Zero robots — a grid line and nothing else? | Valid: empty stdout, exit 0. |
| R9 | **Scent: cell-based or direction-based?** May a robot at a scented corner cell fall off a *different* edge than the robot that scented it? | **Cell-based**: any `F` that would leave the world from a scented cell is ignored, whatever the direction. This follows the brief's own words — a scent "prohibits future robots from dropping off the world at the same grid point", which names a point, not a heading. The sample data does not disambiguate: robot 3's ignored `F` happens to face the same way robot 2 was lost, so a direction-based implementation passes the sample and fails at a corner. |
| R10 | Multiple robots lost from the same cell? | Idempotent: the cell is scented once, and re-scenting has no additional effect. |
| R11 | Line endings? | Accept LF and CRLF on input; emit LF only. |
| R12 | Duplicate or extra tokens on a line, such as `1 1 E X`? | Invalid input: lines must match the grammar exactly once whitespace is normalised. |
| R13 | Missing instruction line after a position line — end of input mid-block? | Invalid input. |
| R14 | Input ends without a final newline? | Valid: end-of-file acts as an implicit end-of-line. |
| R15 | What is a blank line? | `ows , eol` — a line containing only whitespace counts as blank. |
| R16 | Leading zeros in input; number format on output? | Input: grammatical, interpreted as a decimal value, so `05` is 5. Output: canonical decimal, no leading zeros, no signs. |
| R17 | Blank lines before the grid line? | Accepted and ignored, consistent with R4's liberal framing. |
| R18 | Does R14's implicit eol create a phantom blank line after a terminated final line? | No: it applies only to a non-empty, unterminated final line. A terminated input gains nothing, and R13 stands. |
| R19 | A bare CR at the end of the input? | Read as half a CRLF completed by the implicit eol — **only when it terminates a non-empty line**. `…X\r` at end of input is `…X\r\n`; a lone CR forming an empty line is invalid, as is a CR not followed by LF anywhere else. Without the guard, one invisible byte turns an R13 rejection into an accepted mission by manufacturing an empty instruction line, which R18 forbids. |
| R20 | `--help`? | A real help surface: `--help` or `-h` prints usage to stdout, exits 0, and leaves stdin unread. Silently ignoring argv is not acceptable — it leaves `--help` hanging on a terminal waiting for input. |
| R21 | Other arguments? | Usage error: a brief message to stderr, a non-zero exit, no stdout, stdin unread. |
| R22 | Non-UTF-8 stdin? | Invalid input per §2.4. The contract's text is ASCII; bytes that are not valid UTF-8 cannot be contract input. |
| R23 | What does scent do besides blocking world-leaving moves? Does a robot placed on a scented cell, or one that merely passes through, behave differently? | Nothing else. Entering, crossing, or starting on a scented cell is unremarkable, and scent never rejects input. Protection is by **occupancy, not provenance**: however a robot came to stand on a scented cell, its world-leaving `F` is ignored. |
| R24 | `--help` or `-h` together with another argument? | Usage error, per R21: a help flag prints help only when it is the sole argument. |
| R25 | How many diagnostics must a rejection carry? | Every violation found in one pass: each line that breaks a rule on its own is diagnosed, whatever is wrong with the other lines. The unit is the line, so one diagnostic for an offending line satisfies this however many rules that line breaks. A violation that can only be judged against another, invalid line — an off-grid start measured against an unreadable grid line — need not be reported. |

## 4. Open questions

These are known gaps, left open deliberately. A suite may not test them, and an
implementation may answer them however it likes without being wrong. They are
recorded so that an answer arriving later is a visible decision rather than an
unnoticed drift.

- **Mixed LF and CRLF within one input.** R11 accepts both; whether a single
  input may use each on different lines is unstated.
- **Output on stderr for valid input.** Nothing forbids it, and nothing requires
  it. A suite should ignore stderr when the exit code is 0.
- **Crashes and hangs.** How a grader treats a candidate that panics or never
  terminates is a policy for the grader, not an obligation of this contract.
- **An unterminated final line of only whitespace.** R14's letter admits it — a
  non-empty final line gains an implicit eol — while the "implicit endings
  never create content" principle of §2.1 would refuse it. The two disagree:
  one trailing space can turn an R13 rejection into an accepted mission. R19's
  guard closed exactly this flip for a carriage return; this one is open.
- **Which ruling owns a token of the right count and the wrong width.** A
  position line ending `1 1 EE` has three tokens, so R12's "exactly `x y
  orientation`" and R7's "`EE` is not an orientation" both fit. It is rejected
  either way; which tag it carries is unstated.
- **Whitespace that is neither a space nor a tab.** R4 names spaces and tabs as
  the only separators, so a non-breaking space inside a line is not a separator
  and the line is invalid. Whether the diagnostic should treat it as a bad
  separator or as an unknown character is unstated.
- **Additional command types.** The brief asks that provision be made for them.
  That is a structural obligation on the implementation — adding one should be a
  small, local change — and this contract deliberately does not speculate about
  their syntax.

## 5. Revision history

| Version | Date | Change |
|---|---|---|
| 1.0.0 | 2026-09-22 | Initial contract for the command-line surface: the grammar and its framing rules, the simulation's semantics, the output format, the failure discipline, the content of diagnostics, the invocation surface, and rulings R1–R25. |

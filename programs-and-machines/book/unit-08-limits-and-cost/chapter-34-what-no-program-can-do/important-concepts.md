# Important Concepts

**The halting problem** — given a program and an input, does the program stop?
Turing proved in 1936 that no program decides it, before any computer existed.

**Halting is a property of a program *and* an input** — the verified `countdown`
halts for positive inputs and not for zero, which is why `halts` takes two
arguments.

**Simulation can confirm halting and never non-halting** — a budget that runs out
reports *unknown*, not *never*. Verified: `countdown` from 500 gave `?` under a
budget of 100 and 1500 steps under 2000.

**Collatz** — six lines, verified halting for every value tested, open since 1937.
Chapter 9's question answered: "this loop terminates" is a claim needing proof, and
the proof can be arbitrarily hard.

**The requirements on `halts`** — always terminates, always correct, works for
every program and input. Relax any one and the impossibility goes away, which is
what every real tool does.

**Programs are countable** — finite strings over a finite alphabet, so they can be
listed and numbered. Which makes the diagonal argument possible, and which proves
by itself that most languages have no recognizer, since there are uncountably many.

**The contradiction** — `trouble(P)` loops if `halts(P, P)` says halt and halts if
it says loop. Asking whether `trouble` halts on `trouble` contradicts in both
cases, so `halts` does not exist.

**What carried the proof** — self-reference (a program applied to itself, which
requires Unit VI's thesis), negation, and ordinary construction.

**The diagonal** — rows are programs, columns inputs, entries halting. Flipping the
diagonal produces a row differing from every row, so it is not in the table.
Cantor's 1891 argument, Gödel's 1931 argument, and Turing's, all the same shape.

**Reduction** — show that solving your problem would solve halting, and yours is
unsolvable. How undecidability spreads without a new diagonal argument each time.

**Rice's theorem** — every non-trivial property of a program's *behavior* is
undecidable. Questions about its *text* are decidable, which is why compilers work.

**It does not mean you cannot tell whether a particular program halts** — most
cases are easy. It says no single procedure works for all programs.

**It does not mean verification is pointless** — seL4 and CompCert are formally
verified. No *automatic* procedure works for everything; specific programs can be
proved correct with human guidance.

**It does not mean machines are limited relative to people** — no human decides
halting either, and the proof does not mention machines.

**Sound, complete, terminating — pick two.** The practical statement of
undecidability, and every static analysis tool has made the choice.

**The four strategies** — be conservative and answer "cannot determine"; accept
false positives; accept false negatives; bound the effort. Every real tool is one
of these.

**Java's definite-assignment analysis** rejects code that is in fact correct,
deliberately, because it chose soundness and termination over completeness.

**Type systems decide a decidable approximation** — "no type errors" is weaker than
"no errors" and is checkable. That is why type systems reject some correct
programs.

**Gödel's incompleteness** is the same argument in logic, and Turing's paper was
about Hilbert's *Entscheidungsproblem*. The machine was invented to make "effective
procedure" precise; the computer is a side effect of a proof about mathematics.

**Kolmogorov complexity $K(s)$** — the length of the shortest program that outputs
$s$ and halts. Information without probabilities, and a property of the string
rather than of a source.

**The invariance theorem** — $K$ depends on the language only up to an additive
constant, because an interpreter for one language can be written in another. Which
is Chapter 25, used as a proof technique.

**Almost all strings are incompressible** — fewer than one in $2^k$ can be
described in $k$ bits fewer than their length. Chapter 33's counting in
Kolmogorov's terms.

**$K$ is uncomputable** — a short program searching for a high-complexity string
would be a short description of something with no short description. The Berry
paradox made rigorous.

**Shannon against Kolmogorov** — a source against a string; probabilities against
nothing; computable against not. They agree on average, and the trade is exact: a
definition that assumes something, or one you cannot evaluate.

**Uses of $K$ despite uncomputability** — compressed size as an upper bound,
normalized compression distance, minimum description length as quantitative
Occam's razor, and the incompressibility method for proving lower bounds.

**Random means incompressible** — $K(s) \ge |s|$. Probability cannot distinguish
`HHHHHHHH` from a patternless string, since both have probability $2^{-n}$;
incompressibility can, and it applies to individual strings.

**Statistical properties follow from the definition** — a string with too many `H`s
has a shorter description, so incompressibility implies the frequencies come out
right.

**Almost every string is random, and no program can exhibit one** — both true, and
not in tension. Overwhelmingly common and individually unverifiable.

**Pseudorandom output is not random** — a gigabyte from `new Random(42)` has
Kolmogorov complexity of a few hundred bits. Use `Random` for simulation, where
reproducibility is a feature, and `SecureRandom` for anything security-related;
`Random`'s state is recoverable from two consecutive outputs.

**Three impossibility results, one technique** — count what exists, count what the
mechanism distinguishes, observe the second is smaller. Sorting, compression,
computability. Its power is that it ignores the method.

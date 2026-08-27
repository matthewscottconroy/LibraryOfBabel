# People

## Edsger W. Dijkstra (1930–2002)

A Dutch computer scientist, Turing Award winner in 1972, and the person most
responsible for the idea that programs should be *derived* rather than debugged.

His position, stated repeatedly and without much diplomacy, was that testing can
show the presence of bugs but never their absence, and that a discipline relying
on it is not engineering. The alternative he developed — construct the program
alongside a proof of its correctness, so that the two grow together — is what
Section 9.2.1 sketches when it suggests writing the invariant first.

Three specific things in this chapter are his. The technique of proving a loop by
establishment and preservation, developed with C. A. R. Hoare and others. The use
of a decreasing non-negative variant to prove termination. And the argument for
half-open ranges, which he set out in a 1982 handwritten note, EWD831, explaining
why `0 ≤ i < n` is the convention that makes counts subtract cleanly and adjacent
ranges join without adjustment.

He also wrote the 1968 letter published as "Go To Statement Considered Harmful",
whose real argument is precisely this chapter's: unrestricted jumps make it
impossible to say what is true at a given point in the program, and structured
loops are valuable because they make invariants statable.

Dijkstra wrote over a thousand numbered notes, the EWD series, by hand, and
circulated them by post. They are archived and searchable, and they are worth
dipping into — he is a genuinely good writer, frequently unfair, and never dull.

## C. A. R. Hoare (born 1934)

A British computer scientist, Turing Award winner in 1980, known to most
programmers for the quicksort algorithm he invented in 1959 at the age of 25.

His more fundamental contribution is the one behind this chapter. The 1969 paper
"An Axiomatic Basis for Computer Programming" introduced what is now called Hoare
logic, a formal system for proving programs correct. Its central notation is the
**Hoare triple**:

```
{P}  S  {Q}
```

read as: if `P` holds before statement `S` runs, and `S` terminates, then `Q`
holds afterwards. The rule for `while` loops in that system is exactly the
invariant: the loop's precondition and postcondition are both the invariant, and
the loop's exit adds the negation of the condition.

Hoare also gave the most-quoted apology in computing, about introducing the null
reference into ALGOL W in 1965: he called it his billion-dollar mistake, and said
he did it because it was easy. We meet that decision, and its cost, in Chapter 16.

## Robert W. Floyd (1936–2001)

An American computer scientist, Turing Award winner in 1978, whose 1967 paper
"Assigning Meanings to Programs" introduced the method of annotating a flowchart
with assertions that must hold at each point.

This is the direct ancestor of Hoare logic and of the loop invariant. Floyd's
framing is worth knowing because it is geometric: attach a claim to every arrow in
the flowchart, then check that each box preserves the claims around it. A loop is
a cycle in the chart, and the assertion attached to the cycle's entry point is the
invariant.

Floyd was largely self-taught in computing — his degrees were in liberal arts and
physics — and he never completed a doctorate, becoming a full professor at
Stanford on the strength of the work.

## Lothar Collatz (1910–1990)

A German mathematician who posed, in 1937, the problem that appears in Section
9.2.2: repeatedly halve if even and take three-times-plus-one if odd, and ask
whether every starting value reaches 1.

The conjecture has been verified by computer for every starting value up to about
$2^{68}$ and remains unproved. Paul Erdős remarked that mathematics is not yet
ready for such problems, and offered $500 for a solution.

He is included not for his other work — which was in numerical analysis and is
substantial — but because his conjecture is the cleanest available demonstration
that termination is a genuinely separate and genuinely hard question. A loop
short enough to fit on one line can encode an open problem.

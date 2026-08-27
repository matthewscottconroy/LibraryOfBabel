# Key Concepts

**State.** Everything that must be known to continue a computation. The test: if
this were lost, could the work still proceed correctly? If not, it is state.
Includes position within a procedure, not only data.

**Program counter.** The part of the state recording which instruction comes
next. Position is state.

**State space.** The set of all states a machine can be in. Independent
components multiply, so state spaces grow explosively — a program holding two
`int` variables has $2^{64}$ states. **This is why programs cannot be tested
exhaustively**, and why proof-shaped reasoning about loops matters.

**Determinism.** Same state, same next state. A program that behaves
inconsistently is not behaving randomly; it is behaving deterministically on a
state larger than the one you were considering. Finding the bug means finding
the hidden state.

**Transition rule.** A function from state to state. Insisting it is a *function*
is what allows reasoning about a computation without running it.

**A computation is an orbit.** Repeated application of the rule from a starting
state. Exactly three outcomes are possible: reach a final state, cycle forever,
or continue generating new states until memory is exhausted.

**Locality.** A real step inspects and changes a tiny part of the state. This is
what lets fixed, finite hardware operate on states far larger than itself, and it
is the same locality that made column-by-column addition possible in Chapter 2.

**Finite state machine.** A fixed set of states plus a transition table. Enough to
decide parity of a string of any length using one bit of memory — because
tracking parity, rather than a count, is a choice about what to represent.

**The limit of finite state.** With a fixed number of states there is always an
input long enough to overflow the machine's memory. No finite machine accepts
exactly the strings of *n* zeros followed by *n* ones.

**Turing machine.** A finite state machine plus an unbounded tape the head can
read, write, and move over in both directions. Invented in 1936 to define
"mechanical procedure" precisely, so that something could be proved about all
such procedures.

**Church–Turing thesis.** Anything computable at all is computable by a Turing
machine. A thesis rather than a theorem, because "computable at all" is not a
mathematical object. Every alternative model proposed since computes exactly the
same functions.

**Turing complete.** Able to compute everything a Turing machine can. Java is
Turing complete, which means it adds no computational *power* over three rules
and a tape — only expressiveness.

**The stored-program idea.** Instructions live in the same memory as data.
Proposed in the 1945 EDVAC draft; the architecture of every machine you use.

**Instructions are data.** No pattern in memory is marked as code. A pattern is
an instruction when the program counter points at it — being a program is a
matter of use, not of content. This is Chapter 1's thesis in its strongest form,
and it is what makes compilers, operating systems, and interpreters possible.

**The cost of that idea.** Anything that can write data can write instructions,
which is the mechanism behind buffer overflow attacks. The flexibility and the
vulnerability are one feature seen from two sides.

**Java's constructs as states and transitions.** A variable is named state;
assignment is a transition; `if` chooses a continuation by consulting state; a
loop returns the program counter with state changed; a call creates state and
records where to resume; an object bundles state with permitted transitions.

**What a language is actually for.** Not power — the tape machine has that
already — but describability. Java's constructs exist to extend what a person can
hold in their head while writing correct code. That is the engineering problem.

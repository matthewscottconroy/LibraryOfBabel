# Limits and Cost

Two questions remain, and they are the ones the field was founded to answer.

**What does a program cost?** Not on this machine, on this input, today — that is
a measurement and Chapter 18 covered it. The question is how the cost *grows*, so
that you can say something about a program before running it and about inputs you
have not seen.

**What can no program do?** Not "what is hard", not "what would take too long".
What is impossible, provably, for every program that could ever be written.

The second question is the more surprising, and it has a definite answer that
predates the first computer.

Four chapters.

**Chapter 32 — Counting the Cost.** Complexity: counting operations rather than
seconds, big-O notation, and the classes you will meet. Then the practical half —
searching and sorting, space as a cost, and the gap between what theory predicts
and what a machine does, which is wider than the theory admits.

**Chapter 33 — Information.** How much is in a message. Entropy, which measures
surprise, and compression, which is what you can do about it. It ends with a
counting argument proving that no compressor can shrink everything — the first
impossibility result in the book, and a gentle one.

**Chapter 34 — What No Program Can Do.** The halting problem. Undecidability.
Kolmogorov complexity, and the observation that most strings cannot be described
more briefly than by writing them out. Turing's 1936 argument, in full, and it is
shorter than you expect.

**Chapter 35 — Where You Are Now.** What was actually learned, what the through-
line was, and what to read next.

Why is this the last unit? Because these are questions about programs in general, and answering them requires
having written some.

Chapter 6 defined a step. Chapter 12 gave you a stack. Chapter 13 gave you
recursion. Chapter 24 turned a program into a tree, and Chapter 25 wrote a program
that reads programs — which is exactly the move Turing's proof depends on. The
halting problem's diagonal argument is not hard, but it is meaningless if you have
never seen a program take another program as data.

You have. Chapter 25 was, among other things, preparation for Chapter 34.

A note on the arithmetic. This unit has more mathematics in it than the others, and none of it is beyond
what a first-year course assumes: logarithms, exponents, sums, and one limit.

Where a claim can be derived, it is derived rather than asserted, which has been
the book's habit throughout. Where a claim is measured, the measurement is shown —
including one where the measured growth disagrees with the predicted growth and
the chapter says so rather than tidying it away.

That last case is the unit in miniature. Theory tells you what a program must cost
asymptotically; a machine tells you what it costs today; and knowing both, and
where they part company, is what the subject is for.

# Evaluation

You are about to write the part of a language implementation that decides what
things *mean* — and it is under fifty lines.

That is worth bracing for, because the gap between how large the idea sounds and
how small the code is turns out to be the lesson. Every interpreter ever written
has the same two-function skeleton, and the reason it keeps being rediscovered is
that there is not much else it could be.

Three lessons.

The `eval`/`apply` structure first — two mutually recursive functions that between
them are an interpreter, and the reason that shape is universal. Then
environments, which are how a language remembers what a name means, and which
turn out to explain scope, shadowing, and the call stack in one object. Then
evaluating arithmetic, which is where the tree from Chapter 24 first produces a
number.

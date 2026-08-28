# Important Researchers

**John Goodenough** (born 1943) wrote the paper that defined the subject:
"Exception Handling: Issues and a Proposed Notation" (1975). Before it, error
handling was return codes and jumps, and the vocabulary this chapter uses —
raising, handling, propagating, resumption against termination — is largely his.
He argued that exception handling is part of a routine's *interface* rather than an
implementation detail, which is exactly the claim Java's checked exceptions took
seriously and the claim that survives the verdict against them.

**Barbara Liskov** appears again, and this time for CLU, which in 1979 introduced
exceptions in essentially the form Java has: a routine declares what it may signal,
signals terminate rather than resume, and an unhandled signal propagates. CLU also
required exceptions to be declared, making it the ancestor of checked exceptions —
though CLU converted an undeclared exception into a failure rather than a compile
error, which is a softer version of the same idea.

**Bjarne Stroustrup** (born 1950) added exceptions to C++ in 1990 and made the
opposite choice on declarations: C++ had `throw` specifications, they were dynamic
rather than static, they were widely regarded as a mistake, and they were removed
in C++17. That C++ and Java tried the two available versions of the same idea and
both abandoned them is the strongest available evidence about the feature.

**Andrei Alexandrescu** and **Walter Bright**, working on D, and the Go team of
**Rob Pike**, **Ken Thompson** and **Robert Griesemer**, are worth naming together
for having designed languages after Java with full knowledge of it and having
declined to copy checked exceptions — Go declining exceptions altogether in favor
of returned error values, on the argument from Section 28.1.1 that invisible
control flow is too high a price. Go's approach has its own well-known
verbosity problem, which is the propagation cost arriving as predicted.

**Jim Gray** (1944–2007) contributed the other half of thinking about failure. His
work on transactions gave the field the idea that a group of operations should
either all happen or none — atomicity — which is a stronger guarantee than any
exception mechanism provides, and Chapter 29 needs it. His 1985 paper "Why Do
Computers Stop and What Can Be Done About It?" observed that most failures in
production are transient and that the most effective recovery is often to restart
the failing component, an argument that runs directly into Section 28.2.3's
position. Turing Award, 1998. He was lost at sea in 2007.

**Joe Armstrong** (1950–2019) designed Erlang around the most extreme version of
failing loudly: **let it crash**. An Erlang process that encounters an unexpected
condition does not attempt to recover; it dies, and a supervisor process restarts
it in a known-good state. The argument is that defensive error handling inside a
process is guesswork, whereas restarting from a known state is reliable — and the
telephone switches built this way achieved availability figures nobody else
matched. It is worth knowing as the position this chapter's Section 28.2.3 stops
short of.

# People

## Maurice Wilkes (1913–2010)

Wilkes appeared in Chapter 10 for discovering debugging. He appears here for the
invention this chapter is about.

Working on EDSAC in 1949, Wilkes and his colleagues David Wheeler and Stanley
Gill faced a practical problem: certain sequences of instructions — computing a
square root, printing a number — were needed repeatedly, and copying them into
every program was laborious and error-prone.

Their answer was the **subroutine**: write the sequence once, store it, and jump
to it from wherever it was needed. The difficulty was getting back afterwards,
since the return address differs for each call. Wheeler's solution, the *Wheeler
jump*, arranged for the caller to deposit the return address where the subroutine
could find it — the ancestor of the call stack in Chapter 12.

Their 1951 book *The Preparation of Programs for an Electronic Digital Computer*
is the first programming textbook, and it introduced the idea of a **subroutine
library**: a collection of prewritten, tested procedures that a programmer could
draw on rather than rewrite.

That is the beginning of software as a shared discipline rather than a private
craft, and it is the direct ancestor of every library you will ever import.

## David Wheeler (1927–2004)

The author of the Wheeler jump, and the recipient of the world's first computer
science PhD, at Cambridge in 1951.

He is also the source of a remark that has become the field's most-quoted
aphorism:

> All problems in computer science can be solved by another level of indirection.

Usually cited approvingly, and usually without the second half, which Wheeler is
reported to have added:

> ...except for the problem of too many levels of indirection.

Which is precisely Section 11.1.1's warning about cost. The person who invented
the mechanism was also clear that using it without limit does not work.

## Barbara Liskov (born 1939)

Liskov appeared in Chapter 7 for scope and will appear again in Chapter 21. Here
she is for **abstraction as a discipline**.

Her 1974 paper with Stephen Zilles, "Programming with Abstract Data Types",
argued that a program should be built from units defined by *what they promise*
rather than *how they work* — and that a language should make the distinction
enforceable rather than merely conventional.

The contract in Section 11.2.1 is that idea at the level of a single method. Unit
IV applies it to data and Unit V to objects. The through-line is hers: a
specification is the interface, the implementation is private, and the value of
the arrangement is that the two can vary independently.

Her Turing Award citation in 2008 names data abstraction as the contribution, and
the phrase "programming methodology" — the notion that *how to write programs*
is a subject with results, rather than a matter of taste — is largely her doing.

## Bertrand Meyer (born 1950)

A French computer scientist, designer of the Eiffel language, and the person who
made *design by contract* an explicit methodology.

Meyer's insight was that preconditions and postconditions should not be comments
but **part of the language**, checkable by the compiler and the runtime. In Eiffel
a method declares its `require` and `ensure` clauses in its signature, and
violations are detected and attributed automatically — a precondition failure
blames the caller, a postcondition failure blames the method.

That attribution is the part worth carrying. Section 11.2.2's advice to fail fast
is a manual approximation of it: the point of checking at the boundary is not
merely to stop early but to make the error report name the party at fault.

His *Object-Oriented Software Construction* (1988, second edition 1997) is long,
opinionated, and one of the few books that treats specification as central rather
than as documentation.

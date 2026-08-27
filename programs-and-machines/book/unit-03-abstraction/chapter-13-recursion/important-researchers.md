# People

## John McCarthy (1927–2011)

The creator of Lisp in 1958, and the person who put recursion at the center of
programming.

Lisp was built on recursive definitions from the start: its data structure — the
list — is defined recursively, and its functions are naturally written as
recursions over that structure. McCarthy's 1960 paper, "Recursive Functions of
Symbolic Expressions and Their Computation by Machine", is one of the founding
documents of the field and describes `eval`, a function that interprets Lisp
expressions written as Lisp data.

We will build the equivalent in Chapter 25, and it will be a recursive walk over a
tree — this chapter's structural recursion, applied to programs.

McCarthy also coined the term "artificial intelligence" and invented garbage
collection, the mechanism that lets Chapter 12's heap objects be freed
automatically.

## Alonzo Church (1903–1995)

Church appeared in Chapter 6 as the co-discoverer of undecidability. He belongs
here because the lambda calculus is a system in which recursion is the *only* form
of repetition.

There are no loops in the lambda calculus, and no assignment. Everything is
function definition and application, and any repetition is achieved by recursion —
which makes it a good demonstration of Chapter 13's claim that the two constructs
are interconvertible.

There is a genuinely striking result here. The lambda calculus has no way for a
function to name itself, so writing a recursive function looks impossible. It is
nonetheless achievable, through a construction called the Y combinator, which
takes a non-recursive function and produces a recursive one. It is beautiful and
initially incomprehensible, and it is the deepest available answer to "how can
something be defined in terms of itself".

## Giuseppe Peano (1858–1932)

An Italian mathematician whose 1889 axioms defined the natural numbers
recursively: there is 0, every number has a successor, and — the axiom that
matters here — **induction holds**.

Peano's fifth axiom *is* mathematical induction, stated as a defining property of
the natural numbers rather than as a technique for proving things about them.
That is the right way round: induction works because of what the natural numbers
are.

Which means Section 13.1.3's correspondence is deeper than an analogy. Recursion
over the natural numbers is valid because the natural numbers are defined
recursively, and the induction principle is the statement of that definition.

He also invented much of the notation still used in mathematical logic, including
the symbols for set membership and existential quantification.

## Ada Lovelace (1815–1852)

Included for her notes on Charles Babbage's proposed Analytical Engine, published
in 1843 — in particular Note G, which contains a method for computing Bernoulli
numbers and is generally regarded as the first published algorithm intended for a
machine.

The algorithm is iterative rather than recursive, and she is here for something
adjacent. Lovelace was clear, in a way Babbage was not, that the machine's
significance lay in operating on *symbols* rather than numbers — that if the
relationships between things could be expressed symbolically, the engine could
manipulate them, and it might "act upon other things besides number".

That is the stored-program insight of Chapter 6, and it anticipates Unit VI's
programs-as-data by a century. She also observed that the machine could originate
nothing, only do what it was ordered to do — a remark Turing took seriously enough
to devote a section of his 1950 paper to answering.

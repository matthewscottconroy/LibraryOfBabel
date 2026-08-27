# Important Researchers

**Barbara Liskov** (born 1939) has appeared in this book more than anyone else, and
this chapter is why. Her 1987 OOPSLA keynote *Data Abstraction and Hierarchy* made
an argument that was unwelcome at the time: inheritance had been adopted mainly as
a way of sharing implementation, when the only thing it should be used to express
is that one type may stand in for another. The precise formulation came in 1994,
with Jeannette Wing, as *A Behavioral Notion of Subtyping*. It is the principle that
carries her name, and the fact that a square-rectangle hierarchy still catches
people forty years later is a measure of how much it was needed. Turing Award,
2008.

**Jeannette Wing** (born 1956) co-authored the 1994 paper that turned the keynote
into a definition, giving the obligations on preconditions, postconditions,
invariants and history their formal statement. Her later work on computational
thinking argued that the reasoning habits of the field belong in general education,
which is an argument this book is a small instance of.

**Ole-Johan Dahl** (1931–2002) and **Kristen Nygaard** (1926–2002) invented
inheritance. Simula 67, written at the Norwegian Computing Center, introduced
classes, subclasses, virtual methods and dynamic dispatch — nearly everything this
chapter describes — as tools for writing simulations, where modeling kinds of
thing is the natural task. They shared the Turing Award in 2001, both dying the
following year.

**Alan Kay** (born 1940) coined "object-oriented" and meant something narrower than
what the phrase became. Smalltalk's emphasis was message passing and late binding —
the receiver decides what to do — rather than class hierarchies, and Kay later
remarked that he had not had inheritance in mind. Chapter 22's interfaces are
closer to his idea than this chapter's hierarchies are. Turing Award, 2003.

**Bjarne Stroustrup** (born 1950) made the opposite default choice in C++: methods
are statically bound unless marked `virtual`. The reasoning was that a program
should not pay for a facility it does not use. Java's reversal is the reason
`final` on a method carries meaning, and the reason so much JIT engineering goes
into making virtual calls cheap.

**Joshua Bloch** (born 1961) wrote the guidance that the working programmer
actually applies: design for inheritance and document it, or prohibit it. *Effective
Java*'s item on the subject shows how a superclass's own internal calls become part
of its contract the moment subclassing is allowed, which is why the choice must be
deliberate. Chapter 23 takes up his case for composition.

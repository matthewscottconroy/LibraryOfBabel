# Important Researchers

**Alan Kay** (born 1940) returns from Chapter 21, because this is the chapter he
would recognize. Smalltalk's object model was about what an object *responds to*,
not what it inherits from — a receiver gets a message and decides what to do, and
nothing else in the system needs to know its class. An interface is that idea
expressed in a statically typed language: a set of messages, a promise to respond,
and no commitment about what is underneath. Kay's remark that he "did not have
C++ in mind" when he coined object-oriented is usually quoted as a complaint about
syntax; it was a complaint about hierarchy.

**Erich Gamma, Richard Helm, Ralph Johnson, and John Vlissides** — the four
authors of *Design Patterns* (1994), universally called the Gang of Four — wrote
down the two rules this chapter turns on: *program to an interface, not an
implementation*, and *favor object composition over class inheritance*. Both were
observations about what good code already did, catalogued rather than invented.
The book's reputation has suffered from readers who took the patterns as
recipes; the two principles at the front have held up better than most of the
twenty-three chapters behind them. The template method of Section 22.1.2 is one of
those chapters.

**Ralph Johnson** (born 1955) deserves separate mention for the framing that a
framework is defined by its abstract classes and the calls they make into
subclasses — inversion of control, before that phrase existed. It is the template
method scaled up to an entire library, and it describes how nearly every modern
application framework works.

**Robin Milner** (1934–2010) designed ML, whose datatypes are the ancestor of the
sealed-interface-plus-record combination at the end of this chapter. ML let a
programmer declare that a type is one of a fixed set of shapes, each carrying its
own data, and made the compiler check that every pattern match covered all of
them. That was 1973. Java 21 has it. Milner also gave the field type inference
and, separately, the calculus behind concurrent process modeling; Chapter 24 owes
him a good deal. Turing Award, 1991.

**Brian Goetz** (born 1968) led the design of records, sealed types, and pattern
matching as Java's language architect, and wrote the design documents that explain
*why* — most usefully, the argument that a record is not a boilerplate reducer but
a declaration that a type is transparently its data, with the generated methods
following from that claim rather than being the point of it. Reading his JEP
prose after this chapter is a good way to see language design being argued rather
than announced.

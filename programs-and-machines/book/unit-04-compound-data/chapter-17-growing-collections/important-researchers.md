# People

## Joshua Bloch (born 1961)

Bloch's fourth appearance, and this chapter is his work.

He designed and implemented the Java Collections Framework, released in Java 1.2
in 1998. Before it, Java had `Vector`, `Hashtable`, and `Enumeration` — a small,
inconsistent set with no common interfaces, so a method written to work on one
could not work on another.

The framework's contribution was the separation this chapter is organized around:
**interfaces describing what a collection is** (`List`, `Set`, `Map`) and
**classes describing how it is stored** (`ArrayList`, `HashSet`, `TreeMap`). That
is Chapter 16's abstract data type, applied at library scale, and it is why
`Collections.sort` can sort anything that is a `List` regardless of implementation.

He has also written the most useful commentary on the framework's mistakes.
*Effective Java* warns against several behaviors his own designs permit, which is
a more valuable thing for a designer to do than defend them.

## Neal Gafter (born 1959)

The principal implementer of generics in Java 5, working with Gilad Bracha and
others on the design that became JSR 14.

The constraint was brutal: add compile-time type checking to a library with
millions of existing users, without breaking any of it. Erasure was the answer —
check the types, then throw them away, so that generic and non-generic code compile
to the same thing and interoperate.

It works, and Section 17.2.1's three limitations are its permanent cost. Gafter has
written at length about what a non-erased design would have allowed and why it was
not available, and it is a good case study in a technically inferior choice being
correct given the constraints.

He also coined the phrase "super type token" for a trick that recovers some erased
information, which tells you how much people wanted it back.

## Martin Odersky (born 1958)

A German computer scientist, now at EPFL, who wrote the compiler that became
`javac` and co-designed Java's generics before going on to create Scala.

His involvement matters here for a reason that is easy to miss: the generic type
system Java adopted came substantially from research on Pizza and GJ, extensions
he built to demonstrate that generics could be added to Java compatibly. The
demonstration is what made the language change possible.

Scala was, in part, his answer to the constraints that produced erasure — a fresh
language on the same virtual machine, with a type system unconstrained by
compatibility.

## Donald Knuth (born 1938)

Included for the analysis behind Section 17.1.2.

The technique of averaging cost over a sequence of operations — showing that
occasional expensive operations are rare enough not to matter — is **amortized
analysis**, and while the term was popularized by Robert Tarjan in 1985, the
underlying accounting appears throughout Knuth's *The Art of Computer Programming*,
which began publication in 1968 and remains unfinished.

Knuth's larger relevance is the standard he set: that claims about how fast a
program runs should be *derived* rather than asserted, with the arithmetic shown.
The sum of powers of two in Section 17.1.2 is a small instance of that habit, and
Chapter 32 will take it up properly.

He is also the author of TeX, the typesetting system this book is set with, written
because he was unhappy with the appearance of the second edition of his own book —
which is either admirable or alarming depending on your temperament.

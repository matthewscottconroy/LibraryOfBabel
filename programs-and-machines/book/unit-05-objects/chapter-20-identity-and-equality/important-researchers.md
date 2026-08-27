# People

## Joshua Bloch (born 1961)

Bloch's sixth appearance, and this chapter is largely his.

*Effective Java* Item 10 ("Obey the general contract when you override equals"),
Item 11 ("Always override hashCode when you override equals"), and Item 17
("Minimize mutability") are the three sections of this chapter stated as
practice. The five rules for an immutable class in Section 20.2.3 are his.

His framing of the recommendation is worth quoting because it is stronger than
most people expect: classes should be immutable unless there is a very good reason
to make them mutable. Not "consider immutability" — immutable by default, with
mutability justified.

He also documented the failure modes: the `equals` that is an overload rather than
an override, the hash key mutated after insertion, the `final` field holding a
mutable list. Every one of them appears here because he wrote them down first.

## Gottfried Wilhelm Leibniz (1646–1716)

Leibniz appeared in Chapter 1 for binary arithmetic. He appears here for a
principle that predates computing by three centuries.

The **identity of indiscernibles** holds that two things sharing every property
are one thing. Its converse, the indiscernibility of identicals, holds that one
thing has all the same properties as itself.

That is precisely the tension this chapter is about. Java's `==` implements the
second and says nothing about the first: two `Point` objects with identical
coordinates share every property you care about and are still two objects, because
they have different addresses — a property you did not want to count.

`equals` is where you declare which properties count. Deciding that is a
philosophical question with a practical answer, and it is why the language cannot
decide it for you.

## Barbara Liskov (born 1939)

Liskov's sixth appearance, for a warning that lands here and pays off in Chapter
21.

Her work on abstract data types raised a question this chapter has to answer: if a
type is defined by what it promises rather than how it is stored, then two objects
are equal when they represent the same abstract value — not when their
representations match.

Two sets containing the same elements in different internal order are equal. Two
lists with the same elements in different order are not. The difference is in what
the abstraction says, not in the bytes, and an `equals` that compares
representations rather than abstract values is wrong even when it compiles.

The **abstraction function** — the mapping from a representation to the abstract
value it stands for — is her formalization of exactly this, and it is the missing
half of Chapter 16's representation invariant.

## Henry Baker (born 1949)

An American computer scientist who wrote extensively on the semantics of equality
in programming languages, including a 1993 paper on the varieties of equality
predicates — identity, structural equality, and the several notions that sit
between them.

His observation worth carrying: many languages provide two or three equality
operators and none of them is *the* right one, because the right one depends on
what the object models. Lisp has `eq`, `eql`, `equal` and `equalp`, which is
honest about the problem in a way that Java's two are not.

He also worked on garbage collection, which is what allows Java to have references
without addresses and is therefore why Section 20.1.1 can say a reference is
opaque.

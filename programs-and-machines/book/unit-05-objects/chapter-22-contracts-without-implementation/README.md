# Contracts Without Implementation

Chapter 21 got substitutability by inheriting. A `Circle` could stand where a
`Shape` stood because it extended `Shape` and took the implementation along with
the type.

Often you want only the first half. You want a common type — something a method
can be written against — with no shared code at all, because the implementations
have nothing in common except what they promise to do. A `Comparable` string and
a `Comparable` date share a promise and not a line of code.

That is what an **interface** is: a contract with no implementation, and the most
important construct in the chapter.

Three more follow, and the four together are the vocabulary for saying what
something must do without saying how.

**Abstract classes** sit between an interface and a class: some code supplied,
some left as a hole for the subclass to fill. Section 22.1.2 shows an `Account`
that implements deposits completely and leaves the fee calculation open.

**Enums** are the opposite move — not an open contract but a closed set. Three
weathers, exactly three, and the compiler knows there are three, which is why the
`switch` Chapter 8 promised needs no `default`.

**Records** are a closed shape for data. One line declares the components, and
`equals`, `hashCode`, `toString`, the constructor and the accessors are generated
from them. Chapter 20's entire discipline, done for you, correctly.

Records have been promised repeatedly — in Chapter 11 for returning two values, in
Chapter 17 for map keys, in Chapters 19 and 20 for the boilerplate that
encapsulation demands of a class with no invariant. They arrive here.

The theme running underneath is the one from Chapter 19: an abstraction is a
boundary, and its value is what it does not say. An interface is the purest form
of that idea in the language — it is *nothing but* the boundary. And it is what
Alan Kay had in mind when he named object orientation, considerably more than
Chapter 21's hierarchies were.

One warning about the shape of the chapter. Records and enums look like
conveniences and are not; they are both about *restriction*, about telling the
compiler that a set is closed or a type has no hidden state, and getting real
guarantees back. That trade — say less, get more checked — is worth watching for.

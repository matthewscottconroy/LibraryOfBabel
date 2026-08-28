# Contracts Without Implementation

Consider two things with nothing in common: a piece of text, and a date on a
calendar.

They share no parts. Ask each one what it is made of and the answers have no
overlap at all — characters on one side, a year and a month and a day on the
other. There is no sense in which a date is a kind of string or a string is a kind
of date.

And yet one sorting routine, written once, by somebody who had never heard of
either of them, will put a list of dates in order and a list of words in order and
never know the difference.

What do they share, then, if not substance? Only this: each of them can be asked
whether it comes before another of its kind, and each of them will answer. Not the
same way. The same *question*.

That is worth sitting with for a moment, because it is the whole chapter. Circles
could stand in for shapes in the previous chapter by inheriting from `Shape` —
taking the type and the implementation together, in one move. Here you want the
first half without the second. A common type that methods can be written against,
and no shared code at all, because there is none to share.

An **interface** is that: a contract with no implementation. It is the most
important construct in this chapter and arguably in the language.

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

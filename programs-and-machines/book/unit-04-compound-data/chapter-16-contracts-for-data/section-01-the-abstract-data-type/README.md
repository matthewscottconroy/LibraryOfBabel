# The Abstract Data Type

Five different programs can keep a collection of names and support the same three
operations, while storing them in five entirely different ways.

If that is true, then "a collection of names" is not the same kind of thing as the
array holding them — and pulling those two apart is the most practical idea in the
unit, because it is what lets one be replaced without disturbing the other.

Two lessons.

The first separates what a thing is from how it is stored — an idea that sounds
philosophical and is the most practical thing in the unit, because it is what
allows an implementation to be replaced without every user changing.

The second is the representation invariant: the claim that makes a heap of values
into a structure, and the obligation that every operation must preserve it.

# From Text to Tree

By the end of this section you will have written a parser: about sixty lines that
take `(2 + 3) * 4` as text and hand back a structure a program can walk.

The thing worth watching for is not the code. It is that the code's *shape* is the
grammar's shape — each rule becomes a method, and a rule that mentions itself
becomes a method that calls itself. Once you have seen a notation turn directly
into a program, parsers stop being mysterious, and something more useful replaces
the mystery.

Three lessons, and by the end of them a working parser.

First tokenizing: characters into words, the regular half of the problem. Then
the syntax tree, which is what a parser produces and what Chapter 25 will walk —
built from Chapter 22's sealed interfaces and records, because that is exactly
what they are for. Then the parser itself, where each grammar rule becomes a
method and the correspondence is close enough to check line by line.

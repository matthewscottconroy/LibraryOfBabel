# From Text to Tree

Three lessons, and by the end of them a working parser.

First tokenizing: characters into words, the regular half of the problem. Then
the syntax tree, which is what a parser produces and what Chapter 25 will walk —
built from Chapter 22's sealed interfaces and records, because that is exactly
what they are for. Then the parser itself, where each grammar rule becomes a
method and the correspondence is close enough to check line by line.

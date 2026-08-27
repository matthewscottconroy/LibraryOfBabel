# People

## Barbara Liskov (born 1939)

Liskov's fourth appearance, and the one where her contribution is the chapter's
whole subject.

The 1974 paper with Stephen Zilles, "Programming with Abstract Data Types",
proposed that a program should be built from units defined by their operations
rather than their storage — and, crucially, that a *language* should enforce the
separation rather than leaving it to discipline.

That second half is what made it more than good advice. Before it, you could
choose to program through an interface; nothing stopped anyone reaching around it.
Her CLU language made the representation genuinely unreachable, which converts a
convention into a guarantee — and a guarantee is what lets you reason about which
code can break an invariant.

Section 16.1.2's claim that a class is a boundary around an invariant is her idea
stated for a Java audience. Her Turing Award citation in 2008 names data
abstraction specifically.

## David Parnas (born 1941)

Parnas appears again because the 1972 paper supplies the criterion this chapter
uses.

Liskov gave the mechanism — a unit whose internals are unreachable. Parnas gave
the answer to *where to put the boundaries*: around decisions likely to change, so
that a change is contained in one module.

The two together are information hiding. Section 16.1.1's argument that a storage
decision is exactly the kind of thing that changes, and should therefore be what a
boundary encloses, is his.

## Tony Hoare (born 1934)

Hoare's third appearance, and this time for the mistake rather than the
contribution.

He introduced the null reference into ALGOL W in 1965 and apologized for it in a
2009 talk, calling it his billion-dollar mistake. His account of why is worth
attention: he was building a type system meant to make all use of references
safe, and added null because it was easy to implement.

That is the interesting part. The failure was not carelessness or ignorance of the
consequences. It was a small local convenience adopted inside a design whose whole
purpose it undermined — which is a failure mode worth recognizing, because it is
available to anyone and it does not feel like a mistake at the time.

He is also the source of a remark that belongs in this chapter: that there are two
ways of constructing a design, one being to make it so simple that its
deficiencies are apparent, and the other to make it so complicated that they are
not.

## Graydon Hoare (born 1976)

No relation. The original designer of Rust, begun as a personal project in 2006
and adopted by Mozilla in 2009.

He is here because Rust demonstrates that this chapter's two problems are
solvable. Rust has no null; absence is expressed by an `Option` type that the
compiler forces you to unwrap. And its ownership system tracks which code may
modify a value, which means the escaping-reference failure of Section 16.1.2 —
where an invariant is broken by code outside the unit responsible for it — is
caught at compile time rather than by discipline.

The relevance to Java is not that you should use Rust. It is that Section 16.2.3's
list of workarounds are workarounds, and that the underlying problems have
solutions that Java's compatibility obligations put out of reach.

## Joshua Bloch (born 1961)

Bloch appears a third time. He designed the Java Collections Framework of Chapter
17, and he added autoboxing to Java 5.

He has been candid that autoboxing's interaction with overload resolution and with
`==` produced traps nobody wanted, and *Effective Java* contains items warning
against several behaviors his own feature made possible — Item 61, "Prefer
primitive types to boxed primitives", being the most direct.

That is worth noticing rather than scoring points with. A feature designed by a
careful person to remove tedium introduced a set of subtle failures, because it
hid a distinction that could not actually be hidden. Section 16.2.2's argument
about leaky abstractions is not theoretical; it is what happened here, to someone
who knew the language as well as anyone alive.

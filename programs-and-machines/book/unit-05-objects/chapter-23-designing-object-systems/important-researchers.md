# Important Researchers

**Kent Beck** (born 1961) and **Ward Cunningham** (born 1949) invented the CRC
card in 1989, and the paper describing it is three pages long. Their observation
was that people learning object design kept thinking about classes as data
structures, and that a card with *responsibilities* written on it forced the other
question. Beck went on to test-driven development and extreme programming;
Cunningham invented the wiki, partly to hold the pattern discussions that produced
the design-patterns literature. Both are worth reading for the same quality: a
suspicion of ceremony and a preference for the cheapest thing that works.

**Erich Gamma, Richard Helm, Ralph Johnson, and John Vlissides** return from
Chapter 22 for the principle this chapter argues at length — *favor object
composition over class inheritance* — which appears on page 20 of *Design
Patterns* and is the more important of the book's two rules. The Strategy pattern
of Section 23.2.2 is theirs, and it is the one most students should learn first,
because it is small, its use is immediately clear, and it teaches the habit of turning a
decision into an object.

**Joshua Bloch** (born 1961) supplied the demonstration. The counting-set example
in *Effective Java* is the clearest single argument against casual inheritance
ever written, precisely because the code is short, the override is correct, and
the answer is still wrong. Bloch designed the Java collections framework, which
gives the example an edge: he is criticizing the extensibility of a library he
built.

**Martin Fowler** (born 1963) named the distinction between UML as sketch, as
blueprint, and as programming language, and argued that only the first was worth
anyone's time. He was right, and saying so publicly during the modeling-tool boom
took some nerve. *Refactoring* is his most useful book and it is, read properly, a
design book: a catalogue of the small moves by which a bad structure becomes a
good one without stopping to rewrite.

**Barbara Liskov** appears once more, quietly. Every argument in this chapter for
composition rests on her observation that inheritance is a claim about behavior
rather than a mechanism for reuse — the counting set fails because `CountingHashSet`
cannot honor everything a caller could believe about `HashSet`. Section 21.2.3
gave the principle; this chapter is what following it looks like.

**Rebecca Wirfs-Brock** (born 1953) developed responsibility-driven design into a
full method, and coined the framing that a class is best understood by what it is
responsible for and whom it collaborates with. Her book *Designing Object-Oriented
Software* (1990) predates the design-patterns wave and is less well known than it
deserves; the vocabulary of Section 23.1.1 is largely hers.

**David Parnas** (born 1941) wrote the argument underneath all of this in 1972,
before any of the language existed. "On the Criteria to Be Used in Decomposing
Systems into Modules" observed that the useful decomposition is not by processing
step but by **what each module hides** — and that the thing to hide is whatever is
most likely to change. Every principle in this chapter is a special case of that
sentence. Chapter 19 introduced him for information hiding; this is what the idea
was for.

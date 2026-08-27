# Further Reading

**Joshua Bloch, *Effective Java*, third edition.** Items 42 through 48 are the
practical guidance for this chapter: prefer lambdas to anonymous classes, prefer
method references to lambdas, prefer standard functional interfaces, use streams
judiciously, prefer side-effect-free functions in streams, and — Item 48 — use
caution when making streams parallel. Item 45 in particular is the more careful
version of Section 26.2.3.

**Brian Goetz, "State of the Lambda" (2012).** The design document, written before
Java 8 shipped. It explains why function types were rejected, why the target-typing
approach was chosen, and what problems the primitive specializations solve. Short
and unusually honest about trade-offs.

**Venkat Subramaniam, *Functional Programming in Java*.** A practical book aimed
at exactly the transition this chapter describes, with more worked examples than
there was room for here. Good on the design pressure lambdas put on ordinary
object-oriented code.

**Harold Abelson and Gerald Jay Sussman, *Structure and Interpretation of Computer
Programs*, chapter 1.3.** "Formulating Abstractions with Higher-Order Procedures",
and it makes the argument for this chapter's contents better than anything written
since. The `sum` procedure that takes a term and a next as arguments is worth
working through; it is Section 26.1.3 in four lines of Scheme.

**John Backus, "Can Programming Be Liberated from the von Neumann Style?" (1977).**
Recommended again from Chapter 24, and now with the context to read it properly.
The critique in the first half is the part that lasted.

**Michael Feathers and others, on "functional core, imperative shell".** Not a
single canonical source, but the phrase names Section 26.2.3's structural advice
and searching it will find good treatments. The idea — pure computation in the
middle, effects at the boundary — is the most portable thing in this chapter.

**Philip Wadler, "Monads for Functional Programming" (1995).** Considerably harder
than anything else on this list and worth knowing exists. If you become curious
about why `Optional` and `Stream` have the same shape, this is the answer.

# Further Reading

## On methods and decomposition

Martin, R. C. (2008). *Clean Code*. Prentice Hall. Chapter 3, "Functions".

The argument that methods should be small and do one thing, pushed further than
most people find comfortable. Read it and disagree with parts of it; the
disagreement is where you form a view.

McConnell, S. (2004). *Code Complete* (2nd ed.). Microsoft Press. Chapters 7 and
8.

Chapter 7 on "High-Quality Routines" covers parameter counts, naming, and
cohesion with actual evidence rather than assertion, which is unusual for the
genre. Chapter 8 on defensive programming covers Section 11.2.2's territory.

Bloch, J. (2018). *Effective Java* (3rd ed.). Addison-Wesley. Items 49–56.

The whole chapter on methods. Item 49, "Check parameters for validity", is
Section 11.2.2. Item 51, "Design method signatures carefully", covers parameter
counts and ordering. Item 56 is on documentation comments.

## On contracts

Liskov, B., & Zilles, S. (1974). "Programming with Abstract Data Types."
*ACM SIGPLAN Notices*, 9(4), 50–59.

The paper that made specification-versus-implementation a language design
concern. Short and readable.

Meyer, B. (1992). "Applying 'Design by Contract'." *Computer*, 25(10), 40–51.

The methodology stated compactly, with the attribution argument — who is to blame
when a contract is violated — made explicit.

Liskov, B., & Guttag, J. (2000). *Program Development in Java: Abstraction,
Specification, and Object-Oriented Design*. Addison-Wesley.

Contracts and abstraction taught in Java specifically, by the person who
originated the ideas. Closer to this book's level than Meyer, and the treatment
of specification is the best available.

## Historical

Wilkes, M. V., Wheeler, D. J., & Gill, S. (1951). *The Preparation of Programs
for an Electronic Digital Computer*. Addison-Wesley.

The first programming textbook, and the introduction of the subroutine library.
Available in facsimile. Worth ten minutes to see programming being invented.

## Documentation

*How to Write Doc Comments for the Javadoc Tool*. Oracle.

The conventions for `@param`, `@return`, and `@throws`, with guidance on what to
say in each. Short, and it settles a set of questions you would otherwise answer
inconsistently.

## On purity

Hughes, J. (1989). "Why Functional Programming Matters." *The Computer Journal*,
32(2), 98–107.

Recommended in Chapter 7 and relevant again: the case for methods without effects,
made by someone who has thought about what it costs as well as what it buys.

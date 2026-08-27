# Further Reading

## On decomposition

Parnas, D. L. (1972). "On the Criteria To Be Used in Decomposing Systems into
Modules." *Communications of the ACM*, 15(12), 1053–1058.

Five pages, and among the most valuable five pages in the field. Two
decompositions of one program, compared. Read it; it is short, it is free, and the
criterion it gives you is the one worth carrying into Unit V.

Fowler, M. (2018). *Refactoring: Improving the Design of Existing Code* (2nd ed.).
Addison-Wesley.

A catalogue of transformations that change structure without changing behavior —
Extract Method being the one this chapter is about. The value is in the *when*:
each entry says what motivates the change, which is the judgment Section 14.1.1
says has no algorithm.

Martin, R. C. (2008). *Clean Code*. Prentice Hall. Chapters 3 and 17.

Recommended in Chapter 11 and relevant again. Chapter 17's catalogue of "code
smells" is essentially a list of seam signals.

## On testing

Myers, G. J., Sandler, C., & Badgett, T. (2011). *The Art of Software Testing*
(3rd ed.). Wiley.

The origin of equivalence partitioning and boundary value analysis. Try the
triangle exercise in Chapter 1 before reading the answer.

Beck, K. (2002). *Test-Driven Development: By Example*. Addison-Wesley.

The discipline argued for by working two examples end to end. Short. You do not
have to adopt TDD to benefit from watching someone let tests drive a design.

Osherove, R. (2013). *The Art of Unit Testing* (2nd ed.). Manning.

Practical and framework-focused. Good on naming, structure, and what makes a test
maintainable — the aspects that decide whether a suite survives contact with a
real project.

## Java specifics

The JUnit 5 User Guide.

Free and well written. Appendix B of this book covers enough to start; the guide
is where to go for parameterized tests, lifecycle methods, and assertions beyond
equality.

Bloch, J. (2018). *Effective Java* (3rd ed.). Addison-Wesley. Item 15,
"Minimize the accessibility of classes and members".

Parnas's information hiding, stated as Java practice. Read after Chapter 19.

## The counterargument

Hansson, D. H. (2014). "TDD is dead. Long live testing."

A well-argued objection to test-driven development from someone who ships large
systems, and the ensuing public conversation with Kent Beck and Martin Fowler is
worth reading in full. Included because this chapter takes a position and you
should hear the strongest version of the other one.

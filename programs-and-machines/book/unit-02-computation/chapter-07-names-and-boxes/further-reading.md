# Further Reading

## On naming

Martin, R. C. (2008). *Clean Code*. Prentice Hall. Chapter 2, "Meaningful Names".

Twenty pages on choosing names, and the best concentrated treatment of it. The
book as a whole is opinionated and not everyone agrees with all of it; this
chapter is close to uncontroversial.

Ottinger, T. (2009). "Ottinger's Rules for Variable and Class Naming."

Shorter and free, covering most of the same ground.

## On mutation and reasoning

Dijkstra, E. W. (1976). *A Discipline of Programming*. Prentice Hall.

Dijkstra's case for deriving programs rather than debugging them. Difficult, and
worth attempting after Chapter 9 rather than now. The introduction alone is
valuable.

Hughes, J. (1989). "Why Functional Programming Matters." *The Computer Journal*,
32(2), 98–107.

The clearest argument for the style that avoids mutation entirely. You do not
need to accept its conclusion to benefit from seeing the trade stated plainly by
someone who has thought about it hard.

## Reference

*The Java Language Specification*, Java SE 17 edition. Oracle.
Chapter 4 (types and variables), Chapter 6 (names and scope), Section 16
(definite assignment).

Section 16 is the formal statement of the definite-assignment rule that produced
the "might not have been initialized" error. It is more precise than any
explanation, including this chapter's, and reading a page of it is a good way to
see what a language specification is for.

## Style

*Google Java Style Guide*, and the older *Code Conventions for the Java
Programming Language* (Sun/Oracle).

Both freely available. Read one, adopt it, stop thinking about the questions it
settles. The value of a style guide is not that its choices are correct but that
they are made.

Bloch, J. (2018). *Effective Java* (3rd ed.). Addison-Wesley. Items 57 and 58.

"Minimize the scope of local variables" and "Prefer for-each loops to traditional
for loops". Item 57 is this chapter's argument, made in two pages by someone who
designed a large part of the Java libraries.

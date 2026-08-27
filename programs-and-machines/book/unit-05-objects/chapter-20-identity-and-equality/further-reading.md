# Further Reading

## The essential items

Bloch, J. (2018). *Effective Java* (3rd ed.). Addison-Wesley. Items 10, 11, 17,
and 50.

Item 10 on `equals` and Item 11 on `hashCode` are the definitive treatment,
including the symmetry problems that arise when a subclass adds a field — a case
this chapter avoided and which has no fully satisfactory answer. Item 17 is
immutability. Item 50, "Make defensive copies when needed", is Section 20.1.2.

If you read one thing after this chapter, read Item 10. It is longer than the
chapter and it earns the length.

## On abstraction and equality

Liskov, B., & Guttag, J. (2000). *Program Development in Java: Abstraction,
Specification, and Object-Oriented Design*. Addison-Wesley.

Chapter 5 introduces the abstraction function — the mapping from representation to
abstract value — which is what makes "equal" mean the right thing for a type whose
internals are hidden.

Baker, H. G. (1993). "Equal Rights for Functional Objects, or, The More Things
Change, The More They Are the Same." *ACM OOPS Messenger*, 4(4), 2–27.

An argument that the difficulty of equality comes from mutation, and that for
immutable objects the question largely dissolves. Idiosyncratic and rewarding.

## Reference

*The Java Language Specification*, Java SE 17 edition. Oracle. Section 15.21
(equality operators).

The `java.lang.Object` documentation for `equals` and `hashCode`.

The contracts are stated there, in the specification's own words, and they are
short. Worth reading once so that "the contract" stops being something you have
been told about and becomes something you have read.

The `java.util.Objects` documentation — `Objects.equals`, `Objects.hash`,
`Objects.requireNonNull`. Three small methods that remove most of the boilerplate
in this chapter.

## On immutability more widely

Okasaki, C. (1998). *Purely Functional Data Structures*. Cambridge University
Press.

What data structures look like when nothing can be modified — lists, queues, and
trees that produce new versions rather than changing. Harder than this book and a
good answer to "but surely immutability is impractical for real structures".

Goetz, B., et al. (2006). *Java Concurrency in Practice*. Addison-Wesley.
Chapter 3.

The thread-safety benefit that Section 20.2.3 could only assert. Read after
Chapter 31; it is where the largest argument for immutability actually lands.

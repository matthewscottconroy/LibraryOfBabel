# Further Reading

## On encapsulation

Bloch, J. (2018). *Effective Java* (3rd ed.). Addison-Wesley. Items 15–17.

Item 15, "Minimize the accessibility of classes and members", is Section 19.2.1's
argument stated as practice. Item 16, "In public classes, use accessor methods,
not public fields", covers the honest exception. Item 17, "Minimize mutability",
is where Chapter 20 is going.

Parnas, D. L. (1972). "On the Criteria To Be Used in Decomposing Systems into
Modules." *Communications of the ACM*, 15(12), 1053–1058.

Recommended in Chapters 14 and 16 and essential here. `private` is the mechanism;
Parnas gives the criterion for what to put behind it — the decisions likely to
change.

Liskov, B., & Guttag, J. (2000). *Program Development in Java: Abstraction,
Specification, and Object-Oriented Design*. Addison-Wesley.

Chapters 5 and 6 cover representation invariants and abstraction functions in
Java, by the person who originated the ideas. The best available treatment of what
this chapter is doing.

## On object design

Freeman, S., & Pryce, N. (2009). *Growing Object-Oriented Software, Guided by
Tests*. Addison-Wesley.

The source of the "tell, don't ask" framing in Section 19.1.3, worked out at
length. Opinionated, and the first four chapters are worth reading even if you
disagree with the rest.

West, D. (2004). *Object Thinking*. Microsoft Press.

An argument that most object-oriented code is procedural code in disguise —
classes as containers for data with the behaviour elsewhere. Overstated in places
and a useful corrective to the getter-and-setter habit.

## Historical

Dahl, O.-J., & Nygaard, K. (1966). "SIMULA — an ALGOL-Based Simulation Language."
*Communications of the ACM*, 9(9), 671–678.

Where objects appear. Short, and the motivation from simulation is stated plainly.

Kay, A. (1993). "The Early History of Smalltalk." *ACM SIGPLAN Notices*, 28(3),
69–95.

Kay's own account, including what he meant by object orientation and why he thinks
the term went wrong. Discursive and worth the time.

## Reference

*The Java Language Specification*, Java SE 17 edition. Oracle. Chapter 8,
"Classes"; Section 6.6 on access control; Section 8.8 on constructors.

Section 6.6 is the precise statement of the four levels, including the details of
`protected` that make it weaker than it looks.

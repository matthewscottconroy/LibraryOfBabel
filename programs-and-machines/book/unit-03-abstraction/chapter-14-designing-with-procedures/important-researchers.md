# People

## David Parnas (born 1941)

A Canadian-American computer scientist whose 1972 paper "On the Criteria To Be
Used in Decomposing Systems into Modules" is the founding document of this
chapter's subject.

Parnas took a small program, decomposed it two ways, and compared. The first
division was by *processing step* — the way most people decompose, following the
order in which work happens. The second was by **what each module hides**: each
one keeps a design decision private, so that changing that decision affects one
module.

The second was dramatically better, and the reason has nothing to do with the code
being shorter. It is that requirements change, and a module boundary drawn around
a decision *contains* the change, while a boundary drawn around a step spreads it.

That is **information hiding**, and it is the intellectual core of everything from
this chapter through Unit V. Section 14.1's talk of seams is the practical version;
Parnas supplies the criterion for choosing among them — *what is likely to change,
and can I put a boundary around it?*

He is also known for insisting, at some professional cost, that software claims be
justifiable — most publicly when he resigned from a Strategic Defense Initiative
advisory panel in 1985, arguing the software could not be made trustworthy.

## Kent Beck (born 1961)

An American programmer who developed test-driven development and, with others,
extreme programming.

The specific contribution behind Section 14.2.3 is the argument that tests should
be written *first*, and that doing so is a design activity rather than a
verification one. Writing the test first forces you to decide what the method's
interface is before you decide how it works, and makes untestable designs
uncomfortable at the moment they are created rather than afterwards.

Whether you adopt the discipline, the observation underneath is sound: the first
client of a method is its test.

Beck also wrote the original JUnit with Erich Gamma — the framework Appendix B
covers, and the ancestor of the xUnit family that now exists in every language.

## Barbara Liskov (born 1939)

Liskov appears a third time, and the reason belongs here.

Chapter 11 credited her with abstract data types. The specification-versus-
implementation distinction has a consequence for testing that is worth stating
directly: **test the specification, not the implementation.**

A test that checks what a method promises survives a rewrite of how it works. A
test that reaches inside and checks intermediate state breaks when the internals
change, which means it punishes exactly the improvement it should be protecting.

That distinction is why Section 14.2.1 frames a test as a contract executed rather
than as a script that exercises code.

## Glenford Myers (born 1946)

An IBM engineer whose *The Art of Software Testing* (1979) established most of
what Section 14.2.2 says about choosing cases: equivalence partitioning, boundary
value analysis, and the argument that testing is destructive rather than
confirmatory.

That last point is his most useful. Myers argued that a test designed to show a
program works is the wrong shape — you will unconsciously choose inputs it
handles. A test should be designed **to make the program fail**, and a test that
finds nothing is a disappointment rather than a success.

He also opened the book with an exercise: given a program that decides whether
three numbers form a triangle, write a set of test cases. Most experienced
programmers score under half of his fourteen. It is worth trying before reading
his list.

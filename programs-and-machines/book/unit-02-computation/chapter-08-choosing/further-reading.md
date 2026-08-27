# Further Reading

## Primary sources

Boole, G. (1854). *An Investigation of the Laws of Thought, on Which Are Founded
the Mathematical Theories of Logic and Probabilities*. Walton and Maberly.

Freely available, and stranger than you expect — the mathematics is embedded in a
philosophical argument about reasoning. Chapter 2 is where the algebra begins.
Worth twenty minutes to see what the notation looked like before engineers got
hold of it.

Shannon, C. E. (1937). *A Symbolic Analysis of Relay and Switching Circuits*.
Master's thesis, MIT. (Published in *Transactions of the AIEE*, 57(12), 713–723,
1938.)

Short, readable, and the origin of the field. The opening sections state the
correspondence between relay circuits and boolean algebra plainly enough that you
can follow them with this chapter's material and nothing else.

## Building from gates

Petzold, C. (1999). *Code: The Hidden Language of Computer Hardware and
Software*. Microsoft Press.

Recommended in Chapter 1 and recommended again. Chapters 11 through 14 build
gates, then an adder, then a latch, then memory, in exactly the sequence Section
8.1.2 sketches — but with the wiring drawn.

Nisan, N., & Schocken, S. (2005). *The Elements of Computing Systems: Building a
Modern Computer from First Principles*. MIT Press.

Known as *nand2tetris*, and the course materials are free online. You build a
computer starting from a NAND gate: logic, then an ALU, then memory, then a CPU,
then an assembler and a compiler. If Section 8.1.2 was the most interesting thing
in this chapter, do this course. It is the single best complement to this book
that exists.

Harris, D., & Harris, S. (2015). *Digital Design and Computer Architecture*
(2nd ed.). Morgan Kaufmann.

A proper textbook treatment, including Karnaugh maps and systematic
minimization.

## Logic itself

Smullyan, R. (1978). *What Is the Name of This Book?* Prentice-Hall.

Logic puzzles, arranged so that working through them teaches propositional logic
without announcing that it is doing so. Genuinely entertaining, and the knights-
and-knaves problems are excellent practice at exactly the reasoning Section 8.1.3
asks for.

## Java specifics

*The Java Language Specification*, Java SE 17 edition. Oracle.
Sections 15.23 and 15.24 (conditional AND and OR), Section 14.11 (`switch`).

The specification is explicit that `&&` and `||` do not evaluate the right operand
when the left settles the result. Worth reading the two paragraphs, because "the
language guarantees this" is different from "it happens to work".

Bloch, J. (2018). *Effective Java* (3rd ed.). Addison-Wesley. Item 34, "Use enums
instead of int constants".

The argument for the exhaustiveness checking described at the end of Section
8.2.3, made properly.

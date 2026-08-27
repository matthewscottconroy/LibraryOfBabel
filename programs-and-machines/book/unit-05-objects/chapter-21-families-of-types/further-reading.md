# Further Reading

**Barbara Liskov and Jeannette Wing, "A Behavioral Notion of Subtyping" (1994).**
The formal statement. It is a research paper and reads like one, but the four
obligations in Section 21.2.3 are stated there precisely and the examples are
approachable. Worth reading the introduction even if the formalism is not yet
comfortable.

**Barbara Liskov, "Data Abstraction and Hierarchy" (OOPSLA 1987).** The keynote.
Shorter and more direct than the paper, and it makes the argument that inheritance
is about substitutability rather than reuse before anyone wanted to hear it.

**Joshua Bloch, *Effective Java*, third edition.** Item 18, "Favor composition over
inheritance," and Item 19, "Design and document for inheritance or else prohibit
it," are the two most useful pages you can read after this chapter. Item 10 on
`equals` connects back to Chapter 20 and forward to the substitution obligations.

**Ole-Johan Dahl and Kristen Nygaard, "The Development of the SIMULA Languages"
(1978).** The inventors describing what they were trying to do and why classes came
out of it. A good corrective to the impression that object orientation was designed
in the abstract; it came from needing to simulate ships in a harbor.

**The Java Virtual Machine Specification, chapter 6.** The definitions of
`invokevirtual`, `invokeinterface`, `invokespecial` and `invokestatic`. Dry, but it
is the authority for everything Section 21.2.2 describes, and the method-resolution
rules are stated exactly.

**Aleksey Shipilëv, "The Black Magic of (Java) Method Dispatch."** A careful,
measured account of what the JIT actually does with monomorphic, bimorphic and
megamorphic call sites, with benchmarks done properly. Read it if Exercise 21.11
made you curious, and read it before trusting any timing you write yourself.

**Alan Kay, "The Early History of Smalltalk" (1993).** What he meant by
object-oriented, in his own words, and how far it is from class hierarchies.
Entertaining, and it will change how you read Chapter 22.

# Further Reading

## Abstract data types

Liskov, B., & Zilles, S. (1974). "Programming with Abstract Data Types."
*ACM SIGPLAN Notices*, 9(4), 50–59.

The founding paper. Short, and the argument for language enforcement rather than
convention is stated plainly.

Parnas, D. L. (1972). "On the Criteria To Be Used in Decomposing Systems into
Modules." *Communications of the ACM*, 15(12), 1053–1058.

Recommended in Chapter 14 and essential here. Five pages.

Liskov, B., & Guttag, J. (2000). *Program Development in Java: Abstraction,
Specification, and Object-Oriented Design*. Addison-Wesley.

Chapter 5 covers representation invariants and abstraction functions properly,
in Java, by the person who originated the ideas. The closest thing to a full
treatment of Section 16.1 that exists.

## On invariants in practice

Hoare, C. A. R. (1972). "Proof of Correctness of Data Representations."
*Acta Informatica*, 1(4), 271–281.

Where the representation invariant is formalized, together with the *abstraction
function* — the mapping from a representation to the abstract value it stands for.
The pair is the complete story, and this chapter told half of it.

## Wrappers, boxing, and null

Bloch, J. (2018). *Effective Java* (3rd ed.). Addison-Wesley. Items 55, 61, and
62.

Item 61, "Prefer primitive types to boxed primitives", is Section 16.2.2 in two
pages, by the person who added the feature. Item 55 covers `Optional` and, usefully,
when *not* to use it.

Hoare, C. A. R. (2009). "Null References: The Billion Dollar Mistake." Talk at
QCon London.

The source of the quotation. Twenty minutes if you can find the recording; the
transcript circulates widely.

*The Java Language Specification*, Java SE 17 edition. Oracle. Section 5.1.7.

The exact rules for boxing conversion, including the caching guarantee for values
between −128 and 127 — which is specified behavior rather than an implementation
accident, and therefore something you can rely on and should still not use.

## Where the problems are solved

Kotlin's documentation on null safety.

Short and clear, and reading it is the fastest way to see what a type system can do
about the problem when it is not carrying thirty years of compatibility.

Klabnik, S., & Nichols, C. *The Rust Programming Language*. Chapter 6, "Enums and
Pattern Matching".

The `Option` type, and why a language with no null is not thereby inconvenient.
Free online.

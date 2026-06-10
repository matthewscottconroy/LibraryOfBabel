# References and Primary Sources

## Foundational Texts

**Per Martin-Löf.** *Intuitionistic Type Theory.* Bibliopolis, Naples, 1984. (Notes by Giovanni Sambin from the 1980 Padova lectures.)
The foundational document of Martin-Löf Type Theory (MLTT), presenting Π types, Σ types, identity types, natural numbers, and the universe hierarchy in their mature form. Martin-Löf's philosophical introduction — arguing that judgments, not propositions, are the basic notion of type theory — is as important as the formal rules. Every major proof assistant in use today (Coq, Agda, Lean) is built on the formal system of these lecture notes.

**Thierry Coquand and Gérard Huet.** "The Calculus of Constructions." *Information and Computation* 76(2–3): 95–120, 1988.
The paper introducing the Calculus of Constructions (CoC), which unifies Martin-Löf type theory with System F in a single framework organized by the *lambda cube*. CoC is the core type theory of Coq, and this paper establishes its key meta-theoretic properties: type-checking is decidable, strong normalization holds, and the system is consistent. The lambda cube diagram — showing eight typed lambda calculi corresponding to different combinations of term-over-type, type-over-type, and type-over-term dependencies — is introduced here.

**Nicolaas Govert de Bruijn.** "The Mathematical Language AUTOMATH, Its Usage, and Some of Its Extensions." *Symposium on Automatic Demonstration*, Lecture Notes in Mathematics 125, pp. 29–61. Springer, 1970.
De Bruijn's presentation of AUTOMATH, the first computer system for checking full mathematical proofs. AUTOMATH was designed in the late 1960s and predates both Martin-Löf's and Coquand's work; it is dependent type theory avant la lettre. This paper describes the type-theoretic language used to express proofs and the checking algorithm — the first verified mathematical proofs on a computer.

**Bengt Nordström, Kent Petersson, and Jan M. Smith.** *Programming in Martin-Löf's Type Theory: An Introduction.* Oxford University Press, 1990. (Available free online.)
The most accessible textbook presentation of MLTT, with extensive programming examples. Develops Π types, Σ types, the identity type, natural numbers, lists, and trees, with a consistent focus on the computational content of type-theoretic proofs. A good bridge between the formalism and practice.

**Benjamin C. Pierce (ed.).** *Advanced Topics in Types and Programming Languages.* MIT Press, 2005.
Chapter 2 by Luo covers the Calculus of Constructions and the Unified Theory of Dependent Types; Chapter 1 by Aspinall and Hofmann covers dependent types in detail with attention to implementation. A useful companion to the primary sources for readers with a programming language background.

---

## Seminal Papers

**Nicolaas Govert de Bruijn.** "A Survey of the Project AUTOMATH." In *To H.B. Curry: Essays on Combinatory Logic, Lambda Calculus and Formalism*, eds. Seldin and Hindley. Academic Press, 1980.
De Bruijn's retrospective on the AUTOMATH project, which was the first machine-checkable formal proof system and which introduced many of the concepts of dependent type theory (including what are now called *de Bruijn indices* — a representation of variables by their position in the binding context, eliminating name conflicts).

**Per Martin-Löf.** "An Intuitionistic Theory of Types: Predicative Part." In *Logic Colloquium '73*, eds. Rose and Shepherdson, pp. 73–118. North-Holland, 1975.
The first published version of MLTT, introducing the four forms of judgment and the type-theoretic rules for $\Pi$, $\Sigma$, finite types, and $\mathbb{N}$. This paper (not the 1984 Bibliopolis notes) is where Martin-Löf first presented identity types and the $J$ elimination rule — the rule that characterizes the identity type and, in HoTT, becomes the foundation for path induction.

**Christine Paulin-Mohring.** "Inductive Definitions in the System Coq." *Typed Lambda Calculi and Applications (TLCA 1993)*, Lecture Notes in Computer Science 664, pp. 328–345. Springer, 1993.
The paper establishing inductive types as a first-class feature of the Calculus of Constructions, forming what is now called the Calculus of Inductive Constructions (CIC) — the actual type theory of Coq. Paulin-Mohring showed how to add $\mathbb{N}$, lists, trees, and other inductive types with their recursors within CoC, and proved that the extension is conservative (adds no new non-inductive theorems).

**Conor McBride and James McKinna.** "The View from the Left." *Journal of Functional Programming* 14(1): 69–111, 2004.
Introduces the concept of "views" — a programming pattern in dependent type theory that separates the definition of a type from the way you pattern-match on it. The paper demonstrates how dependent pattern matching and the `with` clause (now standard in Agda) allow expressive, type-correct programs with non-trivial dependent types, including the vector operations central to this chapter.

**Thierry Coquand and Christine Paulin-Mohring.** "Inductively Defined Types." *COLOG-88: International Conference on Computer Logic*, Lecture Notes in Computer Science 417, pp. 50–66. Springer, 1990.
The first paper on inductive types in the CoC setting, establishing the formal framework that became the Calculus of Inductive Constructions. Introduces the general schema for inductive type definitions and the computation rules for their recursors.

**Frank Pfenning and Conal Elliott.** "Higher-Order Abstract Syntax." *PLDI 1988*, pp. 199–208. ACM, 1988.
Introduces the use of higher-order abstract syntax (HOAS) — representing binders in the object language using binders in the meta-language — as a technique for implementing proof assistants. This became central to the LF logical framework and Twelf, and is deeply connected to dependent types.

---

## Textbooks and Modern Treatments

**Per Martin-Löf.** *Intuitionistic Type Theory.* Bibliopolis, 1984. (See above.)
The primary source. Essential reading for anyone working in dependent type theory or HoTT.

**The Univalent Foundations Program.** *Homotopy Type Theory: Univalent Foundations of Mathematics.* Institute for Advanced Study, 2013. (Free at homotopytypetheory.org)
Chapter 1 of the HoTT Book develops MLTT in full — Π types, Σ types, identity types, universes, inductive types — presenting it as the foundation for everything that follows. This is the canonical modern reference for the formal rules in a notation consistent with HoTT.

**Adam Chlipala.** *Certified Programming with Dependent Types.* MIT Press, 2013. (Free online at adam.chlipala.net/cpdt/)
A practical guide to programming with dependent types in Coq, with an emphasis on proof automation and certified software. Shows how $\mathsf{Vec}$, $\mathsf{Fin}$, sorted lists, and other indexed types are used in practice to write programs with machine-checked correctness proofs.

**Ana Bove, Peter Dybjer, and Ulf Norell.** "A Brief Overview of Agda — A Functional Language with Dependent Types." *Theorem Proving in Higher Order Logics (TPHOLs 2009)*, Lecture Notes in Computer Science 5674, pp. 73–78. Springer, 2009.
A concise introduction to Agda as a programming language, emphasizing the features that arise specifically from dependent types: pattern matching on indices, the `with` construct, universe levels, and the termination checker.

**Edwin Brady.** *Type-Driven Development with Idris.* Manning Publications, 2017.
A practical introduction to dependent types as used in programming (via Idris, a language designed for practical type-driven development). Excellent for building the intuition that dependent types are a natural extension of good programming practice, not an obscure research curiosity.

---

## Online Resources and Lecture Notes

**Andrej Bauer and Thierry Coquand.** *Notes on Dependent Type Theory.* Available at andrej.com and related course pages.
Bauer's lecture notes are mathematically careful and well-motivated, covering the same material as this chapter from a slightly different angle. Particularly good on the formal rules and their categorical semantics.

**Ulf Norell.** *Dependently Typed Programming in Agda.* Available at www.cse.chalmers.se/~ulfn/papers/afp08/tutorial.pdf
The standard tutorial for Agda, written by its main author. Covers Vec, Fin, equality proofs, and dependently typed programs with a light touch. Prerequisite: some familiarity with Haskell.

**Lean 4 Documentation.** *Theorem Proving in Lean 4.* leanprover.github.io/theorem_proving_in_lean4/
The official Lean 4 tutorial, organized around dependent types and the Curry-Howard correspondence. Chapters 7–9 on inductive types, structures, and type classes are directly relevant to the material of this chapter.

**"Programming Language Foundations in Agda" (PLFA).** plfa.inf.ed.ac.uk
An online textbook teaching programming language theory entirely in Agda, using dependent types for both specification and proof. The chapters on lambda calculus, type systems, and operational semantics are directly relevant, and the entire book is a working Agda formalization.

**Coq Reference Manual.** coq.inria.fr/documentation
The definitive reference for Coq's type theory (CIC). The chapter on the Calculus of Inductive Constructions gives the formal rules for inductive types, Π types, and the universe hierarchy as implemented in Coq.

---

## Historical Context

Dependent type theory has a dual origin: from logic and from engineering. On the logic side, Per Martin-Löf developed his type theory in the 1970s as a foundational system for constructive mathematics — a system where every proof is a construction and every proposition has computational content. His motivation was philosophical: to give a coherent account of what mathematical objects are and how we come to know them, without appealing to a set-theoretic universe. The formal system emerged through several iterations (1971, 1973, 1975, 1979, 1982, culminating in the 1984 Bibliopolis notes), each refining the treatment of universes, identity types, and the elimination principles for inductive types.

On the engineering side, Nicolaas de Bruijn built AUTOMATH at Eindhoven in the late 1960s — independently of Martin-Löf and predating his published work. De Bruijn was a mathematician who wanted a language for writing down proofs in a way that a computer could check. AUTOMATH was dependent type theory implemented in practice; it was used to formalize significant portions of Landau's *Grundlagen der Analysis* and Knuth's Art of Computer Programming. De Bruijn indices (representing variables by their distance from their binder) and the concept of a definitional equality (two terms are equal if they reduce to the same normal form) both come from AUTOMATH.

The Calculus of Constructions (1988) brought these two traditions together: Coquand and Huet showed how to combine Martin-Löf's dependent types with Girard's System F in a single clean framework with decidable type checking. Paulin-Mohring's addition of inductive types to CoC (forming CIC) gave Coq its current core. Since then, Agda, Lean, Idris, and many other systems have been built on dependent type theory, and the field has grown from a foundational curiosity to a practical tool for verified software and formalized mathematics. The connection to homotopy theory — discovered by Voevodsky around 2006 and developed in the HoTT program — opened an entirely new chapter: the same formal system that was designed for constructive proofs turns out to have a natural geometric interpretation where types are spaces, terms are points, and identity types are path spaces.

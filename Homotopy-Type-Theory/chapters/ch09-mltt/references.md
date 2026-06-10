# References and Primary Sources

## Foundational Texts

**Per Martin-Löf.** *An Intuitionistic Theory of Types: Predicative Part.* In H.E. Rose and J.C. Shepherdson (eds.), *Logic Colloquium '73*, North-Holland, 1975.
The original systematic presentation of MLTT. Martin-Löf introduces the four judgment forms, the Π and Σ type formers, and the universe in a single unified framework. This paper is the founding document of the entire field.

**Per Martin-Löf.** *Intuitionistic Type Theory* (Notes by Giovanni Sambin of lectures given in Padova, 1980). Bibliopolis, Naples, 1984.
The most widely read exposition of MLTT; these "Bibliopolis notes" present the full theory including W types, the identity type with the J rule, and the philosophical grounding in constructive mathematics. Essential reading — dense but extraordinarily clear once you have context.

**Bengt Nordström, Kent Petersson, and Jan M. Smith.** *Programming in Martin-Löf's Type Theory: An Introduction.* Oxford University Press, 1990. Freely available at [https://www.cse.chalmers.se/research/group/logic/book/](https://www.cse.chalmers.se/research/group/logic/book/).
The standard introductory text for MLTT as a programming language. Explains the theory from the programmer's perspective, with emphasis on how the type formers correspond to data structures and how the J rule enables proof by pattern matching.

**The Univalent Foundations Program.** *Homotopy Type Theory: Univalent Foundations of Mathematics.* Institute for Advanced Study, 2013. Freely available at [https://homotopytypetheory.org/book/](https://homotopytypetheory.org/book/).
The HoTT Book. Chapter 1 gives the most modern and accessible introduction to MLTT, presenting all four judgment forms, all the type formers, and the identity type in a way that is already primed for the homotopy-theoretic interpretation.

**Per Martin-Löf.** *Truth of a Proposition, Evidence of a Judgment, Validity of a Proof.* *Synthese* 73, 1987.
The philosophical paper in which Martin-Löf articulates why there are four forms of judgment (rather than, say, just typing assertions), grounding the theory in the epistemology of constructive mathematics.

---

## Seminal Papers

**Per Martin-Löf.** "An Intuitionistic Theory of Types." *Logic Colloquium '73*, 1975.
The paper that defined the field. Martin-Löf presents what we now call intensional MLTT, introduces the W type as a general well-founded recursion principle, and motivates the entire enterprise from the constructive standpoint. All modern proof assistants based on dependent types are descendants of this work.

**Per Martin-Löf.** "Constructive Mathematics and Computer Programming." *Logic, Methodology and Philosophy of Science VI*, North-Holland, 1982.
A highly influential paper on the Curry-Howard correspondence as embodied in MLTT: propositions are types, proofs are programs, and the distinction between definitional and propositional equality is the distinction between computation and proof.

**Per Martin-Löf.** "On the Meanings of the Logical Constants and the Justifications of the Logical Laws." *Nordic Journal of Philosophical Logic* 1(1), 1996.
Develops the proof-theoretic semantics for type theory, explaining why the elimination rules (and in particular, the J rule for the identity type) are justified by the introduction rules through the principle of local reducibility.

**Thierry Coquand.** "An Analysis of Girard's Paradox." *LICS*, 1986.
Proves that Martin-Löf's original 1971 type theory (with Type : Type) is inconsistent via a version of the Burali-Forti paradox. This motivates the stratified universe hierarchy in modern MLTT and is essential background for understanding why universes are set up the way they are.

**Thierry Coquand and Gérard Huet.** "The Calculus of Constructions." *Information and Computation* 76(2–3), 1988.
Introduces the Calculus of Constructions (CoC), closely related to MLTT, and proves its key meta-theoretic properties. Coq is built on an extension of CoC; understanding CoC illuminates the design choices in modern proof assistants.

**Robert L. Constable et al.** "Implementing Mathematics with the NuPRL Proof Development System." Prentice Hall, 1986.
The NuPRL system is based on *extensional* MLTT; this book explains the tradeoffs between intensional and extensional type theory from a systems perspective, showing concretely where extensionality makes some things easier at the cost of decidable type checking.

**Michael Hedberg.** "A Coherence Theorem for Martin-Löf's Type Theory." *Journal of Functional Programming* 8(4), 1998.
Proves Hedberg's theorem: any type with decidable equality satisfies UIP (Uniqueness of Identity Proofs). This is one of the earliest results showing that the identity type has genuinely nontrivial structure in general.

---

## Textbooks and Modern Treatments

**Simon Thompson.** *Type Theory and Functional Programming.* Addison-Wesley, 1991. Freely available online.
An accessible introduction to MLTT from a functional programming perspective. Covers the type formers systematically and includes substantial material on programming in MLTT (as opposed to just proving theorems). Good for readers coming from a CS background.

**Bengt Nordström, Kent Petersson, and Jan M. Smith.** *Programming in Martin-Löf's Type Theory.* Oxford, 1990.
(See Foundational Texts above.) This deserves double mention: as a textbook, it is the most systematic reference for the rules themselves — all the inference rules are listed in an appendix, making it invaluable as a reference even after you've read it once.

**Pierre-Louis Curien, Hugo Herbelin, and colleagues.** *Proofs and Types.* Jean-Yves Girard, Yves Lafont, Paul Taylor. Cambridge, 1989. Freely available online.
While focused on Girard's System F and linear logic, this book gives essential background in the proof-theory / type-theory duality (Curry-Howard) that underpins the interpretation of MLTT.

**Edwin Brady.** *Type-Driven Development with Idris.* Manning, 2017.
A modern programming-oriented introduction to dependent types. The Idris language is descended from MLTT; this book makes the judgment system and type formers concrete through programming exercises. Excellent for readers who want to write actual code.

**Ulf Norell.** *Towards a Practical Programming Language Based on Dependent Type Theory.* PhD thesis, Chalmers University, 2007. Freely available.
The thesis that defines Agda. Chapter 2 gives a clean and precise account of the core MLTT on which Agda is based, including the treatment of definitional equality and the universe hierarchy.

---

## Online Resources and Lecture Notes

**nLab: Martin-Löf Dependent Type Theory.**
[https://ncatlab.org/nlab/show/Martin-Löf+dependent+type+theory](https://ncatlab.org/nlab/show/Martin-L%C3%B6f+dependent+type+theory)
The nLab article on MLTT is dense but comprehensive, connecting the type-theoretic presentation to categorical semantics and pointing to the primary literature. Use it as a crossroads document when following up on specific points.

**Agda Documentation and Standard Library.**
[https://agda.readthedocs.io/](https://agda.readthedocs.io/)
Agda is the proof assistant most directly descended from Martin-Löf's original system. Reading the standard library source code — particularly `Agda.Builtin.Equality` and `Data.Nat` — is one of the most effective ways to see MLTT rules in their fully explicit form.

**William Lovas and Frank Pfenning.** *A Bidirectional Refinement Type System for LF.* LFMTP, 2010; and the Twelf documentation at [http://twelf.org/](http://twelf.org/).
The Edinburgh Logical Framework (LF) is a type theory specifically designed for representing other type theories, including MLTT. The Twelf implementation mechanically checks meta-theoretic properties of MLTT. Essential for anyone interested in the meta-theory.

**Andrej Bauer and Peter LeFanu Lumsdaine.** *Setoids in Type Theory.* (Various; see also Sozeau-Tabareau's Coq setoid library.)
Setoids (types equipped with a propositional equivalence relation, used when you want to quotient by something) are the standard workaround for the absence of quotient types in plain MLTT. The setoid library in Coq/Agda implements this pattern. Understanding setoids is essential for working in an intensional type theory.

**Lars Birkedal and Aleš Bizjak.** *Lecture Notes on Homotopy Type Theory.* Aarhus University, 2021. Available online.
Modern lecture notes that present MLTT and HoTT together, emphasizing the places where MLTT's identity type becomes the source of higher structure. A clean presentation for readers who want to go from MLTT to HoTT as directly as possible.

---

## Historical Context

Martin-Löf's type theory did not emerge in isolation. It grew out of two intellectual traditions: the constructive mathematics of Brouwer, Heyting, and Kolmogorov (which insisted that mathematical objects must be constructively given, and proofs must be explicit witnesses); and the proof theory of Gentzen and Prawitz (which studied natural deduction and the relationship between introduction and elimination rules). Martin-Löf attended Prawitz's seminars and was deeply influenced by the idea that the meaning of a logical connective is given by its introduction rules — a principle he extended to the identity type by making its introduction rule (reflexivity) the basic constructor and deriving all other facts about equality from the J-rule.

The development of MLTT went through several stages. The original 1971 system (presented in unpublished lectures) had Type : Type, which Coquand showed to be inconsistent in 1986. The 1975 paper introduced the predicative universe hierarchy to avoid this. The 1984 Bibliopolis notes represented the theory in its most developed classical form. In the 1990s, the intensional/extensional distinction became a central research question: Martin-Löf himself at times favored extensional MLTT (where the identity type is trivial but type checking becomes undecidable), while the development of Coq and Agda committed firmly to the intensional version. The discovery in the 2000s (by Hofmann-Streicher's groupoid model, then by Voevodsky's simplicial set model) that intensional MLTT models were naturally homotopy-theoretic, and that Univalence was consistent, transformed the field entirely and gave MLTT a new geometric meaning that Martin-Löf had not anticipated. The HoTT Book (2013) formalized this synthesis, and the subsequent cubical type theory program (Coquand, Huber, Mörtberg, and collaborators) has produced new type theories in which Univalence holds definitionally rather than as an axiom.

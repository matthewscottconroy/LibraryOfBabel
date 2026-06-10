# Important Figures

## Per Martin-Löf (1942–present)
*Logician and philosopher of mathematics; founder of Martin-Löf Type Theory*

Per Martin-Löf was born in Sweden and studied mathematics and logic, becoming a student of Andrei Kolmogorov in Moscow before returning to Stockholm and eventually taking a chair at Stockholm University. His intellectual formation combined deep knowledge of mathematical logic, constructive mathematics in the tradition of Brouwer and Heyting, and proof theory in the tradition of Gentzen and Prawitz. He came to type theory not as an exercise in computer science but as a philosophical project: to give a complete and rigorous account of what it means to do constructive mathematics.

The theory that bears his name went through several published and unpublished versions. The original 1971 system was shown inconsistent by Thierry Coquand in 1986 because it had Type : Type. The 1975 paper introduced a cumulative universe hierarchy to repair this. The 1980 Padova lectures, transcribed by Giovanni Sambin and published in 1984 as the *Bibliopolis notes*, gave the most polished classical form of the theory. In this version, the four judgment forms, the W type (well-founded trees), the identity type with the J elimination rule, and the universe hierarchy are all present and motivated with philosophical care. Martin-Löf's innovation of the identity type — treating equality itself as a type whose elements are proofs of equality — was the seed that would eventually grow into HoTT: the J rule is exactly the axiom that allows synthetic homotopy theory.

Martin-Löf has continued to write on the philosophy of mathematics and proof theory throughout his career. His 1987 paper "Truth of a Proposition, Evidence of a Judgment" remains the definitive philosophical defense of the four-judgment framework. His later lecture "An Intuitionistic Theory of Types: Predicative Part" (the 1998 expanded version) addresses the predicativity issues that arise from impredicative universes. His influence on the next generation of type theorists — Thierry Coquand, Ulf Norell, and their students — is immense and direct.

---

## Bengt Nordström (1947–present)
*Computer scientist; lead author of the standard MLTT programming textbook*

Bengt Nordström worked at Chalmers University of Technology in Gothenburg and was one of the principal architects of the ALF and Agda proof assistants, the direct forerunners of modern Agda. Together with Kent Petersson and Jan M. Smith, he authored *Programming in Martin-Löf's Type Theory* (Oxford University Press, 1990), which remains the standard reference for MLTT as a formal system for programming. The book translates Martin-Löf's philosophical and logical presentation into a working programming language, showing how to write correct-by-construction programs whose types express their specifications.

Nordström's specific contribution in the book is the systematic treatment of all type formers in the "FIEC" presentation style: formation, introduction, elimination, and computation rules presented uniformly. This systematization made MLTT pedagogically tractable and influenced every proof assistant designed after 1990. His work also emphasized the *definitional* character of the computation rules — the fact that the $\beta$-reduction rules for the recursors are definitional (not just propositional) equalities, which is what makes proof assistants based on MLTT capable of computing answers automatically.

---

## Kent Petersson (1948–2008)
*Computer scientist; contributor to MLTT and the programming-in-type-theory tradition*

Kent Petersson was a colleague of Nordström at Chalmers and co-author of *Programming in Martin-Löf's Type Theory*. His work focused on the practical aspects of implementing dependent type theories: how to represent contexts, how to check definitional equality efficiently, and how to implement the universe hierarchy without performance collapse. Petersson contributed substantially to the design of the ALF system, which directly preceded Agda.

Petersson's contribution to the joint book was particularly in the formalization of substitution and context extension: the careful treatment of how dependent types behave under substitution is one of the technical difficulties in implementing MLTT, and Petersson's influence is visible in every subsequent type theory implementation. The "simultaneous substitution" and "hereditary substitution" techniques in current proof assistants trace back to this line of work.

---

## Jan M. Smith (dates not publicly available; active 1980s–2000s)
*Computer scientist and logician; contributor to the MLTT programming tradition*

Jan M. Smith was a third co-author of *Programming in Martin-Löf's Type Theory* and contributed the chapters on subset types and setoids — the mechanisms by which MLTT handles quotients and propositions without a separate Prop universe. The subset type $\{x : A \mid P(x)\}$, whose elements are elements of $A$ satisfying the predicate $P$, is an MLTT-internal way to carve out a sub-type without leaving the system. This is a precursor to the modern treatment of h-propositions in HoTT.

Smith's work on the setoid interpretation is particularly relevant to the chapter on intensional vs. extensional MLTT. When working in intensional MLTT and needing quotient types, the standard pattern is to work with setoids — pairs $(A, \sim)$ of a type and a propositional equivalence relation — with all functions required to preserve the equivalence. Smith made this pattern precise and showed it is powerful enough to recover most of constructive mathematics.

---

## Giovanni Sambin (1948–present)
*Logician; transcribed Martin-Löf's Padova lectures*

Giovanni Sambin is best known in the MLTT community as the transcriber of the 1980 Padova lectures that became the *Bibliopolis notes*. His role was not merely clerical: Sambin worked closely with Martin-Löf to ensure mathematical accuracy, and the text reflects both Martin-Löf's ideas and Sambin's own deep knowledge of proof theory and formal logic. Sambin's own research encompasses formal topology — a point-free approach to topology that can be developed entirely within MLTT — and he has contributed extensively to the constructive mathematics program.

His influence on MLTT is subtle but lasting: the precise formulation of the rules in the Bibliopolis notes, which have been reprinted and translated repeatedly, shaped how subsequent generations learned the theory. The philosophical introductions to each section, which motivate each type former constructively before giving its rules, reflect Sambin's conviction that formal rules should be understood, not just memorized.

---

## Jan von Plato (1951–present)
*Logician and proof theorist; structural proof theory*

Jan von Plato is a professor at the University of Helsinki and one of the foremost proof theorists of his generation. His work on structural proof theory — the study of normal forms for proofs, cut elimination, and the relationship between different styles of proof calculus — is directly relevant to MLTT because the J rule can be seen as an elimination rule in a Gentzen-style natural deduction calculus, and understanding its normalization behavior requires exactly the tools of structural proof theory.

Von Plato's book *Structural Proof Theory* (with Sara Negri, Cambridge 2001) develops the frameworks for understanding why derivations in MLTT are well-founded and why the computation rules correspond to cut-elimination steps. His analyses of identity in natural deduction — asking what the right introduction and elimination rules for equality are in a Gentzen system — informed the understanding of when the J rule is the "correct" and "complete" elimination rule for the identity type.

---

## Robert L. Constable (1943–present)
*Computer scientist; creator of NuPRL and extensional type theory*

Robert Constable is a professor at Cornell University and the creator of the NuPRL (Nuprl Proof Refinement Logic) proof development system, the first large-scale proof assistant based on dependent type theory (predating Coq and Agda). NuPRL is based on *extensional* MLTT: in extensional type theory, the identity type is trivial (all proofs of a given identity are definitionally equal, i.e., UIP holds), and moreover, the *reflection rule* holds: if $p : a =_A b$, then $a$ and $b$ are definitionally equal.

Constable's extensional system makes some theorems easier to state and some programs easier to write: you never have to carry around identity proofs, because they're all trivially equal. The cost is that type checking becomes undecidable — the reflection rule can encode arbitrary computation. NuPRL handles this by using a tactic-based system where the user guides the type checker, rather than checking fully automatically. The contrast between NuPRL's extensional approach and Agda/Coq's intensional approach runs throughout the practical literature on dependent types, and understanding Constable's design philosophy illuminates why the intensional choice was ultimately made by most modern systems — and why HoTT builds on the intensional version.

Constable's team produced important results in the application of dependent type theory to verified programming: the *Implementing Mathematics with the NuPRL Proof Development System* book (1986) showed that large-scale mathematics could be formalized in a dependent type theory, setting the agenda for what Lean's Mathlib would eventually accomplish in the intensional setting.

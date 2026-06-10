# Important Thinkers: Chapter 8

## Per Martin-Löf (1942–)

No figure is more central to this chapter than Per Martin-Löf, the Swedish logician who created the type theory that bears his name. Martin-Löf trained as a statistician and probabilist — his early work was on von Mises' frequency theory of probability — before turning to mathematical logic in the late 1960s under the influence of Dag Prawitz.

His 1971 paper "A Theory of Types" introduced a dependent type theory with Type : Type. It was shown inconsistent by Girard's paradox almost immediately. His 1975 "An Intuitionistic Theory of Types" corrected this with a universe hierarchy and introduced the key features of what we now call MLTT: dependent products, dependent sums, and natural numbers with their eliminators. The 1984 Bibliopolis lectures (published as *Intuitionistic Type Theory*) are the standard reference for the classical formulation.

What makes Martin-Löf distinctive is not just technical achievement but philosophical commitment. He developed MLTT explicitly as a *foundation for constructive mathematics* — a complete alternative to set theory that takes computation seriously as part of mathematical meaning. His extensive philosophical essays on the meaning explanations (the "meaning of a judgment," "the meanings of the logical constants") show a thinker for whom type theory is not just a formal system but a philosophical position about the nature of mathematical knowledge.

## Jean-Yves Girard (1947–)

Girard's 1972 PhD thesis introduced System F (independently discovered by John Reynolds, who called it the polymorphic lambda calculus). More relevant to this chapter, Girard showed in the same period that Martin-Löf's original type theory with Type : Type was inconsistent — a result now called Girard's paradox, with a simplified formulation by Antonius Hurkens in 1995.

Girard also developed *linear logic* (1987), a resource-sensitive logic that counts uses of propositions, and *Geometry of Interaction*, a program that interprets proofs as geometric objects. These later developments are less directly relevant to HoTT but show the range of a mind that consistently discovered paradoxes and impossibility results — and then figured out how to construct consistent systems in their wake.

## Thierry Coquand (1961–) and Christine Paulin-Mohring

The Calculus of Constructions (CoC), introduced by Thierry Coquand and Gérard Huet in 1988, unified dependent types and polymorphism in a single system. Coquand, now at Chalmers, went on to develop the Calculus of Inductive Constructions (CIC) with Christine Paulin-Mohring — the formal system underlying the Coq proof assistant (now called Rocq).

The crucial addition was inductive types: Paulin-Mohring worked out how to add inductive type definitions to CoC in a controlled way, with the positivity checker ensuring consistency. This made Coq practical — you could define ℕ, List, and other data structures natively, not just encode them using Church numerals in the style of System F.

Coquand has also made fundamental contributions to cubical type theory (with Simon Huber, Cyril Cohen, and Anders Mörtberg), a computational interpretation of HoTT that avoids the univalence axiom by making it a theorem of the system.

## William Howard (1926–)

Howard's 1969 unpublished note "The formulae-as-types notion of construction" (finally published in 1980 in a Festschrift for Curry) established the formal Curry-Howard correspondence between natural deduction proofs and typed lambda calculus terms. The principle that proofs are programs was implicit in Heyting's intuitionistic semantics and in Kleene's realizability, but Howard made it explicit and syntactic.

Howard intended the note as a technical observation, not a manifesto. The philosophical implications were drawn out by others. The extension to dependent types — full Curry-Howard — requires the dependent function and pair types of MLTT and was developed gradually through the 1970s and 1980s.

## Ulf Norell and the Agda Team

Agda, the dependently typed proof assistant most used for HoTT development, was designed by Ulf Norell as his 2007 PhD thesis. Agda inherits from the earlier system ALF (Another Logical Framework) and from Cayenne (a dependently typed programming language by Lennart Augustsson). But Norell's redesign made Agda practical: the interactive editing mode (holes, goal display, proof search) makes developing dependently typed proofs genuinely feasible.

The Agda standard library and the cubical Agda library (developed by Coquand, Mörtberg, and collaborators) are the main environments in which HoTT is developed today. The `--without-K` flag disables Axiom K, allowing the development of proper HoTT where Uniqueness of Identity Proofs fails.

## The Lambda Cube: Henk Barendregt

The *lambda cube* — a systematic organization of type systems by which of three axes of generalization they include — was introduced by Henk Barendregt in 1991. The eight vertices of the cube correspond to eight type systems, with the Calculus of Constructions at the "most expressive" corner. The framework clarified the relationships among System F (second-order polymorphism), System Fω (higher-order type operators), and the dependent type systems (Automath-style and LF-style).

Barendregt's comprehensive 1984 book *The Lambda Calculus: Its Syntax and Semantics* remains the standard reference for the untyped lambda calculus. His work on the lambda cube helped establish what features are needed for full dependent type theory and provided a taxonomy that continues to organize the field.

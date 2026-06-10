# References and Primary Sources

## Foundational Texts

- **Per Martin-Löf.** "An Intuitionistic Theory of Types: Predicative Part." In *Logic Colloquium '73*, edited by H.E. Rose and J.C. Shepherdson, North-Holland, 1975. The paper that introduced identity types into type theory as an inductive type, with the J elimination rule and the reflexivity constructor. This is the original source for all of this chapter's material; every subsequent development is a refinement or extension of what Martin-Löf set out here.

- **Per Martin-Löf.** "Intuitionistic Type Theory." Notes by Giovanni Sambin from lectures in Padova, 1980. Published by Bibliopolis, Naples, 1984. The most accessible original account of Martin-Löf type theory, with a clear explanation of the identity types and their role in the theory. Still worth reading as a primary source.

- **The Univalent Foundations Program.** *Homotopy Type Theory: Univalent Foundations of Mathematics.* IAS, Princeton, 2013. Chapter 2 develops identity types from the homotopy-theoretic perspective: paths, path composition, higher paths, transport, ap, and the groupoid laws. The definitive modern textbook source for this chapter's material.

- **Martin Hofmann.** "Syntax and Semantics of Dependent Types." In *Semantics and Logics of Computation*, Cambridge University Press, 1997. A careful and rigorous account of dependent type theory including identity types, with full attention to the metatheory. Useful for understanding the precise meaning of the J rule and its computation rule.

- **Bengt Nordström, Kent Petersson, and Jan Smith.** *Programming in Martin-Löf's Type Theory: An Introduction.* Oxford University Press, 1990. Available free online. A pedagogical introduction to MLTT from the programming perspective, with clear coverage of identity types and their role in equational reasoning in type theory.

## Seminal Papers

- **Martin Hofmann and Thomas Streicher.** "The Groupoid Interpretation of Type Theory." In *Twenty-Five Years of Constructive Type Theory*, edited by G. Sambin and J.M. Smith, Oxford University Press, 1998 (conference version 1994). This is the paper that proved the Groupoid Model: there is a model of Martin-Löf type theory in which types are interpreted as groupoids and identity types as morphism sets, and in which the K axiom (Uniqueness of Identity Proofs) fails. This established for the first time that UIP is independent of MLTT — the foundational result that made HoTT possible.

- **Michael Hedberg.** "A Coherence Theorem for Martin-Löf's Type Theory." *Journal of Functional Programming* 8(4) (1998), 413–436. Proved what is now called Hedberg's theorem: if a type has decidable equality, then it satisfies UIP (every identity type is a proposition). The proof uses a "constant endofunction trick" that has become a fundamental technique in HoTT metatheory.

- **Steve Awodey and Michael A. Warren.** "Homotopy Theoretic Models of Identity Types." *Mathematical Proceedings of the Cambridge Philosophical Society* 146(1) (2009), 45–55. The paper that independently (and more conceptually than Hofmann-Streicher) showed that the identity types of MLTT have a natural homotopy-theoretic interpretation, with types as spaces, terms as points, and identity proofs as paths. This is the paper that directly inspired Voevodsky's Univalent Foundations program.

- **Vladimir Voevodsky.** "Univalent Foundations Project." IAS Letter, Spring 2010. Introduced the Univalence axiom and connected it to the simplicial set model. Established that Voevodsky's simplicial model of MLTT satisfies Univalence — and in particular, that identity types in the universe correspond to homotopy equivalences.

- **Thorsten Altenkirch, Conor McBride, and Wouter Swierstra.** "Observational Equality, Now!" *PLPV 2007.* Introduced observational type theory, an alternative to Martin-Löf's identity types in which equality is extensional at base types and intensional at function types. Influential in the design of extensional variants of HoTT.

- **Dan Licata and Robert Harper.** "2-Dimensional Directed Type Theory." *MFPS 2011.* Introduced directed type theory, where identity types have a directed interpretation (morphisms rather than paths), connecting HoTT to the theory of (∞,1)-categories.

- **Peter Dybjer.** "Inductive Families." *Formal Aspects of Computing* 6(4) (1994), 440–465. The paper establishing inductive families (indexed inductive types) as a general framework, of which the identity type (indexed by two endpoints) is the most important example. Gives the general theory of which the identity type's J rule is a special case.

## Textbooks and Modern Treatments

- **The HoTT Book** (referenced above under Foundational Texts) — Chapter 2 is the core modern treatment, accessible at the level of this chapter. Read sections 2.1–2.4 (path operations), 2.7 (Σ-types), 2.9 (Π-types), 2.10 (universes and univalence).

- **Egbert Rijke.** *Introduction to Homotopy Type Theory.* Cambridge University Press, 2022. Also available as arXiv:2212.11082. An excellent modern textbook at the graduate level, starting from first principles. Chapter 5 covers the identity type and its basic properties; subsequent chapters develop the theory systematically. More accessible than the HoTT Book for a first reading.

- **Bengt Nordström, Kent Petersson, and Jan Smith** (above) — Part II covers identity types from the programming perspective, with attention to the intensional/extensional distinction.

- **Simon Boulier, Pierre-Marie Pédrot, and Nicolas Tabareau.** "The next 700 Syntactical Models of Type Theory." *CPP 2017.* Classifies models of type theory that validate various combinations of equality principles, helping to map the design space around identity types.

- **Bob Harper.** *Practical Foundations of Mathematics for Computer Science.* Cambridge University Press, 2016. Part IV develops dependent type theory including identity types from the perspective of programming language theory. Careful about definitional vs. propositional equality.

## Online Resources and Lecture Notes

- **nLab: identity type.** `https://ncatlab.org/nlab/show/identity+type`. The nLab article covers identity types in MLTT, the J rule, the homotopy interpretation, and connections to path objects in model categories. Good for finding further references.

- **nLab: path space object.** `https://ncatlab.org/nlab/show/path+space+object`. The model-categorical concept that identity types interpret — the factorization of the diagonal through a path object.

- **Andrej Bauer and Peter LeFanu Lumsdaine.** "Homotopy Type Theory." Lecture notes from a course at the OPLSS 2012. Available online. A clear, concise introduction to HoTT including identity types from the homotopy perspective.

- **Robert Harper and Dan Licata.** "Mechanizing Metatheory in a Logical Framework." *J. Automated Reasoning* 48(3) (2012). On formalizing type theory (including identity types and the J rule) in a logical framework.

- **Agda documentation: `Relation.Binary.PropositionalEquality`.** `https://agda.github.io/agda-stdlib/`. The Agda standard library's treatment of propositional equality is a direct implementation of identity types, with `refl`, `trans`, `sym`, `cong` (ap), and `subst` (transport). Reading the source is instructive.

## Historical Context

The identity type has a long pre-history. In the early days of Martin-Löf type theory (late 1960s and early 1970s), Martin-Löf worked with an extensional version of identity types: equality was definitional (computationally decidable), and two terms were equal if and only if they reduced to the same normal form. This gave a clean theory but made type-checking undecidable in general (because checking equality of functions required checking them on all inputs).

Martin-Löf's 1975 paper introduced the intensional identity type — the one with the J rule — as a compromise. Identity is no longer definitional but must be *proved*, by constructing a term of the identity type. The J rule says: to prove a property of all paths (identity proofs), prove it for reflexivity. This is a small change in presentation but has enormous consequences. In the intensional setting, type-checking is decidable (equality checking does not require running programs), but the system is richer: you can state, but not always prove, that two functions are equal.

For twenty years, the intensional identity type was viewed as a technical inconvenience. The "right" theory was thought to be extensional, and the intensional version was used primarily because it was computationally better-behaved. The situation changed entirely with Hofmann and Streicher's 1994 groupoid model. By constructing a model where types are groupoids and equality proofs are morphisms, they showed that the intensional system genuinely has more models than the extensional one — and that UIP (the K axiom) is *independent*: it holds in some models but not others. This was the key insight that led, fifteen years later, to Voevodsky's recognition that the identity type is literally the path space of a homotopy-theoretic interpretation.

Awodey and Warren (2009) and Voevodsky (2009–2010) arrived at the homotopy interpretation from two different directions — Awodey-Warren from abstract homotopy theory (weak factorization systems), Voevodsky from the simplicial set model — and their simultaneous realization that identity types are path spaces sparked the HoTT program. The full development followed rapidly in the 2010s, crystallized in the HoTT Book (2013).

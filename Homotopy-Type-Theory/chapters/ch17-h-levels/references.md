# References and Primary Sources

## Foundational Texts

- **Vladimir Voevodsky.** "Univalent Foundations Project." IAS, 2010. Lecture slides and notes available at `https://www.math.ias.edu/~vladimir/Site3/Univalent_Foundations.html`. The primary source where h-levels were introduced: Voevodsky defined the h-level hierarchy, introduced the notation $\|A\|_n$ for $n$-truncation, and laid out the framework of univalent foundations in which h-levels play a central organizing role.

- **The Univalent Foundations Program.** *Homotopy Type Theory: Univalent Foundations of Mathematics.* IAS, Princeton, 2013. Chapter 3 is the definitive modern treatment: "Sets and Logic" covers propositions, sets, the principle of unique choice, the axiom of choice in HoTT, and the general h-level hierarchy. Essential reading alongside this chapter.

- **Egbert Rijke.** *Introduction to Homotopy Type Theory.* Cambridge University Press, 2022. Available as arXiv:2212.11082. The most accessible modern textbook, written explicitly as a teaching text. Chapters on contractible types, propositions, sets, and truncations are exceptionally clear. Recommended as the primary accompanying text for this chapter.

- **Martin Hofmann and Thomas Streicher.** "The Groupoid Interpretation of Type Theory." 1994/1998. The proof that UIP is independent of MLTT, described in the Ch16 references, is also foundational for h-levels: it established that types can fail to be sets (h-level 0), motivating the entire hierarchy.

- **Per Martin-Löf.** "An Intuitionistic Theory of Types." 1975 (full reference in Ch16 references). The base layer: the original treatment of the identity type, from which all h-level considerations follow.

## Seminal Papers

- **Nicolai Kraus, Martín Escardó, Thierry Coquand, and Thorsten Altenkirch.** "Notions of Anonymous Existence in Martin-Löf Type Theory." *Logical Methods in Computer Science* 13(1) (2017). The comprehensive study of propositional truncation and anonymous existence in MLTT. Introduces the "hub and spoke" construction for truncation, proves that the propositional truncation of a type $A$ is the colimit of a certain diagram, and characterizes when maps factor through the truncation. The definitive reference for the metatheory of propositional truncation.

- **Michael Hedberg.** "A Coherence Theorem for Martin-Löf's Type Theory." *Journal of Functional Programming* 8(4) (1998). Hedberg's theorem — decidable equality implies h-set — is the key result for Section 3 of this chapter, and the proof technique (constant endofunctions) generalizes to truncation questions. Full reference in Ch16.

- **Nicolai Kraus.** "The General Universal Property of the Propositional Truncation." *TYPES 2014 Proceedings.* Extended treatment of the universal property of propositional truncation, including the uniqueness of maps out of $\|A\|$ into propositions and the precise relationship to the axiom of choice.

- **Karol Szumiło.** "Two Models for the Homotopy Theory of Cocomplete Homotopy Theories." *PhD thesis, University of Bonn*, 2014. On the general theory of n-truncations in model categories, connecting the h-level hierarchy to the Postnikov tower in classical homotopy theory.

- **Andrej Bauer, Jason Gross, Peter LeFanu Lumsdaine, Michael Shulman, Matthieu Sozeau, and Bas Spitters.** "The HoTT Library: A Formalization of Homotopy Type Theory in Coq." *CPP 2017.* Documents the formalization of the h-level hierarchy, truncations, and related material in the Coq proof assistant. A source for verified versions of theorems in this chapter.

- **Paolo Capriotti and Nicolai Kraus.** "Univalent Higher Categories via Complete Semi-Segal Types." *POPL 2018.* Connects the h-level hierarchy to higher category theory: complete Semi-Segal types are the HoTT analog of complete Segal spaces, and their formulation uses the h-level hierarchy to specify the correct notion of "equivalence at each dimension."

## Textbooks and Modern Treatments

- **Egbert Rijke.** *Introduction to Homotopy Type Theory* (full citation above). Chapters 12–15 cover contractible types, propositions, sets, and truncations in order, with extensive exercises and a careful treatment of each universal property. This is the most thorough pedagogical treatment of h-levels currently available.

- **The HoTT Book**, Chapter 3 — covers propositions, sets, logic, and the principle of unique choice. Chapter 7 covers the full h-level hierarchy (n-types) and truncations. Both chapters should be read alongside this material.

- **Steve Awodey.** *Category Theory.* Oxford University Press, 2006/2010. Not specifically about h-levels, but the categorical perspective (propositions as subobject classifiers, sets as discrete categories, n-types as n-groupoids) illuminates the h-level hierarchy. Chapter 2 on monomorphisms and Chapter 5 on limits are relevant.

- **Thorsten Altenkirch, Nils Anders Danielsson, and Nicolai Kraus.** "Partiality, Revisited: The Partiality Monad as a Quotient Inductive-Inductive Type." *FoSSaCS 2017.* Uses h-levels (specifically propositional truncation) to formalize partiality in type theory. An example of h-levels applied to a programming language theory question.

- **Michael Shulman.** "Homotopy Type Theory Should Eat Itself." Blog post, The n-Category Café, 2014. An accessible discussion of the h-level hierarchy and its role in organizing HoTT, written for mathematicians. Available at `https://golem.ph.utexas.edu/category/2014/03/homotopy_type_theory_should_ea.html`.

## Online Resources and Lecture Notes

- **nLab: h-level.** `https://ncatlab.org/nlab/show/homotopy+level`. The nLab article on h-levels covers the definition, the cumulative hierarchy, examples at each level, and connections to classical homotopy theory (Postnikov towers). Good starting point for further reading.

- **nLab: truncated object.** `https://ncatlab.org/nlab/show/truncated+object`. The model-categorical perspective on truncation, connecting h-levels to Postnikov sections and n-truncated objects in an ∞-topos.

- **Agda standard library: `Data.HLevel`.** `https://agda.github.io/agda-stdlib/`. The Agda formalization of the h-level hierarchy. Reading the source code gives a precise, checkable account of how h-levels are defined and used in practice.

- **Lean 4 Mathlib: `Mathlib.Topology.Homotopy`.** The Lean formalization includes significant material on h-sets and propositions. Lean's `Prop` universe handles h-level (-1) at the kernel level; h-sets and higher levels require explicit proofs.

- **Voevodsky's Coq Files.** `https://github.com/UniMath/Foundations`. The original Coq/UniMath formalization by Voevodsky, which defines h-levels (as `iscontr`, `isaprop`, `isaset`, `isofhlevel`) and proves the basic results. Historical importance: this is where the h-level concept was first formalized.

## Historical Context

The h-level hierarchy has a double origin: one in homotopy theory and one in type theory. In classical homotopy theory, the Postnikov tower of a space $X$ is a sequence of spaces $\cdots \to X[2] \to X[1] \to X[0]$ where $X[n]$ has trivial homotopy groups above dimension $n$. A space is "$n$-truncated" if $\pi_k(X) = 0$ for $k > n$. This structure was well understood by the 1950s. The h-level hierarchy in HoTT directly mirrors this: a type is an $n$-type (h-level $n$) if and only if, in Voevodsky's simplicial set model, the corresponding Kan complex is $(n-1)$-truncated in the classical sense. The offset by one (h-level $n$ corresponds to classical $(n-1)$-truncation) comes from the convention that contractible types (h-level $-2$) correspond to the "(-2)-truncated" = terminal object.

The type-theoretic origin is more recent. Martin-Löf's original type theory had no h-level distinction — all types were treated uniformly. The Hofmann-Streicher groupoid model (1994) showed that some types behave like sets (UIP holds) and others do not, but did not systematize this. Voevodsky, in his foundational work on univalent foundations (2009–2010), introduced the h-level hierarchy as a systematic stratification of types by "homotopy complexity." His definitions of `iscontr`, `isaprop`, and `isaset` (in Coq) are essentially the definitions in Section 1–3 of this chapter, and his notation $\|A\|_n$ for $n$-truncation is now standard.

The study of truncation as a *construction* (not just a property) required the development of higher inductive types (HITs), which came slightly later. The propositional truncation $\|A\|_{-1}$ was one of the first HITs to be systematically studied, and its universal property — maps out of $\|A\|$ into propositions are the same as maps out of $A$ into propositions — was recognized as fundamental early on. The paper by Kraus, Escardó, Coquand, and Altenkirch (2017) gave a comprehensive treatment of when truncation can be built from simpler constructions and when it genuinely requires a HIT.

The interaction between h-levels and the axiom of choice is a rich topic with classical roots. In classical set theory, the axiom of choice says: if every fiber of a surjection is nonempty, the surjection has a section. In HoTT, "nonempty" must be interpreted carefully: $\|F_x\|_{-1}$ (merely nonempty) or $F_x$ (concretely nonempty)? The HoTT Book Chapter 3.8 shows that choice holds when the domain is a set and the fibers are merely nonempty — but fails in general. This is one of the most important consequences of the h-level hierarchy for mathematics.

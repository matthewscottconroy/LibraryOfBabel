# References and Primary Sources

## Foundational Texts

**Vladimir Voevodsky.** *Univalent Foundations of Mathematics* (lecture notes and unpublished manuscripts, 2009–2013). The original source for the univalence axiom, written by its discoverer; these lectures (available via the IAS website and the Voevodsky archive) contain the first public articulation of the program and the intended semantics.

**The Univalent Foundations Program.** *Homotopy Type Theory: Univalent Foundations of Mathematics*. Institute for Advanced Study, 2013. Available free at homotopytypetheory.org/book. The collective output of the 2012–2013 IAS special year; Chapters 2 and 4 contain the definitive treatment of equivalences and the univalence axiom as used in this chapter.

**Per Martin-Löf.** *Intuitionistic Type Theory* (notes by Giovanni Sambin). Bibliopolis, Napoli, 1984. The foundational text for dependent type theory on which HoTT is built; understanding the J rule, transport, and identity types requires familiarity with Martin-Löf's original formulation.

**Thierry Coquand, Simon Huber, Anders Mörtberg et al.** *Cubical Agda* (tool and library documentation, 2018–present). The reference for the computational implementation of univalence; in cubical type theory, `ua` is not an axiom but a derived construction, making it amenable to computation and normalization.

---

## Seminal Papers

**Steve Awodey and Michael A. Warren.** "Homotopy theoretic models of identity types." *Mathematical Proceedings of the Cambridge Philosophical Society*, 146(1):45–55, 2009. The paper that first showed identity types in MLTT can model higher-dimensional homotopy; the groupoid model is extended here to establish that homotopy-theoretic models of type theory exist, setting the stage for Voevodsky's model.

**Chris Kapulkin and Peter LeFanu Lumsdaine.** "The simplicial model of univalent foundations (after Voevodsky)." *Journal of the European Mathematical Society*, 23(6):2071–2126, 2021 (preprint 2012). The rigorous proof that Kan complexes model all of HoTT including the univalence axiom; this is the primary reference for the consistency of univalence, filling in full detail for results Voevodsky sketched.

**Cyril Cohen, Thierry Coquand, Simon Huber, and Anders Mörtberg.** "Cubical Type Theory: A constructive interpretation of the univalence axiom." In *21st International Conference on Types for Proofs and Programs (TYPES 2015)*, 2015. Introduces cubical type theory, in which `ua` satisfies a genuine computation rule (the `Glue` type), resolving the open problem of making univalence computational.

**Vladimir Voevodsky.** "A very short note on the homotopy $\lambda$-calculus." Unpublished note, 2006, revised 2010. Available via the Voevodsky archive at IAS. The earliest written formulation of the univalence axiom; historically significant as the first statement of the central idea.

**Benedikt Ahrens, Krzysztof Kapulkin, and Michael Shulman.** "Univalent categories and the Rezk completion." *Mathematical Structures in Computer Science*, 25(5):1010–1039, 2015. Demonstrates the structure identity principle (SIP) at work: two categories satisfying the Segal condition are equal iff they are equivalent, making univalence essential for doing category theory in HoTT.

**Martín Hötzel Escardó.** "A self-contained, brief and complete proof of Voevodsky's conjecture that the model of Martin-Löf type theory in Kan complexes satisfies the univalence axiom." Unpublished note, 2018. A compressed, self-contained proof of the simplicial set model, useful as a supplement to the Kapulkin-Lumsdaine paper.

**Andrew Pitts.** "Nominal Sets: Names and Symmetry in Computer Science." *Cambridge Tracts in Theoretical Computer Science*, Cambridge University Press, 2013. Background for the nominal models of type theory that motivate some aspects of the univalence and transport story.

---

## Textbooks and Modern Treatments

**Egbert Rijke.** *Introduction to Homotopy Type Theory*. Cambridge University Press (to appear; preprint at arXiv:2212.11082, 2022). A modern textbook at the graduate level, covering equivalences and univalence with a clean pedagogical presentation; particularly strong on the fundamental theorem of identity types and its applications.

**Bengt Nordström, Kent Petersson, and Jan M. Smith.** *Programming in Martin-Löf's Type Theory*. Oxford University Press, 1990. A classic introduction to dependent type theory; useful as background for the rules of identity types and transport before tackling univalence.

**Simon Thompson.** *Type Theory and Functional Programming*. Addison-Wesley, 1991. Available free online. Accessible introduction to type theory aimed at computer scientists; good preparation for understanding why equivalences and function extensionality matter in programming.

**Thorsten Altenkirch, Conor McBride, and Wouter Swierstra.** "Observational equality, now!" *Proceedings of the 2007 workshop on Programming Languages meets Program Verification (PLPV '07)*, 57–68, 2007. Addresses how equality should behave in dependent type theory; historically important for understanding why propositional extensionality and function extensionality matter, predating but motivating the univalence approach.

---

## Online Resources and Formalization Code

**Martín Hötzel Escardó.** *Introduction to Univalent Foundations of Mathematics with Agda*. Available at www.cs.bham.ac.uk/~mhe/HoTT-UF-in-Agda-Lecture-Notes/. A comprehensive free resource that formalizes univalent foundations directly in Agda; covers equivalences, univalence, function extensionality, propositional truncation, and the structure identity principle with full machine-checked proofs.

**The `agda/cubical` library.** Available at github.com/agda/cubical. The primary repository for formalized HoTT in Cubical Agda; the `Cubical.Foundations.Univalence` module contains the computational proof of `ua`, and `Cubical.Foundations.Equiv` contains the main definitions of equivalences used in this chapter.

**The `UniMath` library.** Available at github.com/UniMath/UniMath. Voevodsky's own formalization project in Coq, implementing the univalent foundations program; the `Foundations/` directory contains the core identity type and equivalence machinery, and is historically significant as the first large-scale formalization based on univalence.

**nLab: Univalence axiom.** Available at ncatlab.org/nlab/show/univalence+axiom. A comprehensive, maintained reference connecting the HoTT formulation of univalence to its categorical semantics (Kan complexes, elementary $\infty$-toposes), with extensive links to primary literature.

**The HoTT/HoTT library in Coq.** Available at github.com/HoTT/HoTT. A large Coq library implementing Book HoTT; the `theories/Basics/Equivalences.v` and `theories/Basics/UnivalenceAxiom.v` files contain the standard presentations of equivalences and the univalence axiom.

---

## Historical Context

The univalence axiom emerged from Vladimir Voevodsky's search for a rigorous foundation for mathematics that would reflect how mathematicians actually think about structures. After winning the Fields Medal in 1990 for his work on algebraic geometry and motivic cohomology, Voevodsky became concerned about the reliability of mathematical proofs, including some of his own earlier work. In the early 2000s he began exploring formalized mathematics and proof assistants as a remedy. By 2006 he had formulated the core idea: that in the presence of a suitable homotopy-theoretic semantics, the identity type $A = B$ in a universe should be equivalent to the type $A \simeq B$ of equivalences. He announced this publicly in a 2010 lecture at Carnegie Mellon University, "Univalent Foundations of Mathematics," which introduced the program to a wider audience. The pivotal 2009 paper of Awodey and Warren showed that identity types already had homotopy-theoretic models (the groupoid model extended to higher groupoids), establishing that there was no contradiction lurking in Voevodsky's idea. Voevodsky then constructed his simplicial set model, and Chris Kapulkin and Peter LeFanu Lumsdaine wrote the full proof into a rigorous paper.

The 2012–2013 special year at the Institute for Advanced Study brought together type theorists, homotopy theorists, and logicians to develop HoTT collectively, resulting in the HoTT Book (2013). At this point univalence was a celebrated but still somewhat mysterious axiom: it was consistent and had good consequences, but it was not computational — there was no way to reduce `ua` to simpler constructions, so it blocked normalization in proof assistants. The computational resolution came with cubical type theory, developed by Cohen, Coquand, Huber, and Mörtberg in 2015–2017, which gives `ua` a genuine computation rule via the `Glue` type. This transformed univalence from an axiom that one adds to a type theory into a theorem provable from the rules of a more refined type theory, and it is this computational univalence that is implemented in Cubical Agda and is the basis for the most advanced current formalizations.

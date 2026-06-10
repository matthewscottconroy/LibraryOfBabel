# References and Primary Sources

## Foundational Texts

**The Univalent Foundations Program.** *Homotopy Type Theory: Univalent Foundations of Mathematics*. Institute for Advanced Study, 2013. Chapter 6 ("Higher Inductive Types") is the primary reference for HITs in the Book HoTT framework; it introduces the circle, suspension, pushouts, propositional truncation, and set truncation, and proves the Seifert-van Kampen theorem using pushouts. Available free at homotopytypetheory.org/book.

**Per Martin-Löf.** *Intuitionistic Type Theory* (notes by Giovanni Sambin). Bibliopolis, Napoli, 1984. The foundational text for ordinary inductive types and dependent type theory; HITs extend the inductive types described here by adding path constructors, so understanding this background is essential.

**Michael Shulman.** *All $(\infty,1)$-toposes have strict univalent universes* (preprint, arXiv:1904.07004, 2019). Establishes the semantic framework in which HITs have their intended homotopy-theoretic meaning; proves that every $(\infty,1)$-topos models HoTT with HITs, establishing a sweeping generalization of the simplicial set model.

**Evan Cavallo and Robert Harper.** *Higher Inductive Types in Cubical Computational Type Theory.* Proceedings of the ACM on Programming Languages (POPL 2019), 2019. The reference for computational HITs: in cubical type theory, HITs are not postulated axiomatically but given precise reduction rules, making them fully computable. This is the foundation for HIT support in Cubical Agda.

---

## Seminal Papers

**Peter LeFanu Lumsdaine and Michael Shulman.** "Semantics of higher inductive types." *Mathematical Proceedings of the Cambridge Philosophical Society*, 169(1):159–208, 2020 (preprint 2017). The definitive formal semantics of HITs; proves that any HIT specified by a suitable signature (a globular sum) has a model in any $(\infty,1)$-topos with enough universes, and in particular in simplicial sets. This resolved the longstanding question of whether HITs were well-defined — prior to this paper, most work with HITs was informal.

**Daniel R. Licata and Michael Shulman.** "Calculating the Fundamental Group of the Circle in Homotopy Type Theory." In *Proceedings of the 28th Annual ACM/IEEE Symposium on Logic in Computer Science (LICS 2013)*, 223–232, 2013. The first machine-checked proof that $\pi_1(S^1) = \mathbb{Z}$ in HoTT; introduces the encode-decode method in full generality, using the circle HIT, univalence (to define the code family), and transport. This paper established the pattern that all subsequent computations of homotopy groups in HoTT have followed.

**Kristina Sojakova.** "Higher Inductive Types as Homotopy-Initial Algebras." In *Proceedings of the 42nd ACM SIGPLAN-SIGACT Symposium on Principles of Programming Languages (POPL 2015)*, 31–42, 2015. Proposes a semantic characterization of HITs as homotopy-initial algebras (the initial algebra in the $\infty$-categorical sense), giving a universal property that any correct interpretation of a HIT must satisfy.

**Evan Cavallo and Robert Harper.** "Higher Inductive Types in Cubical Computational Type Theory." *Proceedings of the ACM on Programming Languages* 3(POPL), 1:1–1:27, 2019. See above; the key result is that HITs in cubical type theory satisfy *strict* computation rules for both point and path constructors, resolving the longstanding gap between HITs as stated and HITs as computable.

**Guillaume Brunerie, Axel Ljungström, and Anders Mörtberg.** "Synthetic Integral Cohomology in Cubical Agda." In *30th EACSL Annual Conference on Computer Science Logic (CSL 2022)*, 2022. Demonstrates the power of computational HITs by formalizing integral cohomology of spheres in Cubical Agda, using Eilenberg-MacLane spaces (themselves constructed as HITs) and computing cohomology groups mechanically.

**Nicolai Kraus.** "Truncation Levels in Homotopy Type Theory." PhD thesis, University of Nottingham, 2015. Develops the metatheory of truncation and HITs, including the theory of propositional truncation as a HIT, the elimination principle for truncations, and the connection between h-levels and HIT structure.

---

## Textbooks and Modern Treatments

**Egbert Rijke.** *Introduction to Homotopy Type Theory*. Cambridge University Press (to appear; preprint arXiv:2212.11082, 2022). Chapters on HITs, pushouts, and truncations are clear and modern; the treatment of the circle and its fundamental group uses up-to-date techniques and is well-suited for a first reading.

**Bengt Nordström, Kent Petersson, and Jan M. Smith.** *Programming in Martin-Löf's Type Theory*. Oxford University Press, 1990. Background on ordinary inductive types; useful for understanding what HITs add to the picture.

**Robert Harper.** *Practical Foundations of Mathematics*. Cambridge University Press, 2016. An advanced text on type theory with attention to the computational content of proofs; useful background for understanding the computational gap that cubical HITs fill.

**Andy Pitts and Marcelo Fiore (eds.).** *Semantics and Logics of Computation*. Cambridge University Press, 1997. Background on the categorical semantics of inductive types and their universal properties; the universal property of HITs (as initial algebras) is an extension of the initial algebra semantics for ordinary inductive types.

---

## Online Resources and Formalization Code

**The `agda/cubical` library.** Available at github.com/agda/cubical. The most comprehensive formalization of HITs; the `Cubical.HITs/` directory contains the circle, spheres, suspension, pushouts, propositional truncation, set truncation, join, torus, real projective spaces, Hopf fibration, and more. The code is directly runnable and serves as both a reference and a testing ground.

**The HoTT/HoTT library in Coq.** Available at github.com/HoTT/HoTT. Contains Book HoTT formalization of HITs including the circle (`theories/HIT/Circle.v`), pushouts, truncations, and the Seifert-van Kampen theorem; reflects the formalization state-of-the-art for Coq-based HoTT.

**Martín Hötzel Escardó.** *Introduction to Univalent Foundations with Agda*. Available at www.cs.bham.ac.uk/~mhe/HoTT-UF-in-Agda-Lecture-Notes/. Covers HITs including propositional truncation and set truncation, with full Agda formalization; the discussion of the relationship between HITs and the logic of existence is particularly useful.

**nLab: Higher inductive type.** Available at ncatlab.org/nlab/show/higher+inductive+type. A maintained reference connecting the HoTT presentation of HITs to their categorical semantics (as homotopy-initial algebras in $\infty$-toposes), with historical notes and links to primary literature.

**Floris van Doorn's PhD thesis.** *On the Formalization of Higher Inductive Types and Synthetic Homotopy Theory*. Carnegie Mellon University, 2018. Available at florisvandoorn.com. Develops a systematic theory of HITs in Lean 2, including a formal treatment of the pushout, the Seifert-van Kampen theorem, and the Freudenthal suspension theorem; a useful companion to the HoTT Book treatment.

---

## Historical Context

Higher inductive types were not part of the original HoTT program as Voevodsky conceived it in the mid-2000s. The idea that one could add *path constructors* to inductive type definitions — not just point constructors — emerged gradually from discussions at the IAS special year in 2012–2013. The key conceptual move was recognizing that the path constructors of a HIT are just constructors for the identity type of the HIT: the circle is the inductive type generated by `base : S¹` and `loop : base = base`, and this is perfectly well-formed from the type theory's point of view once one allows constructors to mention identity types.

The first HITs to be formalized were the interval and the circle, which appeared in early drafts of the HoTT Book. The fundamental group computation $\pi_1(S^1) = \mathbb{Z}$, first proved formally by Licata and Shulman in 2013, was a landmark: it demonstrated that HITs, combined with the univalence axiom, could do genuine homotopy theory within type theory. The Seifert-van Kampen theorem (proved in HoTT by Favonia and Shulman) and the Freudenthal suspension theorem followed.

The semantic foundations of HITs were, however, murky for several years. The HoTT Book stated HITs and their elimination principles without a rigorous definition of what a valid HIT specification is, or a proof that any intended interpretation exists. This gap was addressed by Lumsdaine and Shulman (2019), who gave the first complete semantic account of HITs as globular sums in any $(\infty,1)$-topos. Concurrently, Cavallo and Harper (2019) resolved the computational gap: in Book HoTT, HITs are stated as axioms (the eliminator exists, but its computation rule on path constructors is only propositional), whereas in cubical type theory they satisfy strict definitional computation rules. The combination of Lumsdaine-Shulman semantics and cubical computational HITs gives the modern, complete picture.

The scope of HITs continues to expand. Eilenberg-MacLane spaces (constructed as HITs), spectra (infinite loop spaces built from HITs), and synthetic algebraic geometry all rely on increasingly sophisticated HIT constructions. The `agda/cubical` library, with its growing collection of formalized HITs and results about them, is currently the most accurate picture of the frontier.

# References and Primary Sources

## Primary Documentation and Tools

**The CCHM Cubical Type Theory** does not have a single "official implementation" — it is a formal theory defined by its typing rules. The two main implementations are:

**Cubical Agda** — the most mature implementation, integrated into Agda.
- Repository: [github.com/agda/agda](https://github.com/agda/agda)
- Cubical library: [github.com/agda/cubical](https://github.com/agda/cubical)
- Documentation: [agda.readthedocs.io](https://agda.readthedocs.io/) (under "Cubical Agda")

**cubicaltt** — the original stand-alone implementation of CCHM by Huber and Mörtberg. Small, clean, and readable. Ideal for understanding the reduction rules.
- Repository: [github.com/mortberg/cubicaltt](https://github.com/mortberg/cubicaltt)
- Contains many example files demonstrating `transp`, `hcomp`, `Glue`, and HITs.

**redtt / RedTT** — Carnegie Mellon's implementation of *Cartesian* cubical type theory (CHTT), a closely related but distinct approach that uses a box operator instead of De Morgan complement. The CMU implementation is a companion to the Angiuli-Favonia-Harper family of papers.
- Repository: [github.com/RedPRL/redtt](https://github.com/RedPRL/redtt)

**cooltt** — the successor to redtt, implementing Cartesian cubical type theory more cleanly.
- Repository: [github.com/RedPRL/cooltt](https://github.com/RedPRL/cooltt)

---

## Foundational Papers

**Cyril Cohen, Thierry Coquand, Simon Huber, Anders Mörtberg.** "Cubical Type Theory: A Constructive Interpretation of the Univalence Axiom." *IfCoLoG Journal of Logics and Their Applications* 4(10):3127–3169, 2017. (Preliminary version at TYPES 2015; see also the final arxiv version at arxiv:1611.02108.)
The primary reference for this entire chapter. The CCHM paper establishes: the interval $\mathbb{I}$ with De Morgan algebra structure, face formulas, partial elements, the `hcomp` composition operation, the `transp` transport operation, the formation rules for each type former, the `Glue` type constructor, the proof of univalence as a theorem, and canonicity. Every section of this chapter traces back to this paper. The IfCoLoG version is the most complete; the arxiv version is the most accessible.

**Marc Bezem, Thierry Coquand, Simon Huber.** "A Model of Type Theory in Cubical Sets." *19th International Conference on Types for Proofs and Programs (TYPES 2013)*, LIPIcs Vol. 26, pp. 107–128. Schloss Dagstuhl, 2014.
The precursor to CCHM: the paper that introduced the cubical sets model of type theory. Establishes that there exists a model of MLTT (with function extensionality and a form of univalence) in cubical sets. The key insight is replacing presheaves over the simplex category (the Kan complex / Quillen model) with presheaves over a category of cubes, which supports *constructive* Kan filling. The CCHM paper refines this model into a full type theory.

**Carlo Angiuli, Kuen-Bang Hou (Favonia), Robert Harper.** "Computational Higher-Dimensional Type Theory." *Proceedings of POPL 2017*, pp. 680–693. ACM, 2017.
The primary paper for *Cartesian* cubical type theory — the competing approach from CMU. Instead of the De Morgan algebra on the interval, Cartesian cubical uses a simpler interval with only the two endpoints (no complement, no meet/join in the type theory proper). The composition operation is replaced by a "box" operation. The advantage: simplicity and clear computational semantics. The disadvantage: path reversal (`sym`) is not definitional. Essential reading for understanding the design space explored in Section 4 of this chapter.

**Simon Huber.** "Cubical Interpretations of Type Theory." PhD thesis, Chalmers University of Technology, 2016. Available at [research.chalmers.se](https://research.chalmers.se/).
The most detailed technical treatment of the CCHM theory. Contains complete typing rules with all the side conditions, the full specification of `hcomp` for each type former (Sigma, Pi, path, Glue, inductive), the proof of canonicity for natural numbers, and the meta-theoretical development. This is the reference for anyone who wants to implement CCHM or check a specific reduction rule.

**Ian Orton and Andrew M. Pitts.** "Axioms for Modelling Cubical Type Theory in a Topos." *Proceedings of CSL 2016*, LIPIcs Vol. 62, article 24. Schloss Dagstuhl, 2016.
An axiomatic treatment of cubical type theory: instead of working in a specific cubical sets model, Orton and Pitts identify the topos-theoretic axioms needed to support the cubical constructions. This gives a cleaner and more general meta-theory. Useful for understanding the "why" behind the De Morgan algebra: the axioms require a *distributive lattice with complement* on the interval, and the De Morgan laws follow.

**Thierry Coquand, Simon Huber, Christian Sattler.** "Homotopy Canonicity for Cubical Type Theory." *Proceedings of FSCD 2019*, LIPIcs Vol. 131, article 11. Schloss Dagstuhl, 2019.
The canonicity paper for CCHM. Proves that every closed term of type `ℕ` in CCHM reduces to a numeral — a key correctness guarantee for the system. The proof uses a notion of "homotopy canonicity" (every term is *homotopic* to a canonical form) rather than strict canonicity, because the presence of path types means not every term is *definitionally* equal to a numeral even if it is *propositionally* equal to one.

---

## Textbooks and Learning Resources

**The Univalent Foundations Program.** *Homotopy Type Theory: Univalence, Higher Inductive Types, and Their Applications.* IAS, 2013. [homotopytypetheory.org/book](https://homotopytypetheory.org/book/).
The HoTT Book — the primary mathematical reference for the content that cubical type theory is computing. Chapters 1–2 (MLTT), Chapter 4 (equivalences and univalence), Chapter 6 (HITs), and the appendix (type theory rules) are most relevant. The Book's treatment of univalence as an axiom motivates the cubical theory's solution.

**Anders Mörtberg.** *Introduction to Cubical Type Theory.* Lecture notes, CMU and Stockholm University, 2019–2022.
The most accessible introduction specifically to CCHM as a type theory (not just as Cubical Agda). Covers the interval, paths, transport, `hcomp`, the Glue type, and univalence with concrete examples. Written by a co-creator of the theory. Available through Mörtberg's academic webpage.

**Carlo Angiuli, Guillaume Brunerie, Thierry Coquand, Kuen-Bang Hou (Favonia), Robert Harper, Daniel R. Licata.** "Syntax and Models of Cartesian Cubical Type Theory." *Mathematical Structures in Computer Science* 31(4):424–468, 2021.
The comprehensive treatment of Cartesian cubical type theory — the CMU approach. Develops the type theory with formal syntax, a categorical semantics, and detailed comparison with the De Morgan approach. Reading this alongside the CCHM paper gives a complete picture of the design choices in Section 4 of this chapter.

**Evan Cavallo and Robert Harper.** "Internal Parametricity for Cubical Type Theory." *Proceedings of CSL 2020*, LIPIcs Vol. 152, article 13. Schloss Dagstuhl, 2020.
Develops parametricity — the abstract principle that polymorphic functions preserve structure — internally within cubical type theory. Shows that the cubical interval can be used to internalize the parametricity argument that is usually external to the type theory. Relevant to Section 4 (variations) and to the connections between cubical type theory and relational parametricity.

**Robert Harper.** *Practical Foundations of Mathematics and Programming.* Cambridge University Press, 2016.
Harper's textbook on constructive type theory from the CMU perspective. Less focused on HoTT specifically, but provides the deepest treatment of computational type theory — the philosophy that type theory is a theory of *computation* first, and logic second. Understanding this perspective clarifies why the CMU group (Harper, Angiuli, Favonia) emphasized Cartesian cubical type theory's clear computational content.

---

## Key Libraries and Online Resources

**cubicaltt** — [github.com/mortberg/cubicaltt](https://github.com/mortberg/cubicaltt)
The original stand-alone CCHM checker. Small enough to read completely. Running the examples in `examples/` makes the reduction behavior of `transp` and `hcomp` visible in a minimal setting. The `univalence.ctt` file contains the Glue-based proof of univalence from the CCHM paper.

**The Cubical Agda Library** — [github.com/agda/cubical](https://github.com/agda/cubical)
The production library for CCHM in Agda. `Cubical/Core/Primitives.agda` exposes the raw `transp`, `hcomp`, and `Glue` primitives. `Cubical/Foundations/Univalence.agda` contains the proof of univalence from Glue. These files are the direct implementation of the CCHM paper in a working proof assistant.

**RedPRL Zulip** — [redprl.zulipchat.com](https://redprl.zulipchat.com/)
The community forum for the CMU cubical and computational type theory tools (redtt, cooltt, RedPRL). Active discussions on the design of Cartesian cubical type theory, normalization algorithms, and type-directed program extraction.

**HoTT Zulip** — the `#cubical` and `#homotopy-theory` streams at [homotopytypetheory.zulipchat.com](https://homotopytypetheory.zulipchat.com/) (or search for the current HoTT community Zulip).
Discussions spanning the theoretical and applied sides of cubical type theory. Good for questions that sit at the boundary of CCHM theory and mathematical formalization.

**nLab: Cubical Type Theory** — [ncatlab.org/nlab/show/cubical+type+theory](https://ncatlab.org/nlab/show/cubical+type+theory)
The nLab entry on cubical type theory, with links to all major papers and implementations. Provides the categorical perspective on why cubical sets are the right model for this theory.

---

## Historical Context

The problem of giving computational content to univalence was recognized essentially from the moment that Voevodsky proposed the univalence axiom (around 2006–2009, during the IAS special year on Univalent Foundations in 2012–2013). Voevodsky himself was aware that adding `ua` as an axiom breaks canonicity, and he worked on an alternative foundation (HTS, homotopy type system, with a two-level type theory) as a potential solution. Meanwhile, Bezem, Coquand, and Huber were developing a different approach: instead of modifying the universe of types, they changed what a *path* is.

The Bezem-Coquand-Huber 2013 paper gave the first model, but it was restricted: it handled function extensionality and a weak form of univalence, but not full univalence or HITs. The key missing piece was the right notion of composition — specifically, the Kan condition, which says that every open box can be filled to a full cube. Coquand's insight was that the De Morgan algebra on the interval makes it possible to formulate a Kan composition operation that is both constructive and has definitional computation rules for all type formers. This insight, developed with Cohen, Huber, and Mörtberg into the CCHM paper (first presented at TYPES 2015, published in full in 2017), gave the first type theory with computable univalence and HITs.

At approximately the same time, the CMU group (Harper, Angiuli, Favonia, Licata, and others) was developing Computational Higher-Dimensional Type Theory (CHTT) — a different approach to the same problem. CHTT uses a Cartesian product structure on the interval (no De Morgan complement) and formulates the Kan condition differently. The two approaches — CCHM (De Morgan) and CHTT (Cartesian) — represent the two main lines of cubical type theory today. Both have implementations (Cubical Agda for CCHM; cooltt for CHTT) and active research communities. The comparison between them, explored in Section 4 of this chapter, illuminates which features are essential and which are design choices.

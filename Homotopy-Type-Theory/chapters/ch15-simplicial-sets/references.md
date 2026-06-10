# References and Primary Sources

## Foundational Texts

- **Samuel Eilenberg and Joseph Zilber.** "Semi-Simplicial Complexes and Singular Homology." *Annals of Mathematics*, 1950. The paper that introduced semi-simplicial sets (simplicial sets without degeneracies) and proved the Eilenberg-Zilber theorem relating the singular homology of a product space to the tensor product of the individual complexes — a landmark result that launched the combinatorial approach to algebraic topology.

- **Daniel M. Kan.** "A Combinatorial Definition of Homotopy Groups." *Annals of Mathematics*, 1958. Kan's foundational paper defining what are now called Kan complexes (he called them "c.s.s. complexes satisfying the extension condition"), defining homotopy groups combinatorially, and establishing the adjunction between geometric realization and the singular functor — one of the most influential single papers in the history of algebraic topology.

- **Daniel G. Quillen.** *Homotopical Algebra.* Lecture Notes in Mathematics 43, Springer-Verlag, 1967. The monograph that introduced the axioms for model categories, proved that simplicial sets and topological spaces carry Quillen-equivalent model structures, and launched the modern framework for abstract homotopy theory. Everything in §§4–5 of this chapter lives in Quillen's shadow.

- **J. Peter May.** *Simplicial Objects in Algebraic Topology.* Van Nostrand Mathematical Studies, 1967; reprinted by University of Chicago Press, 1992. The definitive classical reference for the theory of simplicial sets from the perspective of algebraic topology. Covers the singular complex, geometric realization, simplicial groups, and the connection to loop spaces. Terse but complete.

- **Paul G. Goerss and John F. Jardine.** *Simplicial Homotopy Theory.* Progress in Mathematics 174, Birkhäuser, 1999. The modern standard reference for simplicial homotopy theory, covering both the Quillen model structure and many advanced topics (bisimplicial sets, Postnikov towers, function complexes, simplicial sheaves). Comprehensive and rigorous; a graduate-level textbook.

## Seminal Papers

- **Daniel M. Kan.** "Adjoint Functors." *Transactions of the American Mathematical Society* 87 (1958), 294–329. In the same year as the homotopy groups paper, Kan introduced the notion of adjoint functors in full generality — and proved that geometric realization and the singular complex are adjoint. This paper effectively invented one of the most important concepts in all of mathematics.

- **Daniel G. Quillen.** "Rational Homotopy Theory." *Annals of Mathematics* 90 (1969), 205–295. Using the model structure on simplicial sets, Quillen gave a complete algebraic description of the rational homotopy type of simply connected spaces, solving a long-standing problem and demonstrating the power of the model-categorical framework.

- **André Joyal.** "Quasi-Categories and Kan Complexes." *Journal of Pure and Applied Algebra* 175 (2002), 207–222. The paper introducing quasi-categories (simplicial sets in which every inner horn has a filler, but outer horns need not be filled) as models of (∞,1)-categories and proving the Joyal model structure on simplicial sets. This paper opened up the field of higher category theory using simplicial sets.

- **André Joyal.** "The Theory of Quasi-Categories and Its Applications." Lectures at the CRM Barcelona, 2008 (notes available online). A comprehensive treatment of the theory of quasi-categories as models of ∞-categories, covering limits, colimits, adjunctions, and fibrations in this setting.

- **Charles Rezk.** "A Model for the Homotopy Theory of Homotopy Theory." *Transactions of the American Mathematical Society* 353 (2001), 973–1007. Introduced complete Segal spaces as an alternative model for (∞,1)-categories, proved a Quillen equivalence with the Joyal model structure, and established many of the foundational properties of the model for higher category theory.

- **Vladimir Voevodsky.** "A Model of Univalent Foundations." Lecture at Harvard CMSA, 2010; see also "Univalent Foundations Project" (IAS 2010). The original account of the simplicial set model validating HoTT's axioms, including the proof that the universe of small Kan complexes is itself fibrant (univalent). This is the consistency proof for HoTT.

- **Cisinski, Denis-Charles.** "Les préfaisceaux comme modèles des types d'homotopie." *Astérisque* 308, Société Mathématique de France, 2006. A comprehensive treatment of Cisinski model structures on presheaf categories, generalizing Quillen's simplicial model structure and providing a uniform framework that covers cubical sets, globular sets, and many other combinatorial models.

## Textbooks and Modern Treatments

- **Jacob Lurie.** *Higher Topos Theory.* Annals of Mathematics Studies 170, Princeton University Press, 2009. Available free on arXiv. Appendix A contains the definitive self-contained account of the theory of simplicial sets for applications to higher category theory. The main text develops the theory of (∞,1)-topoi using quasi-categories. The appendix alone is a complete graduate course in the subject.

- **Emily Riehl.** *Categorical Homotopy Theory.* Cambridge University Press, 2014. Available free on the author's website. A modern, carefully written textbook on homotopical algebra from the perspective of category theory. Chapter 3 gives a beautiful account of simplicial homotopy theory accessible to a reader with category theory background.

- **The Univalent Foundations Program.** *Homotopy Type Theory: Univalent Foundations of Mathematics.* IAS, 2013. Available free at homotopytypetheory.org. The standard reference for HoTT; does not develop simplicial sets in full, but the appendix and Chapter 2 discuss the simplicial model motivating the type-theoretic constructions.

- **Dmitri Ara, Marc Bezem, Thierry Coquand, et al.** *Cubical Type Theory.* 2017 and subsequent papers. While not about simplicial sets, the cubical approach (using cubical sets rather than simplicial sets) provides a directly computational model of HoTT that sheds light on the simplicial model by comparison. Start with Cohen-Coquand-Huber-Mörtberg (2015).

- **Simon Boulier, Pierre-Marie Pédrot, and Nicolas Tabareau.** "The next 700 syntactical models of type theory." *CPP 2017*. On the general methodology of building models of type theory within type theory, with simplicial sets as the motivating example.

## Online Resources and Lecture Notes

- **nLab: simplicial set.** `https://ncatlab.org/nlab/show/simplicial+set`. The nLab article on simplicial sets is one of the best freely available treatments, with connections to dozens of related concepts, worked examples, and links to the primary literature. Essential reference.

- **nLab: Kan complex.** `https://ncatlab.org/nlab/show/Kan+complex`. Covers the horn-filling condition, connections to ∞-groupoids, and the relationship to Kan fibrations. Contains precise statements of the main theorems.

- **Emily Riehl.** "A Leisurely Introduction to Simplicial Sets." Available at `https://math.jhu.edu/~eriehl/ssets.pdf`. A beautifully written 20-page introduction to simplicial sets for mathematicians with category theory background. Covers all the basics with clarity and geometric intuition.

- **Greg Friedman.** "An Elementary Illustrated Introduction to Simplicial Sets." *Rocky Mountain Journal of Mathematics* 42 (2012). Available on arXiv: `math/0809.4221`. A very accessible introduction with extensive illustrations, working through the combinatorics in detail. Ideal for readers new to the subject.

- **Kerodon.** `https://kerodon.net`. Jacob Lurie's online reference for (∞,1)-category theory, regularly updated. The chapters on simplicial sets and quasi-categories are authoritative and more readable than *Higher Topos Theory*.

## Historical Context

The story of simplicial sets begins with the birth of algebraic topology in the early 20th century. Simplicial complexes — the precursors — go back to Poincaré's work on homology. The key step toward simplicial *sets* came in the late 1940s and early 1950s. Eilenberg and Mac Lane's work on singular homology showed that one could compute topological invariants by probing spaces with "singular simplices" (continuous maps from standard simplices). Eilenberg and Zilber's 1950 paper on semi-simplicial complexes gave the first systematic combinatorial treatment, introducing the face maps. The full structure — including the degeneracy maps — was completed by Kan in the 1950s. The degeneracy maps appear at first like a technical convenience, but they are essential: without them, you cannot define homotopy groups purely combinatorially, as Kan proved.

Kan's 1958 papers on homotopy groups and adjoint functors are a pivotal moment in 20th century mathematics. In a single stroke, Kan introduced adjoint functors (the most important concept in category theory after categories themselves), proved the adjunction between geometric realization and the singular complex, and identified the Kan condition (horn filling) as the correct condition for a simplicial set to model a homotopy type. Quillen's 1967 monograph *Homotopical Algebra* completed the classical picture by axiomatizing the structure that made the equivalence between simplicial sets and topological spaces work — model categories — and proving that the two categories are Quillen equivalent. For 30 years, the Quillen model structure on simplicial sets was the central example motivating all of abstract homotopy theory.

The next revolution came in the late 1990s and 2000s. Joyal realized that a slight weakening of the Kan condition — requiring only inner horn fillings, not outer — gives a model for (∞,1)-categories (what Joyal called quasi-categories or weak Kan complexes). This observation led to the rich theory of ∞-categories developed by Joyal, Lurie, and many others. Voevodsky's simplicial set model of HoTT (2009–2010) showed that Kan complexes were not just models of homotopy types but also models of *types* in a constructive foundation for mathematics. This connection — simplicial sets as the bridge between homotopy theory and foundations — is still actively being developed.

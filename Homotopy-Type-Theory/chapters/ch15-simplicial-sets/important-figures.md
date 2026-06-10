# Important Figures

## Samuel Eilenberg (1913–1998)
*Co-founder of homological algebra and algebraic topology; introduced the singular chain complex and co-developed the first systematic treatment of simplicial structures.*

Samuel Eilenberg was born in Warsaw and completed his doctorate there before emigrating to the United States in 1939. His collaboration with Saunders Mac Lane at Columbia University produced the category theory program as a byproduct of their work on natural transformations in algebraic topology — they invented categories in order to define "naturality" in the comparison of homology theories. Eilenberg spent most of his career at Columbia, where he also became a distinguished collector of Indian art.

Eilenberg's direct contributions to the theory of simplicial sets stem from two directions. First, with Norman Steenrod he axiomatized singular homology theory (the Eilenberg-Steenrod axioms, 1945), establishing the singular chain complex as the central tool for computing homological invariants of topological spaces. The singular chain complex is, in essence, the shadow of the singular complex functor $\mathsf{Sing}$ that appears in this chapter — it records only the homological information. Second, with Joseph Zilber he proved the Eilenberg-Zilber theorem (1950): there is a natural chain homotopy equivalence $C_*(X \times Y) \simeq C_*(X) \otimes C_*(Y)$, which allows the computation of homology of products. The proof introduced what are now called "semi-simplicial complexes" and established the combinatorial machinery of face maps.

Eilenberg's influence on simplicial homotopy theory is foundational even though he did not himself develop the full theory of simplicial sets (with degeneracies). He created the language — singular simplices, chain complexes, natural transformations — that Kan and Quillen later assembled into simplicial homotopy theory. His collaboration with Mac Lane on Eilenberg-Mac Lane spaces remains central: these spaces appear naturally as simplicial sets (their singular complexes), and they are the basic building blocks from which all Kan complexes are assembled via Postnikov towers.

---

## Joseph Zilber (1923–2009)
*Collaborator with Eilenberg on the Eilenberg-Zilber theorem; one of the first to systematically use simplicial structures to compute homological invariants.*

Joseph Zilber received his doctorate from Harvard in 1954 and spent his career at the University of Massachusetts. His primary influence on our subject comes through the 1950 paper with Eilenberg, "Semi-Simplicial Complexes and Singular Homology," which appeared in the *Annals of Mathematics*. The paper introduced the term "semi-simplicial complex" for what we now call a simplicial set without degeneracies (sometimes called a $\Delta$-set), and proved the fundamental theorem relating the homology of a Cartesian product to the tensor product of the homology of its factors.

The Eilenberg-Zilber theorem has a simplicial-set proof that is purely combinatorial: there is an "Alexander-Whitney" map $C_*(X \times Y) \to C_*(X) \otimes C_*(Y)$ and a "shuffle" map in the other direction, and these are chain homotopy inverses. Both maps are defined using the simplicial structure — face and degeneracy maps — and the proof that they compose to a homotopy uses only simplicial identities. This makes the theorem a paradigmatic example of the combinatorial power of the simplicial formalism.

Zilber's contribution is sometimes underappreciated because his later career moved toward areas of mathematics unrelated to topology. But the paper he wrote with Eilenberg established the notation, terminology, and methods that Kan immediately adopted and extended into the full theory of Kan complexes. In particular, the "Alexander-Whitney diagonal approximation" used in the proof is related to the coassociativity of the diagonal on a simplicial set — a structure that reappears in the theory of A∞-algebras and the formality of the cochains of a space.

---

## Daniel Kan (1927–2013)
*The founding figure of combinatorial homotopy theory: introduced Kan complexes, Kan fibrations, adjoint functors, and the homotopy groups of simplicial sets — nearly single-handedly built the combinatorial infrastructure underlying simplicial homotopy theory.*

Daniel Kan was born in the Netherlands and completed his doctorate at the Hebrew University of Jerusalem in 1955, under Samuel Eilenberg's influence. He spent his entire academic career at MIT, where he supervised many students who went on to be leaders in algebraic topology and homotopy theory.

In 1958, Kan published two papers that changed mathematics. "A Combinatorial Definition of Homotopy Groups" introduced what are now called Kan complexes: simplicial sets in which every horn $\Lambda^n_k \to X$ can be filled. Kan showed that homotopy groups can be defined purely combinatorially for such simplicial sets, without reference to topology, and that the homotopy groups of $\mathsf{Sing}(X)$ (for a topological space $X$) agree with the classical homotopy groups of $X$. In the same year, "Adjoint Functors" in the *Transactions of the AMS* introduced adjoint functors in full generality and proved — as the main example — that geometric realization $|\text{-}|$ and the singular complex $\mathsf{Sing}$ are adjoint. This adjunction is now one of the most important examples of an adjunction in all of mathematics, and it underpins the Quillen equivalence between simplicial sets and topological spaces.

Kan also introduced Kan fibrations (the "fibrations" in the Quillen model structure — simplicial maps with the right lifting property against horn inclusions), the loop group functor $G : \mathbf{sSet}_* \to \mathbf{sGrp}$ (simplicial analogue of the loop space), and the $\mathsf{Ex}^\infty$ functor (an explicit fibrant replacement for simplicial sets, used in the proof that every simplicial set has a Kan complex approximation). His 1970 paper "On c.s.s. Complexes" established many of the fundamental properties of simplicial homotopy theory. Every major construction in this chapter — Kan complexes, Kan fibrations, horn fillers, the adjunction $(|\text{-}|, \mathsf{Sing})$ — is due to Kan.

---

## Daniel Quillen (1940–2011)
*Introduced model categories and proved the Quillen model structure on simplicial sets, establishing the equivalence between simplicial homotopy theory and classical homotopy theory; one of the most original mathematicians of the 20th century.*

Daniel Quillen was born in Orange, New Jersey, studied at Harvard (B.A. 1961, Ph.D. 1964 under Raoul Bott), and spent his career primarily at MIT. He was awarded the Fields Medal in 1978 for his work on algebraic K-theory — the resolution of the Adams conjecture, the development of higher algebraic K-theory, and the proof of the Quillen-Lichtenbaum conjecture. His contributions to simplicial homotopy theory came earlier and were, in a sense, the technical foundation on which his later work rested.

Quillen's 1967 monograph *Homotopical Algebra* introduced the axioms for a model category: a category equipped with three distinguished classes of morphisms (weak equivalences, fibrations, cofibrations) satisfying lifting properties and factorization axioms. He proved the fundamental theorem: there is a model structure on the category of simplicial sets (the "Quillen model structure") in which the fibrant objects are exactly the Kan complexes, and the weak equivalences are the simplicial maps whose geometric realizations are weak homotopy equivalences. He then proved that this model structure is Quillen equivalent to the model structure on topological spaces, making precise the informal idea that simplicial sets and topological spaces "carry the same homotopy-theoretic information." Every fibration in this chapter — including the fibrant replacement functor and the factorization of maps as cofibration followed by acyclic fibration — is constructed using Quillen's small object argument, another invention from the same monograph.

Quillen's broader legacy is the entire field of abstract homotopy theory. By axiomatizing what it means for a category to "have homotopy theory," he made it possible to do homotopy theory in settings far removed from topological spaces: chain complexes, simplicial commutative rings, operads, presheaf categories. His model-categorical viewpoint is the framework within which Joyal's quasi-category theory, Lurie's ∞-topos theory, and Voevodsky's simplicial set model of HoTT are all formulated.

---

## André Joyal (1943–present)
*Introduced quasi-categories as combinatorial models of (∞,1)-categories and proved the Joyal model structure on simplicial sets; originator of the theory of ∞-categories in its modern simplicial form.*

André Joyal is a Canadian mathematician who has spent his career at the Université du Québec à Montréal. He is a broadly influential figure whose contributions span category theory, topos theory, combinatorics (the theory of species), and higher category theory. His work is characterized by deep conceptual clarity and a willingness to work on difficult problems for long periods before publishing.

The central contribution to this chapter is the discovery, in the 1980s (published partially in 2002 and more fully in his 2008 Barcelona lectures), that simplicial sets satisfying only the inner horn-filling condition — now called quasi-categories or ∞-categories — provide a combinatorial model for (∞,1)-categories: categories with objects, morphisms, 2-morphisms, ..., all of which are invertible above dimension 1. This is a weakening of the Kan condition (which requires all horns to be filled): quasi-categories fill only horns $\Lambda^n_k$ with $0 < k < n$ (inner horns). Joyal proved that there is a model structure on $\mathbf{sSet}$ — now called the Joyal model structure — in which the fibrant objects are exactly the quasi-categories, and the fibrations are "isofibrations" (maps with the right lifting property against inner horn inclusions and the inclusion $\{1\} \hookrightarrow J$, where $J$ is the "walking isomorphism"). This model structure is Quillen equivalent to Bergner's model structure on simplicial categories.

Joyal's influence extends through the work of his student Jacob Lurie, whose development of ∞-topos theory in *Higher Topos Theory* (2009) is built entirely on quasi-categories. The Joyal model structure, and the theory of quasi-categories, is now the standard framework for higher category theory and appears throughout modern algebraic topology and algebraic geometry.

---

## Charles Rezk (1969–present)
*Introduced complete Segal spaces as an alternative model for (∞,1)-categories; established the Rezk model structure and the comparison between different models of higher categories.*

Charles Rezk received his doctorate from MIT in 1996 (under Michael Hopkins) and is currently a professor at the University of Illinois at Urbana-Champaign. His work focuses on homotopy theory, higher category theory, and their connections to algebraic geometry and elliptic cohomology.

The key contribution to this chapter's setting is Rezk's 2001 paper "A Model for the Homotopy Theory of Homotopy Theory" (*Trans. Amer. Math. Soc.*), which introduced complete Segal spaces. A complete Segal space is a bisimplicial set (a simplicial object in the category of simplicial sets) satisfying the "Segal condition" (composition is defined up to contractible choice, just as in a quasi-category) and a "completeness condition" (the equivalences — the invertible 1-cells — are parametrized by the space of objects). Rezk proved that there is a model structure on bisimplicial sets in which the fibrant objects are exactly the complete Segal spaces, and that this model structure is Quillen equivalent to the Joyal model structure on simplicial sets and to Bergner's model structure on simplicial categories. This "comparison theorem" established that all the competing models of (∞,1)-categories are equivalent — a crucial structural result.

Rezk's completeness condition has an important interpretation for HoTT: in Voevodsky's simplicial model, the universe of types is a complete Segal space when one takes morphisms to be equivalences. The completeness condition says roughly that "being an equivalence" is determined by the identity types in the model — a simplicial shadow of the Univalence axiom.

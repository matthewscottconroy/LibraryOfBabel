# Important Thinkers: Simplicial Sets and the HoTT Model

## Samuel Eilenberg (1913–1998) and J.A. Zilber

Simplicial sets in their modern form were introduced by Eilenberg and Zilber in their 1950 paper "Semi-simplicial complexes and singular homology." Before their work, the combinatorial approach to topology used simplicial complexes — finite systems of simplices with compatibility conditions. Eilenberg and Zilber introduced the crucial innovation of degeneracy maps: maps that insert "degenerate" simplices (a point viewed as an edge, an edge viewed as a triangle, etc.). This generalization — from simplicial complexes to simplicial sets — made the theory algebraically cleaner and more powerful.

The effect of degeneracy maps was immediately felt: singular homology, previously defined by a complicated normalization procedure, became the straightforward homology of the simplicial abelian group of singular chains. The Eilenberg-Zilber theorem (that the diagonal of a bisimplicial set is equivalent to the total complex) became a cornerstone of the theory. The formalism of simplicial sets made the relationship between topology and algebra transparent, and the subsequent development of simplicial homotopy theory — Kan complexes, model structures — built directly on their work.

## Daniel Kan (1927–2013)

Kan was the founder of simplicial homotopy theory proper. His 1956 paper "Abstract Homotopy I" introduced what are now called *Kan complexes*: simplicial sets satisfying the horn-filling condition. He was motivated by the question of which simplicial sets correctly model homotopy types — and the answer turned out to be the Kan complexes. He proved that the singular complex $\text{Sing}(Y)$ of any topological space is a Kan complex, and that the homotopy groups of a Kan complex (defined combinatorially) agree with those of its geometric realization.

Kan also introduced *Kan extensions* (the most general notion of "extending" a functor along another functor, now fundamental in category theory), *Kan fibrations* (the simplicial fibrations that form the fibrations in Quillen's model structure), and the *Dold-Kan correspondence* (an equivalence between simplicial abelian groups and chain complexes, connecting simplicial algebra to homological algebra). He spent his career at MIT, where he influenced generations of algebraic topologists and category theorists. His influence on the foundations of simplicial homotopy theory is pervasive: many of the core definitions (Kan complex, Kan fibration, Kan extension) bear his name.

## Daniel Quillen (1940–2011)

Quillen's 1967 book *Homotopical Algebra* is one of the most influential pieces of mathematics of the twentieth century. It introduced the notion of a *model category* and proved the Quillen model structure on simplicial sets, establishing the Quillen equivalence between simplicial sets and topological spaces. By doing so, it showed that simplicial homotopy theory and classical homotopy theory are the same subject — a profound unification.

Quillen was also a Fields Medalist (1978) for his work on algebraic K-theory. His Q-construction and plus construction gave a clean definition of the higher K-groups of a ring, and his calculation of $K(\mathbb{F}_q)$ for finite fields was a landmark. These constructions make essential use of simplicial sets and model categories, and they were one of the driving applications of the abstract machinery.

Quillen's approach was characterized by an unusual combination of algebraic sophistication and geometric intuition. His model category axioms are abstract enough to apply to many settings but concrete enough to be checkable. The Quillen equivalence between $\mathbf{sSet}$ and $\mathbf{Top}$ is his great contribution to the foundations of homotopy theory, and the subsequent development of the theory of $(\infty, 1)$-categories (Lurie, Joyal, Rezk) builds directly on his foundations.

## Alexander Grothendieck (1928–2014)

Grothendieck's influence on the mathematics of this chapter is indirect but fundamental. His *homotopy hypothesis* (stated in a 1983 letter to Quillen, published as "Pursuing Stacks") conjectured that homotopy types should be the same as $\infty$-groupoids — and that the right model for $\infty$-groupoids should be combinatorial (simplicial or similar). This conjecture, now proved in various forms, is the mathematical foundation for the identification of Kan complexes with homotopy types.

Grothendieck also developed the theory of toposes — categories of sheaves on a site — which is the correct setting for Voevodsky's model. The simplicial set model of HoTT is built in the $\infty$-topos of simplicial sets (or, more precisely, in the Quillen model structure, which is the presenting structure for this $\infty$-topos). Grothendieck's vision of cohomology theory as intrinsically "sheaf-theoretic" and "$\infty$-categorical" anticipated much of what Voevodsky later formalized.

## Vladimir Voevodsky (1966–2017)

Voevodsky's simplicial set model of HoTT is the culmination of this chapter. His Fields Medal (2002) was for work on motivic cohomology and the proof of the Milnor conjecture, which required developing a theory of motivic homotopy theory — homotopy theory for algebraic varieties, built using simplicial sheaves over algebraic sites. The model structures and simplicial set techniques that Voevodsky developed for motivic homotopy theory were directly transplanted into the construction of the HoTT model.

When Voevodsky turned to foundations in the mid-2000s, he brought with him a deep understanding of simplicial sets, model categories, and the relationship between type theory and homotopy theory. The simplicial set model is, in a sense, the application of motivic homotopy techniques to the foundations of mathematics itself: instead of sheaves over algebraic varieties, one works with simplicial sets over the terminal topos; instead of motivic cohomology, one studies identity types.

Voevodsky spent the last decade of his life developing HoTT and the univalent foundations program at the Institute for Advanced Study in Princeton. He wrote code in the Coq proof assistant, developing libraries of formalized mathematics. He gave a series of lectures on the Univalent Foundations program that are available online and are essential reading for anyone who wants to understand what motivated the development of HoTT. He died in September 2017 at the age of 51.

## Jacob Lurie (b. 1977)

Lurie's *Higher Topos Theory* (2009) and *Higher Algebra* (2017) are the comprehensive treatments of $(\infty, 1)$-categories and their applications. The $(\infty, 1)$-topos of simplicial sets is one of the central objects of study, and Lurie's straightening/unstraightening construction — which establishes the equivalence between fibrations over a base and functors into the "space of types" — is precisely the mathematical mechanism that makes Voevodsky's model of univalence work.

Lurie's work makes precise the sense in which HoTT is the "internal language" of an $(\infty, 1)$-topos. The type-forming operations of HoTT correspond to the categorical operations (limits, colimits, fibrations) in the $(\infty, 1)$-topos, and the univalence axiom corresponds to the *object classifier* of the $(\infty, 1)$-topos. The mathematical connection between Lurie's $(\infty, 1)$-topos theory and Voevodsky's HoTT model is now well understood, and it is one of the deepest results in the foundations of mathematics.

# References and Primary Sources

## Foundational Texts

**Samuel Eilenberg and Saunders Mac Lane.** "General Theory of Natural Equivalences." *Transactions of the American Mathematical Society* 58, 1945.
The paper that invented category theory. Eilenberg and Mac Lane introduce categories, functors, and natural transformations in order to make precise the naturality of the double-dual embedding of a vector space. Worth reading in the original — it is remarkably readable for a paper that created an entire field.

**Saunders Mac Lane.** *Categories for the Working Mathematician.* Springer, 1st ed. 1971; 2nd ed. 1998. (GTM 5)
The canonical graduate text on category theory. Mac Lane's book covers categories, functors, natural transformations, limits, adjunctions, monads, abelian categories, and ends/coends. The second edition added a chapter on symmetric monoidal categories. Dense, authoritative, and written by one of the theory's founders — every serious student should own a copy.

**Saunders Mac Lane and Ieke Moerdijk.** *Sheaves in Geometry and Logic: A First Introduction to Topos Theory.* Springer, 1992. (Universitext)
After Mac Lane's CWM, this is the most important single-volume reference for connections between category theory and logic/type theory. The chapters on elementary toposes, Lawvere-Tierney topologies, and the internal language are essential for understanding the categorical semantics of MLTT. The geometric examples (sheaves on spaces) motivate the abstract machinery.

**Alexander Grothendieck.** "Sur quelques points d'algèbre homologique." *Tôhoku Mathematical Journal* 9, 1957.
Known as "the Tohoku paper." Grothendieck essentially invented homological algebra as it is practiced today, introducing abelian categories, exact functors, derived functors, and the framework of sheaves with the language of category theory fully integrated. This paper initiated the use of categorical methods that would eventually become category theory's central applications in algebraic geometry.

**F. William Lawvere.** *Functorial Semantics of Algebraic Theories.* PhD thesis, Columbia University, 1963. Republished in *Reprints in Theory and Applications of Categories* 5, 2004.
Lawvere's thesis introduced Lawvere theories — a categorical framework for universal algebra that replaced the set-theoretic treatment of algebraic structures with a purely categorical one. This is the foundational document for understanding how category theory models algebraic structures, and it is the precursor to the theory of monads.

---

## Seminal Papers

**Samuel Eilenberg and Saunders Mac Lane.** "General Theory of Natural Equivalences." *Transactions of the AMS* 58, 1945.
(See above.) Not merely historically significant: the paper already contains the core ideas of functoriality and naturality in their modern form. Reading the original is worthwhile for seeing how the concepts were motivated by concrete topology problems before being abstracted.

**Daniel M. Kan.** "Adjoint Functors." *Transactions of the American Mathematical Society* 87, 1958.
The paper that introduced adjoint functors, one of the most important concepts in all of mathematics. Kan defines adjunctions (in the unit-counit formulation), proves the adjoint functor theorem, and gives the first systematic examples. Every adjunction in modern mathematics — free groups, tensor-hom, direct/inverse image, etc. — fits into Kan's framework.

**F. William Lawvere.** "Adjointness in Foundations." *Dialectica* 23, 1969.
Lawvere's philosophical paper arguing that adjunctions are the basic structural principle of logic and mathematics. The quantifiers ∀ and ∃ are adjoints to substitution; the modalities of modal logic are parts of adjunctions; the relationship between syntax and semantics in logic is an adjunction. Essential for understanding why category theory is relevant to logic and type theory.

**Nobuo Yoneda.** (Letter to Saunders Mac Lane; the Yoneda Lemma was communicated informally circa 1954.)
The Yoneda Lemma — that natural transformations from a representable functor $\mathsf{Hom}(-, A)$ to $F$ are in bijection with elements of $F(A)$ — was first communicated by Nobuo Yoneda to Mac Lane in a conversation at the Gare du Nord in Paris in 1954 (according to Mac Lane's account). Yoneda never published a paper on this result, but Mac Lane recorded it in *Categories for the Working Mathematician*. The lemma is perhaps the deepest elementary theorem in category theory.

**Peter Freyd.** "Abelian Categories." Harper and Row, 1964. (Also: "Aspects of Topoi," *Bulletin of the Australian Mathematical Society* 7, 1972.)
Freyd's work on abelian categories systematized the categorical treatment of homological algebra; his later work on topoi (independent of Lawvere-Tierney) contributed to the foundations of categorical logic.

**Max Kelly and Ross Street.** "Review of the Elements of 2-Categories." *Lecture Notes in Mathematics* 420, Springer, 1974.
The foundational text for 2-category theory, the precursor to higher category theory. Categories themselves form a 2-category (Cat), and understanding Cat requires 2-categorical tools. Kelly and Street's work is the starting point for the higher categorical structures that appear in HoTT.

---

## Textbooks and Modern Treatments

**Steve Awodey.** *Category Theory.* Oxford University Press, 2nd ed. 2010. (Oxford Logic Guides)
Level: graduate/advanced undergraduate. A modern, logically-oriented introduction to category theory, with emphasis on the connections to logic and type theory. Awodey's treatment of the internal language of a CCC and the semantics of the simply-typed lambda calculus is clearer than any other textbook treatment. Essential reading for this curriculum.

**Emily Riehl.** *Category Theory in Context.* Dover, 2016. Freely available at [https://math.jhu.edu/~eriehl/context.pdf](https://math.jhu.edu/~eriehl/context.pdf).
Level: advanced undergraduate to early graduate. Riehl's book is distinguished by its thorough treatment of the representability perspective on universal properties and its careful treatment of the Yoneda lemma and adjunctions. The appendix on 2-categorical perspectives is valuable. Freely available online.

**Tom Leinster.** *Basic Category Theory.* Cambridge University Press, 2014. Also freely on arXiv at [https://arxiv.org/abs/1612.09375](https://arxiv.org/abs/1612.09375).
Level: undergraduate. A concise, clean introduction covering categories, functors, natural transformations, the Yoneda lemma, limits, and adjunctions in around 180 pages. Leinster's exposition is exceptionally clear, and the book's brevity makes it excellent for a first pass before working through Awodey or Riehl.

**Bart Jacobs.** *Categorical Logic and Type Theory.* Elsevier, 1999. Freely available from the author's website.
Level: research. A comprehensive reference for the correspondence between category theory and type theory, covering fibered categories, hyperdoctrines, and the categorical semantics of dependent type theory. The most thorough treatment of the topics in Chapter 11 of this curriculum.

**David Spivak.** *Category Theory for the Sciences.* MIT Press, 2014. Freely available at [https://arxiv.org/abs/1302.6946](https://arxiv.org/abs/1302.6946).
Level: undergraduate / applied. Spivak's book motivates category theory through scientific applications — databases, networks, systems — and is the source of the "categorical databases" paradigm (database schemas as categories, queries as functors). Useful for seeing category theory through an applied lens.

---

## Online Resources and Lecture Notes

**nLab: Category Theory.**
[https://ncatlab.org/nlab/show/category+theory](https://ncatlab.org/nlab/show/category+theory)
The nLab is the essential online reference for category theory, with articles ranging from basic definitions to research-level topics. Its treatment of adjunctions, monads, and higher category theory is particularly strong. Use it for cross-referencing and for looking up unfamiliar terminology.

**The Stacks Project.**
[https://stacks.math.columbia.edu/](https://stacks.math.columbia.edu/)
A collaboratively written algebraic geometry textbook that is also one of the most thorough expositions of categories and sheaves available. The early chapters on categories and sites are accessible and precise.

**Eugenia Cheng.** *The Joy of Abstraction.* Cambridge University Press, 2022. (Also: YouTube lectures on category theory.)
Cheng's YouTube series (search "Eugenia Cheng category theory") provides an exceptionally accessible visual and conceptual introduction to category theory. Recommended as a supplement for building geometric intuition about categories, functors, and natural transformations.

**Mathlib4 `CategoryTheory` module.**
[https://leanprover-community.github.io/mathlib4_docs/Mathlib/CategoryTheory/](https://leanprover-community.github.io/mathlib4_docs/Mathlib/CategoryTheory/)
The Lean 4 implementation of category theory in Mathlib. Every major concept from this chapter — categories, functors, natural transformations, limits, adjunctions, the Yoneda embedding — is formalized here. Reading the Lean source alongside the mathematical exposition is one of the most effective ways to achieve precise understanding.

**Paolo Perrone.** *Notes on Category Theory with Examples from Basic Mathematics.* arXiv:1912.10642.
A recent set of notes (regularly updated) with an emphasis on concrete examples from all branches of mathematics — not just topology and algebra but also probability theory, measure theory, and analysis. Particularly useful for readers who want to see how categorical concepts apply across different mathematical domains.

---

## Historical Context

Category theory was created in 1945 as a side effect of algebraic topology. Eilenberg and Mac Lane were working on the relationship between topological spaces and their algebraic invariants (homology groups, homotopy groups) and noticed that many constructions were "natural" — they commuted with continuous maps in a systematic way. To state this precisely, they needed the concept of a functor (a map between categories that preserves the categorical structure) and then a natural transformation (a systematic comparison between two functors). The notions of "category" and "functor" were introduced as scaffolding for making "natural transformation" precise.

The theory developed rapidly in the 1950s and 1960s. Kan's 1958 paper on adjoint functors showed that the concept of adjunction unified an enormous range of seemingly unrelated constructions: free objects, tensor products, products and coproducts, and the quantifiers of logic were all adjoints. Grothendieck's Tohoku paper (1957) brought category theory to the heart of algebraic geometry: sheaves, sites, toposes, and the language of abelian categories reorganized homological algebra and made possible the proof of the Weil conjectures (Grothendieck and Deligne). Lawvere's 1963 thesis and 1969 paper "Adjointness in Foundations" moved category theory into logic and the foundations of mathematics: Lawvere showed that the logical connectives, quantifiers, and substitution of predicate logic are all adjunctions, turning logic into a branch of category theory.

The impact on type theory and computer science came through two channels. First, the discovery (Lambek 1969, Scott 1970) that cartesian closed categories provide the denotational semantics of the simply-typed lambda calculus made it clear that the type structures of programming languages are categorical structures. Second, the development of topos theory (Lawvere-Tierney, 1969–1970) showed that categories could model entire logical universes, with the internal logic of a topos being an intuitionistic higher-order logic. The current importance of category theory for HoTT flows directly from these two lines of development: HoTT's semantics lives in $\infty$-toposes, and the entire internal language apparatus of categorical logic is the ancestor of the type-theoretic language of HoTT.

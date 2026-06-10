# Important Figures

## Alexander Grothendieck (1928–2014)
*Algebraic geometer; originator of the homotopy hypothesis and the vision of ∞-groupoids as foundational objects.*

Grothendieck is one of the most influential mathematicians of the 20th century. Born in Berlin to anarchist parents, he spent years as a stateless person before establishing himself in Paris. During the 1950s and 1960s he rebuilt algebraic geometry from scratch using the language of schemes and sheaves, introduced étale cohomology and crystalline cohomology, constructed the theory of toposes, and proved the Weil conjectures (through the work of Deligne, who carried out the last step). He was awarded the Fields Medal in 1966 but declined the Crafoord Prize in 1988, and withdrew from mathematics in 1991.

What matters most for this chapter is his 1983 manuscript "Pursuing Stacks," written in the form of a long letter to Daniel Quillen and circulated informally. In it, Grothendieck conjectured — with remarkable precision and foresight — that homotopy types of spaces could be completely algebraized using a suitable notion of ∞-groupoid: a structure with objects, morphisms, 2-morphisms, and so on at every level, all invertible. He envisioned a globular approach (cells of all dimensions) and sketched the program of constructing a fundamental ∞-groupoid $\Pi_\infty(X)$ of a space, capturing its full homotopy type. This is now called the **homotopy hypothesis**: homotopy types are the same as ∞-groupoids.

Grothendieck's influence on this chapter is foundational in the most literal sense: the central theorem connecting higher categories to topology — and connecting HoTT's identity types to homotopy theory — is precisely his conjecture. His insistence that weak higher structure (not strict) is the natural object captures a truth that took decades for the community to fully absorb. Every development in ∞-category theory, from Kan complexes to quasi-categories to complete Segal spaces, is an attempt to make Grothendieck's vision rigorous.

---

## Michael Boardman (1938–2021) and Rainer Vogt (1942–2015)
*Algebraic topologists; inventors of quasi-categories and homotopy-coherent algebraic structures.*

J. Michael Boardman and Rainer M. Vogt collaborated on the problem of making homotopy-coherent structures precise, culminating in their 1973 monograph *Homotopy Invariant Algebraic Structures on Topological Spaces* (Lecture Notes in Mathematics 347). At the time, homotopy theorists had noticed that many natural algebraic structures in topology — loop spaces, spectra, operadic algebras — only satisfied their axioms up to homotopy, with the homotopies themselves satisfying higher coherence conditions, and so on. Making this precise was the challenge.

Boardman and Vogt introduced the concept of a **weak Kan complex** (a simplicial set satisfying the inner horn-filling conditions), which they used to define "homotopy everything" structures. They also introduced the key notion of what is now called a **quasi-category**: a simplicial set in which every inner horn $\Lambda^n_k \hookrightarrow \Delta^n$ (for $0 < k < n$) can be filled, but the filling is not required to be unique. This is the condition encoding composition that is associative and unital up to coherent homotopy, without requiring strict algebraic axioms.

Boardman passed away in 2021 and Vogt in 2015. Their contribution was only fully appreciated decades after it was made: Joyal recognized in the late 1990s that their weak Kan complexes (quasi-categories) were the right model for (∞,1)-categories, and Lurie built the entire edifice of higher topos theory on this foundation. The concept Boardman and Vogt invented in 1973 became, in the 2000s, the standard model for ∞-category theory.

---

## André Joyal (1943–)
*Category theorist; inventor of the Joyal model structure and the systematic theory of quasi-categories.*

André Joyal is a professor at the Université du Québec à Montréal, known for deep contributions ranging from combinatorics (the Joyal correspondence between species and power series) to topos theory to the foundations of ∞-category theory.

In the late 1990s and early 2000s, Joyal recognized that Boardman and Vogt's quasi-categories — which had been defined in 1973 but not fully developed — were the correct model for homotopy-coherent category theory. He constructed the **Joyal model structure** on the category of simplicial sets $\mathsf{sSet}$: a model category in which the fibrant objects are exactly the quasi-categories and the weak equivalences are "categorical equivalences" (maps inducing equivalences of homotopy categories). This is different from the Quillen model structure (whose fibrant objects are Kan complexes, modeling ∞-groupoids); the Joyal model structure models (∞,1)-categories.

Joyal's unpublished 2002 notes circulated widely among topologists and category theorists and contained the core results that Lurie would later systematize in Higher Topos Theory. Joyal proved that the Joyal model structure exists, that quasi-categories have an internal theory of functors (morphism spaces), and that they support the main categorical constructions (limits, colimits, adjunctions, Kan extensions). He also proved that the Joyal and Rezk models for (∞,1)-categories are Quillen equivalent, establishing that the two main approaches give the same answers.

---

## Charles Rezk (1969–)
*Algebraic topologist; inventor of complete Segal spaces, a model of (∞,1)-categories using bisimplicial sets.*

Charles Rezk is a professor at the University of Illinois at Urbana-Champaign. His 2001 paper "A Model for the Homotopy Theory of Homotopy Theory" (*Transactions of the AMS* 353) introduced **complete Segal spaces** as a model for (∞,1)-categories.

The key idea: an (∞,1)-category should be a simplicial space $X : \Delta^{op} \to \mathsf{sSet}$ satisfying two conditions. The **Segal condition** says that $X_n \simeq X_1 \times_{X_0} X_1 \times_{X_0} \cdots \times_{X_0} X_1$ (the space of composable $n$-tuples of morphisms is equivalent to $n$-fold fiber products of the space of single morphisms) — this captures that composition is well-defined. The **completeness condition** says that the space of "equivalences" in $X$ is equivalent to the space $X_0$ of objects — this captures that the only objects related by equivalences are equal (a Segal version of univalence).

Complete Segal spaces have the advantage that the $\infty$-categorical structure is fully visible as simplicial data, without the need to phrase things in terms of horn-filling as in quasi-categories. Rezk proved that his model category of complete Segal spaces is Quillen equivalent to the Joyal model structure on simplicial sets, establishing that quasi-categories and complete Segal spaces model the same notion of (∞,1)-category. The completeness condition in complete Segal spaces is strikingly analogous to the univalence axiom in HoTT — in both cases, the condition says that "equivalences are equalities."

---

## Ross Street (1945–)
*Category theorist; pioneer of strict ω-categories, orientals, and the algebraic approach to higher categories.*

Ross Street is an emeritus professor at Macquarie University in Sydney, a key figure in the Australian school of category theory. His contributions to higher category theory span from strict ω-categories in the 1980s through enriched category theory and the theory of orientals.

Street introduced **orientals** — the free strict ω-categories generated by the formal $n$-simplex — in his 1987 paper "The algebra of oriented simplexes." These are the strict analogues of simplices: each oriental $\mathcal{O}(n)$ captures a strict higher-categorical structure appropriate for $n$-dimensional "pasting diagrams." The orientals are important because they show how to interpret simplicial combinatorics in terms of higher-categorical operations, providing a bridge between strict higher categories and the simplicial approach.

Street also formulated and developed the theory of **2-categories** and **bicategories** in the Australian school, producing clean and general treatments of 2-categorical limit and colimit theory. His work on **Yoneda lemmas for bicategories** and on **formal theory of monads** established the basic tools of 2-category theory that are prerequisites for higher-categorical reasoning. His **comprehensive factorization system** and his work on **fibrations in 2-categories** laid the groundwork for the theory of (∞,1)-categorical fibrations. Though his strict ω-categories are known to be insufficient (by Simpson's theorem) to model all homotopy types, they remain an important conceptual foundation, and the theory of computads and pasting schemes that Street developed is used throughout modern ∞-category theory.

---

## Jacob Lurie (1977–)
*Mathematician; author of Higher Topos Theory and the systematic modern development of ∞-categories.*

Jacob Lurie received his Ph.D. from MIT in 2004 under Michael Hopkins and is now a professor at the Institute for Advanced Study. He is the principal architect of the modern theory of ∞-categories and ∞-toposes.

Lurie's 2009 book *Higher Topos Theory* (arXiv:math/0608040) systematically develops quasi-categories as a foundation for (∞,1)-category theory, proving that quasi-categories support all the expected categorical structures — limits and colimits, adjunctions, presentable ∞-categories, Cartesian and coCartesian fibrations — and that the resulting theory is equivalent to that of complete Segal spaces and simplicially enriched categories. The key theorem of the book is the **Giraud axioms for ∞-toposes**: an ∞-category is an ∞-topos if and only if it satisfies the ∞-categorical analogues of the classical Giraud axioms (colimits are universal, effective descent).

Lurie's companion text *Higher Algebra* develops monoidal ∞-categories, operads, and structured ring spectra in the ∞-categorical framework. His preprint on the classification of topological field theories proves the **cobordism hypothesis** (conjectured by Baez-Dolan in 1995): fully extended TFTs are classified by fully dualizable objects in symmetric monoidal (∞,n)-categories. This is the flagship application of ∞-categorical machinery.

For HoTT specifically, Lurie's work establishes that HoTT is the internal language of ∞-toposes (together with work of Shulman and others). Every ∞-topos models HoTT, meaning that every theorem of HoTT holds internally in every ∞-topos simultaneously. This makes HoTT not just a formal system but a tool for proving ∞-topos-theoretic theorems.

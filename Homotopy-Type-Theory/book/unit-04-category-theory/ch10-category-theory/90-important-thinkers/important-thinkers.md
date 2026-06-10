# Important Thinkers: Category Theory

## Samuel Eilenberg (1913–1998)

Eilenberg co-invented category theory with Mac Lane in a 1945 paper on algebraic topology. A Polish mathematician who emigrated to the United States in 1939, Eilenberg was one of the founders of modern algebraic topology. His work with Mac Lane on homology theories required a precise notion of "natural transformation" — a concept that required the invention of categories and functors to state properly. Eilenberg went on to make fundamental contributions to homological algebra (the Eilenberg-Moore spectral sequence), algebraic topology, and automata theory. His textbook *Homological Algebra* (with Cartan) shaped an entire generation of algebraists.

## Saunders Mac Lane (1909–2005)

Mac Lane co-invented category theory and spent the rest of his career developing it. His 1971 textbook *Categories for the Working Mathematician* remains the standard reference. Mac Lane was the first to see that category theory was not just a language for algebraic topology but a foundational framework applicable throughout mathematics. He introduced monads (which he called "triples"), developed the theory of adjunctions, and proved the coherence theorems for monoidal categories. Mac Lane's connection to logic was deep: he saw category theory as providing the right notion of "mathematical structure" — a notion that set theory, with its focus on membership, could not adequately capture.

## William Lawvere (born 1937)

Lawvere revolutionized the relationship between category theory and logic. His 1963 doctoral thesis introduced the *Elementary Theory of the Category of Sets* (ETCS) — an axiomatization of mathematics using categorical language rather than membership-based set theory. Lawvere introduced *adjoint functors* as the fundamental organizing principle of mathematics ("Adjoint functors arise everywhere"). He developed the concept of *Lawvere theories* (categorical treatment of algebraic structures), *categorical logic* (identifying logical operations with categorical constructions), *elementary toposes* (with Tierney), and *cohesion* (an axiomatic framework for differential geometry). His work is foundational to HoTT's categorical semantics.

## Myles Tierney (1937–2017)

Tierney worked with Lawvere to develop the theory of *elementary toposes* in the early 1970s. An elementary topos is a category satisfying a short list of axioms — finite limits, exponentials, a subobject classifier — that captures the essential structure of the category of sets. Every topos has an internal intuitionistic logic; this is the logical content of the topos axioms. Tierney and Lawvere's work showed that elementary toposes provided a logic-independent foundation for mathematics, in which the choice of topos determines what "logical principles" hold. Sheaf toposes, in particular, led to independence proofs and new models of set theory.

## Peter Johnstone (born 1948)

Johnstone is the world authority on topos theory, having written the comprehensive *Sketches of an Elephant: A Topos Theory Compendium* (two volumes of a projected three). His earlier textbook *Topos Theory* (1977) introduced the subject to a generation of mathematicians. Johnstone has shown how topos theory connects to sheaf theory, locale theory (topology without points), and logic. His work on geometric morphisms (maps between toposes), localic toposes (toposes arising from locales), and internal category theory in a topos has been definitive. The *Elephant* is the standard reference for anyone working in topos theory.

## F. William Lawvere and the Development of Adjointness

(Lawvere is listed above; this entry focuses on his specific contribution to adjunctions.)

Lawvere's 1969 paper "Adjointness in Foundations" argued that adjunctions are the fundamental concept of mathematics. He showed that:
- Quantifiers $\exists$ and $\forall$ are left and right adjoints to substitution (reindexing)
- Lawvere theories and their algebras are related by adjunctions
- The fundamental operations of mathematics (free constructions, limits, colimits) are all expressed by adjunctions

This was not just a slogan. Lawvere gave categorical proofs that adjunctions — not equations, not set-theoretic definitions — are the organizing principle behind mathematical definitions and theorems.

## Grothendieck's Contribution to Category Theory

Alexander Grothendieck (1928–2014) used category theory to revolutionize algebraic geometry. His main categorical contributions include:
- *Grothendieck toposes*: categories of sheaves on a site (a category with a Grothendieck topology), generalizing sheaves on topological spaces
- *Fibered categories* (Grothendieck fibrations): a framework for "varying" categories that subsumes all of type theory's variable types
- The *Yoneda perspective*: defining algebraic varieties by their functor of points, not their underlying set
- *Abelian categories* and *derived categories*: the categorical framework for homological algebra
- *Motives*: a hypothetical universal cohomology theory, still not fully constructed

Grothendieck's 1983 letter to Quillen, *Pursuing Stacks*, formulated the homotopy hypothesis and sketched the program of higher category theory that has occupied the field for forty years.

## Jacob Lurie (born 1977)

Lurie is the architect of modern higher category theory and ∞-topos theory. His two major works — *Higher Topos Theory* (2009) and *Higher Algebra* (2017) — established the foundations of ∞-category theory using the quasi-category model (Kan complexes as ∞-groupoids, simplicial sets satisfying the inner horn condition as ∞-categories). Lurie proved that ∞-toposes are the correct higher-categorical generalization of Grothendieck toposes, and that they satisfy descent conditions making them the right setting for homotopy theory. His work provides the mathematical framework for the slogan "HoTT is the internal language of ∞-toposes."

## Emily Riehl (born 1983)

Riehl has made major contributions to both the foundations and accessibility of higher category theory. Her book *Category Theory in Context* (2016) is the best modern introduction to category theory. Her research, with Dominic Verity, on *∞-cosmoi* provides a framework for doing higher category theory without choosing a specific model (quasi-categories, complete Segal spaces, etc.). Her work on algebraic model structures and enriched homotopy theory has connected classical homotopy theory to the ∞-categorical framework. Riehl's textbook and research have significantly broadened the accessibility of these ideas to working mathematicians.

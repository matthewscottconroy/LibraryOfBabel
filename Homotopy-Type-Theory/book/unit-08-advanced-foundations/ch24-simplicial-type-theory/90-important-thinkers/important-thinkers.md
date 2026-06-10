# Important Thinkers in Simplicial Type Theory

## Emily Riehl (Johns Hopkins University)

Riehl is one of the most important category theorists of her generation, with foundational work in enriched category theory, model categories, and ∞-category theory. Her book *Category Theory in Context* is widely regarded as one of the clearest introductions to category theory written in decades. Her research monograph *Categorical Homotopy Theory* develops the theory of algebraic model structures and Quillen functors with exceptional rigor.

The simplicial type theory program is Riehl's most foundational contribution. The 2017 Riehl-Shulman paper "A Type Theory for Synthetic ∞-Categories" represents a decade of thinking about what a native language for ∞-category theory would look like. Her perspective is that classical ∞-category theory — done in the model of quasi-categories — requires too much infrastructure (simplicial sets, the Joyal model structure, fibration theory) before you can say anything interesting. A synthetic theory should let you state and prove the Yoneda lemma without all that.

Riehl continues to develop the synthetic program, both in the Rzk proof assistant and in ongoing research on synthetic adjoint functor theory, limits, and colimits.

## Michael Shulman (University of San Diego)

Shulman is among the most technically versatile type theorists working today. He has made foundational contributions to homotopy type theory (the HoTT Book, where he is a key contributor), to the theory of modalities (Chapter 25 of this book), to 2-dimensional type theory, and to the foundations of ∞-categorical logic.

The Riehl-Shulman paper is, on the technical side, primarily Shulman's work. The two-level type theory framework — the separation between the outer (shape) level and the inner (homotopy) level — is his contribution. So is the precise formulation of extension types, the proof of the synthetic Yoneda lemma, and the connection to the Joyal model structure on simplicial sets.

Shulman is also the primary author of *real-cohesive HoTT* (Chapter 25). He has an unusual ability to work simultaneously at the level of concrete type theory (what the inference rules say) and at the level of categorical semantics (what the model is), and to translate between the two.

## Graeme Segal (University of Oxford)

Segal is the originator of the "Segal conditions" — though he formulated them in the language of simplicial spaces and topological categories, not type theory. His 1974 paper "Categories and Cohomology Theories" introduced the *Segal condition* for simplicial spaces: the condition that the restriction maps $X_n \to X_1 \times_{X_0} \cdots \times_{X_0} X_1$ are equivalences.

Segal's original motivation was understanding configuration spaces and operads in algebraic topology. The connection to category theory — that the Segal condition characterizes "up-to-homotopy categories" — came later. By now, his condition has been enormously generalized (complete Segal spaces, Segal ∞-categories, and the synthetic Segal types of Chapter 24) and has become one of the central organizing principles of higher category theory.

## Charles Rezk (University of Illinois)

Rezk introduced *complete Segal spaces* in a 2001 paper, adding the completeness condition (now called the Rezk condition) to Segal's original framework. His insight: the Segal condition alone does not capture the correct notion of equivalence between objects in an ∞-category. The completeness condition says that isomorphisms are the same as identities, making the notion of "same object" precise.

Rezk's model structure on simplicial spaces — the *Rezk model structure* or *complete Segal space model structure* — is one of several equivalent models for ∞-categories. His completeness condition is now understood, in the language of Chapter 24, as the analogue of univalence for ∞-categories.

## Jacob Lurie (Institute for Advanced Study)

Lurie's two monumental books — *Higher Topos Theory* (2009) and *Higher Algebra* (2017) — are the foundational references for classical (non-synthetic) ∞-category theory. Working in the model of quasi-categories, Lurie developed a complete foundational framework for ∞-category theory: ∞-toposes, ∞-operads, stable ∞-categories, and their applications to algebraic K-theory and derived algebraic geometry.

The Riehl-Shulman synthetic approach is, in part, a response to Lurie's classical approach: both programs aim to develop the same ∞-category theory, but one uses simplicial sets and the Joyal model structure (Lurie) and the other uses type theory (Riehl-Shulman). The two approaches should give the same mathematical content, but the type-theoretic approach aspires to be more *foundational* and more *computable*.

## Nikolaus Kuhn (Rzk contributor)

Among the developers of the Rzk proof assistant, Kuhn and collaborators (including contributors to the rzk-lang GitHub organization) have formalized significant portions of the Riehl-Shulman theory. The formalization effort is the practical test of the synthetic program: does the type theory actually support ∞-categorical reasoning without too much overhead?

The answer emerging from the Rzk formalization is: yes, with caveats. The basic theory (Segal types, Rezk types, Yoneda) works cleanly. The more advanced theory (adjunctions, limits, colimits) requires more work but is feasible. The formalization has also revealed places where the theory needs refinement — points where the synthetic approach hits technical difficulties that the classical approach handles differently.

## André Joyal (UQAM)

Joyal introduced the model of ∞-categories as *quasi-categories* (Kan complexes with an inner horn-filling condition), developing a model structure on simplicial sets that captures ∞-category theory. The "Joyal model structure" is the classical semantics for simplicial type theory.

Joyal's original proof of the Yoneda lemma for quasi-categories — the classical analogue of the synthetic Yoneda of Chapter 24 — is one of the technical foundations of the entire program. Understanding the classical proof makes the simplicity of the synthetic proof even more striking: what required pages of simplicial set theory becomes, in STT, a few lines using extension types and the Segal condition.

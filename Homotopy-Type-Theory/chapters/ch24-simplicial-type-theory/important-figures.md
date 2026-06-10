# Important Figures

The development of simplicial type theory draws on two generations of mathematical work: the classical foundations of ∞-category theory laid in the 2000s–2010s by Joyal and Lurie, and the type-theoretic synthesis accomplished by Riehl and Shulman in the 2010s–2020s. The figures below are those whose contributions are most directly at stake in Chapter 24.

---

## Emily Riehl (1983–present)
*Co-creator of simplicial type theory; homotopy theorist and category theorist at Johns Hopkins University*

Emily Riehl received her PhD from the University of Chicago in 2011 under the supervision of Peter May, specializing in homotopy theory and the formal theory of homotopy coherent structures. She is one of the founders of the ∞-cosmos framework (developed with Dominic Verity), an axiomatic approach to ∞-category theory that abstracts away from any specific model, proving theorems that hold simultaneously for quasi-categories, complete Segal spaces, and other models.

Riehl's contribution to the subject matter of Chapter 24 is the co-creation of simplicial type theory with Michael Shulman. Their 2017 paper "A Synthetic Theory of ∞-Categories in Homotopy Type Theory" is the primary reference for everything in this chapter: the two-interval framework, the Segal condition, the Rezk condition, covariant and contravariant fibrations, and the synthetic Yoneda lemma are all from this paper. The synthetic Yoneda lemma is particularly notable: in classical category theory, the Yoneda lemma requires checking naturality; in STT, Riehl and Shulman showed that naturality is automatic from the type structure, giving a proof that is both simpler and more conceptual than the classical proof.

Her textbook "Category Theory in Context" (2016) is freely available online and provides the classical category theory background that STT synthesizes. Riehl has also been an active contributor to the broader HoTT community through the HoTTEST seminar and summer school series, making STT accessible to students through recorded lectures and exercises. The open problem of directed univalence — whether there is a good type of ∞-categories satisfying an ∞-categorical analogue of the univalence axiom — is one Riehl has identified as the central open question in STT.

---

## Michael Shulman (1980–present)
*Co-creator of simplicial type theory; logician and type theorist at the University of San Diego*

Michael Shulman received his PhD from the University of Chicago in 2009 and works at the intersection of type theory, category theory, and homotopy theory. He is one of the most broadly productive contributors to the HoTT program, having made significant contributions to the metatheory of HoTT (including work on higher-dimensional models, the semantics of univalence, and the relationship between HoTT and ∞-toposes), to cohesive type theory (Chapter 25), and to the foundational question of directed type theory.

For Chapter 24, Shulman's key contribution is the co-creation of STT and the technical development of extension types. Extension types — types of the form $\{f : A^B \mid f|_S = g\}$ specifying that a function agrees with a given partial function on a subtype $S \subseteq B$ — are the technical engine behind the Segal and Rezk conditions in STT. The Segal condition says that the restriction map from 2-simplices to composable pairs is an equivalence; this is stated using extension types. The definition of the hom type itself uses extension types: $\mathsf{hom}_A(a, b) = (A^{\mathbf{2}} \text{ restricting to } a, b)$.

Shulman has also worked extensively on the semantics of STT: showing that STT is the internal language of certain ∞-toposes, understanding when the axioms are consistent, and connecting STT to the classical models (Joyal model structure, Reedy model structure on bisimplicial sets). His contributions to understanding the relationship between different kinds of type theories — cubical, simplicial, cohesive — make him the figure most responsible for the theoretical coherence of the overall HoTT program.

---

## André Joyal (1943–present)
*Inventor of quasi-categories; category theorist at UQAM (Université du Québec à Montréal)*

André Joyal received his PhD in 1971 and has been one of the central figures in categorical logic and homotopy theory for over fifty years. His contributions span a remarkable range: combinatorics (the Joyal bijection for parking functions), categorical logic (the Joyal-Tierney theorem characterizing Grothendieck toposes), homotopy theory (the Joyal model structure on simplicial sets), and ∞-category theory (quasi-categories).

The subject of Chapter 24 rests on Joyal's theory of quasi-categories, developed in a series of preprints and lectures in the 2000s. A quasi-category is a simplicial set satisfying the *inner horn filling condition* — the Segal condition that composable pairs of morphisms have a (non-unique) composite. This is precisely the condition that STT internalizes: the Segal condition in Chapter 24 is the type-theoretic version of Joyal's inner horn filling. The Joyal model structure on simplicial sets is the model that STT is designed to be the internal language of (in an appropriate ∞-categorical sense).

Joyal's "Notes on Quasi-Categories" (unpublished but widely circulated) remain an important reference for the classical theory that STT is modeling. The conceptual debt of STT to Joyal's work is substantial: without the quasi-category theory, there would have been no target for the synthetic development. Joyal has been an active presence in the HoTT community and has contributed to discussions of directed type theory. The directed interval $\mathbf{2}$ in STT is, in a precise sense, the type-theoretic internalization of the simplicial interval $[0, 1] \cap \mathbb{Z}_{\geq 0}$ (the "walking arrow") that generates the combinatorics of quasi-categories.

---

## Jacob Lurie (1977–present)
*Author of "Higher Topos Theory"; mathematician at the Institute for Advanced Study*

Jacob Lurie received his PhD from MIT in 2004 under the supervision of Michael Hopkins and is widely regarded as one of the most influential mathematicians of his generation. His two major works — "Higher Topos Theory" (2009) and "Higher Algebra" (2017) — developed ∞-category theory into a mature mathematical discipline capable of handling the most sophisticated constructions in algebraic topology, algebraic geometry, and mathematical physics.

For Chapter 24, Lurie's importance is semantic: "Higher Topos Theory" is the book that STT is attempting to internalize. When the chapter discusses ∞-toposes, ∞-categories of presheaves, the Yoneda embedding, and adjunctions, these concepts are fully developed (in quasi-categorical terms) in HTT. The synthetic proofs in STT correspond to the analytic proofs in Lurie's framework, and showing that they agree — that the synthetic and analytic approaches are equivalent — is one of the central technical tasks in the STT research program.

Lurie's "cobordism hypothesis" (from "On the Classification of Topological Field Theories," 2009) is a long-term target for formalization in STT: it states that fully extended topological field theories are classified by fully dualizable objects in a symmetric monoidal ∞-category. This statement is entirely ∞-categorical and would require the full machinery of STT (and more) to formalize. Lurie's work is responsible for establishing that ∞-category theory is not a curiosity but a necessary part of modern mathematics — without this, the motivation for STT would be less clear.

---

## Martín Hötzel Escardó (1967–present)
*Homotopy type theory in Agda; type theorist at the University of Birmingham*

Martín Escardó received his PhD from the University of Edinburgh and has worked on domain theory, computable analysis, and constructive mathematics throughout his career. He is best known within the HoTT community for his extensive development of homotopy type theory in Agda — the "TypeTopology" Agda library, which he maintains, contains many theorems about MLTT and HoTT formalized without the use of any axioms, or with explicit tracking of which axioms are used.

Escardó's direct contributions to the topics of Chapter 24 center on the metatheory: results about the behavior of the type theory, the interpretation of the universe, and the relationship between different formulations of the basic axioms. His work on "injective types" and "ainjective spaces" — types satisfying an algebraic injectivity condition — is related to the theory of covariant fibrations in STT, since both are about types that can be "extended" along maps in a functorial way.

More broadly, Escardó represents the tradition of doing HoTT in a proof assistant with minimal assumptions — forcing everything to be made explicit. This tradition is important for the STT program because STT involves assumptions (the directed interval $\mathbf{2}$, extension types, the Segal and Rezk conditions) that go beyond standard MLTT. Escardó's work on what can be proved without these axioms provides the baseline against which the power of the STT axioms can be measured. His online lecture notes on HoTT in Agda are among the best written in the field and complement the more research-oriented presentation of Chapter 24.

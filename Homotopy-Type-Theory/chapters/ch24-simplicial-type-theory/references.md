# References and Primary Sources

## Foundational Texts and Papers

The following works are the indispensable foundations for simplicial type theory and its connection to ∞-category theory. A serious reader of Chapter 24 should eventually work through all of these.

**Riehl and Shulman, "A Synthetic Theory of ∞-Categories in Homotopy Type Theory" (2017)**
The founding paper of simplicial type theory. Introduces the two-interval framework (the cubical interval $\mathbb{I}$ and the directed simplicial interval $\mathbf{2}$), the hom type, Segal types, Rezk types, covariant and contravariant fibrations, and the synthetic Yoneda lemma. Over 100 pages; the introduction and Sections 1–5 contain the essential ideas.

**André Joyal, "Notes on Quasi-Categories" (unpublished, 2008)**
The classical background that STT is modeling synthetically. Joyal introduced quasi-categories (Kan complexes without the full Kan condition — inner horn filling only) as a model for ∞-categories, and the Joyal model structure on simplicial sets. STT's Segal condition is the type-theoretic internalization of Joyal's quasi-category theory.

**Jacob Lurie, "Higher Topos Theory" (Princeton University Press, 2009)**
The comprehensive development of ∞-category theory using quasi-categories. A 900-page treatise covering limits and colimits, adjunctions, presentable ∞-categories, and ∞-toposes. STT is designed to be the internal language of ∞-toposes in the sense of Lurie, so Higher Topos Theory is the semantic background for everything in Chapter 24.

**The HoTT Book, Chapter 9: "The Principle of Equivalence" (Univalent Foundations Program, 2013)**
Chapter 9 of the HoTT Book introduces categories in HoTT (the "precategories" and "categories" of Chapter 9), which is a precursor to the full STT development. The discussion of why the Rezk condition matters — why isomorphism should equal identity — is here in embryo.

**Emily Riehl, "Category Theory in Context" (Dover, 2016; free online)**
A modern graduate-level introduction to classical category theory. The synthetic Yoneda lemma in STT corresponds to the classical Yoneda lemma proved here (Chapter 2). For anyone who needs to fill gaps in classical category theory before or alongside reading STT, this is the right text.

---

## Seminal Papers

**Graeme Segal, "Categories and Cohomology Theories" (Topology, 13(3):293–312, 1974)**
The paper that introduced the Segal condition: a simplicial space satisfying the Segal condition models a "coherent" multiplication. Segal used this to define $\Gamma$-spaces and prove that the classifying space $B\Gamma$ models stable homotopy theory. The condition that composition in an ∞-category is unique (inner horn filling) generalizes Segal's original insight.

**Charles Rezk, "A Model for the Homotopy Theory of Homotopy Theory" (Transactions of the AMS, 353(3):973–1007, 2001)**
Introduced complete Segal spaces as a model for ∞-categories. The "completeness" condition (Rezk condition) — that isomorphisms are the same as homotopy equivalences — is the classical precursor to the Rezk condition in Chapter 24. The model structure on simplicial spaces constructed here is the primary semantic model for STT.

**André Joyal and Miles Tierney, "Quasi-Categories vs Segal Spaces" (Contemporary Mathematics, 431:277–326, 2007)**
Proves the equivalence of quasi-categories and complete Segal spaces as models for ∞-categories. The two perspectives are unified, which is philosophically important for STT: the Segal condition and the Rezk condition together (Rezk types) capture both quasi-categorical and complete Segal space approaches.

**Emily Riehl and Dominic Verity, "Fibrations and Yoneda's Lemma in an ∞-Cosmos" (Journal of Pure and Applied Algebra, 221(3):499–564, 2017)**
Introduces the ∞-cosmos framework as an axiomatic setting for ∞-category theory. The ∞-cosmos approach (Riehl-Verity) and the STT approach (Riehl-Shulman) are closely related: both axiomatize ∞-category theory without choosing a specific model. Understanding ∞-cosmoi clarifies what STT is capturing synthetically.

**Jonathan Weinberger, "Strict Stability of Extension Types" (2022, arXiv:2203.07194)**
A technical advance in STT: proves that extension types in simplicial type theory satisfy a strict stability condition, resolving a coherence issue in the original Riehl-Shulman formulation. Required reading for anyone working with the formal metatheory of STT.

**Nikolai Kudasov, Emily Riehl, and Jonathan Weinberger, "Formalizing the ∞-Categorical Yoneda Lemma" (Proceedings of CPP 2024)**
The formalization of the synthetic Yoneda lemma in the Rzk proof assistant, giving the first machine-verified proof of this central theorem of STT. Describes the Rzk architecture and the formalization methodology.

**Thorsten Altenkirch and Ambrus Kaposi, "Type Theory in Type Theory Using Quotient Inductive Types" (POPL 2016)**
Although not directly about STT, this paper on quotient inductive-inductive types (QIITs) is relevant to the question of how to give a formal syntax for simplicial type theory. The metatheory of STT (defining STT as a type theory with a model in itself) requires QIIT techniques.

---

## Related Work and Extensions

**Ulrik Buchholtz and Jonathan Weinberger, "Synthetic Fibered (∞,1)-Category Theory" (2021, arXiv:2105.01724)**
Extends the Riehl-Shulman framework to a synthetic treatment of fibrations and their classification. Introduces "two-sided fibrations" and proves classification results that make the ∞-categorical machinery of Lurie accessible synthetically.

**César Bardomiano Martínez, "Limits and Colimits of Synthetic ∞-Categories" (2022, arXiv:2202.12225)**
The systematic development of limits and colimits in simplicial type theory. While the definitions of limits and colimits are clear in STT (as representations of presheaves), making the theory work coherently requires careful argument. This paper develops the basic theory.

**Jonathan Weinberger, "A Synthetic Perspective on (∞,1)-Category Theory: Fibrational and Semantic Aspects" (PhD thesis, TU Darmstadt, 2022)**
A comprehensive PhD thesis covering the foundations of STT, extension types, fibrational structures, and semantics. Excellent as a secondary reference to Riehl-Shulman for anyone who finds the original paper dense.

**David Jaz Myers, "Homotopy Type Theory for Directed Mathematics" (2023, lecture notes)**
Expository notes on directed type theory more broadly, covering both STT and other approaches. Good for understanding the landscape of directed type theories and how STT relates to alternatives.

**Emily Riehl and Michael Shulman, "The Synthetic Theory of ∞-Categories vs the Analytic Theory of ∞-Categories" (Algebraic & Geometric Topology, 2023)**
A comparison paper clarifying the relationship between the synthetic STT approach and the classical quasi-category approach of Joyal-Lurie. Addresses common questions about what STT can and cannot do compared to classical theory.

---

## Online Resources and Formalization

**Rzk Proof Assistant: rzk-lang.github.io**
The proof assistant implementing simplicial type theory. Rzk is designed specifically for STT: its type theory includes the two-interval setup, extension types, and the tope layer. The documentation at rzk-lang.github.io includes tutorials, a language reference, and worked examples. The type-checker is available online (no installation required) and is actively developed.

**sHoTT Library: github.com/rzk-lang/sHoTT**
The "simplicial Homotopy Type Theory" library for Rzk. Contains formalizations of the basic theory: Segal types, Rezk types, the Yoneda lemma, fibrations, and ongoing work on limits and colimits. The library files are the best place to see how STT theorems are actually stated and proved. Browse the source alongside reading the Riehl-Shulman paper.

**HoTTEST Summer School 2023 — STT Lectures (YouTube)**
Emily Riehl gave a series of lectures on simplicial type theory at the HoTTEST Summer School in 2023. These lectures are the most accessible introduction to STT available, pitched at students who know HoTT but not ∞-category theory. Free on YouTube; the associated exercises are excellent.

**Rzk Playground: rzk-lang.github.io/rzk/v1/playground**
An interactive browser-based environment for running Rzk code. Use this to experiment with the definitions from Chapter 24 — define the Segal condition, check horn fillings, verify the Yoneda maps — without installing anything locally.

**nLab: "Simplicial Type Theory" (ncatlab.org/nlab/show/simplicial+type+theory)**
The nLab article on STT provides concise definitions, references to the primary literature, and connections to related mathematical structures. As with all nLab articles, it assumes mathematical maturity; treat it as a reference, not an introduction.

---

## Historical Context

The development of simplicial type theory in 2017 was the convergence of two research programs that had been running in parallel for a decade. On the type theory side, the HoTT Book had established that Martin-Löf type theory with univalence is the internal language of ∞-groupoids. But ∞-groupoids are all morphisms invertible — they model homotopy theory, not category theory. The question of a "directed" type theory, one that could reason about ∞-categories with non-invertible morphisms, was raised in the HoTT Book itself (Chapter 9 defined categories in HoTT and noted the limitations). On the category theory side, the ∞-cosmos framework of Riehl and Verity (2012–2017) showed that many theorems of ∞-category theory could be proved from purely formal axioms, without committing to a specific model such as quasi-categories. The insight that these two programs could be unified — that the ∞-cosmos axioms could be internalized in a type theory — led to the 2017 Riehl-Shulman paper.

The simplicial interval $\mathbf{2}$ is the key novelty. In cubical type theory, the interval $\mathbb{I}$ has a complement operation (symmetry of paths), which means all paths are reversible — morphisms are invertible. By removing the complement (making the interval directed), Riehl and Shulman obtained a type theory where some paths are genuinely non-invertible. The technical tool that makes this work — extension types, which let you specify partial terms and then fill them — was present in earlier work on dependent type theory but had not been combined with the directed interval before.

Since 2017, the theory has been developed actively. The Rzk proof assistant (Kudasov et al.) was developed specifically to check STT proofs, producing the first machine-verified proofs of STT theorems including the Yoneda lemma (2024). The connections to ∞-topos theory (Lurie's framework) are being developed, and the open problem of directed univalence — the analogue of the univalence axiom for the universe of ∞-categories — remains the central open question in the field.

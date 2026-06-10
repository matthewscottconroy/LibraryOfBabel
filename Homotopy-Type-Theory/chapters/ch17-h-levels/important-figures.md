# Important Figures

## Vladimir Voevodsky (1966–2017)
*Fields Medal-winning algebraic geometer; creator of the univalent foundations program, inventor of the h-level hierarchy, and the central figure in the development of HoTT as a foundational system.*

Vladimir Voevodsky was born in Moscow and showed extraordinary mathematical talent from an early age. He received his doctorate from Harvard in 1992 under David Kazhdan and spent most of his career at the Institute for Advanced Study in Princeton. He was awarded the Fields Medal in 2002 for his work on the Milnor conjecture and motivic cohomology — he built a new cohomology theory for algebraic varieties (motivic cohomology) and used it to prove deep results about quadratic forms and algebraic K-theory. This work required the development of an entirely new framework: the motivic homotopy theory of algebraic varieties, built using techniques directly analogous to those of simplicial homotopy theory.

Voevodsky's transition to foundations came partly from his experience with computer verification. After spending years checking an important paper, he became convinced that human mathematicians make errors that only formal verification could catch. Around 2009–2010, he began developing univalent foundations: a new foundation for mathematics based on Martin-Löf type theory with the Univalence axiom added, and designed from the ground up for computer formalization. The h-level hierarchy is one of his key contributions to this program. Voevodsky defined `iscontr` (contractibility), `isaprop` (being a mere proposition), and `isaset` (being a set) in his Coq formalization, along with the general `isofhlevel n` predicate. He introduced the notation $\|A\|_n$ for $n$-truncation and proved the basic closure properties of each level. His insight that the h-level hierarchy mirrors the Postnikov tower in classical homotopy theory — and that this mirror relationship is not an analogy but an identity, via the simplicial set model — is the conceptual foundation of the entire chapter.

Voevodsky also proved the fundamental "h-level arithmetic" lemmas: that function types preserve h-levels (if $B$ is an $n$-type then $A \to B$ is an $n$-type), that $\Sigma$-types of $n$-types are $n$-types (when the fiber is uniformly an $n$-type), and that the universe of $n$-types is an $(n+1)$-type. These are not difficult theorems, but stating and proving them correctly — in a way that plays well with the J rule and transport — requires care. Voevodsky's Coq formalization made all of this precise and machine-checkable. His death in 2017, at 51, was a significant loss to the field.

---

## Steve Awodey (1959–present)
*Category theorist and logician; co-developer of the homotopy-theoretic interpretation of identity types; influential in shaping the mathematical and philosophical foundations of HoTT.*

Steve Awodey is a professor at Carnegie Mellon University (full profile in Ch16 important figures). His contributions to the h-level hierarchy come primarily through the homotopy-theoretic semantics of type theory that he developed with Michael Warren (2009), which made clear why the h-level hierarchy has the shape it does: in the Awodey-Warren semantics, an $n$-type corresponds to a "(n-1)-truncated" fibrant object in a model category, and the h-level arithmetic follows from general facts about Postnikov sections and n-connected maps.

Awodey has also contributed to the presentation and pedagogy of h-levels. His lectures at the 2012–2013 IAS special year on Univalent Foundations helped establish the conventions and the "right" way to state results about h-levels that now appear in the HoTT Book. His philosophical perspective — that the h-level hierarchy reflects a fundamental stratification of mathematical structure that was always present but only made visible by homotopy type theory — has influenced how the hierarchy is taught and understood.

---

## Michael Shulman (1980–present)
*Category theorist and type theorist; co-developer of the HoTT Library in Coq; contributor to the formalization and metatheory of h-levels and truncation.*

Michael Shulman received his doctorate from the University of Chicago in 2009 (under Peter May) and is currently a professor at the University of San Diego. His work spans higher category theory, topos theory, and homotopy type theory, with a particular focus on the interfaces between these subjects. He is one of the most productive contributors to the HoTT project, having co-authored significant portions of the HoTT Book and many research papers on the foundations and applications of HoTT.

Shulman's contributions to the theory of h-levels are distributed across several papers and the HoTT Book. He played a major role in the formalization of the h-level hierarchy in Coq (as part of the HoTT Library), verifying the closure properties and providing machine-checked proofs of key theorems. His paper "Idempotents in Intensional Type Theory" (*Logical Methods in Computer Science*, 2015) develops the theory of idempotent equivalences and their role in truncation, connecting the HoTT notion of truncation to the theory of idempotent monads in category theory. His work on "Mirroring Without Error" and related topics explores how the h-level stratification of HoTT corresponds to stratified structures in higher topos theory.

Shulman is also known for blog posts on The n-Category Café that explain HoTT concepts — including h-levels — to a broad mathematical audience. These posts, while informal, have been influential in communicating why the h-level hierarchy matters and how it relates to classical mathematics. His 2013 post "In Defense of Propositional Truncation" explains the subtleties of the distinction between mere existence ($\|A\|_{-1}$) and computational existence ($\Sigma$) in a way that is directly relevant to the applied exercises in this chapter.

---

## Nicolai Kraus (present)
*Type theorist; primary contributor to the metatheory of truncation in MLTT; developer of the "constant endofunction" technique and the hub-and-spoke construction for propositional truncation.*

Nicolai Kraus received his doctorate from the University of Nottingham under Thorsten Altenkirch and has since held positions in Nottingham and Birmingham. He works on the foundations of HoTT and type theory, with particular attention to the metatheory of truncation, the relationship between different notions of equality, and the semantics of higher inductive types.

Kraus's most direct contribution to this chapter is the series of papers (with Escardó, Coquand, and Altenkirch) on "Notions of Anonymous Existence in Martin-Löf Type Theory" (LMCS 2017). This paper gives a comprehensive treatment of when a map factors through a propositional truncation: the key result is that a function $f : A \to P$ (where $P$ is a proposition) factors through $\|A\|$ if and only if $f$ is "constant" in a suitable sense (its fibers are all equal). This is a significant generalization of the universal property of propositional truncation, and it clarifies exactly what information is lost when you truncate. Kraus also showed that the "hub and spoke" construction provides a HIT definition of propositional truncation that works in any ∞-topos, not just Voevodsky's simplicial model.

Kraus's work on "the general universal property of propositional truncation" (2014) precisely identifies the universal property: $\|A\| \to P$ (for $P$ a proposition) is equivalent to $A \to P$ (if $P$ is a proposition). This seems tautological but requires careful proof in intensional MLTT, because the universal property must be stated using identity types and function extensionality, and its proof uses a non-trivial coherence argument. The constant-endofunction technique introduced by Hedberg and generalized by Kraus is now a standard tool in HoTT, used whenever one wants to show that a type is a proposition without access to a complete definition.

---

## Egbert Rijke (present)
*Type theorist; author of the most accessible modern HoTT textbook; contributor to the systematic development of h-level theory and synthetic homotopy theory in HoTT.*

Egbert Rijke received his doctorate from Utrecht University under Ieke Moerdijk and has held positions in Pittsburgh (Carnegie Mellon) and Ljubljana. He is currently a researcher in Zagreb. His work focuses on synthetic homotopy theory, the foundations of HoTT, and formalization in proof assistants.

Rijke is the author of *Introduction to Homotopy Type Theory* (Cambridge University Press, 2022; also freely available as arXiv:2212.11082), which is the most thorough and accessible modern textbook on HoTT. The treatment of h-levels in this book is exceptional: Rijke covers contractible types (Chapter 12), propositions (Chapter 13), sets (Chapter 14), and truncations (Chapter 15) in sequence, with extensive worked examples, exercises, and careful proofs of all the standard results. His presentation of the "fundamental theorem of identity types" — which characterizes equivalences in terms of identity types — and its application to computing path spaces is a highlight of the book.

Rijke's research contributions to h-level theory include work on the "join construction" for propositional truncation (the propositional truncation of $A$ is the colimit of the sequence $A \to A * A \to A * A * A \to \cdots$ where $*$ is the join), which provides a more explicit construction than the hub-and-spoke. He has also contributed to the systematic development of synthetic homotopy theory: computing homotopy groups of spheres and other spaces using h-level tools (truncation, connectedness) in ways that closely mirror classical arguments. His formalization in Agda is one of the most complete available implementations of HoTT.

---

## Thierry Coquand (1961–present)
*Logician and computer scientist; co-creator of the Calculus of Constructions (the basis of Coq); contributor to the foundations of HoTT through cubical type theory.*

Thierry Coquand is a professor at the University of Gothenburg and one of the central figures in the theory of dependent types. He co-developed the Calculus of Constructions with Gérard Huet in the mid-1980s, which became the foundation of the Coq proof assistant. His subsequent work has explored various extensions and alternatives to the Calculus of Constructions, always with an eye toward computational content and constructive validity.

Coquand's contributions to h-levels are primarily through two directions. First, as a co-author of the Kraus-Escardó-Coquand-Altenkirch paper on anonymous existence, he contributed to the metatheory of propositional truncation in intensional type theory. Second, and more recently, he has been the primary developer of cubical type theory (with Mörtberg, Cohen, Bezem, Huber, and others), which provides a *computational* model of HoTT in which the h-level hierarchy has direct computational content. In cubical type theory, propositional truncation and set-truncation can be defined as higher inductive types with explicit reduction rules, making them computable rather than axiomatically postulated. This makes the entire h-level machinery available in a type theory with decidable type-checking and normalization — resolving a longstanding tension between the expressiveness of HoTT and its computational properties.

Coquand's broader influence on the formalization of h-levels is through the design of Agda and Coq as systems in which the h-level hierarchy can be expressed and reasoned about. His philosophical commitment to constructive mathematics informs the treatment of h-levels throughout: the principle of unique choice (if $\|A\|_{-1}$ and $A$ is a set, then choice is validated) is a theorem rather than an axiom in cubical type theory, following from the computational interpretation of truncation.

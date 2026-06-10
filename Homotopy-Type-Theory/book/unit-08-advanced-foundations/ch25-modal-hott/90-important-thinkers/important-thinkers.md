# Important Thinkers in Modal HoTT and Cohesive Geometry

## Urs Schreiber (NYUAD / Czech Academy of Sciences)

Schreiber is the originator and primary developer of the cohesive HoTT program for mathematical physics. His work spans a remarkable range: from the abstract foundations of higher topos theory, through the theory of ∞-Lie algebroids, to the formalization of string theory.

His research program, documented in the monograph *Differential Cohomology in a Cohesive Infinity-Topos* (nLab/arxiv), is the most comprehensive attempt to formalize the foundations of modern theoretical physics in a type-theoretic setting. The key insight — that the adjoint triple $\int \dashv \flat \dashv \sharp$ captures the essential structure of differential geometry — is his.

Schreiber's work connects to several different communities: he is in regular conversation with type theorists (for the foundations), with differential geometers (for the classical content), and with mathematical physicists (for the applications to string theory and gauge theory). His nLab contributions (he is one of the primary contributors to nLab, the online reference for higher category theory and related mathematics) are an invaluable resource.

The scale of ambition in Schreiber's program is itself remarkable. To formalize the Green-Schwarz mechanism for anomaly cancellation in string theory — to give a type-theoretic account of why the quantum theory of strings is consistent — requires building up an enormous amount of mathematics: higher Lie theory, differential cohomology, twisted K-theory, and more. Schreiber's program aims to do all of this within a single cohesive type-theoretic framework.

## Michael Shulman (University of San Diego)

Shulman is the type-theorist who gave cohesive HoTT its precise form. His paper *Brouwer's Fixed-Point Theorem in Real-Cohesive Homotopy Type Theory* (2018) introduced the real cohesion axiom and showed how to use it to prove classical results in topology synthetically.

Shulman's contribution is foundational in the technical sense: he translated Schreiber's intuitions about cohesive toposes into precise type-theoretic axioms, proved their consistency (relative to the existence of certain ∞-toposes), and demonstrated their power through synthetic proofs of classical theorems.

He is also, crucially, a type theorist who understands model theory. His proofs of the consistency of the cohesion axioms — that they have models in suitable ∞-toposes — are what give the axiomatic program its credibility. Without these semantic results, the cohesion axioms would be formal assertions with unknown status.

Beyond cohesive HoTT, Shulman has worked on modalities in HoTT generally (*Modalities in Homotopy Type Theory*, 2023), giving the most systematic treatment of modal operators in the type-theoretic setting. This general theory underlies all the specific modalities studied in Chapter 25.

## William Lawvere (1937–2023, SUNY Buffalo)

Lawvere is the originator of the ideas that eventually led to cohesive HoTT, though the type-theoretic form would have surprised him. His concept of *cohesion* — a topos satisfying a specific set of axioms relating to the adjoint triple $\int \dashv \flat \dashv \sharp$ — was developed in the 1980s and 1990s as a categorical foundation for *synthetic differential geometry*.

Lawvere's broader program was nothing less than to reformulate the foundations of mathematics using category theory. His earlier work on *functorial semantics* (using categories to model algebraic theories), *adjoint functors as conceptual tools* (the 1969 paper arguing that adjointness is the fundamental concept of mathematics), and *elementary toposes* (the categorical foundation for set theory) set the stage for the cohesive program.

The transition from Lawvere's topos-theoretic cohesion to Shulman's type-theoretic cohesion required a translation between two mathematical languages. That translation was possible because of the deep connection between elementary toposes and dependent type theories — a connection Lawvere himself understood, but which later developed into the precise correspondence of the Seely-Clairambault-Dybjer work.

## Kock and Dubuc (Synthetic Differential Geometry)

Anders Kock and Eduardo Dubuc developed *synthetic differential geometry* (SDG) in the 1970s and 1980s — the first attempt to do differential geometry without coordinates, using a non-classical foundation that allowed nilsquare infinitesimals.

SDG is based on the *Kock-Lawvere axiom*: every map $D \to \mathbb{R}$ (where $D = \{x : \mathbb{R} \mid x^2 = 0\}$) is of the form $d \mapsto a + bd$ for unique $a, b$. This axiom is inconsistent with classical logic (it implies that $D$ is non-trivial, but in classical set theory, $D = \{0\}$ is a singleton). It requires *intuitionistic logic*.

SDG is the direct predecessor of synthetic differential geometry in cohesive HoTT. The Kock-Lawvere axiom holds in the smooth sets model of cohesive HoTT. The cohesive HoTT program can be seen as the ∞-categorical generalization of SDG, replacing the toposes of SDG with ∞-toposes and adding the homotopy-theoretic content of HoTT.

## Graeme Segal and Atiyah-Singer

The *Atiyah-Singer index theorem* and Segal's *K-theory* and *elliptic cohomology* are the classical results that the Schreiber program aims to formalize and generalize.

The Atiyah-Singer theorem says: the analytical index of an elliptic differential operator on a compact manifold equals its topological index (computed from characteristic classes). This theorem connects differential analysis (eigenvalues of operators) to topology (characteristic classes). It is one of the deepest theorems in 20th century mathematics.

In cohesive HoTT, the analytical index should be a type-theoretic invariant of the elliptic operator (a specific map between function spaces). The topological index should be computed from the characteristic classes of the principal bundle associated to the operator. The Atiyah-Singer theorem would then be an equivalence between two type-theoretic constructions.

Formalizing the Atiyah-Singer theorem in cohesive HoTT remains an open problem, but it is one of the explicit goals of the Schreiber program.

## Dominic Verity and Ross Street

Verity and Street, two Australian category theorists, have developed the theory of *Street's orientals* and *complicial sets* — an approach to ∞-categories via globular or simplicial sets with orientation data. Their work connects to the direction, modality, and shape ideas of Chapters 24 and 25.

Verity's *weak complicial sets* are a model for ∞-categories that is in some ways more natural than quasi-categories — particularly for the "directed" aspects that Chapter 24 addresses. The connection between Verity's model and the Riehl-Shulman simplicial type theory is an active area of research.

Street's influence on the nLab and on Australian category theory has been foundational. His formulation of *orientals* (the ∞-categorical analogues of simplices with orientation data) predates the Lurie-style ∞-category theory by decades and provides a complementary perspective.

## The nLab Community

The *nLab* (nLab.org) is a collaborative mathematical reference for category theory, higher category theory, topos theory, and their connections to type theory and physics. It is maintained by a community including Schreiber, Shulman, and many others.

The nLab is not a traditional textbook — it is a living document reflecting ongoing research. Many of the ideas in Chapter 25 are first articulated or most clearly explained in nLab entries. The *nLab entry on cohesive homotopy type theory* is the best online reference for the material of this chapter.

The nLab community represents a particular intellectual tradition: the conviction that higher category theory, topos theory, type theory, and mathematical physics are not separate fields but different aspects of a single conceptual framework. This tradition — sometimes called the *nPOV* (n-categorical point of view) — is the intellectual context in which cohesive HoTT lives.

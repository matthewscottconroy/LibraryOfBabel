# References and Primary Sources

## Foundational Texts and Papers

**Mike Shulman, "Brouwer's Fixed-Point Theorem in Real-Cohesive Homotopy Type Theory" (Mathematical Structures in Computer Science, 28(6):856–941, 2018)**
The primary application paper for cohesive HoTT and the flagship reference for Chapter 25. Shulman proves the Brouwer fixed-point theorem synthetically — without coordinates, analysis, or any model-theoretic machinery — using the shape modality and cohesion axioms. The paper also develops the cohesion axioms (the adjoint triple $\int \dashv \flat \dashv \sharp$) in a self-contained way, making it the best single-paper introduction to the subject.

**Urs Schreiber, "Differential Cohomology in a Cohesive ∞-Topos" (arXiv:1310.7930, 2013)**
The comprehensive development of differential cohomology, gauge theory, and topological field theory in a cohesive ∞-topos. At over 600 pages, this is the most ambitious application of cohesive methods to mathematical physics. Chapter 25's discussion of principal bundles, connections, and higher gauge theory draws on this work. Reading the introduction and the first two chapters gives the physical motivation; the rest is a research treatise.

**F. William Lawvere, "Axiomatic Cohesion" (Theory and Applications of Categories, 19(3):41–49, 2007)**
The categorical origin of the cohesion concept. Lawvere defined a "cohesive topos" as one with an adjoint quadruple $\Pi_0 \dashv \Delta \dashv \Gamma \dashv \text{Codisc}$, capturing the idea that in a cohesive topos, sets are "discrete" (all points separated) while the ambient topos has continuous structure. The cohesion axioms in Chapter 25 are the type-theoretic internalization of Lawvere's concept. Short (9 pages) and essential.

**Anders Kock, "Synthetic Differential Geometry" (Cambridge University Press, 1981; 2nd edition 2006)**
The foundational text of synthetic differential geometry (SDG), which provides the geometric background for Section 3 of Chapter 25. SDG develops differential geometry from axioms about a line object $R$ and an infinitesimal interval $D = \{x : R \mid x^2 = 0\}$, avoiding the need for epsilon-delta analysis. Cohesive HoTT extends SDG by adding homotopy-theoretic structure: the cohesive ∞-topos provides both the smooth structure (SDG) and the homotopy structure (HoTT) simultaneously.

**The HoTT Book, "Appendix: Modalities and Cohesive HoTT" (Univalent Foundations Program, 2013)**
The appendix to the HoTT Book contains a treatment of modalities in HoTT, including the definition of a modality (unit, universal property), the relationship between modalities and reflective subcategories, and a brief introduction to the cohesion framework. This is the most accessible entry point to the abstract theory of modalities.

---

## Seminal Papers

**Peter Johnstone, "Sketches of an Elephant: A Topos Theory Compendium, Volume 1" (Oxford, 2002) — Part C: Toposes as Spaces**
The comprehensive reference on cohesive toposes from the classical viewpoint. Part C develops the geometric morphisms, local operators, and sheaf conditions that underlie the categorical semantics of modal HoTT. The "local and global sections" adjunctions that generate the cohesion modalities are developed here in detail.

**David Jaz Myers, "Simplicial, Divisorial, and Orientable Cohesion" (2022, arXiv:2204.00938)**
Develops an axiomatic framework for cohesion in which different geometric structures (smooth, algebraic, combinatorial) are distinguished by their cohesion modalities. Myers shows that the Brouwer fixed-point theorem, the de Rham theorem, and other synthetic geometry results follow from abstract cohesion axioms that work uniformly across these different geometric settings.

**Mike Shulman, "Cohesive HoTT in Homotopy Type Theory" (lecture notes, 2015)**
A shorter and more pedagogical development of cohesive HoTT than the full Brouwer paper. Introduces the cohesion axioms, the three modalities, and their basic properties, with worked examples including the computation $\int(\mathbb{R}/\mathbb{Z}) \simeq S^1$. This is the best second document to read after the Brouwer paper introduction.

**Urs Schreiber and Mike Shulman, "Quantum Gauge Field Theory in Cohesive HoTT" (2012, article)**
An early paper applying cohesive HoTT to quantum field theory, showing that the cohesive ∞-topos setting supports a synthetic treatment of gauge fields, their quantization, and the resulting quantum observables. This paper established that cohesive HoTT is not merely of foundational interest but has concrete applications in theoretical physics.

**Felix Wellen, "Cartan Geometry in Modal Homotopy Type Theory" (PhD thesis, Karlsruhe Institute of Technology, 2017)**
The formalization of differential geometry (Cartan geometry, connections, curvature) in cohesive HoTT. Wellen shows that the concepts of classical differential geometry — tangent bundles, covariant derivatives, geodesics — can be defined synthetically using the cohesion modalities and have the expected properties. A key reference for Section 3 of Chapter 25.

**David Jaz Myers, "String Diagrams for Double Categories and Equipments" (2016)**
Although the title suggests category theory, this paper develops the "string diagram" calculus for working with adjunctions and modalities, which is the most efficient way to perform calculations in cohesive HoTT. The diagrammatic calculus is especially helpful for keeping track of which modality applies where in cohesive proofs.

---

## Related Work and Extensions

**Egbert Rijke and Bas Spitters, "Sets in Homotopy Type Theory" (Mathematical Structures in Computer Science, 25(5):1172–1202, 2015)**
Develops the theory of sets and decidable properties in HoTT, including the relationship between truncation modalities and the set-level structure. This provides the background for understanding how the discrete modality $\flat$ picks out the "set-theoretic" part of a cohesive type.

**Ulrik Buchholtz and Egbert Rijke, "The Real Projective Spaces in Homotopy Type Theory" (LICS 2017)**
Constructs real projective spaces as HITs in HoTT and computes their homotopy groups, using methods that relate to the cohesive structure of real spaces. The techniques overlap with cohesive HoTT's treatment of real-valued types.

**Mike Shulman, "Linear Logic for Constructive Mathematics" (2018, arXiv)**
Shulman's work connecting modal type theory to linear logic, showing that the $\flat$ modality corresponds to the "of course" modality $!$ of linear logic. This gives a programming languages perspective on the flat modality: $\flat A$ is the "reusable" or "duplicable" version of $A$.

**Urs Schreiber, "Higher Prequantum Geometry" (2016, New Spaces for Mathematics and Physics, Cambridge)**
An expository account of how cohesive HoTT applies to the geometric quantization program, giving an accessible overview of the physics applications before the full technical development of the Differential Cohomology book.

**David Corfield, "Modal HoTT and Philosophy" (2020)**
Explores the philosophical implications of modal HoTT for the philosophy of mathematics and science, arguing that the cohesion modalities give a type-theoretic account of the distinction between mathematical and physical objects.

---

## Online Resources and Formalization

**Cohesion in Agda: github.com/agda/cubical (Modalities subfolder)**
The Cubical Agda library contains formalizations of several modalities and their properties, including propositional and $n$-truncation modalities, localization, and basic aspects of cohesion. The modality files provide executable code for the definitions in Chapter 25 and are a good starting point for formalization work.

**Felix Wellen's Agda Formalization: github.com/felixwellen/synthetic-geometry**
Wellen's formalization of synthetic algebraic geometry in cohesive HoTT, extending his PhD thesis work. The repository contains Agda code for affine schemes, bundles, and connections in the cohesive setting. Examining this code alongside Section 3 of Chapter 25 gives a concrete implementation of the abstract definitions.

**Schreiber's nLab Pages: ncatlab.org/nlab/show/cohesive+homotopy+type+theory**
The nLab's extensive treatment of cohesive HoTT, including a comprehensive overview of the modalities, their properties, worked examples (including the computation of $\int(\mathbb{R}/\mathbb{Z}) \simeq S^1$), and connections to the physics applications. The nLab is particularly strong on the semantics side, connecting the synthetic axioms to specific models.

**HoTTEST Lectures on Modal HoTT (YouTube, 2021)**
A recorded lecture series by Mike Shulman on cohesive HoTT, given at the HoTTEST electronic seminar. Covers the cohesion axioms, the Brouwer fixed-point theorem, and the de Rham theorem. Accessible to anyone who has completed Chapters 16–19 of this curriculum.

**nLab: "Synthetic Differential Geometry" (ncatlab.org/nlab/show/synthetic+differential+geometry)**
The nLab's overview of SDG with connections to cohesive HoTT. Includes the axiomatics of the infinitesimal interval $D$, the Kock-Lawvere axiom, and the relationship between SDG and microlocal analysis. Essential reading alongside Section 3 of Chapter 25.

---

## Historical Context

The cohesion concept has a long history in categorical logic. Lawvere's idea, developed in a series of papers from the late 1980s to the 2000s, was that a "cohesive topos" is one where sets can be "glued together" into continuous spaces in a way that is captured by an adjoint quadruple of functors between the topos and the category of sets. The intuition: sets have a "discrete" structure (no cohesion), and every cohesive space can be "discretized" (applying $\Gamma$) or "connected up" (applying $\Pi_0$). Lawvere's 2007 paper "Axiomatic Cohesion" gave the definitive abstract formulation.

Schreiber recognized in the early 2010s that the cohesive topos framework provides exactly the right setting for the "higher" gauge theory and string theory constructions he had been developing with Lurie's ∞-topos theory. His 2013 preprint "Differential Cohomology in a Cohesive ∞-Topos" brought together these strands. The key step from Schreiber to Shulman was type-theoretic: Shulman saw that the cohesive ∞-topos structure could be internalized as axioms in HoTT, giving a type theory where the geometry is built in. The resulting "cohesive HoTT" or "real-cohesive HoTT" retains the synthetic character of ordinary HoTT (no models, just axioms) while gaining geometric content that ordinary HoTT lacks.

The synthetic differential geometry tradition (Kock, Lawvere, Dubuc, Reyes, and others, from the 1970s onward) provides the geometric backbone. SDG showed that the differential calculus — tangent vectors, differential forms, de Rham cohomology — can be developed from the axiom that the line object $R$ contains an infinitesimal interval $D = \{x : R \mid x^2 = 0\}$ satisfying a certain "microlinearity" condition (the Kock-Lawvere axiom). Cohesive HoTT extends SDG by adding the homotopy-theoretic structure (the shape modality) needed to handle topological invariants. The result is a type theory capable of handling both the smooth structure (differential forms, connections) and the topological structure (homotopy groups, characteristic classes) of modern differential geometry.

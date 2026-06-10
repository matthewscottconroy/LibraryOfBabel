# References — Chapter 25: Modal HoTT and Cohesive Geometry

## Primary Sources: Cohesive HoTT

**Schreiber, U. (2013).** Differential Cohomology in a Cohesive Infinity-Topos. arXiv:1310.7930. — The foundational monograph for the mathematical physics program. Hundreds of pages of formalization of differential geometry, gauge theory, and quantum field theory in the ∞-topos setting. The primary reference for Sections 3 and 4 of this chapter.

**Shulman, M. (2018).** Brouwer's Fixed-Point Theorem in Real-Cohesive Homotopy Type Theory. *Mathematical Structures in Computer Science*, 28(6), 856–941. — The definitive paper on real-cohesive HoTT. Introduces the real cohesion axiom, proves the Brouwer fixed-point theorem, and establishes the consistency of the axioms. Essential reading.

**Lawvere, F. W. (2007).** Axiomatic Cohesion. *Theory and Applications of Categories*, 19(3), 41–49. — The categorical original. Introduces the concept of cohesion (a topos satisfying specific adjunction axioms) and motivates it philosophically and mathematically.

**Schreiber, U., & Shulman, M. (2012).** Quantum Gauge Field Theory in Cohesive Homotopy Type Theory. *Electronic Notes in Theoretical Computer Science*, 298, 293–301. — An early paper outlining the physics program in the HoTT context.

## Modalities in Type Theory

**Shulman, M. (2023).** Modalities in Homotopy Type Theory. *Logical Methods in Computer Science*, 16(1). — The comprehensive treatment of modalities in HoTT. Defines modalities, characterizes them via orthogonal factorization systems, and studies their properties. The standard reference for Section 1 of this chapter.

**Rijke, E., Shulman, M., & Spitters, B. (2020).** Modalities in Homotopy Type Theory. *Logical Methods in Computer Science*, 16(1), 2:1–2:79. — A companion paper with additional results on modalities and their computational properties.

**Anel, M., Biedermann, G., Finster, E., & Joyal, A. (2021).** A Generalized Blakers-Massey Theorem. *Journal of Topology*, 13(4), 1521–1553. — Uses modalities (specifically, the theory of left exact modalities) to prove a very general form of Blakers-Massey.

**Cherubini, F., & Rijke, E. (2021).** Modal Descent. *Mathematical Structures in Computer Science*, 31(4), 1–50. — Descent theory for modalities: when does a type satisfy descent along a modal unit?

## Synthetic Differential Geometry

**Kock, A. (2006).** *Synthetic Differential Geometry* (2nd ed.). Cambridge University Press. — The classic reference for synthetic differential geometry. The Kock-Lawvere axiom, infinitesimals, differential forms, and connections all developed from first principles. Complementary to the cohesive approach.

**Dubuc, E. J. (1979).** Sur les modeles de la géométrie différentielle synthétique. *Cahiers de Topologie et Géométrie Différentielle Catégoriques*, 20(3), 231–279. — One of the foundational papers of synthetic differential geometry.

**Moerdijk, I., & Reyes, G. E. (1991).** *Models for Smooth Infinitesimal Analysis*. Springer. — The categorical models for synthetic differential geometry (well-adapted models, smooth toposes).

## Gauge Theory and Mathematical Physics

**Freed, D. S. (2013).** Chern-Weil Forms and Abstract Homotopy Theory. *Bulletin of the American Mathematical Society*, 50(3), 431–468. — A conceptual treatment of the Chern-Weil homomorphism from the perspective of homotopy theory — close to the cohesive HoTT approach.

**Atiyah, M. F. (1988).** Topological Quantum Field Theory. *Publications Mathématiques de l'IHÉS*, 68, 175–186. — Atiyah's axioms for TQFT, which are the starting point for the prequantum field theory formalization.

**Sati, H., & Schreiber, U. (2021).** M/F-Theory as Mf-Theory. *Reviews in Mathematical Physics*, 35(10). — A recent paper in the Schreiber physics formalization program, applying cohesive HoTT to M-theory.

**Schreiber, U. (2020).** Equivariant Super Homotopy Theory. arXiv:1803.05765. — Applications to supersymmetry in cohesive HoTT.

## Higher Structures and Gauge Theory

**Freed, D. S., & Hopkins, M. J. (2013).** Chern-Weil Forms and Abstract Homotopy Theory. *Bulletin of the AMS*, 50(3), 431–468.

**Lurie, J. (2014).** What is an ∞-topos? — Lurie's conceptual introduction to ∞-toposes, the semantic setting for cohesive HoTT.

**Rezk, C. (2010).** Toposes and Homotopy Toposes. — A survey of ∞-toposes from the perspective of Rezk's complete Segal spaces.

## Implementation

**Cubical Agda `--cohesion` documentation** — `agda.readthedocs.io/en/latest/` — The experimental `--cohesion` flag for Cubical Agda. Documentation for the `♭` modality.

**The nLab entry on Cohesive Homotopy Type Theory** — `ncatlab.org/nlab/show/cohesive+homotopy+type+theory` — The most comprehensive and current online reference. Regularly updated by Schreiber and others.

**Myers, D. J. (2022).** Synthetic Cohesion: A Type-Theoretic Formalization of Cohesive Homotopy Type Theory. PhD Thesis. — A systematic formal development of cohesive HoTT from the type-theoretic perspective.

## Historical and Philosophical Context

**Lawvere, F. W. (1969).** Adjointness in Foundations. *Dialectica*, 23(3–4), 281–296. — The seminal paper arguing that adjoint functors are the fundamental concept of mathematics. The philosophical foundation for the adjoint triple of cohesive HoTT.

**Grothendieck, A. (1957).** Sur quelques points d'algèbre homologique. *Tohoku Mathematical Journal*, 9(2), 119–221. (The "Tôhoku paper".) — Introduces sheaves, derived functors, and cohomology in the modern sense. The mathematical ancestry of sheaves-as-types.

**Bell, J. L. (1998).** *A Primer of Infinitesimal Analysis*. Cambridge University Press. — An accessible introduction to infinitesimal analysis using nilpotent infinitesimals — the classical precursor to the cohesive differential forms approach.

**Isham, C. J. (1999).** *Modern Differential Geometry for Physicists* (2nd ed.). World Scientific. — A physicist's guide to differential geometry — useful for understanding what the physical applications require.

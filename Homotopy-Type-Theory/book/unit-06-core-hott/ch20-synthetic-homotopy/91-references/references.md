# References: Synthetic Homotopy Theory

## Primary Sources

**Brunerie, Guillaume.** *On the Homotopy Groups of Spheres in Homotopy Type Theory.* PhD thesis, Université Nice Sophia Antipolis, 2016.
- The definitive synthetic treatment of homotopy groups of spheres through π₄(S³). Introduces the Brunerie number β and proves π₄(S³) = Z/|β|Z. The most technically ambitious single work in synthetic homotopy theory to date. Available at: https://arxiv.org/abs/1606.05916

**Licata, Daniel R. and Shulman, Michael.** "Calculating the Fundamental Group of the Circle in Homotopy Type Theory." *Proceedings of LICS 2013*, 2013.
- The first fully formalized proof of π₁(S¹) = Z, establishing the encode-decode method as the canonical tool for homotopy group computations. Essential reading for Section 2 of this chapter.

**Favonia (Kuen-Bang Hou) and Shulman, Michael.** "The Seifert-van Kampen Theorem in Homotopy Type Theory." *Proceedings of CSL 2016*, 2016.
- The formalization of van Kampen's theorem using the universal property of pushouts. Demonstrates how the HoTT proof is simultaneously simpler and more general than the classical version.

**Anel, Mathieu; Biedermann, Georg; Finster, Eric; and Joyal, André.** "A Generalized Blakers-Massey Theorem." *Journal of Topology*, 13(4), 1521–1553, 2020.
- The proof of the Blakers-Massey theorem in HoTT, with the Freudenthal Suspension Theorem as a corollary. The HoTT proof is strictly stronger than the classical version: it works for any ∞-topos and is entirely constructive.

**Lumsdaine, Peter LeFanu and Shulman, Michael.** "Semantics of Higher Inductive Types." *Mathematical Proceedings of the Cambridge Philosophical Society*, 169(1), 159–208, 2020.
- The foundational paper giving a rigorous semantics for HITs in homotopy type theory. Establishes that the HITs used in synthetic homotopy theory (circle, suspension, pushouts) are well-defined.

**Hopf, Heinz.** "Über die Abbildungen der dreidimensionalen Sphäre auf die Kugeloberfläche." *Mathematische Annalen*, 104(1), 637–665, 1931.
- The original paper in which Hopf discovered the Hopf fibration and computed π₃(S²) = Z. One of the most important papers in the history of algebraic topology. In German; the key ideas are reconstructed in the section on the Hopf fibration.

## The HoTT Book

**The Univalent Foundations Program.** *Homotopy Type Theory: Univalent Foundations of Mathematics.* Institute for Advanced Study, Princeton, 2013. Available at: https://homotopytypetheory.org/book/
- Chapter 6: Higher Inductive Types (circles, suspensions, pushouts, truncations)
- Chapter 8: Homotopy Theory (encode-decode, van Kampen, Freudenthal, Hopf fibration, π₁(S¹) = Z)

Chapter 8 is the primary reference for the material in this chapter. It contains complete proofs of π₁(S¹) = Z, the van Kampen theorem, the Freudenthal suspension theorem, and the construction of the Hopf fibration — all in HoTT.

## Computational Implementations

**Ljungström, Axel and Mörtberg, Anders.** "Formalizing π₄(S³) ≅ Z/2Z and Computing Brunerie's Number in Cubical Agda." *Proceedings of LICS 2023*, 2023.
- The formal verification that the Brunerie number β = 2, completing the proof of π₄(S³) = Z/2Z. The paper describes both the mathematical proof and the computational verification in Cubical Agda.

**Cohen, Cyril; Coquand, Thierry; Huber, Simon; and Mörtberg, Anders.** "Cubical Type Theory: A Constructive Interpretation of the Univalence Axiom." *Proceedings of TYPES 2015*, 2018.
- Cubical type theory, in which the Univalence Axiom is not merely an axiom but has a computation rule. Essential for making the encode-decode method and the computation of winding numbers actually computable.

**HoTT-Agda Library.** Available at: https://github.com/HoTT/HoTT-Agda
- The Agda library in which many of the results of this chapter were first formalized, including the encode-decode proofs of Licata-Shulman and the formalization work of Favonia.

**Cubical Agda Library.** Available at: https://github.com/agda/cubical
- The standard library for Cubical Agda, containing formalizations of the Brunerie number computation, the Hopf fibration, and many other synthetic homotopy theory results.

## Background: Classical Homotopy Theory

**Hatcher, Allen.** *Algebraic Topology.* Cambridge University Press, 2002. Available at: https://pi.math.cornell.edu/~hatcher/AT/ATpage.html
- The standard modern textbook on algebraic topology. Chapter 4 covers homotopy groups of spheres, the Hopf fibration, and the Freudenthal suspension theorem from the classical perspective. Comparing the classical proofs with the HoTT proofs is extremely instructive.

**Whitehead, George W.** *Elements of Homotopy Theory.* Springer-Verlag, 1978.
- The comprehensive classical reference for homotopy theory, including the long exact sequence of fibrations, the Blakers-Massey theorem, and stable homotopy theory. At roughly 750 pages, it illustrates how much machinery the synthetic approach bypasses.

**May, J. Peter.** *A Concise Course in Algebraic Topology.* University of Chicago Press, 1999.
- A shorter, more conceptual treatment of classical algebraic topology. Particularly useful for understanding the relationship between fibrations, fiber bundles, and the long exact sequence — the classical backdrop for Section 5 of this chapter.

**Toda, Hiroshi.** *Composition Methods in Homotopy Groups of Spheres.* Princeton University Press, 1962.
- The classical reference for systematic computations of πₙ(Sᵏ). Contains the values tabulated in Section 5. Reading this alongside Brunerie's thesis illustrates the shift from classical to synthetic computation.

## Surveys and Introductions

**Shulman, Michael.** "Homotopy Type Theory: A Synthetic Approach to Higher Equalities." In *New Spaces in Mathematics*, Cambridge University Press, 2021.
- An excellent survey of HoTT from the perspective of synthetic mathematics. Includes discussion of synthetic homotopy theory and its relationship to classical homotopy theory.

**Rijke, Egbert.** *Introduction to Homotopy Type Theory.* 2022. Available at: https://arxiv.org/abs/2212.11082
- A comprehensive introduction to HoTT covering all the prerequisites for this chapter. Chapters 12-15 cover the synthetic homotopy theory developed here.

**Licata, Daniel R.** "Synthetic Homotopy Theory." Lecture notes from the summer school at the Institute for Advanced Study, 2012.
- The original lecture notes from the 2012 IAS special year, giving the first systematic presentation of synthetic homotopy theory. Written while the subject was being created. Essential historical document.

## Philosophical Context

**Voevodsky, Vladimir.** "An Experimental Library of Formalized Mathematics based on the Univalent Foundations." *Mathematical Structures in Computer Science*, 25(5), 1278–1294, 2015.
- Voevodsky's account of the UniMath project, motivated by his concern for the correctness of mathematics and the role of computer verification. The philosophy behind the entire enterprise of synthetic homotopy theory.

**Awodey, Steve.** "Structuralism, Invariance, and Univalence." *Philosophia Mathematica*, 22(1), 1–11, 2014.
- A philosophical analysis of why the Univalence Axiom is mathematically and philosophically correct, in terms of the structure invariance principle. Relevant to understanding why synthetic homotopy theory is not just technically convenient but philosophically justified.

## Notes on Mathematical Prerequisites

For readers approaching this chapter without extensive classical algebraic topology background, we recommend the following reading sequence:

1. **Sections 1-2** (encode-decode method, π₁(S¹) = Z): No classical background required. Read Chapter 19 (Higher Inductive Types) first.

2. **Section 3** (van Kampen): Classical background helpful but not required. The classical statement is in Hatcher §1.2. The HoTT proof requires only the universal property of pushouts.

3. **Section 4** (Freudenthal): Basic understanding of connectivity and homotopy groups helpful. The classical statement is in Hatcher §4.2. The HoTT proof requires Blakers-Massey (referenced above).

4. **Section 5** (Hopf fibration): Classical Hopf fibration discussed in Hatcher §4.3. The HoTT proof requires the join construction from Chapter 19 and the Freudenthal theorem.

The reader who works through the HoTT Book Chapters 6-8 alongside this chapter will have the complete picture of what synthetic homotopy theory has achieved.

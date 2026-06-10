# References — Chapter 23: Cubical Type Theory

## Primary Sources

**Cohen, C., Coquand, T., Huber, S., & Mörtberg, A. (2018).** Cubical Type Theory: A Constructive Interpretation of the Univalence Axiom. *TYPES 2015*, Leibniz International Proceedings in Informatics (LIPIcs), Vol. 69. — The foundational paper. The CCHM paper defines the interval, face formulas, partial elements, `hcomp`, `transp`, the Glue type, and proves univalence as a theorem. Every subsequent development in this chapter is downstream of this paper.

**Huber, S. (2016).** Cubical Interpretations of Type Theory. *PhD Thesis, University of Gothenburg.* — The metatheory: consistency and canonicity proofs for CCHM. Essential reading for understanding why the system works, not just how.

**Angiuli, C., Harper, R., & Wilson, T. (2018).** Computational Higher-Dimensional Type Theory. *POPL 2018.* — The Cartesian cubical type theory perspective, grounded in computational type theory (operational semantics first).

**Angiuli, C., Hou (Favonia), K.-B., & Harper, R. (2018).** Cartesian Cubical Computational Type Theory: Constructive Reasoning with Paths and Equalities. *CSL 2018.* — Full development of CCTT with NbE.

**Sterling, J., Angiuli, C., & Gratzer, D. (2019).** Cubical Syntax for Reflection-Free Extensional Equality. *FSCD 2019.* — The XTT paper. Introduces boundary separation and establishes XTT as a strict cubical type theory.

**Bezem, M., Coquand, T., & Huber, S. (2014).** A Model of Type Theory in Cubical Sets. *TYPES 2013.* — The precursor paper, introducing the BCH model (without complement), which motivated the CCHM extension.

## Metatheory and Models

**Orton, I., & Pitts, A. M. (2016).** Axioms for Modelling Cubical Type Theory in a Topos. *CSL 2016.* — Abstract axiomatization of the CCHM model, making the categorical structure explicit.

**Coquand, T., Huber, S., & Mörtberg, A. (2018).** On Higher Inductive Types in Cubical Type Theory. *LICS 2018.* — HITs in CCHM: how to add higher inductive types (suspension, pushouts, propositional truncation) with computation rules.

**Cavallo, E., & Harper, R. (2019).** Higher Inductive Types in Cubical Computational Type Theory. *POPL 2019.* — HITs in the Cartesian setting.

**Licata, D. R., Orton, I., Pitts, A. M., & Spitters, B. (2018).** Internal Universes in Models of Homotopy Type Theory. *FSCD 2018.* — Internal universes and the Glue construction from the categorical perspective.

## Implementation and Practice

**Vezzosi, A., Mörtberg, A., & Abel, A. (2019).** Cubical Agda: A Dependently Typed Programming Language with Univalence and Higher Inductive Types. *ICFP 2019.* — The Cubical Agda paper: design choices, implementation, performance, examples.

**Mörtberg, A., & Pujet, L. (2020).** Cubical Synthetic Homotopy Theory. *CPP 2020.* — Examples of synthetic homotopy in Cubical Agda: Seifert-van Kampen, Blakers-Massey.

**Brunerie, G. (2016).** On the Homotopy Groups of Spheres in Homotopy Type Theory. *PhD Thesis, Université Nice Sophia Antipolis.* — The thesis defining the Brunerie number and the synthetic program for computing homotopy groups.

**Brunerie, G., Ljungström, A., & Mörtberg, A. (2022).** Synthetic Integral Cohomology in Cubical Agda. *CSL 2022.* — Cohomology computations in Cubical Agda using the cubical infrastructure.

## Normalization

**Sterling, J., & Angiuli, C. (2021).** Normalization for Cubical Type Theory. *LICS 2021.* — Synthetic Tait computability applied to cubical type theory: a rigorous normalization proof.

**Gratzer, D., Sterling, J., & Birkedal, L. (2019).** Implementing a Modal Dependent Type Theory. *ICFP 2019.* — NbE for type theories with modalities, applicable to XTT.

## Historical and Philosophical Context

**Martin-Löf, P. (1975).** An Intuitionistic Theory of Types. — The original intensional type theory. Understanding why canonicity matters requires understanding what Martin-Löf intended.

**Hofmann, M., & Streicher, T. (1994).** The Groupoid Interpretation of Type Theory. — The original groupoid model, motivating the homotopy interpretation of identity types.

**Awodey, S., & Warren, M. A. (2009).** Homotopy Theoretic Models of Identity Types. *Mathematical Proceedings of the Cambridge Philosophical Society.* — The initial connection between identity types and homotopy theory.

## Web Resources

**The Cubical Agda library** — `github.com/agda/cubical` — The primary library for cubical Agda, with over 10,000 lines of formalized mathematics. The source code is the best practical reference for how to use cubical type theory.

**cooltt** — `github.com/RedPRL/cooltt` — Reference implementation of Cartesian cubical type theory.

**Agda documentation: Cubical mode** — `agda.readthedocs.io/en/latest/cubical-compatible.html` — Documentation for the `--cubical` flag and the primitives.

# Chapter 3: Number Systems

---

## Chapter Introduction

Numbers are so familiar that we rarely question what they *are*. We learned to count as children; we learned fractions and decimals in school; we accepted the existence of irrational numbers like √2 and π without much analysis. But what, exactly, *is* the number 2? What is the number π? What makes the real number system special?

These questions were not taken seriously until the 19th century, when mathematicians working on the foundations of analysis realized that intuitive notions of "number" were insufficient. The same crisis that produced rigorous calculus (Cauchy, Weierstrass) required a rigorous construction of the real numbers themselves. Richard Dedekind gave the first rigorous construction in 1872, using "cuts" in the rationals. Cantor, working independently, used equivalence classes of Cauchy sequences.

This chapter constructs the standard number systems from the ground up, each one solving a limitation of the previous:

- **Natural numbers ℕ**: defined by Peano's axioms, they give us arithmetic.
- **Integers ℤ**: extend ℕ so that subtraction is always defined.
- **Rationals ℚ**: extend ℤ so that division (by nonzero elements) is always defined.
- **Reals ℝ**: extend ℚ to fill the "gaps" — the irrational numbers like √2, π, and e that ℚ misses. The real numbers form the arena of analysis.
- **Complex numbers ℂ**: extend ℝ so that every polynomial has a root. They are essential for quantum mechanics and for the analytic methods used in GR.

By the end of this chapter, you will have a precise, rigorous understanding of what numbers are — and an appreciation for how much careful work it took to achieve that understanding.

---

## Why Number Systems Matter for General Relativity

The spacetime manifold in GR is a topological space locally homeomorphic to ℝ⁴. The metric tensor at each point is a bilinear form on the tangent space, taking values in ℝ. Geodesics are curves from an interval in ℝ into the manifold. The Einstein field equations are partial differential equations relating tensor fields over ℝ⁴.

Every one of these structures depends on the real numbers in a fundamental way. The completeness of ℝ — the absence of gaps — is what makes analysis (and hence calculus, and hence differential geometry, and hence GR) possible. If spacetime were described over the rationals, continuity would be a pathological concept, and the smooth manifold structure would collapse.

The complex numbers appear in GR through the Newman-Penrose formalism (Chapter 54), spinors (Chapter 54), and the analytic continuation used in the study of Hawking radiation (Chapter 57) and the definition of the Hartle-Hawking state in quantum cosmology.

---

## Sections in This Chapter

- [Section 3.1: Natural Numbers and Integers](section-3.1-natural-numbers-integers/README.md)
- [Section 3.2: Rational Numbers](section-3.2-rational-numbers/README.md)
- [Section 3.3: Real Numbers](section-3.3-real-numbers/README.md)
- [Section 3.4: Complex Numbers](section-3.4-complex-numbers/README.md)
- [Exercises](exercises.md)
- [Further Reading and References](further-reading.md)
- [Important Researchers](important-researchers.md)
- [Important Concepts](important-concepts.md)

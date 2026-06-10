# Chapter 25: Modal Homotopy Type Theory — Cohesion and Geometry

## The Geometry Problem

Standard HoTT lives in a world without geometry. Every type is an ∞-groupoid, every function is a morphism, and the type theory is completely "abstract" — it can't distinguish between a smooth function and a discontinuous one, between a contractible space and a discrete set, between a differential form and an arbitrary cochain.

This isn't a bug — it's a feature of HoTT as a foundation for *homotopy theory*, where you want to ignore geometric structure and focus on topological invariants. But for geometry, gauge theory, and differential topology, you need to retain the distinction.

*Modal HoTT* extends homotopy type theory with *modalities* — type-level operators that capture geometric structure. The most important example is *cohesive HoTT* (Schreiber-Shulman), which adds modalities for:
- **Discreteness**: which types have no continuous paths?
- **Homotopy type**: what is the underlying homotopy type of a smooth space?
- **Codiscreteness**: which types are "totally connected"?

With these modalities, you can do differential geometry, gauge theory, and even parts of mathematical physics *synthetically* — from axioms, without coordinates or point-set constructions.

## Modalities: The General Framework

A *modality* in type theory is a type-level operator $\bigcirc$ with a universal property: $\bigcirc A$ is the "best approximation" of $A$ satisfying some property.

Familiar examples:
- Propositional truncation $\|A\|$ is a modality (the "propositional approximation" of $A$)
- $n$-truncation $\|A\|_n$ is a modality (the "$n$-type approximation")
- In cohesive HoTT, the shape $\int A$, flat $\flat A$, and sharp $\sharp A$ are modalities

The general theory of modalities — when they exist, how they compose, what their semantics is — is developed in this chapter.

## Chapter Roadmap

**Section 1: Modalities** — The definition of a modality, left exact modalities, examples.

**Section 2: Cohesive HoTT** — The cohesion axioms, the three modalities $\int$, $\flat$, $\sharp$, their intuition and properties.

**Section 3: Differential Geometry** — Recovering de Rham cohomology, differential forms, and connections synthetically.

**Section 4: Applications** — Principal bundles, Chern-Weil theory, connections to physics.

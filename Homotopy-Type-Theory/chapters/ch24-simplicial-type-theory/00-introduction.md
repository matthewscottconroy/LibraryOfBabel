# Chapter 24: Simplicial Type Theory — Synthetic ∞-Category Theory

## The Missing Direction

Homotopy type theory is the internal language of ∞-groupoids: every type is an ∞-groupoid where all paths are invertible. This is perfect for homotopy theory, but it misses half of mathematics.

Much of what mathematicians actually study is *directed*: category theory, where morphisms need not be invertible; module theory, where homomorphisms may not be isomorphisms; order theory, where the order relation is asymmetric. The fundamental operation of composition in a category goes one way — from a morphism $f : A \to B$ and $g : B \to C$, you get $g \circ f : A \to C$, but there's no reason to expect a morphism back from $C$ to $A$.

HoTT can't directly reason about this directed structure. In HoTT, every path $p : a = b$ has an inverse `sym p : b = a`. Every morphism is an equivalence. The type theory enforces invertibility.

*Simplicial type theory* (STT), developed by Emily Riehl and Michael Shulman in a series of papers starting from 2017, addresses this gap. It's a type theory designed to be the internal language of ∞-toposes, where types can be ∞-categories (with directed, non-invertible morphisms) as well as ∞-groupoids (with invertible paths).

The key innovation: a second interval, the *simplicial interval* $\mathbf{2}$, which models directed paths. Unlike the cubical interval $\mathbb{I}$ (which has a complement making paths reversible), the simplicial interval has no complement — its morphisms are directed.

## What STT Can Do That HoTT Cannot

In simplicial type theory, you can reason about:

**Functors and natural transformations synthetically.** A functor between Segal types is just a function. A natural transformation is a directed homotopy. The naturality condition is automatic from the type-theoretic structure.

**The Yoneda lemma.** The representable functor $\mathsf{hom}(a, -)$ represents natural transformations to $F$: elements of $F(a)$ correspond to natural transformations from $\mathsf{hom}(a, -)$ to $F$. This is a theorem in STT.

**Limits and colimits.** Defined as universal elements in the ∞-categorical sense.

**Adjunctions.** A pair of functors with a natural bijection on hom-sets.

All without needing to work with quasi-categories, simplicial sets, or any external model. Just type theory.

## Chapter Roadmap

**Section 1: Two Intervals** — The cubical interval $\mathbb{I}$ (undirected) vs. the simplicial interval $\mathbf{2}$ (directed). The hom type.

**Section 2: Segal Types** — The Segal condition (composition is unique), Segal types as ∞-categories, examples.

**Section 3: Rezk Types** — Complete Segal spaces in type theory. The Rezk condition. Directed univalence.

**Section 4: Functors and the Yoneda Lemma** — Functors as functions, natural transformations as directed homotopies, the synthetic Yoneda lemma.

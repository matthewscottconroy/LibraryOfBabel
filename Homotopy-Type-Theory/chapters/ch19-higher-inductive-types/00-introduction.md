# Chapter 19: Higher Inductive Types

## Defining Spaces by Their Paths

Ordinary inductive types are defined by *point constructors* — ways to build new elements from existing ones. The natural numbers have zero and successor. Lists have nil and cons. Trees have leaves and nodes. Every element of an inductive type is built from these constructors.

This is powerful, but it can only build "discrete" types — types where all paths are trivial (reflexivity). The natural numbers are a set; lists form a set; trees form a set. You can't build the circle as an ordinary inductive type, because the circle has a non-trivial loop.

**Higher inductive types (HITs)** fix this by allowing *path constructors* in addition to point constructors. A HIT can specify not just the elements it contains, but also the paths that exist between those elements.

The paradigmatic example: the circle $S^1$ has one point ($\mathsf{base}$) and one non-trivial loop ($\mathsf{loop} : \mathsf{base} = \mathsf{base}$). This is not possible in ordinary type theory, but as a HIT it works perfectly.

## What HITs Can Build

With HITs, we can define directly in type theory:
- **Topological spaces:** Circle, spheres, tori, real projective planes, suspensions, joins
- **Algebraic constructions:** Free groups, free algebras, coequalizers
- **Logical constructions:** Propositional truncation, set truncation, $n$-truncation
- **Colimits:** Pushouts, coproducts, sequential colimits
- **Eilenberg-MacLane spaces:** $K(G, n)$ for any group $G$ and $n \geq 1$

These are not just "models" of these things — they *are* these things, built directly in the type theory with all the correct homotopy-theoretic properties.

## The Power of HITs

HITs transform HoTT from a logical framework into a genuine geometric/topological language. Before HITs, HoTT could reason about homotopy types that were already present (as Kan complexes in the model), but couldn't construct new ones. With HITs, you can build any space you need from scratch.

This enables *synthetic homotopy theory* (Chapter 20): proving theorems about homotopy groups, fibrations, and spectra entirely within type theory, without any external geometric constructions.

## Chapter Roadmap

**Section 1: The Interval and Circle.** The two simplest HITs. The interval is contractible but gives function extensionality. The circle has $\pi_1 = \mathbb{Z}$ and is the paradigmatic non-trivial HIT.

**Section 2: Suspensions and Spheres.** The suspension construction iteratively builds spheres. $\Sigma^n \mathbf{Bool} = S^n$.

**Section 3: Pushouts.** The general "gluing" construction. Suspension and join are special cases. The van Kampen theorem follows.

**Section 4: Truncations as HITs.** The propositional truncation $\|A\|$ and $n$-truncations are HITs. This is the definition underlying Chapter 17's use.

**Section 5: Eilenberg-MacLane Spaces.** $K(G, n)$ constructed as HITs. These are the building blocks of cohomology in HoTT.

## Prerequisites and Connections

Builds on:
- Chapter 16 (Identity Types): Path constructors are identity type elements
- Chapter 17 (H-Levels): Truncations are HITs; h-levels determine the HIT structure
- Chapter 18 (Univalence): The $\pi_1(S^1) = \mathbb{Z}$ proof uses Univalence to define the code family

Connects forward to:
- Chapter 20 (Synthetic Homotopy): All the spaces for synthetic homotopy theory are HITs
- Chapter 22 (Cubical Agda): HITs have computational content in cubical type theory
- Chapter 23 (Cubical Type Theory): Where HITs become first-class and computable

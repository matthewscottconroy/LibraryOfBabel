# Eilenberg-MacLane Spaces and Spectra

## Cohomology from Type Theory

The most important objects in algebraic topology are, arguably, not individual spaces but *invariants* of spaces — cohomology groups. The n-th cohomology group H^n(X; G) of a space X with coefficients in an abelian group G captures information about n-dimensional "holes" in X.

A profound theorem of algebraic topology says: cohomology is *representable*. There exist spaces K(G,n) — the Eilenberg-MacLane spaces — such that:
```
H^n(X; G) ≅ [X, K(G,n)]
```

Maps from X to K(G,n), up to homotopy, are the same as elements of H^n(X;G). The space K(G,n) "represents" n-dimensional G-valued cohomology.

In HoTT, we can define K(G,n) as HITs, and the cohomology theory they represent becomes internal to the type theory.

## The Eilenberg-MacLane Space K(G,1)

For an abelian group G (or even a non-abelian group), K(G,1) is the "classifying space" of G:

**Definition.** K(G,1) (also written BG) is the 1-type with fundamental group G.

In HoTT, we construct BG as a HIT:

**Definition.** BG is the HIT with:
- Point constructor: `pt : BG`
- For each g : G, a path constructor: `g-loop : pt = pt`
- Path-of-path constructors: `mult(g,h) : g-loop · h-loop = (g·h)-loop` (group operation is path concatenation)
- 2-path constructors making BG a 1-type (set-truncating the path types between paths)

The key property: `(pt =_{BG} pt) ≃ G`. The loop space of BG at pt is G.

**Delooping.** The process of constructing BG from G is called *delooping* — we are constructing a space whose loop space is G. This is the inverse of the loop-space functor Ω.

The delooping gives an adjunction: `G = ΩBG` and `BAut(BG) = BG` (roughly).

## K(G,n) for Higher n

For n ≥ 2, K(G,n) is a type with:
```
πₙ(K(G,n)) = G
πₖ(K(G,n)) = 0 for k ≠ n
```

The higher Eilenberg-MacLane spaces are constructed iteratively:
- K(G,0) = G (the discrete type with elements of G)
- K(G,1) = BG (the classifying space, as above)
- K(G,n) = B(K(G,n-1)) for n ≥ 2 (the delooping of the previous space)

This requires G to be abelian for n ≥ 2 (since πₙ for n ≥ 2 is always abelian, by Eckmann-Hilton).

**As a HIT.** K(G,2) (for abelian G) is the HIT with:
- One point pt
- 2-path constructors: for each g : G, a 2-path `g-disk : refl_pt = refl_pt`
- 3-path constructors enforcing the group structure on 2-paths
- Higher path constructors making K(G,2) a 2-type

## Cohomology in HoTT

With K(G,n) defined as a HIT, cohomology becomes internal to HoTT:

**Definition.** The n-th cohomology group of a type X with coefficients in G is:
```
H^n(X; G) := ‖X →* K(G,n)‖₀
```

The set-truncation of the type of pointed maps from X to K(G,n).

This is the *representability* of cohomology in HoTT: cohomology is defined as the set of maps to the representing space, with the group structure given by the group structure of K(G,n).

**Key properties:**
- H^0(X; G) = ‖X →* G‖₀ (locally constant G-valued functions)
- H^1(X; G) = ‖X →* BG‖₀ (G-bundles over X, up to isomorphism)
- H^n(X; G) = ‖X →* K(G,n)‖₀ in general

**The long exact sequence.** For a fibration F → E → B, the long exact sequence in cohomology:
```
... → H^{n-1}(F;G) → H^n(B;G) → H^n(E;G) → H^n(F;G) → ...
```

follows from the fiber sequence in HoTT, applied to the representing spaces K(G,n).

## Spectra in HoTT

A *spectrum* is a sequence of types and maps that captures *stable* homotopy theory — homotopy theory "in the limit" as suspension stabilizes.

**Definition.** A spectrum E consists of:
- Types Eₙ for each n : Z
- Equivalences σₙ : Eₙ ≃ ΩEₙ₊₁ (the spectrum maps)

The Eilenberg-MacLane spectrum HG is the spectrum with (HG)ₙ = K(G,n) for n ≥ 0 and (HG)ₙ = 1 for n < 0. The spectrum maps are the delooping maps K(G,n) ≃ ΩK(G,n+1).

**Stable homotopy groups.** The stable homotopy groups of a spectrum E are:
```
πₙ(E) = πₙ₊ₖ(Eₖ) for any sufficiently large k
```

These are well-defined because the Freudenthal theorem ensures stability for large k.

**Cohomology from spectra.** For a spectrum E:
```
H^n(X; E) = [X, Eₙ]
```

Eilenberg-MacLane spectra give ordinary cohomology. Other spectra give exotic cohomology theories (K-theory, cobordism, etc.).

## The Synthetic Theory of Cohomology

The HoTT development of cohomology, using Eilenberg-MacLane spaces defined as HITs, has several advantages over the classical development:

**Synthetic.** No need for singular chains, simplicial sets, or other analytic constructions. Cohomology is defined directly as a type of maps.

**Automatic functoriality.** A map f : X → Y induces f* : H^n(Y;G) → H^n(X;G) by precomposition. No separate functoriality proof is needed.

**Covariant in X.** For a covariant (homology) version, use the left Kan extension or homotopy pushforward.

**Verified computations.** The cohomology of specific spaces (S^n, projective spaces, Grassmannians) can be computed in HoTT by constructing explicit maps to K(G,n) and verifying they generate the cohomology.

## Summary

| Space | Definition | Key property |
|---|---|---|
| BG = K(G,1) | HIT with G as loop space | Classifying space; H^1(-;G) = G-bundles |
| K(G,n) | n-fold delooping | πₙ=G, πₖ=0 for k≠n |
| HG (spectrum) | Sequence K(G,n) with maps | Ordinary G-cohomology |

Eilenberg-MacLane spaces and spectra show that cohomology theory — one of the most powerful tools in algebraic topology — is fully internal to HoTT, defined and computed using HITs and the synthetic tools of the type theory. This is the payoff of the whole development: a fully synthetic, computationally meaningful theory of cohomology.

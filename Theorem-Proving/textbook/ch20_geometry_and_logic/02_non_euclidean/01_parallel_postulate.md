# Non-Euclidean Geometry and the Parallel Postulate

Euclid deferred his fifth postulate to Proposition I.29, as if hoping to avoid it, and for two thousand years geometers tried to discharge the hope entirely — to *prove* the parallel postulate from the other four. Every attempt failed, and for the deepest possible reason: the postulate is **independent**. There are consistent geometries in which it is false. Establishing this required inventing the modern notion of a **model**, and with it the first rigorous independence proof in mathematics — the direct ancestor of the independence proofs for the Axiom of Choice and the Continuum Hypothesis (Chapter 6).

## Equivalents of the Postulate

Over **neutral geometry** — the incidence, order, and congruence axioms with no parallel assumption (Hilbert's Groups I–III, [Section 3](../03_hilbert/01_hilbert_axioms.md)) — Postulate 5 is provably equivalent to a list of familiar statements. Each can replace it, and each is therefore equally *unprovable* without it:

- **Playfair's axiom**: through a point not on a line there is *exactly one* parallel.
- Every triangle has angle sum exactly $180^\circ$ (or: some one triangle does — Legendre).
- Rectangles exist; similar non-congruent triangles exist; the Pythagorean theorem holds.

That so heterogeneous a list stands or falls together shows the postulate encodes something structural — the *flatness* of the plane — not an isolated fact.

## Saccheri and Neutral Geometry

The most searching pre-modern attempt was Girolamo Saccheri's *Euclides ab omni naevo vindicatus* ("Euclid Freed of Every Flaw," 1733). From the **Saccheri quadrilateral** $ABCD$ (with $AB \perp AD$, $AB \perp BC$, $AD \cong BC$) he studied the summit angles at $C$ and $D$, which neutral geometry proves congruent. Exactly three hypotheses are possible:

| Hypothesis | Summit angles | Triangle angle sum | Geometry |
|---|---|---|---|
| Right (HRA) | $= 90^\circ$ | $= 180^\circ$ | Euclidean |
| Obtuse (HOA) | $> 90^\circ$ | $> 180^\circ$ | elliptic |
| Acute (HAA) | $< 90^\circ$ | $< 180^\circ$ | hyperbolic |

Saccheri *refuted* the obtuse hypothesis (correctly, but only because his tacit infinite lines exclude elliptic geometry), and against the acute one derived theorem after theorem without contradiction — in fact proving dozens of theorems of hyperbolic geometry — before declaring it, without warrant, "repugnant to the nature of the straight line." Legendre later proved the **Saccheri–Legendre theorem**: in neutral geometry the angle sum of any triangle is *at most* $180^\circ$. Whether the deficit is zero, neutral geometry cannot decide — that decision *is* the parallel postulate.

## Hyperbolic Geometry

Developed independently by Gauss (who left it unpublished, fearing "the clamor of the Boeotians"), János Bolyai (1832), and Nikolai Lobachevsky (1829), **hyperbolic geometry** replaces Playfair's axiom with:

> **Hyperbolic parallel axiom.** Through a point not on a line pass *at least two* — hence infinitely many — lines that never meet the given line.

Its characteristic theorems follow:

- Every triangle has angle sum *strictly less than* $180^\circ$; the shortfall is the **defect** $\delta(T) = \pi - (\alpha+\beta+\gamma)$.
- There are **no similar non-congruent triangles**: equal angles force equal size (AAA is a congruence criterion), so there is an *absolute unit of length*.
- The circumference of a circle of radius $r$ grows exponentially, as $2\pi\sinh r$.
- For a point $P$ at distance $d$ from a line $\ell$, the two **limiting parallels** make with the perpendicular an **angle of parallelism** $\Pi(d) < 90^\circ$, given by the Bolyai–Lobachevsky formula $\tan\!\frac{\Pi(d)}{2} = e^{-d}$: locally Euclidean ($\Pi \to 90^\circ$ as $d\to 0$), sharply curved at large $d$. Lines through $P$ outside this angle are **ultraparallel** — sharing a common perpendicular and diverging on both sides.

## Models: Making the Geometry Concrete

None of this shows hyperbolic geometry *consistent*. That requires a **model** — an interpretation, built in trusted mathematics, under which all axioms come out true. Beltrami (1868), Klein, and Poincaré supplied several inside the unit disk $\mathbb{D} = \{z : |z| < 1\}$.

**The Beltrami–Klein model.** Points are the interior of $\mathbb{D}$; "lines" are open chords. Incidence and betweenness are the ordinary Euclidean ones, but the model is *not conformal* — congruence is measured by a projective (cross-ratio) metric.

*Worked example (parallels).* Let $\ell$ be a chord with ideal endpoints $A,B$ on the boundary, and $P$ a point off it. Two chords "meet" only at a shared interior point. The chords $PA$ and $PB$, being *open*, do not contain $A,B$, so they never meet $\ell$ inside the disk — they are the two **limiting parallels**. Every chord through $P$ in the open wedge between them also misses $\ell$. So infinitely many lines through $P$ are parallel to $\ell$: Playfair's axiom fails, visibly, in a Euclidean picture.

```
        _____
      /   P   \        P sees ℓ through the disk;
     | \  |  / |       chords PA and PB (dashed) are the
     |  \ | /  |       limiting parallels — they hit the
     A========B        boundary but never meet chord ℓ = AB
     |  ℓ inside |     inside the open disk.
      \ _______ /
```

**The Poincaré disk model.** Points are again the interior of $\mathbb{D}$, but "lines" are diameters and **arcs meeting the boundary at right angles**, with metric $ds = \dfrac{2\,|dz|}{1 - |z|^2}$, which sends the boundary to infinity. This model is **conformal** — hyperbolic angles equal Euclidean ones — which is why it underlies Escher's *Circle Limit* prints. The upper-half-plane model, $ds = |dz|/\operatorname{Im}z$, is isometric to it.

## The Angle-Defect Theorem

Hyperbolic area has a purely logical flavor — it is measured by angle deficit alone.

**Theorem (Gauss–Bolyai).** In the hyperbolic plane of curvature $-1$, $\operatorname{Area}(T) = \pi - (\alpha+\beta+\gamma)$.

*Proof idea.* Both area and defect are additive under subdivision and invariant under the isometry group; two such functionals are proportional. Computing an **ideal triangle** — all three vertices on the circle at infinity, all angles $0$ — fixes the constant: its defect is the maximal $\pi$ and, in the model, its area is $\pi$. $\square$

*Consequences.* No triangle has area $\ge \pi$; and since AAA fixes a triangle, there is no scaling — the deep reason hyperbolic geometry has an absolute unit of length.

## Elliptic Geometry

The obtuse hypothesis is realized by **elliptic geometry**, modeled on the sphere $S^2$ with antipodal points identified: "lines" are great circles, any two of which meet, so **there are no parallels at all**. The angle sum of a triangle *exceeds* $180^\circ$, and its area is proportional to the **excess** (Girard's theorem), $\operatorname{Area}(T) = R^2\bigl((\alpha+\beta+\gamma) - \pi\bigr)$. Elliptic geometry violates not only Postulate 5 but Euclid's tacit assumption that lines are infinite — which is exactly why Saccheri could dismiss the obtuse hypothesis: it clashes with the *order* axioms, not the parallel axiom alone.

## The Independence of the Parallel Postulate

The models discharge the two-thousand-year problem at a stroke. Each is built inside analytic geometry — ultimately inside the theory of $\mathbb{R}$ — so each yields a **relative consistency** result.

> **Theorem (Independence).** If Euclidean geometry (equivalently, the theory of $\mathbb{R}$) is consistent, the parallel postulate is independent of the remaining neutral axioms:
> - it is **not provable** from them — the Poincaré/Klein model satisfies the neutral axioms while refuting it;
> - its **negation is not provable** — the Cartesian plane $\mathbb{R}^2$ satisfies the neutral axioms *and* the postulate.

This is the paradigm of the model-theoretic method (Chapter 9): to show $\varphi$ unprovable from $\Gamma$, exhibit a model of $\Gamma \cup \{\neg\varphi\}$; independence needs models on both sides. It was the **first** clear instance, and it reset the meaning of "axiom": one cannot prove the postulate because neutral geometry simply does not determine whether space is flat. The same pattern later showed the Axiom of Choice and the Continuum Hypothesis independent of ZF (Gödel 1938, Cohen 1963). Geometry taught logic what independence is.

## From Curvature to Physics

Riemann (1854) subsumed all three geometries under **differential geometry** — a manifold whose curvature may vary from point to point, constant curvature $0$, $-1$, $+1$ recovering the Euclidean, hyperbolic, and elliptic planes. Sixty years later the abstraction became physics: in Einstein's general relativity (1915) spacetime is a four-dimensional pseudo-Riemannian manifold and gravitation *is* its curvature. Geometries that looked in 1830 like consistent fictions turned out to be the structure of reality — the merely logically possible is what physics later needs.

## Exercises
See [problems/ch20_geometry_and_logic/](../../../problems/ch20_geometry_and_logic/)

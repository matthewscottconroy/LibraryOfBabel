# 9.9 Partial Hyperbolicity

Full hyperbolicity — where every tangent direction is either contracting or expanding — is a powerful condition, but it's also a strong one. Many interesting systems have some hyperbolic directions but also directions that are neither contracting nor expanding. These are *partially hyperbolic* systems.

Not all interesting systems are fully hyperbolic. *Partial hyperbolicity* relaxes the requirement.

**Definition 9.9.1.** $f: M \to M$ is *partially hyperbolic* if there exists a $Df$-invariant splitting $TM = E^s \oplus E^c \oplus E^u$ (stable, center, unstable) with uniform expansion in $E^u$, contraction in $E^s$, and the center $E^c$ being "dominated" — weaker contraction/expansion than the extreme bundles.

The center bundle $E^c$ is neither contracting nor expanding, but it's "squeezed" between $E^s$ (which contracts more) and $E^u$ (which expands more). The domination condition says: the expansion in $E^u$ is stronger than any expansion in $E^c$, and the contraction in $E^s$ is stronger than any contraction in $E^c$.

**Examples:** Frame flows, geodesic flows on non-constant curvature manifolds, certain algebraic systems.

Frame flows are a good example to think about. The geodesic flow on a negatively curved manifold is Anosov. But the *frame flow* — the geodesic flow lifted to the bundle of orthonormal frames — has an additional rotation symmetry that prevents full hyperbolicity. It is partially hyperbolic, with the center bundle corresponding to the frame rotations.

**Open Problems:** Does partial hyperbolicity imply ergodicity? (Pugh-Shub conjecture, partially resolved.) Does every center-bunched partially hyperbolic system have finitely many ergodic measures?

The Pugh-Shub conjecture — that "stably ergodic" systems are dense among volume-preserving diffeomorphisms, and that partial hyperbolicity plus accessibility implies ergodicity — has motivated a large research program. The conjecture is partially resolved: accessibility (the condition that any two points can be connected by a path tangent to $E^s \cup E^u$) together with center-bunching does imply ergodicity (Burns-Wilkinson, 2010).

The question of how many ergodic measures a partially hyperbolic system can have is harder and remains largely open. For fully hyperbolic systems, the SRB measure is unique. For partially hyperbolic systems, there may be several, and their structure can be very complicated.

Partial hyperbolicity is the current frontier of hyperbolic dynamics — a theory that is rich and active, with many open problems and new techniques being developed. Chapter 11 will return to some of these questions.

---

## Looking Ahead

Hyperbolic dynamics is the best-understood part of smooth dynamical systems. The theory of Anosov diffeomorphisms, Markov partitions, and SRB measures gives a nearly complete picture. Chapter 10 shifts perspective: instead of asking about the structure of orbits within a fixed system, it asks what happens as we vary a parameter. How does the orbit structure change? When are qualitative changes — bifurcations — forced? That is the subject of bifurcation theory.

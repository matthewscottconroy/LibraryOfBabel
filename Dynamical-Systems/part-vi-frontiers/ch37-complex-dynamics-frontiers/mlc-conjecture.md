# 37.1 The MLC Conjecture

Local connectivity of the Mandelbrot set is one of the oldest and most important open problems in complex dynamics. Here's what's at stake.

**Conjecture 37.1.1 (MLC — Mandelbrot Set is Locally Connected).** The Mandelbrot set $\mathcal{M} \subset {\mathbb C}$ is locally connected.

Local connectivity means: at every point $c \in \mathcal{M}$, the connected components of $\mathcal{M} \cap U$ (for small neighborhoods $U$) are connected. For a set as complicated as $\mathcal{M}$, this is highly nontrivial — there are fractal sets that are connected but not locally connected, like the topologist's sine curve.

**Why MLC Matters:** If $\mathcal{M}$ is locally connected, then:
1. The Böttcher coordinate $\phi: {\mathbb C} \setminus \mathcal{M} \to {\mathbb C} \setminus \overline{\mathbb{D}}$ extends continuously to $\partial\mathcal{M}$
2. The *topological model* of $\mathcal{M}$ is the Carathéodory loop $\gamma = \phi^{-1}(e^{2\pi i\theta})$, $\theta \in {\mathbb R}/{\mathbb Z}$ — a topological circle
3. The *combinatorial description* of $\mathcal{M}$ via external angles is complete: every Misiurewicz point and hyperbolic component is identifiable by its external angles

Points 1 and 2 together mean: if MLC holds, the Mandelbrot set is a topological quotient of the circle by a combinatorial equivalence relation that can be described explicitly. The "fuzzy" fractal boundary would turn out to be completely described by the external angle map.

Point 3 means: every parameter in $\mathcal{M}$ can be identified by its "address" in terms of external angles, and that address completely determines the dynamics. MLC implies density of hyperbolic dynamics in the space of quadratic polynomials — one of the deepest open problems in dynamics.

**Theorem 37.1.2 (Known MLC Results).** MLC is proved for:
- All real parameters $c \in [-2, 1/4]$ (by Yoccoz 1990, using parapuzzle)
- All parameters of bounded combinatorial type (Yoccoz)
- All infinitely-renormalizable parameters of bounded type (Lyubich, 1997)
- Parameters in "combinatorial classes" of bounded type (Kahn-Lyubich, 2009)

**Theorem 37.1.3 (Yoccoz's Theorem — Local Connectivity at Finitely-Renormalizable Points).** If $c$ is not infinitely renormalizable, the Mandelbrot set is locally connected at $c$. The key tool: the *Yoccoz puzzle* (a partition of ${\mathbb C}$ into "pieces" by rays landing at the critical point).

So MLC is known at every finitely renormalizable parameter. The remaining cases are the infinitely renormalizable parameters of unbounded combinatorial type. These are the "deepest" points in the Mandelbrot set, and they're the hardest to understand.

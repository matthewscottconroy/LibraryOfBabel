# 37.3 The Lambda Lemma and Holomorphic Motions

How do Julia sets move as the parameter $c$ varies? The Lambda lemma — proved by Mañé, Sad, and Sullivan in 1983 — gives the fundamental answer: in structurally stable regions, Julia sets move holomorphically.

**Theorem 37.3.1 (Lambda Lemma — Mañé-Sad-Sullivan, 1983).** Let $\Lambda$ be a connected complex manifold, $z_0 \in {\mathbb C}$, and $f: \Lambda \times \{z_0\} \to {\mathbb C}$ a holomorphic motion (holomorphic in $\lambda$, injective in $z_0$, fixing a basepoint). Then $f$ extends to a holomorphic motion of the closure $\overline{\{z_0\}}$.

More precisely: any holomorphic motion of a set $E \subseteq \hat{{\mathbb C}}$ over a simply connected base extends (possibly after shrinking) to a holomorphic motion of $\hat{{\mathbb C}}$.

A *holomorphic motion* is a family of injective maps $f_\lambda: E \to \hat{\mathbb{C}}$ parametrized by $\lambda$ in a complex manifold, varying holomorphically in $\lambda$. The Lambda lemma says you can always extend such a motion to the entire Riemann sphere — the partial information determines the whole.

Słodkowski strengthened this in 1991 without the "after shrinking" caveat:

**Theorem 37.3.2 (Słodkowski, 1991).** Every holomorphic motion of any subset of $\hat{{\mathbb C}}$ over the unit disk $\mathbb{D}$ extends to a holomorphic motion of all of $\hat{{\mathbb C}}$ over $\mathbb{D}$.

This is a remarkable removability theorem for holomorphic motions. You don't need the base to shrink at all.

**Application 37.3.3 (Structural Stability).** In a structurally stable family $\{f_\lambda\}$ (no topological changes in Julia sets), the Julia sets move holomorphically. The Lambda lemma shows this motion extends to all of $\hat{{\mathbb C}}$, giving a quasiconformal deformation.

What this means for the Mandelbrot set: in any connected open region of parameter space where the Julia set doesn't undergo a topological change (no bifurcation), the Julia sets move quasiconformally. The maps $f_\lambda$ are all quasiconformally conjugate to each other. This is the rigidity that makes the Mandelbrot set's hyperbolic components easy to understand — within each component, the dynamics is "the same" in a very strong sense.

# 14.4 Twist Maps and Aubry-Mather Theory

KAM theory tells us which tori survive a small perturbation: the Diophantine ones. But it says nothing about what happens to the destroyed tori — the resonant ones and the ones with near-rational frequencies. They do not simply vanish without a trace. Aubry-Mather theory, developed independently by Aubry and Mather in the early 1980s, describes what is left.

The setting is *twist maps* — area-preserving maps of an annulus satisfying a monotone twist condition. The prototype is the *standard map*, the discrete version of a pendulum driven by a periodic kick.

**Definition 14.4.1.** A *twist map* is an area-preserving diffeomorphism $f: \mathbb{T} \times [a, b] \to \mathbb{T} \times [a,b]$ of the annulus satisfying the *twist condition*: the map $(q, p) \mapsto q'(q, p)$ is monotone increasing in $p$ for each fixed $q$. In Darboux coordinates: $\partial q'/\partial p > 0$.

The *standard map* is the prototype:
$$f(q, p) = (q + p + K\sin q,\ p + K\sin q) \pmod{2\pi}.$$
At $K = 0$ (integrable), orbits move on circles $\{p = \text{const}\}$. For small $K$, most circles persist (by KAM), but resonant ones break. For $K \approx 0.9716...$ (the critical value), the last KAM circle breaks — this is computed numerically and is a delicate threshold.

## The Aubry-Mather Theorem

The question is: when a KAM torus breaks, what remains?

**Theorem 14.4.2 (Aubry-Mather Theorem, 1982-1983).** For every *irrational* rotation number $\alpha \in \mathbb{R} \setminus \mathbb{Q}$, a twist map $f$ has an *Aubry-Mather set* $M_\alpha$ with:
1. $M_\alpha$ is compact and invariant under $f$.
2. $M_\alpha$ is contained in the graph of a Lipschitz function $p = p(q)$ over the circle $\mathbb{T}$.
3. The dynamics of $f|_{M_\alpha}$ is semi-conjugate to the rigid rotation $R_\alpha: \theta \mapsto \theta + \alpha$ on $\mathbb{T}$.
4. If $M_\alpha$ is a continuous curve (a Lipschitz graph over the whole circle), it is a KAM invariant torus. If it is not continuous — if it is a Cantor set — it is a *cantorus*.

**Interpretation:** Aubry-Mather sets are the remnants of KAM tori after they break down. When the perturbation is too large for a particular frequency $\alpha$ to survive as a full torus, the set $M_\alpha$ breaks into a Cantor set — but the Cantor set is still invariant, still carries the rotation dynamics, and still constrains the dynamics in a meaningful way.

The cantori are like "fractal KAM tori." They do not form barriers (there are gaps in a Cantor set through which orbits can pass), but they slow down the dynamics. In the context of plasma physics, cantori explain why charged particles take a long time to diffuse across magnetic field lines even after the invariant tori have been destroyed.

The Aubry-Mather theory provides action-minimizing orbits for all irrational rotation numbers — including those whose KAM tori have been destroyed. The Mather sets are the supports of action-minimizing measures, and they generalize KAM tori to the non-perturbative regime.

In the next section, we confront what happens in higher dimensions, where even intact KAM tori cannot fully constrain the dynamics.

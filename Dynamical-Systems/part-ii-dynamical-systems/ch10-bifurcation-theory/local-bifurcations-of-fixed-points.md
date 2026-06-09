# 10.2 Local Bifurcations of Fixed Points

The local bifurcations of equilibria are the building blocks of bifurcation theory. There are three codimension-1 cases — situations where a single "generic" condition (the eigenvalue condition) forces a specific type of qualitative change. Let's meet them one by one.

---

## 10.2.1 Saddle-Node Bifurcation

The saddle-node bifurcation is the most common: a pair of equilibria collide and annihilate each other. Or, reading the parameter in reverse, a pair of equilibria is born from nothing.

The *saddle-node bifurcation* is the creation or destruction of a pair of equilibria as a parameter varies.

**Normal Form:** $\dot{x} = \mu - x^2$.
- $\mu < 0$: no equilibria
- $\mu = 0$: one equilibrium at $x = 0$ (half-stable)
- $\mu > 0$: two equilibria $x = \pm\sqrt{\mu}$ (one stable, one unstable)

Here's the key observation: for the normal form, when $\mu > 0$ you have two equilibria (a stable one at $x = +\sqrt{\mu}$ and an unstable one at $x = -\sqrt{\mu}$). As $\mu$ decreases to 0, these two equilibria approach each other and merge. For $\mu < 0$, neither exists. The birth point is $\mu = 0$, where the single equilibrium at $x = 0$ is "half-stable" — stable from one side, unstable from the other.

**Theorem 10.2.1 (Saddle-Node Bifurcation Theorem).** Let $f: {\mathbb R}^n \times {\mathbb R} \to {\mathbb R}^n$ satisfy $f_{\mu_0}(x_0) = 0$, $Df_{\mu_0}(x_0)$ has exactly one zero eigenvalue (eigenvector $v$), and the following non-degeneracy conditions:
- (SN1) $v^T D^2 f_{\mu_0}(x_0)(v, v) \neq 0$ (quadratic non-degeneracy)
- (SN2) $v^T \partial f/\partial \mu |_{\mu_0, x_0} \neq 0$ (transversality)

Then near $(x_0, \mu_0)$: for $\mu$ on one side of $\mu_0$, two equilibria; for $\mu$ on the other, none.

The conditions (SN1) and (SN2) are the "generic" conditions that ensure the bifurcation is of saddle-node type. Condition (SN1) says the quadratic part of $f$ along the critical eigenvector is nonzero — so the zero eigenvalue is created by a quadratic, not a cubic, term. Condition (SN2) says the parameter is actually moving the equilibrium, not leaving it fixed.

**Example 10.2.2 (Fold Catastrophe).** In $x' = \mu - x^2$, the bifurcation occurs at $\mu = 0$. Equilibria trace out the parabola $\mu = x^2$ in the $(x, \mu)$-plane. The "fold" of this curve is the bifurcation locus.

---

## 10.2.2 Transcritical Bifurcation

The transcritical bifurcation is different: two equilibria exist on both sides of the bifurcation value, but they *exchange stability* at the bifurcation. Neither appears nor disappears.

**Normal Form:** $\dot{x} = \mu x - x^2$.
- The equilibrium $x = 0$ always exists but changes stability at $\mu = 0$.
- For $\mu > 0$: $x = 0$ unstable, $x = \mu$ stable.
- For $\mu < 0$: $x = 0$ stable, $x = \mu < 0$ unstable.
- The two equilibria exchange stability at $\mu = 0$.

**Occurrence:** This bifurcation occurs when an equilibrium is "forced" by the structure (e.g., in population models where $x = 0$ is always an equilibrium).

In population dynamics, $x = 0$ (extinction) is always an equilibrium — the system satisfies $f(0, \mu) = 0$ for all $\mu$. The transcritical bifurcation describes how the extinction state changes from stable (low growth rate) to unstable (high growth rate), with a nonzero equilibrium (coexistence) taking over.

---

## 10.2.3 Pitchfork Bifurcation

The pitchfork bifurcation is characteristic of systems with a symmetry $x \mapsto -x$. At the bifurcation, the symmetric equilibrium at $x = 0$ changes stability, and two new symmetric equilibria appear.

**Normal Form (Supercritical):** $\dot{x} = \mu x - x^3$.
- $\mu \leq 0$: only $x = 0$ (stable for $\mu < 0$, unstable for $\mu = 0$)
- $\mu > 0$: three equilibria; $x = 0$ unstable, $x = \pm\sqrt{\mu}$ stable

**Normal Form (Subcritical):** $\dot{x} = \mu x + x^3$.
- $\mu < 0$: three equilibria; $x = 0$ stable, $x = \pm\sqrt{-\mu}$ unstable
- $\mu \geq 0$: only $x = 0$ (unstable for $\mu > 0$)

**Occurrence:** Pitchfork bifurcations are typical when the system has a symmetry $x \mapsto -x$ (odd functions). Breaking the symmetry turns the pitchfork into a pair of saddle-nodes.

The supercritical pitchfork is the "nice" version: the new equilibria appear as stable replacements for the destabilized equilibrium at $x = 0$. The subcritical pitchfork is more dangerous: the two new equilibria are unstable, and above the bifurcation value only the unstable equilibrium remains — the system must jump discontinuously to a distant attractor.

The pitchfork bifurcation governs the buckling of elastic beams under compressive load (Euler buckling), the instability of convection rolls in Rayleigh-Bénard convection, and the spontaneous symmetry breaking in bifurcation theory more generally. Wherever there's a symmetry, the pitchfork (or a higher-codimension analogue) is lurking.

In the next section, we examine the most important bifurcation in continuous-time systems: the birth of periodic orbits.

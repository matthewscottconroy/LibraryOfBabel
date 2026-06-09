# 10.3 Hopf Bifurcation

The Hopf bifurcation is the most important local bifurcation for continuous-time systems: it is how periodic orbits are born from equilibria.

Here's the scenario. You have a system in equilibrium. As you vary a parameter $\mu$, the equilibrium stays put, but the eigenvalues of the linearization change. At some critical value $\mu_0$, a pair of complex conjugate eigenvalues crosses the imaginary axis — they go from having negative real part (stable) to having positive real part (unstable). At that moment, a periodic orbit is born.

The Hopf bifurcation is the most important local bifurcation for continuous-time systems: it is how periodic orbits are born from equilibria.

**Setup:** The linearization $Df_\mu(0)$ has eigenvalues $\alpha(\mu) \pm i\omega(\mu)$ with $\alpha(\mu_0) = 0$, $\omega(\mu_0) \neq 0$ (a pair of purely imaginary eigenvalues at $\mu = \mu_0$).

**Theorem 10.3.1 (Hopf Bifurcation Theorem).** Under the above setup, assume:
- (H1) $\alpha'(\mu_0) \neq 0$ (the eigenvalues cross the imaginary axis transversally)
- (H2) The first Lyapunov coefficient $\ell_1 \neq 0$ (non-degeneracy of the cubic terms in the normal form)

Then near $(\mu_0, 0)$: a unique family of periodic orbits bifurcates from the equilibrium.
- *Supercritical* ($\ell_1 < 0$): stable periodic orbits exist for $\mu > \mu_0$
- *Subcritical* ($\ell_1 > 0$): unstable periodic orbits exist for $\mu < \mu_0$

**Normal Form:** Near the bifurcation, in complex coordinates $z = x_1 + ix_2$:
$$\dot{z} = (\alpha(\mu) + i\omega(\mu))z + \ell_1 |z|^2 z + O(|z|^4).$$

For $\mu > \mu_0$ (supercritical): the periodic orbit has radius $r \approx \sqrt{-\alpha(\mu)/\ell_1}$ and frequency $\approx \omega(\mu_0)$.

The normal form is clean and revealing. The term $(\alpha + i\omega)z$ is the linear part — it spins $z$ around the origin (at frequency $\omega$) and grows or shrinks it (at rate $\alpha$). The cubic term $\ell_1 |z|^2 z$ is the key nonlinear correction: it produces a restoring force that balances the linear growth when $\ell_1 < 0$. The balance point is $r^2 = -\alpha/\ell_1$, and that's the radius of the bifurcating periodic orbit.

---

## Example: Van der Pol

**Example 10.3.2 (Van der Pol).** $\dot{x}_1 = x_2$, $\dot{x}_2 = -x_1 + \mu(1-x_1^2)x_2$. At $\mu = 0$, eigenvalues $\pm i$. For $\mu > 0$: unique stable limit cycle of amplitude $\approx 2$.

The Van der Pol oscillator is the canonical Hopf example. For $\mu < 0$, the origin is stable (the damping term $-|\mu|(1-x_1^2)x_2$ is stabilizing for $|x_1| < 1$). At $\mu = 0$, the damping vanishes and the eigenvalues are $\pm i$. For $\mu > 0$, the linear part is destabilizing, but the nonlinear term creates a stable limit cycle at amplitude $\approx 2$. This is a supercritical Hopf bifurcation.

---

## Computing the First Lyapunov Coefficient

The first Lyapunov coefficient $\ell_1$ is the critical number that determines whether the Hopf bifurcation is super- or subcritical. Computing it requires knowing the second and third Taylor coefficients of the vector field.

**Computing $\ell_1$:** The first Lyapunov coefficient is a specific combination of the second and third order Taylor coefficients of $f_\mu$ at the equilibrium. Kuznetsov's formula:
$$\ell_1 = \frac{1}{2\omega} \text{Re}\left[\langle p, C(q,q,\bar{q})\rangle - 2\langle p, B(q, A^{-1}B(q,\bar{q}))\rangle + \langle p, B(\bar{q}, (2i\omega I - A)^{-1}B(q,q))\rangle\right]$$
where $B, C$ are the bilinear/trilinear parts of $f$, and $p, q$ are the left/right eigenvectors of $A = Df_0(0)$.

This formula is worth knowing but not worth memorizing. The key facts are: $\ell_1$ is determined by the second and third order terms; computing it for a specific system is a straightforward but tedious calculation; and the sign of $\ell_1$ determines the type of bifurcation. For numerical work, AUTO, MATCONT, and similar software compute $\ell_1$ automatically.

In the next section, we develop normal form theory — the systematic method for reducing any vector field near a bifurcation to its simplest possible form.

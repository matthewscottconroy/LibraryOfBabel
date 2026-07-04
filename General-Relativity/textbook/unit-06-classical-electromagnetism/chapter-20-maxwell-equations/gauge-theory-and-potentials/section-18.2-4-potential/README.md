# Section 18.2: The 4-Potential and Covariant Electromagnetism

---

## Section Introduction

Special relativity (Unit VII) teaches us to think in 4-dimensional spacetime. When we combine the scalar potential $\phi$ and the vector potential $\mathbf{A}$ into a 4-vector $A^\mu = (\phi/c, \mathbf{A})$, Maxwell's equations simplify dramatically. The four equations become two. The gauge transformation becomes a single equation. The wave equation in Lorenz gauge takes the form $\Box A^\mu = \mu_0 J^\mu$ — four uncoupled wave equations.

More importantly: the covariant formulation reveals that electromagnetism is a U(1) gauge theory, and the structure of this section is exactly what recurs (with the non-Abelian gauge group SU(2) or SU(3)) in the Standard Model. Understanding U(1) gauge theory deeply is the key to understanding all of modern particle physics.

And the deepest reason to study this: the Christoffel connection $\Gamma^\rho_{\mu\nu}$ in GR plays the role of $A_\mu$ in electromagnetism. The Riemann tensor $R^\rho_{\ \sigma\mu\nu}$ plays the role of $F_{\mu\nu}$. The Einstein-Hilbert action is the GR analog of the Maxwell action $-\frac{1}{4}\int F_{\mu\nu}F^{\mu\nu}d^4x$.

---

## 18.2.1 The 4-Potential

Combine the scalar and vector potentials into the **4-potential** (or electromagnetic 4-vector):

$$A^\mu = \left(\frac{\phi}{c}, A^x, A^y, A^z\right) = \left(\frac{\phi}{c}, \mathbf{A}\right)$$

(with the Minkowski metric $\eta_{\mu\nu} = \text{diag}(-1, +1, +1, +1)$, so $A_\mu = \eta_{\mu\nu}A^\nu = (-\phi/c, \mathbf{A})$.)

The Faraday tensor is the **antisymmetric derivative** of $A_\mu$:

$$F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$$

In components:

$$F_{0i} = \partial_0 A_i - \partial_i A_0 = \frac{1}{c}\partial_t A^i + \frac{1}{c}\partial_i\phi = -\frac{E^i}{c}$$

$$F_{ij} = \partial_i A_j - \partial_j A_i = -\varepsilon_{ijk}B^k$$

So:
$$F_{\mu\nu} = \begin{pmatrix} 0 & -E_x/c & -E_y/c & -E_z/c \\ E_x/c & 0 & -B_z & B_y \\ E_y/c & B_z & 0 & -B_x \\ E_z/c & -B_y & B_x & 0 \end{pmatrix}$$

**Gauge transformation**: $A_\mu \to A_\mu + \partial_\mu\chi$ (where $\chi$ is any smooth scalar function). This is precisely the covariant form of $\mathbf{A} \to \mathbf{A} + \nabla\chi$, $\phi \to \phi - \partial_t\chi$.

Check: $F_{\mu\nu} \to \partial_\mu(A_\nu + \partial_\nu\chi) - \partial_\nu(A_\mu + \partial_\mu\chi) = F_{\mu\nu} + \partial_\mu\partial_\nu\chi - \partial_\nu\partial_\mu\chi = F_{\mu\nu}$ (since partial derivatives commute). ✓

---

## 18.2.2 Maxwell's Equations in Covariant Form

**First pair** ($dF = 0$, equivalently $\partial_{[\alpha}F_{\beta\gamma]} = 0$): Automatic, since $F = dA$ and $d^2 = 0$.

**Second pair** ($\partial_\mu F^{\mu\nu} = \mu_0 J^\nu$): The dynamical content of Maxwell's equations.

$$\partial_\mu F^{\mu\nu} = \mu_0 J^\nu$$

where $J^\mu = (c\rho, \mathbf{J})$ is the 4-current. Expanding:
- $\nu = 0$: $\partial_i F^{i0} = \mu_0 J^0 = \mu_0 c\rho$. Since $F^{i0} = E^i/c$ (raising indices with $\eta^{\mu\nu}$): $\nabla\cdot\mathbf{E}/c = \mu_0 c\rho = \rho/(\varepsilon_0 c)$, i.e., $\nabla\cdot\mathbf{E} = \rho/\varepsilon_0$. ✓ (Gauss's law)
- $\nu = i$: gives $\nabla\times\mathbf{B} - (1/c^2)\partial_t\mathbf{E} = \mu_0\mathbf{J}$. ✓ (Ampère-Maxwell)

Charge conservation follows automatically: $\partial_\nu(\partial_\mu F^{\mu\nu}) = 0$ (since $F^{\mu\nu}$ is antisymmetric), so $\partial_\nu J^\nu = 0$.

**The 4-current** $J^\mu = (c\rho, \mathbf{J})$: a Lorentz 4-vector. Charge density and current density mix under Lorentz boosts — just as electric and magnetic fields mix.

---

## 18.2.3 The Lorenz Gauge and the Wave Equation

The Lorenz gauge condition is:

$$\partial_\mu A^\mu = 0$$

i.e., $-\frac{1}{c^2}\partial_t\phi + \nabla\cdot\mathbf{A} = 0$ (in components with $\eta_{00} = -1$).

In Lorenz gauge, the Maxwell equations $\partial_\mu F^{\mu\nu} = \mu_0 J^\nu$ become:

$$\partial_\mu(\partial^\mu A^\nu - \partial^\nu A^\mu) = \Box A^\nu - \partial^\nu(\partial_\mu A^\mu) = \Box A^\nu = \mu_0 J^\nu$$

**Each component** of $A^\mu$ satisfies a wave equation:

$$\Box A^\nu = \mu_0 J^\nu, \qquad \Box = \partial_\mu\partial^\mu = -\frac{1}{c^2}\partial_{tt} + \nabla^2$$

This is the key result: in Lorenz gauge, the four Maxwell equations reduce to four decoupled wave equations for the four components of $A^\mu$. This is the form used in radiation theory and quantum electrodynamics.

**Green's function solution**: The retarded solution is (Section 11.4):

$$A^\mu(\mathbf{r}, t) = \frac{\mu_0}{4\pi}\int \frac{J^\mu(\mathbf{r}', t - |\mathbf{r}-\mathbf{r}'|/c)}{|\mathbf{r}-\mathbf{r}'|}d^3r'$$

Fields at $(\mathbf{r}, t)$ are determined by the source at retarded time $t_{\rm ret} = t - |\mathbf{r}-\mathbf{r}'|/c$ — the signal travels at speed $c$. This causality is built in by the retarded Green's function.

---

## 18.2.4 The Electromagnetic Action

The electromagnetic field has an **action** (in flat spacetime):

$$S[A] = \int\left(-\frac{1}{4\mu_0}F_{\mu\nu}F^{\mu\nu} + A_\mu J^\mu\right)d^4x$$

**Varying with respect to $A_\nu$**: The Euler-Lagrange equations for this action give exactly $\partial_\mu F^{\mu\nu} = \mu_0 J^\nu$ — the Maxwell equations. *Derivation*:

$$\frac{\partial\mathcal{L}}{\partial A_\nu} = J^\nu, \qquad \frac{\partial\mathcal{L}}{\partial(\partial_\mu A_\nu)} = -\frac{1}{\mu_0}F^{\mu\nu}$$

E-L equations: $J^\nu - \partial_\mu(-F^{\mu\nu}/\mu_0) = 0$, i.e., $\partial_\mu F^{\mu\nu}/\mu_0 = J^\nu$. ✓

**Gauge invariance of the action**: Under $A_\mu \to A_\mu + \partial_\mu\chi$:
- $F_{\mu\nu}$ is unchanged, so $-\frac{1}{4\mu_0}F_{\mu\nu}F^{\mu\nu}$ is unchanged.
- The coupling term $A_\mu J^\mu \to A_\mu J^\mu + \partial_\mu\chi J^\mu = A_\mu J^\mu + \partial_\mu(\chi J^\mu) - \chi\partial_\mu J^\mu$.
- For conserved current ($\partial_\mu J^\mu = 0$): the extra term is a total derivative, which doesn't affect the equations of motion. ✓

The action is gauge-invariant (up to boundary terms) when $J^\mu$ is conserved. This is Noether's second theorem: gauge invariance implies charge conservation.

**The Maxwell action and GR**: The Einstein-Hilbert action $S_{\rm EH} = \frac{1}{16\pi G}\int R\sqrt{-g}\,d^4x$ is the gravitational analog of the Maxwell action. The Ricci scalar $R$ plays the role of $-\frac{1}{4}F_{\mu\nu}F^{\mu\nu}$ — both are the squared curvature of the respective connection. But the Maxwell action is quadratic in $F$ (linear equations of motion), while the Einstein-Hilbert action is linear in $R$ (which is quadratic in $\Gamma$), giving nonlinear equations. This is why GR is harder than Maxwell.

---

## 18.2.5 Electromagnetism as U(1) Gauge Theory

The deepest way to understand electromagnetism: it is a **U(1) gauge theory** — a theory invariant under local U(1) transformations.

**Global U(1) symmetry**: The Lagrangian for a complex scalar field $\phi$ coupled to electromagnetism is invariant under the global phase transformation $\phi \to e^{i\alpha}\phi$ (for constant $\alpha$). By Noether's theorem, this gives a conserved current $J^\mu \propto \phi^*\partial^\mu\phi - \phi\partial^\mu\phi^*$ — the electric current.

**Gauging**: Make $\alpha$ position-dependent ($\alpha = \alpha(x)$). The derivative $\partial_\mu\phi$ is no longer covariant: $\partial_\mu(e^{i\alpha}\phi) = e^{i\alpha}(\partial_\mu + i\partial_\mu\alpha)\phi$. To maintain covariance, introduce a connection $A_\mu$ and replace $\partial_\mu$ with the **covariant derivative**:

$$D_\mu = \partial_\mu - iqA_\mu$$

Under $\phi \to e^{i\alpha}\phi$, $A_\mu \to A_\mu + (1/q)\partial_\mu\alpha$: then $D_\mu\phi \to e^{i\alpha}D_\mu\phi$ (covariant). The field strength $F_{\mu\nu} = (i/q)[D_\mu, D_\nu] = \partial_\mu A_\nu - \partial_\nu A_\mu$ is gauge invariant. The locally gauge-invariant Lagrangian is:

$$\mathcal{L} = -|D_\mu\phi|^2 - m^2|\phi|^2 - \frac{1}{4\mu_0}F_{\mu\nu}F^{\mu\nu}$$

This is **scalar QED** at the classical level. The entire structure — covariant derivative, connection, field strength, Yang-Mills action — follows from demanding local U(1) invariance.

**GR as gauge theory**: Replacing U(1) by the local Lorentz group SO(3,1):
- The connection becomes $\omega^\mu_{\ \nu}$ (the spin connection) — analog of $A_\mu$.
- The curvature becomes $R^\mu_{\ \nu\rho\sigma}$ (the Riemann tensor) — analog of $F_{\mu\nu}$.
- The Einstein-Hilbert action $\int R\sqrt{-g}\,d^4x$ — analog of $\int F_{\mu\nu}F^{\mu\nu}d^4x$.

The difference: the Einstein-Hilbert action is linear in curvature (not quadratic), giving the "dilaton" structure of GR. But the formal parallel is exact: GR is (in the tetrad formulation) a gauge theory of local Lorentz invariance.

---

## References

- Yang, C.N. and Mills, R.L. (1954). "Conservation of isotopic spin and isotopic gauge invariance." *Physical Review*, 96, 191–195. [The paper that generalized U(1) gauge theory to non-Abelian gauge groups (SU(2)). The founding paper of Yang-Mills theory, which underlies the entire Standard Model of particle physics.]
- Weyl, H. (1929). "Electron and gravitation." *Zeitschrift für Physik*, 56, 330–352. [First uses the term "gauge theory" (Eichinvarianz); introduces the modern U(1) gauge formulation of electromagnetism. Weyl's original 1918 paper proposed a different (incorrect) gauge theory to unify EM and GR, and this 1929 paper is the corrected version.]
- Utiyama, R. (1956). "Invariant theoretical interpretation of interaction." *Physical Review*, 101, 1597–1607. [The generalization of gauge theory to arbitrary Lie groups, providing the framework for GR as a gauge theory of the local Lorentz group.]
- Jackson, J.D. (1999). *Classical Electrodynamics*, 3rd ed. Wiley. [Chapter 12: covariant formulation of Maxwell's equations. The standard reference.]
- Nakahara, M. (2003). *Geometry, Topology and Physics*, 2nd ed. Institute of Physics Publishing. [Chapters 9–10: fiber bundles, connections, and gauge theories. The definitive mathematical treatment.]

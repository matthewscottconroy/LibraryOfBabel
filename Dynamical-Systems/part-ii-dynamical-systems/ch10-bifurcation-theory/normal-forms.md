# 10.4 Normal Forms

The saddle-node, transcritical, pitchfork, and Hopf bifurcations each had a "normal form" — a simple polynomial system that captures the essential dynamics near the bifurcation. But why can we always reduce to a polynomial? What exactly can be simplified, and what can't?

Normal form theory answers these questions systematically. The key theorem, due to Poincaré and Dulac, says: near an equilibrium, you can eliminate most of the nonlinear terms by a smooth coordinate change. The terms you can't eliminate are called *resonant*, and they're determined by algebraic conditions on the eigenvalues.

**Goal:** Reduce a system near a bifurcation to its simplest possible form by a coordinate change.

**Definition 10.4.1.** The *normal form* of a vector field $f$ at an equilibrium is the simplest polynomial vector field to which $f$ can be $C^k$-conjugated near the equilibrium.

---

## Poincaré-Dulac Normal Form

**Theorem 10.4.2 (Poincaré-Dulac).** Let $f(x) = Ax + \text{higher order}$ where $A$ has eigenvalues $\lambda_1, \ldots, \lambda_n$. A monomial $x^\alpha e_i$ (where $\alpha \in {\mathbb N}^n$, $|\alpha| \geq 2$) can be eliminated from the normal form *unless* there is a *resonance*:
$$\lambda_i = \sum_{j=1}^n \alpha_j \lambda_j \quad (\alpha = (\alpha_1, \ldots, \alpha_n), |\alpha| = \sum \alpha_j \geq 2).$$

The normal form contains only resonant monomials.

The resonance condition is the key. A monomial $x^\alpha e_i$ can be eliminated if and only if the eigenvalue $\lambda_i$ is not a linear combination (with nonneg integer coefficients) of the other eigenvalues in the pattern $\alpha$. If there is such a resonance, the monomial cannot be eliminated — it's "locked in" by the eigenvalue arithmetic.

**Example 10.4.3.** For eigenvalues $\lambda_1 = 0$, $\lambda_2 = -1$: resonances occur when $0 = 0 \cdot k_1 + (-1) \cdot k_2$, i.e., $k_2 = 0$. So the normal form in the $x_1$ direction contains arbitrary powers of $x_1$: $\dot{x}_1 = a_2 x_1^2 + a_3 x_1^3 + \cdots$

This is the saddle-node case: the center direction (eigenvalue 0) has resonances that allow all powers of $x_1$, so the normal form in that direction can be a general polynomial in $x_1$. The full normal form then tells you exactly what the dynamics near the bifurcation look like.

**Remark 10.4.4.** The Poincaré-Dulac normal form is formal (a formal power series). Convergence is a subtle issue (Siegel vs. Brjuno conditions).

The distinction between formal and convergent is real. Poincaré showed that for generic eigenvalue ratios (Siegel's condition), the formal normal form actually converges. Brjuno weakened the condition. But for "generic" eigenvalues (e.g., at bifurcation, where eigenvalues are on the imaginary axis), the formal series may not converge. For practical applications, truncating at a finite order (which does converge, as a polynomial approximation) gives the relevant dynamics.

---

## Versal Deformations

**Definition 10.4.5.** A *$k$-parameter deformation* of a vector field $f_0$ is a family $f_\alpha$ ($\alpha \in {\mathbb R}^k$) with $f_0 = f_{|_{\alpha=0}}$. A deformation is *versal* if every other deformation of $f_0$ factors through it (via a reparametrization).

**Definition 10.4.6.** The *codimension* of a bifurcation is the minimum number of parameters needed in a versal unfolding.

**Example 10.4.7.** The saddle-node is codimension 1 (one parameter needed). The cusp bifurcation ($\dot{x} = \mu_1 + \mu_2 x - x^3$) is codimension 2. Elementary catastrophes in Thom's classification are codimension $\leq 5$.

The codimension is the "number of parameters you need to generically encounter this bifurcation." A codimension-1 bifurcation occurs generically in a one-parameter family — you'll typically encounter it. A codimension-2 bifurcation requires tuning two parameters — you'll encounter it on curves in parameter space, not at isolated points.

This classification gives a hierarchy of bifurcations: codimension-1 (the three in Section 10.2 plus Hopf) are the most common; codimension-2 (cusp, Bogdanov-Takens, etc.) require more careful parameter tuning; and so on. In applications, you typically worry most about codimension-1 bifurcations, since those are the ones you'll reliably encounter as a single parameter varies.

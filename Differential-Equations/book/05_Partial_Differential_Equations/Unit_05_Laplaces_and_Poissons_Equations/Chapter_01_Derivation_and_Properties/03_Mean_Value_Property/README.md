# The Mean Value Property of Harmonic Functions

The mean value property is one of the most elegant and powerful theorems in analysis. It states that the value of a harmonic function at any point equals its average over any ball centered at that point. This single property encodes the qualitative content of Laplace's equation — that $u$ has no interior extrema, that it represents an equilibrium, that it smooths — and can in fact be taken as the definition of harmonicity.

## Statement

**Theorem (Mean Value Property).** Let $u$ be harmonic in $\Omega$ and let $B_r(\mathbf{x}_0) \subset\subset \Omega$ (a ball compactly contained in $\Omega$). Then:

**Spherical mean:** $u(\mathbf{x}_0) = \frac{1}{|\partial B_r|}\int_{\partial B_r(\mathbf{x}_0)}u\,dS = \frac{1}{n\omega_n r^{n-1}}\int_{\partial B_r(\mathbf{x}_0)}u\,dS.$

**Ball mean:** $u(\mathbf{x}_0) = \frac{1}{|B_r|}\int_{B_r(\mathbf{x}_0)}u\,d\mathbf{x} = \frac{n}{\omega_n r^n}\int_{B_r(\mathbf{x}_0)}u\,d\mathbf{x}.$

(The ball mean follows from the spherical mean by integrating over $r$.)

## Proof of the Spherical Mean Property

Define $\phi(r) = \frac{1}{|\partial B_r(\mathbf{x}_0)|}\int_{\partial B_r(\mathbf{x}_0)}u(\mathbf{x})\,dS(\mathbf{x})$, the spherical mean of $u$ at radius $r$. Substituting $\mathbf{x} = \mathbf{x}_0 + r\boldsymbol{\omega}$ (with $|\boldsymbol{\omega}|=1$):

$$\phi(r) = \frac{1}{n\omega_n}\int_{|\boldsymbol{\omega}|=1}u(\mathbf{x}_0+r\boldsymbol{\omega})\,dS(\boldsymbol{\omega}).$$

Differentiating:

$$\phi'(r) = \frac{1}{n\omega_n}\int_{|\boldsymbol{\omega}|=1}\nabla u(\mathbf{x}_0+r\boldsymbol{\omega})\cdot\boldsymbol{\omega}\,dS(\boldsymbol{\omega}) = \frac{1}{n\omega_n}\int_{|\boldsymbol{\omega}|=1}\frac{\partial u}{\partial r}\,dS.$$

By the divergence theorem (applied to the ball $B_r(\mathbf{x}_0)$):

$$\int_{\partial B_r(\mathbf{x}_0)}\frac{\partial u}{\partial\nu}\,dS = \int_{B_r(\mathbf{x}_0)}\Delta u\,d\mathbf{x} = 0$$

(since $u$ is harmonic). Therefore $\phi'(r) = 0$, so $\phi$ is constant in $r$. Taking $r\to 0$: $\phi(r) \to u(\mathbf{x}_0)$ by continuity. So $\phi(r) = u(\mathbf{x}_0)$ for all $r$ with $B_r(\mathbf{x}_0) \subset \Omega$.

## Converse: The Mean Value Property Characterizes Harmonicity

**Theorem (Converse).** If $u \in C(\Omega)$ satisfies the spherical mean property at every point of $\Omega$, then $u$ is harmonic.

This means the mean value property can be taken as an alternative definition of harmonicity, one that does not require differentiability in the classical sense. This "definition" extends naturally to defining harmonic functions in non-smooth settings and is the basis for the probabilistic theory (harmonic functions are characterized by the martingale property of Brownian motion: $u(\mathbf{X}(t))$ is a martingale).

**Proof sketch.** Suppose $u$ satisfies the mean value property but $\Delta u(\mathbf{x}_0) \neq 0$, say $\Delta u(\mathbf{x}_0) > 0$. By continuity of $\Delta u$, there is a ball $B_\varepsilon(\mathbf{x}_0)$ on which $\Delta u > 0$. The divergence theorem gives $\phi'(r) = \frac{r}{n}\Delta u(\mathbf{x}_0+O(r)) > 0$ for small $r$, contradicting $\phi = \text{const}$.

## Physical Interpretation

The mean value property says: in a region with no sources ($\Delta u = 0$), the temperature (or potential) at any point equals the average of the temperature over any surrounding sphere. This is the mathematical expression of the physical intuition that, at equilibrium, every point is in balance with its surroundings — no net heat flow, no net charge accumulation.

Equivalently: if you compute the average temperature over any sphere in a heat-conducting body at steady state (no sources), you get exactly the temperature at the center of that sphere. Any deviation from the mean would cause heat to flow, breaking the equilibrium.

## The Harnack Inequality

A consequence of the mean value property is Harnack's inequality:

**Theorem (Harnack).** Let $u \geq 0$ be harmonic in $B_{2r}(\mathbf{x}_0)$. Then:

$$\frac{1}{C}\,u(\mathbf{x}_0) \leq u(\mathbf{y}) \leq C\,u(\mathbf{x}_0)$$

for all $\mathbf{y} \in B_r(\mathbf{x}_0)$, where $C = C(n)$ depends only on dimension.

**Proof sketch.** For any $\mathbf{y} \in B_r(\mathbf{x}_0)$, the ball $B_r(\mathbf{y}) \subset B_{2r}(\mathbf{x}_0)$. By the mean value property:

$$u(\mathbf{y}) = \frac{1}{|B_r|}\int_{B_r(\mathbf{y})}u\,d\mathbf{x} \geq \frac{|B_r(\mathbf{y})\cap B_r(\mathbf{x}_0)|}{|B_r|}\inf_{B_r(\mathbf{x}_0)}u.$$

A geometric argument shows $|B_r(\mathbf{y})\cap B_r(\mathbf{x}_0)| \geq c|B_r|$ for some $c = c(n) > 0$. Applying the mean value property at $\mathbf{x}_0$:

$$u(\mathbf{x}_0) = \frac{1}{|B_r|}\int_{B_r(\mathbf{x}_0)}u\,d\mathbf{x} \leq \sup_{B_r(\mathbf{x}_0)}u.$$

Combining these estimates gives Harnack's inequality.

## Gradient Estimate

The mean value property also gives sharp estimates on the gradient of a harmonic function:

**Theorem.** If $u$ is harmonic in $B_r(\mathbf{x}_0)$, then:

$$|\nabla u(\mathbf{x}_0)| \leq \frac{n}{r}\|u\|_{L^\infty(\partial B_r)}.$$

**Proof.** Differentiate the mean value formula: $\nabla u(\mathbf{x}_0) = \nabla_{\mathbf{x}_0}\frac{1}{|\partial B_r|}\int_{\partial B_r(\mathbf{x}_0)}u\,dS$. The differentiation of the average (moving the center) is bounded by $n/r$ times the sup of $u$.

These gradient estimates are used to prove compactness theorems (families of uniformly bounded harmonic functions are equicontinuous — a consequence of Ascoli's theorem) and to establish convergence of sequences of harmonic functions.

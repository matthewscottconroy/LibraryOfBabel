# Distributions and Examples

With the space of test functions in hand, the definition of a distribution is clean: a distribution is a continuous linear functional on $\mathcal{D}$. This section gives the formal definition, establishes the continuity condition in usable form, and develops a rich catalog of examples illustrating the diversity of the class of distributions.

## The Definition

**Definition.** A **distribution** on $\mathbb{R}^n$ is a linear functional $T: \mathcal{D}(\mathbb{R}^n) \to \mathbb{R}$ that is continuous with respect to the topology of $\mathcal{D}$: for every compact set $K \subset \mathbb{R}^n$, there exist constants $C \geq 0$ and $m \in \mathbb{N}_0$ such that

$$|T(\phi)| \leq C \sum_{|\alpha| \leq m} \sup_x |D^\alpha\phi(x)|, \quad \text{for all } \phi \in C_c^\infty(K).$$

The **order** of $T$ is the smallest $m$ for which such a $C$ exists (for each $K$). If the same $m$ works for all compact $K$, the distribution has **finite order**. Distributions of order 0 are measures; distributions of order $m$ are (in a precise sense) $m$-th derivatives of continuous functions.

The space of all distributions on $\mathbb{R}^n$ is denoted $\mathcal{D}'(\mathbb{R}^n)$. It is a vector space under the operations:

$$(aS + bT)(\phi) = a S(\phi) + b T(\phi).$$

## Regular Distributions

Every locally integrable function $f \in L^1_{\text{loc}}(\mathbb{R}^n)$ defines a distribution by integration:

$$T_f(\phi) = \int_{\mathbb{R}^n} f(x)\phi(x) \, dx.$$

Continuity: $|T_f(\phi)| \leq \|f\|_{L^1(K)} \cdot \|\phi\|_{L^\infty}$ for $\phi \in \mathcal{D}_K$, so $T_f$ is of order 0. Distributions of this form are called **regular**. The map $f \mapsto T_f$ is injective (if $\int f\phi = 0$ for all $\phi \in \mathcal{D}$, then $f = 0$ a.e.), allowing us to identify $f$ with $T_f$ and write $\langle f, \phi \rangle = \int f\phi$.

## The Dirac Delta

$\langle \delta, \phi \rangle = \phi(0)$. More generally, for $a \in \mathbb{R}^n$: $\langle \delta_a, \phi \rangle = \phi(a)$. The estimate $|\phi(a)| \leq \|\phi\|_{L^\infty}$ shows $\delta_a$ is of order 0 and is a measure (the unit point mass at $a$). 

**Claim:** $\delta$ is not a regular distribution. Suppose $\delta = T_f$ for some $f \in L^1_{\text{loc}}$. For any $r > 0$, choose $\phi_r \in \mathcal{D}$ with $\phi_r \equiv 1$ on $B(0, r)$ and $\phi_r \geq 0$. Then $1 = \phi_r(0) = \langle \delta, \phi_r \rangle = \int f \phi_r \, dx$. But as $r \to 0$, $\phi_r \to 0$ pointwise everywhere except $\{0\}$, so $\int f\phi_r \to 0$ by dominated convergence. Contradiction. $\square$

## Derivatives of the Dirac Delta

For any multi-index $\alpha$: $\langle D^\alpha\delta, \phi \rangle = (-1)^{|\alpha|}(D^\alpha\phi)(0)$. This is a distribution of order $|\alpha|$.

**Example:** $\langle \delta', \phi \rangle = -\phi'(0)$. This can be approximated by $\langle f_\varepsilon, \phi \rangle = \int f_\varepsilon(x)\phi(x) \, dx$ where $f_\varepsilon(x) = (\phi_\varepsilon(x) - \phi_\varepsilon(-x))/\varepsilon$ (a finite difference of bump functions) as $\varepsilon \to 0$.

## The Heaviside Function

$H: \mathbb{R} \to \mathbb{R}$, $H(x) = \mathbf{1}_{[0,\infty)}(x)$. As a locally integrable function, $H$ defines a regular distribution:

$$\langle H, \phi \rangle = \int_0^\infty \phi(x) \, dx.$$

Its distributional derivative is $\delta$:

$$\langle H', \phi \rangle = -\langle H, \phi' \rangle = -\int_0^\infty \phi'(x) \, dx = \phi(0) = \langle \delta, \phi \rangle.$$

## The Principal Value Distribution

The function $1/x$ is not locally integrable (it has a non-integrable singularity at $x = 0$), so it does not define a regular distribution directly. However, the **principal value** does:

$$\left\langle \text{p.v.}\frac{1}{x}, \phi \right\rangle = \lim_{\varepsilon \to 0^+} \int_{|x| > \varepsilon} \frac{\phi(x)}{x} \, dx.$$

This limit exists because $\phi(x)/x - \phi(-x)/(-x)$ is bounded near $x = 0$ (by the mean value theorem), so the two contributions near $x = 0$ cancel. The principal value distribution is of order 1.

**Relation to $\log$:** $\text{p.v.}(1/x) = (d/dx)\log|x|$ in the distributional sense, where $\log|x|$ is a locally integrable function defining a regular distribution.

## Distributions from Measures

Any locally finite Borel measure $\mu$ on $\mathbb{R}^n$ defines a distribution of order 0:

$$\langle T_\mu, \phi \rangle = \int \phi \, d\mu.$$

The Dirac delta $\delta_a$ corresponds to the point measure $\mu = \delta_{x=a}$ (unit mass at $a$). A surface measure $d\sigma_S$ on a smooth hypersurface $S \subset \mathbb{R}^n$ (Hausdorff measure restricted to $S$) gives a distribution supported on $S$.

## The Dirac Comb

The **Dirac comb** (or Shah distribution) is:

$$\text{III}(x) = \sum_{n=-\infty}^\infty \delta(x - n).$$

This sum converges in $\mathcal{D}'(\mathbb{R})$: for any $\phi \in \mathcal{D}$ supported in a compact set, only finitely many terms are nonzero. The Dirac comb models ideal periodic sampling in signal processing. Its Fourier transform is again a Dirac comb (with frequency $1$): $\widehat{\text{III}} = \text{III}$, a statement of the Poisson summation formula.

## Order and Structure

**Theorem (Structure theorem for distributions).** Every distribution of order $m$ on a compact set $K$ is a finite sum of derivatives (of order $\leq m$) of continuous functions:

$$T = \sum_{|\alpha| \leq m} D^\alpha f_\alpha, \quad f_\alpha \in C^0.$$

Every distribution on $\mathbb{R}^n$ is locally a finite-order derivative of a continuous function. This structure theorem shows that, despite their seemingly abstract definition, distributions are not more exotic than iterated derivatives of continuous functions.

## Convergence of Distributions

The **distributional limit** $T_j \to T$ means $\langle T_j, \phi \rangle \to \langle T, \phi \rangle$ for all $\phi \in \mathcal{D}$. This weak-* convergence is the natural notion of convergence for distributions. Examples:
- $\delta_\varepsilon \to \delta$ (any approximation to the identity converges to $\delta$).
- $n \sin(nx) \to 0$ distributionally but not pointwise.
- $\sum_{k=1}^N e^{ikx} \to \pi\delta(x)$ distributionally (partial sums of the Fourier series of the delta function on $[0, 2\pi]$).

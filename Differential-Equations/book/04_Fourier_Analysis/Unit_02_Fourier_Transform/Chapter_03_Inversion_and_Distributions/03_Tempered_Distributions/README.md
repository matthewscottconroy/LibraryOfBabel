# Tempered Distributions

The Dirac delta "function" $\delta(x)$ appears throughout physics and engineering: it models a point mass, an instantaneous impulse, or an idealized point source. Yet $\delta$ cannot be a function in any classical sense — a function that is zero everywhere except at one point and whose integral is one does not exist in standard analysis. The framework of **distributions** (generalized functions) provides the rigorous setting in which $\delta$ exists, has a Fourier transform, and behaves exactly as physical intuition demands.

## Test Functions and the Schwartz Space

A **test function** is a smooth function against which a distribution is tested. For Fourier analysis, the appropriate space of test functions is the **Schwartz space** $\mathcal{S}(\mathbb{R})$: the set of infinitely differentiable functions $\phi : \mathbb{R} \to \mathbb{C}$ such that
$$\sup_{x \in \mathbb{R}} |x^k \phi^{(j)}(x)| < \infty \quad \text{for all } k, j \geq 0.$$
Functions in $\mathcal{S}$ decay faster than any power of $|x|$ as $|x| \to \infty$, and so do all their derivatives. Examples include $e^{-\pi x^2}$, $p(x)e^{-\pi x^2}$ for any polynomial $p$, and smooth bump functions.

The Fourier transform maps $\mathcal{S}(\mathbb{R})$ to itself bijectively, which is the key reason $\mathcal{S}$ is the right domain for distribution-theoretic Fourier analysis. (By contrast, the space of compactly supported smooth functions $C_c^\infty$ is not preserved by $\mathcal{F}$.)

## Tempered Distributions

A **tempered distribution** $T$ is a continuous linear functional on $\mathcal{S}(\mathbb{R})$: a map $T : \mathcal{S}(\mathbb{R}) \to \mathbb{C}$ satisfying linearity ($T(\alpha\phi + \beta\psi) = \alpha T(\phi) + \beta T(\psi)$) and continuity in the Schwartz topology.

We write $\langle T, \phi\rangle$ or $T(\phi)$ for the action of $T$ on a test function $\phi$. The space of tempered distributions is denoted $\mathcal{S}'(\mathbb{R})$.

**Examples of tempered distributions:**

1. **Regular distributions from $L^p$ functions.** Any function $f \in L^p(\mathbb{R})$ for $1 \leq p \leq \infty$ defines a tempered distribution $T_f$ via $\langle T_f, \phi\rangle = \int_{-\infty}^\infty f(x)\phi(x)\,dx$. We identify $f$ with $T_f$.

2. **The Dirac delta.** $\langle\delta, \phi\rangle = \phi(0)$. This is not a regular distribution (no $L^p$ function gives the point evaluation functional).

3. **Derivatives of delta.** $\langle\delta^{(n)}, \phi\rangle = (-1)^n\phi^{(n)}(0)$.

4. **Principal value.** $\langle\text{p.v.}(1/x), \phi\rangle = \lim_{\epsilon\to 0^+}\int_{|x|>\epsilon}\frac{\phi(x)}{x}\,dx$.

5. **Polynomials and exponentials.** The constant function $1$ and the function $x^n$ are in $\mathcal{S}'$ (as functionals $\phi \mapsto \int x^n\phi(x)\,dx$). The function $e^{2\pi i\nu_0 x}$ is in $\mathcal{S}'$.

## The Fourier Transform of a Tempered Distribution

**Definition.** The **Fourier transform** of a tempered distribution $T \in \mathcal{S}'(\mathbb{R})$ is the tempered distribution $\hat{T}$ defined by
$$\langle\hat{T}, \phi\rangle = \langle T, \hat{\phi}\rangle \quad \text{for all } \phi \in \mathcal{S}(\mathbb{R}).$$

This definition is motivated by the identity: for $f, g \in \mathcal{S}$,
$$\int \hat{f}(\xi)g(\xi)\,d\xi = \int f(x)\hat{g}(x)\,dx,$$
which follows from Fubini's theorem. Rewriting in distributional notation: $\langle\hat{T}_f, g\rangle = \langle T_f, \hat{g}\rangle$. The distributional definition simply takes this as the definition when $f$ is replaced by a general distribution $T$.

One verifies that this is well-defined: if $\phi \in \mathcal{S}$, then $\hat{\phi} \in \mathcal{S}$, so $\langle T, \hat{\phi}\rangle$ makes sense. The map $T \mapsto \hat{T}$ is a continuous bijection on $\mathcal{S}'$.

## Fundamental Examples

**The Dirac delta.** $\langle\hat{\delta}, \phi\rangle = \langle\delta, \hat{\phi}\rangle = \hat{\phi}(0) = \int_{-\infty}^\infty \phi(x)\,dx = \langle 1, \phi\rangle$.
Therefore $\hat{\delta} = 1$: the Fourier transform of the Dirac delta is the constant function $1$.

This is consistent with the integral formula: $\hat{\delta}(\xi) = \int_{-\infty}^\infty \delta(x)e^{-2\pi i\xi x}\,dx = e^{-2\pi i\xi\cdot 0} = 1$.

**The constant function $1$.** By duality and the self-inverse nature of $\mathcal{F}$:
$\hat{1} = \delta$. That is, $\mathcal{F}[1] = \delta$ in $\mathcal{S}'$.

This says $\int_{-\infty}^\infty 1\cdot e^{-2\pi i\xi x}\,dx = \delta(\xi)$ (interpreted distributionally). The "integral" does not converge classically, but as a distribution it gives the delta function. This is consistent with the fact that the constant function $1$ has all its energy at frequency zero.

**Complex exponentials.** For fixed $\nu_0 \in \mathbb{R}$:
$$\mathcal{F}[e^{2\pi i\nu_0 x}] = \delta(\xi - \nu_0).$$
A pure sinusoid at frequency $\nu_0$ has a delta function spectrum at $\pm\nu_0$.

**Proof.** $\langle\mathcal{F}[e^{2\pi i\nu_0 x}], \phi\rangle = \langle e^{2\pi i\nu_0 x}, \hat{\phi}\rangle = \int e^{2\pi i\nu_0 x}\hat{\phi}(x)\,dx = \mathcal{F}^{-1}[\hat{\phi}](\nu_0) = \phi(\nu_0) = \langle\delta(\cdot - \nu_0), \phi\rangle$.

**Trigonometric functions.** Since $\cos(2\pi\nu_0 x) = (e^{2\pi i\nu_0 x} + e^{-2\pi i\nu_0 x})/2$:
$$\mathcal{F}[\cos(2\pi\nu_0 x)] = \frac{1}{2}[\delta(\xi - \nu_0) + \delta(\xi + \nu_0)].$$

## Differentiation of Distributions

Distributions can be differentiated to any order. The derivative of $T \in \mathcal{S}'$ is defined by
$$\langle T', \phi\rangle = -\langle T, \phi'\rangle.$$
This extends the classical integration-by-parts formula: if $T = T_f$ for smooth $f$, $\int f'(x)\phi(x)\,dx = -\int f(x)\phi'(x)\,dx$.

The Fourier transform of $T'$: $\langle\widehat{T'}, \phi\rangle = \langle T', \hat{\phi}\rangle = -\langle T, (\hat{\phi})'\rangle = -\langle T, \mathcal{F}[(-2\pi ix)\phi]\rangle = -\langle\hat{T}, (-2\pi ix)\phi\rangle = \langle 2\pi i\xi\hat{T}, \phi\rangle$.
So $\widehat{T'} = 2\pi i\xi\cdot\hat{T}$, exactly as for classical functions.

## The Heaviside Function and Its Transform

The **Heaviside step function** $H(x) = \mathbf{1}_{(0,\infty)}(x)$ is in $L^\infty \subset \mathcal{S}'$ but not in $L^1$ or $L^2$. Its distributional derivative is $H' = \delta$ (check: $\langle H', \phi\rangle = -\langle H, \phi'\rangle = -\int_0^\infty \phi'(x)\,dx = \phi(0) = \langle\delta, \phi\rangle$).

From $\widehat{H'} = 2\pi i\xi\hat{H}$ and $\widehat{\delta} = 1$:
$$2\pi i\xi\hat{H}(\xi) = 1 \implies \hat{H}(\xi) = \frac{1}{2\pi i\xi} + C\delta(\xi)$$
for some constant $C$. The correct distributional Fourier transform of $H$ is:
$$\hat{H}(\xi) = \frac{1}{2\pi i\xi} + \frac{1}{2}\delta(\xi),$$
where $1/(2\pi i\xi)$ means the principal value distribution $\text{p.v.}(1/(2\pi i\xi))$. The delta term reflects the fact that $H$ has a nonzero mean (it equals $1/2$ on average in a symmetric sense).

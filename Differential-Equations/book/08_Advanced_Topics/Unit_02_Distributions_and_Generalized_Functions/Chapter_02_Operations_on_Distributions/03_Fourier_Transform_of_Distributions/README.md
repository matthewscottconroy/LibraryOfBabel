# The Fourier Transform of Distributions

The Fourier transform is the central tool for solving linear PDEs with constant coefficients: it converts differentiation to multiplication by polynomials, turning differential equations into algebraic equations. For smooth, rapidly decaying functions, the Fourier transform is classical. Extending it to distributions—particularly the tempered distributions $\mathcal{S}'$—enables the treatment of PDEs with singular sources (like delta functions) and produces the fundamental solutions that are the basis of Green's function theory.

## The Fourier Transform on $L^1$ and $L^2$

Recall: for $f \in L^1(\mathbb{R}^n)$, the **Fourier transform** is:

$$\hat{f}(\xi) = \mathcal{F}[f](\xi) = \int_{\mathbb{R}^n} f(x) e^{-2\pi i x \cdot \xi} \, dx.$$

(We use the $2\pi$ convention; other normalizations change the Fourier inversion formula by factors of $(2\pi)^n$.) The inverse Fourier transform is $\check{f}(x) = \int \hat{f}(\xi) e^{2\pi i x \cdot \xi} \, d\xi$.

On $L^1$: $\|\hat{f}\|_{L^\infty} \leq \|f\|_{L^1}$; $\hat{f} \to 0$ as $|\xi| \to \infty$ (Riemann-Lebesgue). On $L^2$: $\mathcal{F}$ is a unitary isomorphism (Plancherel: $\|\hat{f}\|_{L^2} = \|f\|_{L^2}$).

Key formulas:
- Differentiation: $\widehat{D^\alpha f}(\xi) = (2\pi i \xi)^\alpha \hat{f}(\xi)$.
- Multiplication: $\widehat{x^\alpha f}(\xi) = (2\pi i)^{-|\alpha|}(D^\alpha \hat{f})(\xi)$.
- Convolution: $\widehat{f*g}(\xi) = \hat{f}(\xi)\hat{g}(\xi)$.

## The Schwartz Space and Its Fourier Transform

The Schwartz space $\mathcal{S}(\mathbb{R}^n)$ (smooth functions decaying faster than any polynomial, with all derivatives also rapidly decaying) is the ideal domain for the Fourier transform:

**Theorem.** The Fourier transform $\mathcal{F}: \mathcal{S} \to \mathcal{S}$ is a topological isomorphism with inverse $\mathcal{F}^{-1} = \check{\mathcal{F}}$.

This means: if $f \in \mathcal{S}$, then $\hat{f} \in \mathcal{S}$ (the transform of a Schwartz function is again Schwartz). The inversion formula $f(x) = \int \hat{f}(\xi) e^{2\pi i x \cdot \xi} \, d\xi$ holds exactly in $\mathcal{S}$.

## Tempered Distributions

**Definition.** A **tempered distribution** is a continuous linear functional $T: \mathcal{S}(\mathbb{R}^n) \to \mathbb{R}$. The space of tempered distributions is $\mathcal{S}'(\mathbb{R}^n)$.

Continuity means: $\phi_j \to 0$ in $\mathcal{S}$ implies $T(\phi_j) \to 0$, where convergence in $\mathcal{S}$ is in every seminorm $\|\phi\|_{\alpha,\beta} = \sup_x |x^\beta D^\alpha\phi(x)|$.

Every distribution with compact support is tempered: $\mathcal{E}' \subset \mathcal{S}'$. Every polynomially bounded function $f$ (i.e., $|f(x)| \leq C(1+|x|)^N$ for some $N$) defines a tempered distribution. In particular, $L^p$ functions are tempered distributions for $1 \leq p \leq \infty$.

The Dirac delta $\delta$ is tempered: $|\langle\delta, \phi\rangle| = |\phi(0)| \leq \|\phi\|_{\infty}$, and $\|\phi\|_\infty$ is one of the Schwartz seminorms.

## The Fourier Transform on $\mathcal{S}'$

**Definition.** For $T \in \mathcal{S}'$, the **Fourier transform** $\hat{T} \in \mathcal{S}'$ is defined by duality:

$$\langle \hat{T}, \phi \rangle = \langle T, \hat{\phi} \rangle, \quad \phi \in \mathcal{S}.$$

**Consistency.** If $T = T_f$ for $f \in L^1 \cap \mathcal{S}'$: $\langle \hat{T}_f, \phi \rangle = \langle T_f, \hat\phi \rangle = \int f \hat\phi \, dx$. By Parseval/Fubini (when justified): $= \int \hat{f} \phi \, d\xi = \langle T_{\hat{f}}, \phi \rangle$. So $\widehat{T_f} = T_{\hat{f}}$.

**Theorem.** $\mathcal{F}: \mathcal{S}' \to \mathcal{S}'$ is a topological isomorphism with $\mathcal{F}^{-1} = \check{\mathcal{F}}$.

## Key Formulas for Tempered Distributions

1. **Differentiation becomes multiplication:** $\widehat{D^\alpha T} = (2\pi i\xi)^\alpha \hat{T}$.
   
   Proof: $\langle \widehat{D^\alpha T}, \phi \rangle = \langle D^\alpha T, \hat\phi \rangle = (-1)^{|\alpha|}\langle T, D^\alpha\hat\phi \rangle = (-1)^{|\alpha|}\langle T, \widehat{(2\pi i\xi)^\alpha\phi}\cdot(-1)^{|\alpha|}\rangle$...more directly: use $D^\alpha\hat\phi = \widehat{(-2\pi ix)^\alpha \phi}$ and $\widehat{D^\alpha\phi}(\xi) = (2\pi i\xi)^\alpha\hat\phi(\xi)$.

2. **Multiplication becomes differentiation:** $\widehat{x^\alpha T} = (-2\pi i)^{-|\alpha|} D^\alpha \hat{T}$.

3. **Convolution becomes multiplication:** $\widehat{T * \phi} = \hat{T} \cdot \hat\phi$ for $T \in \mathcal{S}'$, $\phi \in \mathcal{S}$.

## Examples

**Fourier transform of $\delta$:** $\langle\hat\delta, \phi\rangle = \langle\delta, \hat\phi\rangle = \hat\phi(0) = \int \phi(x) \, dx = \langle 1, \phi\rangle$. So $\hat\delta = 1$ (the constant function 1, as a tempered distribution).

**Fourier transform of 1:** $\hat{1} = \delta$ (by the Fourier inversion formula: $\int e^{-2\pi i\xi\cdot x} \, dx = \delta(\xi)$ in $\mathcal{S}'$).

**Fourier transform of $e^{2\pi i a \cdot x}$:** $\widehat{e^{2\pi ia\cdot x}} = \delta_a$ (delta at $a$).

**Fourier transform of $\text{p.v.}(1/x)$:** $\widehat{\text{p.v.}(1/x)}(\xi) = -\pi i \, \text{sgn}(\xi)$.

**Fourier transform of $H$:** $\hat{H}(\xi) = \frac{1}{2}\delta(\xi) + \frac{1}{2\pi i\xi}$ (as a tempered distribution; the second term is $\text{p.v.}(1/2\pi i\xi)$).

## Application: Solving the Heat Equation

The heat equation $\partial_t u = \partial_{xx} u$, $u(x,0) = u_0(x)$, can be solved by taking the Fourier transform in $x$:

$$\partial_t \hat{u}(\xi, t) = (2\pi i\xi)^2 \hat{u} = -4\pi^2\xi^2 \hat{u}, \quad \hat{u}(\xi, 0) = \hat{u}_0(\xi).$$

This ODE in $t$ has solution $\hat{u}(\xi, t) = e^{-4\pi^2\xi^2 t} \hat{u}_0(\xi)$. Inverting:

$$u(x,t) = \mathcal{F}^{-1}\left[e^{-4\pi^2\xi^2 t}\right] * u_0(x) = \frac{1}{2\sqrt{\pi t}} e^{-x^2/(4t)} * u_0(x).$$

The distributional Fourier transform makes this argument rigorous for $u_0 \in \mathcal{S}'$, including initial data that are distributions (e.g., $u_0 = \delta$, giving the heat kernel $u(x,t) = (4\pi t)^{-1/2} e^{-x^2/(4t)}$).

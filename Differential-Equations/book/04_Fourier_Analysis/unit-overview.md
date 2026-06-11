# Unit Overview: Fourier Analysis

## The Idea That Changed Mathematics

In 1807, Joseph Fourier submitted a paper to the French Academy of Sciences in which he claimed that any function defined on a finite interval could be expressed as an infinite sum of sines and cosines. The referees — including Lagrange, the greatest living analyst of the time — rejected the paper. Lagrange in particular was convinced the claim was false: how could a continuous superposition of smooth periodic functions represent, say, a function with a jump discontinuity? The controversy was not resolved for decades, and the effort to resolve it gave birth to modern analysis. The precise statement of Fourier's theorem, the definition of what it means for an infinite series to "represent" a function, the construction of Cantor's set theory to understand sets of convergence — all of this grew from the question of whether Fourier series converge.

Today, Fourier analysis is one of the most powerful and broadly applicable areas of mathematics. Structurally, it is the theory of how to decompose a function into oscillatory components of definite frequency, and how to recover the function from those components. The decomposition is linear (the Fourier transform is a linear operator), isometric (Parseval's/Plancherel's theorem says it preserves the $L^2$ norm), and admits a complete inversion formula. These properties together make it a tool of extraordinary utility: it converts differentiation into multiplication (in frequency space), convolution into pointwise multiplication, and hard-to-solve PDEs into algebraic equations.

## Fourier Series: The Classical Theory

Let $f$ be a $2\pi$-periodic function with $\int_{-\pi}^\pi |f|^2 < \infty$ (i.e., $f \in L^2[-\pi,\pi]$).

**Definition (Fourier Coefficients).** The Fourier coefficients of $f$ are
$$a_n = \frac{1}{\pi}\int_{-\pi}^\pi f(x)\cos(nx)\,dx, \quad b_n = \frac{1}{\pi}\int_{-\pi}^\pi f(x)\sin(nx)\,dx$$
for $n = 0, 1, 2, \ldots$ The Fourier series of $f$ is
$$\frac{a_0}{2} + \sum_{n=1}^\infty (a_n\cos(nx) + b_n\sin(nx)).$$

The complex form is more elegant: with $c_n = \frac{1}{2\pi}\int_{-\pi}^\pi f(x)e^{-inx}\,dx$, the Fourier series is $\sum_{n=-\infty}^\infty c_n e^{inx}$.

**Why this works: inner product structure.** The space $L^2[-\pi,\pi]$ carries an inner product $\langle f, g\rangle = \frac{1}{\pi}\int_{-\pi}^\pi f(x)\overline{g(x)}\,dx$. The functions $e^{inx}$ form an orthonormal set: $\langle e^{inx}, e^{imx}\rangle = \delta_{nm}$. The Fourier coefficient $c_n = \langle f, e^{inx}\rangle$ is the projection of $f$ onto the $n$th basis vector. This is exactly the same computation as projecting a vector onto an orthonormal basis in $\mathbb{R}^n$; the infinite-dimensional generalization is Fourier analysis.

**Theorem (Pointwise Convergence, Dirichlet).** If $f$ is $2\pi$-periodic and piecewise smooth (piecewise $C^1$) on $[-\pi,\pi]$, then the Fourier series of $f$ converges at every $x$ to $\frac{1}{2}[f(x^+) + f(x^-)]$ — the average of the left and right limits. At points of continuity, this equals $f(x)$.

**Theorem (Uniform Convergence).** If $f$ is $2\pi$-periodic and $C^1$, the Fourier series converges uniformly to $f$.

**Theorem (Parseval's Identity).** For $f \in L^2[-\pi,\pi]$,
$$\frac{1}{\pi}\int_{-\pi}^\pi |f(x)|^2\,dx = \frac{|a_0|^2}{2} + \sum_{n=1}^\infty (|a_n|^2 + |b_n|^2) = \sum_{n=-\infty}^\infty |c_n|^2.$$

Parseval's identity says that the $L^2$ norm of $f$ equals the $\ell^2$ norm of its sequence of Fourier coefficients. Geometrically, it is the Pythagorean theorem in $L^2$: the "length squared" of a vector equals the sum of the squares of its components.

**Theorem (Completeness / $L^2$ Convergence).** For $f \in L^2[-\pi,\pi]$, the Fourier partial sums converge to $f$ in the $L^2$ norm:
$$\lim_{N\to\infty} \int_{-\pi}^\pi \left|f(x) - \sum_{|n|\leq N} c_n e^{inx}\right|^2 dx = 0.$$

This is much stronger than pointwise convergence: the Fourier series converges to $f$ as a vector in $L^2$, meaning the $L^2$ error goes to zero, even if pointwise convergence fails on a set of measure zero.

**The Gibbs Phenomenon.** At a jump discontinuity, the Fourier partial sums overshoot the function by approximately $9\%$ of the jump height, regardless of how many terms are taken. This overshoot does not vanish; it localizes near the discontinuity as more terms are added. The Gibbs phenomenon shows that $L^2$ convergence and uniform convergence are genuinely different, and it explains why Fourier series are unsuitable for approximating functions with sharp discontinuities in applications requiring pointwise accuracy (motivating wavelets as an alternative).

## The Fourier Transform

For functions defined on all of $\mathbb{R}$, taking the period to infinity in a Fourier series converts discrete sums into continuous integrals.

**Definition.** For $f \in L^1(\mathbb{R})$, the Fourier transform is
$$\hat{f}(\xi) = \int_{-\infty}^\infty f(x)e^{-2\pi i\xi x}\,dx.$$

The inverse transform (when $\hat{f} \in L^1$) is $f(x) = \int_{-\infty}^\infty \hat{f}(\xi)e^{2\pi i\xi x}\,d\xi$.

**Key Algebraic Properties:**
- Linearity: $\widehat{\alpha f + \beta g} = \alpha\hat{f} + \beta\hat{g}$.
- Shifting: $\widehat{f(x-a)}(\xi) = e^{-2\pi ia\xi}\hat{f}(\xi)$.
- Scaling: $\widehat{f(ax)}(\xi) = \frac{1}{|a|}\hat{f}(\xi/a)$.
- Differentiation: $\widehat{f'}(\xi) = 2\pi i\xi\,\hat{f}(\xi)$.
- Convolution: $\widehat{f*g} = \hat{f}\cdot\hat{g}$.

The differentiation formula $\widehat{f'}(\xi) = 2\pi i\xi\,\hat{f}(\xi)$ is the key to solving PDEs: differentiation becomes multiplication by a polynomial in frequency space, converting a differential equation into an algebraic one.

**Theorem (Plancherel).** The Fourier transform extends to a unitary isometry on $L^2(\mathbb{R})$:
$$\int_{-\infty}^\infty |f(x)|^2\,dx = \int_{-\infty}^\infty |\hat{f}(\xi)|^2\,d\xi.$$

The Plancherel theorem says that the Fourier transform preserves energy (in the signal processing sense) and is therefore an isometric isomorphism of $L^2(\mathbb{R})$ onto itself.

**Uncertainty Principle.** If $f \in L^2(\mathbb{R})$ with $\int x^2|f(x)|^2\,dx < \infty$ and $\int \xi^2|\hat{f}(\xi)|^2\,d\xi < \infty$, then
$$\left(\int x^2|f|^2\,dx\right)\left(\int \xi^2|\hat{f}|^2\,d\xi\right) \geq \frac{\|f\|_2^4}{16\pi^2}.$$

A function cannot be simultaneously highly concentrated in space and highly concentrated in frequency. In quantum mechanics, with $x$ the position and $\xi$ the momentum (up to $\hbar$), this is Heisenberg's uncertainty principle.

## Applications to the Heat Equation

Consider the heat equation on the whole line: $u_t = \kappa u_{xx}$, $u(x,0) = f(x)$.

Apply the Fourier transform in $x$: let $\hat{u}(\xi, t) = \int_{-\infty}^\infty u(x,t)e^{-2\pi i\xi x}\,dx$.

Then $\hat{u}_t = -(2\pi\xi)^2\kappa\hat{u}$. This is an ODE in $t$: $\hat{u}(\xi,t) = \hat{f}(\xi)e^{-4\pi^2\kappa\xi^2 t}$.

Inverting: $u(x,t) = \int_{-\infty}^\infty \hat{f}(\xi)e^{-4\pi^2\kappa\xi^2 t}e^{2\pi i\xi x}\,d\xi$.

By the convolution theorem, this is $u = f * K_t$ where
$$K_t(x) = \frac{1}{\sqrt{4\pi\kappa t}}e^{-x^2/(4\kappa t)}$$
is the heat kernel (or Gaussian kernel). The solution is the convolution of the initial data with this Gaussian kernel, which spreads and diffuses as $t$ increases.

This derivation illustrates the power of the Fourier transform: a PDE in two variables $(x,t)$ becomes an ODE in one variable $t$ (parametrized by frequency $\xi$), which is trivially solvable, and the final answer comes from inverting the transform.

## Parseval's Theorem and Energy

Parseval's theorem has a beautiful physical interpretation. If $f(t)$ is a time-domain signal, $|f(t)|^2$ is instantaneous power, and $\int|f|^2$ is total energy. The Fourier transform $\hat{f}(\xi)$ is the amplitude of frequency $\xi$, and $|\hat{f}(\xi)|^2$ is the power spectral density. Parseval says total energy computed in the time domain equals total energy computed in the frequency domain.

**Application: Proving an Inequality.** For $f \in L^2$ with $\int|f'|^2 < \infty$:
$$\int|f|^2\,dx = \int|\hat{f}|^2\,d\xi, \quad \int|f'|^2\,dx = \int (2\pi\xi)^2|\hat{f}(\xi)|^2\,d\xi.$$
These are the Plancherel identity for $f$ and $f'$ respectively. Together they allow one to prove Sobolev-type inequalities relating $\|f\|_{L^2}$ to $\|f'\|_{L^2}$ via the Fourier representation.

## Historical Notes

**Jean-Baptiste Joseph Fourier (1768–1830)** introduced the idea of expanding functions in trigonometric series in his study of heat conduction (submitted 1807, published as *Théorie analytique de la chaleur*, 1822). His bold claim — that any function could be so represented — was visionary and controversial. Fourier was a civil administrator and military engineer as much as a mathematician; he accompanied Napoleon to Egypt and served as prefect of a French department. His mathematical work was driven entirely by the physical problem of heat diffusion.

**Peter Gustav Lejeune Dirichlet (1805–1859)** provided the first rigorous proof of pointwise convergence of Fourier series (under piecewise smoothness conditions) in 1829. His paper introduced what we now call the Dirichlet kernel and the notion of piecewise smooth functions.

**Bernhard Riemann (1826–1866)** generalized Dirichlet's conditions and introduced the Riemann integral in the course of his work on Fourier series (1854). His Habilitation lecture introduced Riemannian geometry, but his doctoral thesis and Habilitation thesis are both centered on Fourier series.

**Georg Cantor (1845–1918)** proved that if a Fourier series converges to zero except on a finite set, then all its coefficients are zero (1870). The effort to generalize this to more complicated exceptional sets led him to invent set theory and transfinite numbers.

**Marc-Antoine Parseval (1755–1836)** stated the theorem bearing his name in 1805, before Fourier's paper, in a purely formal context; its rigorous proof required the development of $L^2$ theory.

**Henri Lebesgue (1875–1941)** and **Frigyes Riesz (1880–1956)** developed the $L^2$ theory of Fourier series rigorously in the early twentieth century. The Riesz-Fischer theorem (1907) — that the space of square-integrable functions is complete — made it possible to prove $L^2$ convergence of Fourier series.

**Norbert Wiener (1894–1964)** developed the Fourier transform on $L^2(\mathbb{R})$ and applied it to stochastic processes, laying the foundations for signal processing and communication theory. His book *The Fourier Integral and Certain of its Applications* (1933) is a landmark.

**Laurent Schwartz (1915–2002)** extended the Fourier transform to tempered distributions (1945–50), enabling the transform of objects like the Dirac delta that are not classical functions. This extension is essential for a rigorous treatment of the heat kernel and fundamental solutions of PDEs.

## Connections to Other Units

**Prerequisites:**
- Unit 00 (Foundations): $L^2$ convergence, Cauchy sequences, completeness (the Hilbert space $L^2$ is complete).
- Unit 01 (Multivariable Calculus): improper integrals, uniform convergence.
- Unit 03 (ODEs): Sturm-Liouville theory produces the eigenfunctions that generalize sines and cosines.

**Downstream:**
- Unit 05 (PDEs): Fourier series are the engine of separation of variables for the heat, wave, and Laplace equations on bounded domains. The Fourier transform solves PDEs on $\mathbb{R}^n$.
- Unit 06 (Complex Analysis): The Fourier transform extends to a contour integral in the complex plane; the inverse transform uses the residue theorem. The Paley-Wiener theorem characterizes the Fourier transforms of functions with compact support in terms of their analytic continuation.
- Unit 08 (Advanced Topics): Tempered distributions are the proper setting for the Fourier transform of non-$L^2$ objects; the Schwartz space and its dual are the technical apparatus.

## Key Theorems at a Glance

1. **Fourier Coefficients as Projections:** $c_n = \langle f, e^{inx}\rangle$ in $L^2[-\pi,\pi]$ with the standard inner product.
2. **Dirichlet's Theorem (Pointwise Convergence):** Piecewise smooth functions have Fourier series converging to the midpoint of jumps.
3. **Parseval's Identity:** $\|f\|_{L^2}^2 = \sum_n |c_n|^2$ — energy is preserved.
4. **$L^2$ Convergence (Completeness):** Fourier partial sums converge to $f$ in $L^2$ norm; $\{e^{inx}\}$ is a complete orthonormal system.
5. **Gibbs Phenomenon:** Near a jump discontinuity, partial sums overshoot by $\approx 9\%$ of the jump.
6. **Fourier Transform Differentiation Rule:** $\widehat{f^{(k)}}(\xi) = (2\pi i\xi)^k\hat{f}(\xi)$ — differentiation becomes multiplication.
7. **Convolution Theorem:** $\widehat{f*g} = \hat{f}\cdot\hat{g}$ — convolution becomes multiplication in frequency space.
8. **Plancherel's Theorem:** $\|f\|_{L^2(\mathbb{R})} = \|\hat{f}\|_{L^2(\mathbb{R})}$ — the Fourier transform is an isometry on $L^2$.
9. **Uncertainty Principle:** $\|xf\|_{L^2}\cdot\|\xi\hat{f}\|_{L^2} \geq \|f\|_{L^2}^2/(4\pi)$.
10. **Heat Kernel Formula:** The solution to the heat equation with initial data $f$ is $u(\cdot,t) = f * K_t$ where $K_t = (4\pi\kappa t)^{-1/2}e^{-x^2/(4\kappa t)}$.

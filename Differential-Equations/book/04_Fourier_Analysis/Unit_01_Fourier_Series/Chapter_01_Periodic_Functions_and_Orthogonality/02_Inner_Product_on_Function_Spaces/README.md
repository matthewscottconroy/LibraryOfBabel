# Inner Products on Function Spaces

In linear algebra, the dot product on $\mathbb{R}^n$ provides a way to measure angles and lengths, decompose vectors into components, and project one vector onto another. The power of this geometry is not limited to finite-dimensional spaces. Function spaces carry a natural analog of the dot product, and exploiting this structure is the key to understanding why Fourier series work.

## Motivation: Projection in $\mathbb{R}^n$

Suppose $\{\mathbf{e}_1, \ldots, \mathbf{e}_n\}$ is an orthonormal basis of $\mathbb{R}^n$, meaning $\mathbf{e}_i \cdot \mathbf{e}_j = \delta_{ij}$. Any vector $\mathbf{v}$ can be written uniquely as
$$\mathbf{v} = \sum_{i=1}^n (\mathbf{v} \cdot \mathbf{e}_i)\,\mathbf{e}_i.$$
The coefficient $c_i = \mathbf{v} \cdot \mathbf{e}_i$ is the component of $\mathbf{v}$ in the direction of $\mathbf{e}_i$. No complicated system of equations needs to be solved; the orthogonality of the basis makes each coefficient computable independently.

The Fourier coefficient formulas are exactly this: the trigonometric functions are orthogonal, and the Fourier coefficients are the corresponding "components" of $f$. But to make this analogy rigorous for functions, we need an inner product on a suitable function space.

## The $L^2$ Inner Product

Let $[a, b]$ be a bounded interval. Define the space
$$L^2([a,b]) = \left\{ f : [a,b] \to \mathbb{R} \;\Big|\; \int_a^b |f(x)|^2\,dx < \infty \right\},$$
where the integral is in the Lebesgue sense (for our purposes, Riemann-integrable functions automatically qualify). Define the **$L^2$ inner product**
$$\langle f, g \rangle = \int_a^b f(x)\,g(x)\,dx.$$

This satisfies the four defining properties of an inner product:

1. **Symmetry:** $\langle f, g \rangle = \langle g, f \rangle$, since multiplication of real numbers is commutative.
2. **Linearity in the first argument:** $\langle \alpha f + \beta g, h \rangle = \alpha\langle f, h\rangle + \beta\langle g, h\rangle$, which follows from linearity of the integral.
3. **Positive semi-definiteness:** $\langle f, f \rangle = \int_a^b f(x)^2\,dx \geq 0$.
4. **Non-degeneracy:** $\langle f, f \rangle = 0$ implies $f(x) = 0$ for almost every $x$ in $[a,b]$.

(Property 4 is slightly subtle: a nonzero function can have $\int f^2 = 0$ if it is nonzero only on a set of measure zero. The standard convention identifies functions that agree almost everywhere, turning $L^2([a,b])$ into a space of equivalence classes where property 4 holds exactly.)

## The $L^2$ Norm and Distance

The **$L^2$ norm** associated to this inner product is
$$\|f\|_2 = \sqrt{\langle f, f\rangle} = \left(\int_a^b f(x)^2\,dx\right)^{1/2}.$$
This is the natural notion of "size" for a function in $L^2$. It is not the maximum value of $|f|$, nor the integral of $|f|$, but the root-mean-square amplitude. In signal processing, $\|f\|_2^2$ is proportional to the total energy of the signal $f$.

The **distance** between two functions is $\|f - g\|_2$, and convergence in $L^2$ means $\|f_n - f\|_2 \to 0$. This is weaker than uniform convergence (convergence in the sup norm), but stronger than mere pointwise convergence.

**Theorem (Cauchy-Schwarz).** For all $f, g \in L^2([a,b])$,
$$|\langle f, g \rangle| \leq \|f\|_2 \cdot \|g\|_2,$$
with equality if and only if $f$ and $g$ are proportional (i.e., $f = cg$ a.e. for some constant $c$).

The proof is the same as in finite dimensions: consider $0 \leq \|f - tg\|_2^2 = \|f\|_2^2 - 2t\langle f,g\rangle + t^2\|g\|_2^2$ as a quadratic in $t$, and require its discriminant to be non-positive.

## Weighted Inner Products

For some problems, particularly Sturm-Liouville boundary value problems, a **weighted inner product** is more natural:
$$\langle f, g \rangle_w = \int_a^b f(x)\,g(x)\,w(x)\,dx,$$
where $w : [a,b] \to (0, \infty)$ is a positive weight function. The standard $L^2$ inner product is the special case $w \equiv 1$.

Different weight functions lead to different families of orthogonal functions. For example, on $[-1, 1]$ with weight $w(x) = 1/\sqrt{1-x^2}$, the Chebyshev polynomials are orthogonal. On $(-\infty, \infty)$ with weight $w(x) = e^{-x^2}$, the Hermite polynomials are orthogonal. The trigonometric system is orthogonal with constant weight on $[-\pi, \pi]$, which is one reason it is the simplest and most natural example.

## Hilbert Spaces

A key fact about $L^2([a,b])$ is that it is **complete**: every Cauchy sequence in the $L^2$ norm converges to an element of $L^2([a,b])$. (This is not obvious, since pointwise limits of $L^2$ functions need not be $L^2$, but the Lebesgue theory handles this.) A complete inner product space is called a **Hilbert space**.

Hilbert spaces have many of the geometric properties of $\mathbb{R}^n$:

- The **parallelogram law** $\|f+g\|^2 + \|f-g\|^2 = 2\|f\|^2 + 2\|g\|^2$ holds.
- Every closed subspace has an orthogonal complement.
- The projection theorem holds: for any closed subspace $V$ and any $f \in L^2$, there is a unique $g \in V$ minimizing $\|f - g\|$.
- An orthonormal sequence $\{e_n\}$ is a basis (in the $L^2$ sense) if and only if it is **complete**, meaning $\langle f, e_n\rangle = 0$ for all $n$ implies $f = 0$.

The trigonometric system is a complete orthonormal basis for $L^2([-\pi, \pi])$ (with appropriate normalization). This is the theorem that guarantees every $L^2$ function equals its Fourier series in the $L^2$ sense.

## Orthogonality and Projections

Two functions $f, g \in L^2([a,b])$ are **orthogonal** if $\langle f, g \rangle = 0$. A set of functions $\{f_n\}$ is **orthogonal** if $\langle f_m, f_n \rangle = 0$ whenever $m \neq n$, and **orthonormal** if additionally $\langle f_n, f_n \rangle = 1$ for all $n$.

Given an orthonormal sequence $\{e_n\}$ in $L^2([a,b])$ and a function $f$, the **$n$-th Fourier coefficient of $f$ with respect to $\{e_n\}$** is $c_n = \langle f, e_n \rangle$. The partial sum
$$S_N = \sum_{n=1}^N c_n e_n$$
is the **orthogonal projection** of $f$ onto the span of $\{e_1, \ldots, e_N\}$. This means $S_N$ is the best approximation to $f$ in this subspace: $\|f - S_N\|_2 \leq \|f - g\|_2$ for all $g$ in the span of $\{e_1, \ldots, e_N\}$.

**Bessel's inequality.** For any $f \in L^2([a,b])$ and any orthonormal sequence $\{e_n\}$,
$$\sum_{n=1}^\infty |\langle f, e_n \rangle|^2 \leq \|f\|_2^2.$$
This says the sum of squares of the Fourier coefficients is bounded. If the orthonormal sequence is a complete basis, then equality holds (Parseval's identity), and the partial sums $S_N$ converge to $f$ in $L^2$.

## Worked Example: Computing an $L^2$ Inner Product

Let $f(x) = x$ and $g(x) = x^2$ on $[-1, 1]$. Then
$$\langle f, g \rangle = \int_{-1}^1 x \cdot x^2\,dx = \int_{-1}^1 x^3\,dx = 0,$$
since $x^3$ is odd and the interval is symmetric about zero. So $f$ and $g$ are orthogonal in $L^2([-1,1])$.

Now compute $\|f\|_2$ and $\|g\|_2$:
$$\|f\|_2^2 = \int_{-1}^1 x^2\,dx = \frac{2}{3}, \quad \|g\|_2^2 = \int_{-1}^1 x^4\,dx = \frac{2}{5}.$$
The angle $\theta$ between $f$ and $g$ satisfies $\cos\theta = \langle f,g\rangle / (\|f\|_2 \|g\|_2) = 0$, so they are perpendicular — exactly as the orthogonality computation said.

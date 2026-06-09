# Chapter 01: Periodic Functions and Orthogonality

Before we can speak of representing a function by a trigonometric series, we need to make precise the concepts that underlie the entire theory. This chapter establishes that foundation in three layers: what it means for a function to be periodic and how periods and harmonics relate; what an inner product on a space of functions is and why it matters; and how the trigonometric functions form an orthogonal set under that inner product.

## Periodicity and Harmonics

A function $f : \mathbb{R} \to \mathbb{R}$ is **periodic with period $T > 0$** if $f(x + T) = f(x)$ for all $x \in \mathbb{R}$. If $T$ is a period, so is every positive integer multiple $nT$. The smallest such positive $T$ is called the **fundamental period**. A constant function is technically periodic with every period but has no fundamental period.

The frequency corresponding to period $T$ is $\nu = 1/T$, measured in cycles per unit. In angular units, the **fundamental frequency** is $\omega_0 = 2\pi/T$ radians per unit. The $n$-th harmonic is a sinusoidal oscillation at $n$ times the fundamental frequency, with period $T/n$. Harmonics are the building blocks of Fourier series: the series $\sum a_n \cos(n\omega_0 x) + b_n \sin(n\omega_0 x)$ consists of the fundamental and all its harmonics.

For most of this chapter we normalize to $T = 2\pi$, so $\omega_0 = 1$ and the harmonics are $\cos(nx)$ and $\sin(nx)$ for $n = 1, 2, 3, \ldots$. Any result for period $2\pi$ extends to arbitrary period $2L$ by the substitution $x \mapsto \pi x / L$.

## Inner Products on Function Spaces

In Euclidean space $\mathbb{R}^n$, the dot product $\mathbf{u} \cdot \mathbf{v} = \sum_i u_i v_i$ gives us a notion of angle and length. Two vectors are orthogonal when their dot product is zero, and we can project any vector onto a subspace using inner products with basis vectors. Fourier analysis rests on extending this geometric structure to spaces of functions.

Let $L^2([-\pi, \pi])$ denote the space of (equivalence classes of) functions $f : [-\pi, \pi] \to \mathbb{R}$ with $\int_{-\pi}^\pi |f(x)|^2\,dx < \infty$. We define the **inner product**
$$\langle f, g \rangle = \frac{1}{\pi} \int_{-\pi}^\pi f(x)\,g(x)\,dx.$$
(The factor $1/\pi$ is a normalization convention; some texts use $\int_{-\pi}^\pi$ without the factor, adjusting coefficient formulas accordingly.) This satisfies the axioms of an inner product: symmetry, linearity in the first argument, and positive-definiteness ($\langle f, f \rangle = 0$ implies $f = 0$ a.e.).

The associated **norm** is $\|f\| = \langle f, f \rangle^{1/2}$, and the **Cauchy-Schwarz inequality** $|\langle f, g \rangle| \leq \|f\| \cdot \|g\|$ holds in this setting just as in $\mathbb{R}^n$. The space $L^2([-\pi, \pi])$ with this inner product is a **Hilbert space**: complete with respect to the metric induced by the norm.

## Orthogonality of the Trigonometric System

**Theorem (Orthogonality Relations).** The trigonometric functions $\{1, \cos x, \sin x, \cos 2x, \sin 2x, \ldots\}$ satisfy, with respect to the inner product $\langle f, g \rangle = \frac{1}{\pi}\int_{-\pi}^\pi fg\,dx$:

1. $\langle \cos(mx), \cos(nx) \rangle = \delta_{mn}$ for $m, n \geq 1$.
2. $\langle \sin(mx), \sin(nx) \rangle = \delta_{mn}$ for $m, n \geq 1$.
3. $\langle \cos(mx), \sin(nx) \rangle = 0$ for all $m, n \geq 0$.
4. $\langle 1, \cos(nx) \rangle = \langle 1, \sin(nx) \rangle = 0$ for $n \geq 1$.
5. $\langle 1, 1 \rangle = 2$ (or $= 1$ with a slightly different normalization).

These follow from the product-to-sum identities: for instance,
$$\cos(mx)\cos(nx) = \frac{1}{2}[\cos((m-n)x) + \cos((m+n)x)],$$
and integrating over $[-\pi, \pi]$ kills any cosine term with nonzero integer argument.

The orthogonality relations mean that the trigonometric functions, once properly normalized, form an **orthonormal set** in $L^2([-\pi, \pi])$. In Chapter 03 of this unit, we will see that this set is also **complete**: no nonzero function in $L^2$ is orthogonal to all trigonometric functions. Completeness is what guarantees that the Fourier series of an $L^2$ function converges to that function in the $L^2$ sense.

## Preview of Chapters 02–04

With orthogonality established, the Fourier coefficient formulas in Chapter 02 become conceptually obvious: $a_n = \langle f, \cos(nx) \rangle$ is simply the projection of $f$ onto the $n$-th cosine basis element. The convergence theory asks how well the partial sums approximate $f$, which is where the analysis becomes subtle. Chapter 03 exploits the even/odd decomposition to produce pure sine or cosine series. Chapter 04 unifies everything using the complex orthonormal basis $\{e^{inx}\}_{n \in \mathbb{Z}}$, whose orthogonality follows from $\frac{1}{2\pi}\int_{-\pi}^\pi e^{i(m-n)x}\,dx = \delta_{mn}$.

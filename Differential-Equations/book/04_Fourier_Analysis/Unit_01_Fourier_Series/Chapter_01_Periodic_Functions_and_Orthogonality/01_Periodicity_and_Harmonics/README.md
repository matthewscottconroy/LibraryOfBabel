# Periodicity and Harmonics

Consider the problem of modeling the displacement of a vibrating string clamped at both ends. Physical intuition says the string oscillates at a fundamental frequency determined by its length and tension, but in practice it also vibrates simultaneously at integer multiples of that frequency. These higher modes are the harmonics, and their superposition produces the rich waveform we hear as a musical note. Making this idea mathematically precise requires a careful treatment of periodicity and how periodic functions at different frequencies relate to one another.

## Periodic Functions: Definition and Basic Properties

A function $f : \mathbb{R} \to \mathbb{R}$ is **periodic** if there exists $T > 0$ such that
$$f(x + T) = f(x) \quad \text{for all } x \in \mathbb{R}.$$
Any such $T$ is called a **period** of $f$. If $T$ is a period, then so is $nT$ for every positive integer $n$, because $f(x + nT) = f(x + (n-1)T + T) = f(x + (n-1)T)$, and induction completes the argument. Also, if $f$ and $g$ both have period $T$, then so does $\alpha f + \beta g$ for any constants $\alpha, \beta$.

The **fundamental period** of $f$ is the smallest positive $T$ for which $f(x + T) = f(x)$ for all $x$. Not every periodic function has a fundamental period: a constant function satisfies $f(x + T) = f(x)$ for every $T > 0$, so the infimum of periods is zero, and there is no smallest period. However, any continuous, non-constant periodic function does have a fundamental period (this requires a short argument using the intermediate value theorem to show the period cannot become arbitrarily small without the function being constant).

**Example.** The function $f(x) = \sin(x)$ has fundamental period $2\pi$. The function $g(x) = \sin(2x)$ has fundamental period $\pi$. The sum $h(x) = \sin(x) + \sin(2x)$ has fundamental period $2\pi$, since after one full period of $\sin(x)$, both components simultaneously return to their starting values, and no smaller period achieves this.

## Frequencies, Angular Frequency, and the Spectrum

Given a periodic function with fundamental period $T$, we define its **fundamental frequency** as $\nu_0 = 1/T$ (cycles per unit, or hertz in SI units) and its **fundamental angular frequency** as
$$\omega_0 = \frac{2\pi}{T} \quad \text{(radians per unit)}.$$
The function $\cos(\omega_0 t)$ completes exactly one full cycle as $t$ runs from $0$ to $T$. More generally, $\cos(n\omega_0 t)$ completes $n$ full cycles in the same interval, so it has period $T/n$ and angular frequency $n\omega_0$.

The **$n$-th harmonic** refers to a sinusoidal function oscillating at frequency $n\nu_0$ or equivalently at angular frequency $n\omega_0$. The case $n = 1$ is called the **fundamental** or **first harmonic**. The case $n = 2$ is the **second harmonic** (or first overtone), and so on.

We can write any sinusoidal function at the $n$-th harmonic in the form
$$A_n \cos(n\omega_0 t) + B_n \sin(n\omega_0 t) = R_n \cos(n\omega_0 t - \phi_n),$$
where the **amplitude** is $R_n = \sqrt{A_n^2 + B_n^2}$ and the **phase** is $\phi_n = \arctan(B_n / A_n)$. The collection of amplitudes $\{R_n\}$ as a function of the harmonic index $n$ is called the **amplitude spectrum** of the function, and the collection of phases $\{\phi_n\}$ is the **phase spectrum**.

## Normalization to Period $2\pi$

For most theoretical work, it is convenient to normalize to period $T = 2\pi$. Any function of period $2L$ can be converted to period $2\pi$ by the substitution $x \mapsto \pi x / L$. Specifically, if $g$ has period $2L$, then $f(x) = g(Lx/\pi)$ has period $2\pi$.

Under this normalization, the harmonics are $\cos(nx)$ and $\sin(nx)$ for $n = 1, 2, 3, \ldots$, all defined on $[-\pi, \pi]$ and extended periodically to $\mathbb{R}$. The fundamental period is $2\pi$ and the fundamental angular frequency is $\omega_0 = 1$.

When working with period $2L$, the harmonic frequencies are $n\pi/L$ and the coefficient formulas involve integrals over $[-L, L]$. We will state the general-period versions when presenting results, but proofs and computations are typically done in the normalized case.

## Worked Example: Identifying the Period and Harmonics

Let $f(x) = 3\cos(2x) - \sin(4x) + 2\cos(6x)$.

The term $3\cos(2x)$ has angular frequency $2$, so period $2\pi/2 = \pi$.
The term $-\sin(4x)$ has angular frequency $4$, so period $2\pi/4 = \pi/2$.
The term $2\cos(6x)$ has angular frequency $6$, so period $2\pi/6 = \pi/3$.

The fundamental period of $f$ is the least common multiple of $\pi$, $\pi/2$, and $\pi/3$. Dividing by $\pi$: LCM of $1$, $1/2$, $1/3$ as rational numbers is $1$ (the LCM of the numerators over the GCD of the denominators: $\text{lcm}(1,1,1)/\gcd(1,2,3) = 1$). So the fundamental period is $\pi$.

The fundamental angular frequency is $\omega_0 = 2\pi/\pi = 2$. Expressed in harmonics of $\omega_0 = 2$:
- $3\cos(2x) = 3\cos(\omega_0 x)$: the first harmonic.
- $-\sin(4x) = -\sin(2\omega_0 x)$: the second harmonic.
- $2\cos(6x) = 2\cos(3\omega_0 x)$: the third harmonic.

The amplitude spectrum is: $R_1 = 3$, $R_2 = 1$, $R_3 = 2$, with all other $R_n = 0$.

## Why Harmonics Are Special

The harmonic structure is not merely convenient; it is intrinsically connected to the eigenvalue problem for the differential operator $d^2/dx^2$ subject to periodic boundary conditions. On $[-\pi, \pi]$ with the boundary condition that the function and its derivative match at the endpoints, the eigenvalue equation $f'' = -\lambda f$ has solutions only for $\lambda = n^2$, $n = 0, 1, 2, \ldots$, and the corresponding eigenfunctions are exactly $\cos(nx)$ and $\sin(nx)$.

This means that the trigonometric harmonics are distinguished not just by convention but by the structure of the second-derivative operator. Any physical system governed by a second-order linear equation with periodic symmetry will naturally decompose into harmonic modes. The string, the drum, the electromagnetic cavity: all exhibit harmonic spectra for this fundamental reason.

## Piecewise-Defined Periodic Functions

In practice, periodic functions often arise from extending a function defined on one period. Given $f : [-\pi, \pi] \to \mathbb{R}$, the periodic extension $\tilde{f} : \mathbb{R} \to \mathbb{R}$ is defined by $\tilde{f}(x) = f(x - 2\pi k)$ where $k$ is the unique integer such that $x - 2\pi k \in [-\pi, \pi)$.

A natural question is what happens at the endpoints: if $f(-\pi) \neq f(\pi)$, then $\tilde{f}$ has a jump discontinuity at every odd multiple of $\pi$. The Fourier series will still converge in an appropriate sense, but its behavior at the jump is governed by the Gibbs phenomenon, discussed in Chapter 02.

A function is **piecewise smooth** on $[-\pi, \pi]$ if it is continuous except at finitely many points, and if at each point it has finite left and right limits and left and right derivatives. Most functions encountered in applications are piecewise smooth, and the convergence theory for Fourier series works particularly well for this class.

## Connection to Complex Exponentials

Euler's formula $e^{i\theta} = \cos\theta + i\sin\theta$ allows us to write the harmonics as
$$\cos(nx) = \frac{e^{inx} + e^{-inx}}{2}, \quad \sin(nx) = \frac{e^{inx} - e^{-inx}}{2i}.$$
The functions $\{e^{inx}\}_{n \in \mathbb{Z}}$ therefore contain the same information as the real trigonometric system, but in a more algebraically uniform form. Each function $e^{inx}$ has period $2\pi/|n|$ for $n \neq 0$ and is constant (period anything) for $n = 0$. In this language, positive $n$ corresponds to counterclockwise rotation in the complex plane and negative $n$ to clockwise rotation, and a general Fourier series is a superposition of rotations at all integer rates.

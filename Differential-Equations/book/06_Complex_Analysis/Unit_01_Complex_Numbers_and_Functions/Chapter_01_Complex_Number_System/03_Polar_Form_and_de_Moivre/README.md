# Polar Form and de Moivre's Theorem

The rectangular form $z = x + iy$ is natural for addition, but the polar form $z = r(\cos\theta + i\sin\theta)$ is far more efficient for multiplication, division, and the computation of powers and roots. This section develops the polar representation, establishes the key identity $e^{i\theta} = \cos\theta + i\sin\theta$ at an appropriate level of rigor, and derives de Moivre's theorem as an immediate consequence. The results here are used throughout the course.

## The Polar Form

Every nonzero complex number $z = x + iy$ can be written uniquely (up to the choice of argument) as
$$z = r(\cos\theta + i\sin\theta), \qquad r = |z| > 0, \quad \theta = \arg z.$$

The modulus $r = \sqrt{x^2 + y^2}$ and argument $\theta$ are recovered from $z$ by $r = |z|$ and $\theta = \arctan(y/x)$ (with the quadrant of $z$ determining the correct branch of arctangent).

Multiplication in polar form is transparent: if $z_1 = r_1(\cos\theta_1 + i\sin\theta_1)$ and $z_2 = r_2(\cos\theta_2 + i\sin\theta_2)$, then using the addition formulas for sine and cosine,
$$z_1 z_2 = r_1 r_2\bigl(\cos(\theta_1 + \theta_2) + i\sin(\theta_1 + \theta_2)\bigr).$$

Thus $|z_1 z_2| = |z_1||z_2|$ and $\arg(z_1 z_2) = \arg z_1 + \arg z_2$ (modulo $2\pi$). Division yields $|z_1/z_2| = |z_1|/|z_2|$ and $\arg(z_1/z_2) = \arg z_1 - \arg z_2$.

## Euler's Formula

**Theorem (Euler's Formula).** For all $\theta \in \mathbb{R}$,
$$e^{i\theta} = \cos\theta + i\sin\theta.$$

This equation can be established at different levels of rigor. The most elementary justification uses power series: the Taylor series for $e^z$, $\cos z$, and $\sin z$ converge absolutely for all $z \in \mathbb{C}$ (proved in Unit 03), and substituting $z = i\theta$ gives
$$e^{i\theta} = \sum_{n=0}^{\infty} \frac{(i\theta)^n}{n!} = \sum_{k=0}^{\infty} \frac{(-1)^k \theta^{2k}}{(2k)!} + i\sum_{k=0}^{\infty} \frac{(-1)^k \theta^{2k+1}}{(2k+1)!} = \cos\theta + i\sin\theta.$$

With Euler's formula, the polar form becomes $z = r e^{i\theta}$, and multiplication is the simple rule $r_1 e^{i\theta_1} \cdot r_2 e^{i\theta_2} = r_1 r_2\, e^{i(\theta_1+\theta_2)}$, which follows from the exponential law $e^{a+b} = e^a e^b$ (valid for complex numbers by the power series definition, as shown in Unit 02).

The most celebrated special case of Euler's formula is $e^{i\pi} + 1 = 0$, which relates the five fundamental constants of mathematics in a single equation.

## De Moivre's Theorem

**Theorem (de Moivre).** For any $\theta \in \mathbb{R}$ and any integer $n$,
$$(\cos\theta + i\sin\theta)^n = \cos(n\theta) + i\sin(n\theta).$$

**Proof.** In polar form, $(\cos\theta + i\sin\theta)^n = (e^{i\theta})^n = e^{in\theta} = \cos(n\theta) + i\sin(n\theta)$, where the identity $(e^{i\theta})^n = e^{in\theta}$ holds for integer $n$ by the exponential law and the standard rules of exponentiation. $\square$

For positive integer $n$, this can also be proved by induction using the angle-addition identity.

## Deriving Trigonometric Identities

De Moivre's theorem provides an efficient method for expressing $\cos(n\theta)$ and $\sin(n\theta)$ as polynomials in $\cos\theta$ and $\sin\theta$.

**Worked example.** Derive the triple angle formulas.

By de Moivre with $n = 3$:
$$\cos(3\theta) + i\sin(3\theta) = (\cos\theta + i\sin\theta)^3.$$
Expanding the right side using the binomial theorem:
$$(\cos\theta + i\sin\theta)^3 = \cos^3\theta + 3i\cos^2\theta\sin\theta - 3\cos\theta\sin^2\theta - i\sin^3\theta.$$
Separating real and imaginary parts:
$$\cos(3\theta) = \cos^3\theta - 3\cos\theta\sin^2\theta = 4\cos^3\theta - 3\cos\theta,$$
$$\sin(3\theta) = 3\cos^2\theta\sin\theta - \sin^3\theta = 3\sin\theta - 4\sin^3\theta.$$
These are the standard triple-angle formulas, obtained here without computing integrals or appealing to other identities.

## Computing Powers of Complex Numbers

**Worked example.** Compute $(1 + i)^{10}$.

Write $1 + i = \sqrt{2}\, e^{i\pi/4}$. Then
$$(1 + i)^{10} = (\sqrt{2})^{10} e^{i \cdot 10\pi/4} = 2^5 e^{i \cdot 5\pi/2} = 32\, e^{i\pi/2} = 32i.$$

The polar approach reduces a tedious binomial expansion to a two-step computation.

## The $n$-th Root Formula

The equation $z^n = w$ for $w \neq 0$ has exactly $n$ solutions in $\mathbb{C}$. If $w = \rho e^{i\phi}$, then the solutions are
$$z_k = \rho^{1/n}\, e^{i(\phi + 2\pi k)/n}, \qquad k = 0, 1, \ldots, n-1.$$
These $n$ roots are equally spaced on the circle of radius $\rho^{1/n}$, forming the vertices of a regular $n$-gon.

**Worked example.** Find all cube roots of $-8$.

Write $-8 = 8 e^{i\pi}$. The cube roots are
$$z_k = 8^{1/3}\, e^{i(\pi + 2\pi k)/3} = 2\, e^{i(\pi + 2\pi k)/3}, \qquad k = 0, 1, 2.$$
Explicitly:
$$z_0 = 2e^{i\pi/3} = 1 + i\sqrt{3}, \quad z_1 = 2e^{i\pi} = -2, \quad z_2 = 2e^{i5\pi/3} = 1 - i\sqrt{3}.$$
Check: $(-2)^3 = -8$ confirms $z_1$; for $z_0$, $(1 + i\sqrt{3})^3 = 8(\frac{1}{2} + i\frac{\sqrt{3}}{2})^3 = 8 e^{i\pi} = -8$. $\square$

## Connection to Fourier Analysis

De Moivre's theorem and the $n$-th roots of unity underpin discrete Fourier analysis. The roots of unity $\omega_k = e^{2\pi i k/n}$ are the fundamental frequencies of the discrete Fourier transform, and the orthogonality relation $\sum_{k=0}^{n-1} \omega_k^m = n \cdot \mathbf{1}_{n | m}$ is the discrete analogue of the continuous Fourier orthogonality of $e^{2\pi i mx}$ on $[0,1]$. This connection will resurface when residues are used to sum series in Unit 04.

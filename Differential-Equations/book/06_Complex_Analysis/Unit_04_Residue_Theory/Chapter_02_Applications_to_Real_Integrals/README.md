# Chapter 02: Applications of Residues to Real Integrals

The residue theorem is not merely a tool for complex contour integrals; it provides a systematic method for evaluating real definite integrals that are inaccessible by elementary calculus. The strategy is always the same: embed the real integral into a complex contour integral, choose a contour that closes the path in the complex plane, apply the residue theorem, and show that the contributions from the non-real parts of the contour vanish or can be computed. Different classes of integrands require different contour strategies.

## Section 01: Rational Trigonometric Integrals

Integrals of the form $\int_0^{2\pi} R(\cos\theta, \sin\theta)\, d\theta$ are evaluated by substituting $z = e^{i\theta}$:
$$\cos\theta = \frac{z + z^{-1}}{2}, \qquad \sin\theta = \frac{z - z^{-1}}{2i}, \qquad d\theta = \frac{dz}{iz}.$$
The integral becomes $\oint_{|z|=1} f(z)\, dz$ for some rational function $f$, evaluated by the residue theorem.

## Section 02: Improper Integrals of Rational Functions

For $\int_{-\infty}^\infty f(x)\, dx$ where $f$ is rational and has no real poles, close the contour with a large semicircle in the upper half-plane. The ML inequality shows the semicircular contribution vanishes as $R \to \infty$ (when $\deg q \geq \deg p + 2$), and the residue theorem gives the integral as $2\pi i$ times the sum of residues in the upper half-plane.

## Section 03: Jordan's Lemma

For integrals involving $e^{i\xi x}$ with $\xi > 0$, the standard ML estimate fails (the exponential $e^{i\xi z} = e^{-\xi y}e^{i\xi x}$ decays for $y > 0$ but the estimate requires more care). Jordan's lemma provides the needed bound: the integral of $e^{i\xi z} f(z)$ over the upper semicircle goes to zero if $\max|f(z)| \to 0$ on the semicircle.

## Section 04: Summation of Series

The function $\pi\cot(\pi z)$ has simple poles at every integer $n \in \mathbb{Z}$ with residue $1$. If $f$ is meromorphic with poles away from the integers, the residue theorem applied to a large square contour gives:
$$\sum_{n=-\infty}^{\infty} f(n) = -\sum [\text{residues of } \pi\cot(\pi z)f(z) \text{ at non-integer poles}].$$

## Learning Objectives

After this chapter, a student should be able to:

- Apply the unit circle substitution to evaluate $\int_0^{2\pi} R(\cos\theta, \sin\theta)\, d\theta$.
- Set up the semicircular contour for $\int_{-\infty}^\infty f(x)\, dx$ and verify the conditions for the ML estimate to apply.
- State Jordan's lemma and apply it to Fourier-type integrals.
- Use the cotangent summation formula to evaluate series $\sum f(n)$.
- Choose appropriate contours for integrals with branch cuts (keyhole contours) and poles on the real axis (indented contours).

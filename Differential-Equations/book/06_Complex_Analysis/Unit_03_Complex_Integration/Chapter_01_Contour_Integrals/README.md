# Chapter 01: Contour Integrals

A contour integral is an integral of a complex-valued function along a curve in the complex plane. The definition reduces to a real line integral via parametrization, so the technical machinery is essentially the same as in multivariable calculus. What is new is the complex structure: the integrand $f(z)\, dz$ carries information about both the function and the geometry of the curve, and the interplay between these gives contour integration its power.

## Section 01: Definition and Properties

The contour integral $\int_C f(z)\, dz$ of a continuous function $f$ along a smooth curve $C$ is defined by choosing a smooth parametrization $z(t)$, $t \in [a, b]$, and setting:
$$\int_C f(z)\, dz = \int_a^b f(z(t))\, z'(t)\, dt.$$
The right side is a standard (Riemann) integral of a complex-valued function, computed by integrating real and imaginary parts separately. The value is independent of the choice of parametrization (as long as orientation is preserved), which is proved by the chain rule.

Key properties: linearity in $f$; additivity over concatenations of curves; reversal of orientation negates the integral; the integral over a closed curve does not necessarily vanish (unlike the case of exact 1-forms in real calculus, where it depends on path-independence).

## Section 02: Parametrization

Effective computation of contour integrals requires skill in choosing and applying parametrizations. The most common curves are:
- Straight line segments from $z_1$ to $z_2$: parametrized by $z(t) = z_1 + t(z_2 - z_1)$, $t \in [0,1]$.
- Circular arcs: $z(t) = z_0 + re^{it}$, $t$ ranging over the appropriate angular interval.
- More general smooth paths, where the computation is an exercise in real calculus.

## Section 03: The Estimation Lemma

The estimation lemma (ML inequality) provides an upper bound for the modulus of a contour integral:
$$\left|\int_C f(z)\, dz\right| \leq ML,$$
where $M = \max_{z \in C} |f(z)|$ and $L$ is the arc length of $C$.

This bound is indispensable for proving convergence theorems (showing that integrals over auxiliary curves go to zero as radii go to infinity or to zero), which are the technical heart of the residue calculus in Unit 04.

## Learning Objectives

After this chapter, a student should be able to:

- Define the contour integral and verify that the definition is independent of parametrization.
- Parametrize standard curves and compute contour integrals directly.
- Apply the estimation lemma to bound integrals over circular arcs and line segments.
- Recognize which estimates are tight and which have significant slack.

# Chapter 1: Scalar Line Integrals

The simplest generalization of the one-variable integral to curves in space is the scalar line integral, which accumulates the values of a scalar function $f$ along a curve, weighted by arc length. This chapter defines the integral precisely, develops the computational formula based on parametrization, and works through the primary application: arc length and the mass of a thin wire.

## Chapter Overview

A **scalar line integral** integrates a function $f: D \subseteq \mathbb{R}^n \to \mathbb{R}$ over a curve $C \subset D$, measuring the accumulated value of $f$ as one travels along $C$, weighted by how much arc length is covered at each step. Formally, the integral $\int_C f\,ds$ is defined by a Riemann-sum process: subdivide the curve into small pieces, multiply the value of $f$ at a sample point in each piece by the length of that piece, and take the limit as the subdivision is refined.

The result is independent of both the parametrization of the curve and the direction of traversal — the integral measures the total "amount" of $f$ along $C$ as a geometric object, not as a directed path.

**Section 1: Definition and Computation** gives the parametric formula for $\int_C f\,ds$ and works through computational examples in two and three dimensions. The key step is always to reduce the line integral to an ordinary definite integral in the parameter $t$ via the substitution $ds = |\mathbf{r}'(t)|\,dt$.

**Section 2: Arc Length as a Line Integral** treats the special case $f \equiv 1$, recovering the arc length formula $L = \int_C ds = \int_a^b |\mathbf{r}'(t)|\,dt$. This makes explicit the connection between scalar line integrals and geometry.

## The Parametrization Approach

To compute $\int_C f\,ds$, choose a smooth parametrization $\mathbf{r}: [a, b] \to \mathbb{R}^n$ of $C$ with $\mathbf{r}'(t) \neq \mathbf{0}$ for all $t \in (a,b)$. Then

$$\int_C f\,ds = \int_a^b f(\mathbf{r}(t))\,|\mathbf{r}'(t)|\,dt.$$

The factor $|\mathbf{r}'(t)|$ is the speed at time $t$ — it converts the parameter increment $dt$ into an arc length increment $ds$. This formula is independent of the particular parametrization chosen: if you reparametrize the same curve with a different parameter, the factor $|\mathbf{r}'|$ changes in a way that exactly compensates, leaving the integral unchanged.

## Physical Interpretation

If $\rho(\mathbf{r})$ is the linear mass density (mass per unit length) of a thin wire bent into the shape of curve $C$, then

$$M = \int_C \rho\,ds$$

is the total mass of the wire. Similarly, the center of mass coordinates are

$$\bar{x} = \frac{1}{M}\int_C x\,\rho\,ds, \qquad \bar{y} = \frac{1}{M}\int_C y\,\rho\,ds, \qquad \bar{z} = \frac{1}{M}\int_C z\,\rho\,ds.$$

Other physical quantities — the moment of inertia of a wire about an axis, the total electric charge on a charged wire, the average temperature along a curve — all reduce to scalar line integrals of appropriate density functions.

## Symmetry

An important feature of the scalar line integral is that $\int_C f\,ds = \int_{-C} f\,ds$, where $-C$ denotes the curve traversed in the reverse direction. Because arc length is positive regardless of direction, the sign does not change. This contrasts sharply with vector line integrals (Chapter 2), where reversing direction changes the sign.

## Relationship to Single-Variable Calculus

When the curve $C$ is the segment $[a,b]$ on the $x$-axis, parametrized by $\mathbf{r}(t) = (t, 0)$ for $t \in [a,b]$, the scalar line integral reduces to the familiar definite integral: $\int_C f\,ds = \int_a^b f(t,0)\cdot 1\,dt$. All single-variable integrals are thus special cases of scalar line integrals.

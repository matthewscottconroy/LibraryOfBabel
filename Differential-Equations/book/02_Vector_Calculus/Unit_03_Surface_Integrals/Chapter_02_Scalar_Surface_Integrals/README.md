# Chapter 2: Scalar Surface Integrals

A scalar surface integral integrates a scalar function $f$ over a surface $S$, weighted by the area element $dS$. It is the natural two-dimensional analogue of the scalar line integral from Unit 2: just as $\int_C f\,ds$ sums up $f$ along a curve weighted by arc length, $\iint_S f\,dS$ sums up $f$ over a surface weighted by area. Applications include computing total mass, total charge, average temperature, and moments of inertia for thin shells (surfaces in three-dimensional space with negligible thickness).

## The Integral

Given a surface $S$ parametrized by $\mathbf{r}(u,v)$ on a domain $D$, and a continuous scalar function $f$ on $S$:

$$\iint_S f\,dS = \iint_D f(\mathbf{r}(u,v))\,|\mathbf{r}_u\times\mathbf{r}_v|\,du\,dv.$$

This reduces the surface integral to an ordinary double integral. The factor $|\mathbf{r}_u\times\mathbf{r}_v|$ is the local area distortion introduced by the parametrization.

**For graph surfaces $z = g(x,y)$:**

$$\iint_S f\,dS = \iint_{D_{xy}} f(x, y, g(x,y))\sqrt{1+g_x^2+g_y^2}\,dx\,dy.$$

## Physical Interpretation

If $\rho(\mathbf{p})$ is the surface mass density (mass per unit area) at the point $\mathbf{p} \in S$, then:

- **Total mass:** $M = \iint_S\rho\,dS$.
- **Center of mass:** $\bar{\mathbf{r}} = (1/M)\iint_S\mathbf{r}\,\rho\,dS$ (component by component).
- **Moment of inertia** about the $z$-axis: $I_z = \iint_S(x^2+y^2)\,\rho\,dS$.
- **Average value** of $f$ over $S$: $\langle f\rangle = (1/A(S))\iint_S f\,dS$.

## Key Properties

**Orientation independence.** Unlike flux integrals, scalar surface integrals do not depend on the orientation of the surface. Reversing the normal changes the sign of $d\mathbf{S}$ but leaves $dS = |d\mathbf{S}|$ unchanged.

**Parametrization independence.** The value of $\iint_S f\,dS$ is the same for any regular parametrization of $S$ (with any orientation), because the area element $|\mathbf{r}_u\times\mathbf{r}_v|\,du\,dv$ transforms correctly under reparametrization.

**Linearity and additivity.** $\iint_S(af+bg)\,dS = a\iint_S f\,dS + b\iint_S g\,dS$, and $\iint_{S_1\cup S_2} f\,dS = \iint_{S_1} f\,dS + \iint_{S_2} f\,dS$.

## Chapter Contents

**Section 1: Definition and Computation** works through the computational procedure in detail, with examples on spheres, cylinders, and cones. The section emphasizes the step-by-step process: (1) parametrize, (2) compute $\mathbf{r}_u\times\mathbf{r}_v$ and its magnitude, (3) substitute into the double integral, (4) evaluate.

**Section 2: Mass and Center of Mass of a Surface** develops the physical applications of the scalar surface integral. The analogy with one-dimensional mass-on-a-wire (using $\int\rho\,ds$) is made explicit, and examples with non-uniform density are worked through.

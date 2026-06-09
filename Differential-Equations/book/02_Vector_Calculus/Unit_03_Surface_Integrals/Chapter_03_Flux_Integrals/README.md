# Chapter 3: Flux Integrals

Imagine a fluid flowing through a permeable membrane. At each point of the membrane, some fluid passes through in the direction of the membrane's normal — the amount depends on both the fluid's speed and the angle between its velocity and the normal. The total volume of fluid crossing the membrane per unit time is the **flux**. Computing it requires a vector surface integral: the integral of the dot product of the vector field with the oriented surface normal, summed over the entire surface.

Flux integrals are the most important type of surface integral for physics and for the fundamental theorems of vector calculus. The Divergence Theorem equates the total flux out of a closed surface to the integral of divergence inside, and Stokes' Theorem uses flux of the curl through a surface.

## The Flux Integral

Given an oriented surface $S$ with unit normal $\hat{\mathbf{n}}$, the **flux** of a vector field $\mathbf{F}$ through $S$ is

$$\iint_S \mathbf{F}\cdot d\mathbf{S} = \iint_S \mathbf{F}\cdot\hat{\mathbf{n}}\,dS.$$

The vector area element is $d\mathbf{S} = \hat{\mathbf{n}}\,dS$, combining the scalar area element $dS$ with the orientation $\hat{\mathbf{n}}$.

In parametric form, with $d\mathbf{S} = (\mathbf{r}_u\times\mathbf{r}_v)\,du\,dv$:

$$\iint_S\mathbf{F}\cdot d\mathbf{S} = \iint_D \mathbf{F}(\mathbf{r}(u,v))\cdot(\mathbf{r}_u\times\mathbf{r}_v)\,du\,dv.$$

This formula is the key: the cross product $\mathbf{r}_u\times\mathbf{r}_v$ provides both the normal direction and the area scaling.

## Chapter Overview

**Section 1: Flux Through a Surface** defines the flux integral precisely, works through computation for planar, spherical, and cylindrical surfaces, and develops the intuition of flux as rate of passage through a surface.

**Section 2: Orientation and Sign Convention** addresses the dependence of flux on orientation. Reversing the normal direction reverses the sign. This section clarifies the conventions (outward for closed surfaces, right-hand rule for surfaces bounded by curves) that are required for the Divergence Theorem and Stokes' Theorem to have the correct signs.

**Section 3: Applications in Physics** applies flux integrals to Gauss's law (electric and gravitational), heat flux through surfaces, and fluid flow rate. These applications motivate why the Divergence Theorem is the natural tool for connecting local source strength (divergence) to global outflow (flux).

## Why Flux is Orientation-Dependent

The dot product $\mathbf{F}\cdot\hat{\mathbf{n}}$ changes sign when $\hat{\mathbf{n}}$ is replaced by $-\hat{\mathbf{n}}$. Physically: the same amount of fluid flowing from left to right is positive flux if the surface is oriented with its normal pointing right, and negative flux if the normal points left. The total flux through a closed surface (net outflow) is positive if more leaves than enters and negative if more enters than leaves — and this sign convention is crucial for the Divergence Theorem.

# Unit 2: Line Integrals

The definite integral $\int_a^b f(x)\,dx$ accumulates the values of $f$ along a segment of the real line. Line integrals extend this idea to curves in two or three dimensions, integrating either a scalar function or a vector field along a path. The two types of line integral — scalar and vector — measure physically distinct quantities, require different computational approaches, and connect to different aspects of the theory. Together they are the foundation for the integral theorems that close this module.

## Why Line Integrals?

The first motivation is purely geometric: the arc length of a curve is naturally expressed as a line integral of the constant function $f = 1$ with respect to arc length. Any scalar quantity distributed along a curve — mass, charge, heat — is recovered by integrating its density against arc length. This is the scalar line integral.

The second motivation is physical: the work done by a force field $\mathbf{F}$ on a particle moving along a curve $C$ is the accumulated dot product of $\mathbf{F}$ with the direction of motion, integrated over the length of the path. This is the vector line integral. Unlike the scalar integral, the vector integral depends on the direction of traversal and can be negative.

## Unit Structure

**Chapter 1: Scalar Line Integrals** develops the integral $\int_C f\,ds$ of a scalar function with respect to arc length. The key formula parametrizes the curve $C$ as $\mathbf{r}(t)$ for $t \in [a, b]$ and reduces the integral to an ordinary one:

$$\int_C f\,ds = \int_a^b f(\mathbf{r}(t))\,|\mathbf{r}'(t)|\,dt.$$

The factor $|\mathbf{r}'(t)|$ is the speed, converting the parameter $t$ into actual arc length. This chapter includes the important special case of arc length itself ($f \equiv 1$) and shows how to use scalar line integrals to find the mass and center of mass of a wire.

**Chapter 2: Vector Line Integrals** develops the work integral $\int_C \mathbf{F} \cdot d\mathbf{r}$, computed by the formula

$$\int_C \mathbf{F} \cdot d\mathbf{r} = \int_a^b \mathbf{F}(\mathbf{r}(t)) \cdot \mathbf{r}'(t)\,dt.$$

This integral changes sign when the direction of traversal is reversed, reflecting the directional nature of work. A key topic here is circulation — the vector line integral around a closed curve — which will be central to both Green's Theorem and Stokes' Theorem.

**Chapter 3: The Fundamental Theorem for Line Integrals** is the first of the great generalizations of the one-dimensional Fundamental Theorem of Calculus. It states that if $\mathbf{F} = \nabla f$ is conservative, then

$$\int_C \mathbf{F} \cdot d\mathbf{r} = f(\mathbf{r}(b)) - f(\mathbf{r}(a)),$$

regardless of the path $C$ from $\mathbf{r}(a)$ to $\mathbf{r}(b)$. The chapter proves this theorem, establishes the equivalence between conservativity, path independence, and vanishing circulation, and examines the topological condition (simply connected domains) under which the curl test completely characterizes conservative fields.

## Connections to Later Material

Line integrals are the boundary data for the higher-dimensional integral theorems. Green's Theorem equates a double integral over a region $D$ to a line integral around its boundary $\partial D$. Stokes' Theorem equates a surface integral (of the curl of $\mathbf{F}$) to the line integral of $\mathbf{F}$ around the boundary of the surface. Understanding how to compute and interpret line integrals is therefore not just a unit goal — it is prerequisite for comprehending the fundamental theorems.

## Prerequisites

You should be comfortable with parametric curves: writing a circle, line segment, or helix as $\mathbf{r}(t)$, computing $\mathbf{r}'(t)$, and evaluating $|\mathbf{r}'(t)|$. You should also know the arc length formula from single-variable calculus. The material in Unit 1 (vector fields, conservative fields, potential functions) is used throughout.

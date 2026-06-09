# Chapter 2: Vector Line Integrals

When a force field $\mathbf{F}$ acts on a particle moving along a curve $C$, the work done is not simply the product of force magnitude and distance — it depends on the angle between the force and the direction of motion at each point. The vector line integral $\int_C \mathbf{F} \cdot d\mathbf{r}$ is precisely the tool for computing this accumulated dot product. Unlike the scalar line integral, which is direction-independent, the vector line integral is sensitive to the orientation of the curve: reversing the direction of traversal changes the sign.

## Chapter Overview

This chapter develops vector line integrals in two sections.

**Section 1: Work Integrals** defines $\int_C \mathbf{F} \cdot d\mathbf{r}$ through the parametric formula

$$\int_C \mathbf{F} \cdot d\mathbf{r} = \int_a^b \mathbf{F}(\mathbf{r}(t)) \cdot \mathbf{r}'(t)\,dt,$$

computes it for a range of vector fields and curves, and establishes its basic properties (linearity, additivity, sign reversal). The interpretation as work done by a force field is developed carefully, and the connection to the scalar line integral via $\int_C \mathbf{F} \cdot d\mathbf{r} = \int_C (\mathbf{F} \cdot \hat{\mathbf{T}})\,ds$ (where $\hat{\mathbf{T}}$ is the unit tangent) is made explicit.

**Section 2: Circulation** studies the special case where $C$ is a closed curve, so $\oint_C \mathbf{F} \cdot d\mathbf{r}$ measures the net tendency of $\mathbf{F}$ to circulate around the loop. Circulation is the quantity related to curl by Stokes' Theorem: the curl at a point is the circulation per unit area of infinitesimal loops encircling that point. This section develops the physical interpretation and prepares for Green's Theorem (Unit 4, Chapter 1) and Stokes' Theorem (Unit 4, Chapter 2).

## The Key Formula

In components, if $\mathbf{F} = P\,\mathbf{i} + Q\,\mathbf{j} + R\,\mathbf{k}$ and $\mathbf{r}(t) = (x(t), y(t), z(t))$, then

$$\int_C \mathbf{F} \cdot d\mathbf{r} = \int_a^b (P\,x' + Q\,y' + R\,z')\,dt = \int_C P\,dx + Q\,dy + R\,dz.$$

The differential form $P\,dx + Q\,dy + R\,dz$ is called a **1-form** or **line element**, and its integral over $C$ is the vector line integral. This notation is especially useful for piecewise-smooth curves and for connecting to the theory of differential forms in Unit 4, Chapter 4.

## Why the Sign Changes with Direction

If $-C$ denotes the curve traversed in the opposite direction, then $\int_{-C}\mathbf{F}\cdot d\mathbf{r} = -\int_C\mathbf{F}\cdot d\mathbf{r}$. The physical reason: work done by a force moving a particle from $A$ to $B$ is the negative of the work done moving it from $B$ to $A$. Lifting a book upward against gravity does positive work; lowering it does negative work (the gravitational force and displacement are anti-parallel).

## Relation to Conservative Fields

For a conservative field $\mathbf{F} = \nabla f$, the work integral from $A$ to $B$ equals $f(B) - f(A)$ regardless of path. This Fundamental Theorem for Line Integrals (proved in Chapter 3) is what makes conservative fields so computationally tractable. For non-conservative fields, the work integral genuinely depends on the path, and one must parametrize and integrate directly.

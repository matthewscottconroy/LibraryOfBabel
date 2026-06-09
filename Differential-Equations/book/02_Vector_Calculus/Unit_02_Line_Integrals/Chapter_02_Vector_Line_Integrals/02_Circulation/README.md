# Circulation

When a fluid flows in a closed loop — like water circling a drain or air swirling in a cyclone — there is a net rotational motion around the loop. The **circulation** of a vector field around a closed curve is the line integral that measures this net rotational tendency: it sums up the component of the field tangent to the loop at each point. Positive circulation means the field tends to push fluid around the loop in the direction of traversal; negative circulation means it resists that traversal; zero circulation means the rotational contributions cancel.

Circulation is the central quantity in Green's Theorem and Stokes' Theorem. Understanding it deeply is essential for understanding why curl and circulation are related, and how local rotation determines global circulation.

## Definition

Let $\mathbf{F}: D \to \mathbb{R}^n$ be a continuous vector field and $C$ a smooth **closed** oriented curve in $D$ (so the start and end points coincide). The **circulation** of $\mathbf{F}$ around $C$ is

$$\oint_C \mathbf{F} \cdot d\mathbf{r}.$$

The symbol $\oint$ (rather than $\int$) indicates that the curve is closed. The value is computed by the same parametric formula as any other line integral: choose any orientation-preserving parametrization $\mathbf{r}: [a,b] \to D$ with $\mathbf{r}(a) = \mathbf{r}(b)$, and compute $\int_a^b \mathbf{F}(\mathbf{r}(t)) \cdot \mathbf{r}'(t)\,dt$.

## Physical Interpretation

In fluid mechanics, if $\mathbf{F} = \mathbf{v}$ is the velocity field of a fluid, the circulation $\oint_C \mathbf{v}\cdot d\mathbf{r}$ around a closed loop $C$ measures the net flow of fluid around the loop. More precisely, it is the line integral of the fluid's speed in the direction of the loop's tangent: a high value means the fluid races around the loop; a value near zero means fluid crosses the loop without circulating.

In meteorology, atmospheric circulation is measured this way. A cyclone has large positive circulation (counterclockwise in the Northern Hemisphere) around a loop encircling its center. An anticyclone has large negative circulation (clockwise). The circulation around a loop that avoids the storm center can be small even in a storm-affected region.

## Relation to Curl

The curl at a point $\mathbf{p}$ is related to circulation by:

$$(\nabla \times \mathbf{F})(\mathbf{p}) \cdot \hat{\mathbf{n}} = \lim_{A \to 0} \frac{1}{A}\oint_{C_A} \mathbf{F}\cdot d\mathbf{r},$$

where $C_A$ is a loop of area $A$ encircling $\mathbf{p}$, oriented by the right-hand rule with respect to $\hat{\mathbf{n}}$, and the limit is taken as the loop shrinks to $\mathbf{p}$. In other words, **curl is circulation per unit area**. This is both the geometric definition of curl (coordinate-independent) and the intuition behind Stokes' Theorem: the curl flux through a surface equals the circulation around the surface's boundary.

## Worked Examples

**Example 1: Circulation of a rotational field.** Let $\mathbf{F}(x,y) = -y\,\mathbf{i} + x\,\mathbf{j}$ and $C$ the circle of radius $r$ traversed counterclockwise.

$\mathbf{r}(t) = (r\cos t, r\sin t)$, $\mathbf{r}'(t) = (-r\sin t, r\cos t)$, $\mathbf{F}(\mathbf{r}(t)) = (-r\sin t, r\cos t)$.

$$\oint_C \mathbf{F}\cdot d\mathbf{r} = \int_0^{2\pi}(-r\sin t, r\cos t)\cdot(-r\sin t, r\cos t)\,dt = \int_0^{2\pi} r^2\,dt = 2\pi r^2.$$

The circulation grows with $r^2$, which is consistent with $\nabla\times\mathbf{F} = 2\,\mathbf{k}$ (constant curl 2): by Stokes' Theorem, the circulation equals the curl flux through the disk, which is $2 \times \pi r^2 = 2\pi r^2$.

**Example 2: Circulation of a conservative field.** For any conservative field $\mathbf{F} = \nabla f$, the circulation around any closed curve is zero:

$$\oint_C \mathbf{F}\cdot d\mathbf{r} = f(\mathbf{r}(b)) - f(\mathbf{r}(a)) = f(\mathbf{r}(a)) - f(\mathbf{r}(a)) = 0.$$

This is a direct consequence of the Fundamental Theorem for Line Integrals and characterizes conservative fields: a field is conservative if and only if its circulation around every closed curve (in a simply connected domain) is zero.

**Example 3: The vortex field.** Let $\mathbf{F}(x,y) = \frac{-y}{x^2+y^2}\,\mathbf{i} + \frac{x}{x^2+y^2}\,\mathbf{j}$ (defined on $\mathbb{R}^2 \setminus \{(0,0)\}$) and $C$ the unit circle.

We computed in Section 3 of Chapter 1 that $\oint_C \mathbf{F}\cdot d\mathbf{r} = 2\pi$. This field has zero curl everywhere on its domain, but non-zero circulation around any loop encircling the origin. The non-zero circulation is not detected by the local curl because the curl test reflects local rotation — and the "rotation" in this field is concentrated at the singular origin, which is outside the domain.

## Circulation and Stokes' Theorem

Stokes' Theorem (Unit 4, Chapter 2) states:

$$\oint_C \mathbf{F}\cdot d\mathbf{r} = \iint_S (\nabla\times\mathbf{F})\cdot d\mathbf{S},$$

where $S$ is any oriented surface bounded by $C$ with compatible orientation. This converts a line integral (circulation) into a surface integral (curl flux). For planar curves $C$ bounding a planar region $D$, this reduces to Green's Theorem:

$$\oint_C P\,dx + Q\,dy = \iint_D\left(\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}\right)dA.$$

In both cases, the local rotation (curl) throughout the region aggregates to give the global circulation around the boundary.

## Kelvin's Circulation Theorem

In ideal fluid mechanics (inviscid, incompressible), **Kelvin's circulation theorem** states that the circulation around any closed material curve (one that moves with the fluid) is constant in time:

$$\frac{d}{dt}\oint_{C(t)}\mathbf{v}\cdot d\mathbf{r} = 0.$$

This is a consequence of Euler's equations and the Stokes theorem applied to the vorticity $\boldsymbol{\omega} = \nabla\times\mathbf{v}$. It means that an initially irrotational ideal fluid remains irrotational, and that vorticity is "frozen in" to the fluid (Helmholtz's vortex theorems).

## Signed Circulation and Orientation

The sign of circulation depends on the choice of orientation for $C$. Reversing the orientation reverses the sign. By convention, loops in the plane are taken counterclockwise (positive orientation) when using Green's Theorem, and surfaces are oriented consistently with their boundary loops by the right-hand rule. These conventions are not arbitrary — they are required for the integral theorems to hold with the correct signs.

## Summary

Circulation $\oint_C\mathbf{F}\cdot d\mathbf{r}$ measures the net tendency of a vector field to travel around a closed loop in the direction of traversal. It is zero for conservative fields on any loop, nonzero for non-conservative fields or when the loop encloses a topological obstruction. The curl is the local version of circulation (circulation per unit area), and Stokes' Theorem relates the two globally. Circulation is one of the fundamental quantities of fluid mechanics and electrodynamics.

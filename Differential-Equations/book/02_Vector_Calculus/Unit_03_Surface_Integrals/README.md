# Unit 3: Surface Integrals

Curves in space are one-dimensional objects; surfaces are two-dimensional ones. Just as line integrals extend the one-variable definite integral to curves, surface integrals extend double integrals to curved surfaces embedded in three-dimensional space. These integrals are essential for computing physical quantities that are distributed over surfaces — the total mass of a thin shell, the flux of heat or electric field through a membrane, the net flow of fluid through a pipe's cross-section.

## The Challenge of Surfaces

The essential complication of surface integrals, compared with double integrals, is that the domain of integration is a curved surface rather than a flat rectangle or region in the $xy$-plane. To integrate over a surface, we must first describe it mathematically — this is the **parametrization** problem — and then account for how the surface is oriented relative to the integrand.

A surface in $\mathbb{R}^3$ can be described in several ways: as the graph $z = g(x,y)$ of a function (a **graph surface**), as the zero set $F(x,y,z) = c$ of a function (a **level surface**), or as the image $\mathbf{r}(u,v) = (x(u,v), y(u,v), z(u,v))$ of a region $D$ in the $uv$-plane (a **parametric surface**). Each description has its advantages. Parametric surfaces are the most general and subsume the others; the theory is built in this generality.

## Unit Structure

**Unit 3, Chapter 1: Parametric Surfaces** develops the machinery for describing and working with parametric surfaces. The key geometric objects are the tangent vectors $\mathbf{r}_u = \partial\mathbf{r}/\partial u$ and $\mathbf{r}_v = \partial\mathbf{r}/\partial v$, and their cross product $\mathbf{r}_u\times\mathbf{r}_v$, which gives both a normal vector to the surface and the infinitesimal area element $dS = |\mathbf{r}_u\times\mathbf{r}_v|\,du\,dv$. Surface area and orientation are developed here.

**Unit 3, Chapter 2: Scalar Surface Integrals** defines $\iint_S f\,dS$ for a scalar function $f$ on a surface $S$. The formula reduces to a double integral over the parameter domain via the area element $dS$. Applications include computing the total mass and center of mass of a thin curved shell with given surface mass density.

**Unit 3, Chapter 3: Flux Integrals** defines the vector surface integral $\iint_S\mathbf{F}\cdot d\mathbf{S}$, which measures the net flux of a vector field through the surface. Here orientation is crucial: $d\mathbf{S} = (\mathbf{r}_u\times\mathbf{r}_v)\,du\,dv$ is a vector area element pointing in the normal direction, and the sign of the flux depends on whether $\mathbf{F}$ points with or against the chosen normal. This unit closes with applications in physics: Gauss's law for flux of $\mathbf{E}$, heat flux through a surface, and fluid volume flow rate.

## Why Surface Integrals Matter

Surface integrals are the natural domain of the Divergence Theorem (Unit 4, Chapter 3): that theorem relates the flux of $\mathbf{F}$ through a closed surface to the divergence of $\mathbf{F}$ within the enclosed volume. Similarly, Stokes' Theorem relates the circulation of $\mathbf{F}$ around a closed curve to the flux of the curl $\nabla\times\mathbf{F}$ through any surface bounded by that curve. Without facility with surface integrals, neither of these fundamental theorems can be stated or applied.

In physics, flux integrals appear in: Gauss's law ($\iint_S\mathbf{E}\cdot d\mathbf{S} = Q_{\text{enc}}/\varepsilon_0$), Faraday's law of induction, heat flow, probability flux in quantum mechanics, and the analysis of fluid flow through cross-sections.

## Prerequisites

This unit uses double integrals freely. You should be comfortable setting up and evaluating $\iint_D f(x,y)\,dA$ over regions in the plane. The material on parametric curves (arc length, unit tangent) from Unit 2 provides intuition for the analogous surface constructions.

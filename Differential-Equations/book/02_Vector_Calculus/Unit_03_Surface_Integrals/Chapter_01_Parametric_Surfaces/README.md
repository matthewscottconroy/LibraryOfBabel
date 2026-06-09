# Chapter 1: Parametric Surfaces

A parametric curve assigns a point in $\mathbb{R}^3$ to each value of a single parameter $t$. A parametric surface assigns a point in $\mathbb{R}^3$ to each pair of parameters $(u, v)$. The image of the parameter domain — a region $D$ in the $uv$-plane — is a two-dimensional surface in three-dimensional space. This chapter builds the foundation for integrating over such surfaces: how to compute the normal vector at each point, how to measure surface area, and what it means to orient a surface.

## Chapter Overview

**Section 1: Parametrization of Surfaces** develops the definition and the fundamental tool: the tangent vectors $\mathbf{r}_u$ and $\mathbf{r}_v$ and their cross product. The cross product $\mathbf{r}_u \times \mathbf{r}_v$ is normal to the surface and its magnitude equals the area of the infinitesimal parallelogram spanned by $\mathbf{r}_u$ and $\mathbf{r}_v$ — making it the natural area element for integration.

**Section 2: Surface Area** uses the area element $dS = |\mathbf{r}_u\times\mathbf{r}_v|\,du\,dv$ to compute the total area of a surface. This is the two-dimensional analogue of arc length. Examples include spheres, cones, and graphs of functions.

**Section 3: Normal Vectors and Orientation** addresses the sign and choice of normal. A smooth surface in $\mathbb{R}^3$ has two distinct normal directions at each point (inward and outward, top and bottom, left and right). Choosing one consistently across the surface is called **orienting** the surface. Not all surfaces can be oriented — the Möbius strip is the classic non-orientable example. For flux integrals, the orientation determines the sign: an outward-pointing normal gives positive flux for a field pointing outward.

## Why Parametrization is Necessary

When a surface curves through space, there is no universal formula for integrating over it. The parametrization $\mathbf{r}(u,v)$ converts the integral over the surface into an ordinary double integral over the parameter domain $D$ — a flat region in the plane. The cross product $\mathbf{r}_u\times\mathbf{r}_v$ acts as the distortion factor, analogously to $|\mathbf{r}'(t)|$ for curves, accounting for how the parametrization stretches or compresses the surface.

Different parametrizations of the same surface give the same integrals (up to sign, depending on orientation). This invariance is guaranteed by the change of variables theorem for double integrals, applied to the change of parameters.

## Key Parametrizations to Know

Several surface parametrizations appear repeatedly:

**Graph surface** $z = g(x,y)$: Use $\mathbf{r}(x,y) = (x, y, g(x,y))$. Then $\mathbf{r}_x = (1, 0, g_x)$, $\mathbf{r}_y = (0, 1, g_y)$, and $\mathbf{r}_x\times\mathbf{r}_y = (-g_x, -g_y, 1)$, giving $dS = \sqrt{1 + g_x^2 + g_y^2}\,dx\,dy$.

**Sphere of radius $a$**: Use $\mathbf{r}(\theta,\phi) = (a\sin\phi\cos\theta, a\sin\phi\sin\theta, a\cos\phi)$, where $\phi \in [0,\pi]$ is the polar angle and $\theta \in [0,2\pi]$ is the azimuthal angle. Then $dS = a^2\sin\phi\,d\phi\,d\theta$.

**Cylinder of radius $a$, height $h$**: $\mathbf{r}(\theta, z) = (a\cos\theta, a\sin\theta, z)$, $\theta \in [0,2\pi]$, $z \in [0,h]$. Then $dS = a\,d\theta\,dz$.

These parametrizations form the basic toolkit for surface integral computation.

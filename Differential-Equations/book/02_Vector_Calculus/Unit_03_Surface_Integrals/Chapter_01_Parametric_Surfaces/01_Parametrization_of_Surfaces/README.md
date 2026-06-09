# Parametrization of Surfaces

A sphere, a cylinder, a saddle surface, the graph of $z = xy$ — each of these is a two-dimensional curved object living in three-dimensional space. To integrate over any of them, we need a systematic way to describe them mathematically and to measure infinitesimal pieces of them. The parametrization of surfaces provides exactly this: a map from a flat parameter domain into three-dimensional space whose image is the desired surface.

## Definition

Let $D$ be a connected closed region in the $uv$-plane. A **parametric surface** is a function $\mathbf{r}: D \to \mathbb{R}^3$ of the form

$$\mathbf{r}(u, v) = (x(u,v),\, y(u,v),\, z(u,v)),$$

where the component functions $x, y, z$ are $C^1$ on $D$. The image $\mathbf{r}(D) = S \subset \mathbb{R}^3$ is the surface.

A parametrization is **regular** (or **smooth**) at a point $(u_0, v_0)$ if the tangent vectors

$$\mathbf{r}_u = \frac{\partial\mathbf{r}}{\partial u} = \left(\frac{\partial x}{\partial u}, \frac{\partial y}{\partial u}, \frac{\partial z}{\partial u}\right), \quad \mathbf{r}_v = \frac{\partial\mathbf{r}}{\partial v} = \left(\frac{\partial x}{\partial v}, \frac{\partial y}{\partial v}, \frac{\partial z}{\partial v}\right)$$

are linearly independent at $(u_0, v_0)$, i.e., $\mathbf{r}_u \times \mathbf{r}_v \neq \mathbf{0}$. At a regular point, the vectors $\mathbf{r}_u$ and $\mathbf{r}_v$ span the **tangent plane** to the surface at $\mathbf{r}(u_0, v_0)$.

## The Cross Product and Area Element

The cross product $\mathbf{r}_u \times \mathbf{r}_v$ is perpendicular to both $\mathbf{r}_u$ and $\mathbf{r}_v$, hence normal to the tangent plane — and therefore normal to the surface. Its magnitude

$$|\mathbf{r}_u \times \mathbf{r}_v| = |\mathbf{r}_u||\mathbf{r}_v|\sin\theta,$$

where $\theta$ is the angle between $\mathbf{r}_u$ and $\mathbf{r}_v$, equals the area of the parallelogram spanned by $\mathbf{r}_u$ and $\mathbf{r}_v$. This parallelogram approximates the image of the small rectangle $[u, u+du]\times[v, v+dv]$ under $\mathbf{r}$. The **area element** is therefore

$$dS = |\mathbf{r}_u \times \mathbf{r}_v|\,du\,dv.$$

## Standard Parametrizations

**Graph surfaces.** If $S$ is the graph of $z = g(x,y)$ over a domain $D_{xy}$, use $\mathbf{r}(x,y) = (x, y, g(x,y))$ with $u = x$, $v = y$:

$\mathbf{r}_x = (1, 0, g_x)$, $\mathbf{r}_y = (0, 1, g_y)$.

$\mathbf{r}_x \times \mathbf{r}_y = \begin{vmatrix}\mathbf{i} & \mathbf{j} & \mathbf{k} \\ 1 & 0 & g_x \\ 0 & 1 & g_y\end{vmatrix} = (-g_x)\mathbf{i} + (-g_y)\mathbf{j} + (1)\mathbf{k} = (-g_x, -g_y, 1)$.

$|\mathbf{r}_x\times\mathbf{r}_y| = \sqrt{1 + g_x^2 + g_y^2}$, so $dS = \sqrt{1 + g_x^2 + g_y^2}\,dx\,dy$.

**Sphere of radius $a$.** Spherical coordinates: $\mathbf{r}(\phi,\theta) = (a\sin\phi\cos\theta, a\sin\phi\sin\theta, a\cos\phi)$, $\phi \in [0,\pi]$, $\theta \in [0,2\pi]$.

$\mathbf{r}_\phi = a(\cos\phi\cos\theta, \cos\phi\sin\theta, -\sin\phi)$,
$\mathbf{r}_\theta = a(-\sin\phi\sin\theta, \sin\phi\cos\theta, 0)$.

Computing the cross product (a good exercise):

$\mathbf{r}_\phi\times\mathbf{r}_\theta = a^2(\sin^2\phi\cos\theta, \sin^2\phi\sin\theta, \sin\phi\cos\phi)$.

$|\mathbf{r}_\phi\times\mathbf{r}_\theta| = a^2\sin\phi$. (Verified: $a^4(\sin^4\phi + \sin^2\phi\cos^2\phi) = a^4\sin^2\phi$, so $|\cdot| = a^2\sin\phi$.)

$dS = a^2\sin\phi\,d\phi\,d\theta$.

**Cylinder of radius $a$, $z \in [0,h]$.** $\mathbf{r}(\theta,z) = (a\cos\theta, a\sin\theta, z)$.

$\mathbf{r}_\theta = (-a\sin\theta, a\cos\theta, 0)$, $\mathbf{r}_z = (0,0,1)$.

$\mathbf{r}_\theta\times\mathbf{r}_z = (a\cos\theta, a\sin\theta, 0)$.

$|\mathbf{r}_\theta\times\mathbf{r}_z| = a$. $dS = a\,d\theta\,dz$.

**Cone.** $z = \sqrt{x^2+y^2}$, parametrize by $\mathbf{r}(r,\theta) = (r\cos\theta, r\sin\theta, r)$, $r \geq 0$.

$\mathbf{r}_r = (\cos\theta, \sin\theta, 1)$, $\mathbf{r}_\theta = (-r\sin\theta, r\cos\theta, 0)$.

$\mathbf{r}_r\times\mathbf{r}_\theta = (-r\cos\theta, -r\sin\theta, r)$.

$|\mathbf{r}_r\times\mathbf{r}_\theta| = r\sqrt{2}$. $dS = r\sqrt{2}\,dr\,d\theta$.

## The Tangent Plane

At a regular point $\mathbf{r}(u_0, v_0) = \mathbf{p}$, the tangent plane to the surface is the plane through $\mathbf{p}$ containing the vectors $\mathbf{r}_u(u_0,v_0)$ and $\mathbf{r}_v(u_0,v_0)$:

$$\{\mathbf{p} + s\,\mathbf{r}_u + t\,\mathbf{r}_v : s, t \in \mathbb{R}\}.$$

Equivalently, the tangent plane is the set of points $(x,y,z)$ satisfying $(\mathbf{r}_u\times\mathbf{r}_v)\cdot(\mathbf{r} - \mathbf{p}) = 0$.

**Example.** For the sphere $\mathbf{r}(\phi,\theta)$ of radius $a$ at the point $(\phi_0, \theta_0) = (\pi/2, 0)$, the point is $\mathbf{p} = (a,0,0)$. The normal is $\mathbf{r}_\phi\times\mathbf{r}_\theta = a^2(0, 0, 0)$... let me be careful: at $\phi=\pi/2$, $\theta=0$: $\mathbf{r}_\phi\times\mathbf{r}_\theta = a^2(\sin^2(\pi/2)\cos 0, \sin^2(\pi/2)\sin 0, \sin(\pi/2)\cos(\pi/2)) = a^2(1, 0, 0)$. The tangent plane at $(a,0,0)$ has normal $(1,0,0)$, i.e., the plane $x = a$ — which is indeed tangent to the unit sphere at its rightmost point.

## Singular Points

At points where $\mathbf{r}_u \times \mathbf{r}_v = \mathbf{0}$, the parametrization is singular. These are typically isolated points (like the poles of a sphere parametrized by spherical coordinates, where $\sin\phi = 0$) and usually cause no difficulty in integration (since a set of measure zero does not affect the integral).

## Independence of Parametrization

If $\mathbf{r}_1(u,v)$ and $\mathbf{r}_2(s,t)$ are two regular parametrizations of the same surface $S$, related by a change of variables $(u,v) = \boldsymbol{\psi}(s,t)$ with positive Jacobian, then $|\mathbf{r}_{1,u}\times\mathbf{r}_{1,v}|\,du\,dv = |\mathbf{r}_{2,s}\times\mathbf{r}_{2,t}|\,ds\,dt$ by the change-of-variables theorem for double integrals. Surface area and scalar surface integrals are therefore independent of the parametrization.

## Summary

A parametric surface $\mathbf{r}(u,v)$ maps a flat parameter domain into $\mathbb{R}^3$. The tangent vectors $\mathbf{r}_u, \mathbf{r}_v$ span the tangent plane; their cross product $\mathbf{r}_u\times\mathbf{r}_v$ is the normal vector and its magnitude gives the infinitesimal area element $dS = |\mathbf{r}_u\times\mathbf{r}_v|\,du\,dv$. Standard parametrizations for graphs, spheres, cylinders, and cones are essential computational tools and should be memorized or reconstructed fluently.

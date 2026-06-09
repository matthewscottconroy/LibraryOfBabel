# General Change of Variables

The change-of-variables formula in $\mathbb{R}^3$ generalizes the single-variable substitution rule and extends the two-dimensional formula to three dimensions. Just as in two dimensions, the key is the Jacobian determinant: the absolute value $|\det J_\mathbf{g}|$ of the Jacobian of the coordinate transformation measures how much volume is stretched or compressed by the change of coordinates. Cylindrical and spherical coordinates are both special cases, with Jacobian determinants $r$ and $\rho^2\sin\phi$ respectively.

## The General Formula

**Theorem.** Let $\mathbf{g}: E^*\subseteq\mathbb{R}^3\to E\subseteq\mathbb{R}^3$ be a $C^1$ bijection with nonzero Jacobian determinant except possibly on a set of zero volume. Write $(x,y,z) = \mathbf{g}(u,v,w) = (g_1, g_2, g_3)$. Then for any continuous $f: E\to\mathbb{R}$:

$$\iiint_E f(x,y,z)\,dx\,dy\,dz = \iiint_{E^*} f(\mathbf{g}(u,v,w))\,\left|\frac{\partial(x,y,z)}{\partial(u,v,w)}\right|\,du\,dv\,dw,$$

where

$$\frac{\partial(x,y,z)}{\partial(u,v,w)} = \det\begin{pmatrix}\partial x/\partial u & \partial x/\partial v & \partial x/\partial w \\ \partial y/\partial u & \partial y/\partial v & \partial y/\partial w \\ \partial z/\partial u & \partial z/\partial v & \partial z/\partial w\end{pmatrix} = \det J_\mathbf{g}.$$

## Verification for Cylindrical Coordinates

$(x,y,z) = \mathbf{g}(r,\theta,z) = (r\cos\theta, r\sin\theta, z)$.

$J_\mathbf{g} = \begin{pmatrix}\cos\theta & -r\sin\theta & 0 \\ \sin\theta & r\cos\theta & 0 \\ 0 & 0 & 1\end{pmatrix}$, $\det J_\mathbf{g} = r$.

Volume element: $dV = |r|\,dr\,d\theta\,dz = r\,dr\,d\theta\,dz$ (since $r\geq 0$).

## Verification for Spherical Coordinates

$(x,y,z) = \mathbf{g}(\rho,\theta,\phi) = (\rho\sin\phi\cos\theta, \rho\sin\phi\sin\theta, \rho\cos\phi)$.

Computing $\det J_\mathbf{g}$ (by expansion along the third column or by cofactors):

$\det J_\mathbf{g} = \rho^2\sin\phi$.

Volume element: $dV = \rho^2|\sin\phi|\,d\rho\,d\theta\,d\phi = \rho^2\sin\phi\,d\rho\,d\theta\,d\phi$ (since $\sin\phi\geq 0$ for $\phi\in[0,\pi]$).

## Custom Coordinate Systems

The power of the general formula is that it works for any smooth bijection, not just the standard ones. This is useful when the domain has an unusual shape that fits naturally into some non-standard coordinates.

**Example: Ellipsoidal coordinates.** The region $E = \{x^2/a^2+y^2/b^2+z^2/c^2\leq 1\}$ (solid ellipsoid) has complicated description in Cartesian but simple description under the transformation $x = ar\sin\phi\cos\theta$, $y = br\sin\phi\cos\theta$, $z = cr\cos\phi$ with $0\leq r\leq 1$. The Jacobian is $abc r^2\sin\phi$, and the volume element is $dV = abc r^2\sin\phi\,dr\,d\theta\,d\phi$. The volume of the ellipsoid:

$V = \int_0^{2\pi}\int_0^{\pi}\int_0^1 abcr^2\sin\phi\,dr\,d\phi\,d\theta = abc\cdot\frac{4\pi}{3}.$

This is the correct formula for the volume of an ellipsoid with semi-axes $a$, $b$, $c$: $V = \frac{4\pi abc}{3}$.

**Example: Parabolic coordinates** in $\mathbb{R}^2$: $x = \frac{u^2-v^2}{2}$, $y = uv$. The Jacobian is $u^2+v^2$. These coordinates are natural for problems with parabolic boundary conditions, appearing in quantum mechanics (hydrogen atom in a uniform field) and fluid dynamics.

## Why the Absolute Value

The Jacobian determinant can be negative if the transformation reverses orientation. The volume element must be positive (volume is non-negative), so one takes the absolute value. An orientation-preserving transformation has $\det J_\mathbf{g} > 0$; an orientation-reversing transformation has $\det J_\mathbf{g} < 0$. For purposes of integration, only $|\det J_\mathbf{g}|$ matters.

## The Inverse Relationship

If $\mathbf{g}: E^*\to E$ and $\mathbf{h} = \mathbf{g}^{-1}: E\to E^*$, then $\det J_\mathbf{h} = 1/\det J_\mathbf{g}$ (by the chain rule applied to $\mathbf{g}\circ\mathbf{h} = \text{id}$). So the two change-of-variables formulas (using $\mathbf{g}$ and using $\mathbf{h}$) are consistent.

## Connection to the Inverse Function Theorem

The change-of-variables formula requires $\mathbf{g}$ to be a bijection with nonzero Jacobian (except on a zero-measure set). The inverse function theorem (Unit 2, Chapter 7) guarantees that if $\det J_\mathbf{g} \neq 0$ everywhere on $E^*$, then $\mathbf{g}$ is locally bijective. Global bijectivity requires additional assumptions (e.g., $\mathbf{g}$ is proper and simply connected domain). The formula is typically applied in cases where bijectivity is geometrically obvious (like the coordinate transformations above), where the theorem's hypotheses are clearly satisfied.

## Worked Example: Custom Transformation

Evaluate $\iiint_E xyz\,dV$ where $E = \{1\leq xy\leq 4, 0\leq xz\leq 2, 0\leq y/x\leq 1, x>0\}$.

Substitute $u = xy$, $v = xz$, $w = y/x$ (so $x = \sqrt{u/w}$, $y = \sqrt{uw}$, $z = v/\sqrt{u/w} = v\sqrt{w/u}$).

$E^* = \{1\leq u\leq 4, 0\leq v\leq 2, 0\leq w\leq 1\}$.

$xyz = \sqrt{u/w}\cdot\sqrt{uw}\cdot v\sqrt{w/u} = u^{1/2-1/2}\cdot w^{-1/2+1/2}\cdot v\cdot\sqrt{wu/u}$... this becomes cleaner by computing: $xyz = \sqrt{u/w}\cdot\sqrt{uw}\cdot v\sqrt{w/u} = v\cdot\sqrt{(u/w)(uw)(w/u)} = v\cdot\sqrt{w}$... let's just note that for such custom substitutions, computing the Jacobian explicitly is essential. The key point is that the general formula handles any smooth bijection.

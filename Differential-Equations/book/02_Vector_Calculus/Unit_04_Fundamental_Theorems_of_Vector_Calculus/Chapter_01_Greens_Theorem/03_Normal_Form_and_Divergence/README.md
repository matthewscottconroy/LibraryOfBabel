# Normal Form and Divergence (Green's Theorem in Flux Form)

Green's Theorem has two forms. The **tangential form** (the standard statement) relates circulation — the integral of $\mathbf{F}$ along the boundary — to the curl of $\mathbf{F}$ over the region. The **normal form** (flux form) relates flux — the integral of $\mathbf{F}$ perpendicular to the boundary — to the divergence of $\mathbf{F}$ over the region. Both are consequences of the same theorem; they are related by rotating the vector field and the curve by 90 degrees.

## The Normal Form

**Green's Theorem (Flux Form).** Let $D$ and $P, Q$ satisfy the hypotheses of Green's Theorem. Let $\hat{\mathbf{n}}$ be the outward unit normal to $\partial D$ and $ds$ the arc length element. Then

$$\oint_{\partial D}\mathbf{F}\cdot\hat{\mathbf{n}}\,ds = \iint_D\nabla\cdot\mathbf{F}\,dA,$$

where $\mathbf{F} = P\,\mathbf{i} + Q\,\mathbf{j}$ and $\nabla\cdot\mathbf{F} = \partial P/\partial x + \partial Q/\partial y$.

**Derivation.** For a curve traversed counterclockwise with parametrization $\mathbf{r}(t) = (x(t), y(t))$, the outward unit normal is $\hat{\mathbf{n}} = (y'(t), -x'(t))/|\mathbf{r}'(t)|$. Then

$$\mathbf{F}\cdot\hat{\mathbf{n}}\,ds = (P,Q)\cdot(y', -x')\,dt = P\,dy - Q\,dx.$$

Applying Green's Theorem with $P' = -Q$, $Q' = P$:

$$\oint_{\partial D}(P\,dy - Q\,dx) = \iint_D\left(\frac{\partial P}{\partial x} - \frac{\partial(-Q)}{\partial y}\right)dA = \iint_D\left(\frac{\partial P}{\partial x} + \frac{\partial Q}{\partial y}\right)dA = \iint_D\nabla\cdot\mathbf{F}\,dA.$$

## Interpretation

The left side $\oint_{\partial D}\mathbf{F}\cdot\hat{\mathbf{n}}\,ds$ is the total **outward flux** of $\mathbf{F}$ through the boundary curve $\partial D$ in two dimensions.

The right side $\iint_D\nabla\cdot\mathbf{F}\,dA$ integrates the divergence — the local source strength — over the interior.

The theorem says: **the net outward flux through the boundary equals the total source strength inside**. This is conservation of "fluid" in two dimensions: if the field represents fluid velocity, the net amount leaving the region per unit time equals the total production (positive divergence) minus total consumption (negative divergence) inside.

## This is the 2D Divergence Theorem

The flux form of Green's Theorem is the two-dimensional version of the Divergence Theorem. In three dimensions, the Divergence Theorem states $\oiint_{\partial V}\mathbf{F}\cdot d\mathbf{S} = \iiint_V\nabla\cdot\mathbf{F}\,dV$. Restricting to a planar domain with $\mathbf{F}$ having no $z$-component and no $z$-dependence gives exactly the flux form of Green's Theorem.

## Application: Verification of a Zero-Flux Field

Consider $\mathbf{F}(x,y) = (x,y)$ (the position vector field). $\nabla\cdot\mathbf{F} = 2$ everywhere. By the flux form, the outward flux through $\partial D$ is $2\cdot\text{Area}(D)$ for any domain $D$. For the unit disk: flux $= 2\pi$.

Direct check: $\hat{\mathbf{n}} = (\cos\theta, \sin\theta)$ on the unit circle, $\mathbf{F} = (\cos\theta,\sin\theta)$, so $\mathbf{F}\cdot\hat{\mathbf{n}} = 1$, and flux $= \oint 1\,ds = 2\pi$. Verified.

## Summary

The normal (flux) form of Green's Theorem equates the outward flux of $\mathbf{F}$ through a closed curve to the integral of $\nabla\cdot\mathbf{F}$ over the enclosed region. This is the two-dimensional Divergence Theorem and provides the direct link between local source strength (divergence) and global outflow (boundary flux). The three-dimensional Divergence Theorem (Chapter 3) extends this to volumes and surfaces.

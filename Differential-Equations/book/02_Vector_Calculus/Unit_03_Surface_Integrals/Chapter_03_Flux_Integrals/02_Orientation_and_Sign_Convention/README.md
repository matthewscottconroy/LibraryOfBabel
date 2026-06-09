# Orientation and Sign Convention

The flux $\iint_S\mathbf{F}\cdot d\mathbf{S}$ is not an intrinsic property of the surface $S$ and the field $\mathbf{F}$ alone — it also depends on the choice of orientation. Reversing the orientation (replacing $\hat{\mathbf{n}}$ by $-\hat{\mathbf{n}}$) changes the sign of the flux. This sign is not arbitrary: it carries physical and geometric meaning, and the conventions for choosing orientation in the context of the fundamental theorems are precise and must be followed carefully.

## Orientation Reversal

If $S$ has one orientation $\hat{\mathbf{n}}$ giving flux $\Phi = \iint_S\mathbf{F}\cdot\hat{\mathbf{n}}\,dS$, then the opposite orientation $-\hat{\mathbf{n}}$ gives flux $-\Phi$:

$$\iint_{-S}\mathbf{F}\cdot d\mathbf{S} = -\iint_S\mathbf{F}\cdot d\mathbf{S}.$$

In parametric terms, swapping $u$ and $v$ (i.e., replacing $\mathbf{r}(u,v)$ by $\mathbf{r}(v,u)$) replaces $\mathbf{r}_u\times\mathbf{r}_v$ by $\mathbf{r}_v\times\mathbf{r}_u = -\mathbf{r}_u\times\mathbf{r}_v$, reversing the sign.

## Conventions for Closed Surfaces

For a **closed surface** bounding a bounded volume $V$ (such as a sphere, cube, or cylinder with caps), the standard orientation is the **outward normal** — pointing away from the enclosed volume. This is the convention assumed in the Divergence Theorem:

$$\oiint_S\mathbf{F}\cdot d\mathbf{S} = \iiint_V\nabla\cdot\mathbf{F}\,dV.$$

With the outward orientation, the left side measures the net flux leaving the volume. If $\nabla\cdot\mathbf{F} > 0$ inside (a source), more leaves than enters, and the flux is positive. If $\nabla\cdot\mathbf{F} < 0$ (a sink), more enters than leaves, and the flux is negative.

Using the inward normal would give the opposite sign and would require writing $-\iiint_V\nabla\cdot\mathbf{F}\,dV$ on the right side.

## Conventions for Stokes' Theorem

When $S$ is a **surface with boundary** (a disk, a hemisphere, a soap film stretched across a wire loop), the orientation of $S$ and the orientation of its boundary curve $\partial S$ must be compatible. The standard convention is the **right-hand rule**: if the right thumb points in the direction of $\hat{\mathbf{n}}$, the fingers curl in the positive direction of traversal of $\partial S$.

Equivalently: standing on the side of $S$ that $\hat{\mathbf{n}}$ points toward and walking along $\partial S$ in the positive direction, the surface $S$ should be on your left.

Stokes' Theorem states:

$$\iint_S(\nabla\times\mathbf{F})\cdot d\mathbf{S} = \oint_{\partial S}\mathbf{F}\cdot d\mathbf{r},$$

where $\partial S$ is oriented consistently with $S$ via the right-hand rule.

**Example.** For the upper hemisphere $S: x^2+y^2+z^2=1$, $z\geq 0$, with upward orientation $\hat{\mathbf{n}} = +\hat{\mathbf{z}}$ at the north pole, the boundary $\partial S$ is the unit circle $x^2+y^2=1$ in the $z=0$ plane. The right-hand rule says: the right thumb pointing upward (in $\hat{\mathbf{n}}$ direction) means fingers curl counterclockwise when viewed from above. So $\partial S$ is traversed counterclockwise.

## Sign Rules for Composite Surfaces

When a closed surface $S$ is split into pieces $S = S_1 \cup S_2 \cup \cdots$, each piece inherits the orientation induced by the global outward normal. At the interface between two pieces, the normals cancel (one piece's boundary is traversed in one direction, the adjacent piece's is traversed in the opposite direction, consistent with Stokes). This cancellation is exactly the mechanism behind Green's theorem proofs using rectangles, and the Divergence Theorem proof using cubes.

## Worked Example: Flux and Orientation

Let $\mathbf{F} = \mathbf{k} = (0,0,1)$ and $S$ the upper hemisphere of radius 1.

**With upward normal** $\hat{\mathbf{n}}_+ = (\sin\phi\cos\theta, \sin\phi\sin\theta, \cos\phi)$ (outward from sphere):

$\mathbf{F}\cdot\hat{\mathbf{n}} = \cos\phi$.

$\Phi = \int_0^{2\pi}\int_0^{\pi/2}\cos\phi\cdot\sin\phi\,d\phi\,d\theta = 2\pi\cdot\frac{1}{2} = \pi$.

**With downward normal** $\hat{\mathbf{n}}_- = -\hat{\mathbf{n}}_+$ (inward normal):

$\Phi = -\pi$.

The field $\mathbf{k}$ points upward through the hemisphere with positive flux $\pi$ for the outward orientation.

## Practical Checklist for Flux Computation

1. **Identify the orientation** required by the problem or theorem (outward for closed surfaces, right-hand rule for Stokes surfaces).
2. **Compute $\mathbf{N} = \mathbf{r}_u\times\mathbf{r}_v$** and check its direction — does it agree with the required orientation?
3. **If not, swap $u$ and $v$** (or negate $\mathbf{N}$) to get the correct orientation.
4. **Proceed with the integral** using the correctly-oriented $\mathbf{N}$.

## Summary

Orientation is a sign convention that determines whether flux is positive or negative. For closed surfaces, the outward normal is standard (used in the Divergence Theorem). For surfaces with boundary, the right-hand rule establishes compatibility between the surface orientation and the boundary orientation (used in Stokes' Theorem). Getting orientation right is not optional — it determines the sign of the flux integral and the validity of the fundamental theorems.

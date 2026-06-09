# Definition and Computation of Scalar Surface Integrals

Suppose you have a thin metallic shell in the shape of a hemisphere, and the metal's density varies with position — denser near the equator, lighter near the pole. To find the total mass, you need to sum the product of density and area over the entire surface. This sum, in the limit of infinitely fine subdivision, is the scalar surface integral.

## Motivation via Riemann Sums

Let $S$ be a smooth surface and $f$ a continuous scalar function on $S$. Subdivide $S$ into $n$ small surface patches of areas $\Delta S_1, \ldots, \Delta S_n$. In each patch, pick a sample point $\mathbf{p}_i^*$. Form the Riemann sum

$$\sum_{i=1}^n f(\mathbf{p}_i^*)\,\Delta S_i.$$

As the patches shrink to zero, this converges to the **scalar surface integral**

$$\iint_S f\,dS.$$

## Computational Formula

Given a parametrization $\mathbf{r}: D \to \mathbb{R}^3$ with $\mathbf{r}_u\times\mathbf{r}_v \neq \mathbf{0}$ (a.e. on $D$):

$$\iint_S f\,dS = \iint_D f(\mathbf{r}(u,v))\,|\mathbf{r}_u\times\mathbf{r}_v|\,du\,dv.$$

**For graph surfaces $z = g(x,y)$:**

$$\iint_S f\,dS = \iint_{D_{xy}} f(x,y,g(x,y))\sqrt{1+g_x^2+g_y^2}\,dx\,dy.$$

**For $x = h(y,z)$** (graph in the $yz$-plane): $dS = \sqrt{1+h_y^2+h_z^2}\,dy\,dz$.

## Worked Examples

**Example 1.** Compute $\iint_S z\,dS$ where $S$ is the hemisphere $x^2+y^2+z^2 = a^2$, $z \geq 0$.

Parametrize: $\mathbf{r}(\phi,\theta) = (a\sin\phi\cos\theta, a\sin\phi\sin\theta, a\cos\phi)$, $\phi \in [0,\pi/2]$, $\theta \in [0,2\pi]$. $dS = a^2\sin\phi\,d\phi\,d\theta$.

$f(\mathbf{r}(\phi,\theta)) = a\cos\phi$.

$$\iint_S z\,dS = \int_0^{2\pi}\int_0^{\pi/2}(a\cos\phi)\cdot a^2\sin\phi\,d\phi\,d\theta = 2\pi a^3\int_0^{\pi/2}\cos\phi\sin\phi\,d\phi.$$

$\int_0^{\pi/2}\cos\phi\sin\phi\,d\phi = \frac{1}{2}\int_0^{\pi/2}\sin(2\phi)\,d\phi = \frac{1}{2}\cdot 1 = \frac{1}{2}$.

$$\iint_S z\,dS = 2\pi a^3 \cdot \frac{1}{2} = \pi a^3.$$

**Example 2.** Compute $\iint_S (x^2+y^2)\,dS$ where $S$ is the cylinder $x^2+y^2 = 1$, $0 \leq z \leq 1$.

Parametrize: $\mathbf{r}(\theta,z) = (\cos\theta, \sin\theta, z)$, $\theta \in [0,2\pi]$, $z \in [0,1]$. $dS = 1\cdot d\theta\,dz$.

$f(\mathbf{r}) = \cos^2\theta + \sin^2\theta = 1$.

$$\iint_S(x^2+y^2)\,dS = \int_0^{2\pi}\int_0^1 1\,dz\,d\theta = 2\pi.$$

**Example 3.** Compute $\iint_S (x+y+z)\,dS$ where $S$ is the triangle with vertices $(1,0,0)$, $(0,1,0)$, $(0,0,1)$.

The plane containing this triangle is $x+y+z=1$. Parametrize: $\mathbf{r}(x,y) = (x, y, 1-x-y)$ for $(x,y)$ in the triangle $D: x \geq 0$, $y \geq 0$, $x+y \leq 1$.

$g = 1-x-y$, $g_x = -1$, $g_y = -1$. $dS = \sqrt{1+1+1}\,dx\,dy = \sqrt{3}\,dx\,dy$.

$f(x,y,1-x-y) = x+y+(1-x-y) = 1$.

$$\iint_S(x+y+z)\,dS = \iint_D 1\cdot\sqrt{3}\,dx\,dy = \sqrt{3}\cdot\text{Area}(D) = \sqrt{3}\cdot\frac{1}{2}.$$

(This also equals $\sqrt{3}/2$, the area of the triangle times $f=1$, which makes sense since the integrand is constant on the surface.)

## Symmetry Arguments

Symmetry can simplify scalar surface integrals dramatically.

**Example 4.** By symmetry, $\iint_S x\,dS = \iint_S y\,dS = \iint_S z\,dS$ when $S$ is the full sphere $|\mathbf{r}| = a$ (since the sphere has the full rotational symmetry group $SO(3)$). Therefore $\iint_S x\,dS = (1/3)\iint_S(x+y+z)\,dS$. Moreover $\iint_S(x+y+z)\,dS = \iint_S 0\,dS = 0$ by the hemisphere symmetry (each hemisphere gives equal and opposite contributions in $x$, $y$, and $z$). So $\iint_S x\,dS = 0$ over the full sphere — consistent with the center of mass being at the origin.

## Step-by-Step Procedure

1. **Parametrize the surface:** Choose $\mathbf{r}(u,v)$ and identify the parameter domain $D$.
2. **Compute tangent vectors:** Find $\mathbf{r}_u$ and $\mathbf{r}_v$.
3. **Compute the cross product and its magnitude:** $\mathbf{N} = \mathbf{r}_u\times\mathbf{r}_v$, then $|\mathbf{N}|$.
4. **Substitute:** Replace $\mathbf{r}$ in $f$ to get $f(\mathbf{r}(u,v))$.
5. **Integrate:** Evaluate $\iint_D f(\mathbf{r}(u,v))\,|\mathbf{N}|\,du\,dv$ as an ordinary double integral.

## Common Pitfalls

**Forgetting the area distortion factor.** The factor $|\mathbf{r}_u\times\mathbf{r}_v|$ is essential. A common error is writing $\iint_D f(\mathbf{r}(u,v))\,du\,dv$ without this factor, which would be correct only if the parametrization is unit-speed in all directions (very unusual).

**Incorrect limits.** Make sure the parameter domain $D$ matches the surface $S$ — not more and not less.

## Summary

The scalar surface integral $\iint_S f\,dS$ is computed by parametrizing the surface, computing the area element $dS = |\mathbf{r}_u\times\mathbf{r}_v|\,du\,dv$, substituting $\mathbf{r}(u,v)$ into $f$, and evaluating the resulting double integral. The result is independent of orientation and of the choice of regular parametrization. The physical applications — total mass, average value, moments — make scalar surface integrals indispensable in mechanics and electrostatics.

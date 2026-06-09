# Iterated Triple Integrals

The triple integral $\iiint_E f(x,y,z)\,dV$ is computed by reducing it to three successive single-variable integrations. The strategy is the same as for double integrals: fix all but one variable and integrate; then the next variable; then the last. The challenge is setting up the limits correctly, which requires a careful description of the region $E$ as a nested sequence of bounds.

## Setup for a Box

For the rectangular box $[a,b]\times[c,d]\times[e,f]$, the triple integral is:

$$\iiint_E f\,dV = \int_a^b\int_c^d\int_e^f f(x,y,z)\,dz\,dy\,dx.$$

By Fubini's theorem (in three dimensions), any of the $3! = 6$ orderings of the three integrations gives the same result.

## General Regions

For a **$z$-simple** region $E = \{(x,y,z): (x,y)\in D,\; h_1(x,y)\leq z\leq h_2(x,y)\}$:

$$\iiint_E f\,dV = \iint_D\left[\int_{h_1(x,y)}^{h_2(x,y)}f(x,y,z)\,dz\right]dA.$$

The inner integral over $z$ gives a function of $(x,y)$, which is then integrated over the planar region $D$ as a double integral.

If $D$ is itself vertically simple: $D = \{a\leq x\leq b,\; g_1(x)\leq y\leq g_2(x)\}$, the full iterated integral is:

$$\int_a^b\int_{g_1(x)}^{g_2(x)}\int_{h_1(x,y)}^{h_2(x,y)}f(x,y,z)\,dz\,dy\,dx.$$

## Setting Up Limits: Strategy

To set up a triple integral over $E$ in the order $dz\,dy\,dx$:
1. **Outer ($x$) limits:** Find the range of $x$ over the entire region $E$: $a\leq x\leq b$.
2. **Middle ($y$) limits:** For each fixed $x$, find the range of $y$: $g_1(x)\leq y\leq g_2(x)$.
3. **Inner ($z$) limits:** For each fixed $(x,y)$, find the range of $z$: $h_1(x,y)\leq z\leq h_2(x,y)$.

The key rule: inner limits can depend on both outer variables; middle limits can depend only on the outermost variable; outermost limits are constants.

## Worked Example 1: Tetrahedron

Integrate $f(x,y,z) = 1$ over the tetrahedron bounded by $x=0$, $y=0$, $z=0$, and $x+y+z=1$.

$x$ ranges from $0$ to $1$. For fixed $x$, $y$ ranges from $0$ to $1-x$. For fixed $(x,y)$, $z$ ranges from $0$ to $1-x-y$.

$$\int_0^1\int_0^{1-x}\int_0^{1-x-y}dz\,dy\,dx = \int_0^1\int_0^{1-x}(1-x-y)\,dy\,dx.$$

Inner: $\int_0^{1-x}(1-x-y)\,dy = [(1-x)y - y^2/2]_0^{1-x} = (1-x)^2-(1-x)^2/2 = (1-x)^2/2$.

Outer: $\int_0^1\frac{(1-x)^2}{2}\,dx = \frac{1}{2}\cdot\frac{(1-x)^3}{-3}\bigg|_0^1 = \frac{1}{2}\cdot\frac{1}{3} = \frac{1}{6}$.

The volume of the tetrahedron with vertices $(0,0,0),(1,0,0),(0,1,0),(0,0,1)$ is $1/6$. Confirmed: the formula for the volume of a tetrahedron with one vertex at the origin and edge vectors $\mathbf{a},\mathbf{b},\mathbf{c}$ is $|\mathbf{a}\cdot(\mathbf{b}\times\mathbf{c})|/6$.

## Worked Example 2: Region Between Two Surfaces

Find $\iiint_E z\,dV$ where $E$ is bounded above by $z=4-x^2-y^2$ and below by $z=x^2+y^2$ (two paraboloids).

Intersection: $4-x^2-y^2=x^2+y^2 \Rightarrow x^2+y^2=2$, a circle of radius $\sqrt{2}$.

In Cartesian: $x^2+y^2\leq 2$, $x^2+y^2\leq z\leq 4-x^2-y^2$. This integral is easier in cylindrical coordinates (Section 2), but setting it up in Cartesian:

$D: -\sqrt{2}\leq x\leq\sqrt{2}$, $-\sqrt{2-x^2}\leq y\leq\sqrt{2-x^2}$.

Inner: $\int_{x^2+y^2}^{4-x^2-y^2}z\,dz = \frac{z^2}{2}\bigg|_{x^2+y^2}^{4-x^2-y^2} = \frac{(4-x^2-y^2)^2-(x^2+y^2)^2}{2}$.

This is manageable but messy in Cartesian. Cylindrical coordinates (Section 2) handle it much more cleanly.

## Switching the Order of Integration

As with double integrals, sometimes one order is analytically feasible while another is not. Switching order requires re-describing the region in the new order.

**Example.** $\int_0^1\int_0^x\int_0^y f(z)\,dz\,dy\,dx$ where $f(z)$ has no simple antiderivative in $z$.

The region: $0\leq z\leq y\leq x\leq 1$. Reorder to integrate $x$ first (outermost), then $y$, then $z$:
- $z$ ranges from $0$ to $1$ (outermost after switching).
- For fixed $z$: $y$ from $z$ to $1$.
- For fixed $(y,z)$: $x$ from $y$ to $1$.

$\int_0^1\int_z^1\int_y^1 f(z)\,dx\,dy\,dz = \int_0^1 f(z)\int_z^1\int_y^1 dx\,dy\,dz = \int_0^1 f(z)\int_z^1(1-y)\,dy\,dz = \int_0^1 f(z)\cdot\frac{(1-z)^2}{2}\,dz$.

## Common Pitfalls

The limits of the triple integral must satisfy the nesting rule: inner limits can depend on all outer variables, but outer limits can only depend on variables not yet integrated. A common mistake is writing, say, $\int_0^{1-x-y}$ as the outermost limit, which is illegal since both $x$ and $y$ are still free.

Also, when the region is not simple in any one ordering, it may need to be split into simpler subregions, each integrated separately. Sketching the region (at least schematically) before setting up limits is strongly recommended.

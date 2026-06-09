# Area and Volume

The most basic application of multiple integrals is the computation of geometric measures: the area of a region in the plane and the volume of a solid in space. These are the definitions that motivate the integrals in the first place — the double integral of $f(x,y)\geq 0$ computes the volume under the surface $z=f(x,y)$, and integrating the constant $1$ over a region gives its area or volume. But these formulas are also genuinely computational tools, allowing the measurement of regions bounded by curves and surfaces that cannot be treated by elementary geometry.

## Area of Planar Regions

The **area** of a bounded region $D\subseteq\mathbb{R}^2$ is:

$$A(D) = \iint_D\,dA = \iint_D 1\,dA.$$

For a vertically simple region $D = \{a\leq x\leq b,\; g_1(x)\leq y\leq g_2(x)\}$:

$$A(D) = \int_a^b [g_2(x)-g_1(x)]\,dx,$$

which reduces to the standard single-variable formula for the area between two curves.

**Example.** Area enclosed by $y = x^2$ and $y = 4-x^2$.

Intersections: $x^2 = 4-x^2 \Rightarrow x = \pm\sqrt{2}$. Area: $\int_{-\sqrt{2}}^{\sqrt{2}}[(4-x^2)-x^2]\,dx = \int_{-\sqrt{2}}^{\sqrt{2}}(4-2x^2)\,dx = \left[4x-\frac{2x^3}{3}\right]_{-\sqrt{2}}^{\sqrt{2}} = 2\left(4\sqrt{2}-\frac{4\sqrt{2}}{3}\right) = \frac{16\sqrt{2}}{3}$.

**Example (polar).** Area of the cardioid $r = 1+\cos\theta$.

$A = \frac{1}{2}\int_0^{2\pi}r^2\,d\theta = \frac{1}{2}\int_0^{2\pi}(1+\cos\theta)^2\,d\theta = \frac{1}{2}\int_0^{2\pi}(1+2\cos\theta+\cos^2\theta)\,d\theta = \frac{1}{2}(2\pi+0+\pi) = \frac{3\pi}{2}$.

(Using $\int_0^{2\pi}\cos^2\theta\,d\theta = \pi$ and $\int_0^{2\pi}\cos\theta\,d\theta = 0$.)

## Volume Under a Surface

The **volume** of the solid between the $xy$-plane and the surface $z = f(x,y)\geq 0$ over a region $D$ is:

$$V = \iint_D f(x,y)\,dA.$$

This generalizes the area formula $A = \int_a^b f(x)\,dx$ from one dimension.

**Example.** Volume of the solid bounded by $z = 4-x^2-y^2$ and $z = 0$.

The surface intersects $z=0$ when $x^2+y^2=4$, so the base region is the disk $D: x^2+y^2\leq 4$.

$V = \iint_{x^2+y^2\leq 4}(4-x^2-y^2)\,dA$. In polar: $\int_0^{2\pi}\int_0^2(4-r^2)r\,dr\,d\theta = 2\pi\int_0^2(4r-r^3)\,dr = 2\pi\left[2r^2-\frac{r^4}{4}\right]_0^2 = 2\pi(8-4) = 8\pi$.

## Volume of Solids: Triple Integrals

The **volume** of a solid region $E\subseteq\mathbb{R}^3$ is:

$$V(E) = \iiint_E\,dV = \iiint_E 1\,dV.$$

**Example.** Volume of the ellipsoid $x^2/a^2+y^2/b^2+z^2/c^2\leq 1$.

Use the transformation $x=au$, $y=bv$, $z=cw$ (maps the unit ball to the ellipsoid), with Jacobian $abc$.

$V = \int\int\int_{u^2+v^2+w^2\leq 1}abc\,du\,dv\,dw = abc\cdot\frac{4\pi}{3} = \frac{4\pi abc}{3}$.

For a sphere ($a=b=c=R$): $V = \frac{4\pi R^3}{3}$.

**Example.** Volume of the region bounded by the cone $z=\sqrt{x^2+y^2}$ and the paraboloid $z=x^2+y^2$.

Intersection: $\sqrt{x^2+y^2} = x^2+y^2 \Rightarrow r = r^2 \Rightarrow r=0$ or $r=1$.

$V = \int_0^{2\pi}\int_0^1\int_{r^2}^r r\,dz\,dr\,d\theta = 2\pi\int_0^1 r(r-r^2)\,dr = 2\pi\int_0^1(r^2-r^3)\,dr = 2\pi\left(\frac{1}{3}-\frac{1}{4}\right) = \frac{\pi}{6}$.

## Volumes by Slicing

Sometimes the volume is computed most easily by integrating cross-sectional areas. If the cross-section of $E$ at height $z$ is a region $D(z)$ with area $A(z)$, then:

$$V = \int_{z_{\min}}^{z_{\max}} A(z)\,dz.$$

**Example.** Volume of a sphere of radius $R$ by slicing.

At height $z$, the cross-section is a disk of radius $\sqrt{R^2-z^2}$ and area $\pi(R^2-z^2)$.

$V = \int_{-R}^R\pi(R^2-z^2)\,dz = \pi\left[R^2z-\frac{z^3}{3}\right]_{-R}^R = \pi\cdot\frac{4R^3}{3} = \frac{4\pi R^3}{3}$.

## Connection to Surface Integrals

The volume formulas here are the simplest case of a more general theory. The surface area of a surface $z=f(x,y)$ over a region $D$ is $\iint_D\sqrt{1+f_x^2+f_y^2}\,dA$ — a surface integral, which is the next generalization beyond multiple integrals. The volume under a surface (this section) does not require the surface area element $\sqrt{1+f_x^2+f_y^2}$; only the surface integral itself does.

## Common Pitfalls

When computing the volume between two surfaces, the integrand is the height of the solid at each $(x,y)$, which is the top surface minus the bottom surface: $f_{\text{top}}(x,y) - f_{\text{bot}}(x,y)$. The region $D$ is the projection of the solid onto the $xy$-plane, not the solid itself.

When using polar coordinates to compute area, the formula $A = \frac{1}{2}\int_\alpha^\beta r^2(\theta)\,d\theta$ (for regions bounded by a polar curve $r=f(\theta)$) comes from $A = \int\int r\,dr\,d\theta = \int_\alpha^\beta\int_0^{r(\theta)}r\,dr\,d\theta = \frac{1}{2}\int_\alpha^\beta r(\theta)^2\,d\theta$.

# Iterated Integrals

The double integral $\iint_D f(x,y)\,dA$ is defined as a limit of Riemann sums over a planar region $D$, just as the single-variable integral is defined as a limit of Riemann sums over an interval. The key computational question is: how do we actually evaluate such an integral? The answer — reducing a double integral to two successive single-variable integrals — is the content of this section. The result is an **iterated integral**, computed from the inside out.

## The Basic Idea

Think of computing the volume under a surface $z=f(x,y)\geq 0$ over a rectangle $R=[a,b]\times[c,d]$. One approach: slice the solid with planes of constant $x$. For each fixed $x\in[a,b]$, the cross-sectional area is $A(x) = \int_c^d f(x,y)\,dy$ (a single-variable integral in $y$, with $x$ treated as a parameter). The total volume is then $\int_a^b A(x)\,dx = \int_a^b\left[\int_c^d f(x,y)\,dy\right]dx$.

This is the iterated integral: integrate first with respect to $y$ (inner integral), then with respect to $x$ (outer integral).

## Notation and Mechanics

The **iterated integral** is written:

$$\int_a^b\int_c^d f(x,y)\,dy\,dx = \int_a^b\left[\int_c^d f(x,y)\,dy\right]dx.$$

To evaluate it:
1. **Inner integral:** fix $x$ and integrate $f(x,y)$ with respect to $y$ from $y=c$ to $y=d$. The result is a function of $x$ alone, call it $F(x) = \int_c^d f(x,y)\,dy$.
2. **Outer integral:** integrate $F(x)$ with respect to $x$ from $x=a$ to $x=b$.

## Worked Examples (Rectangles)

**Example 1.** $\int_0^1\int_0^2 (x^2+y)\,dy\,dx$.

Inner integral (with $x$ fixed): $\int_0^2(x^2+y)\,dy = [x^2y + y^2/2]_0^2 = 2x^2+2$.

Outer integral: $\int_0^1(2x^2+2)\,dx = [2x^3/3+2x]_0^1 = 2/3+2 = 8/3$.

**Example 2.** Same integrand, opposite order: $\int_0^2\int_0^1(x^2+y)\,dx\,dy$.

Inner: $\int_0^1(x^2+y)\,dx = [x^3/3+xy]_0^1 = 1/3+y$.

Outer: $\int_0^2(1/3+y)\,dy = [y/3+y^2/2]_0^2 = 2/3+2 = 8/3$.

Both orderings give the same result (Fubini's theorem), as they must.

## Non-Rectangular Regions: Vertically Simple

For a **vertically simple** (type I) region $D = \{(x,y): a\leq x\leq b,\; g_1(x)\leq y\leq g_2(x)\}$:

$$\iint_D f(x,y)\,dA = \int_a^b\int_{g_1(x)}^{g_2(x)} f(x,y)\,dy\,dx.$$

The limits of the inner integral depend on $x$.

**Example.** Integrate $f(x,y) = xy$ over the region bounded by $y=x$ and $y=x^2$ (i.e., $0\leq x\leq 1$, $x^2\leq y\leq x$).

$\int_0^1\int_{x^2}^x xy\,dy\,dx = \int_0^1 x\left[\frac{y^2}{2}\right]_{x^2}^x dx = \int_0^1 x\cdot\frac{x^2-x^4}{2}\,dx = \frac{1}{2}\int_0^1(x^3-x^5)\,dx$

$= \frac{1}{2}\left[\frac{x^4}{4}-\frac{x^6}{6}\right]_0^1 = \frac{1}{2}\left(\frac{1}{4}-\frac{1}{6}\right) = \frac{1}{2}\cdot\frac{1}{12} = \frac{1}{24}$.

## Horizontally Simple Regions (Type II)

For a **horizontally simple** (type II) region $D = \{(x,y): c\leq y\leq d,\; h_1(y)\leq x\leq h_2(y)\}$:

$$\iint_D f(x,y)\,dA = \int_c^d\int_{h_1(y)}^{h_2(y)} f(x,y)\,dx\,dy.$$

**Example.** Integrate $f(x,y) = y^2$ over the region bounded by $x = y^2$ and $x = y+2$.

The curves intersect where $y^2 = y+2$, i.e., $y^2-y-2=0$, so $y=-1$ and $y=2$.

$\int_{-1}^2\int_{y^2}^{y+2} y^2\,dx\,dy = \int_{-1}^2 y^2(y+2-y^2)\,dy = \int_{-1}^2(y^3+2y^2-y^4)\,dy$

$= \left[\frac{y^4}{4}+\frac{2y^3}{3}-\frac{y^5}{5}\right]_{-1}^2 = \left(4+\frac{16}{3}-\frac{32}{5}\right)-\left(\frac{1}{4}-\frac{2}{3}+\frac{1}{5}\right) = \frac{71}{12} - \frac{-43}{60} = \frac{355-43}{60}$...

Let me compute: at $y=2$: $4+16/3-32/5 = 60/15+80/15-96/15 = 44/15$. Wait, let me use $\text{lcm}(4,3,5)=60$: $4=240/60$, $16/3=320/60$, $32/5=384/60$: sum $= 176/60$. At $y=-1$: $1/4-2/3+1/5 = 15/60-40/60+12/60 = -13/60$. Total: $176/60-(-13/60) = 189/60 = 63/20$.

## Switching Order of Integration

Sometimes one order is easier than the other. If the inner integral in one order has no closed-form antiderivative but the other order does, switching order is essential.

**Example.** Evaluate $\int_0^1\int_x^1 e^{y^2}\,dy\,dx$.

$\int e^{y^2}\,dy$ has no elementary antiderivative. Switch order: the region is $0\leq x\leq y\leq 1$, i.e., $0\leq y\leq 1$, $0\leq x\leq y$.

$\int_0^1\int_0^y e^{y^2}\,dx\,dy = \int_0^1 ye^{y^2}\,dy = \left[\frac{e^{y^2}}{2}\right]_0^1 = \frac{e-1}{2}$.

## Common Pitfalls

When setting up limits for a non-rectangular region, the inner limits must depend only on the outer variable. Drawing a careful picture of the region and checking with a vertical or horizontal "slice" that sweeps through the entire region is essential.

Also, when switching the order of integration, the region must be re-described in the new order. The limits change, not just the order of the differentials.

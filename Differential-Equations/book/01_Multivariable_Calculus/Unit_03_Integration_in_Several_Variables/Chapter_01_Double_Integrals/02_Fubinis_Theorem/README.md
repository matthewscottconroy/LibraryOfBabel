# Fubini's Theorem

The iterated integral technique works in practice, but why does it give the correct value for the double integral? The answer is Fubini's theorem, which asserts that for sufficiently nice functions, the double integral equals the iterated integral regardless of which variable is integrated first. This is not obvious: integrating first over $y$ and then over $x$ is a genuinely different process from integrating first over $x$ and then over $y$, and the equality of the two iterated integrals (and their common equality with the double integral) is a nontrivial fact.

## Statement

**Theorem (Fubini, for rectangles).** Let $f: R\to\mathbb{R}$ be continuous on the rectangle $R = [a,b]\times[c,d]$. Then the double integral exists, equals both iterated integrals, and the two iterated integrals are equal:

$$\iint_R f(x,y)\,dA = \int_a^b\int_c^d f(x,y)\,dy\,dx = \int_c^d\int_a^b f(x,y)\,dx\,dy.$$

**Fubini's Theorem (for general regions).** If $f$ is continuous on a bounded closed region $D$ and $D$ is either vertically or horizontally simple (or can be decomposed into finitely many such pieces), then

$$\iint_D f(x,y)\,dA = \int_a^b\int_{g_1(x)}^{g_2(x)} f(x,y)\,dy\,dx = \int_c^d\int_{h_1(y)}^{h_2(y)} f(x,y)\,dx\,dy,$$

where the limits are chosen appropriately for each order.

## Why Continuity is Sufficient

The hypothesis of continuity guarantees two things: (1) the double integral (Riemann sum limit) exists, and (2) the single-variable integrals that appear inside the iterated integrals exist and are continuous functions of the outer variable, so the outer integral also exists.

The theorem extends to functions that are merely bounded and have discontinuities on a set of measure zero (e.g., finitely many curves), but continuity is the most practical hypothesis.

## A Counterexample Without Sufficient Hypotheses

Without appropriate hypotheses, the two iterated integrals can exist but differ.

**Example.** Define $f(x,y)$ on $[0,1]^2$ by $f(x,y) = (x^2-y^2)/(x^2+y^2)^2$ for $(x,y)\neq(0,0)$ and $f(0,0)=0$.

$\int_0^1\left[\int_0^1 f(x,y)\,dy\right]dx = \int_0^1\left[-\frac{y}{x^2+y^2}\right]_0^1 dx = \int_0^1\frac{-1}{x^2+1}\,dx = -\pi/4$.

$\int_0^1\left[\int_0^1 f(x,y)\,dx\right]dy = \pi/4$ (by symmetry: $f(x,y) = -f(y,x)$).

The two iterated integrals give $-\pi/4$ and $\pi/4$. Fubini's theorem does not apply because $f$ is not integrable on $[0,1]^2$ (it is not absolutely integrable: $\iint|f|\,dA = \infty$).

## Tonelli's Theorem

A companion result, due to Leonida Tonelli, guarantees that if $f\geq 0$ is measurable, then:

$$\iint f\,dA = \int\left[\int f\,dy\right]dx = \int\left[\int f\,dx\right]dy,$$

where all three expressions are either finite and equal, or all are $+\infty$. Tonelli's theorem is useful for non-negative functions where one doesn't need to worry about cancellation.

## Practical Implications: Switching the Order of Integration

The most common application of Fubini's theorem in computation is switching the order of integration to obtain a more tractable inner integral.

**Strategy:**
1. Draw the region $D$.
2. Identify the current order (say, $dy\,dx$) and find the bounds.
3. Re-describe $D$ in the opposite order ($dx\,dy$) by finding the new bounds.
4. Evaluate in whichever order is easier.

**Example.** $\int_0^1\int_{\sqrt{y}}^1 e^{x^3}\,dx\,dy$. The inner integral $\int e^{x^3}\,dx$ has no elementary form. Switch order.

Region: $0\leq y\leq x^2$, $0\leq x\leq 1$ (since $y\leq x^2$ iff $\sqrt{y}\leq x$).

$\int_0^1\int_0^{x^2} e^{x^3}\,dy\,dx = \int_0^1 e^{x^3}\cdot x^2\,dx = \left[\frac{e^{x^3}}{3}\right]_0^1 = \frac{e-1}{3}$.

## Fubini's Theorem in Physics

Fubini's theorem underpins many physics computations. Computing the total charge in a region of charge density $\rho(x,y)$ reduces to an iterated integral. The probability that a random variable $(X,Y)$ falls in a region $D$ is $\iint_D p(x,y)\,dA$, where $p$ is the joint density — again an iterated integral. In each case, Fubini's theorem guarantees the computation is valid and order-independent.

## Proof Sketch

The key step is showing that the function $F(x) = \int_c^d f(x,y)\,dy$ is continuous in $x$ when $f$ is continuous (by the uniform continuity of $f$ on the compact rectangle). Continuity of $F$ guarantees that $\int_a^b F(x)\,dx$ exists. One then shows $\int_a^b F(x)\,dx = \iint_R f\,dA$ by comparing Riemann sums: both approximate the volume under $z=f(x,y)$ over $R$.

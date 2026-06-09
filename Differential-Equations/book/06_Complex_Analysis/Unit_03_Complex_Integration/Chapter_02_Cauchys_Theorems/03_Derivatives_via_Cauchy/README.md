# Derivatives via the Cauchy Integral Formula

One of the most striking consequences of the Cauchy integral formula is that all higher-order derivatives of an analytic function can be expressed as contour integrals. This proves that every analytic function is infinitely differentiable — a property with no real-variable analogue, where once-differentiable functions need not be twice differentiable. The integral formulas for derivatives are not merely theoretical tools; they yield sharp estimates (Cauchy's inequalities) that are the key to proving Liouville's theorem and the identity principle.

## The Higher-Derivative Formula

**Theorem.** If $f$ is analytic on and inside a simple closed contour $C$, and $z_0$ is any interior point, then for every $n \geq 0$:
$$f^{(n)}(z_0) = \frac{n!}{2\pi i} \oint_C \frac{f(z)}{(z - z_0)^{n+1}}\, dz.$$

**Proof by induction.** The case $n = 0$ is the Cauchy integral formula. Assume the formula holds for $n - 1$. Differentiate the formula for $f^{(n-1)}(z_0)$ with respect to $z_0$:
$$f^{(n)}(z_0) = \frac{(n-1)!}{2\pi i}\oint_C f(z) \cdot \frac{d}{dz_0}\frac{1}{(z-z_0)^n}\, dz = \frac{(n-1)!}{2\pi i}\oint_C \frac{n f(z)}{(z-z_0)^{n+1}}\, dz.$$

The differentiation under the integral sign is justified because $f$ is continuous (hence bounded) on $C$ and the integrand $f(z)/(z-z_0)^{n+1}$ is uniformly continuous in $z_0$ as long as $z_0$ stays away from $C$. $\square$

## Analyticity Implies Infinite Differentiability

**Corollary.** Every function analytic on a domain $D$ is infinitely differentiable on $D$.

This is a stark contrast with real analysis. The function $f(x) = x|x|$ is differentiable on $\mathbb{R}$ but not twice differentiable at $0$. There is no complex-analytic function with similar behavior: once differentiability holds in the complex sense, all orders follow for free.

**Corollary.** Every analytic function is equal to its Taylor series in a neighborhood of every point (i.e., every analytic function is complex-analytic, an equivalent notion). This is the content of the Taylor series theorem, proved in Chapter 03.

## Cauchy's Inequality

**Theorem (Cauchy's Inequality).** If $f$ is analytic on the closed disk $|z - z_0| \leq R$ and $|f(z)| \leq M$ on the circle $|z - z_0| = R$, then:
$$|f^{(n)}(z_0)| \leq \frac{n! M}{R^n}.$$

**Proof.** Apply the higher-derivative formula with $C$ being the circle $|z - z_0| = R$, parametrized by $z = z_0 + Re^{i\theta}$:
$$|f^{(n)}(z_0)| = \left|\frac{n!}{2\pi i}\oint_C \frac{f(z)}{(z-z_0)^{n+1}}\, dz\right| \leq \frac{n!}{2\pi} \cdot \frac{M}{R^{n+1}} \cdot 2\pi R = \frac{n! M}{R^n}. \quad \square$$

These inequalities are the key tool in bounding the coefficients of Taylor series and in proving Liouville's theorem.

## Worked Examples

**Example 1.** Compute $\oint_{|z|=3} \frac{\sin z}{z^4}\, dz$.

By the formula with $n = 3$, $f(z) = \sin z$, $z_0 = 0$:
$$\frac{3!}{2\pi i}\oint \frac{\sin z}{z^4}\, dz = f'''(0) = \frac{d^3}{dz^3}\sin z\bigg|_{z=0} = -\cos(0) = -1.$$
Wait — $(\sin z)''' = -\cos z$, so $f'''(0) = -1$.
$$\oint_{|z|=3} \frac{\sin z}{z^4}\, dz = \frac{2\pi i}{3!} f'''(0) = \frac{2\pi i}{6}(-1) = -\frac{\pi i}{3}. \quad \square$$

**Example 2.** Compute $\oint_{|z|=2} \frac{e^z}{(z-1)^3}\, dz$.

Formula with $n = 2$, $f(z) = e^z$, $z_0 = 1$:
$$\oint \frac{e^z}{(z-1)^3}\, dz = \frac{2\pi i}{2!} f''(1) = \pi i \cdot e^1 = \pi i e.$$

**Example 3.** Use Cauchy's inequality to bound the coefficients of the power series of $f(z) = \sum_{n=0}^\infty a_n z^n$, given $|f(z)| \leq M$ for $|z| \leq R$.

Since $f^{(n)}(0)/n! = a_n$ (the $n$-th Taylor coefficient), Cauchy's inequality gives $|a_n| \leq M/R^n$.

This is a sharp bound: it is achieved by $f(z) = M(z/R)^n$ at the Taylor coefficient $a_n$.

## The Identity Principle

**Theorem (Identity Principle).** If $f$ and $g$ are analytic on a connected domain $D$ and agree on a set $S$ that has an accumulation point in $D$, then $f \equiv g$ on $D$.

**Proof sketch.** Let $h = f - g$. The set $\{z : h^{(n)}(z) = 0 \text{ for all } n \geq 0\}$ is both closed (by continuity) and open (because if $h$ and all its derivatives vanish at $z_0$, then $h = 0$ in a neighborhood of $z_0$ by the Taylor series). By connectedness, the set is either empty or all of $D$. If $S$ has an accumulation point $z_0$, then $h(z_0) = 0$ and (by the Taylor series) $h \equiv 0$ near $z_0$, so the set is nonempty, hence $h \equiv 0$ on $D$. $\square$

The identity principle has dramatic consequences:
- If two analytic functions agree on any arc, interval, or set with an accumulation point, they agree everywhere on their common domain of analyticity.
- An analytic function is determined by its values on an arbitrarily small open set.
- There is essentially unique way to analytically continue a function from a subdomain to a larger domain (if the continuation exists).

## Morera's Theorem: A Converse to Cauchy

**Theorem (Morera).** If $f$ is continuous on a domain $D$ and $\oint_C f(z)\, dz = 0$ for every simple closed contour $C$ in $D$, then $f$ is analytic on $D$.

**Proof.** Define $F(z) = \int_{z_0}^z f(w)\, dw$ (path-independent by hypothesis). Then $F$ is analytic with $F' = f$. Since $F$ is analytic, $F' = f$ is also analytic. $\square$

Morera's theorem is used to prove that limits of analytic functions are analytic (under uniform convergence), and it is the key tool in constructing analytic functions by series or integrals.

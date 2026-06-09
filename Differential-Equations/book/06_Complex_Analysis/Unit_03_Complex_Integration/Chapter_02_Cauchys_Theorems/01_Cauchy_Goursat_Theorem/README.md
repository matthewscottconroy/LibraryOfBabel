# The Cauchy-Goursat Theorem

The Cauchy-Goursat theorem asserts that the contour integral of an analytic function around a simple closed curve is zero. This result is the foundation of all of complex integration theory, and its proof is a masterpiece of mathematical argument: a global conclusion (the integral is zero) is deduced from a purely local hypothesis (the function is differentiable at each point).

## Statement of the Theorem

**Theorem (Cauchy-Goursat).** Let $D$ be a simply connected domain and $f : D \to \mathbb{C}$ analytic on $D$. Let $C$ be any simple closed contour in $D$ (a piecewise smooth, simple, closed curve whose interior lies in $D$). Then:
$$\oint_C f(z)\, dz = 0.$$

The remarkable feature of this theorem, as strengthened by Goursat, is that it requires only that $f$ be differentiable at every point of $D$ — not that $f'$ be continuous. In most treatments, continuity of $f'$ is not assumed; it is a consequence of the integral formula derived from the theorem itself.

## Preliminary: The Integral over a Triangle

**Lemma (Goursat's Lemma).** If $f$ is analytic on an open set $D$ and $T$ is any closed triangle in $D$, then $\oint_{\partial T} f(z)\, dz = 0$.

**Proof.** Let $I = \oint_{\partial T} f\, dz$. Subdivide $T$ into four congruent sub-triangles $T_1, T_2, T_3, T_4$ by connecting the midpoints of the sides. Then:
$$I = \sum_{k=1}^4 \oint_{\partial T_k} f\, dz,$$
because all interior edges are traversed twice in opposite directions and cancel. At least one summand satisfies $\left|\oint_{\partial T_k} f\, dz\right| \geq |I|/4$; call this sub-triangle $T^{(1)}$. Repeat the subdivision with $T^{(1)}$ to get $T^{(2)}$, and so on, obtaining a nested sequence $T \supset T^{(1)} \supset T^{(2)} \supset \cdots$ with
$$\left|\oint_{\partial T^{(n)}} f\, dz\right| \geq \frac{|I|}{4^n}.$$

The perimeter of $T^{(n)}$ is $L/2^n$ (where $L$ is the perimeter of $T$) and the diameter tends to zero. By completeness of $\mathbb{C}$, $\bigcap_n T^{(n)}$ consists of a single point $z_0 \in D$. Since $f$ is differentiable at $z_0$:
$$f(z) = f(z_0) + f'(z_0)(z - z_0) + \varepsilon(z)(z - z_0),$$
where $\varepsilon(z) \to 0$ as $z \to z_0$.

Now $\oint_{\partial T^{(n)}} [f(z_0) + f'(z_0)(z - z_0)]\, dz = 0$ because constant and linear functions have antiderivatives, so their integrals over closed curves vanish. Therefore:
$$\left|\oint_{\partial T^{(n)}} f\, dz\right| = \left|\oint_{\partial T^{(n)}} \varepsilon(z)(z - z_0)\, dz\right|.$$
On $\partial T^{(n)}$, $|z - z_0| \leq L/2^n$ (the diameter is at most the perimeter), and $|\varepsilon(z)| \leq \eta_n \to 0$. By ML:
$$\left|\oint_{\partial T^{(n)}} f\, dz\right| \leq \eta_n \cdot \frac{L}{2^n} \cdot \frac{L}{2^n} = \eta_n \frac{L^2}{4^n}.$$
Combined with the lower bound: $\frac{|I|}{4^n} \leq \eta_n \frac{L^2}{4^n}$, so $|I| \leq \eta_n L^2 \to 0$. Hence $I = 0$. $\square$

## Extension to Simple Closed Contours

From Goursat's lemma for triangles, the result extends to arbitrary polygons (by triangulation) and then to smooth contours (by approximation by polygons). The full theorem for simply connected domains follows from the fact that any simple closed contour in a simply connected domain can be deformed to a point, with the integral changing continuously and remaining zero throughout.

## The Deformation Principle

**Theorem.** If $f$ is analytic in the region between two simple closed contours $C_1$ and $C_2$ (where $C_2$ is interior to $C_1$), then:
$$\oint_{C_1} f(z)\, dz = \oint_{C_2} f(z)\, dz.$$

This follows by connecting $C_1$ and $C_2$ with two cuts to form a simply connected region, applying Cauchy's theorem, and observing that the contributions along the cuts cancel.

The deformation principle is enormously practical: it allows the contour of integration to be deformed at will, as long as no singularities of $f$ are crossed. In computing residues, one routinely replaces a complicated contour by a small circle around each singularity.

## Worked Examples

**Example 1.** Evaluate $\oint_C \frac{e^z}{z^2 + 4}\, dz$ where $C$ is the circle $|z| = 1$.

The singularities of the integrand are at $z = \pm 2i$, both of which have modulus $2 > 1$. So the integrand is analytic on and inside $C$. By Cauchy's theorem, the integral is $0$. $\square$

**Example 2.** Evaluate $\oint_C \frac{\cos z}{z}\, dz$ where $C$ is the circle $|z| = 2$.

The function $\cos z / z$ has a singularity at $z = 0$, which is inside $C$. So Cauchy's theorem does not directly apply. However, by the Cauchy integral formula (next section):
$$\oint_C \frac{\cos z}{z}\, dz = 2\pi i \cos(0) = 2\pi i.$$

**Example 3.** Evaluate $\oint_C \frac{1}{(z-1)(z-3)}\, dz$ where $C$ is the circle $|z-1| = 1$.

The singularity $z = 1$ is inside $C$; the singularity $z = 3$ is outside. By partial fractions:
$$\frac{1}{(z-1)(z-3)} = \frac{1/2}{z-1} \cdot \frac{-1}{1} + \ldots$$

Wait, let us write $\frac{1}{(z-1)(z-3)} = \frac{A}{z-1} + \frac{B}{z-3}$. Then $A = -1/2$, $B = 1/2$. The term $B/(z-3)$ is analytic inside $C$ (since $z = 3$ is outside), so its integral is $0$. The term $A/(z-1)$ contributes $A \cdot 2\pi i = -\pi i$. So the integral is $-\pi i$. $\square$

## Antiderivatives on Simply Connected Domains

**Theorem.** If $f$ is analytic on a simply connected domain $D$, then $f$ has an analytic antiderivative $F$ on $D$: there exists analytic $F : D \to \mathbb{C}$ with $F' = f$.

**Proof.** Fix $z_0 \in D$ and define $F(z) = \int_{z_0}^z f(w)\, dw$ where the integral is along any path in $D$ from $z_0$ to $z$. The path-independence of this integral (which follows from Cauchy's theorem applied to the closed curve formed by two paths from $z_0$ to $z$) ensures that $F$ is well-defined. Computing $F'(z) = f(z)$ is then a standard argument using the definition of the derivative and the continuity of $f$. $\square$

This theorem shows that path-independence of contour integrals, existence of antiderivatives, and vanishing of integrals over closed curves are all equivalent for analytic functions on simply connected domains — exactly as in real calculus with exact forms.

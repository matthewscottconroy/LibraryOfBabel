# Classification of Isolated Singularities

An isolated singularity of $f$ at $z_0$ is a point where $f$ is not analytic but is analytic in some punctured disk $0 < |z - z_0| < R$. The Laurent series valid in this punctured disk provides a complete classification of the singularity: removable, pole of some finite order, or essential. The behavior of $f(z)$ as $z \to z_0$ is entirely determined by this classification, and the three types exhibit radically different qualitative behaviors.

## Definition

**Definition.** A point $z_0$ is an isolated singularity of $f$ if:
- $f$ is not analytic (or not defined) at $z_0$, and
- $f$ is analytic on some punctured disk $0 < |z - z_0| < R$.

By Laurent's theorem, $f$ has a Laurent expansion $\sum_{n=-\infty}^\infty a_n(z-z_0)^n$ valid in $0 < |z - z_0| < R$.

## The Three Types

**Type 1: Removable Singularity.** The principal part is identically zero: $a_n = 0$ for all $n < 0$. Then $f(z) = \sum_{n=0}^\infty a_n(z-z_0)^n$ is a power series, and defining $f(z_0) = a_0$ extends $f$ to an analytic function at $z_0$.

**Characterization:** $f$ has a removable singularity at $z_0$ if and only if $\lim_{z \to z_0} f(z)$ exists (and is finite).

**Riemann's removable singularity theorem:** $f$ has a removable singularity at $z_0$ if and only if $|f(z)|$ is bounded in a neighborhood of $z_0$, equivalently if $\lim_{z \to z_0}(z - z_0)f(z) = 0$.

**Type 2: Pole of order $m$.** The principal part has finitely many nonzero terms, with $a_{-m} \neq 0$ and $a_n = 0$ for $n < -m$:
$$f(z) = \frac{a_{-m}}{(z-z_0)^m} + \cdots + \frac{a_{-1}}{z-z_0} + a_0 + a_1(z-z_0) + \cdots$$

**Characterization:** $f$ has a pole of order $m$ at $z_0$ if and only if $\lim_{z \to z_0}|f(z)| = +\infty$ and $(z-z_0)^m f(z)$ extends analytically to $z_0$ with a nonzero limit.

A pole of order $1$ is called a simple pole.

**Type 3: Essential Singularity.** The principal part has infinitely many nonzero terms: $a_n \neq 0$ for infinitely many $n < 0$.

**Characterization:** $f$ has an essential singularity at $z_0$ if and only if neither of the above holds: $f(z)$ neither approaches a finite limit nor goes to infinity as $z \to z_0$.

## Riemann's Theorem and Proof

**Theorem (Riemann's Removable Singularity Theorem).** If $f$ is analytic on $0 < |z - z_0| < R$ and $\lim_{z \to z_0}(z - z_0)f(z) = 0$, then $f$ has a removable singularity at $z_0$.

**Proof.** Define $g(z) = (z-z_0)f(z)$ for $z \neq z_0$ and $g(z_0) = 0$. Then $g$ is continuous and analytic for $z \neq z_0$. By Morera's theorem (the integral of $g$ around any triangle in the full disk is $0$, as the triangle integrals for $z \neq z_0$ converge), $g$ is analytic at $z_0$ with $g(z_0) = 0$. Write $g(z) = (z - z_0)h(z)$ for analytic $h$. Then $f(z) = h(z)$ for $z \neq z_0$, and defining $f(z_0) = h(z_0)$ gives the analytic extension. $\square$

## Casorati-Weierstrass Theorem

**Theorem (Casorati-Weierstrass).** If $f$ has an essential singularity at $z_0$, then for every punctured neighborhood $0 < |z - z_0| < \delta$ and every $w \in \mathbb{C}$, the image $f(\{0 < |z - z_0| < \delta\})$ is dense in $\mathbb{C}$.

In other words, near an essential singularity, $f$ takes values arbitrarily close to every complex number. The behavior is wildly oscillatory.

**Example.** $f(z) = e^{1/z}$ has an essential singularity at $0$. Near $z = 0$, $f$ takes all nonzero complex values: for any $w \neq 0$, $e^{1/z} = w$ gives $1/z = \log w$, so $z = 1/\log w$ which is close to $0$ when $|\log w|$ is large. The value $0$ is not taken but is a limit point.

The stronger Picard's great theorem states that near an essential singularity, $f$ takes every complex value with at most one exception (infinitely many times).

## Worked Examples of Classification

**Example 1.** Classify the singularity of $f(z) = \frac{\sin z}{z}$ at $z = 0$.

Laurent expansion: $\frac{\sin z}{z} = \frac{1}{z}\left(z - \frac{z^3}{6} + \frac{z^5}{120} - \cdots\right) = 1 - \frac{z^2}{6} + \frac{z^4}{120} - \cdots$

No negative powers: removable singularity. The function extends to $f(0) = 1$ (using L'Hopital or directly from the series). $\square$

**Example 2.** Classify the singularity of $g(z) = \frac{1 - \cos z}{z^2}$ at $z = 0$.

$1 - \cos z = z^2/2 - z^4/24 + \cdots$, so $g(z) = 1/2 - z^2/24 + \cdots$: removable, $g(0) = 1/2$.

**Example 3.** Classify and find the order of the pole of $h(z) = \frac{z + 1}{z^2(z - 2)}$ at $z = 0$ and $z = 2$.

At $z = 0$: The denominator has a zero of order $2$, and the numerator $z + 1$ does not vanish at $0$. So $z = 0$ is a pole of order $2$. Laurent expansion for small $|z|$: $h(z) = \frac{z+1}{z^2(z-2)} = \frac{-(z+1)}{2z^2(1-z/2)} = -\frac{z+1}{2z^2}\sum_{n=0}^\infty (z/2)^n$, giving a principal part involving $z^{-2}$ and $z^{-1}$.

At $z = 2$: denominator has a simple zero, numerator $= 3 \neq 0$, so simple pole.

**Example 4.** $e^{1/z}$ at $z = 0$: Laurent series $= \sum_{n=0}^\infty \frac{1}{n!z^n}$, infinitely many negative powers: essential singularity.

## Meromorphic Functions

**Definition.** A function is meromorphic on a domain $D$ if it is analytic on $D$ except for poles (isolated singularities that are poles, not essential). Meromorphic functions are analytic as maps to the Riemann sphere $\hat{\mathbb{C}} = \mathbb{C} \cup \{\infty\}$, sending each pole to $\infty$.

Rational functions are meromorphic on $\hat{\mathbb{C}}$. The function $\tan z$ is meromorphic on $\mathbb{C}$ with simple poles at $\pi/2 + n\pi$, $n \in \mathbb{Z}$.

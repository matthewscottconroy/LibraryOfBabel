# The Complex Exponential and Logarithm

The complex exponential function is the cornerstone of complex analysis. Every elementary analytic function — trigonometric, hyperbolic, power, and logarithm — is defined in terms of $e^z$, and many of the deepest theorems (Cauchy's integral formula, the residue theorem, conformal mappings) depend on properties that $e^z$ exemplifies. This section defines $e^z$ carefully, establishes its fundamental properties, and then constructs the complex logarithm as its multi-valued inverse.

## The Complex Exponential Function

**Definition.** The complex exponential function is defined for all $z \in \mathbb{C}$ by the power series
$$e^z = \sum_{n=0}^{\infty} \frac{z^n}{n!}.$$

This series converges absolutely for all $z \in \mathbb{C}$ (the ratio test gives absolute convergence for every complex $z$, since $|z^{n+1}/(n+1)!| / |z^n/n!| = |z|/(n+1) \to 0$). Term-by-term differentiation (valid for uniformly convergent power series on compact sets) gives $(e^z)' = e^z$.

**Theorem.** The complex exponential satisfies:
1. $e^{z+w} = e^z e^w$ for all $z, w \in \mathbb{C}$.
2. $e^0 = 1$ and $|e^z| = e^{\mathrm{Re}(z)}$.
3. $e^z \neq 0$ for all $z \in \mathbb{C}$.
4. $e^z$ is entire with $(e^z)' = e^z$.
5. $e^{z + 2\pi i} = e^z$ for all $z$ (periodicity with period $2\pi i$).

**Proof of (1).** Multiply the series for $e^z$ and $e^w$ using the Cauchy product formula:
$$e^z e^w = \left(\sum_{n=0}^\infty \frac{z^n}{n!}\right)\left(\sum_{m=0}^\infty \frac{w^m}{m!}\right) = \sum_{k=0}^\infty \frac{1}{k!}\sum_{j=0}^k \binom{k}{j} z^j w^{k-j} = \sum_{k=0}^\infty \frac{(z+w)^k}{k!} = e^{z+w}. \quad \square$$

**Proof of (2).** For $z = x + iy$, write $e^{iy} = \sum \frac{(iy)^n}{n!}$. Separating even and odd terms gives $e^{iy} = \cos y + i\sin y$ (Euler's formula). Then $e^z = e^x e^{iy} = e^x(\cos y + i\sin y)$, so $|e^z| = e^x |e^{iy}| = e^x \cdot 1 = e^x = e^{\mathrm{Re}(z)}$. $\square$

**Proof of (5).** $e^{z + 2\pi i} = e^z \cdot e^{2\pi i} = e^z \cdot (\cos 2\pi + i\sin 2\pi) = e^z \cdot 1 = e^z$. $\square$

## Mapping Properties of $e^z$

The formula $e^z = e^x(\cos y + i\sin y)$ shows:
- The modulus $|e^z| = e^x$ depends only on $\mathrm{Re}(z)$.
- The argument $\arg(e^z) = y$ (modulo $2\pi$) depends only on $\mathrm{Im}(z)$.

Consequently:
- **Vertical lines** $\{x = c\}$ map to **circles** $\{|w| = e^c\}$.
- **Horizontal lines** $\{y = c\}$ map to **rays** $\{\arg w = c\}$ from the origin.
- The **horizontal strip** $\{-\pi < \mathrm{Im}(z) < \pi\}$ maps bijectively onto $\mathbb{C} \setminus (-\infty, 0]$. (The image of the vertical line $x = c$ is the circle $|w| = e^c$; as $c$ ranges over $\mathbb{R}$ and $y$ ranges over $(-\pi, \pi)$, we cover $\mathbb{C}$ minus the negative real axis.)

The exponential is not injective on $\mathbb{C}$ because of its $2\pi i$ periodicity: $e^z = e^w \iff z - w \in 2\pi i \mathbb{Z}$.

## The Complex Logarithm

**Definition.** The (multivalued) complex logarithm is any $w \in \mathbb{C}$ satisfying $e^w = z$. For $z \neq 0$:
$$\log z = \ln|z| + i\arg z,$$
where $\arg z$ ranges over all arguments of $z$. Any two values of $\log z$ differ by an integer multiple of $2\pi i$.

**Worked examples.**
$$\log(1) = \ln 1 + i \cdot 2\pi k = 2\pi ki, \quad k \in \mathbb{Z}.$$
$$\log(-1) = \ln 1 + i(\pi + 2\pi k) = i(2k+1)\pi, \quad k \in \mathbb{Z}.$$
$$\log(1 + i) = \ln\sqrt{2} + i(\pi/4 + 2\pi k) = \tfrac{1}{2}\ln 2 + i(\pi/4 + 2\pi k), \quad k \in \mathbb{Z}.$$

## The Principal Branch

**Definition.** The principal branch of the logarithm is
$$\mathrm{Log}\, z = \ln|z| + i\,\mathrm{Arg}\, z, \qquad z \in \mathbb{C} \setminus (-\infty, 0],$$
where $\mathrm{Arg}\, z \in (-\pi, \pi]$ is the principal argument.

**Theorem.** $\mathrm{Log}\, z$ is analytic on $\mathbb{C} \setminus (-\infty, 0]$, with $\frac{d}{dz}\mathrm{Log}\, z = \frac{1}{z}$.

**Proof.** Write $\mathrm{Log}(x + iy) = \frac{1}{2}\ln(x^2 + y^2) + i\arctan(y/x)$ (using the appropriate branch of $\arctan$ for the quadrant). Let $u = \frac{1}{2}\ln(x^2 + y^2)$ and $v = \arctan(y/x)$.

Compute partial derivatives:
$$u_x = \frac{x}{x^2 + y^2}, \quad u_y = \frac{y}{x^2 + y^2}.$$
$$v_x = \frac{-y/x^2}{1 + y^2/x^2} = \frac{-y}{x^2 + y^2}, \quad v_y = \frac{1/x}{1 + y^2/x^2} = \frac{x}{x^2 + y^2}.$$

Cauchy-Riemann check: $u_x = v_y = \dfrac{x}{x^2+y^2}$ and $u_y = -v_x = \dfrac{y}{x^2+y^2}$. Both hold. All partial derivatives are continuous away from the branch cut, so $\mathrm{Log}\, z$ is analytic there. The derivative is:
$$\frac{d}{dz}\mathrm{Log}\, z = u_x + iv_x = \frac{x}{x^2+y^2} - i\frac{y}{x^2+y^2} = \frac{x - iy}{x^2+y^2} = \frac{\bar{z}}{|z|^2} = \frac{1}{z}. \quad \square$$

## Algebraic Properties of the Logarithm

For the multivalued logarithm: $\log(z_1 z_2) = \log z_1 + \log z_2$ (as sets of values). For the principal branch, the analogous formula holds only up to multiples of $2\pi i$:
$$\mathrm{Log}(z_1 z_2) = \mathrm{Log}\, z_1 + \mathrm{Log}\, z_2 + 2\pi i k$$
for some $k \in \{-1, 0, 1\}$ depending on whether the sum of the principal arguments exceeds $\pi$ in absolute value.

**Worked example.** Compute $\mathrm{Log}((-1)(-1))$.

$\mathrm{Log}(-1) = i\pi$ and $(-1)(-1) = 1$, so $\mathrm{Log}(1) = 0$. But $\mathrm{Log}(-1) + \mathrm{Log}(-1) = 2\pi i \neq 0$. The discrepancy is $2\pi i \cdot 1$: the formula fails here because $\mathrm{Arg}(-1) + \mathrm{Arg}(-1) = 2\pi > \pi$.

## Connection to Integration

The formula $\frac{d}{dz}\mathrm{Log}\, z = 1/z$ has a profound consequence: since $1/z$ has no antiderivative that is analytic on all of $\mathbb{C} \setminus \{0\}$ (the logarithm is multivalued), the integral $\oint_{|z|=1} \frac{dz}{z}$ is not zero. Computing directly:
$$\int_0^{2\pi} \frac{1}{e^{i\theta}} \cdot ie^{i\theta}\, d\theta = 2\pi i.$$

This is the first and most important example of a nonzero contour integral, and it is the source of the $2\pi i$ that appears in Cauchy's integral formula and the residue theorem.

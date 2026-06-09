# Definition and Properties of Contour Integrals

The contour integral is the fundamental operation of complex integration. It generalizes the real line integral to the complex plane and, in the presence of analyticity, acquires properties — path-independence, representation formulas, vanishing over closed curves — that have no real-variable analogue. This section defines the contour integral rigorously and establishes its elementary algebraic properties.

## Curves in the Complex Plane

**Definition.** A smooth curve in $\mathbb{C}$ is a continuously differentiable function $z : [a, b] \to \mathbb{C}$. The curve is called a contour if it is piecewise smooth (i.e., $[a, b]$ can be divided into finitely many subintervals on each of which $z$ is smooth). A curve is simple if it does not cross itself: $z(t_1) \neq z(t_2)$ for $t_1 \neq t_2$ (except possibly at the endpoints). A curve is closed if $z(a) = z(b)$.

The arc length of a smooth curve is $L = \int_a^b |z'(t)|\, dt$.

## Definition of the Contour Integral

**Definition.** Let $C$ be a contour parametrized by $z : [a, b] \to \mathbb{C}$, and let $f$ be a continuous function on the image of $C$. The contour integral of $f$ along $C$ is
$$\int_C f(z)\, dz = \int_a^b f(z(t))\, z'(t)\, dt.$$

The right-hand side is the integral of the complex-valued function $g(t) = f(z(t)) z'(t)$, which is computed by splitting into real and imaginary parts:
$$\int_a^b g(t)\, dt = \int_a^b \mathrm{Re}(g(t))\, dt + i\int_a^b \mathrm{Im}(g(t))\, dt.$$

**Independence of parametrization.** If $\tilde{z} = z \circ \phi$ is another parametrization of $C$ (with $\phi$ smooth and increasing), then by the substitution rule for real integrals:
$$\int_{\tilde{a}}^{\tilde{b}} f(\tilde{z}(s))\tilde{z}'(s)\, ds = \int_a^b f(z(t)) z'(t)\, dt.$$
So the contour integral depends only on the oriented curve, not the particular parametrization.

**Orientation reversal.** If $-C$ denotes $C$ traversed in the opposite direction, $\int_{-C} f(z)\, dz = -\int_C f(z)\, dz$.

## Reduction to Real Integrals

Writing $f = u + iv$ and $z = x + iy$, $dz = dx + i\,dy$:
$$\int_C f\, dz = \int_C (u + iv)(dx + i\,dy) = \int_C (u\,dx - v\,dy) + i\int_C (v\,dx + u\,dy).$$
This expresses the complex contour integral as two real line integrals. The first is $\int_C \mathbf{F}_1 \cdot d\mathbf{r}$ with $\mathbf{F}_1 = (u, -v)$, and the second is $\int_C \mathbf{F}_2 \cdot d\mathbf{r}$ with $\mathbf{F}_2 = (v, u)$.

The Cauchy-Riemann equations ensure that $\mathbf{F}_1$ is the gradient of $u$ (when $f$ is analytic) and $\mathbf{F}_2$ is the gradient of $v$, making both integrals path-independent. This is the analytic content of Cauchy's theorem, which we develop in Chapter 02.

## Elementary Properties

**Linearity.** For constants $\alpha, \beta \in \mathbb{C}$ and continuous $f, g$ on $C$:
$$\int_C (\alpha f + \beta g)\, dz = \alpha\int_C f\, dz + \beta\int_C g\, dz.$$

**Additivity.** If $C = C_1 \cup C_2$ (traversed in order):
$$\int_C f\, dz = \int_{C_1} f\, dz + \int_{C_2} f\, dz.$$

**Conjugation.** In general, $\int_C \overline{f(z)}\, dz \neq \overline{\int_C f(z)\, dz}$ because of the complex $dz$.

## Worked Examples

**Example 1.** Compute $\int_C z^2\, dz$ where $C$ is the straight line segment from $0$ to $1 + i$.

Parametrize: $z(t) = t(1 + i) = t + it$, $t \in [0, 1]$. Then $z'(t) = 1 + i$ and:
$$\int_0^1 (t + it)^2(1 + i)\, dt = (1+i)\int_0^1 t^2(1+i)^2\, dt = (1+i)^3 \int_0^1 t^2\, dt = (1+i)^3 \cdot \frac{1}{3}.$$
$(1+i)^2 = 2i$ and $(1+i)^3 = 2i(1+i) = -2 + 2i$.
$$\int_C z^2\, dz = \frac{-2 + 2i}{3}.$$
Alternatively, $F(z) = z^3/3$ is an antiderivative of $z^2$, so by the fundamental theorem: $F(1+i) - F(0) = (1+i)^3/3 = (-2+2i)/3$. $\square$

**Example 2.** Compute $\int_C \bar{z}\, dz$ where $C$ is the unit circle $|z| = 1$ traversed counterclockwise.

Parametrize: $z(t) = e^{it}$, $t \in [0, 2\pi]$. Then $\bar{z}(t) = e^{-it}$ and $z'(t) = ie^{it}$:
$$\int_0^{2\pi} e^{-it} \cdot ie^{it}\, dt = i\int_0^{2\pi} 1\, dt = 2\pi i.$$
Note: $\bar{z}$ is not analytic (it fails the Cauchy-Riemann equations), so the integral over a closed curve need not vanish. $\square$

**Example 3.** Compute $\int_C z^{-1}\, dz$ where $C$ is the unit circle $|z| = 1$ traversed counterclockwise.

Parametrize: $z(t) = e^{it}$, $z'(t) = ie^{it}$:
$$\int_0^{2\pi} \frac{1}{e^{it}} \cdot ie^{it}\, dt = i\int_0^{2\pi} dt = 2\pi i.$$
This integral is the fundamental building block of Cauchy's integral formula: $\oint_{|z-z_0|=r} \frac{dz}{z - z_0} = 2\pi i$ for any $r > 0$. $\square$

## The Modulus Inequality

**Theorem.** $\left|\int_C f(z)\, dz\right| \leq \int_C |f(z)|\, |dz| = \int_a^b |f(z(t))| |z'(t)|\, dt$.

**Proof.** If the integral is $0$, the inequality is trivial. Otherwise, let $\theta = \arg\int_C f\, dz$. Then:
$$\left|\int_C f\, dz\right| = e^{-i\theta}\int_C f\, dz = \int_C e^{-i\theta} f\, dz = \mathrm{Re}\int_C e^{-i\theta} f\, dz = \int_a^b \mathrm{Re}(e^{-i\theta} f(z(t))) z'(t)\, dt.$$

Wait — the last equality uses that the imaginary part integrates to zero since the left side is real. More carefully:
$$\left|\int_C f\, dz\right| = \mathrm{Re}\left(\int_C e^{-i\theta}f\, dz\right) \leq \int_C |e^{-i\theta}f|\, |dz| = \int_C |f|\, |dz|. \quad \square$$

This gives the ML inequality (estimation lemma), discussed further in Section 03.

## Connection to the Fundamental Theorem of Calculus

**Theorem.** If $F$ is analytic on a domain $D$ with $F' = f$, then for any contour $C$ in $D$ from $z_1$ to $z_2$:
$$\int_C f(z)\, dz = F(z_2) - F(z_1).$$

This is the complex analogue of the real fundamental theorem. It implies that if $f$ has an analytic antiderivative on a domain, all contour integrals of $f$ in that domain are path-independent, and all integrals over closed contours vanish. The antiderivative $F$ exists whenever $f$ is analytic on a simply connected domain — this is part of the content of Cauchy's theorem.

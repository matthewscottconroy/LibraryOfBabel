# Associated Legendre Functions

When Laplace's equation is solved in spherical coordinates without azimuthal symmetry — when the solution depends on the azimuthal angle $\phi$ — the polar equation becomes the **associated Legendre equation**, whose bounded solutions are the **associated Legendre functions** $P_\ell^m(t)$. These functions are the polar building blocks of spherical harmonics: $Y_\ell^m(\theta,\phi) \propto P_\ell^{|m|}(\cos\theta)e^{im\phi}$. They reduce to ordinary Legendre polynomials when $m=0$ and become increasingly oscillatory (with more sign changes in $\theta$) as $|m|$ increases toward $\ell$.

## The Associated Legendre Equation

With $t = \cos\theta$ and integer azimuthal wavenumber $m$, the polar equation from separating $\Delta_{S^2}Y = -\ell(\ell+1)Y$ is:

$$\frac{d}{dt}\!\left[(1-t^2)\frac{dP}{dt}\right] + \left[\ell(\ell+1) - \frac{m^2}{1-t^2}\right]P = 0, \quad t \in [-1,1]. \tag{Associated Legendre}$$

For $m=0$ this reduces to Legendre's equation. The singular points are $t = \pm 1$ (i.e., $\theta = 0,\pi$, the poles of the sphere). The requirement that solutions be bounded (and hence continuous) at these poles imposes the constraint $\ell \in \{0,1,2,\ldots\}$ and $m \in \{-\ell, -\ell+1, \ldots, \ell\}$.

## Definition and Construction

**Definition (for $m \geq 0$).** The associated Legendre functions are:

$$P_\ell^m(t) = (-1)^m (1-t^2)^{m/2}\frac{d^m}{dt^m}P_\ell(t), \qquad 0 \leq m \leq \ell. \tag{Def}$$

The factor $(-1)^m$ is the **Condon-Shortley phase convention**, standard in physics but sometimes omitted in mathematics texts. The factor $(1-t^2)^{m/2}$ vanishes at $t=\pm 1$ (for $m > 0$), ensuring boundedness at the poles.

**Explicit computation using Rodrigues' formula.** Since $P_\ell(t) = \frac{1}{2^\ell\ell!}\frac{d^\ell}{dt^\ell}(t^2-1)^\ell$, we can write:

$$P_\ell^m(t) = \frac{(-1)^m}{2^\ell\ell!}(1-t^2)^{m/2}\frac{d^{\ell+m}}{dt^{\ell+m}}(t^2-1)^\ell.$$

**Verification that $P_\ell^m$ satisfies the associated Legendre equation.** Let $u = P_\ell(t)$, so $(1-t^2)u'' - 2tu' + \ell(\ell+1)u = 0$. Differentiate this $m$ times using Leibniz's rule to get an equation for $v = u^{(m)} = (d/dt)^m P_\ell$. Then set $P_\ell^m = (1-t^2)^{m/2}v$ and compute the associated Legendre equation for $P_\ell^m$: the $(1-t^2)^{m/2}$ factor produces the $m^2/(1-t^2)$ term in the eigenvalue equation. $\square$

## First Few Associated Legendre Functions

For $\ell = 1$:
$$P_1^0(t) = t = \cos\theta, \quad P_1^1(t) = -\sqrt{1-t^2} = -\sin\theta.$$

For $\ell = 2$:
$$P_2^0(t) = \tfrac{1}{2}(3t^2-1), \quad P_2^1(t) = -3t\sqrt{1-t^2}, \quad P_2^2(t) = 3(1-t^2).$$

For $\ell = 3$:
$$P_3^0(t) = \tfrac{1}{2}(5t^3-3t), \quad P_3^1(t) = -\tfrac{3}{2}(5t^2-1)\sqrt{1-t^2},$$
$$P_3^2(t) = 15t(1-t^2), \quad P_3^3(t) = -15(1-t^2)^{3/2}.$$

Notice the pattern: $P_\ell^\ell(t) = (-1)^\ell(2\ell-1)!!(1-t^2)^{\ell/2}$, where $(2\ell-1)!! = 1\cdot 3\cdot 5\cdots(2\ell-1)$. This is the "maximum azimuthal mode" — a function that vanishes at the poles to order $\ell$ and has a single maximum near the equator.

## Negative $m$

For $m < 0$, the natural convention is:

$$P_\ell^{-m}(t) = (-1)^m\frac{(\ell-m)!}{(\ell+m)!}P_\ell^m(t), \qquad 1 \leq m \leq \ell.$$

Some references omit the $(-1)^m$ factor. With this convention, the associated Legendre equation is satisfied for all integer $m$ with $|m| \leq \ell$.

**Motivation.** The complex conjugate $\overline{Y_\ell^m} = (-1)^m Y_\ell^{-m}$ (Condon-Shortley convention) should be consistent with the definition. The factor $(\ell-m)!/(\ell+m)!$ is a normalization so that $P_\ell^{-m}$ has the same $L^2$ norm as $P_\ell^m$ up to a sign.

## Orthogonality

**Theorem.** For fixed $m$, the functions $\{P_\ell^m\}_{\ell \geq |m|}$ satisfy:

$$\int_{-1}^1 P_\ell^m(t)\,P_k^m(t)\,dt = \frac{2}{2\ell+1}\frac{(\ell+m)!}{(\ell-m)!}\delta_{\ell k}. \tag{Orthogonality}$$

The normalization constant $\frac{(\ell+m)!}{(\ell-m)!}$ grows with $m$, reflecting the fact that $P_\ell^m$ has increasing amplitude near the equator as $m$ increases.

**Proof.** The functions $P_\ell^m$ and $P_k^m$ both satisfy the associated Legendre equation (with the same $m$ but different $\ell$, $k$), which is a Sturm-Liouville equation:

$$-\frac{d}{dt}\!\left[(1-t^2)\frac{d}{dt}\right]P + \frac{m^2}{1-t^2}P = \ell(\ell+1)P.$$

Multiplying the equation for $P_\ell^m$ by $P_k^m$ and the equation for $P_k^m$ by $P_\ell^m$, subtracting, and integrating from $-1$ to $1$: the left side gives $(\ell(\ell+1)-k(k+1))\int P_\ell^m P_k^m\,dt$, and the right side gives zero (boundary terms vanish because $(1-t^2)^{m/2+1/2} \to 0$ at $t=\pm 1$ for $m \geq 0$). Since $\ell(\ell+1) \neq k(k+1)$ for $\ell \neq k$, the integral must be zero.

For the normalization ($\ell = k$): use the Rodrigues representation and integrate by parts $\ell+m$ times; the boundary terms vanish and the result reduces to a beta function integral evaluated by the formula $\int_0^1 t^{2m}(1-t^2)^{\ell-m}\,dt = B(m+1/2, \ell-m+1)/2$ (Euler beta function). The final result is $2/(2\ell+1) \cdot (\ell+m)!/(\ell-m)!$. $\square$

## Recursion Relations

The associated Legendre functions satisfy several recursion relations, derived by differentiating the Legendre recursions:

**In $\ell$ (three-term recursion for fixed $m$):**

$$(\ell-m+1)P_{\ell+1}^m(t) = (2\ell+1)t\,P_\ell^m(t) - (\ell+m)P_{\ell-1}^m(t).$$

**In $m$ (relating different azimuthal orders):**

$$P_\ell^{m+1}(t) = \frac{-2mt}{\sqrt{1-t^2}}P_\ell^m(t) - (\ell+m)(\ell-m+1)P_\ell^{m-1}(t).$$

**Derivative recursions:**

$$(1-t^2)\frac{d}{dt}P_\ell^m(t) = (\ell+m)P_{\ell-1}^m(t) - \ell t\,P_\ell^m(t),$$

$$(1-t^2)\frac{d}{dt}P_\ell^m(t) = -(\ell-m+1)P_{\ell+1}^m(t) + (\ell+1)t\,P_\ell^m(t).$$

These are essential for computing $P_\ell^m$ recursively in $\ell$ (for fixed $m$) and for deriving derivative formulas needed to verify that spherical harmonics satisfy the Laplace-Beltrami eigenvalue equation.

## Connection to Hypergeometric Functions

The associated Legendre equation is a special case of the hypergeometric equation (Gauss's equation). Specifically:

$$P_\ell^m(t) = \frac{(-1)^m(\ell+m)!}{2^m m!(\ell-m)!}(1-t^2)^{m/2}\,{}_2F_1\!\left(-\ell+m, \ell+m+1; m+1; \frac{1-t}{2}\right),$$

where ${}_2F_1(a,b;c;z) = \sum_{k=0}^\infty \frac{(a)_k(b)_k}{(c)_k k!}z^k$ is the Gauss hypergeometric function ($(a)_k = a(a+1)\cdots(a+k-1)$ is the Pochhammer symbol). This representation is useful for analytic continuation and asymptotic analysis.

## Parity and Special Values

- **Parity:** $P_\ell^m(-t) = (-1)^{\ell+m}P_\ell^m(t)$.
- **At the poles:** $P_\ell^m(\pm 1) = 0$ for $m \neq 0$ (since $(1-t^2)^{m/2} = 0$ at $t=\pm 1$). $P_\ell^0(1) = 1$.
- **At the equator ($t=0$):** Nonzero only for $\ell+m$ even. $P_\ell^m(0) = (-1)^{(\ell+m)/2}\frac{(\ell+m-1)!!}{(\ell-m)!!}$ when $\ell+m$ even.

## Physical Interpretation: Nodal Structure

The function $P_\ell^m(\cos\theta)$ on the sphere has $\ell - |m|$ nodal circles parallel to the equator (zeros in $\theta$) and $|m|$ nodal circles passing through the poles (from the $e^{im\phi}$ factor in the full spherical harmonic). This gives $\ell - |m| + |m| = \ell$ nodal circles total, dividing the sphere into "checkerboard" regions of alternating sign. 

For $m = \ell$ (maximum azimuthal mode): $P_\ell^\ell(\cos\theta) \propto \sin^\ell\theta$ has no nodal circles parallel to the equator — it is a function that peaks at the equator and vanishes at the poles to order $\ell$. In quantum mechanics, this corresponds to the "circular orbit" state (maximum angular momentum along $z$-axis).

For $m = 0$: $P_\ell^0(\cos\theta) = P_\ell(\cos\theta)$ has $\ell$ nodal circles (parallels of latitude), dividing the sphere into $\ell+1$ latitudinal bands. This is the "pure zonal" case, invariant under rotations about the $z$-axis.

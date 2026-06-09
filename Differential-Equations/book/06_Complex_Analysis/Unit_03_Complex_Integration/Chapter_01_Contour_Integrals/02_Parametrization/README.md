# Parametrization of Contours

The practical computation of a contour integral $\int_C f(z)\, dz$ begins with choosing a parametrization of the curve $C$. A good parametrization must be smooth (or piecewise smooth), must traverse the curve with the correct orientation, and should be algebraically convenient for the particular integrand. This section develops the standard parametrizations for the most common curves and illustrates the technique through worked examples.

## Standard Parametrizations

**Line segment from $z_1$ to $z_2$:**
$$z(t) = (1 - t)z_1 + t z_2 = z_1 + t(z_2 - z_1), \qquad t \in [0, 1].$$
Then $z'(t) = z_2 - z_1$ (constant), and $dz = (z_2 - z_1)\, dt$.

**Circular arc centered at $z_0$, radius $r$, from angle $\theta_1$ to $\theta_2$:**
$$z(t) = z_0 + re^{it}, \qquad t \in [\theta_1, \theta_2].$$
Then $z'(t) = ire^{it}$ and $|z'(t)| = r$, so arc length is $r(\theta_2 - \theta_1)$. For the full counterclockwise unit circle: $z(t) = e^{it}$, $t \in [0, 2\pi]$.

**Semicircle in the upper half-plane:**
$$z(t) = Re^{it}, \qquad t \in [0, \pi].$$
This is the standard "large semicircle" used in the upper-half-plane contour technique of Unit 04.

**Small circle around $z_0$:**
$$z(t) = z_0 + \varepsilon e^{it}, \qquad t \in [0, 2\pi],$$
where $\varepsilon > 0$ is the radius. This appears when isolating contributions near singularities.

## Contour Integral via Parametrization: Detailed Procedure

Given $C$ parametrized by $z(t)$, $t \in [a, b]$:

1. Substitute $z = z(t)$ into $f(z)$: compute $f(z(t))$.
2. Compute $z'(t)$ and multiply: $f(z(t)) z'(t)$.
3. Integrate from $a$ to $b$.

## Worked Examples

**Example 1.** Compute $\int_C \mathrm{Re}(z)\, dz$ along the unit circle traversed counterclockwise.

Parametrize: $z(t) = e^{it} = \cos t + i\sin t$, $t \in [0, 2\pi]$. Then $\mathrm{Re}(z) = \cos t$ and $z'(t) = ie^{it}$.
$$\int_0^{2\pi} \cos t \cdot ie^{it}\, dt = i\int_0^{2\pi} \cos t(\cos t + i\sin t)\, dt = i\int_0^{2\pi}(\cos^2 t + i\cos t\sin t)\, dt.$$
$$= i\left[\int_0^{2\pi}\cos^2 t\, dt + i\int_0^{2\pi}\cos t\sin t\, dt\right] = i\left[\pi + i \cdot 0\right] = \pi i.$$
Here $\int_0^{2\pi}\cos^2 t\, dt = \pi$ and $\int_0^{2\pi}\cos t\sin t\, dt = 0$. $\square$

**Example 2.** Compute $\int_C \frac{dz}{z-1}$ where $C$ is the circle $|z - 1| = 2$ traversed counterclockwise.

Parametrize: $z(t) = 1 + 2e^{it}$, $t \in [0, 2\pi]$. Then $z - 1 = 2e^{it}$ and $z'(t) = 2ie^{it}$.
$$\int_0^{2\pi} \frac{2ie^{it}}{2e^{it}}\, dt = i\int_0^{2\pi} dt = 2\pi i.$$
This confirms the fundamental result $\oint_{|z-z_0|=r} \frac{dz}{z-z_0} = 2\pi i$ for any $r > 0$. $\square$

**Example 3.** Compute $\int_C \frac{dz}{z-1}$ where $C$ is the circle $|z| = 1/2$ (which does not enclose $z = 1$) traversed counterclockwise.

Parametrize: $z(t) = \frac{1}{2}e^{it}$, $t \in [0, 2\pi]$. Then $z'(t) = \frac{i}{2}e^{it}$ and $z - 1 = \frac{1}{2}e^{it} - 1$.
$$\int_0^{2\pi} \frac{\frac{i}{2}e^{it}}{\frac{1}{2}e^{it} - 1}\, dt.$$
Since $f(z) = 1/(z-1)$ is analytic inside $|z| = 1/2$ (the singularity $z = 1$ is outside), Cauchy's theorem will give $0$. Indeed, direct computation confirms this. $\square$

**Example 4.** Compute $\int_C e^z\, dz$ where $C$ is any contour from $0$ to $\pi i$.

Since $e^z$ has antiderivative $e^z$, the integral is path-independent:
$$\int_C e^z\, dz = e^{\pi i} - e^0 = -1 - 1 = -2.$$
Let us verify by choosing the vertical segment $z(t) = it$, $t \in [0, \pi]$, with $z'(t) = i$:
$$\int_0^{\pi} e^{it} \cdot i\, dt = i\left[\frac{e^{it}}{i}\right]_0^{\pi} = [e^{it}]_0^{\pi} = e^{i\pi} - e^0 = -1 - 1 = -2. \quad \checkmark$$

## Piecewise Smooth Contours

Many useful contours are assembled from smooth pieces. For instance, a triangular contour $C = C_1 \cup C_2 \cup C_3$ is computed as $\int_C = \int_{C_1} + \int_{C_2} + \int_{C_3}$, where each $C_k$ is a straight line segment. The standard keyhole contour (used for integrands with branch cuts) consists of two radial segments and two circular arcs. Indented contours (used to avoid singularities on the real axis) include small semicircular detours. Each piece is parametrized separately and the contributions are added.

## The Winding Number

For a closed contour $C$ that does not pass through $z_0$, the integral $\frac{1}{2\pi i}\int_C \frac{dz}{z - z_0}$ counts the number of times $C$ winds around $z_0$, called the winding number $n(C, z_0)$. It is always an integer, and it is $+1$ for a simple counterclockwise loop around $z_0$, $-1$ for a clockwise loop, and $0$ for a loop that does not enclose $z_0$.

The winding number appears in the general form of Cauchy's integral formula: $\int_C \frac{f(z)}{z - z_0}\, dz = 2\pi i \cdot n(C, z_0) \cdot f(z_0)$.

## Changing Parametrization: An Example

**Example.** Compute $\int_C z^n\, dz$ for integer $n \neq -1$ around the unit circle, using two different parametrizations.

**Parametrization 1:** $z(t) = e^{it}$, $t \in [0, 2\pi]$. Then $z'(t) = ie^{it}$ and:
$$\int_0^{2\pi} e^{int} \cdot ie^{it}\, dt = i\int_0^{2\pi} e^{i(n+1)t}\, dt = i \cdot \frac{e^{i(n+1)t}}{i(n+1)}\Bigg|_0^{2\pi} = \frac{e^{2\pi i(n+1)} - 1}{n+1} = 0 \quad \text{for } n \neq -1.$$

**Parametrization 2:** $z(s) = e^{2is}$, $s \in [0, \pi]$. Then $z'(s) = 2ie^{2is}$ and:
$$\int_0^{\pi} e^{2ins} \cdot 2ie^{2is}\, ds = 2i\int_0^{\pi} e^{2i(n+1)s}\, ds = 2i \cdot \frac{e^{2\pi i(n+1)} - 1}{2i(n+1)} = 0 \quad \text{for } n \neq -1.$$

Both give $0$, confirming that $z^n$ has an antiderivative $z^{n+1}/(n+1)$ for $n \neq -1$. $\square$

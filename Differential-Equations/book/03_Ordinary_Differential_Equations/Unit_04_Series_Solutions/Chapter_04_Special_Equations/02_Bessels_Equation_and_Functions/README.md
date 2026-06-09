# Bessel's Equation and Bessel Functions

Bessel's equation of order $\nu \geq 0$ is

$$x^2 y'' + xy' + (x^2 - \nu^2)y = 0,$$

or in standard form $y'' + (1/x)y' + (1 - \nu^2/x^2)y = 0$. It arises from the Helmholtz equation $\nabla^2 u + k^2 u = 0$ in cylindrical coordinates: $u = R(r)\Theta(\theta)Z(z)$ gives $r^2 R'' + rR' + (k^2r^2 - n^2)R = 0$, which is Bessel's equation with $\nu = n$ and $x = kr$.

## Frobenius Solution: The Bessel Function $J_\nu(x)$

The regular singular point is $x = 0$. With $P_0 = 1$, $Q_0 = -\nu^2$, the indicial equation is $r^2 - \nu^2 = 0$, giving $r_1 = \nu$, $r_2 = -\nu$.

For $r_1 = \nu$, the recurrence gives:

$$a_n = \frac{-a_{n-2}}{n(n + 2\nu)}.$$

Since only even-indexed coefficients are nonzero (odd terms give 0 from the recurrence when $p_{\text{odd}} = 0$ in the coefficient expansion), with $a_0 = 1/(2^\nu \Gamma(\nu+1))$ (by convention):

$$J_\nu(x) = \sum_{m=0}^\infty \frac{(-1)^m}{m!\,\Gamma(m+\nu+1)}\left(\frac{x}{2}\right)^{2m+\nu}.$$

This is the **Bessel function of the first kind of order $\nu$**. It is analytic at $x = 0$ when $\nu \geq 0$ is an integer; for non-integer $\nu$, it has a branch point.

## The Second Solution: $Y_\nu(x)$

When $\nu$ is not an integer, $J_{-\nu}(x) = \sum_{m=0}^\infty \frac{(-1)^m}{m!\,\Gamma(m-\nu+1)}\left(\frac{x}{2}\right)^{2m-\nu}$ is the second linearly independent solution (corresponding to $r_2 = -\nu$).

When $\nu = n$ is a non-negative integer, $J_{-n}(x) = (-1)^n J_n(x)$ (linearly dependent), and the second solution is the **Bessel function of the second kind** (Neumann function):

$$Y_\nu(x) = \lim_{\mu \to \nu}\frac{J_\mu(x)\cos(\mu\pi) - J_{-\mu}(x)}{\sin(\mu\pi)}.$$

$Y_\nu(x)$ has a logarithmic singularity at $x = 0$ and is not expressible as a Frobenius series without the $\ln$ term.

## Key Properties

**Recurrence relations:** $J_{\nu-1}(x) + J_{\nu+1}(x) = (2\nu/x)J_\nu(x)$ and $J_{\nu-1}(x) - J_{\nu+1}(x) = 2J_\nu'(x)$.

**Oscillatory behavior:** For large $x$, $J_\nu(x) \approx \sqrt{2/(\pi x)}\cos(x - \nu\pi/2 - \pi/4)$: damped oscillations with infinitely many zeros.

**Zeros:** $J_\nu(x)$ has infinitely many positive zeros $0 < j_{\nu,1} < j_{\nu,2} < \cdots$. These zeros are the resonant frequencies of circular drum membranes and cylindrical waveguides.

**Orthogonality:** $\int_0^1 J_\nu(j_{\nu,m}r)J_\nu(j_{\nu,n}r)\,r\,dr = \frac{1}{2}[J_{\nu+1}(j_{\nu,m})]^2\delta_{mn}$.

## Physical Applications

Bessel functions appear in: the vibration of circular membranes (drum frequencies), the modes of cylindrical waveguides, heat conduction in cylinders, the radial part of quantum mechanical wave functions in cylindrical symmetry, and the scattering of waves by spheres (spherical Bessel functions). They are the cylindrical analogs of the sine and cosine functions.

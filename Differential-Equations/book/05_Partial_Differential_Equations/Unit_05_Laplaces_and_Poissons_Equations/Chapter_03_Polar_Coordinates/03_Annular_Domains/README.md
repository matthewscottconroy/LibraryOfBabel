# Laplace's Equation on Annular Domains

An annulus $\Omega = \{(r,\theta): a < r < b\}$ arises in modeling a cylindrical pipe (heat flow between inner and outer walls), a coaxial capacitor (electric field between two cylinders), or the velocity potential of flow with a cylindrical obstacle. The key feature distinguishing the annulus from the disk is that the solution includes both positive and negative powers of $r$ — the singularity at $r=0$ is not in the domain, so $r^{-n}$ terms are permissible.

## General Harmonic Function on the Annulus

For Laplace's equation in polar coordinates on $a < r < b$, both $r^n$ and $r^{-n}$ are bounded (and neither diverges in the interior of the annulus). Separation of variables gives:

$$u(r,\theta) = a_0 + b_0\log r + \sum_{n=1}^\infty (a_n r^n + c_n r^{-n})\cos(n\theta) + \sum_{n=1}^\infty(d_n r^n + e_n r^{-n})\sin(n\theta). \tag{1}$$

The $\log r$ term (for $n=0$) is now allowed since $r > 0$ in the annulus. It represents a logarithmic potential — the potential of a uniformly charged infinite wire at the origin.

## Dirichlet Problem on the Annulus

Prescribe:

$$u(a,\theta) = f(\theta), \qquad u(b,\theta) = g(\theta).$$

Expand $f$ and $g$ in Fourier series: $f(\theta) = \frac{A_0}{2} + \sum_{n=1}^\infty(A_n\cos n\theta + B_n\sin n\theta)$ and similarly for $g$.

For each Fourier mode $n$, the radial coefficients $a_n$, $c_n$ (or $d_n$, $e_n$) are determined by two linear equations:

$$n=0: \quad a_0 + b_0\log a = A_0/2, \quad a_0 + b_0\log b = G_0/2.$$
$$n\geq 1: \quad a_n a^n + c_n a^{-n} = A_n, \quad a_n b^n + c_n b^{-n} = C_n,$$

where $A_n$, $C_n$ are the $n$-th cosine Fourier coefficients of $f$ and $g$. The $2\times2$ system for $(a_n, c_n)$ has determinant $a^n b^{-n} - a^{-n}b^n \neq 0$ (since $a \neq b$), so it is always uniquely solvable.

## The Pure $n=0$ Problem: Radially Symmetric Case

For radially symmetric data $u(a) = u_0$ and $u(b) = u_1$ (constants), the solution depends only on $r$:

$$u(r) = a_0 + b_0\log r.$$

From $u_0 = a_0 + b_0\log a$ and $u_1 = a_0 + b_0\log b$:

$$b_0 = \frac{u_1 - u_0}{\log(b/a)}, \qquad a_0 = u_0 - b_0\log a.$$

The solution: $u(r) = u_0 + (u_1-u_0)\frac{\log(r/a)}{\log(b/a)}$.

**Physical interpretation (coaxial capacitor):** The electric potential between two coaxial cylinders of radii $a$ and $b$ (inner held at $V_0$, outer at $V_1$) is exactly this logarithmic profile. The electric field is $E_r = -u'(r) = -b_0/r$, decaying as $1/r$ — the electric field of an infinite charged cylinder.

**Heat conduction in a pipe:** The steady temperature between two coaxial cylinders with prescribed wall temperatures is the same logarithmic profile.

## Conformal Mapping to the Annulus

The annulus is conformally equivalent to no rectangle (it has a conformal invariant — the modulus $\log(b/a)/(2\pi)$ — that distinguishes it from rectangles). However, the Möbius transformation $w = (z-a)/(b-z)$ maps the disk to the right half-plane, and further maps can convert the annulus to a standard form useful for complex analytic methods.

## Connection to Complex Analysis

In 2D, any harmonic function on the annulus $a < r < b$ is the real part of a holomorphic function $F(z) = f(z) + ic\log z$ (where the $\log z$ term is needed because the annulus is not simply connected). The period of the harmonic conjugate around the annulus:

$$\oint_{|z|=r}\frac{\partial u}{\partial\theta}\,d\theta = 2\pi b_0$$

measures the "circulation" of the harmonic function — a topological invariant related to the non-simply-connected topology of the annulus. This connects potential theory on the annulus to the theory of abelian differentials and Riemann surfaces.

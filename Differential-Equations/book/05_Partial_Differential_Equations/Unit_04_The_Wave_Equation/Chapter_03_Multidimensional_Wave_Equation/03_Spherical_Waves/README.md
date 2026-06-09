# Spherical Waves

Spherical waves are solutions of the 3D wave equation with spherical symmetry — they depend only on the radial distance $r = |\mathbf{x}|$ from a central source point. They model the sound from a point source (an explosion, a speaker), the electromagnetic radiation from an antenna, and the seismic waves from an earthquake. The key algebraic fact is that the substitution $v = ru$ reduces the spherical wave equation to the 1D wave equation, making the full theory of d'Alembert immediately available.

## The Spherically Symmetric Wave Equation

For $u = u(r,t)$ with $r = |\mathbf{x}| > 0$, the 3D Laplacian becomes:

$$\Delta u = u_{rr} + \frac{2}{r}u_r = \frac{1}{r^2}\frac{\partial}{\partial r}\!\left(r^2\frac{\partial u}{\partial r}\right).$$

The wave equation $u_{tt} = c^2\Delta u$ is:

$$u_{tt} = c^2\!\left(u_{rr} + \frac{2}{r}u_r\right).$$

**The key substitution:** Set $v(r,t) = r\,u(r,t)$. Then:

$$v_{tt} = r u_{tt} = rc^2\!\left(u_{rr} + \frac{2}{r}u_r\right) = c^2(ru_{rr} + 2u_r) = c^2 v_{rr}.$$

So $v$ satisfies the 1D wave equation $v_{tt} = c^2 v_{rr}$ for $r > 0$.

## General Solution

The general solution of $v_{tt} = c^2 v_{rr}$ is $v = F(r+ct) + G(r-ct)$. Therefore:

$$u(r,t) = \frac{v}{r} = \frac{F(r+ct)}{r} + \frac{G(r-ct)}{r}.$$

The term $G(r-ct)/r$ is an **outgoing spherical wave**: as $t$ increases, the argument $r-ct$ decreases, so the wave moves in the direction of increasing $r$ (outward). The amplitude decays as $1/r$ — a consequence of energy conservation in 3D (energy spreads over a sphere of area $4\pi r^2$, so amplitude scales as $1/r$).

The term $F(r+ct)/r$ is an **incoming spherical wave**: it converges toward the origin. For a source problem (a point source at $r=0$), one typically takes $F=0$ (outgoing radiation condition).

## Regularity at the Origin

At $r=0$, $u = v/r$ must be bounded. Since $v = F(ct) + G(-ct)$ at $r=0$, we need $v(0,t) = F(ct) + G(-ct)$ to equal $ru|_{r=0}$, which requires $v(0,t) = 0$ for all $t$ (assuming $u$ is regular). This means $F(ct) = -G(-ct)$, i.e., $F(s) = -G(-s)$.

For a pure outgoing wave: $F = 0$, and the condition at the origin gives $G(0) = 0$ in the free-space problem.

## Point Source Solution

The fundamental solution of the 3D wave equation ($u_{tt} - c^2\Delta u = \delta(\mathbf{x})\delta(t)$) is:

$$E(\mathbf{x},t) = \frac{\delta(t - r/c)}{4\pi r}, \qquad r = |\mathbf{x}|.$$

This is an outgoing spherical wave emanating from $\mathbf{x}=0$ at $t=0$. It is supported on the expanding sphere $r = ct$ (the light cone), confirming the strong Huygens principle in 3D.

The factor $1/r$ ensures that the total energy is conserved: the energy flux through a sphere of radius $r$ is proportional to $r^2 |E|^2 \propto r^2/r^2 = 1$, independent of $r$.

## Cauchy Problem for Spherically Symmetric Data

If $u(r,0) = \phi(r)$ and $u_t(r,0) = \psi(r)$ (with $\phi, \psi$ extended as even functions to $r < 0$ to ensure regularity at $r=0$), then $v(r,0) = r\phi(r)$ and $v_t(r,0) = r\psi(r)$.

Applying d'Alembert:

$$v(r,t) = \frac{(r+ct)\phi(r+ct) + (r-ct)\phi(r-ct)}{2} + \frac{1}{2c}\int_{r-ct}^{r+ct}s\psi(s)\,ds.$$

Dividing by $r$ gives the explicit solution $u(r,t) = v(r,t)/r$.

For a pulse initial condition $\phi(r) = \delta(r-r_0)/(4\pi r_0)$ (a thin spherical shell of radius $r_0$), the solution evolves as two outgoing shells: one at $r = r_0 + ct$ and one at $r = r_0 - ct$ (the inward-going one reaches the origin and reflects, continuing as an outward-going pulse). Both shells have amplitude $1/(2r)$ — the energy distributes equally between the two shells.

## The $1/r$ Decay Law

The $1/r$ decay of spherical wave amplitude (and the $1/r^2$ decay of intensity) is the **inverse square law** of physics:

- Sound intensity from a point source decays as $1/r^2$.
- Light intensity from a star decays as $1/r^2$ (the basis of the inverse square law of gravity and electrostatics in 3D).
- Earthquake amplitude decays as $\sim 1/r$ for body waves in a homogeneous medium.

In 2D, the amplitude of cylindrical waves (from a line source) decays as $1/\sqrt{r}$, corresponding to $1/r$ decay of intensity. In 1D (plane waves), there is no decay — the amplitude is constant.

## Nonradial Spherical Waves

For waves with angular dependence $u = u(r,\theta,\phi,t)$, the separation of variables leads to:

$$u = \sum_{\ell=0}^\infty\sum_{m=-\ell}^\ell \frac{v_{\ell}(r,t)}{r}Y_\ell^m(\theta,\phi),$$

where the radial part satisfies:

$$\frac{\partial^2 v_\ell}{\partial t^2} = c^2\frac{\partial^2 v_\ell}{\partial r^2} - \frac{c^2\ell(\ell+1)}{r^2}v_\ell.$$

This is the 1D wave equation perturbed by a centrifugal potential $\ell(\ell+1)/r^2$. For $\ell = 0$ it reduces to the standard 1D wave equation; for $\ell > 0$, the solutions are spherical Bessel functions $j_\ell(kr)$ and $y_\ell(kr)$ times $e^{\pm i\omega t}$.

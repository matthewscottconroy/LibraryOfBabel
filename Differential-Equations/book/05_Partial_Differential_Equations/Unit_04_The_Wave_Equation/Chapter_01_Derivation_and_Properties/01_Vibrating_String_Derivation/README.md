# Derivation: The Vibrating String

A taut string under tension is one of the oldest and most instructive mechanical systems in physics. Its mathematics, worked out by d'Alembert, Euler, Daniel Bernoulli, and Lagrange in the 18th century, laid the foundations for the entire theory of wave propagation and Fourier analysis. The derivation from Newton's second law is a model of applied mathematics at its best.

## Physical Setup and Assumptions

Consider a string of length $L$ stretched along the $x$-axis under tension $T$ (newtons). The string is assumed to be:

1. **Flexible** (no bending stiffness): the only internal force is tension directed tangentially to the string.
2. **Under small deflections**: the transverse displacement $u(x,t)$ is small compared to $L$, so $|u_x| \ll 1$.
3. **Of uniform linear density** $\rho$ (kg/m), independent of position and displacement.
4. **Undamped**: no energy dissipation from air resistance or internal friction.
5. **The tension $T$ is constant** (a consequence of the small-amplitude assumption).

## Applying Newton's Second Law

Consider a small element of the string between $x$ and $x + \Delta x$. The mass of this element is $\rho\,\Delta x$. The forces on it are the tensions at the two ends, directed tangentially:

- At $x+\Delta x$: tension $T$ at angle $\theta(x+\Delta x,t)$ above the horizontal.
- At $x$: tension $T$ at angle $\theta(x,t)$ below the horizontal (pointing leftward and downward).

The net vertical (transverse) force on the element is:

$$F_\perp = T\sin\theta(x+\Delta x) - T\sin\theta(x).$$

Under the small-amplitude assumption, $\sin\theta \approx \tan\theta = u_x$. So:

$$F_\perp \approx T(u_x(x+\Delta x,t) - u_x(x,t)) \approx Tu_{xx}(x,t)\,\Delta x.$$

Newton's second law ($F = ma$) applied to the transverse direction:

$$\rho\,\Delta x\,u_{tt}(x,t) = Tu_{xx}(x,t)\,\Delta x.$$

Dividing by $\rho\,\Delta x$ and letting $\Delta x \to 0$:

$$u_{tt} = c^2 u_{xx}, \qquad c = \sqrt{\frac{T}{\rho}}. \tag{1}$$

This is the **one-dimensional wave equation** with wave speed $c$.

## Physical Meaning of the Wave Speed

The wave speed $c = \sqrt{T/\rho}$ has a clear physical interpretation: a tighter string (larger $T$) or lighter string (smaller $\rho$) supports faster wave propagation. Typical values:

- Piano string (steel, high tension): $c \approx 400$ m/s.
- Guitar string (nylon): $c \approx 120$ m/s.
- Sound wave in air: $c \approx 343$ m/s (different mechanism, same equation form).
- Light in vacuum: $c = 3\times 10^8$ m/s (electromagnetic wave equation).

## Boundary and Initial Conditions

To specify a unique solution:

**Boundary conditions** (string fixed at both ends):
$$u(0,t) = 0, \qquad u(L,t) = 0, \qquad t > 0.$$

These Dirichlet conditions model the string attached to fixed walls. Alternative: free endpoints ($u_x = 0$) model frictionless rings through which the string passes.

**Initial conditions:**
$$u(x,0) = \phi(x) \qquad \text{(initial displacement)},$$
$$u_t(x,0) = \psi(x) \qquad \text{(initial velocity)}.$$

Both are needed because the wave equation is second-order in time. Plucking a string gives a nonzero $\phi$ with $\psi = 0$. Striking a string (as in a piano) gives $\phi = 0$ with a nonzero $\psi$.

## The General Solution

The wave equation (1) can be factored:

$$\left(\frac{\partial}{\partial t} + c\frac{\partial}{\partial x}\right)\left(\frac{\partial}{\partial t} - c\frac{\partial}{\partial x}\right)u = 0.$$

The characteristics are the lines $x + ct = \text{const}$ (slope $-c$ in the $xt$-plane) and $x - ct = \text{const}$ (slope $+c$). In characteristic coordinates $\xi = x + ct$, $\eta = x - ct$, the wave equation reduces to:

$$u_{\xi\eta} = 0.$$

The general solution of this is $u = f(\xi) + g(\eta) = f(x+ct) + g(x-ct)$, where $f$ and $g$ are arbitrary $C^2$ functions. The solution is a superposition of a right-traveling wave $g(x-ct)$ and a left-traveling wave $f(x+ct)$, each maintaining its shape as it propagates at speed $c$.

## Conservation of Energy

Multiply equation (1) by $u_t$ and integrate over $[0,L]$:

$$\int_0^L u_t u_{tt}\,dx = c^2\int_0^L u_t u_{xx}\,dx.$$

The left side is $\frac{d}{dt}\frac{1}{2}\int u_t^2\,dx$. Integration by parts on the right (using $u_t(0,t) = u_t(L,t) = 0$ from the Dirichlet conditions):

$$c^2\int_0^L u_t u_{xx}\,dx = -c^2\int_0^L u_{tx}u_x\,dx = -\frac{d}{dt}\frac{c^2}{2}\int u_x^2\,dx.$$

Therefore:

$$\frac{d}{dt}E(t) = 0, \qquad E(t) = \frac{1}{2}\int_0^L\left(u_t^2 + c^2 u_x^2\right)dx.$$

The energy $E$ is conserved. The two terms are the kinetic energy density $\frac{1}{2}\rho u_t^2$ (times $1/\rho$) and the potential energy density $\frac{1}{2}T u_x^2$ (times $1/T \cdot c^2 = 1/\rho$).

Energy conservation immediately implies uniqueness: if $w = u_1 - u_2$ satisfies the wave equation with zero initial data, then $E(t) = E(0) = 0$ for all $t$, so $w_t = w_x = 0$ everywhere, hence $w$ is constant, and $w(x,0) = 0$ gives $w \equiv 0$.

## Wave Equation for Other Media

The same derivation applies (with appropriate modifications) to:

**Sound waves** in a gas: $p_{tt} = c_s^2 p_{xx}$ (or $\Delta p$) where $p$ is pressure perturbation and $c_s = \sqrt{\gamma p_0/\rho_0}$ is the adiabatic sound speed.

**Electromagnetic waves** in vacuum: the electric field $\mathbf{E}$ satisfies $\mathbf{E}_{tt} = c^2\Delta\mathbf{E}$ with $c = 1/\sqrt{\varepsilon_0\mu_0}$ — Maxwell's prediction of the speed of light.

**Elastic waves** in a solid: longitudinal (P-waves) and transverse (S-waves) with different speeds, satisfying vector wave equations — the foundation of seismology.

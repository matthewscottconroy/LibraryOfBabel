# Standing Waves

A standing wave is a solution of the wave equation that appears to "stand still" spatially — different parts of the domain oscillate up and down, but the spatial pattern does not travel. This contrasts with a traveling wave (such as d'Alembert's solution) where the entire profile moves. Standing waves arise naturally when a domain has boundaries — the wave reflects back and forth, and the superposition of forward and backward waves creates the standing pattern.

## Derivation via Separation of Variables

We seek solutions of the wave equation on $[0,L]$ with Dirichlet boundary conditions:

$$u_{tt} = c^2 u_{xx}, \quad u(0,t) = u(L,t) = 0.$$

Try $u(x,t) = X(x)T(t)$. Substituting and separating:

$$\frac{T''(t)}{c^2 T(t)} = \frac{X''(x)}{X(x)} = -\lambda.$$

The boundary conditions give $X(0) = X(L) = 0$.

**Spatial problem:** $X'' + \lambda X = 0$, $X(0) = X(L) = 0$.

As in the heat equation analysis, the only nontrivial solutions occur for $\lambda > 0$, with:

$$\lambda_n = \left(\frac{n\pi}{L}\right)^2, \qquad X_n(x) = \sin\!\left(\frac{n\pi x}{L}\right), \qquad n = 1, 2, 3, \ldots$$

**Temporal problem:** $T'' + c^2\lambda_n T = 0$, which is a simple harmonic oscillator:

$$T_n(t) = A_n\cos(\omega_n t) + B_n\sin(\omega_n t), \qquad \omega_n = c\sqrt{\lambda_n} = \frac{cn\pi}{L}.$$

The angular frequency $\omega_n$ determines the oscillation rate; the corresponding ordinary frequency is $f_n = \omega_n/(2\pi) = cn/(2L)$.

## The Standing Wave Solutions

The product solutions are:

$$u_n(x,t) = \sin\!\left(\frac{n\pi x}{L}\right)\left[A_n\cos\!\left(\frac{cn\pi t}{L}\right) + B_n\sin\!\left(\frac{cn\pi t}{L}\right)\right].$$

Each $u_n$ is a standing wave: the spatial shape $\sin(n\pi x/L)$ oscillates harmonically in time at frequency $\omega_n$. The nodes (points where $u_n = 0$ for all $t$) are fixed at $x = 0, L/n, 2L/n, \ldots, L$ — exactly $n-1$ interior nodes for the $n$-th mode.

## Relation to Traveling Waves

A standing wave can be decomposed into traveling waves via the product-to-sum formula:

$$\sin\!\left(\frac{n\pi x}{L}\right)\cos(\omega_n t) = \frac{1}{2}\left[\sin\!\left(\frac{n\pi x}{L} + \omega_n t\right) + \sin\!\left(\frac{n\pi x}{L} - \omega_n t\right)\right].$$

The first term $\sin(n\pi x/L + \omega_n t)$ is a left-traveling wave; the second is a right-traveling wave. The standing wave is the superposition of two traveling waves of equal amplitude moving in opposite directions.

This connection shows that standing waves arise from wave reflections at the boundary: the wave travels to the right, reflects at $x=L$, travels back to the left, reflects again at $x=0$, and the forward and reflected waves interfere constructively to form a stable standing pattern when the frequency is exactly right ($\omega = cn\pi/L$).

## Nodal Structure

The $n$-th mode has $n-1$ interior nodes, located at $x_k = kL/n$ for $k=1,\ldots,n-1$. The spatial wavelength of the $n$-th mode is $\lambda_n^{\text{phys}} = 2L/n$ (not to be confused with the eigenvalue $\lambda_n$ — the notation is unfortunately standard). The condition for a standing wave is that an integer number $n$ of half-wavelengths fit in the domain: $n\cdot(\lambda_n^{\text{phys}}/2) = L$.

For the fundamental mode ($n=1$): one half-wavelength fits in $[0,L]$, with a single arch and no interior nodes.

For $n=2$: two half-wavelengths, with one interior node at $x=L/2$.

## Energy of a Standing Wave

The energy of the $n$-th mode at amplitude $A_n$ is:

$$E_n = \frac{1}{4}\rho A_n^2\omega_n^2 L + \frac{c^2\rho}{4}\left(\frac{n\pi}{L}\right)^2 A_n^2 L = \frac{1}{4}\rho\omega_n^2 A_n^2 L\left[1 + 1\right] = \frac{\rho\omega_n^2 A_n^2 L}{2},$$

where the kinetic and potential energies are equal on average (equipartition for harmonic oscillators). Higher modes ($n$ large) have higher energy for the same amplitude, due to the $\omega_n^2 = (cn\pi/L)^2$ factor.

## Example: Plucked String

A string plucked to a triangular shape $\phi(x) = (2A/L)\min(x, L-x)$ and released from rest ($\psi = 0$):

$A_n = \frac{2}{L}\int_0^L\phi(x)\sin(n\pi x/L)\,dx$. The integral evaluates to $A_n = 8A/(n^2\pi^2)$ for odd $n$ and $A_n = 0$ for even $n$ (by symmetry, $\phi$ is symmetric about $x=L/2$, so only odd modes contribute).

The solution is:

$$u(x,t) = \frac{8A}{\pi^2}\sum_{k=0}^\infty \frac{(-1)^k}{(2k+1)^2}\sin\!\left(\frac{(2k+1)\pi x}{L}\right)\cos\!\left(\frac{(2k+1)c\pi t}{L}\right).$$

The fundamental frequency $f_1 = c/(2L)$ dominates (coefficient $\propto 1$), with the third harmonic at $1/9$ the amplitude, the fifth at $1/25$, etc. This rapid decrease in amplitude explains why a plucked string has a strong fundamental and weak overtones — it sounds "pure."

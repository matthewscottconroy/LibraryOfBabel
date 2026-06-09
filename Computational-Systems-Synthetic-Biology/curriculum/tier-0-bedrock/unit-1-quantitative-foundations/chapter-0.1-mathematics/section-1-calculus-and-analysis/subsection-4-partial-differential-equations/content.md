# Partial Differential Equations

In 1952, Alan Turing published a paper that would eventually be recognized as one of the most original contributions to theoretical biology ever written. He was not trying to explain a specific organism. He was asking a mathematical question: can chemistry alone generate spatial patterns from a uniform initial state? His answer — that a system of two diffusing, reacting chemicals could spontaneously break spatial symmetry — was arrived at by analyzing what we now call reaction-diffusion partial differential equations.

Turing's insight illustrates the power and the necessity of PDEs for biology. When biological quantities vary not just in time but also in space — a morphogen gradient across an embryo, a wave of calcium signaling sweeping through a cell, bacteria diffusing through tissue — you need partial differential equations. PDEs describe functions of multiple independent variables, and their solutions are functions rather than numbers. Understanding their structure is essential for modeling spatial phenomena in biology: it is what connects the mathematics of diffusion to the biology of body plans.

## The Diffusion Equation

The most important PDE in biological modeling is the **diffusion equation** (also called the heat equation):

$$\frac{\partial u}{\partial t} = D \nabla^2 u = D \left(\frac{\partial^2 u}{\partial x^2} + \frac{\partial^2 u}{\partial y^2} + \frac{\partial^2 u}{\partial z^2}\right)$$

where $u(\mathbf{x}, t)$ is concentration and $D$ is the diffusion coefficient. In one spatial dimension:

$$\frac{\partial u}{\partial t} = D \frac{\partial^2 u}{\partial x^2}$$

This is **Fick's second law**: the rate of change of concentration at a point is proportional to the local curvature of the concentration profile. Where concentration has a "peak," $\partial^2 u/\partial x^2 < 0$ and the concentration decreases; where it has a "valley," $\partial^2 u/\partial x^2 > 0$ and concentration increases. Diffusion always acts to flatten gradients. The second derivative is literally measuring how pointy the concentration profile is — and the more pointy, the faster it smooths.

**Fundamental solution:** Given a point source of amount $M$ at $x = 0$, $t = 0$, the solution for an infinite domain is the Gaussian:

$$u(x, t) = \frac{M}{\sqrt{4\pi D t}} \exp\left(-\frac{x^2}{4Dt}\right)$$

The width of this Gaussian spreads as $\sigma(t) = \sqrt{2Dt}$, giving the characteristic diffusion length scale. For a morphogen with $D = 1\ \mu\text{m}^2/\text{s}$ over a timescale of $t = 1$ hour ($3600$ s), the characteristic length is $\sqrt{2 \times 1 \times 3600} \approx 85\ \mu\text{m}$ — on the order of typical developmental patterning scales. This is not a coincidence. Morphogen gradients are functional precisely because diffusion, over developmental timescales, produces spatial length scales that match tissue dimensions.

## Reaction-Diffusion Systems

The most powerful PDE models in biology combine diffusion with local chemical reactions:

$$\frac{\partial u}{\partial t} = D_u \nabla^2 u + f(u, v)$$

$$\frac{\partial v}{\partial t} = D_v \nabla^2 v + g(u, v)$$

This is the **reaction-diffusion system**, which Alan Turing analyzed in his landmark 1952 paper "The Chemical Basis of Morphogenesis." Turing showed that even if each species is at a spatially uniform stable equilibrium in the ODE system, spatial instabilities can arise when diffusion is included — specifically when the **inhibitor** diffuses much faster than the **activator** ($D_v \gg D_u$).

The **Turing instability** condition requires that a uniform steady state that is stable without diffusion becomes unstable with diffusion. This produces spontaneous spatial patterns — stripes, spots, labyrinthine patterns — from a homogeneous initial condition. You might expect that adding diffusion, which smooths out spatial variation, would make a system *more* uniform. It turns out the opposite can happen, and the reason is subtle: the inhibitor, diffusing fast, dampens perturbations at long wavelengths; the activator, diffusing slow, amplifies perturbations at short wavelengths; the net effect is spontaneous patterning at an intermediate wavelength determined by the ratio $D_v/D_u$. These patterns are believed to underlie animal coat patterns (cheetah spots, zebra stripes), digit formation in vertebrate limbs, and feather follicle spacing.

**Linear stability analysis of reaction-diffusion systems:** Perturb the uniform steady state $(u^*, v^*)$ by $\delta u = \hat{u} e^{\sigma t + ikx}$, $\delta v = \hat{v} e^{\sigma t + ikx}$. Substituting into the linearized PDE gives the **dispersion relation** relating growth rate $\sigma$ to wavenumber $k$:

$$\det(J_{\text{RD}}(k)) = 0$$

where $J_{\text{RD}} = J - Dk^2$ with $J$ being the reaction Jacobian and $D = \text{diag}(D_u, D_v)$. A Turing instability exists when there is a range of $k > 0$ for which $\text{Re}(\sigma) > 0$ while the $k = 0$ mode is stable.

## Boundary Conditions

PDEs require **boundary conditions** in addition to initial conditions. The solution you get depends critically on what is happening at the boundaries of your spatial domain — and different boundary conditions correspond to very different biological scenarios.

- **Dirichlet boundary conditions**: fix the value of $u$ at the boundary ($u|_{\partial \Omega} = u_0$). Applicable when concentration is clamped at a tissue boundary — for example, a localized source that maintains a fixed concentration.
- **Neumann boundary conditions**: fix the flux (normal derivative) at the boundary ($\partial u/\partial n|_{\partial \Omega} = 0$ for no-flux, meaning no material crosses the boundary). Appropriate for an isolated tissue with no external sources or sinks.
- **Robin boundary conditions**: a linear combination: $\alpha u + \beta \partial u/\partial n = \gamma$ at the boundary.

The choice of boundary condition dramatically changes the solution. A morphogen source at one end and a sink (Dirichlet, $u = 0$) at the other produces a steady-state exponential gradient; reflecting boundaries (Neumann, no-flux) produce flat equilibria. Before running any PDE simulation, specify and justify your boundary conditions — they are not a technical detail, they are part of the biological model.

## Method of Characteristics

For **first-order PDEs** of the form:

$$a(x, y) \frac{\partial u}{\partial x} + b(x, y) \frac{\partial u}{\partial y} = c(x, y, u)$$

the **method of characteristics** transforms the PDE into a system of ODEs along characteristic curves. This technique is used in population balance equations that track cell age or size distributions, and in the analysis of traveling wave solutions.

## Finite Difference Discretization

For complex geometries or nonlinear reaction terms, PDEs are solved numerically by replacing continuous derivatives with finite differences on a grid. Given grid spacing $\Delta x$ and time step $\Delta t$:

$$\frac{u_i^{n+1} - u_i^n}{\Delta t} = D \frac{u_{i+1}^n - 2u_i^n + u_{i-1}^n}{(\Delta x)^2} + f(u_i^n)$$

The **Courant-Friedrichs-Lewy (CFL) condition** for explicit schemes requires $D \Delta t / (\Delta x)^2 \leq 1/2$ for numerical stability — a constraint that forces very small time steps when spatial resolution is fine. Implicit schemes (Crank-Nicolson) are unconditionally stable but require solving a linear system at each time step. This tradeoff between stability and computational cost is a recurring theme in scientific computing that you will encounter whenever you simulate spatial models.

## Why This Matters for Computational Biology

Spatial organization is a defining feature of multicellular biology. Morphogen gradients specify cell fates in developing embryos; calcium waves propagate across sheets of heart muscle; bacterial colonies form spatial patterns driven by diffusion and chemotaxis. PDEs are the tools for modeling all of these. Even within a single cell, the spatial distribution of signaling molecules matters: mRNA localization in *Drosophila* oocytes, the gradient of Ran-GTP across the nucleus, and the spatial pattern of PIP3 in a migrating cell are all PDE problems. Connecting the mathematical structure of reaction-diffusion systems to the biological phenomena they predict is one of the deepest payoffs in computational biology.

```python
import numpy as np
import matplotlib.pyplot as plt

# Solve 1D diffusion equation numerically (explicit scheme)
D = 1.0         # diffusion coefficient (um^2/s)
L = 100.0       # domain length (um)
T = 3600.0      # total time (s)
nx = 200        # spatial grid points
nt = 10000      # time steps

dx = L / (nx - 1)
dt = T / nt
x = np.linspace(0, L, nx)

# Stability check
r = D * dt / dx**2
assert r <= 0.5, f"Unstable: r = {r:.3f}"

# Initial condition: point source at center
u = np.zeros(nx)
u[nx // 2] = 1.0 / dx  # normalized to unit total amount

snapshots = []
for n in range(nt):
    u_new = u.copy()
    u_new[1:-1] = u[1:-1] + r * (u[2:] - 2*u[1:-1] + u[:-2])
    u_new[0] = u_new[1]    # no-flux at left boundary
    u_new[-1] = u_new[-2]  # no-flux at right boundary
    u = u_new
    if n in [100, 500, 2000, 9999]:
        snapshots.append((n * dt, u.copy()))

for t_snap, u_snap in snapshots:
    plt.plot(x, u_snap, label=f't = {t_snap:.0f} s')
plt.xlabel('Position (μm)')
plt.ylabel('Concentration (nM/μm)')
plt.title('1D Diffusion from Point Source')
plt.legend()
plt.tight_layout()
```

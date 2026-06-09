# Spectral Methods for PDEs

There is something almost magical about what happens when you represent a smooth function as a sum of sine and cosine waves. Instead of tracking the function's value at hundreds of grid points, you track a handful of amplitudes — one for each frequency. To differentiate the function, you do not compute finite differences between neighboring points; you simply multiply each amplitude by its wavenumber. The operation that would introduce $O(\Delta x^2)$ truncation error in a finite difference scheme is, in spectral space, exact.

**Spectral methods** represent the solution to a PDE as a global series of smooth basis functions — typically Fourier modes (for periodic problems) or Chebyshev polynomials (for bounded non-periodic domains). Unlike finite differences, which achieve $O(\Delta x^2)$ convergence, spectral methods achieve **exponential convergence** for smooth solutions: the error decreases faster than any power of the number of modes. This makes them extraordinarily efficient when the solution is well-behaved, and the method of choice for problems such as Turing pattern formation, fluid dynamics of biological flows, and spectral analysis of spatiotemporal gene expression data.

## Fourier Spectral Methods for Periodic Problems

For a periodic domain $[0, L]$ (periodic boundary conditions), any smooth function can be expanded in a discrete Fourier series:

$$u(x) \approx \sum_{k=-N/2}^{N/2-1} \hat{u}_k e^{2\pi i k x / L}$$

The key property: differentiation in physical space becomes **multiplication in spectral space**:

$$\widehat{\partial u / \partial x}_k = \frac{2\pi i k}{L} \hat{u}_k$$

This transforms the spatial derivative operator from a matrix multiplication (finite differences) into an element-wise multiplication of the Fourier coefficients — a far cheaper operation, and exact to machine precision for the represented modes.

## The Fast Fourier Transform

The bridge between physical-space values $\{u_j\}$ and spectral coefficients $\{\hat{u}_k\}$ is the **Discrete Fourier Transform (DFT)**, computed in $O(N \log N)$ operations by the **Fast Fourier Transform (FFT)**:

$$\hat{u}_k = \sum_{j=0}^{N-1} u_j e^{-2\pi i jk/N}, \quad k = 0, \ldots, N-1$$

SciPy and NumPy provide highly optimized FFT implementations:

```python
import numpy as np
from scipy.fft import fft, ifft, fftfreq
import matplotlib.pyplot as plt

# Solve the 1D diffusion equation spectrally (periodic domain)
# du/dt = D d²u/dx²

L = 2 * np.pi       # domain length
N = 256             # number of grid points (power of 2 for FFT efficiency)
x = np.linspace(0, L, N, endpoint=False)
D = 0.01            # diffusion coefficient

# Initial condition: localized Gaussian pulse
u0 = np.exp(-10 * (x - np.pi)**2)

# Wavenumbers (multiplied by 2π/L = 1 since L=2π)
k = fftfreq(N, d=1.0/N)   # integer wavenumbers
k_vals = 2 * np.pi * k / L  # physical wavenumbers

# Spectral coefficient of initial condition
u_hat = fft(u0)

# Exact spectral solution: u_hat(k, t) = u_hat0(k) * exp(-D k^2 t)
# Each mode decays independently — no numerical diffusion!
def spectral_solve(t):
    u_hat_t = u_hat * np.exp(-D * k_vals**2 * t)
    return np.real(ifft(u_hat_t))

# Compare with analytical solution for diffusion on periodic domain
def analytical_diffusion(x, t, u0_fn, L, N_terms=50):
    """Analytical solution via Fourier series."""
    u = np.zeros_like(x)
    for n in range(-N_terms, N_terms+1):
        kn = 2 * np.pi * n / L
        cn = np.trapz(u0_fn * np.exp(-1j * kn * x), x) / L
        u += np.real(cn * np.exp(1j * kn * x) * np.exp(-D * kn**2 * 1.0))
    return u

t_eval = [0.1, 0.5, 2.0]
fig, axes = plt.subplots(1, 3, figsize=(12, 4))
for ax, t in zip(axes, t_eval):
    u_spec = spectral_solve(t)
    ax.plot(x, u_spec, 'C0-', label='Spectral', lw=2)
    ax.set_title(f't = {t}')
    ax.set_xlabel('x')
    ax.set_ylabel('u(x,t)')
plt.tight_layout()
plt.savefig('spectral_diffusion.pdf')
```

## Nonlinear Terms and Dealiasing

For nonlinear PDEs like the Turing reaction-diffusion system, products of functions appear (e.g., $u^2$). Computing $u^2$ in spectral space requires convolving $\hat{u}$ with itself, which introduces **aliasing errors** — spurious energy appearing at low wavenumbers from high-wavenumber interactions that cannot be represented on the grid.

The standard solution is the **2/3 rule (padding)**: zero out the top 1/3 of Fourier modes before transforming to physical space, compute the nonlinear product there, then transform back. This prevents aliasing at the cost of slightly reduced resolution.

```python
def dealias(u_hat, N):
    """Zero out top 1/3 of modes to prevent aliasing."""
    u_hat_d = u_hat.copy()
    cutoff = N // 3
    u_hat_d[cutoff:-cutoff] = 0
    return u_hat_d

def turing_spectral_step(a_hat, h_hat, k2, dt, rho=0.01, mu=0.02, nu=0.02, 
                          D_a=0.01, D_h=1.0):
    """
    Single time step for Gierer-Meinhardt Turing system using
    spectral diffusion + forward Euler reaction.
    
    da/dt = D_a * d²a/dx² + rho*a²/h - mu*a
    dh/dt = D_h * d²h/dx² + rho*a² - nu*h
    """
    # Transform to physical space (dealias for nonlinear products)
    a = np.real(ifft(dealias(a_hat, len(a_hat))))
    h = np.real(ifft(dealias(h_hat, len(h_hat))))
    
    # Compute nonlinear reaction terms in physical space
    a2 = a**2
    reaction_a = rho * a2 / (h + 1e-10) - mu * a
    reaction_h = rho * a2 - nu * h
    
    # Transform reactions to spectral space
    ra_hat = fft(reaction_a)
    rh_hat = fft(reaction_h)
    
    # Integrating factor for diffusion (exact in spectral space)
    # Implicit diffusion + explicit reaction (IMEX scheme)
    diffusion_a = np.exp(-D_a * k2 * dt)
    diffusion_h = np.exp(-D_h * k2 * dt)
    
    a_hat_new = diffusion_a * (a_hat + dt * ra_hat)
    h_hat_new = diffusion_h * (h_hat + dt * rh_hat)
    
    return a_hat_new, h_hat_new
```

## Turing Pattern Formation: Worked Example

```python
N = 256
L = 100.0
x = np.linspace(0, L, N, endpoint=False)
k = 2 * np.pi * fftfreq(N, d=L/N)
k2 = k**2  # k squared for diffusion operator

# Random initial condition near homogeneous steady state
rng = np.random.default_rng(42)
a0 = 1.0 + 0.01 * rng.standard_normal(N)
h0 = 1.0 + 0.01 * rng.standard_normal(N)

a_hat = fft(a0)
h_hat = fft(h0)

dt = 0.1
n_steps = 5000
save_every = 500

snapshots = []
for step in range(n_steps):
    a_hat, h_hat = turing_spectral_step(a_hat, h_hat, k2, dt)
    if step % save_every == 0:
        snapshots.append(np.real(ifft(a_hat)).copy())
        print(f"Step {step}: max(a) = {snapshots[-1].max():.3f}")

# Final pattern shows spatial periodicity at wavelength lambda = 2*pi/k_max
# where k_max maximizes the linear growth rate
```

## Chebyshev Methods for Non-Periodic Domains

For non-periodic problems (e.g., a cell with specified boundary conditions at each end), **Chebyshev spectral methods** expand the solution on $[-1, 1]$ in Chebyshev polynomials $T_n(x)$. The **Chebyshev differentiation matrix** $D$ maps nodal values to values of the derivative at the same nodes. The method achieves the same exponential convergence as Fourier methods but with non-periodic boundary conditions built in naturally.

The **Dedalus** Python framework implements spectral methods for PDEs in 1D, 2D, and 3D with automatic differentiation matrices and support for Fourier, Chebyshev, Legendre, and spherical harmonic bases:

```python
# Dedalus example: 2D Turing pattern in a square domain
# pip install dedalus
import dedalus.public as d3

coords = d3.CartesianCoordinates('x', 'y')
dist = d3.Distributor(coords, dtype=np.float64)
xbasis = d3.RealFourier(coords['x'], size=128, bounds=(0, 100))
ybasis = d3.RealFourier(coords['y'], size=128, bounds=(0, 100))

a = dist.Field(name='a', bases=(xbasis, ybasis))
h = dist.Field(name='h', bases=(xbasis, ybasis))

# Problem specification: da/dt = D_a*Δa + rho*a²/h - mu*a
problem = d3.IVP([a, h], namespace=locals())
problem.add_equation("dt(a) - D_a*lap(a) = rho*a**2/h - mu*a")
problem.add_equation("dt(h) - D_h*lap(h) = rho*a**2 - nu*h")
```

## Why This Matters

Spectral methods achieve accuracy levels that would require millions of finite-difference grid points using only hundreds of modes. For problems where the solution is smooth — morphogen gradients, Turing patterns, spectral analysis of gene expression oscillations — they offer a uniquely powerful combination of accuracy, efficiency, and mathematical elegance. The FFT-based implementation in NumPy/SciPy makes them accessible to any computational biologist with Python fluency.

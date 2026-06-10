"""
Standard benchmark generators for reservoir computing experiments.

All functions return NumPy arrays. Conventions:
  - NARMA: returns (u, y) where u is the driving input and y is the target
  - Lorenz/Mackey-Glass: returns the time series (shape (T,) or (T, d))
  - All sequences start at a settled state (burn-in discarded)
"""

import numpy as np
from numpy.random import default_rng


# ------------------------------------------------------------------
# NARMA (Nonlinear AutoRegressive Moving Average)
# ------------------------------------------------------------------

def narma(
    T: int = 10_000,
    order: int = 10,
    burn: int = 200,
    seed: int = 0,
) -> tuple[np.ndarray, np.ndarray]:
    """
    NARMA-n benchmark (Jaeger 2002, Atiya & Parlos 2000).

    The NARMA-10 recurrence:
        y_{t+1} = 0.3 y_t
                + 0.05 y_t (sum_{i=0}^{n-1} y_{t-i})
                + 1.5 u_{t-n+1} u_t
                + 0.1

    The target signal depends on the full past n steps, making it a
    hard benchmark for memory-limited models.

    Parameters
    ----------
    T     : total length (after burn-in)
    order : n in NARMA-n (typically 10 or 20)
    burn  : burn-in steps to discard
    seed  : random seed

    Returns
    -------
    u : (T,) driving input,  uniform on [0, 0.5]
    y : (T,) target output
    """
    rng = default_rng(seed)
    total = T + burn + order
    u = rng.uniform(0.0, 0.5, total)
    y = np.zeros(total)

    for t in range(order, total - 1):
        y[t + 1] = (
            0.3 * y[t]
            + 0.05 * y[t] * np.sum(y[t - order + 1 : t + 1])
            + 1.5 * u[t - order + 1] * u[t]
            + 0.1
        )

    start = burn + order
    return u[start:], y[start:]


# ------------------------------------------------------------------
# Mackey-Glass delay differential equation
# ------------------------------------------------------------------

def mackey_glass(
    T: int = 5_000,
    tau: int = 17,
    dt: float = 1.0,
    burn: int = 1_000,
    seed: int = 0,
) -> np.ndarray:
    """
    Mackey-Glass time series (Mackey & Glass 1977).

    dx/dt = β x(t-τ) / (1 + x(t-τ)^n) - γ x(t)
    with β=0.2, γ=0.1, n=10.

    τ=17 → weakly chaotic; τ=30 → more chaotic.
    Euler integration with step dt=1.0 is standard for benchmarks.

    Returns
    -------
    x : (T,) time series
    """
    rng = default_rng(seed)
    beta, gamma, n = 0.2, 0.1, 10
    history_len = max(tau + 1, 100)
    x = rng.uniform(0.1, 1.3) * np.ones(history_len + burn + T)

    # Euler integration
    for t in range(history_len - 1, history_len + burn + T - 1):
        x_tau = x[t - tau]
        dx = beta * x_tau / (1.0 + x_tau**n) - gamma * x[t]
        x[t + 1] = x[t] + dt * dx

    return x[history_len + burn : history_len + burn + T]


# ------------------------------------------------------------------
# Lorenz system (RK4 integration)
# ------------------------------------------------------------------

def lorenz_rk4(
    T: int = 10_000,
    dt: float = 0.02,
    sigma: float = 10.0,
    rho: float = 28.0,
    beta: float = 8.0 / 3.0,
    burn: int = 500,
    seed: int = 0,
) -> np.ndarray:
    """
    Lorenz attractor via 4th-order Runge-Kutta.

    dx/dt = σ(y - x)
    dy/dt = x(ρ - z) - y
    dz/dt = xy - βz

    Returns
    -------
    xyz : (T, 3) array of (x, y, z) coordinates
    """
    rng = default_rng(seed)
    state = rng.standard_normal(3) * 0.1

    def _deriv(s):
        x, y, z = s
        return np.array([
            sigma * (y - x),
            x * (rho - z) - y,
            x * y - beta * z,
        ])

    total = T + burn
    traj = np.empty((total, 3))

    for t in range(total):
        traj[t] = state
        k1 = _deriv(state)
        k2 = _deriv(state + 0.5 * dt * k1)
        k3 = _deriv(state + 0.5 * dt * k2)
        k4 = _deriv(state + dt * k3)
        state = state + (dt / 6.0) * (k1 + 2 * k2 + 2 * k3 + k4)

    return traj[burn:]


# Alias: lorenz() returns the Lorenz trajectory
lorenz = lorenz_rk4


# ------------------------------------------------------------------
# Kuramoto-Sivashinsky (1D PDE, spatial discretisation)
# ------------------------------------------------------------------

def kuramoto_sivashinsky(
    T: int = 10_000,
    L: float = 22.0,
    N: int = 64,
    dt: float = 0.25,
    burn: int = 2_000,
    seed: int = 0,
) -> np.ndarray:
    """
    Kuramoto-Sivashinsky PDE: u_t + u u_x + u_xx + u_xxxx = 0
    Periodic BCs, pseudo-spectral spatial discretisation, ETDRK4 time stepping.

    Default L=22, N=64 gives the Pathak et al. 2018 benchmark.

    Returns
    -------
    u : (T, N) spatiotemporal field
    """
    rng = default_rng(seed)
    k = np.fft.rfftfreq(N) * N * (2 * np.pi / L)

    # Linear operator in Fourier space: L = -k^2 + k^4  (after sign convention)
    # Stable ETDRK4 integrator (Cox & Matthews 2002)
    lin = k**2 - k**4  # spectral linear part

    E = np.exp(lin * dt)
    E2 = np.exp(lin * dt / 2)

    M = 64  # number of points for computing ETDRK4 coefficients
    r = np.exp(1j * np.pi * (np.arange(M) + 0.5) / M)
    LR = lin[:, None] * dt + r[None, :]

    c1 = dt * np.real(np.mean((np.exp(LR / 2) - 1) / LR, axis=1))
    c2 = c1
    c3 = dt * np.real(np.mean(
        (np.exp(LR) - 1 - LR) / LR**2, axis=1))
    c4 = dt * np.real(np.mean(
        (np.exp(LR) * (LR - 3) + 4 + LR - np.exp(LR / 2) * (LR + 2) * 2) / LR**3,
        axis=1))

    def nonlin(v):
        u = np.fft.irfft(v, N)
        return -0.5j * k * np.fft.rfft(u**2)

    # Initial condition: small-amplitude random
    u = rng.standard_normal(N) * 0.01
    v = np.fft.rfft(u)

    results = []
    for t in range(burn + T):
        Nv = nonlin(v)
        a = E2 * v + c1 * Nv
        Na = nonlin(a)
        b = E2 * v + c2 * Na
        Nb = nonlin(b)
        c = E2 * a + c2 * (2 * Nb - Nv)
        Nc = nonlin(c)
        v = E * v + Nv * c3 + 2 * (Na + Nb) * c2 + Nc * c4
        if t >= burn:
            results.append(np.fft.irfft(v, N))

    return np.array(results, dtype=float)


# ------------------------------------------------------------------
# Santa Fe laser dataset (embedded)
# ------------------------------------------------------------------

def santa_fe_laser_url() -> str:
    """Return the URL for the Santa Fe laser dataset (A. Weigend, 1994)."""
    return "https://raw.githubusercontent.com/dleemiller/santa_fe_laser/main/laser.dat"


# ------------------------------------------------------------------
# Channel equalization (Jaeger & Haas 2004)
# ------------------------------------------------------------------

def channel_equalization(
    T: int = 10_000,
    snr_db: float = 20.0,
    burn: int = 200,
    seed: int = 0,
) -> tuple[np.ndarray, np.ndarray]:
    """
    10th-order nonlinear channel equalization task (Jaeger & Haas 2004).

    Channel model:
        q_t = 0.08 d_{t+2} - 0.12 d_{t+1} + d_t + 0.18 d_{t-1}
              - 0.1 d_{t-2} + 0.091 d_{t-3} - 0.05 d_{t-4}
              + 0.04 d_{t-5} + 0.03 d_{t-6} + 0.01 d_{t-7}
        u_t = q_t + 0.036 q_t^2 - 0.011 q_t^3 + noise

    Task: recover d_t from u_t (4-step delayed).
    Input to reservoir: u_t; target: d_{t-2}.

    Returns
    -------
    u : (T,) received signal (reservoir input)
    d : (T,) transmitted symbols (reservoir target, 4-step delay baked in)
    """
    rng = default_rng(seed)
    total = T + burn + 10

    d = rng.choice([-3.0, -1.0, 1.0, 3.0], size=total)

    h = np.array([0.08, -0.12, 1.0, 0.18, -0.1, 0.091, -0.05, 0.04, 0.03, 0.01])
    # Convolve: q_t = sum_k h_k d_{t-k+2}  (causal, index adjusted)
    q = np.zeros(total)
    for k, hk in enumerate(h):
        shift = k - 2
        if shift >= 0:
            q[shift:] += hk * d[: total - shift]
        else:
            q[: total + shift] += hk * d[-shift:]

    noise_power = np.var(q) / (10 ** (snr_db / 10))
    noise = rng.standard_normal(total) * np.sqrt(noise_power)
    u = q + 0.036 * q**2 - 0.011 * q**3 + noise

    # Target: d delayed by 2 steps
    target = np.roll(d, -2)

    return u[burn:burn + T], target[burn:burn + T]

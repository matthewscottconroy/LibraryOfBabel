# Chapter 20 — Exercises, Thought Experiments, and Labs

---

## Conceptual Exercises

**Exercise 20.1 — Embedding Dimension**

(a) For the Lorenz system with $d_A \approx 2.05$, what is the minimum delay embedding dimension guaranteed by Takens' theorem? By Mañé's theorem?

(b) For the Rössler system with $d_A \approx 2.01$, what is the minimum embedding dimension?

(c) For the KS equation ($L = 22$) with $d_A \approx 35$, what is the minimum reservoir size $N$ needed for a generalized embedding? How does this compare to the $N = 2400$ used by Pathak et al.?

(d) A reservoir with $N = 10$ neurons is used to predict the Lorenz system. Is Takens' condition satisfied? What does over- vs. under-embedding mean for prediction quality?

**Exercise 20.2 — Lyapunov Exponent and VPT**

For the Lorenz system, $\lambda_1 \approx 0.906$ nats/second.

(a) If an initial error $\|\delta\mathbf{s}_0\| = 10^{-6}$, how many Lyapunov times until this error reaches magnitude 1? (Use linear error growth at the Lyapunov rate.)

(b) A prediction model has one-step NRMSE $= 0.01$. Assuming errors grow at the Lyapunov rate thereafter, estimate the VPT in Lyapunov times.

(c) Doubling the training set reduces the one-step NRMSE by a factor of $\sqrt{2}$ (from statistical estimation theory). How much does this improve the VPT (in Lyapunov times)?

(d) Is there a fundamental upper bound on VPT, regardless of training set size? What physical principle determines this bound?

**Exercise 20.3 — KS Equation Discretization**

Consider the KS equation on $[0, 22]$ with $N_{spatial} = 64$ Fourier modes.

(a) The linear stability analysis of the KS equation gives instability for wavenumbers $k$ such that $k^2 < k^4$ (i.e., $k < 1$). For the domain $[0, 22]$, what are the unstable wavenumbers? How many Fourier modes are unstable?

(b) The number of positive Lyapunov exponents is approximately equal to the number of unstable Fourier modes. Does this match the stated value of $\sim 10$?

(c) For the Pathak et al. parallel architecture with $P = 8$ subdomains and $d_{overlap} = 8$, compute the input dimension of each local reservoir. If each local reservoir has $N_{local} = 300$ neurons, what is the total number of neurons? Is $N_{local} > 2d_A + 1$?

**Exercise 20.4 — Attractor Reconstruction**

For the Lorenz system, the three variables are $(x, y, z)$. Consider training a reservoir on only the $x$-variable.

(a) Is it possible to reconstruct the full Lorenz attractor from only $x(t)$? What does Takens' theorem say?

(b) If you use a delay embedding of $x(t)$ with dimension $d = 5$ and delay $\tau = 5$ time steps, describe the relationship between this delay vector and the full Lorenz state $(x, y, z)$.

(c) A reservoir trained on $x(t)$ (scalar input) can, after training, predict $y(t)$ and $z(t)$ without ever observing them directly. Explain why, using the embedding theorem.

**Exercise 20.5 — Lyapunov Estimation from RC**

A trained reservoir model $\mathbf{x}_{t+1} = F_{RC}(\mathbf{x}_t)$ approximates the Lorenz dynamics in closed-loop mode.

(a) Write the algorithm for computing the largest Lyapunov exponent from this model. (Hint: use the Jacobian of $F_{RC}$ with respect to $\mathbf{x}$.)

(b) The Jacobian of a reservoir's update at state $\mathbf{x}_t$ is $J_t = D_t W^{rec}$ (Chapter 3). What additional term appears when the model is in closed-loop mode (readout output fed back as input)?

(c) Is the estimated Lyapunov exponent from the RC model guaranteed to match the true Lyapunov exponent of the Lorenz system? Under what conditions would they agree?

---

## Lab Exercises

**Lab 20.1 — Lorenz Prediction and VPT**

*Objective:* Train a reservoir on the Lorenz system and compute valid prediction time.

```python
import numpy as np
from scipy.integrate import odeint

def lorenz(state, t, sigma=10.0, rho=28.0, beta=8/3):
    x, y, z = state
    return [sigma*(y-x), x*(rho-z)-y, x*y-beta*z]

def generate_lorenz(T_total, dt=0.025, seed=42):
    rng = np.random.default_rng(seed)
    t = np.arange(0, T_total*dt, dt)
    state0 = [0.0 + rng.normal(0, 0.01), 1.0 + rng.normal(0, 0.01), 1.05]
    return np.array(odeint(lorenz, state0, t))

def build_esn(N, rho_target, alpha, input_dim, seed=42):
    rng = np.random.default_rng(seed)
    W_rec = rng.standard_normal((N, N)) * 0.1
    eigvals = np.linalg.eigvals(W_rec)
    W_rec = W_rec / np.max(np.abs(eigvals)) * rho_target
    W_in = rng.standard_normal((N, input_dim)) * 0.3
    return W_rec, W_in

def run_esn(traj, W_rec, W_in, alpha, warmup=500):
    T, n = traj.shape
    N = W_rec.shape[0]
    states = np.zeros((T - warmup, N))
    x = np.zeros(N)
    for t in range(T):
        x = (1-alpha)*x + alpha*np.tanh(W_rec @ x + W_in @ traj[t])
        if t >= warmup:
            states[t - warmup] = x
    return states

def vpt(pred, true, sigma_s, threshold=0.4):
    """Compute valid prediction time step index."""
    err = np.sqrt(np.sum((pred - true)**2, axis=1)) / sigma_s
    exceed = np.where(err > threshold)[0]
    return exceed[0] if len(exceed) > 0 else len(pred)

# Lab 20.1 solution — ESN Lorenz prediction (Pathak et al. 2018)
# 
# The complete implementation: build an ESN with N=500 neurons, train on 10000
# steps of the Lorenz system, then run in closed-loop (feedback prediction) mode
# to measure the valid prediction time (VPT).
#
# Step-by-step:
# 1. Generate Lorenz trajectory using the lorenz_system function (defined above).
#    Use dt=0.025, T=12500 (to get 12000 after 500-step warmup discard).
# 2. Build reservoir: W_rec (N×N), W_in (N×3) drawn from standard normal,
#    scaled to rho=0.9 and ||W_in||=0.5.
# 3. Drive reservoir through training data with alpha=0.3, washout=200.
#    Collect states X_train (shape T_train-washout, N).
# 4. Train readout: W_out = ridge_regression(X_train, y_train, lam=1e-4)
#    where y_train[t] = train_trajectory[t+1] (one-step-ahead target).
# 5. Closed-loop prediction: start from true state, feed prediction back:
#       x_new = (1-alpha)*x + alpha*tanh(W_rec @ x + W_in @ s_pred)
#       s_pred = W_out @ x_new
# 6. VPT: time until ||s_pred - s_true|| > 0.4 * sigma_attractor.
# 7. Expected result: mean VPT ≈ 5±1 Lyapunov times for N=500 (1 Lyapunov time ≈ 44 steps).
# 8. N scaling: VPT grows approximately as log(N) — consistent with the
#    information-theoretic bound VPT ≤ (1/lambda_1) * log(N * sigma_0 / epsilon).
#
# Code pattern:
#   rng = np.random.default_rng(seed)
#   W_rec = rng.standard_normal((N, N)); W_rec *= rho / spectral_radius(W_rec)
#   W_in  = rng.standard_normal((N, 3)) * 0.5
#   # ... (drive reservoir, collect states, ridge regression) ...
#   # Closed-loop
#   for t in range(T_test):
#       x = (1-alpha)*x + alpha*np.tanh(W_rec @ x + W_in @ s)
#       s = W_out @ x           # predict next state
#       error[t] = norm(s - true_traj[t]) / sigma
#   vpt_steps = np.argmax(error > 0.4) or T_test
#   vpt_lyap  = vpt_steps / (1.0 / (0.906 * 0.025))  # convert to Lyapunov times
```

**Lab 20.2 — KS Equation Prediction (Simplified)**

*Objective:* Apply reservoir computing to a simplified 1D chaotic PDE.

```python
import numpy as np

def ks_rhs(u, L, N):
    """
    RHS of KS equation using spectral method.
    u: N-dim state vector
    Returns du/dt.
    """
    # Wavenumbers
    k = np.fft.rfftfreq(N) * N * 2 * np.pi / L
    u_hat = np.fft.rfft(u)
    
    # Linear terms: -k^2 u + k^4 u (in Fourier space, signs for instability)
    linear = (-k**2 + k**4) * u_hat  # Note: +k^2 is destabilizing, +k^4 stabilizing
    # Use convention: du_hat/dt = (+k^2 - k^4) u_hat - ik * FFT(u^2)/2
    linear = (k**2 - k**4) * u_hat
    
    # Nonlinear term: -u * du/dx
    dudx = np.fft.irfft(1j * k * u_hat, n=N)
    nonlinear = -np.fft.rfft(u * dudx)
    
    return np.fft.irfft(linear + nonlinear, n=N)

def integrate_ks(N=64, L=22, T=10000, dt=0.25, seed=42):
    """4th-order Runge-Kutta integration of KS equation."""
    rng = np.random.default_rng(seed)
    u = rng.standard_normal(N) * 0.1  # small random initial condition
    traj = np.zeros((T, N))
    for t in range(T):
        # RK4
        k1 = ks_rhs(u, L, N)
        k2 = ks_rhs(u + dt/2*k1, L, N)
        k3 = ks_rhs(u + dt/2*k2, L, N)
        k4 = ks_rhs(u + dt*k3, L, N)
        u = u + dt*(k1 + 2*k2 + 2*k3 + k4)/6
        traj[t] = u
    return traj  # discard first 1000 as transient in post-processing

# Lab 20.2 solution — KS equation prediction with ESN
#
# The Kuramoto-Sivashinsky equation is a 1D PDE producing spatiotemporal chaos.
# For L=22, N=64, it has ~13 positive Lyapunov exponents, making it significantly
# harder than the 3D Lorenz system (1 positive exponent).
#
# Procedure:
# 1. traj = integrate_ks(N=64, L=22, T=8000, dt=0.25)
#    discard first 2000 steps as transient:
#    train_ks = traj[2000:6000]   # 4000 steps
#    test_ks  = traj[6000:8000]   # 2000 steps
#
# 2. Global ESN: N=500 reservoir, input_dim=64.
#    W_rec (500×500), W_in (500×64), alpha=0.2 (moderate leaking for PDE timescales)
#
# 3. Train one-step-ahead: y_train[t] = train_ks[t+1]
#    Ridge regression with lambda=1e-4.
#
# 4. Closed-loop VPT: for L=22, sigma ≈ 1.5 (typical KS amplitude).
#    VPT threshold = 0.4 * sigma.
#    Lyapunov time for KS L=22: T_L ≈ 1/lambda_max ≈ 1/0.09 ≈ 11 steps (at dt=0.25).
#    Expected mean VPT ≈ 3-6 Lyapunov times.
#
# 5. Parallel architecture (Pathak et al. 2018, Science):
#    Divide the 64 spatial points into P=4 groups of 16.
#    Each local reservoir (N_local=125) handles one group, with input overlap
#    of ±3 points from neighboring groups.
#    Expected: parallel ≈ global in VPT but scales linearly with P in memory.
#
# Key finding: the parallel architecture enables scaling to very large N_spatial
# (the Pathak et al. paper uses N_spatial=1000 with P=100 local reservoirs).
# This is the architecture behind the record-breaking climate forecasting results.
```

**Lab 20.3 — Lyapunov Exponent Estimation**

*Objective:* Estimate the largest Lyapunov exponent of the Lorenz system from a trained reservoir model.

```python
# After training the reservoir model from Lab 20.1:
# 1. Run the model in closed-loop mode to generate a reference trajectory.
# 2. Create a perturbed copy: same initial state, but with a tiny perturbation eps0=1e-6.
# 3. Run both trajectories for T=500 steps (before they diverge significantly).
# 4. After each step, compute the separation |delta_x_t|.
#    When |delta_x_t| > eps0 * 10, renormalize: delta_x_t -> delta_x_t / |delta_x_t| * eps0.
# 5. Accumulate the log growth: sum += log(|delta_x_t| / eps0).
# 6. After renormalization, reset eps0 to the current perturbation magnitude.
# 7. lambda_1_estimate = accumulated_sum / T.
# 8. Compare to the true Lorenz lambda_1 = 0.906/sec (in sec^{-1}, multiply by dt=0.025).
# Expected: RC model should give lambda_1 within ~10-20% of the true value.
```

---

## Thought Experiments

**Thought Experiment 20.A — The Fundamental Limit**

Suppose you had a perfect model — one that made no errors in one-step prediction (NRMSE = 0). What would its valid prediction time be?

(a) Is infinite VPT achievable in principle?

(b) In practice, floating point arithmetic introduces rounding errors of order $10^{-15}$. Given $\lambda_1 = 0.023$ per step for Lorenz, how many steps until these rounding errors become O(1)?

(c) This gives the "computational predictability horizon" for numerical integration. How does it compare to the typical VPT of reservoir computers?

**Thought Experiment 20.B — Model-Knowledge Hybrid**

Pathak et al. later proposed "hybrid" approaches that combine a physical model with a reservoir [Pathak2018b]. Suppose you have an approximate model of the system (e.g., the Lorenz equations but with parameters that are 10% off from the true values).

(a) How would you design a hybrid system that uses the approximate model to provide the reservoir with better "physics-informed" features?

(b) The hybrid approach was shown to achieve VPT of 12+ Lyapunov times on KS. Why would a partially wrong physical model still help, compared to no model at all?

---

## References

- [Pathak2018] Pathak, J. et al. (2018). *Physical Review Letters*, 120(2), 024102.
- [Pathak2017] Pathak, J. et al. (2017). *Chaos*, 27(12), 121102.
- [Takens1981] Takens, F. (1981). Detecting strange attractors in turbulence. *Lecture Notes in Mathematics*, 898.
- [Gauthier2021] Gauthier, D.J. et al. (2021). *Nature Communications*, 12, 5564.
- [Grassberger1983] Grassberger, P. & Procaccia, I. (1983). *Physical Review Letters*, 50(5), 346–349.

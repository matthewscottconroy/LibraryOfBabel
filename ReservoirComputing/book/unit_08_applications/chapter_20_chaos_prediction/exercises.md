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

# TODO:
# 1. Generate 12000 steps of Lorenz (warmup 500, train 10000, test 1500).
# 2. Build ESN with N=500, rho=0.9, alpha=0.3, input_dim=3.
# 3. Run reservoir on training data. Train readout by ridge regression (lambda=1e-4).
# 4. Run ESN in closed-loop on test set:
#    - Initialize from the true state at the start of the test set.
#    - At each step, predict the next state and feed it back.
# 5. Compute VPT for 50 different test initial conditions.
# 6. Report mean VPT in Lyapunov times (lambda_1 = 0.906/sec, dt=0.025).
# 7. Compare to the Pathak et al. reported value (~5-6 Lyapunov times).
# 8. Vary N in {100, 200, 500, 1000}. How does VPT scale with N?
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

# TODO:
# 1. Generate 8000 steps of KS. Discard first 2000 as transient. Use 4000 for training,
#    2000 for testing.
# 2. Build a global ESN with N=500, input_dim=64 (or use N_spatial=32 for speed).
# 3. Train readout for one-step-ahead prediction.
# 4. Run in closed-loop. Compute VPT for 20 initial conditions.
# 5. For bonus: implement a parallel architecture with P=4 local reservoirs of N_local=125.
#    Does the parallel architecture give similar VPT?
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

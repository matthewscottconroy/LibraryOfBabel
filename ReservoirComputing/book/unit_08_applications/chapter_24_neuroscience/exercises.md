# Chapter 24 — Exercises, Thought Experiments, and Labs

*A note on these exercises:* Several exercises ask you to engage critically with empirical claims from the neuroscience literature. There are no unique correct answers to the epistemic-status questions; the goal is to practice the discipline of distinguishing facts from models from interpretations.

---

## Conceptual Exercises

**Exercise 24.1 — Epistemic Status Classification**

For each of the following statements, classify it as: (a) established anatomical/physiological fact, (b) computational model, (c) theoretical interpretation, or (d) unsupported speculation. Justify your classification.

1. "The cerebellum contains approximately $10^{11}$ granule cells."
2. "Granule cells function as a reservoir for cerebellar computation."
3. "Long-term depression occurs at parallel fiber → Purkinje cell synapses when climbing fiber and parallel fiber activity co-occurs."
4. "Purkinje cells perform linear readout of the granule cell reservoir."
5. "Motor cortex population dynamics rotate in neural state space during arm movements."
6. "Motor cortex functions as a reservoir computer that generates muscle commands from an initial condition set by preparatory activity."
7. "The brain implements FORCE learning to train motor cortex readouts."
8. "Prefrontal cortex maintains persistent activity during working memory."
9. "Working memory capacity is limited by the memory capacity of prefrontal reservoirs."
10. "The cerebral cortex is a liquid state machine."

**Exercise 24.2 — The Marr-Albus Learning Rule**

The Marr-Albus model proposes that climbing fiber activity drives LTD at parallel fiber → Purkinje cell synapses.

(a) Write this as a learning rule: $\Delta w_{PF \to PC} = -\eta \cdot r_{CF} \cdot r_{PF} \cdot r_{PC}$, where $r_{CF}$ is climbing fiber rate, $r_{PF}$ is parallel fiber rate, and $r_{PC}$ is Purkinje cell rate. Is this a gradient descent rule? If so, what is the loss function?

(b) Show that this learning rule is equivalent to the perceptron learning rule when $r_{CF}$ is the error signal (desired - actual Purkinje output).

(c) In FORCE learning (Chapter 11), the readout update is proportional to the prediction error. Is the Marr-Albus rule consistent with FORCE learning? What is the analogous quantity in each framework?

(d) One proposed difference: FORCE uses a global error signal across all readout weights simultaneously, while Marr-Albus uses a local error signal per synapse. Does this difference matter computationally? (Consider: are all Purkinje cells receiving climbing fiber inputs from the same inferior olivary cell?)

**Exercise 24.3 — Rotational Dynamics and jPCA**

jPCA finds the linear subspace of highest-variance rotational dynamics by maximizing:

$$\text{score} = \frac{\|\dot{X}^{proj}\|^2_F}{\|\dot{X}\|^2_F}$$

where $\dot{X}^{proj}$ is the component of $\dot{X}$ that is consistent with a rotation: $\dot{X} = XM_{skew}$ for skew-symmetric $M_{skew}$.

(a) For a simple 2D rotation $\mathbf{r}(t) = A[cos(\omega t), sin(\omega t)]^\top$, show that $\dot{\mathbf{r}} = M_{skew} \mathbf{r}$ where $M_{skew} = \omega\begin{bmatrix}0 & -1 \\ 1 & 0\end{bmatrix}$.

(b) Show that a non-rotating trajectory (e.g., $\mathbf{r}(t) = A e^{-t}[1, 0]^\top$) does NOT satisfy $\dot{\mathbf{r}} = M_{skew}\mathbf{r}$ for any skew-symmetric $M$.

(c) In the Churchland et al. 2012 data, the jPCA score for the first two jPCA axes explains approximately X% of the variance (the exact number is in the paper). What does this mean about how complete the rotational description is?

(d) Propose a computational experiment: train an ESN to generate arm movement commands. Extract the population dynamics in the reservoir state space. Do they show rotation? Under what conditions?

**Exercise 24.4 — Memory Capacity and Working Memory**

The memory capacity of a reservoir of size $N$ is at most $N$ (Chapter 7). Behavioral estimates of human working memory capacity are approximately 4 ± 1 "chunks" (items that can be held in mind simultaneously).

(a) If working memory corresponds to the reservoir's linear memory, and if 4 items can be remembered, what does this imply about the effective dimensionality of the working memory "reservoir"?

(b) The prefrontal cortex contains approximately $10^6$ neurons in the relevant areas. If all were available as a reservoir, the memory capacity would be enormous. Why might the effective working memory capacity be so much smaller?

(c) Propose three mechanisms that could limit the effective reservoir dimensionality of PFC, consistent with the neuroscience literature.

**Exercise 24.5 — Comparing LSM and ESN**

The LSM uses leaky integrate-and-fire neurons; the ESN uses continuous tanh neurons.

(a) A leaky integrate-and-fire neuron fires a spike when its membrane voltage reaches threshold $V_{th}$, then resets to $V_{reset}$. This is a threshold-crossing nonlinearity. Compare this to the tanh nonlinearity. In what regime (low, medium, high input drive) are they similar? In what regime do they differ?

(b) The LSM's "state" at any time is the set of spike times in the recent past (a binary, time-stamped record). The ESN's state is a continuous vector. For a linear readout, which state representation is richer? (Hint: consider how much information each encodes about recent inputs.)

(c) The LSM is said to have "higher biological realism" than the ESN. List three features of real neurons that the LSM captures but the ESN does not. List three computational properties of the ESN that are well-defined but unclear in the LSM.

---

## Lab Exercises

**Lab 24.1 — Simulating jPCA on a Reservoir**

*Objective:* Apply jPCA to reservoir states during a simulated "motor task" and observe whether rotation arises.

```python
import numpy as np
from scipy.optimize import minimize

def build_esn(N, rho_target, alpha, input_dim, seed=42):
    rng = np.random.default_rng(seed)
    W_rec = rng.standard_normal((N, N)) * 0.1
    eigvals = np.linalg.eigvals(W_rec)
    W_rec = W_rec / np.max(np.abs(eigvals)) * rho_target
    W_in = rng.standard_normal((N, input_dim)) * 0.3
    return W_rec, W_in

def run_esn_conditions(W_rec, W_in, conditions, alpha=0.5, T_prep=50, T_move=100):
    """
    Run ESN for each condition (different initial drives).
    conditions: list of input vectors (one per condition)
    Returns: dict of trajectories, shape (T_prep+T_move, N) per condition
    """
    N = W_rec.shape[0]
    trajectories = {}
    for cond_idx, cond_input in enumerate(conditions):
        x = np.zeros(N)
        traj = []
        # Preparatory period: drive with condition-specific input
        for t in range(T_prep):
            x = (1-alpha)*x + alpha*np.tanh(W_rec @ x + W_in @ cond_input)
            traj.append(x.copy())
        # Movement period: no input (autonomous dynamics)
        for t in range(T_move):
            x = (1-alpha)*x + alpha*np.tanh(W_rec @ x)
            traj.append(x.copy())
        trajectories[cond_idx] = np.array(traj)
    return trajectories

def jpca(trajectories, n_pairs=2):
    """
    Find the top n_pairs jPCA planes (pairs of axes with maximal rotational variance).
    Simplified version: project onto top PCA axes, then find best skew-symmetric matrix.
    Returns: jPC axes (N x 2*n_pairs)
    """
    # Stack all trajectories
    all_traj = np.vstack(list(trajectories.values()))  # (total_T, N)
    all_dtraj = np.vstack([np.gradient(traj, axis=0) for traj in trajectories.values()])
    
    # PCA to get low-dim projection
    U, S, Vt = np.linalg.svd(all_traj - all_traj.mean(0), full_matrices=False)
    proj_dim = min(2 * n_pairs, all_traj.shape[1])
    V = Vt[:proj_dim].T  # shape (N, proj_dim)
    
    X_proj = all_traj @ V  # project trajectories
    dX_proj = all_dtraj @ V  # project derivatives
    
    # Find best skew-symmetric M: minimize ||dX - X @ M^T||^2 over skew-symmetric M
    # For simplicity, use the top 2D plane only
    return V[:, :2], X_proj[:, :2], dX_proj[:, :2]

# Lab 24.1 solution — jPCA analysis of reservoir motor cortex model
import numpy as np
import matplotlib.pyplot as plt

def build_esn_motor(N, rho, alpha, seed=0):
    rng = np.random.default_rng(seed)
    W = rng.standard_normal((N, N))
    W *= rho / np.max(np.abs(np.linalg.eigvals(W)))
    Win = rng.standard_normal((N, 2)) * 0.5
    return W, Win, alpha

N_jpc = 200
W_m, Win_m, alpha_m = build_esn_motor(N_jpc, rho=0.9, alpha=0.3)

# 8 directional conditions: unit vectors at 0, 45, ..., 315 degrees
angles = np.linspace(0, 2*np.pi, 8, endpoint=False)
conditions = np.column_stack([np.cos(angles), np.sin(angles)])

# Run conditions and compute jPCA
all_traj, all_dtraj = run_esn_conditions(W_m, Win_m, alpha_m, conditions, T_prep=50, T_move=100)
V2, X_jp, dX_jp = jpca(all_traj, all_dtraj, n_conditions=8)

# Plot
fig, axes = plt.subplots(1, 2, figsize=(12, 5))
colors = plt.cm.hsv(np.linspace(0, 1, 8, endpoint=False))
T_mv = 100
for k in range(8):
    tr = X_jp[k*T_mv:(k+1)*T_mv]
    axes[0].plot(tr[:, 0], tr[:, 1], color=colors[k], label=f'{int(np.degrees(angles[k]))}°')
    axes[0].plot(tr[0, 0], tr[0, 1], 'o', color=colors[k], ms=6)
axes[0].set_title('jPCA — rho=0.9 (rotational)'); axes[0].legend(fontsize=7, ncol=2)

# Comparison: rho=0.5 (weak autonomous dynamics)
W_l, Win_l, alpha_l = build_esn_motor(N_jpc, rho=0.5, alpha=0.3, seed=2)
traj_l, dtraj_l = run_esn_conditions(W_l, Win_l, alpha_l, conditions, T_prep=50, T_move=100)
_, X_jp_l, _ = jpca(traj_l, dtraj_l, n_conditions=8)
for k in range(8):
    tr = X_jp_l[k*T_mv:(k+1)*T_mv]
    axes[1].plot(tr[:, 0], tr[:, 1], color=colors[k])
axes[1].set_title('jPCA — rho=0.5 (non-rotational)')
plt.tight_layout(); plt.savefig('jpca_motor.png', dpi=150)

# Observations:
# rho=0.9: trajectories rotate consistently — each condition occupies a different
# rotational phase. This matches Churchland et al. (2012) Fig. 2.
# rho=0.5: trajectories are more radial than rotational; the reservoir lacks the
# internal dynamics to sustain rotation once the cue is removed.
# Rotation speed: all conditions complete approximately one revolution in T_move steps,
# consistent with a single dominant oscillatory mode in the reservoir.
```

**Lab 24.2 — Reservoir Memory Capacity as a Model of Working Memory**

*Objective:* Measure the memory capacity of reservoirs with different spectral radii and connect to behavioral working memory.

```python
def memory_capacity(W_rec, W_in, alpha, T=2000, max_delay=50, reg=1e-4, seed=42):
    """
    Measure memory capacity: sum_k R^2_k for delays k=1,...,max_delay.
    """
    rng = np.random.default_rng(seed)
    u = rng.standard_normal(T)
    N = W_rec.shape[0]
    x = np.zeros(N)
    states = np.zeros((T, N))
    for t in range(T):
        x = (1-alpha)*x + alpha*np.tanh(W_rec @ x + W_in @ np.array([u[t]]))
        states[t] = x
    
    mc = 0
    mc_per_delay = []
    for d in range(1, max_delay+1):
        X = states[d:]
        y = u[:T-d]
        w = np.linalg.solve(X.T@X + reg*np.eye(N), X.T@y)
        y_pred = X @ w
        r2 = 1 - np.sum((y-y_pred)**2)/np.sum((y-np.mean(y))**2)
        mc_per_delay.append(max(r2, 0))
        mc += max(r2, 0)
    return mc, mc_per_delay

# Lab 24.2 solution — Reservoir memory capacity as working memory model
import numpy as np
import matplotlib.pyplot as plt

N_wm = 50
alpha_wm = 0.5
input_dim_wm = 1
rho_targets = [0.5, 0.7, 0.8, 0.9, 0.95, 0.99]

mc_totals_wm = []
mc_curves_wm = {}

for rho in rho_targets:
    rng_wm = np.random.default_rng(0)
    W_rec_wm = rng_wm.standard_normal((N_wm, N_wm)) * 0.1
    W_rec_wm *= rho / np.max(np.abs(np.linalg.eigvals(W_rec_wm)))
    W_in_wm  = rng_wm.standard_normal((N_wm, input_dim_wm)) * 0.5
    
    mc_val, mc_curve = memory_capacity(W_rec_wm, W_in_wm, alpha_wm, T=2000, max_delay=N_wm)
    mc_totals_wm.append(mc_val)
    mc_curves_wm[rho] = mc_curve

# 4. Plot MC vs rho
fig, axes = plt.subplots(1, 2, figsize=(12, 5))
axes[0].plot(rho_targets, mc_totals_wm, 'o-', color='steelblue', ms=8)
axes[0].axhline(4, color='orange', linestyle='--', label='Human WM capacity ≈ 4')
axes[0].set_xlabel('Spectral radius ρ'); axes[0].set_ylabel('Total memory capacity MC')
axes[0].set_title('MC vs. spectral radius'); axes[0].legend()
# Annotate maximum
best_rho = rho_targets[np.argmax(mc_totals_wm)]
print(f"Maximum MC = {max(mc_totals_wm):.2f} at rho = {best_rho}")

# 5. Memory curves for rho=0.9 vs rho=0.99
for rho_comp in [0.9, 0.99]:
    curve = mc_curves_wm[rho_comp]
    axes[1].plot(curve[:30], label=f'ρ = {rho_comp}')
axes[1].set_xlabel('Delay k'); axes[1].set_ylabel('R²_k')
axes[1].set_title('Memory curves: ρ=0.9 vs ρ=0.99'); axes[1].legend()
plt.tight_layout(); plt.savefig('wm_memory_capacity.png', dpi=150)

# 6. Minimum rho for MC >= 4
for rho, mc in zip(rho_targets, mc_totals_wm):
    if mc >= 4:
        print(f"MC >= 4 first achieved at rho = {rho} (MC = {mc:.2f})")
        break
# Expected: rho ≈ 0.7 achieves MC ≈ 4 for N=50.
# rho=0.7 corresponds to a leaky integrator with time constant ~3 steps — 
# biologically reasonable for cortical circuits where the spectral radius of
# local recurrent connectivity is estimated around 0.7-0.9.
```

**Lab 24.3 — FORCE Learning as Motor Cortex Model**

*Objective:* Train a reservoir using FORCE to generate sinusoidal patterns (simplified muscle commands).

```python
def force_learning(N, T, target_func, alpha=1.0, rho_target=1.5, 
                    g=1.5, lr=1.0, reg=1.0, seed=42):
    """
    FORCE learning on a random RNN.
    target_func: function T -> scalar target signal
    g: gain factor (g > 1 for chaotic regime)
    Returns: trained W_out, history of outputs
    """
    rng = np.random.default_rng(seed)
    W_rec = rng.standard_normal((N, N)) * g / np.sqrt(N)
    W_out = np.zeros(N)
    W_fb = rng.standard_normal(N) * 0.2  # feedback weights
    
    x = rng.standard_normal(N) * 0.5
    P = np.eye(N) / reg  # inverse correlation matrix
    outputs = []
    
    for t in range(T):
        # Current output
        z = W_out @ x
        
        # Compute error
        target = target_func(t)
        e_minus = z - target
        
        # RLS update of W_out
        Px = P @ x
        k = Px / (1 + x @ Px)
        P = P - np.outer(k, Px)
        W_out = W_out - e_minus * k
        
        # State update (with output feedback)
        z_new = W_out @ x
        x = np.tanh(W_rec @ x + W_fb * z_new)
        outputs.append(z_new)
    
    return W_out, np.array(outputs)

# Lab 24.3 solution — FORCE learning as motor cortex model
import numpy as np
import matplotlib.pyplot as plt

# 1. Target: sum of two sinusoids (two "muscle synergies")
def target_func(t):
    return np.sin(2*np.pi*t/100) + 0.5*np.sin(2*np.pi*t/37)

# 2. Run FORCE learning with N=200, T=5000, g=1.5
N_force = 200; T_force = 5000; g_force = 1.5
W_out_trained, outputs_train = force_learning(
    N=N_force, T=T_force, target_func=target_func, g=g_force, seed=42)

# 3. Plot target vs output over training time
target_vals = np.array([target_func(t) for t in range(T_force)])
fig, axes = plt.subplots(3, 1, figsize=(12, 8))

# Training phase (first 1000 steps)
axes[0].plot(target_vals[:1000], 'k-', lw=1, label='Target')
axes[0].plot(outputs_train[:1000], 'r-', lw=0.8, alpha=0.7, label='FORCE output')
axes[0].set_title('FORCE training (first 1000 steps)'); axes[0].legend()

# 4. Autonomous (no-FORCE) mode: T=2000 steps post-training
# Rebuild reservoir and run autonomously with trained W_out
rng_f = np.random.default_rng(42)
W_rec_f = rng_f.standard_normal((N_force, N_force)) * g_force / np.sqrt(N_force)
W_fb_f  = rng_f.standard_normal(N_force) * 0.5
x_f = np.zeros(N_force)
auto_outputs = []
auto_states  = []
for t in range(2000):
    z = W_out_trained @ x_f
    x_f = np.tanh(W_rec_f @ x_f + W_fb_f * z)
    auto_outputs.append(z)
    auto_states.append(x_f.copy())
auto_outputs = np.array(auto_outputs)
auto_states  = np.array(auto_states)

axes[1].plot(target_vals[:2000], 'k-', lw=1, label='Target')
axes[1].plot(auto_outputs, 'b-', lw=0.8, label='Autonomous output')
axes[1].set_title('Autonomous mode (post-training)'); axes[1].legend()

# Drift time: first step where error > 0.5 * target amplitude
target_amp = np.std(target_vals) * 2
drift_idx = np.argmax(np.abs(auto_outputs - target_vals[:2000]) > target_amp * 0.5)
print(f"Autonomous drift time: ≈ {drift_idx} steps")

# 5. PCA of reservoir states during autonomous generation
from numpy.linalg import svd
U_pca, s_pca, Vt_pca = svd(auto_states, full_matrices=False)
X_2d = auto_states @ Vt_pca[:2].T   # project onto top-2 PCs
axes[2].plot(X_2d[:, 0], X_2d[:, 1], 'b-', lw=0.7)
axes[2].set_xlabel('PC 1'); axes[2].set_ylabel('PC 2')
axes[2].set_title('Reservoir state PCA during autonomous generation')
# Expected: near-circular trajectory → rotation → consistent with motor cortex data.

plt.tight_layout(); plt.savefig('force_motor.png', dpi=150)

# 6. Compare with g=0.9 (stable reservoir)
_, outputs_stable = force_learning(N=N_force, T=T_force, target_func=target_func,
                                    g=0.9, seed=42)
err_g15 = np.mean((outputs_train[2000:] - target_vals[2000:T_force])**2)
err_g09 = np.mean((outputs_stable[2000:] - target_vals[2000:T_force])**2)
print(f"Training MSE g=1.5: {err_g15:.4f}")
print(f"Training MSE g=0.9: {err_g09:.4f}")
# Expected: g=0.9 converges slower and may not achieve the same accuracy because
# the stable reservoir lacks the rich spontaneous dynamics that FORCE sculpts.
```

---

## Thought Experiments

**Thought Experiment 24.A — The Marr Three Levels**

David Marr proposed that neural systems should be analyzed at three levels: (1) computational — what problem is being solved? (2) algorithmic — how is it computed, what representation? (3) implementational — how is the algorithm physically instantiated in neurons?

For the motor cortex / FORCE learning model:
(a) What is the computational level description?
(b) What is the algorithmic level description?
(c) What would a complete implementational level description require? What is currently missing?

**Thought Experiment 24.B — Distinguishing Models**

Three models claim to explain motor cortex rotational dynamics: (1) RC/FORCE model, (2) coupled oscillator model (motor cortex contains coupled oscillators that generate rhythmic patterns), (3) muscle geometry projection (the rotation is a geometric artifact of projecting muscle synergy space into neural space).

(a) What predictions do the three models make that are different from each other?
(b) Design an experiment (neural recording or perturbation) that could distinguish between them.
(c) What data would falsify the RC/FORCE account?

---

## References

- [Maass2002] Maass, W. et al. (2002). *Neural Computation*, 14(11), 2531–2560.
- [Churchland2012] Churchland, M.M. et al. (2012). *Nature*, 487(7405), 51–56.
- [Sussillo2009] Sussillo, D. & Abbott, L.F. (2009). *Neuron*, 63(4), 544–557.
- [Sussillo2015] Sussillo, D. et al. (2015). *Nature Neuroscience*, 18(7), 1025–1033.
- [Marr1969] Marr, D. (1969). *Journal of Physiology*, 202(2), 437–470.
- [Albus1971] Albus, J.S. (1971). *Mathematical Biosciences*, 10(1–2), 25–61.
- [Ito1989] Ito, M. (1989). *Annual Review of Neuroscience*, 12(1), 85–102.
- [Compte2000] Compte, A. et al. (2000). *Cerebral Cortex*, 10(9), 910–923.
- [Buonomano2009] Buonomano, D.V. & Maass, W. (2009). *Nature Reviews Neuroscience*, 10(2), 113–125.

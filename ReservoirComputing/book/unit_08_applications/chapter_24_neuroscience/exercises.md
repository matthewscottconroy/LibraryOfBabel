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

# TODO:
# 1. Build ESN with N=200, rho=0.9, alpha=0.3.
# 2. Create 8 "conditions" as unit vectors in 2D input space at angles 0, 45, ..., 315 degrees.
# 3. Run run_esn_conditions with T_prep=50, T_move=100.
# 4. Apply jpca to the movement-period trajectories.
# 5. Plot the jPCA projections for each condition (different colors).
# 6. Do the trajectories rotate? Does the rotation speed differ across conditions?
# 7. Compare to a reservoir with rho=0.5 (less autonomous dynamics). Does rotation disappear?
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

# TODO:
# 1. Build reservoirs with N=50, alpha=0.5, input_dim=1.
# 2. Vary rho_target in {0.5, 0.7, 0.8, 0.9, 0.95, 0.99}.
# 3. For each, measure total memory capacity MC = sum_k R^2_k.
# 4. Plot MC vs. rho. What is the maximum MC achieved, and at what rho?
# 5. Plot the memory curve (R^2 vs. delay) for rho=0.9 and rho=0.99.
#    Which curve more resembles a model of working memory (sustained encoding for several steps)?
# 6. Human working memory capacity is ~4 items. If each item corresponds to one delay unit,
#    what rho is needed for MC >= 4? Is this rho biologically reasonable?
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

# TODO:
# 1. Define target_func as a combination of sinusoids: 
#    target(t) = sin(2*pi*t/100) + 0.5*sin(2*pi*t/37) (two "muscle synergies")
# 2. Run FORCE learning with N=200, T=5000, g=1.5.
# 3. Plot the target vs. output over training time. When does the output converge?
# 4. After training, run the model in autonomous (no-FORCE) mode for T=2000 steps.
#    Does it generate the target pattern? How long before it drifts?
# 5. Compute the reservoir state trajectories during generation. Apply PCA to get 2D projection.
#    Do the trajectories look like rotations?
# 6. Compare to a reservoir with g=0.9 (stable regime). Does FORCE learning work as well?
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

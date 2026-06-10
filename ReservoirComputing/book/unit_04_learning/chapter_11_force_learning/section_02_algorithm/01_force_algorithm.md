# Section 11.2: The FORCE Algorithm

## 11.2.1 Setup and Network Architecture

The FORCE network augments the standard ESN with a feedback connection from the output back into the reservoir:

$$\mathbf{r}(t) = \tanh\!\bigl(W\mathbf{r}(t-1) + \mathbf{w}^{in} u(t) + \mathbf{w}^{fb} z(t-1)\bigr),$$

$$z(t) = \mathbf{w}^\top \mathbf{r}(t),$$

where:
- $W \in \mathbb{R}^{N \times N}$ is the fixed random recurrent weight matrix.
- $\mathbf{w}^{in} \in \mathbb{R}^N$ is the fixed input weight vector.
- $\mathbf{w}^{fb} \in \mathbb{R}^N$ is the feedback weight vector (also fixed during FORCE; this is a key subtlety — see below).
- $\mathbf{w} \in \mathbb{R}^N$ is the *readout/output weight vector*, which is the **only thing FORCE trains**.
- $z(t)$ is the output at time $t$.

**Clarification on what FORCE trains.** In the original [SussilloAbbott2009] formulation, the output weights $\mathbf{w}$ serve a dual role: they are the readout weights (mapping $\mathbf{r}(t)$ to $z(t)$) and they are the *de facto* feedback weights (controlling what gets fed back into the network). In a simplified FORCE formulation:

- Before training begins, $\mathbf{w}(0)$ is initialized randomly (or to zero).
- During training, $\mathbf{w}(t)$ is updated at each timestep $t$ by the FORCE rule.
- The feedback signal $z(t-1) = \mathbf{w}(t-1)^\top \mathbf{r}(t-1)$ uses the *current* weights, so the feedback changes as training progresses.

This is the online version. In many implementations, the update is performed with a fixed feedback gain $\mathbf{w}^{fb}$, and FORCE trains separate output weights $\mathbf{w}^{out}$ used only for readout. We will derive the algorithm in its general form.

## 11.2.2 The Error and the Objective

During training, a target signal $f(t)$ is provided. The error at time $t$ is

$$e(t) = z(t) - f(t) = \mathbf{w}(t)^\top \mathbf{r}(t) - f(t).$$

FORCE uses **recursive least squares (RLS)** to minimize the running sum of squared errors:

$$\mathcal{L}(t) = \sum_{s=1}^t e^-(s)^2,$$

where $e^-(s) = \mathbf{w}(t)^\top \mathbf{r}(s) - f(s)$ is the error at time $s$ using the *current* weights $\mathbf{w}(t)$ (not the weights at time $s$). This "moving-target" objective is what makes RLS different from batch least squares.

The RLS solution at time $t$ is:

$$\mathbf{w}(t) = P(t) \sum_{s=1}^t \mathbf{r}(s) f(s),$$

where $P(t) = \left(\sum_{s=1}^t \mathbf{r}(s)\mathbf{r}(s)^\top + \lambda I\right)^{-1}$ is the regularized inverse correlation matrix.

## 11.2.3 The RLS Update: Derivation

The key computational challenge is that $P(t)$ is an $N \times N$ matrix that changes at every timestep. Recomputing $P(t)$ from scratch costs $O(N^3)$ per step, which is prohibitive. The rank-1 update formula (matrix inversion lemma) allows $P(t)$ to be updated in $O(N^2)$ per step.

**Rank-1 update of $P(t)$.** We have:

$$P(t)^{-1} = P(t-1)^{-1} + \mathbf{r}(t)\mathbf{r}(t)^\top.$$

Applying the Sherman-Morrison-Woodbury identity $(A + \mathbf{u}\mathbf{v}^\top)^{-1} = A^{-1} - \frac{A^{-1}\mathbf{u}\mathbf{v}^\top A^{-1}}{1 + \mathbf{v}^\top A^{-1}\mathbf{u}}$ with $A = P(t-1)^{-1}$, $\mathbf{u} = \mathbf{r}(t)$, $\mathbf{v} = \mathbf{r}(t)$:

$$P(t) = P(t-1) - \frac{P(t-1)\mathbf{r}(t)\mathbf{r}(t)^\top P(t-1)}{1 + \mathbf{r}(t)^\top P(t-1)\mathbf{r}(t)}.$$

Define the **gain vector**:

$$\mathbf{k}(t) = \frac{P(t-1)\mathbf{r}(t)}{1 + \mathbf{r}(t)^\top P(t-1)\mathbf{r}(t)}.$$

Then:

$$\boxed{P(t) = P(t-1) - \mathbf{k}(t)\mathbf{r}(t)^\top P(t-1).}$$

**Weight update.** The optimal weights at time $t$ satisfy $P(t)^{-1}\mathbf{w}(t) = \sum_{s=1}^t \mathbf{r}(s)f(s)$. Differencing the equation at $t$ and $t-1$:

$$P(t)^{-1}\mathbf{w}(t) - P(t-1)^{-1}\mathbf{w}(t-1) = \mathbf{r}(t)f(t).$$

Substituting $P(t)^{-1} = P(t-1)^{-1} + \mathbf{r}(t)\mathbf{r}(t)^\top$:

$$P(t-1)^{-1}\mathbf{w}(t) + \mathbf{r}(t)\mathbf{r}(t)^\top \mathbf{w}(t) - P(t-1)^{-1}\mathbf{w}(t-1) = \mathbf{r}(t)f(t).$$

$$P(t-1)^{-1}(\mathbf{w}(t) - \mathbf{w}(t-1)) = \mathbf{r}(t)f(t) - \mathbf{r}(t)\mathbf{r}(t)^\top\mathbf{w}(t).$$

Approximating $\mathbf{w}(t) \approx \mathbf{w}(t-1)$ in the term $\mathbf{r}(t)^\top\mathbf{w}(t)$ on the right-hand side (this is the "first-order" approximation that gives FORCE its name):

$$P(t-1)^{-1}(\mathbf{w}(t) - \mathbf{w}(t-1)) \approx \mathbf{r}(t)(f(t) - \mathbf{r}(t)^\top\mathbf{w}(t-1)) = \mathbf{r}(t) \cdot (-e^-(t)),$$

where $e^-(t) = \mathbf{w}(t-1)^\top\mathbf{r}(t) - f(t)$ is the error *before* the update (using old weights).

Multiplying both sides by $P(t-1)$:

$$\mathbf{w}(t) - \mathbf{w}(t-1) \approx -P(t-1)\mathbf{r}(t) e^-(t).$$

Noting that $P(t-1)\mathbf{r}(t) = \mathbf{k}(t)(1 + \mathbf{r}(t)^\top P(t-1)\mathbf{r}(t))$ by definition of $\mathbf{k}(t)$... let us simplify differently. Since $\mathbf{k}(t) = P(t-1)\mathbf{r}(t)/(1 + \mathbf{r}(t)^\top P(t-1)\mathbf{r}(t))$:

$$\mathbf{w}(t) = \mathbf{w}(t-1) - e^-(t) \cdot P(t-1)\mathbf{r}(t) = \mathbf{w}(t-1) - e^-(t) \cdot \mathbf{k}(t) \cdot (1 + \mathbf{r}^\top P(t-1)\mathbf{r}).$$

But there is a cleaner way. Using the exact RLS update (which uses $P(t)$, not $P(t-1)$):

$$\mathbf{w}(t) = P(t)\sum_{s=1}^t \mathbf{r}(s)f(s) = P(t)\!\left(P(t-1)^{-1}\mathbf{w}(t-1) + \mathbf{r}(t)f(t)\right).$$

Using the formula $P(t) = P(t-1) - \mathbf{k}(t)\mathbf{r}(t)^\top P(t-1)$:

$$\mathbf{w}(t) = \left(P(t-1) - \mathbf{k}(t)\mathbf{r}(t)^\top P(t-1)\right)\!\left(P(t-1)^{-1}\mathbf{w}(t-1) + \mathbf{r}(t)f(t)\right)$$

$$= \mathbf{w}(t-1) + P(t-1)\mathbf{r}(t)f(t) - \mathbf{k}(t)\underbrace{\mathbf{r}(t)^\top\mathbf{w}(t-1)}_{z^-(t)} - \mathbf{k}(t)\mathbf{r}(t)^\top P(t-1)\mathbf{r}(t)f(t).$$

Using $\mathbf{r}(t)^\top P(t-1)\mathbf{r}(t) = P(t-1)\mathbf{r}(t) \cdot \mathbf{r}(t)^\top P(t-1)\mathbf{r}(t) / \|P(t-1)\mathbf{r}(t)\|$... this is getting complex. Let us use the cleaner, equivalent form.

**Final FORCE Update (clean form).** The FORCE update, as presented in [SussilloAbbott2009], is:

$$\boxed{\mathbf{k}(t) = \frac{P(t-1)\,\mathbf{r}(t)}{1 + \mathbf{r}(t)^\top P(t-1)\,\mathbf{r}(t)},}$$

$$\boxed{P(t) = P(t-1) - \mathbf{k}(t)\,\mathbf{r}(t)^\top P(t-1),}$$

$$\boxed{\Delta\mathbf{w}(t) = -e^-(t)\,\mathbf{k}(t),}$$

where $e^-(t) = \mathbf{w}(t-1)^\top\mathbf{r}(t) - f(t)$ is the error before the update.

## 11.2.4 Dimensional Analysis and Tracking

Let us verify dimensions systematically:

| Quantity | Symbol | Dimensions |
|----------|--------|------------|
| Reservoir state | $\mathbf{r}(t)$ | $N \times 1$ |
| Output weights | $\mathbf{w}(t)$ | $N \times 1$ |
| Inverse correlation matrix | $P(t)$ | $N \times N$ |
| Gain vector | $\mathbf{k}(t)$ | $N \times 1$ |
| Error | $e^-(t)$ | scalar |
| Target | $f(t)$ | scalar |

Check $\mathbf{k}(t)$: $P(t-1)$ is $N \times N$, $\mathbf{r}(t)$ is $N \times 1$, so $P(t-1)\mathbf{r}(t)$ is $N \times 1$; $\mathbf{r}(t)^\top P(t-1)\mathbf{r}(t)$ is a scalar; $\mathbf{k}(t)$ is $N \times 1$. ✓

Check $P(t) = P(t-1) - \mathbf{k}(t)\mathbf{r}(t)^\top P(t-1)$: $\mathbf{k}(t)$ is $N \times 1$, $\mathbf{r}(t)^\top$ is $1 \times N$, $P(t-1)$ is $N \times N$; so $\mathbf{k}(t)\mathbf{r}(t)^\top P(t-1)$ is $N \times N$. ✓

Check $\Delta\mathbf{w}(t) = -e^-(t)\mathbf{k}(t)$: scalar times $N \times 1$ = $N \times 1$. ✓

**Computational cost per step:**
- Computing $P(t-1)\mathbf{r}(t)$: $O(N^2)$.
- Computing scalar $\mathbf{r}(t)^\top P(t-1)\mathbf{r}(t)$: $O(N)$ given $P(t-1)\mathbf{r}(t)$.
- Updating $P(t)$: $O(N^2)$ for the outer product subtraction.
- Total per step: $O(N^2)$.

For $N = 1000$ and $T = 10^5$ training steps: $10^{10}$ operations — heavy but feasible on modern hardware.

## 11.2.5 The Self-Correcting Mechanism

The intuition for why FORCE works is the **self-correcting mechanism**: the RLS updates drive the error $e^-(t)$ toward zero, and as $e^-(t) \to 0$, the feedback $z(t) \to f(t)$, which drives the reservoir toward the state it would be in if the target were the teacher. Over time, the reservoir's state trajectory is sculpted to follow the target signal spontaneously.

More precisely: consider what happens when the error $e^-(t)$ is large. The weight update $\Delta\mathbf{w}(t) = -e^-(t)\mathbf{k}(t)$ changes the output weights to reduce the error at the current state $\mathbf{r}(t)$. Because the feedback $z(t)$ changes accordingly, the reservoir state at the next step $\mathbf{r}(t+1)$ is also shifted. This creates a *closed-loop* correction: the weight update not only corrects the current output but also pre-corrects the reservoir's future trajectory.

The gain vector $\mathbf{k}(t)$ plays a key role: it is the correlation between the current state and all past states (via the matrix $P(t-1)$), normalized by the variance of the state. When $P(t) \to 0$ (the inverse correlation accumulates and $P$ shrinks), the gain goes to zero and the updates stop — the system has converged to the RLS solution.

## 11.2.6 Convergence Analysis

FORCE converges when the reservoir is in the "chaos suppression" regime: the feedback signal has enough power to override the chaotic divergence of nearby trajectories, and the RLS algorithm can track the required output weights.

**Spectral radius condition.** For convergence, the effective spectral radius of the *closed-loop* system (reservoir + feedback) must be less than 1. The open-loop reservoir has spectral radius $\rho_W$, which may be $> 1$ (the original Sussillo-Abbott paper used $\rho_W = 1.5$, explicitly in the chaotic regime). The feedback $z(t) = \mathbf{w}^\top\mathbf{r}(t)$ adds a rank-1 term to the effective weight matrix: $W_{eff} = W + \mathbf{w}^{fb}\mathbf{w}^\top$. For the closed-loop system to be stable, the spectral radius of $W_{eff}$ must be less than 1 (at the operating point).

**Convergence guarantee.** Sussillo and Abbott gave an informal argument: as long as the RLS updates reduce the error faster than the chaotic dynamics grow, the system converges. A more rigorous analysis [RivkinasToth2022] shows that FORCE converges with high probability when:

$$\rho_W < \rho_{crit} \approx \frac{\sqrt{N}}{\|\mathbf{w}^{fb}\|},$$

i.e., the chaos is not too strong relative to the feedback gain.

**Practical criterion:** FORCE typically converges when the error $|e^-(t)|$ decreases by at least one order of magnitude per $N$ training steps.

## 11.2.7 Step-by-Step FORCE Algorithm

```python
import numpy as np

def force_training(W, w_fb, f_target, T, N, sigma_r=1.0, 
                   lambda_reg=1.0, dt_update=2):
    """
    FORCE training algorithm.
    
    W:          (N, N) recurrent weights (spectral radius > 1 initially)
    w_fb:       (N,) feedback weights (fixed, randomly initialized)
    f_target:   (T,) target signal
    T:          number of training steps
    N:          number of neurons
    sigma_r:    initial state noise scale (starts with random state)
    lambda_reg: initial value of P (= lambda_reg * I)
    dt_update:  update interval (update every dt_update steps)
    
    Returns: w_out (trained readout weights), error history
    """
    # Initialize
    r = sigma_r * np.random.randn(N)  # random initial state
    w = np.zeros(N)                   # start with zero output weights
    P = (1.0 / lambda_reg) * np.eye(N)  # initial inverse correlation = lambda^{-1} I
    z = 0.0                           # initial output
    
    errors = []
    
    for t in range(T):
        # Step 1: Advance reservoir (using previous output z as feedback)
        r = np.tanh(W @ r + w_fb * z)
        
        # Step 2: Compute current output and error
        z = w @ r
        e_minus = z - f_target[t]    # error before update
        errors.append(abs(e_minus))
        
        # Step 3: FORCE update (every dt_update steps)
        if t % dt_update == 0:
            # Gain vector
            Pr = P @ r
            denom = 1.0 + r @ Pr
            k = Pr / denom
            
            # Update P
            P -= np.outer(k, r @ P)
            
            # Update weights
            w -= e_minus * k
            
            # Recompute output with updated weights
            z = w @ r
    
    return w, errors

# Example: train on a sinusoid
T = 50000
N = 200
target_freq = 1.0 / 100.0  # period of 100 steps

np.random.seed(42)
# Chaotic reservoir: spectral radius 1.5
W = np.random.randn(N, N) / np.sqrt(N)  # Dense, variance 1/N
eigs = np.abs(np.linalg.eigvals(W))
W *= 1.5 / eigs.max()  # Scale to rho = 1.5 (chaotic!)

w_fb = np.random.randn(N) / np.sqrt(N)

t_array = np.arange(T)
f_target = np.sin(2 * np.pi * target_freq * t_array)

w_out, errors = force_training(W, w_fb, f_target, T, N)

import matplotlib.pyplot as plt
plt.figure(figsize=(12, 4))
plt.semilogy(errors)
plt.xlabel('Training step')
plt.ylabel('|e(t)| (log scale)')
plt.title('FORCE learning error convergence')
plt.savefig('force_convergence.pdf')
```

The error should drop by several orders of magnitude over the training run, demonstrating successful FORCE convergence.

## 11.2.8 FORCE vs. Standard ESN Training

| Property | ESN (ridge regression) | FORCE |
|----------|------------------------|-------|
| Reservoir weights $W$ | Fixed | Fixed |
| Output weights | Batch, offline | Online, RLS |
| Feedback during training | None | Output fed back |
| Target signal | Needed after washout | Needed throughout |
| Reservoir spectral radius | $\rho < 1$ (stable) | Can be $\rho > 1$ (chaotic) |
| Computational cost | $O(TN^2)$ (offline) | $O(TN^2)$ (online) |
| Convergence guarantee | Exact (convex) | Approximate (convergence depends on chaos level) |
| Autonomous generation | No (needs input) | Yes (output fed back) |

The key difference: FORCE can train a *chaotic* reservoir to produce a target signal autonomously, while standard ESN training requires a stable ($\rho < 1$) reservoir and cannot produce autonomous periodic patterns without a continuous external input.

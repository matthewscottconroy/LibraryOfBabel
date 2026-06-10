# Chapter 8 Exercises

## Analytical Exercises

**Exercise 8.1 (Spectral radius and memory — exact scalar formula).**
For a scalar linear leaky integrator $r(t) = (1-\alpha)r(t-1) + \alpha(wr(t-1) + w^{in}u_t)$ with $|\tilde{w}| = |1 - \alpha(1-w)| < 1$ and i.i.d. unit-variance input:

(a) Compute the stationary variance $\sigma_r^2 = \text{Var}[r(t)]$ in closed form.

(b) Compute $MC_k$ for all $k \geq 1$. Express as a function of $\tilde{w} = 1 - \alpha(1-w)$ and $\alpha w^{in}$.

(c) Sum to obtain $MC = \sum_k MC_k$. Verify that $MC \leq 1 = N$.

(d) Fix $w = 0.9$ (spectral radius of the non-leaky reservoir). Plot $MC$ as a function of $\alpha \in (0, 1]$. At what $\alpha$ is $MC$ maximized? Is this consistent with the "match the timescale" heuristic?

(e) Now fix $\alpha = 0.5$ and vary $w$. How does $MC$ depend on $w$? What happens as $w \to 1$?

---

**Exercise 8.2 (Effective spectral radius under saturation).**
Consider a linear reservoir $W$ with spectral radius $\rho_W = 0.9$. Assume the stationary pre-activation distribution is approximately $\mathcal{N}(0, \sigma_a^2)$ and that $\sigma_a = c \cdot \sigma_{in}$ for some constant $c$ determined by the reservoir connectivity.

(a) Write the effective spectral radius as $\rho_{eff}(\sigma_{in}) = \bar{g}(\sigma_{in}) \cdot \rho_W$, where $\bar{g}(\sigma_{in}) = \mathbb{E}_{a \sim \mathcal{N}(0, c^2\sigma_{in}^2)}[\tanh'(a)]$.

(b) Show that $\bar{g}(\sigma_{in}) = 1 - \mathbb{E}[\tanh^2(a)]$ and that for large $\sigma_{in}$, $\bar{g}(\sigma_{in}) \to 0$.

(c) Numerically compute $\rho_{eff}(\sigma_{in})$ for $\sigma_{in} \in \{0.1, 0.5, 1, 2, 5\}$ with $c = 1$. Plot the result.

(d) At what $\sigma_{in}$ does $\rho_{eff}$ fall below 0.5? Below 0.1?

(e) Generalize: for a target effective spectral radius $\rho_{eff}^*$, derive an equation for the maximum allowable $\sigma_{in}$ given $c$ and $\rho_W$.

---

**Exercise 8.3 (Frequency response of leaky integrator).**
For a scalar leaky integrator (linear, $w = 0$) with leak rate $\alpha$:

$$r(t) = (1-\alpha)r(t-1) + \alpha w^{in} u_t.$$

(a) Compute the transfer function $H(z) = R(z)/U(z)$.

(b) Compute the magnitude response $|H(e^{i\theta})|$ for $\theta \in [0, \pi]$.

(c) Find the $-3$ dB cutoff frequency $\theta_c$ where $|H(e^{i\theta_c})| = |H(1)|/\sqrt{2}$.

(d) Express $\theta_c$ as a function of $\alpha$ (for small $\alpha$, use the approximation $\theta_c \approx \sqrt{2}\alpha^{1/2}$).

(e) For a signal with period $T_0 = 20$ samples, what value of $\alpha$ puts the $-3$ dB cutoff at the signal's fundamental frequency?

---

**Exercise 8.4 (Joint $\rho$-$\sigma_{in}$ optimization).**
You are designing a reservoir for the following task: $y_t = u_{t-5}^2$ where $u_t \sim \mathcal{U}(-1,1)$.

(a) Decompose the target $u_{t-5}^2$ in the Legendre polynomial basis for uniform $[-1,1]$ inputs: $u^2 = \frac{1}{3}L_0(u) + \frac{2}{3}L_2(u)$ where $L_0 = 1$ and $L_2(u) = (3u^2-1)/2$. What are the relevant capacity components?

(b) The task requires 5-step memory ($k=5$) *and* a quadratic transformation. Using the analysis from Sections 8.2 and 8.3, argue that there is a tension between the requirements: long memory favors large $\rho$ and small $\sigma_{in}$, while quadratic sensitivity requires moderate $\sigma_{in}$.

(c) Set up the optimization problem: maximize $C_{L_2(u_{t-5})}$ (capacity for the quadratic 5-step delayed target) subject to the constraint that the reservoir is stable. What is the objective as a function of $(\rho, \sigma_{in})$?

(d) Using the approximations from Sections 8.2.2 and 8.3.4, derive an approximate formula for the optimal $(\rho^*, \sigma_{in}^*)$.

---

**Exercise 8.5 (Heterogeneous leak rates — frequency response).**
A reservoir is split into two halves: $N/2$ neurons with leak rate $\alpha_1 = 0.8$ and $N/2$ neurons with leak rate $\alpha_2 = 0.1$. Both halves have the same isolated spectral radius $\rho = 0.9$ (ignoring cross-group connections for this exercise).

(a) Compute the effective pole locations $\tilde{w}_1 = 1 - \alpha_1(1-\rho)$ and $\tilde{w}_2 = 1 - \alpha_2(1-\rho)$.

(b) Compute the corresponding effective time constants $\tau_1$ and $\tau_2$.

(c) Plot the magnitude frequency responses $|H_1(e^{i\theta})|$ and $|H_2(e^{i\theta})|$ for $\theta \in [0, \pi]$. At what frequencies do the two groups differ by the most?

(d) Argue that the combined state $[\mathbf{r}_1(t)^\top, \mathbf{r}_2(t)^\top]^\top$ provides a broader frequency representation of the input history than either group alone.

---

**Exercise 8.6 (Optimal regularization under different $\rho$).**
The ridge regression readout minimizes $\|Y - R W^{out}\|^2 + \lambda \|W^{out}\|^2$, with solution $W^{out} = (R^\top R + \lambda I)^{-1} R^\top Y$.

(a) Show that the effective degrees of freedom of the readout (the "hat matrix" trace) is $\text{df}(\lambda) = \sum_{i=1}^N \sigma_i^2 / (\sigma_i^2 + \lambda)$, where $\sigma_i$ are the singular values of $R$.

(b) For a linear reservoir, the singular values of $R$ (the state matrix) scale as $\sigma_i \propto |\lambda_i(W)| / \sqrt{1 - |\lambda_i(W)|^2}$. Show that as $\rho \to 1$, the condition number $\sigma_1/\sigma_N$ diverges.

(c) For $\rho = 0.99$ and $N = 100$, estimate the condition number of $R$. What does this imply for the choice of $\lambda$?

(d) Propose an adaptive regularization scheme that uses mode-specific regularization to account for the ill-conditioning: $\lambda_i = \lambda / \sigma_i$ (proportional to the inverse singular value). Show that this is equivalent to fitting in whitened state space.

---

## Thought Experiments

**Thought Experiment 8.1: What is the information-theoretic meaning of the "edge of chaos"?**

The "edge of chaos" for a reservoir corresponds roughly to $\rho \approx 1$ (or more precisely, to the parameter setting where the Lyapunov exponent of the driven reservoir is near zero).

(a) At $\rho = 1$ exactly (and for a linear reservoir), what is the memory capacity $MC$? (Hint: the pole is on the unit circle — the reservoir is *marginally stable*, not contractive.)

(b) For a marginally stable linear system, the state variance diverges over time. How does this affect the definition of memory capacity? Does $MC$ remain bounded by $N$?

(c) Argued from the capacity perspective: the maximum capacity $N$ is approached as $\rho \to 1^-$. But just above $\rho = 1$, the system is unstable and the state diverges to infinity. What happens to the capacity at $\rho = 1^+$? Is there a phase transition in capacity at $\rho = 1$?

(d) For the nonlinear tanh reservoir, the dynamics are bounded (the tanh always returns the state to $[-1,1]^N$). Does the "edge of chaos" still correspond to $\rho = 1$ for the nonlinear case? What is the nonlinear analogue of the marginal stability condition?

---

**Thought Experiment 8.2: Optimal hyperparameters for a non-stationary task.**

Most hyperparameter analyses assume the task is stationary — the same function is computed at every timestep. But many real tasks are non-stationary: the relevant timescale changes over time.

(a) Consider a task where the relevant timescale switches randomly between 5 timesteps and 50 timesteps. Is there a single optimal $\rho$ for this task? Or would a mixed strategy (e.g., heterogeneous neurons) perform better?

(b) Suppose you could *adapt* the spectral radius over time (by rescaling $W$ at each step). Describe the algorithm you would use to adapt $\rho$ based on a running estimate of the current task timescale. What are the practical difficulties?

(c) Bayesian hyperparameter optimization (see Lab 8.2) finds the optimal hyperparameters for a given task by treating them as unknowns in a Gaussian process model of the performance surface. How would you modify this framework for a non-stationary task? What would the "performance surface" even mean?

---

**Thought Experiment 8.3: Physical limits on hyperparameter tuning.**

In physical implementations of reservoir computing (e.g., photonic reservoirs, memristive networks), the "hyperparameters" are physical quantities (optical coupling, resistance, membrane time constant) with physical constraints.

(a) In a photonic reservoir, the spectral radius is determined by the coupling strengths in an optical network, which are constrained by power and fabrication tolerances. If $\rho$ can only be set to within $\pm 0.05$, how does this affect the achievable memory capacity for tasks requiring $\rho > 0.95$?

(b) In a biological neural circuit, the synaptic time constants (analogous to leak rate $\alpha$) vary across cell types. Does this heterogeneity have a functional benefit in the reservoir computing framework? Can you frame this as a heterogeneous leak rate optimization?

(c) Physical reservoirs often have fixed $N$ (you cannot add more neurons after fabrication). Given a fixed $N$ and the constraint $MC \leq N$, what is the maximum complexity task (in terms of required capacity) that the physical reservoir can solve? How does this motivate the use of *multiple* interconnected physical reservoirs?

---

## Lab Exercises

**Lab 8.1: Grid Search over $\rho$ and $\sigma_{in}$.**

```python
import numpy as np
import matplotlib.pyplot as plt

def build_esn(N=100, connectivity=0.1, seed=42):
    np.random.seed(seed)
    W = np.random.randn(N, N) * (np.random.rand(N, N) < connectivity)
    w_in = np.random.randn(N)
    return W, w_in

def run_esn(W, w_in, rho, sigma_in, u, washout=200):
    N = W.shape[0]
    eigs = np.abs(np.linalg.eigvals(W))
    W_sc = W * rho / eigs.max()
    W_in_sc = w_in[:, None] * sigma_in
    
    r = np.zeros(N)
    states = []
    for t, ut in enumerate(u):
        r = np.tanh(W_sc @ r + W_in_sc.ravel() * ut)
        if t >= washout:
            states.append(r.copy())
    return np.array(states)

def evaluate_task(states, targets, ridge=1e-4):
    """Train readout and return test NRMSE."""
    T = len(states)
    split = T * 3 // 4
    X_train, X_test = states[:split], states[split:]
    y_train, y_test = targets[:split], targets[split:]
    
    N = X_train.shape[1]
    W_out = np.linalg.solve(X_train.T @ X_train + ridge * np.eye(N),
                             X_train.T @ y_train)
    y_pred = X_test @ W_out
    nrmse = np.sqrt(np.mean((y_pred - y_test)**2) / np.var(y_test))
    return nrmse

# Generate task: NARMA-10 (a standard nonlinear benchmark)
def narma10(u, n_steps):
    """NARMA-10: a 10th-order nonlinear autoregressive moving average system."""
    y = np.zeros(n_steps)
    for t in range(10, n_steps):
        y[t] = (0.3 * y[t-1] 
                + 0.05 * y[t-1] * sum(y[t-k] for k in range(1, 11))
                + 1.5 * u[t-10] * u[t-1] + 0.1)
    return y

np.random.seed(0)
T = 3000
u = np.random.uniform(0, 0.5, T + 200)
y_full = narma10(u, T + 200)

W, w_in = build_esn(N=100)

rho_vals = np.linspace(0.3, 0.99, 20)
sigma_vals = np.logspace(-2, 1, 20)
nrmse_grid = np.zeros((len(rho_vals), len(sigma_vals)))

for i, rho in enumerate(rho_vals):
    for j, sigma in enumerate(sigma_vals):
        states = run_esn(W, w_in, rho, sigma, u, washout=200)
        targets = y_full[200:][:len(states)]
        nrmse_grid[i, j] = evaluate_task(states, targets)
        
print(f"Best NRMSE: {nrmse_grid.min():.4f}")
best_i, best_j = np.unravel_index(nrmse_grid.argmin(), nrmse_grid.shape)
print(f"Best rho={rho_vals[best_i]:.3f}, sigma_in={sigma_vals[best_j]:.3f}")

plt.figure(figsize=(9, 7))
plt.contourf(np.log10(sigma_vals), rho_vals, nrmse_grid, 
             levels=30, cmap='viridis_r')
plt.colorbar(label='NRMSE')
plt.scatter([np.log10(sigma_vals[best_j])], [rho_vals[best_i]], 
            color='red', marker='*', s=200, label='Optimum')
plt.xlabel(r'$\log_{10}(\sigma_{in})$')
plt.ylabel(r'Spectral radius $\rho$')
plt.title(r'NARMA-10 NRMSE vs. $(\rho, \sigma_{in})$')
plt.legend()
plt.tight_layout()
plt.savefig('rho_sigma_heatmap.pdf')
plt.show()
```

**Tasks:**
1. Run the grid search and report the optimal $(\rho, \sigma_{in})$ for the NARMA-10 task.
2. Identify the "usable region" (NRMSE < 0.2) in the $(\rho, \log_{10}\sigma_{in})$ plane. Describe its shape.
3. Repeat for a simpler task: 5-step linear memory (target: $u_{t-5}$). Compare the optimal hyperparameters and the shape of the usable region.
4. How does the usable region change when the reservoir size is doubled to $N=200$?

---

**Lab 8.2: Bayesian Optimization of Hyperparameters.**

```python
# Requires: pip install scikit-optimize
from skopt import gp_minimize
from skopt.space import Real
from skopt.plots import plot_convergence, plot_objective
import numpy as np

W, w_in = build_esn(N=100, seed=0)
np.random.seed(1)
T = 3000
u = np.random.uniform(0, 0.5, T + 200)
y_full = narma10(u, T + 200)

def objective(params):
    rho, log_sigma, log_lambda = params
    sigma_in = 10**log_sigma
    ridge = 10**log_lambda
    states = run_esn(W, w_in, rho, sigma_in, u, washout=200)
    targets = y_full[200:][:len(states)]
    return evaluate_task(states, targets, ridge=ridge)

space = [
    Real(0.3, 0.99, name='rho'),
    Real(-2.0, 1.0, name='log_sigma_in'),
    Real(-5.0, 0.0, name='log_lambda'),
]

result = gp_minimize(
    objective,
    space,
    n_calls=50,
    n_initial_points=10,
    acq_func='EI',
    random_state=42,
    verbose=True
)

print(f"\nBayesian optimization result:")
print(f"Best NRMSE: {result.fun:.4f}")
print(f"Best params: rho={result.x[0]:.3f}, "
      f"sigma_in={10**result.x[1]:.3f}, lambda={10**result.x[2]:.6f}")

plot_convergence(result)
plt.savefig('bayesian_convergence.pdf')
```

**Tasks:**
1. Compare the Bayesian optimization result to the grid search result from Lab 8.1. Which finds a better minimum? Which is faster?
2. How many function evaluations does Bayesian optimization need to approach the grid search optimum?
3. Extend the search space to include the reservoir size $N \in \{50, 100, 200, 400\}$ (as a categorical parameter). How does the optimal configuration change with $N$?
4. Implement a comparison with random search [BergstraBengio2012]: draw 50 random points from the same space and take the best. How often does random search find a solution competitive with Bayesian optimization?

---

**Lab 8.3: Heterogeneous Leak Rate Reservoir.**

```python
import numpy as np
import matplotlib.pyplot as plt

def build_hetero_esn(N=100, n_groups=3, alpha_vals=None, rho=0.9, 
                      sigma_in=0.5, seed=0):
    """Build ESN with heterogeneous leak rates."""
    np.random.seed(seed)
    if alpha_vals is None:
        alpha_vals = [0.9, 0.3, 0.05]  # fast, medium, slow
    
    W = np.random.randn(N, N) * (np.random.rand(N, N) < 0.1)
    eigs = np.abs(np.linalg.eigvals(W))
    W *= rho / eigs.max()
    
    w_in = np.random.randn(N) * sigma_in
    
    # Assign neurons to groups
    group_size = N // n_groups
    alpha = np.zeros(N)
    for g, a in enumerate(alpha_vals):
        alpha[g*group_size:(g+1)*group_size] = a
    alpha[n_groups*group_size:] = alpha_vals[-1]  # remainder
    
    return W, w_in, alpha

def run_hetero_esn(W, w_in, alpha, u, washout=200):
    N = W.shape[0]
    r = np.zeros(N)
    states = []
    for t, ut in enumerate(u):
        r_new = np.tanh(W @ r + w_in * ut)
        r = (1 - alpha) * r + alpha * r_new
        if t >= washout:
            states.append(r.copy())
    return np.array(states)

# Test on a multi-timescale task
def multi_timescale_target(u, T):
    """Target requires both fast (lag 2) and slow (lag 20) memory."""
    y = np.zeros(T)
    for t in range(20, T):
        y[t] = 0.5 * u[t-2] + 0.5 * u[t-20]**2
    return y

np.random.seed(42)
T = 4000 + 200
u = np.random.uniform(-1, 1, T)
y = multi_timescale_target(u, T)

# Compare: homogeneous vs. heterogeneous
W, w_in, alpha_hetero = build_hetero_esn(N=100, alpha_vals=[0.9, 0.3, 0.05])
alpha_fast = np.ones(100) * 0.9
alpha_slow = np.ones(100) * 0.05
alpha_mid  = np.ones(100) * 0.3

results = {}
for name, alpha in [('fast', alpha_fast), ('slow', alpha_slow),
                     ('mid', alpha_mid), ('hetero', alpha_hetero)]:
    states = run_hetero_esn(W, w_in, alpha, u, washout=200)
    targets = y[200:][:len(states)]
    nrmse = evaluate_task(states, targets)
    results[name] = nrmse
    print(f"{name:8s}: NRMSE = {nrmse:.4f}")
```

**Tasks:**
1. For the multi-timescale target, which configuration performs best? How does the heterogeneous reservoir compare to the best homogeneous one?
2. Plot the memory capacity profile $MC_k$ for each configuration. Which configuration retains memory at both short ($k \approx 2$) and long ($k \approx 20$) delays?
3. Vary the alpha values: try $\alpha \in \{0.1, 0.5, 0.9\}$ (three groups). Plot NRMSE on the multi-timescale task as a function of the three alpha values. Where is the minimum?
4. Design your own multi-timescale task (e.g., combining features at lag 5 and lag 50) and repeat the comparison.

---

## Programming Projects

**Project 8.A: Hyperparameter Sensitivity Analysis.**

Systematically analyze the sensitivity of reservoir performance to each hyperparameter. For the NARMA-10 task:

1. Fix all hyperparameters at their optimal values (from Lab 8.1 or 8.2). Vary one at a time over a wide range and measure the performance degradation.
2. Compute the *sensitivity* $dE/d\theta$ for each hyperparameter $\theta$ (finite differences). Which hyperparameter does performance depend on most?
3. Compute the *second-order* sensitivity (Hessian diagonal): $d^2E/d\theta^2$. A large second derivative means the optimum is sharp (requires precise tuning). A small value means the optimum is flat (robust to mistuning).
4. Repeat for two other tasks: a simple linear memory task (NRMSE for predicting $u_{t-10}$) and a nonlinear classification task. Do the sensitivities change across tasks?
5. Write a 2-page analysis report discussing which hyperparameters are task-critical and which are robust.

---

**Project 8.B: Automatic Hyperparameter Selection via Cross-Validation.**

Implement a complete hyperparameter selection pipeline using $k$-fold cross-validation:

1. Implement $k$-fold time-series cross-validation (not shuffle — preserve temporal order by using the first $k-1$ folds for training and the $k$-th for validation).
2. Use random search [BergstraBengio2012] with 100 evaluations over the space $\rho \in [0.1, 0.99]$, $\sigma_{in} \in [0.01, 10]$, $\lambda \in [10^{-6}, 1]$, $\alpha \in [0.01, 1]$.
3. Compare to Bayesian optimization (from Lab 8.2) in terms of final performance and computational cost (number of function evaluations).
4. Implement early stopping for Bayesian optimization: stop after 10 evaluations with no improvement exceeding 1% of the current best. How often does this early stopping trigger?
5. Apply your pipeline to three different tasks. Does the optimal hyperparameter configuration depend heavily on the task? What patterns do you observe across tasks?

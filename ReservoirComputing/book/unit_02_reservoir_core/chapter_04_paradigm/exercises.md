# Chapter 4 Exercises

## Conceptual Exercises

**Exercise 4.1 — The Three Components**

For each of the following modifications to a standard reservoir computer, predict the effect on performance and explain why:

(a) Replace the random $W^{in}$ with $W^{in} = 0$ (no input). What does the reservoir compute now?

(b) Replace the random $W^{rec}$ with $W^{rec} = 0$ (no recurrence). What does the system compute? What class of functions can it represent?

(c) Replace the tanh nonlinearity with $f = $ identity (linear reservoir). What class of temporal functions can the linear reservoir + linear readout represent? (Hint: consider Volterra series.)

(d) Replace the linear readout with a two-layer MLP readout. What changes about the training problem? What do you gain? What do you lose?

(e) Remove the washout period entirely (train immediately from $t = 0$, starting with $\mathbf{x}_0 = \mathbf{0}$). What artifact will appear in the trained readout? For a reservoir with spectral radius $\rho = 0.9$, after how many steps does the initial condition contribute less than 1% to the state? (Use $\rho^{T_w} < 0.01$ and solve for $T_w$.)

---

**Exercise 4.2 — Echo State Property**

Let $F(\mathbf{x}, \mathbf{u}) = \tanh(W^{rec}\mathbf{x} + W^{in}\mathbf{u})$ be the state update function.

(a) Define the echo state property (ESP) in your own words. What does it guarantee about the relationship between initial conditions and the state trajectory?

(b) Let $\mathbf{a}$ and $\mathbf{b}$ be two different initial conditions. Define $\boldsymbol{\Delta}_t = \mathbf{x}_t^{(\mathbf{a})} - \mathbf{x}_t^{(\mathbf{b})}$. Show that:
$$\|\boldsymbol{\Delta}_{t+1}\| \leq \|W^{rec}\|_2 \|\boldsymbol{\Delta}_t\|$$
using the fact that $|\tanh(a) - \tanh(b)| \leq |a - b|$ for all $a, b \in \mathbb{R}$.

(c) What condition on $\|W^{rec}\|_2$ guarantees that $\|\boldsymbol{\Delta}_t\| \to 0$ exponentially? How does $\|W^{rec}\|_2$ relate to $\rho(W^{rec})$?

(d) The ESP requires $\rho(W^{rec}) < 1$ as a *necessary* condition. Show by example that $\|W^{rec}\|_2 \geq \rho(W^{rec})$, and that the condition $\rho(W^{rec}) < 1$ alone does not guarantee $\|W^{rec}\|_2 < 1$. (Construct a $2 \times 2$ example.)

(e) Why is the ESP important for training? If the ESP is violated, what goes wrong with the readout's task?

---

**Exercise 4.3 — Linear Regression Analysis**

The readout is trained by ridge regression:

$$W^{out,T} = \left(X^T X + \alpha I\right)^{-1} X^T \hat{Y}$$

(a) Derive this formula by taking the gradient of $L(W^{out}) = \|X W^{out,T} - \hat{Y}\|_F^2 + \alpha \|W^{out}\|_F^2$ with respect to $W^{out,T}$ and setting it to zero.

(b) What is the condition for $(X^T X + \alpha I)$ to be invertible? Is $\alpha > 0$ sufficient? Is $\alpha = 0$ always invertible?

(c) The singular value decomposition of $X$ is $X = U \Sigma V^T$ where $U \in \mathbb{R}^{T \times N}$, $\Sigma = \text{diag}(\sigma_1, \ldots, \sigma_N)$, $V \in \mathbb{R}^{N \times N}$. Show that the ridge regression solution can be written as:

$$W^{out,T} = V \text{diag}\!\left(\frac{\sigma_i}{\sigma_i^2 + \alpha}\right) U^T \hat{Y}$$

(d) What happens to the solution as $\alpha \to 0$? As $\alpha \to \infty$?

(e) Suppose the reservoir has $N = 1000$ units but the training set has only $T_{train} = 500$ examples. What is the rank of $X^T X$? What is the minimum $\alpha$ needed for the solution to be numerically stable (in IEEE double precision, where machine epsilon is $\approx 10^{-16}$)?

---

**Exercise 4.4 — Memory and Spectral Radius**

(a) For a linear reservoir ($f = $ identity) with $W^{rec}$ scalar, $W^{rec} = r$ (a single scalar), and scalar input $u_t$:

$$x_{t+1} = r x_t + w^{in} u_t$$

Show that:

$$x_t = \sum_{s=0}^{t-1} r^{t-1-s} w^{in} u_s + r^t x_0$$

(b) What is the weight assigned to input $u_{t-k}$ (i.e., the input $k$ steps in the past)?

(c) Define the **linear memory capacity** as the sum over all lags $k$ of the squared correlation between $u_{t-k}$ and $x_t$:

$$MC = \sum_{k=1}^{\infty} \frac{\text{Cov}^2(u_{t-k}, x_t)}{\text{Var}(u_{t-k}) \text{Var}(x_t)}$$

For the scalar linear reservoir above with i.i.d. input $u_t$, compute $MC$ as a function of $r$. (Hint: compute $\text{Cov}(u_{t-k}, x_t) = (w^{in})^2 r^{k-1}$ and sum the geometric series.)

(d) The linear memory capacity of a linear reservoir with $N$ units is exactly $N$ [Jaeger2002mem]. Interpret this result: what does it mean for the relationship between reservoir size and memory?

(e) How does the spectral radius $\rho$ affect the distribution of memory across timescales? A reservoir with $\rho \approx 1$ has ____-term memory; a reservoir with $\rho \approx 0.5$ has ____-term memory. Fill in the blanks and justify.

---

**Exercise 4.5 — Volterra Series and Reservoir Approximation**

A **second-order Volterra series** represents a temporal functional as:

$$y_t = h_0 + \sum_{k=0}^{m} h_1(k) u_{t-k} + \sum_{k_1=0}^{m} \sum_{k_2=0}^{m} h_2(k_1, k_2) u_{t-k_1} u_{t-k_2}$$

(a) How many parameters does this series have (for memory $m$)?

(b) Show that a linear reservoir cannot represent this functional (i.e., any second-order term $u_{t-k_1} u_{t-k_2}$ is beyond the representational capacity of a linear readout on a linear reservoir).

(c) Show that a nonlinear reservoir can, in principle, represent second-order terms. (Hint: consider the Taylor expansion of $\tanh(a)$ around $a = 0$, and how products of inputs appear in the second-order term.)

(d) For a reservoir with $N$ units and fading memory $m$, estimate the maximum order of Volterra series it can approximate. (This is an open-ended question; reason from the dimensionality of the state space.)

---

## Thought Experiments

**Thought Experiment 4.1 — What If Random Weights Don't Work?**

Suppose you build a reservoir with random $W^{rec}$ and $W^{in}$, run it on your training data, and find that the readout performs no better than chance.

(a) What are the most likely causes? List at least three, one related to each of: spectral radius, input scaling, and reservoir size.

(b) Suppose the spectral radius is $\rho = 0.1$ (very small). What does the state trajectory look like? Why would this cause poor performance?

(c) Suppose the spectral radius is $\rho = 1.5$ (too large). What happens to the state trajectory? Why would this cause poor performance?

(d) Suppose $\sigma_{in}$ (input scaling) is very large (say, $10^3$). The input drives the neurons deep into the saturation region of $\tanh$. What happens to the effective nonlinearity? To the diversity of state trajectories?

(e) Suppose the reservoir size is $N = 5$. No matter how good the random weights are, the readout is limited to a 5-dimensional representation. For a task that requires tracking 10 independent bits of input history simultaneously, what is the minimum necessary $N$?

---

**Thought Experiment 4.2 — What Is the Minimum Reservoir?**

What is the simplest possible "reservoir"?

(a) A reservoir with $N = 1$ unit is simply a scalar system: $x_{t+1} = \tanh(r x_t + w^{in} u_t)$. The readout is $y_t = w^{out} x_t$. What is the class of functions this system can represent?

(b) Suppose the task is: output $y_t = u_t + u_{t-1}$ (sum of current and previous input). Can a $N=1$ reservoir represent this? If not, what is the minimum $N$?

(c) Suppose the task is: output $y_t = u_t \cdot u_{t-1}$ (product of current and previous input, a nonlinear dependency). Can a linear reservoir of any size represent this? Can a nonlinear $N=1$ reservoir? If not, what is the minimum $N$ for a nonlinear reservoir?

(d) Generalize: what is the minimum reservoir size needed to represent a $d$-th order Volterra series with memory $m$? (Provide a lower bound argument, not just a construction.)

(e) The theoretical minimum and the practical minimum are often different. What factors in practice require larger reservoirs than the theoretical minimum? (Consider noise, finite training data, and conditioning of the regression problem.)

---

**Thought Experiment 4.3 — When Is a Reservoir Equivalent to a Volterra Series?**

(a) A finite-order, finite-memory Volterra series with order $p$ and memory $m$ has $(m+1)^p$ terms (ignoring symmetry). A reservoir with $N$ units has $N$ state dimensions. For what relationship between $N$, $p$, and $m$ is a reservoir likely to provide an accurate approximation?

(b) Argue that a reservoir of size $N$ can represent *at least* an $N$-dimensional linear subspace of the Volterra series. What additional conditions are needed to access the nonlinear terms?

(c) In the limit $\rho \to 0^+$ (spectral radius approaching zero), the reservoir has essentially no memory. In this limit, the state is approximately $\mathbf{x}_t \approx f(W^{in}\mathbf{u}_t)$: a static, memoryless transformation of the current input. What does the reservoir + linear readout compute in this limit?

(d) In the limit $\rho \to 1^-$ (spectral radius approaching one from below), the reservoir has a very long effective memory. What kinds of tasks does this favor? What kinds does it hurt?

(e) The optimal spectral radius is task-dependent. Propose a criterion for choosing $\rho$ based on the characteristic memory time of the target function.

---

## Lab Exercises

**Lab 4.1 — Build a Minimal ESN in Under 50 Lines**

Build a complete, working echo state network using only NumPy. Target: under 50 lines of functional code (not counting comments and imports).

```python
import numpy as np
import matplotlib.pyplot as plt

# ============================================================
# MINIMAL ECHO STATE NETWORK  (fill in the TODOs)
# ============================================================

def build_reservoir(N, K, rho=0.9, sigma_in=0.5, seed=0):
    """
    Construct random reservoir and input matrices.
    
    Returns W_rec (N,N) with spectral radius rho,
    and W_in (N,K) with entries in [-sigma_in, sigma_in].
    """
    rng = np.random.default_rng(seed)
    
    # Sample a random dense Gaussian reservoir matrix and rescale its spectral radius.
    # The spectral radius is the magnitude of the largest eigenvalue; scaling ensures
    # that rho(W_rec) = rho_target, which controls the echo state property.
    W_rec = rng.standard_normal((N, N))
    current_rho = np.max(np.abs(np.linalg.eigvals(W_rec)))
    W_rec = W_rec * (rho / current_rho)   # rescale so spectral radius equals rho

    # Sample W_in uniformly from [-sigma_in, sigma_in]; each entry is independent.
    W_in = rng.uniform(-sigma_in, sigma_in, (N, K))
    return W_rec, W_in


def run_reservoir(U, W_rec, W_in, T_washout=100):
    """
    Run reservoir on input sequence U (T, K).
    Return state matrix X (T - T_washout, N).
    """
    T, K = U.shape
    N    = W_rec.shape[0]
    x    = np.zeros(N)
    
    # Forward pass: drive the reservoir through all T steps.
    # Discard the first T_washout states (the "washout" period removes the influence
    # of the arbitrary zero initial condition x_0 = 0).
    X = np.zeros((T - T_washout, N))
    for t in range(T):
        x = np.tanh(W_rec @ x + W_in @ U[t])   # standard ESN state update
        if t >= T_washout:
            X[t - T_washout] = x               # record states only after washout
    return X


def train_readout(X, Y_target, alpha=1e-6):
    """
    Solve ridge regression: W_out^T = (X^T X + alpha I)^{-1} X^T Y_target.
    Return W_out (M, N).
    """
    # Ridge regression: W_out^T = (X^T X + alpha I)^{-1} X^T Y_target.
    # np.linalg.solve is preferred over explicit inversion for numerical stability.
    # W_out has shape (M, N): each row is one output's readout weight vector.
    W_out = np.linalg.solve(X.T @ X + alpha * np.eye(X.shape[1]), X.T @ Y_target).T
    return W_out


def predict(U_test, W_rec, W_in, W_out):
    """Predict outputs for test input sequence."""
    # Run the reservoir on the test input (no washout needed if we start from
    # the final state of the training run, or accept a short transient).
    X_test = run_reservoir(U_test, W_rec, W_in, T_washout=0)
    # Apply the learned linear readout: Y_pred = X_test @ W_out.T
    Y_pred = X_test @ W_out.T   # shape (T_test, M)
    return Y_pred


# ============================================================
# TEST: Mackey-Glass time series (simplified)
# ============================================================

def mackey_glass(T=5000, tau=17, dt=0.1):
    """Generate a truncated Mackey-Glass time series."""
    x = np.zeros(T + tau)
    x[:tau] = 0.9 * np.ones(tau) + 0.1 * np.random.randn(tau)
    for t in range(tau, T + tau):
        x[t] = (x[t-1] 
                + dt * (0.2 * x[t-tau] / (1 + x[t-tau]**10) - 0.1 * x[t-1]))
    return x[tau:]


if __name__ == "__main__":
    # Generate data
    series = mackey_glass(T=5000)
    U = series[:-1].reshape(-1, 1)   # input:  x_t
    Y = series[1:].reshape(-1, 1)    # target: x_{t+1}
    
    split = 4000
    U_train, Y_train = U[:split], Y[:split]
    U_test,  Y_test  = U[split:], Y[split:]
    
    # Train
    N = 200
    W_rec, W_in = build_reservoir(N=N, K=1, rho=0.9)
    X_train     = run_reservoir(U_train, W_rec, W_in, T_washout=100)
    W_out       = train_readout(X_train, Y_train[100:])  # align washout
    
    # Test
    Y_pred = predict(U_test, W_rec, W_in, W_out)
    
    mse = np.mean((Y_pred - Y_test)**2)
    print(f"Test MSE: {mse:.6f}")
    
    # Plot
    plt.figure(figsize=(12, 4))
    plt.plot(Y_test[:500], label='Target', alpha=0.7)
    plt.plot(Y_pred[:500], label='Predicted', alpha=0.7)
    plt.legend()
    plt.title(f'Mackey-Glass Prediction (MSE = {mse:.4f})')
    plt.savefig('esn_prediction.png', dpi=150)
    plt.show()
```

**Tasks:**
1. Implement all four functions.
2. Run on the Mackey-Glass series. Report test MSE.
3. Vary the spectral radius: try $\rho \in \{0.3, 0.6, 0.9, 0.95, 0.99, 1.1\}$. Plot MSE vs $\rho$. What value gives the best performance?
4. Vary the reservoir size: $N \in \{10, 50, 100, 200, 500, 1000\}$. Plot MSE vs $N$.

---

**Lab 4.2 — Visualize Reservoir State Space**

Investigate the structure of the reservoir's state space and what it reveals about the reservoir's behavior.

```python
import numpy as np
import matplotlib.pyplot as plt
from sklearn.decomposition import PCA

def visualize_state_space(U, W_rec, W_in, T_washout=100, n_components=3):
    """
    Run reservoir on input and visualize first 3 PCA components of states.
    
    U : (T, K) input sequence
    """
    # Step 1: Run reservoir and collect states
    # TODO: use run_reservoir from Lab 4.1
    X = run_reservoir(U, W_rec, W_in, T_washout)
    
    # Step 2: PCA of states
    pca  = PCA(n_components=n_components)
    X_pca = pca.fit_transform(X)   # (T, 3)
    
    # Step 3: Plot
    fig = plt.figure(figsize=(12, 5))
    
    # 2D projection
    ax1 = fig.add_subplot(121)
    ax1.scatter(X_pca[:, 0], X_pca[:, 1], c=np.arange(len(X_pca)),
                cmap='viridis', s=1, alpha=0.5)
    ax1.set_xlabel('PC 1')
    ax1.set_ylabel('PC 2')
    ax1.set_title('State Space (first 2 PCs)')
    
    # Explained variance
    ax2 = fig.add_subplot(122)
    ax2.bar(range(1, n_components + 1), pca.explained_variance_ratio_ * 100)
    ax2.set_xlabel('Principal Component')
    ax2.set_ylabel('Explained Variance (%)')
    ax2.set_title('PCA Explained Variance')
    
    plt.tight_layout()
    plt.savefig('state_space.png', dpi=150)
    plt.show()
    
    return X_pca, pca.explained_variance_ratio_
```

**Tasks:**
1. Run the visualization for three settings: $\rho = 0.5$ (subcritical), $\rho = 0.9$ (near-critical), $\rho = 1.1$ (supercritical).
2. Compare the state space structure in all three cases. What do you observe?
3. For the near-critical reservoir, color the trajectory by the input value. What structure appears?
4. Compute the **participation ratio** of the states: $PR = (\sum_i \lambda_i)^2 / \sum_i \lambda_i^2$, where $\lambda_i$ are eigenvalues of the covariance matrix of $X$. This measures the "effective dimensionality" of the state space. How does $PR$ depend on $\rho$ and $N$?
5. Plot the autocorrelation of individual reservoir units as a function of time lag. How does the decay time relate to $\tau_{\text{eff}} = -1/\ln \rho$?

---

**Lab 4.3 — Compare RC to BPTT on a Benchmark**

Compare the performance of a reservoir computer and a trained RNN (BPTT) on the NARMA10 benchmark.

The **NARMA10 system** (Nonlinear Autoregressive Moving Average of order 10) is defined by:

$$y_{t+1} = 0.3 y_t + 0.05 y_t \left(\sum_{i=0}^{9} y_{t-i}\right) + 1.5 u_{t-9} u_t + 0.1$$

where $u_t \sim \text{Uniform}(0, 0.5)$ is the input. This system has significant nonlinear temporal dependencies spanning 10 time steps.

```python
import numpy as np

def generate_narma10(T=5000, seed=0):
    """Generate NARMA10 time series."""
    rng = np.random.default_rng(seed)
    u = rng.uniform(0, 0.5, T + 10)
    y = np.zeros(T + 10)
    y[:10] = 0.0  # initialization
    
    for t in range(10, T + 10):
        y[t] = (0.3 * y[t-1] 
                + 0.05 * y[t-1] * np.sum(y[t-10:t]) 
                + 1.5 * u[t-10] * u[t-1] 
                + 0.1)
    
    return u[10:].reshape(-1, 1), y[10:].reshape(-1, 1)


# TRAINING COMPARISON PROTOCOL
# 1. Generate 5000 steps of NARMA10 (4000 train, 1000 test)
# 2. Train three models:
#    (a) Reservoir computer (use build_reservoir, run_reservoir, train_readout from Lab 4.1)
#    (b) Vanilla RNN with BPTT (use VanillaRNN from Chapter 3 Lab 3.1)
#    (c) LSTM (use torch.nn.LSTM)
# 3. For each model:
#    - Record training time (wall clock)
#    - Record test NMSE = MSE / Var(y_test)
#    - Record test MSE
# 4. Plot a Pareto plot: test NMSE vs. training time

# Metrics
def nmse(y_pred, y_true):
    """Normalized Mean Squared Error."""
    return np.mean((y_pred - y_true)**2) / np.var(y_true)
```

**Tasks:**
1. Implement the training comparison protocol.
2. For the reservoir, sweep $\rho \in \{0.7, 0.85, 0.9, 0.95, 0.99\}$ and report the best result.
3. For the trained RNN, use 100 hidden units and train for 1000 epochs with Adam optimizer.
4. Plot test NMSE vs. training time for all three models.
5. Write a 1-page analysis: (a) Which method achieves the best NMSE? (b) Which achieves the best NMSE per unit of training time? (c) What does this suggest about when each method is preferred?

---

## Programming Projects

**Project 4.A — Systematic Hyperparameter Study**

Conduct a systematic study of the effect of reservoir hyperparameters on performance across multiple tasks.

**Tasks:**
- NARMA10 (Lab 4.3)
- Mackey-Glass prediction (Lab 4.1)
- Santa Fe laser series prediction
- Spoken digit classification (from the TI-46 corpus or similar)

**Hyperparameters to sweep:**
- $N \in \{50, 100, 200, 500, 1000, 2000\}$
- $\rho \in \{0.5, 0.7, 0.85, 0.9, 0.95, 0.99, 1.05\}$
- $\sigma_{in} \in \{0.01, 0.1, 0.5, 1.0, 2.0\}$
- Sparsity $p \in \{0.05, 0.1, 0.2, 0.5, 1.0\}$
- $\alpha \in \{10^{-8}, 10^{-6}, 10^{-4}, 10^{-2}, 1.0\}$

For each task-hyperparameter combination, run 5 random seeds and report mean and standard deviation of test NMSE.

Produce: (a) heatmaps of performance vs. $(\rho, N)$ for each task; (b) plots of performance vs. each hyperparameter with others fixed at their optimum; (c) a discussion of whether the optimal hyperparameters are task-dependent or task-general.

---

**Project 4.B — Physical Reservoir Prototype**

(Anticipating Unit VII.) A physical dynamical system can serve as a reservoir if it satisfies the separation property and the fading memory property. In this project, you will simulate two candidate "physical reservoirs" and compare their computational properties to a standard ESN.

**System 1: Delay-line reservoir.** A scalar nonlinear delay system:

$$x(t) = \tanh\!\left(\eta x(t - \tau) + \sigma_{in} u(t)\right)$$

discretized with time step $\delta t$. This produces $N = \tau / \delta t$ "virtual neurons" from a single physical node.

**System 2: Coupled oscillators.** A network of $N$ damped oscillators:

$$\ddot{q}_i + \gamma \dot{q}_i + \omega_i^2 q_i = \sum_j C_{ij} q_j + W^{in}_i u(t)$$

where $C_{ij}$ are random coupling coefficients and $\omega_i$ are natural frequencies.

**Tasks:**
1. Implement both systems and run them on the NARMA10 task.
2. Compute the participation ratio (from Lab 4.2) of the states from each system.
3. Measure the effective memory of each system by computing the linear memory capacity (Exercise 4.4c).
4. Compare performance on NARMA10 to a standard ESN of the same effective size.
5. Write a two-page analysis: What properties of the physical substrates make them good or bad reservoirs? What would you need to change to improve their performance?

---

## References for Exercises

- [Jaeger2001] Jaeger, H. (2001). The "echo state" approach to analysing and training recurrent neural networks. *GMD Report 148*.
- [Jaeger2002mem] Jaeger, H. (2002). Short term memory in echo state networks. *GMD Report 152*.
- [Maass2002] Maass, W., Natschläger, T., & Markram, H. (2002). Real-time computing without stable states. *Neural Computation*, 14(11), 2531–2560.
- [Lukoševičius2012] Lukoševičius, M. (2012). A practical guide to applying echo state networks. In *Neural Networks: Tricks of the Trade* (pp. 659–686). Springer.
- [Atiya2000] Atiya, A. F., & Parlos, A. G. (2000). New results on recurrent network training. *IEEE Trans. Neural Networks*, 11(3), 697–709.

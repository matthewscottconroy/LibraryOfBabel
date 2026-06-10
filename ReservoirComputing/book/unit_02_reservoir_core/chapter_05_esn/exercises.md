# Chapter 5: Exercises

## Part A: Conceptual Exercises

**A1. ESP via Contraction Mapping.**

Let $F : \mathbb{R}^N \to \mathbb{R}^N$ be defined by $F(x) = \tanh(Ax + b)$ where $A \in \mathbb{R}^{N \times N}$ and $b \in \mathbb{R}^N$.

(a) Show that $F$ is Lipschitz with Lipschitz constant $\|A\|_2$. (Hint: use the fact that $|\tanh'(z)| \leq 1$.)

(b) Conclude that if $\|A\|_2 < 1$, then $F$ is a contraction.

(c) By the Banach fixed-point theorem, $F$ has a unique fixed point $x^*$. Show that $x^*$ satisfies $x^* = \tanh(Ax^* + b)$ and that for any starting point $x_0$, the iterates $x_n = F(x_{n-1})$ converge to $x^*$ with error $\|x_n - x^*\| \leq \|A\|_2^n \|x_0 - x^*\|$.

(d) Now consider the driven map $F_t(x) = \tanh(Ax + W^{in} u_t + b)$ where $u_t$ varies. Generalize your argument to show that if $\|A\|_2 < 1$, any two trajectories $x_t = F_{t-1}(x_{t-1})$ and $x_t' = F_{t-1}(x_{t-1}')$ satisfy $\|x_t - x_t'\| \leq \|A\|_2^t \|x_0 - x_0'\|$, regardless of the input sequence.

(e) What does this imply about the echo state property?

---

**A2. Deriving Ridge Regression from Scratch.**

Consider the loss $L(W) = \|Y - WX\|_F^2 + \lambda\|W\|_F^2$ where $W \in \mathbb{R}^{L \times N}$, $X \in \mathbb{R}^{N \times T}$, $Y \in \mathbb{R}^{L \times T}$, and $\lambda > 0$.

(a) Expand $\|Y - WX\|_F^2 = \text{tr}[(Y-WX)(Y-WX)^\top]$ and write out all four terms.

(b) Compute $\frac{\partial}{\partial W} \text{tr}(WXX^\top W^\top)$ using the identity $\frac{\partial}{\partial A}\text{tr}(ABA^\top) = 2AB$ for symmetric $B$.

(c) Set $\frac{\partial L}{\partial W} = 0$ and solve for $W$.

(d) Show that the solution $W^* = YX^\top(XX^\top + \lambda I)^{-1}$ is a global minimum (not merely a critical point) by showing that $L$ is strictly convex for $\lambda > 0$.

(e) Verify your answer dimensionally: check that each matrix product in $W^* = YX^\top(XX^\top + \lambda I)^{-1}$ has consistent dimensions.

---

**A3. Power Iteration for Spectral Radius.**

Given a matrix $A \in \mathbb{R}^{N \times N}$, the power iteration method computes $\rho(A)$ iteratively.

(a) If $A$ has eigendecomposition $A = V \Lambda V^{-1}$ with $|\lambda_1| > |\lambda_2| \geq \cdots \geq |\lambda_N|$, show that for a random vector $v_0 = \sum_i c_i v_i$ (where $v_i$ are eigenvectors), the ratio $\|Av_k\|/\|v_k\| \to |\lambda_1|$ as $k \to \infty$, provided $c_1 \neq 0$.

(b) What is the convergence rate? (Express it in terms of $|\lambda_1|$ and $|\lambda_2|$.)

(c) Power iteration converges slowly when $|\lambda_1| \approx |\lambda_2|$. Suggest a modification (the "shift-and-invert" technique) that accelerates convergence when you know that $\lambda_1 \approx \mu$ for some estimate $\mu$. What matrix would you apply power iteration to instead?

(d) For a sparse matrix with $N = 500$, $p = 0.1$ connection probability, estimate the number of nonzero entries. Is it more efficient to compute $\rho$ via power iteration or via full eigendecomposition (QR algorithm)?

(e) Write pseudocode for power iteration that returns both the spectral radius and the corresponding (approximate) eigenvector.

---

**A4. Deriving the RLS Update.**

This exercise walks through the derivation of the RLS update from the Sherman-Morrison formula.

(a) State and prove the Sherman-Morrison formula $(A + uv^\top)^{-1} = A^{-1} - \frac{A^{-1}uv^\top A^{-1}}{1 + v^\top A^{-1}u}$ by directly verifying that the right-hand side is the inverse of $A + uv^\top$.

(b) Define $C_t = C_{t-1} + x_t x_t^\top$ (ordinary least squares case, $\mu = 1$) and $P_t = C_t^{-1}$. Apply Sherman-Morrison to derive the update $P_t = P_{t-1} - \frac{P_{t-1} x_t x_t^\top P_{t-1}}{1 + x_t^\top P_{t-1} x_t}$.

(c) Define the gain vector $k_t = \frac{P_{t-1} x_t}{1 + x_t^\top P_{t-1} x_t}$. Show that $P_t x_t = k_t$ (i.e., after the update, $k_t$ is $P_t$ applied to $x_t$).

(d) The weight update is $w_t = P_t q_t$ where $q_t = q_{t-1} + y_t^* x_t$. Substitute to show that $w_t = w_{t-1} + k_t(y_t^* - x_t^\top w_{t-1})$.

(e) Interpret each term in the weight update: what is $y_t^* - x_t^\top w_{t-1}$? What does $k_t$ do geometrically?

---

**A5. Leaky Integrator Memory Time Constant.**

The leaky integrator ESN with no recurrent connections and $\alpha \in (0,1]$ satisfies $x_{t+1} = (1-\alpha)x_t + \alpha u_{t+1}$ for scalar $x$ and $u$.

(a) Solve this recursion in closed form: express $x_t$ as a function of $u_0, u_1, \ldots, u_t$ and $x_0$.

(b) The impulse response is the sequence of coefficients multiplying $u_{t-k}$ for $k = 0, 1, 2, \ldots$. What is this sequence? Verify that it sums to $1$.

(c) Define the effective memory length as $\tau_{eff} = \sum_{k=0}^\infty k \cdot h_k / \sum_{k=0}^\infty h_k$ where $h_k$ is the coefficient of $u_{t-k}$. Compute $\tau_{eff}$ as a function of $\alpha$.

(d) For $\alpha = 0.1$, what is $\tau_{eff}$? For $\alpha = 0.5$? For $\alpha = 0.9$? What happens in the limit $\alpha \to 0$?

(e) The power in the frequency domain: compute the transfer function $H(z) = \mathcal{Z}\{h_k\}$ of the leaky integrator. At what frequency (in units of $1/T$ where $T$ is the sampling period) does the magnitude response drop to half its DC value?

---

**A6. Spectral Radius Scaling.**

Suppose $W$ is a random matrix with i.i.d. entries from $\mathcal{N}(0, \sigma^2/N)$.

(a) By the Wigner semicircle law, the empirical spectral distribution of $W$ converges (as $N \to \infty$) to a semicircle on $[-2\sigma, 2\sigma]$. What does this imply about $\rho(W)$?

(b) If we draw $W$ with $\sigma = 1/\sqrt{N}$ (so each entry is $\sim \mathcal{N}(0, 1/N)$), what is the expected spectral radius for large $N$?

(c) If we want $\rho(W) = \rho_0$ for some target value $\rho_0$, how should we scale the entries of $W$?

(d) For a sparse matrix where each row has on average $k$ nonzero entries drawn from $\mathcal{N}(0, 1)$: by the random graph analogy, what is the expected spectral radius? How does this change the scaling needed to achieve $\rho_0$?

---

**A7. Washout Length.**

The washout period is the number of initial time steps discarded to eliminate the effect of the arbitrary initial state $x_0 = 0$.

(a) For the linear reservoir $x_{t+1} = Ax_t + W^{in} u_{t+1}$ with $\rho(A) = \rho < 1$, upper-bound the contribution of $x_0$ to $x_t$ at time $t$.

(b) If we want the contribution of $x_0$ to be less than $\epsilon \|x_0\|$, how many washout steps $T_w$ are needed (in terms of $\rho$ and $\epsilon$)?

(c) Evaluate $T_w$ for $\rho = 0.9, \epsilon = 10^{-3}$. For $\rho = 0.99, \epsilon = 10^{-3}$.

(d) What happens if the washout period is too short? What are the symptoms in a trained ESN?

(e) Is there any advantage to using a non-zero initialization $x_0 \neq 0$? When might this be useful?

---

**A8. Multiple Output Channels and Shared Reservoir.**

An ESN has $L$ output channels, with readout $y_t = W^{out} x_t$ where $W^{out} \in \mathbb{R}^{L \times N}$.

(a) Show that the ridge regression solution $W^{out} = Y^* X^\top(XX^\top + \lambda I)^{-1}$ decomposes into $L$ independent row regressions: each row $W^{out}_{l,:}$ is the solution to a separate ridge regression problem for output channel $l$.

(b) Conclude that the optimal $W^{out}$ for $L$ outputs can be computed as cheaply as for a single output (up to the factor $L$ in the final matrix multiply), as long as $(XX^\top + \lambda I)^{-1}$ is precomputed.

(c) For the RLS update (7.20), show that the covariance matrix $P_t$ is shared across all $L$ output channels and need only be updated once per time step, regardless of $L$.

(d) Suppose the $L$ target outputs $y_t^*$ are correlated (e.g., they all encode the same signal at different time lags). Can we exploit this correlation to improve the readout? Suggest a modification to ridge regression that does so.

---

## Part B: Thought Experiments

**B1. What if $\alpha = 0$?**

Consider the leaky integrator ESN $x_{t+1} = (1-\alpha)x_t + \alpha\tanh(W^{rec} x_t + W^{in} u_{t+1} + b)$ with $\alpha = 0$.

- What does the update equation become?
- What is the reservoir state at every time step?
- Can the ESN still compute anything useful?
- Is the echo state property satisfied (trivially, vacuously, or not at all)?
- What physical system does $\alpha = 0$ model?

**B2. What if $\alpha = 1$?**

Now consider $\alpha = 1$ (the vanilla ESN).

- What is the relationship between successive reservoir states?
- What continuous-time dynamics does this correspond to, in the limit $\Delta t \to \tau$?
- Is this a valid Euler discretization? What can go wrong numerically?
- The vanilla ESN has no explicit memory smoothing. Where does its temporal memory come from?
- If the reservoir has $\rho = 0.5$ and $\alpha = 1$, versus $\rho = 0.9$ and $\alpha = 0.5$, which system has longer memory? Compute the memory time constant for each.

**B3. The Geometric Meaning of the Echo State Property.**

Think geometrically about state space $\mathbb{R}^N$.

- Draw a sketch (or describe in words) showing two trajectories $x_t$ and $x_t'$ starting at different initial conditions, both driven by the same input sequence. What does the ESP say about these trajectories?
- The echo states form a curve (or surface) in $\mathbb{R}^N$ parameterized by the input history. What is the dimension of this manifold (roughly)?
- If the ESP holds, the driven reservoir defines a map from the space of input sequences to the state space. Is this map injective (one-to-one)? What would non-injectivity mean computationally?
- The readout $W^{out} x_t$ projects the reservoir state onto an $L$-dimensional subspace. Draw a diagram showing how information flows from the infinite-dimensional input history to the finite-dimensional output space.

**B4. When is a Larger Reservoir Better?**

Consider two ESNs: one with $N = 100$ neurons and one with $N = 1000$ neurons, both trained on the same task.

- All else being equal (same $\rho$, $\alpha$, and training data), what advantages does the larger reservoir have?
- The bound $MC \leq N$ (Theorem 3.1) says memory capacity is bounded by network size. What does this imply for tasks requiring very long memory?
- A larger reservoir is slower to simulate and requires more training data to avoid overfitting. Articulate the bias-variance tradeoff for reservoir size.
- When might a small, carefully designed reservoir outperform a large random one?
- Can increasing $N$ ever hurt performance (not just because of overfitting, but for a more fundamental reason)?

---

## Part C: Lab Exercises

**L1. Testing the Echo State Property Numerically.**

Write a Python function that numerically tests whether an ESN satisfies the ESP.

```python
import numpy as np

def test_esp(W_rec, W_in, alpha=1.0, T=500, n_trials=10, 
             input_dim=1, seed=42):
    """
    Test the Echo State Property numerically.
    
    Run n_trials pairs of trajectories from random initial conditions,
    driven by the same input sequence. Measure the divergence at each
    time step.
    
    Returns:
        divergences: array of shape (n_trials, T) with ||x_t - x_t'||
        converges: bool, True if all pairs converge by time T
    """
    rng = np.random.default_rng(seed)
    N = W_rec.shape[0]
    
    # Generate a random input sequence
    u = rng.standard_normal((T, input_dim))
    
    divergences = np.zeros((n_trials, T))
    
    for trial in range(n_trials):
        # YOUR CODE HERE
        # 1. Initialize two states x, x_prime randomly
        # 2. Run both forward under the same input u
        # 3. Record ||x_t - x_t'|| at each step
        pass
    
    # YOUR CODE HERE: determine if ESP holds
    converges = None  # replace with actual criterion
    
    return divergences, converges

# Test on a well-conditioned reservoir
N = 50
rho_values = [0.5, 0.9, 0.99, 1.0, 1.1]

for rho in rho_values:
    W = np.random.randn(N, N)
    W = W / np.max(np.abs(np.linalg.eigvals(W))) * rho
    W_in = np.random.randn(N, 1) * 0.1
    
    divs, converges = test_esp(W, W_in)
    print(f"rho={rho:.2f}: ESP={converges}, "
          f"final divergence={divs[:, -1].mean():.2e}")
```

Complete the function and run it. Plot the divergence curves for each $\rho$ value on a log-scale y-axis. What do you observe?

**Questions:**
(a) At approximately what value of $\rho$ does convergence fail?
(b) Is there a "soft transition" or a sharp boundary?
(c) How does the convergence rate depend on $\rho$ for $\rho < 1$? Fit an exponential and extract the contraction rate as a function of $\rho$.

---

**L2. Spectral Radius Scan on Mackey-Glass.**

Implement an ESN and measure performance as a function of $\rho$ on the Mackey-Glass time series.

```python
import numpy as np
from scipy.integrate import solve_ivp

def mackey_glass(n_steps=3000, tau=17, beta=0.2, gamma=0.1, 
                 n_int=10, seed=42):
    """Generate Mackey-Glass time series by numerical integration."""
    # Mackey-Glass DDE: dx/dt = beta*x(t-tau)/(1+x(t-tau)^10) - gamma*x(t)
    # Discretize using Euler method with small step size
    dt = 1.0 / n_int
    total_steps = n_steps * n_int + tau * n_int + 500 * n_int
    x = np.zeros(total_steps)
    x[:tau*n_int] = 0.9  # initial condition
    
    for t in range(tau*n_int, total_steps):
        x_delayed = x[t - tau*n_int]
        x[t] = x[t-1] + dt * (
            beta * x_delayed / (1 + x_delayed**10) - gamma * x[t-1]
        )
    
    return x[500*n_int::n_int]  # subsample

class ESN:
    def __init__(self, N, rho, alpha=1.0, input_scaling=0.1, 
                 lambda_reg=1e-4, seed=42):
        rng = np.random.default_rng(seed)
        # YOUR CODE HERE: initialize W_rec, W_in
        # W_rec: sparse random matrix scaled to spectral radius rho
        # W_in: dense random matrix scaled by input_scaling
        self.N = N
        self.alpha = alpha
        self.lambda_reg = lambda_reg
        self.W_out = None
        
    def run(self, u, washout=200):
        """Run reservoir and return states after washout."""
        # YOUR CODE HERE
        pass
    
    def train(self, u, y_target, washout=200):
        """Collect states, then train readout with ridge regression."""
        # YOUR CODE HERE
        pass
    
    def predict(self, u, washout=200):
        """Run trained ESN and return predictions."""
        # YOUR CODE HERE
        pass

# Generate data
data = mackey_glass(n_steps=6000)
data = (data - data.mean()) / data.std()  # normalize

# Split
u_train = data[:4000]
y_train = data[1:4001]  # one-step-ahead target
u_test = data[4000:5000]
y_test = data[4001:5001]

# Scan rho
rho_values = np.linspace(0.1, 1.3, 25)
nrmse_values = []

for rho in rho_values:
    esn = ESN(N=100, rho=rho, alpha=1.0, input_scaling=0.1)
    esn.train(u_train, y_train)
    y_pred = esn.predict(u_test)
    
    # Compute NRMSE = sqrt(mean((y_pred - y_test)**2)) / std(y_test)
    nrmse = np.sqrt(np.mean((y_pred - y_test)**2)) / np.std(y_test)
    nrmse_values.append(nrmse)
    print(f"rho={rho:.2f}: NRMSE={nrmse:.4f}")

# YOUR CODE HERE: plot rho vs NRMSE
```

**Questions:**
(a) At what value of $\rho$ is performance optimal? Is it below 1, at 1, or above 1?
(b) How sensitive is performance to small changes in $\rho$ near the optimum?
(c) Repeat the experiment with $\alpha = 0.5$ (leaky integrator). How does the optimal $\rho$ change?
(d) Explain your observations using the memory-nonlinearity tradeoff discussed in Section 3.3.

---

**L3. Online vs. Offline Training Comparison.**

Compare RLS (online) and ridge regression (offline) training on a tracking task.

```python
import numpy as np

def rls_esn_update(P, w, x, y_target, forgetting=1.0):
    """
    Single RLS update step.
    
    Args:
        P: current covariance matrix (N x N)
        w: current weight vector (N,)
        x: current reservoir state (N,)
        y_target: current scalar target
        forgetting: forgetting factor mu
    
    Returns:
        P_new, w_new, prediction_error
    """
    # YOUR CODE HERE: implement equations (7.17), (7.18), (7.19)
    pass

# Task: track a nonstationary signal
# The target switches between two patterns at T=2000
T_total = 5000
t = np.arange(T_total)

# Input: random signal
rng = np.random.default_rng(0)
u = rng.standard_normal(T_total)

# Target: changes at T=2500
# First half: y = sin(2*pi*f1*t) where f1 tracks low-frequency input
# Second half: y = sin(2*pi*f2*t) where f2 tracks high-frequency input
y_target = np.where(t < 2500, 
                    np.sin(2*np.pi*0.05*t) + 0.5*u,
                    np.sin(2*np.pi*0.15*t) + 0.5*u)

# Build ESN
N = 100
W_rec = np.random.randn(N, N)
W_rec /= np.max(np.abs(np.linalg.eigvals(W_rec)))
W_rec *= 0.95
W_in = np.random.randn(N, 1) * 0.1

# Collect reservoir states
x = np.zeros(N)
states = []
for t_idx in range(T_total):
    x = np.tanh(W_rec @ x + W_in.flatten() * u[t_idx])
    states.append(x.copy())
states = np.array(states)  # (T_total, N)

# YOUR CODE HERE:
# 1. Offline training: train on first 2000 steps, test on remainder
# 2. Online training: run RLS continuously from step 0

# Compare:
# - NRMSE in first half (before switch)
# - NRMSE in second half (after switch)
# - Adaptation speed of RLS after the switch
```

**Questions:**
(a) How long does RLS take to adapt after the distribution shift at $T = 2500$?
(b) How does the forgetting factor $\mu$ affect the adaptation speed vs. steady-state accuracy tradeoff?
(c) Plot the squared error $e_t^2$ vs. time for both methods. What do you observe around the transition point?
(d) Is there a value of $\mu$ for which RLS is essentially equivalent to offline training on the recent past?

---

**L4. Washout Length Effect.**

Systematically study the effect of washout length on ESN performance.

```python
import numpy as np

# Setup: sinusoidal target with initial transient
N = 50
T_train = 1000
T_test = 500
washout_lengths = [0, 5, 10, 20, 50, 100, 200]

# Generate input: random noise
rng = np.random.default_rng(42)
u_total = rng.standard_normal(T_train + T_test + 200)

# Build reservoir
W_rec = np.random.randn(N, N)
W_rec /= np.max(np.abs(np.linalg.eigvals(W_rec))) 
W_rec *= 0.9  # rho = 0.9

W_in = np.random.randn(N, 1) * 0.3

# Target: y_t = u_{t-5} (delayed input with memory requirement)
y_target = np.roll(u_total, 5)

# YOUR CODE HERE:
# For each washout_length:
#   1. Run reservoir forward, discarding first washout_length states
#   2. Train with ridge regression on training states
#   3. Evaluate on test set
#   4. Record NRMSE

# Also compute: the theoretical washout needed as -log(epsilon)/log(1/rho)
# for epsilon = 1e-3, 1e-6

for washout in washout_lengths:
    # ... your implementation
    pass

# Plot washout_length vs NRMSE
```

**Questions:**
(a) At what washout length does performance stabilize?
(b) Compare the empirical result to the theoretical estimate $T_w = -\log(10^{-3}) / \log(1/0.9) \approx 65$.
(c) What happens when the washout length is too short (e.g., 0)? Does the model still learn, or is performance catastrophically bad?
(d) How does the required washout length scale with $\rho$? Test your prediction.

---

## Part D: Programming Projects

**P1. ESN Library Implementation.**

Implement a clean, well-documented ESN library in Python with the following components:

- `ESN` class with methods `fit(X_train, y_train)` and `predict(X_test)`
- Reservoir initialization with configurable: $N$, $\rho$, $\alpha$, sparsity, input scaling, bias scaling
- Offline training: ridge regression with optional cross-validation for $\lambda$
- Online training: RLS with configurable forgetting factor
- State collection with configurable washout
- Utilities: spectral radius computation, ESP test, memory capacity estimation

Your library should follow the scikit-learn API convention (fit/predict/transform). Include a comprehensive test suite.

**P2. Spectral Radius and Task Complexity.**

The optimal $\rho$ depends on the task. Design an experiment to test this hypothesis:

*"Tasks requiring longer memory require higher $\rho$."*

Use a family of tasks parameterized by a "memory demand" $\tau$: predict $u_{t-\tau}$ from the input history $u_{t}, u_{t-1}, \ldots$. For each $(\rho, \tau)$ pair, train and evaluate an ESN. Plot the 2D performance surface in the $(\rho, \tau)$ plane.

Your report should:
- Confirm or refute the hypothesis with data.
- Identify the optimal $\rho$ as a function of $\tau$.
- Relate the optimal $\rho$ to the memory time constant formula (3.4).
- Discuss what happens near $\rho = 1$: is the improvement in memory capacity worth the risk of instability?

**P3. Comparing Activation Functions.**

Compare $\tanh$, sigmoid, ReLU, and a linear activation on three tasks: (1) short-memory task (reconstruct $u_t$), (2) medium-memory task (reconstruct $u_{t-10}$), (3) nonlinear task (reconstruct $u_t^2 - u_{t-1}^2$).

For each activation function and task, tune $\rho$ and $\alpha$ to their optimal values. Report:
- Best achievable NRMSE for each combination.
- Optimal hyperparameters $(\rho^*, \alpha^*)$.
- Stability issues (if any) for each activation.
- A theoretical explanation for the observed differences, based on the Lipschitz constant and boundedness properties of each activation.

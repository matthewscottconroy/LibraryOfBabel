# 4.5.1 The Training Procedure

## Overview

Training a reservoir computer is radically simpler than training a recurrent neural network. The procedure requires no gradient computation, no iterative optimization, no learning rate, and no convergence monitoring. It consists of four steps: (1) generate the reservoir, (2) run the washout, (3) collect states, (4) solve the linear system. Done.

This section describes each step in detail, with full mathematical specification. We then contrast the procedure with BPTT in terms of time complexity, memory requirements, numerical stability, and the practical experience of actually doing it.

## Step 1: Generate the Reservoir

**Choose hyperparameters:**
- $N$: number of reservoir units (typical range: 100–10,000)
- $\rho$: target spectral radius (typical range: 0.8–0.99)
- $p$: connection density (typical range: 0.01–0.20; sparse reservoirs are common)
- $\sigma_{in}$: input scaling (task-dependent; often 0.1–1.0)
- $\alpha$: regularization strength for the readout (determined by cross-validation)

**Construct $W^{rec}$:**
```
1. Sample W_tilde ~ Normal(0, 1) (N x N)
2. Sparsify: for each entry with probability (1-p), set to 0
3. Compute spectral radius: rho_0 = max|eigenvalues(W_tilde)|
4. Scale: W_rec = rho / rho_0 * W_tilde
```

The resulting $W^{rec}$ has spectral radius exactly $\rho$.

**Construct $W^{in}$:**
```
W_in ~ Uniform(-sigma_in, sigma_in) (N x K)
```

or sparse variants (one nonzero per row, connecting each neuron to one input channel).

**That's it.** No training occurs here. The reservoir is a fixed dynamical system. It will not change.

## Step 2: The Washout

Set the initial state to zero: $\mathbf{x}_0 = \mathbf{0}$.

Run the reservoir dynamics on the training input $\mathbf{u}_1, \ldots, \mathbf{u}_{T_w}$ for the washout period of $T_w$ steps:

$$\mathbf{x}_{t+1} = f\!\left(W^{rec}\mathbf{x}_t + W^{in}\mathbf{u}_t + \mathbf{b}\right), \quad t = 0, 1, \ldots, T_w - 1$$

The states $\mathbf{x}_1, \ldots, \mathbf{x}_{T_w}$ are discarded. After $T_w$ steps, the influence of the initial condition $\mathbf{x}_0 = \mathbf{0}$ has decayed to negligibility, and the state $\mathbf{x}_{T_w}$ is (approximately) a function of only the recent input history.

**How long should the washout be?** The minimum washout period is related to the effective memory time $\tau_{\text{eff}} \approx -1/\ln \rho(W^{rec})$. For $\rho = 0.9$: $\tau_{\text{eff}} \approx 9.5$ steps — a washout of 50–100 steps is more than sufficient. For $\rho = 0.99$: $\tau_{\text{eff}} \approx 99.5$ steps — a washout of 200–500 steps is appropriate.

In practice, $T_w = 100$ is a common default. For tasks with very slow dynamics or very high spectral radii, longer washouts may be needed.

## Step 3: Collect States

Starting from $\mathbf{x}_{T_w}$, run the reservoir for $T_{train}$ more steps, collecting the states:

$$\mathbf{x}_{t+1} = f\!\left(W^{rec}\mathbf{x}_t + W^{in}\mathbf{u}_t + \mathbf{b}\right), \quad t = T_w, T_w + 1, \ldots, T_w + T_{train} - 1$$

Assemble the collected states into a matrix:

$$X = \begin{bmatrix} \mathbf{x}_{T_w+1}^T \\ \mathbf{x}_{T_w+2}^T \\ \vdots \\ \mathbf{x}_{T_w+T_{train}}^T \end{bmatrix} \in \mathbb{R}^{T_{train} \times N}$$

Each row is the reservoir state at one time step. Simultaneously, collect the target outputs:

$$\hat{Y} = \begin{bmatrix} \hat{\mathbf{y}}_{T_w+1}^T \\ \vdots \\ \hat{\mathbf{y}}_{T_w+T_{train}}^T \end{bmatrix} \in \mathbb{R}^{T_{train} \times M}$$

**Optional: augment with input and bias.** A common and often beneficial practice is to augment the state with the current input and a bias unit:

$$\tilde{\mathbf{x}}_t = [\mathbf{x}_t; \mathbf{u}_t; 1] \in \mathbb{R}^{N + K + 1}$$

This allows the readout to access the current input directly (not only through the reservoir's nonlinear transformation) and provides a learnable bias. The augmented state matrix $\tilde{X} \in \mathbb{R}^{T_{train} \times (N+K+1)}$ replaces $X$ in the regression step.

## Step 4: Solve the Linear System

Find the readout weight matrix $W^{out} \in \mathbb{R}^{M \times N}$ (or $W^{out} \in \mathbb{R}^{M \times (N+K+1)}$ if augmented) that minimizes the regularized least-squares objective:

$$L(W^{out}) = \|X W^{out,T} - \hat{Y}\|_F^2 + \alpha \|W^{out}\|_F^2$$

The unique minimizer is given in closed form:

$$W^{out,T} = \left(X^T X + \alpha I\right)^{-1} X^T \hat{Y}$$

where $I \in \mathbb{R}^{N \times N}$ is the identity matrix and $\alpha > 0$ is the regularization parameter.

Equivalently, solving column by column for each output dimension $m = 1, \ldots, M$:

$$\mathbf{w}_m = \left(X^T X + \alpha I\right)^{-1} X^T \hat{\mathbf{y}}_m$$

The matrix $(X^T X + \alpha I) \in \mathbb{R}^{N \times N}$ is the **regularized Gram matrix** of the reservoir states. With $\alpha > 0$, it is positive definite and thus invertible for any $X$.

**Alternative: solve via the dual form.** When $T_{train} < N$, it is more efficient to use the dual form:

$$W^{out,T} = X^T \left(X X^T + \alpha I\right)^{-1} \hat{Y}$$

where $(XX^T + \alpha I) \in \mathbb{R}^{T_{train} \times T_{train}}$. This is cheaper when $T_{train} < N$ (the matrix to invert is smaller). For most practical cases, $T_{train} \gg N$ and the primal form is preferred.

## Comparison with BPTT

The contrast between reservoir computing training and BPTT is illuminating.

### Time Complexity

**BPTT (per gradient step):**
- Forward pass: $O(T N^2)$ (dominated by matrix-vector products $W^{rec}\mathbf{x}_t$)
- Backward pass: $O(T N^2)$
- Total per gradient step: $O(T N^2)$
- Number of gradient steps needed: $O(1000)$–$O(100000)$ for convergence
- **Total training cost: $O(10^3 \text{–} 10^5 \cdot T N^2)$**

**Reservoir training:**
- State collection (forward pass): $O(T N^2)$
- Form $X^T X$: $O(T N^2)$
- Solve linear system: $O(N^3)$
- **Total training cost: $O(T N^2 + N^3)$** — no iteration, no learning rate

For $T = 10,000$, $N = 500$: reservoir training costs approximately $T N^2 + N^3 = 2.5 \times 10^9$ operations. BPTT training for 10,000 gradient steps costs $10^4 \times T N^2 = 2.5 \times 10^{13}$ operations — 10,000 times more. For even modest iteration counts, the difference is enormous.

### Memory Requirements

**BPTT** requires storing the entire state trajectory for the backward pass: $O(T \cdot N)$ memory for states, plus $O(T \cdot N)$ for pre-activations. For long sequences and large networks, this is the primary practical bottleneck.

**Reservoir training** requires storing the state matrix $X \in \mathbb{R}^{T_{train} \times N}$, the Gram matrix $X^T X \in \mathbb{R}^{N \times N}$, and the right-hand side $X^T \hat{Y} \in \mathbb{R}^{N \times M}$. For very large $T_{train}$, $X$ can be processed in streaming fashion: accumulate $X^T X$ and $X^T \hat{Y}$ incrementally without storing all of $X$.

**Streaming update:**

$$X^T X \mathrel{+}= \mathbf{x}_t \mathbf{x}_t^T, \quad X^T \hat{Y} \mathrel{+}= \mathbf{x}_t \hat{\mathbf{y}}_t^T$$

at each step $t$. This requires only $O(N^2 + NM)$ memory, independent of $T_{train}$.

### Numerical Stability

**BPTT** is numerically unstable in the presence of exploding gradients and can exhibit chaotic behavior in the loss landscape. Gradient clipping, careful learning rate tuning, and batch normalization are often required to stabilize training.

**Reservoir training** via ridge regression is numerically stable by construction: the regularized Gram matrix $(X^T X + \alpha I)$ is positive definite for any $\alpha > 0$, and the solution is the globally unique minimum. The condition number of the system is $(s_{\max}^2 + \alpha) / (s_{\min}^2 + \alpha)$, where $s_{\max}$ and $s_{\min}$ are the largest and smallest singular values of $X$. With appropriate $\alpha$, the condition number is bounded and the solution is stable.

### The Experience of Training

There is also a qualitative difference in the *experience* of training.

BPTT training involves iterative optimization with hyperparameters (learning rate, momentum, batch size, gradient clipping threshold) that must be set by the user, often through extensive trial and error. Convergence is not guaranteed. The loss may plateau, oscillate, or diverge. Training instability is common. For a practitioner, training a large RNN on a new task is an exercise in patience and engineering.

Reservoir training involves a single run of the forward pass and a single linear algebra operation. There is no convergence to monitor, no learning rate to tune, and no risk of training instability. The only hyperparameter of the training procedure itself is $\alpha$; all other hyperparameters (spectral radius, reservoir size, input scaling) affect the quality of the state representations but do not interact with the training procedure. Cross-validation over the small set of hyperparameters is feasible and routine.

## A Worked Example

To make the procedure concrete, here is a minimal Python implementation:

```python
import numpy as np

def train_reservoir(u_train, y_train, N=200, rho=0.9, sigma_in=0.5,
                    T_washout=100, alpha=1e-6, seed=42):
    """
    Train a minimal reservoir computer.
    
    Parameters
    ----------
    u_train  : (T_total, K)  input sequence
    y_train  : (T_total, M)  target output sequence
    N        : reservoir size
    rho      : spectral radius
    sigma_in : input scaling
    T_washout: washout steps (excluded from training)
    alpha    : ridge regression regularization
    seed     : random seed
    
    Returns
    -------
    W_out  : (M, N) trained readout weights
    W_rec  : (N, N) reservoir weights (fixed)
    W_in   : (N, K) input weights (fixed)
    """
    rng = np.random.default_rng(seed)
    T, K = u_train.shape
    M    = y_train.shape[1]
    
    # Step 1: Generate reservoir
    W_tilde = rng.normal(0, 1, (N, N))
    eigenvalues = np.linalg.eigvals(W_tilde)
    W_rec = W_tilde / np.max(np.abs(eigenvalues)) * rho
    W_in  = rng.uniform(-sigma_in, sigma_in, (N, K))
    
    # Steps 2 & 3: Washout + collect states
    x = np.zeros(N)
    states = []
    
    for t in range(T):
        x = np.tanh(W_rec @ x + W_in @ u_train[t])
        if t >= T_washout:
            states.append(x.copy())
    
    X    = np.array(states)                      # (T_train, N)
    Yhat = y_train[T_washout:]                   # (T_train, M)
    
    # Step 4: Solve linear system (ridge regression)
    W_out_T = np.linalg.solve(X.T @ X + alpha * np.eye(N), X.T @ Yhat)
    W_out   = W_out_T.T                          # (M, N)
    
    return W_out, W_rec, W_in

def predict_reservoir(u_test, W_rec, W_in, W_out, x0=None):
    """Generate predictions from a trained reservoir."""
    T, K = u_test.shape
    x = x0 if x0 is not None else np.zeros(W_rec.shape[0])
    
    predictions = []
    for t in range(T):
        x = np.tanh(W_rec @ x + W_in @ u_test[t])
        predictions.append(W_out @ x)
    
    return np.array(predictions)
```

This is a complete, working implementation in under 50 lines. The contrast with a full BPTT implementation — which requires at minimum 150–200 lines for a correct backward pass, plus hyperparameter tuning and convergence monitoring — is stark.

## Summary

The reservoir computing training procedure decomposes the learning problem into: (1) a fixed, random construction (the reservoir), and (2) a single-step, globally optimal linear regression (the readout). The result is a training algorithm that is faster by orders of magnitude than BPTT, numerically stable, hyperparameter-light, and trivially parallelizable (multiple readouts can be trained simultaneously from the same state matrix). The simplicity of the procedure is not incidental — it follows directly from the decision to fix the recurrent weights, which converts a nonconvex optimization over all parameters into a convex optimization over readout weights only.

---

## References

- [Jaeger2001] Jaeger, H. (2001). The "echo state" approach to analysing and training recurrent neural networks. *GMD Report 148*.
- [Lukoševičius2012] Lukoševičius, M. (2012). A practical guide to applying echo state networks. In *Neural Networks: Tricks of the Trade* (pp. 659–686). Springer.
- [Tikhonov1977] Tikhonov, A. N., & Arsenin, V. Y. (1977). *Solutions of Ill-Posed Problems*. Wiley.

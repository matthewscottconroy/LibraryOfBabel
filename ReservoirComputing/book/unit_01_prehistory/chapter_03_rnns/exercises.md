# Chapter 3 Exercises

## Conceptual Exercises

**Exercise 3.1 — Full BPTT Derivation**

Work through the BPTT derivation from scratch, without referring to the text, for an RNN with $N = 2$ neurons, $K = 1$ input, $M = 1$ output, and sequence length $T = 3$.

(a) Write out the three state update equations explicitly:

$$\mathbf{x}_1 = f(W^{rec}\mathbf{x}_0 + W^{in}u_0 + \mathbf{b})$$
$$\mathbf{x}_2 = f(W^{rec}\mathbf{x}_1 + W^{in}u_1 + \mathbf{b})$$
$$\mathbf{x}_3 = f(W^{rec}\mathbf{x}_2 + W^{in}u_2 + \mathbf{b})$$

(b) Suppose the loss is only at the final step: $L = (y_3 - \hat{y}_3)^2$, with $y_3 = \mathbf{w}^T \mathbf{x}_3$. Write $\partial L / \partial \mathbf{x}_3$.

(c) Compute $\partial L / \partial \mathbf{x}_2$ using the chain rule. Write out the $2 \times 2$ Jacobian $\partial \mathbf{x}_3 / \partial \mathbf{x}_2 = D_3 W^{rec}$ explicitly.

(d) Compute $\partial L / \partial \mathbf{x}_1$ and $\partial L / \partial \mathbf{x}_0$ by continuing the backward pass.

(e) Write out $\partial L / \partial W^{rec}$ as a sum of three outer products. Identify which term corresponds to the gradient "through" the longest chain of time steps.

(f) What happens to the term $\partial L / \partial \mathbf{x}_0$ if $\rho(W^{rec}) = 0.5$? If $\rho(W^{rec}) = 1.5$?

---

**Exercise 3.2 — Gradient Norm Analysis**

Let $J = DW$ where $D = \text{diag}(d_1, \ldots, d_N)$ with $d_i \in [0, 1]$ and $W \in \mathbb{R}^{N \times N}$.

(a) Show that $\|J\|_2 \leq \|D\|_2 \|W\|_2 = \max_i(d_i) \cdot \|W\|_2$.

(b) Show that if all $d_i = 1$ and $W$ is orthogonal, then $\|J^n\|_2 = 1$ for all $n$.

(c) Now suppose $D = \text{diag}(0.9, 0.9, \ldots, 0.9)$ (all entries equal to 0.9) and $W$ is an orthogonal matrix. What is $\|J^n\|_2$? At what value of $n$ does the gradient become smaller than $10^{-4}$?

(d) In part (c), what is the **effective memory** $\tau_{\text{eff}} = -1/\ln(0.9)$? Interpret this in words.

(e) What property of the nonlinearity $\tanh$ makes all $d_i \leq 1$? (Hint: compute $\tanh'(x)$ and find its maximum.) How does this change if we use $\text{ReLU}$ instead of $\tanh$?

---

**Exercise 3.3 — Spectral Radius and Stability**

Consider the linear RNN $\mathbf{x}_{t+1} = W^{rec}\mathbf{x}_t + W^{in}\mathbf{u}_t$ (no nonlinearity).

(a) Show that if $\rho(W^{rec}) < 1$, then the autonomous system $\mathbf{x}_{t+1} = W^{rec}\mathbf{x}_t$ converges to $\mathbf{0}$ for any initial condition.

(b) Show that the state at time $t$ can be written as:

$$\mathbf{x}_t = (W^{rec})^t \mathbf{x}_0 + \sum_{s=0}^{t-1} (W^{rec})^{t-1-s} W^{in} \mathbf{u}_s$$

(c) What does the term $(W^{rec})^{t-1-s} W^{in} \mathbf{u}_s$ represent? What is its norm bounded by, as a function of $s$?

(d) The condition $\rho(W^{rec}) < 1$ is called the **echo state property** (Chapter 5). Explain in words why this name is apt.

(e) Suppose $W^{rec}$ has spectral radius exactly 1 (at least one eigenvalue $|\lambda| = 1$). Does the linear RNN have fading memory? Is the echo state property satisfied? What does this mean for the gradient at the corresponding eigendirection?

---

**Exercise 3.4 — LSTM Gate Analysis**

The LSTM cell state update is:

$$\mathbf{c}_t = \mathbf{f}_t \odot \mathbf{c}_{t-1} + \mathbf{i}_t \odot \tilde{\mathbf{c}}_t$$

where $\mathbf{f}_t = \sigma(W_f [\mathbf{h}_{t-1}; \mathbf{u}_t] + \mathbf{b}_f)$ is the forget gate.

(a) Show that the partial derivative of $c_t^{(i)}$ (the $i$-th component of the cell state) with respect to $c_{t-1}^{(i)}$ is $f_t^{(i)}$.

(b) Therefore, the product $\prod_{k=s+1}^{t} \partial c_k^{(i)} / \partial c_{k-1}^{(i)} = \prod_{k=s+1}^{t} f_k^{(i)}$. If all forget gate values equal $f$, what is this product?

(c) For what value of $f$ do gradients through the cell state neither vanish nor explode? What does this mean for the forget gate's "ideal" setting for long-range memory?

(d) However, the forget gate $\mathbf{f}_t$ is itself computed from $\mathbf{h}_{t-1}$ via $W_f$. Does this mean there is still a vanishing gradient problem through $\mathbf{f}_t$? Explain.

(e) Hochreiter and Schmidhuber called the pathway through the cell state the "constant error carousel." Why? What is the error in this name (i.e., when is the carousel *not* constant)?

---

**Exercise 3.5 — The Addition Problem**

The **addition problem** [Hochreiter1997] is a benchmark for long-range memory in RNNs. The input is a sequence of pairs $(v_t, m_t)$ where $v_t \in [0, 1]$ is a value and $m_t \in \{0, 1\}$ is a marker. Exactly two steps have $m_t = 1$; the target is the sum of the values at those two steps.

(a) What is the minimum temporal distance between the two markers in a sequence of length $T$? What does this imply about the memory required?

(b) Show that a network that outputs 1.0 always (regardless of input) achieves an expected mean squared error of approximately 0.1767 for uniform $v_t \in [0, 1]$. (This is the "trivial baseline.")

(c) For a vanilla RNN with $\rho(W^{rec}) = 0.9$, what effective memory does it have? For what maximum sequence length $T$ would you expect it to solve the addition problem?

(d) Why is the addition problem a particularly good test of long-range memory (rather than, say, pattern classification)?

---

**Exercise 3.6 — Teacher Forcing**

In training RNNs for sequence prediction, **teacher forcing** feeds the ground-truth output $\hat{\mathbf{y}}_{t-1}$ as the input at step $t$ instead of the network's own prediction $\mathbf{y}_{t-1}$.

(a) What problem does teacher forcing solve during training?

(b) What new problem does teacher forcing create at test time? (Hint: at test time, $\hat{\mathbf{y}}_{t-1}$ is not available.)

(c) This is called the **exposure bias** problem. Propose a partial solution.

(d) Does teacher forcing affect the vanishing gradient problem? Explain why or why not.

(e) In reservoir computing, teacher forcing takes on a specific technical meaning in FORCE learning (Chapter 11). Anticipate: how might a reservoir computer use something like teacher forcing?

---

## Thought Experiments

**Thought Experiment 3.1 — Why Doesn't Biological Learning Use BPTT?**

Backpropagation through time requires:
- Storing the entire forward trajectory in memory ("activations at every step").
- Running a backward pass through time, in reverse.
- Propagating error signals through the transpose of the weight matrix $(W^{rec})^T$.

Consider whether the brain could plausibly implement any of these requirements.

(a) Does the brain have memory of past neural activations sufficient to store a forward trajectory? What would "synaptic tags" for past activity look like, and do such mechanisms exist in neuroscience?

(b) The backward pass in BPTT requires a "second network" with transposed weights $(W^{rec})^T$. Is there any neural mechanism that computes with transposed connectivity? What would this require anatomically?

(c) BPTT requires exact error signals propagated backward through time — the network must know, at time $t$, what its error will be at time $T > t$. What does this require about the timing of biological learning?

(d) Given these difficulties, propose two biologically plausible alternatives to BPTT for temporal credit assignment. Which of them is closest to what we know about synaptic plasticity?

(e) Would a reservoir computer make biological learning more plausible? If the recurrent weights are not trained (only the readout is), which of the BPTT requirements disappear?

---

**Thought Experiment 3.2 — Is RTRL Biologically Plausible?**

Real-Time Recurrent Learning (RTRL) [Williams1989] computes gradients online, without unrolling. At each time step, it maintains the matrix $\partial \mathbf{x}_t / \partial w_{ij}$ for every weight $w_{ij}$ in $W^{rec}$. The update for RTRL is:

$$\frac{\partial \mathbf{x}_t}{\partial w_{ij}} = D_t W^{rec} \frac{\partial \mathbf{x}_{t-1}}{\partial w_{ij}} + D_t \mathbf{e}_{ij} x_{t-1}^{(j)}$$

where $\mathbf{e}_{ij}$ is the unit vector in the $ij$ direction.

(a) RTRL maintains the $N \times N^2$ matrix $\partial \mathbf{x}_t / \partial W^{rec}$ at every time step. For $N = 1000$, how many numbers is this?

(b) Is RTRL local in time? (I.e., does the update at time $t$ require information from more than one step ago?) How does this compare to BPTT?

(c) Is RTRL local in space? That is: can neuron $i$ compute its update $\partial x_t^{(i)} / \partial w_{ij}$ using only information available at the synapse from $j$ to $i$? Inspect the update equation and determine what non-local information is required.

(d) Propose a "local approximation" to RTRL that sacrifices some accuracy for biological plausibility. (This is an active research area; there is no single right answer.)

(e) The gradient computed by RTRL is exact; the gradient computed by BPTT is also exact (given the same loss function and sequence). Yet they produce different computational patterns. At what step do their updates diverge, and why?

---

**Thought Experiment 3.3 — The Minimal Memory Machine**

You want to build the simplest possible machine that can solve the following task: given an input sequence of binary values $u_1, u_2, \ldots, u_T$, output 1 if and only if there was exactly one 1 in the entire sequence.

(a) Show that a feedforward network with any fixed input dimension $n < T$ cannot solve this task for all $T$.

(b) Can a one-neuron RNN (i.e., $N = 1$, scalar state) solve this task? Write out the state update equation and the output function explicitly, if so. If not, prove it.

(c) What is the minimum $N$ for which an RNN can solve this task for sequences of unbounded length? (Hint: think about what state the machine needs to track.)

(d) Can a reservoir computer solve this task? What properties must the reservoir have? Is the task solvable with a linear readout?

(e) Now modify the task: output 1 if and only if the number of 1s is divisible by 3. Does this change your answer to (c)? To (d)?

---

## Lab Exercises

**Lab 3.1 — Implement BPTT from Scratch**

Implement a minimal RNN with BPTT, without using any automatic differentiation library. Use only NumPy.

```python
import numpy as np

class VanillaRNN:
    """
    Minimal RNN with manual BPTT.
    
    Architecture:
      x_{t+1} = tanh(W_rec @ x_t + W_in @ u_t + b)
      y_t     = W_out @ x_t
    
    Parameters
    ----------
    N : int
        Number of hidden units.
    K : int
        Input dimension.
    M : int
        Output dimension.
    """
    
    def __init__(self, N: int, K: int, M: int, seed: int = 42):
        rng = np.random.default_rng(seed)
        self.N = N
        # Initialize weights (Xavier-style)
        self.W_rec = rng.normal(0, 1.0 / np.sqrt(N), (N, N))
        self.W_in  = rng.normal(0, 1.0 / np.sqrt(K), (N, K))
        self.W_out = rng.normal(0, 1.0 / np.sqrt(N), (M, N))
        self.b     = np.zeros(N)
    
    def forward(self, U: np.ndarray) -> tuple:
        """
        Forward pass.
        
        Parameters
        ----------
        U : np.ndarray, shape (T, K)
            Input sequence.
        
        Returns
        -------
        states : np.ndarray, shape (T+1, N)
            State trajectory (states[0] is x_0 = 0).
        preacts : np.ndarray, shape (T, N)
            Pre-activation values a_t = W_rec @ x_{t-1} + W_in @ u_{t-1} + b.
        outputs : np.ndarray, shape (T, M)
            Output sequence.
        """
        T, K = U.shape
        states  = np.zeros((T + 1, self.N))
        preacts = np.zeros((T, self.N))
        outputs = np.zeros((T, self.W_out.shape[0]))
        
        # Forward pass implementation.
        for t in range(T):
            preacts[t]     = self.W_rec @ states[t] + self.W_in @ U[t] + self.b
            states[t + 1]  = np.tanh(preacts[t])
            outputs[t]     = self.W_out @ states[t + 1]
        # Explanation: at each step t we compute the pre-activation a_t = W_rec x_t + W_in u_t + b,
        # apply tanh elementwise to obtain the new hidden state x_{t+1}, and project to the output.
        # states[0] is the zero initial condition; states[t+1] corresponds to time step t+1.
        return states, preacts, outputs
    
    def backward(
        self,
        U: np.ndarray,
        states: np.ndarray,
        preacts: np.ndarray,
        outputs: np.ndarray,
        targets: np.ndarray
    ) -> dict:
        """
        Backward pass: BPTT.
        
        Parameters
        ----------
        U       : (T, K)   input sequence
        states  : (T+1, N) state trajectory from forward()
        preacts : (T, N)   pre-activations from forward()
        outputs : (T, M)   outputs from forward()
        targets : (T, M)   target outputs
        
        Returns
        -------
        grads : dict with keys 'W_rec', 'W_in', 'W_out', 'b'
        """
        T = U.shape[0]
        
        dW_rec = np.zeros_like(self.W_rec)
        dW_in  = np.zeros_like(self.W_in)
        dW_out = np.zeros_like(self.W_out)
        db     = np.zeros_like(self.b)
        
        # Gradient of loss w.r.t. outputs: dL/dy_t = 2*(y_t - y_hat_t) / T
        dL_dy = 2.0 * (outputs - targets) / T    # shape (T, M)
        
        # Gradient of loss w.r.t. W_out: sum_t (dL/dy_t) x_{t+1}^T
        # Each time step contributes an outer product of the output error with the hidden state.
        dW_out = dL_dy.T @ states[1:]   # shape (M, N): sum over T time steps
        
        # Backward pass through time
        delta = np.zeros(self.N)  # accumulated error signal flowing backward through time
        
        for t in reversed(range(T)):
            # 1. Add the direct gradient from the loss at step t.
            #    dL/dx_{t+1} via the output: W_out.T maps M-dim output error to N-dim state space.
            delta += self.W_out.T @ dL_dy[t]
            
            # 2. tanh derivative evaluated at the stored post-activation (equivalent to 1 - tanh^2).
            tanh_deriv = 1.0 - states[t + 1] ** 2    # shape (N,)
            
            # 3. Gate the error by the tanh derivative to get the pre-activation gradient.
            delta_pre = tanh_deriv * delta             # shape (N,) — elementwise
            
            # 4-6. Accumulate parameter gradients using the pre-activation gradient.
            dW_rec += np.outer(delta_pre, states[t])   # outer product: shape (N, N)
            dW_in  += np.outer(delta_pre, U[t])        # outer product: shape (N, K)
            db     += delta_pre                         # bias gradient: shape (N,)
            
            # 7. Propagate the error one step further back through W_rec^T.
            #    This is the "through-time" step that can cause vanishing/exploding gradients.
            delta = self.W_rec.T @ delta_pre
        
        return {'W_rec': dW_rec, 'W_in': dW_in, 'W_out': dW_out, 'b': db}


# Test: verify gradients numerically with finite differences.
def numerical_gradient(rnn, U, targets, param_name, eps=1e-5):
    """Compute numerical gradient for a parameter by finite differences."""
    param = getattr(rnn, param_name)
    grad  = np.zeros_like(param)
    
    def loss(rnn):
        _, _, outputs = rnn.forward(U)
        return np.mean((outputs - targets)**2)
    
    for idx in np.ndindex(param.shape):
        param[idx] += eps
        L_plus = loss(rnn)
        param[idx] -= 2 * eps
        L_minus = loss(rnn)
        param[idx] += eps  # restore
        grad[idx] = (L_plus - L_minus) / (2 * eps)
    
    return grad


if __name__ == "__main__":
    N, K, M, T = 4, 2, 1, 6
    rng = np.random.default_rng(0)
    U       = rng.normal(0, 1, (T, K))
    targets = rng.normal(0, 1, (T, M))
    
    rnn = VanillaRNN(N=N, K=K, M=M)
    states, preacts, outputs = rnn.forward(U)
    grads = rnn.backward(U, states, preacts, outputs, targets)
    
    for pname in ['W_rec', 'W_in', 'W_out']:
        num_grad  = numerical_gradient(rnn, U, targets, pname)
        ana_grad  = grads[pname]
        rel_error = np.max(np.abs(num_grad - ana_grad)) / (np.max(np.abs(num_grad)) + 1e-8)
        print(f"{pname}: relative error = {rel_error:.2e}  (should be < 1e-4)")
```

**Tasks:**
1. Implement the `forward` and `backward` methods.
2. Run the gradient check. Report the relative errors.
3. Train the RNN on a simple sequence task (e.g., predict the next element of a sine wave). Plot the training loss.

---

**Lab 3.2 — Measuring Gradient Norms at Depth**

Investigate how gradient norms decay (or explode) as a function of sequence length and spectral radius.

```python
import numpy as np
import matplotlib.pyplot as plt

def measure_gradient_norms(N=50, T=200, spectral_radii=[0.5, 0.9, 0.99, 1.01, 1.1]):
    """
    For each spectral radius, measure how gradient norms decay with distance from the loss.
    
    Returns
    -------
    results : dict mapping spectral_radius -> array of gradient norms of shape (T,)
    """
    results = {}
    
    for rho in spectral_radii:
        # Construct W_rec with the given spectral radius
        W = np.random.randn(N, N)
        W = W / np.max(np.abs(np.linalg.eigvals(W))) * rho   # scale to desired rho
        
        # Simulate a random trajectory
        x = np.zeros(N)
        states = [x.copy()]
        preacts = []
        
        for t in range(T):
            u_t   = np.random.randn(N)  # random "input" (W_in = I for simplicity)
            a_t   = W @ x + u_t
            x     = np.tanh(a_t)
            states.append(x.copy())
            preacts.append(a_t.copy())
        
        # Compute gradient norms: ||dx_T / dx_s|| for s = 0, ..., T-1
        # Start from s = T-1 and propagate backward.
        
        # Initialize: dx_T / dx_T = I
        J_prod = np.eye(N)
        grad_norms = np.zeros(T)
        grad_norms[T-1] = 1.0  # norm of identity
        
        # Propagate J_prod backward: J_prod accumulates the product of Jacobians
        # from time T back to time s.  Each factor J_s = D_s W where D_s is the
        # diagonal matrix of tanh derivatives at time step s.
        for s in reversed(range(T - 1)):
            D_s    = np.diag(1.0 - states[s + 1] ** 2)   # tanh'(a_s): shape (N, N) diagonal
            J_s    = D_s @ W                               # Jacobian at step s: shape (N, N)
            J_prod = J_prod @ J_s                          # accumulate: J_T->s = J_T->s+1 * J_s
            grad_norms[s] = np.linalg.norm(J_prod, ord=2) # spectral norm of the accumulated product
        # Interpretation: grad_norms[s] = ||∂x_T / ∂x_s||_2.
        # For rho < 1 this decays exponentially with (T-s); for rho > 1 it grows exponentially.
        results[rho] = grad_norms
    
    return results


# Visualization
def plot_gradient_norms(results):
    fig, axes = plt.subplots(1, 2, figsize=(12, 5))
    
    T = len(next(iter(results.values())))
    
    for rho, norms in results.items():
        axes[0].plot(range(T), norms, label=f'ρ = {rho}')
        axes[1].plot(range(T), np.log10(norms + 1e-30), label=f'ρ = {rho}')
    
    axes[0].set_xlabel('Time step s (distance from loss)')
    axes[0].set_ylabel('||∂x_T / ∂x_s||')
    axes[0].set_title('Gradient Norms (linear scale)')
    axes[0].legend()
    
    axes[1].set_xlabel('Time step s')
    axes[1].set_ylabel('log₁₀ ||∂x_T / ∂x_s||')
    axes[1].set_title('Gradient Norms (log scale)')
    axes[1].legend()
    
    plt.tight_layout()
    plt.savefig('gradient_norms.png', dpi=150)
    plt.show()
```

**Tasks:**
1. Implement the Jacobian product accumulation.
2. Plot the gradient norms for all spectral radii. What do you observe for $\rho = 0.5$ vs $\rho = 0.99$ vs $\rho = 1.1$?
3. On the log-scale plot, verify that the decay is linear (i.e., exponential decay in linear scale). Measure the empirical decay rate and compare to the theoretical prediction $\rho^{T-s}$.
4. For $\rho = 0.99$, estimate the effective memory $\tau_{\text{eff}}$ from the plot and from the formula $\tau_{\text{eff}} = -1/\ln(0.99)$.

---

**Lab 3.3 — Train on the Addition Problem vs. Sequence Length**

Implement the addition problem and test a vanilla RNN's ability to solve it at different sequence lengths.

```python
import numpy as np

def generate_addition_problem(T: int, batch_size: int, rng=None) -> tuple:
    """
    Generate batch of addition problem instances.
    
    Each instance has:
      - values:  T floats in [0, 1]
      - markers: T binary values, exactly 2 of which are 1
      - target:  sum of values at marked positions
    
    Input to network: (values, markers) concatenated -> shape (T, 2)
    Target: scalar sum
    
    Returns
    -------
    X      : np.ndarray, shape (batch_size, T, 2)
    y      : np.ndarray, shape (batch_size,)
    """
    if rng is None:
        rng = np.random.default_rng(0)
    
    X = np.zeros((batch_size, T, 2))
    y = np.zeros(batch_size)
    
    # Values: uniform [0, 1]
    X[:, :, 0] = rng.uniform(0, 1, (batch_size, T))
    
    # Markers: exactly 2 per sequence, placed randomly
    for i in range(batch_size):
        # First marker in first half, second in second half (standard version)
        p1 = rng.integers(0, T // 2)
        p2 = rng.integers(T // 2, T)
        X[i, p1, 1] = 1.0
        X[i, p2, 1] = 1.0
        y[i] = X[i, p1, 0] + X[i, p2, 0]
    
    return X, y


def train_rnn_addition(T: int, N: int = 50, n_epochs: int = 2000, lr: float = 1e-3):
    """
    Train a vanilla RNN on the addition problem with sequence length T.
    
    Returns
    -------
    final_mse : float   Final test MSE
    losses    : list    Training loss per epoch
    """
    rng = np.random.default_rng(0)
    
    # 1. Generate training (256 examples) and test (512 examples) batches.
    X_train, y_train = generate_addition_problem(T, batch_size=256, rng=rng)
    X_test,  y_test  = generate_addition_problem(T, batch_size=512, rng=rng)
    # Reshape targets to (batch, T, 1) — loss only at final time step.
    # We train with teacher forcing at all steps but only care about the final prediction.
    # Simplest approach: use the final state prediction as the output.
    # Targets: y_train is shape (batch,); we predict the sum at the last step.
    
    # 2. Train VanillaRNN (from Lab 3.1) using SGD + BPTT.
    rnn = VanillaRNN(N=N, K=2, M=1)  # K=2 (value+marker), M=1 (scalar output)
    losses = []
    
    for epoch in range(n_epochs):
        epoch_loss = 0.0
        # Mini-batch gradient descent (one batch per epoch for simplicity).
        batch_dW_rec = np.zeros_like(rnn.W_rec)
        batch_dW_in  = np.zeros_like(rnn.W_in)
        batch_dW_out = np.zeros_like(rnn.W_out)
        batch_db     = np.zeros_like(rnn.b)
        
        for i in range(len(X_train)):
            U_i = X_train[i]                       # (T, 2)
            # Target: zeros for all steps except the last, which holds the sum.
            tgt_i = np.zeros((T, 1))
            tgt_i[-1, 0] = y_train[i]
            
            states, preacts, outputs = rnn.forward(U_i)
            grads = rnn.backward(U_i, states, preacts, outputs, tgt_i)
            
            # Accumulate gradients.
            batch_dW_rec += grads['W_rec']
            batch_dW_in  += grads['W_in']
            batch_dW_out += grads['W_out']
            batch_db     += grads['b']
            epoch_loss   += np.mean((outputs[-1] - tgt_i[-1]) ** 2)
        
        # Average gradient and apply SGD update with gradient clipping.
        n = len(X_train)
        for grad in [batch_dW_rec, batch_dW_in, batch_dW_out, batch_db]:
            np.clip(grad / n, -1.0, 1.0, out=grad)  # gradient clipping to [-1, 1]
        rnn.W_rec -= lr * batch_dW_rec / n
        rnn.W_in  -= lr * batch_dW_in  / n
        rnn.W_out -= lr * batch_dW_out / n
        rnn.b     -= lr * batch_db     / n
        losses.append(epoch_loss / n)
    
    # 3. Evaluate on test set.
    test_mse = 0.0
    for i in range(len(X_test)):
        tgt_i = np.zeros((T, 1)); tgt_i[-1, 0] = y_test[i]
        _, _, outputs = rnn.forward(X_test[i])
        test_mse += (outputs[-1, 0] - y_test[i]) ** 2
    final_mse = test_mse / len(X_test)
    
    return final_mse, losses


# Experiment: sweep over sequence lengths
if __name__ == "__main__":
    import matplotlib.pyplot as plt
    
    T_values = [10, 20, 50, 100, 200, 500]
    N = 50
    
    results = {}
    baseline_mse = 0.1767  # trivial baseline (always predict 1.0)
    
    for T in T_values:
        print(f"Training on T={T}...")
        final_mse, losses = train_rnn_addition(T=T, N=N)
        results[T] = (final_mse, losses)
        print(f"  T={T}: final MSE = {final_mse:.4f} (baseline = {baseline_mse:.4f})")
    
    # Plot final MSE vs sequence length
    T_vals  = list(results.keys())
    mse_vals = [v[0] for v in results.values()]
    
    plt.figure(figsize=(8, 5))
    plt.plot(T_vals, mse_vals, 'o-', label='Vanilla RNN')
    plt.axhline(baseline_mse, color='r', linestyle='--', label='Trivial baseline')
    plt.xlabel('Sequence length T')
    plt.ylabel('Final test MSE')
    plt.title('Addition Problem: RNN performance vs. sequence length')
    plt.legend()
    plt.xscale('log')
    plt.savefig('addition_problem.png', dpi=150)
    plt.show()
```

**Tasks:**
1. Implement `train_rnn_addition` using the `VanillaRNN` from Lab 3.1.
2. Plot final test MSE vs. sequence length $T$.
3. At what sequence length does the vanilla RNN fail to beat the baseline?
4. Replace the vanilla RNN with a simple LSTM (you may use PyTorch's `nn.LSTM`). How much further can it go before failing?
5. Interpret your results in terms of the effective memory $\tau_{\text{eff}}$ from Exercise 3.2.

---

## Programming Projects

**Project 3.A — A Comparative Study: Gradient Flow Architectures**

Implement vanilla RNN, LSTM, and GRU from scratch (manual BPTT, no autograd) and conduct a systematic comparison on three benchmark tasks:

1. **Copying task:** The network must output the first $K$ elements of a length-$T$ sequence, after seeing $T - K$ zeros.
2. **Addition problem** (Lab 3.3).
3. **Sequence classification:** Classify whether a sequence contains more 1s than 0s (where inputs are binary and the sequence length varies from 20 to 500).

For each architecture and task:
- Plot training curves.
- Measure gradient norms at different depths (as in Lab 3.2).
- Plot the "effective memory" as inferred from gradient norms.
- Tabulate final test accuracy/MSE and training time.

Write a one-page analysis discussing: (a) why LSTM outperforms vanilla RNN on long-range tasks, (b) whether GRU's fewer parameters give it an advantage in sample efficiency, and (c) what the gradient norm plots reveal about each architecture's memory mechanism.

---

**Project 3.B — The Reservoir Baseline**

Implement a simple reservoir computer (anticipating Chapter 4) and compare it to a trained vanilla RNN and LSTM on the addition problem.

A minimal reservoir computer:
1. Constructs a random $W^{rec}$ with $\rho(W^{rec}) = 0.9$ (fixed, not trained).
2. Constructs a random $W^{in}$ (fixed, not trained).
3. Runs the state update $\mathbf{x}_{t+1} = \tanh(W^{rec}\mathbf{x}_t + W^{in}\mathbf{u}_t)$ on the training sequence, collecting all states.
4. Fits a linear readout $\mathbf{w}$ by least squares on the final state $\mathbf{x}_T$.

For several sequence lengths $T \in \{20, 50, 100, 200, 500\}$:
- Compare the test MSE of the reservoir computer vs. vanilla RNN vs. LSTM.
- Measure the time required to train each.
- Plot test MSE vs. training time (a "Pareto plot" of performance vs. cost).

Write a two-page discussion: (a) What is surprising about the reservoir's performance? (b) What does the comparison reveal about what BPTT actually learns? (c) For what sequence lengths does the reservoir fail, and why? (d) What does this suggest about when the full cost of training RNNs is justified?

---

## References for Exercises

- [Hochreiter1997] Hochreiter, S., & Schmidhuber, J. (1997). Long short-term memory. *Neural Computation*, 9(8), 1735–1780.
- [Williams1989] Williams, R. J., & Zipser, D. (1989). A learning algorithm for continually running fully recurrent neural networks. *Neural Computation*, 1(2), 270–280.
- [Bengio1994] Bengio, Y., Simard, P., & Frasconi, P. (1994). Learning long-term dependencies with gradient descent is difficult. *IEEE Transactions on Neural Networks*, 5(2), 157–166.
- [Pascanu2013] Pascanu, R., Mikolov, T., & Bengio, Y. (2013). On the difficulty of training recurrent neural networks. *Proceedings of ICML*, 1310–1318.

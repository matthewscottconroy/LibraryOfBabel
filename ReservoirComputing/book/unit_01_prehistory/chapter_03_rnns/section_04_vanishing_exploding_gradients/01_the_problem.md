# 3.4.1 The Vanishing and Exploding Gradient Problem

## The Product of Jacobians

At the heart of BPTT is the product of local Jacobians:

$$\frac{\partial \mathbf{x}_t}{\partial \mathbf{x}_s} = \prod_{k=s+1}^{t} J_k = \prod_{k=s+1}^{t} D_k W^{rec}$$

where $J_k = D_k W^{rec}$ is the Jacobian of the state update at step $k$, $D_k = \text{diag}(f'(\mathbf{a}_k))$ is the diagonal matrix of nonlinearity derivatives, and $W^{rec}$ is the (fixed) recurrent weight matrix.

This product appears in every gradient computation. When $t - s$ is large — when we are propagating gradients across many time steps — we are computing a product of many matrices. The behavior of matrix products at large depth is the central issue of gradient-based training in RNNs.

Let us analyze this product systematically.

## The Norm Bound

The key result is a bound on the norm of the product. For any induced matrix norm, the submultiplicative property gives:

$$\left\|\prod_{k=s+1}^{t} J_k\right\| \leq \prod_{k=s+1}^{t} \|J_k\|$$

Now $\|J_k\| = \|D_k W^{rec}\|$. Since $D_k$ is a diagonal matrix with entries $f'(\mathbf{a}_k^{(i)}) \in [0, 1]$ for $f = \tanh$ (because $\tanh' \in (0, 1]$), we have:

$$\|D_k W^{rec}\| \leq \|D_k\| \cdot \|W^{rec}\| \leq 1 \cdot \|W^{rec}\| = \|W^{rec}\|$$

Therefore:

$$\left\|\prod_{k=s+1}^{t} J_k\right\| \leq \|W^{rec}\|^{t-s}$$

This bound tells us that if $\|W^{rec}\| < 1$, then the product shrinks exponentially with the time horizon $t - s$. The gradient $\partial L_t / \partial \mathbf{x}_s$ shrinks to zero at a rate proportional to $\|W^{rec}\|^{t-s}$.

But the bound is an upper bound. We also need to think about when products grow.

## The Spectral Radius Condition

Let $\rho(A)$ denote the spectral radius of a matrix $A$ — the largest absolute value of its eigenvalues:

$$\rho(A) = \max_i |\lambda_i(A)|$$

The spectral radius is the "natural size" of a matrix for the purposes of asymptotic power growth: $\|A^n\|$ grows like $\rho(A)^n$ as $n \to \infty$ (for generic matrices).

**Theorem [Bengio1994, Pascanu2013]:** Consider the RNN state update with $f = \tanh$ and suppose the pre-activations $\mathbf{a}_k$ are roughly constant across time steps (a simplifying assumption). Let $\bar{D}$ denote the average diagonal matrix of nonlinearity derivatives. Then the product of Jacobians satisfies:

$$\left\|\frac{\partial \mathbf{x}_t}{\partial \mathbf{x}_s}\right\| \approx \rho(\bar{D} W^{rec})^{t-s}$$

The critical threshold is $\rho(\bar{D} W^{rec}) = 1$:

- If $\rho(\bar{D} W^{rec}) < 1$: gradients **vanish** exponentially as $t - s \to \infty$.
- If $\rho(\bar{D} W^{rec}) > 1$: gradients **explode** exponentially as $t - s \to \infty$.
- If $\rho(\bar{D} W^{rec}) = 1$: gradients are (marginally) stable.

Since $\bar{D}$ has diagonal entries in $(0, 1]$, and $\rho(\bar{D} W^{rec}) \leq \rho(W^{rec}) \max_i |D_{ii}| \leq \rho(W^{rec})$, the spectral radius of $W^{rec}$ provides an upper bound: a sufficient condition for vanishing gradients is $\rho(W^{rec}) < 1$.

## Proof of the Exponential Bound

Let us prove the exponential decay rigorously under simplified assumptions. Assume:
1. The Jacobians $J_k = J$ are constant (the network operates near a fixed point).
2. $J$ is a normal matrix (so $\|J^n\|_2 = \rho(J)^n$ exactly).

Then:

$$\left\|\frac{\partial \mathbf{x}_t}{\partial \mathbf{x}_s}\right\|_2 = \|J^{t-s}\|_2 = \rho(J)^{t-s}$$

If $\rho(J) < 1$: for a distance of $n = t - s$ time steps,

$$\left\|\frac{\partial \mathbf{x}_t}{\partial \mathbf{x}_s}\right\|_2 = \rho(J)^n \xrightarrow{n \to \infty} 0$$

exponentially fast. For example, if $\rho(J) = 0.9$ and $n = 100$:

$$\rho(J)^n = 0.9^{100} = e^{100 \ln 0.9} = e^{-10.5} \approx 2.7 \times 10^{-5}$$

The gradient has been attenuated by a factor of $27{,}000$. At $n = 200$: $\rho(J)^{200} \approx 7.2 \times 10^{-10}$. The gradient has essentially vanished in floating-point arithmetic.

If $\rho(J) > 1$: for $n = 100$ and $\rho(J) = 1.01$:

$$\rho(J)^n = 1.01^{100} = e^{100 \ln 1.01} \approx e^{0.995} \approx 2.70$$

Already by step 100, the gradient has more than doubled. At $n = 1000$: $1.01^{1000} \approx 21916$. Gradients overflow.

## The Dilemma

Notice what this means. If the network is stable — if it forgets its initial conditions, which is exactly the property we want from a memory system that doesn't remember forever — then the spectral radius satisfies $\rho(J) < 1$, and gradients vanish. But if $\rho(J) > 1$, the network is unstable (perturbations grow), and gradients explode.

The region where neither happens — where $\rho(J) \approx 1$ — is a knife-edge. It is both extremely difficult to achieve and maintain during training, and numerically delicate.

**This is the fundamental dilemma of gradient-based training of RNNs:** the same spectral condition that governs gradient propagation also governs dynamical stability. Stable dynamics imply vanishing gradients. Unstable dynamics imply exploding gradients. Long-range memory requires dynamics near the boundary — which is exactly where training is most sensitive.

## What Vanishing Gradients Mean for Learning

When gradients vanish, the parameter update for $W^{rec}$ is dominated by the most recent time steps:

$$\frac{\partial L}{\partial W^{rec}} = \sum_{t=1}^{T} \sum_{s=1}^{t} D_s \boldsymbol{\delta}_s^{(t)} \mathbf{x}_{s-1}^T$$

The terms with $t - s$ large are multiplied by $\prod_{k=s+1}^{t} D_k W^{rec}$, which is exponentially small. So the gradient is effectively:

$$\frac{\partial L}{\partial W^{rec}} \approx \sum_{t=1}^{T} \sum_{s=t-\tau}^{t} D_s \boldsymbol{\delta}_s^{(t)} \mathbf{x}_{s-1}^T$$

for some effective truncation time $\tau$ — determined not by us, but by the spectral radius of $J$. The effective memory of the gradient is:

$$\tau_{\text{eff}} \approx -\frac{1}{\ln \rho(J)}$$

For $\rho(J) = 0.9$: $\tau_{\text{eff}} \approx 9.5$ steps. For $\rho(J) = 0.99$: $\tau_{\text{eff}} \approx 99.5$ steps.

This means the network can only learn dependencies within a window of size roughly $\tau_{\text{eff}}$ — regardless of the length of the training sequences. A network with $\rho(J) = 0.9$ presented with sequences of length 1000 will only be able to learn dependencies up to 10 steps back.

## Empirical Evidence: Gradients at Depth 100

To make this concrete, let us describe what happens in practice. Consider a vanilla RNN with $N = 50$ units, $\tanh$ nonlinearity, and $W^{rec}$ drawn from a Gaussian with variance $\sigma^2/N$. For $\sigma < 1$ (sub-critical regime), the spectral radius is approximately $\sigma$.

After running a forward pass on a sequence of length 100, BPTT produces gradients $\partial L / \partial \mathbf{x}_s$ for $s = 0, 1, \ldots, 99$. If we plot $\|\partial L / \partial \mathbf{x}_s\|$ as a function of $s$ (the time distance from the loss), we observe:

- For $\sigma = 0.5$ ($\rho \approx 0.5$): the gradient norm decays as $\approx 0.5^{100-s}$. By $s = 90$ (10 steps from the loss), the gradient is already $\approx 0.001$ of its value at $s = 99$. By $s = 0$ (100 steps from the loss), the gradient is numerically zero in double precision.
  
- For $\sigma = 0.9$ ($\rho \approx 0.9$): the gradient decays more slowly, but by $s = 0$ it has decayed by a factor of $\approx 0.9^{100} \approx 2.6 \times 10^{-5}$.
  
- For $\sigma = 1.1$ ($\rho \approx 1.1$): the gradient norms fluctuate wildly and can be $10^{10}$ or larger at $s = 0$, causing numerical overflow in 32-bit floating point.

The "long-term credit assignment problem" is precisely this: the gradient cannot propagate information about the loss backwards through more than a few tens of time steps. The network cannot learn "this output was wrong because of what the input said 80 steps ago."

## Why LSTM Helps (and Why It Doesn't Fully Solve It)

The Long Short-Term Memory network [Hochreiter1997] introduces **gating mechanisms** that create pathways where gradients can flow without passing through the repeated application of $W^{rec}$. The cell state $\mathbf{c}_t$ satisfies an update of the form:

$$\mathbf{c}_t = \mathbf{f}_t \odot \mathbf{c}_{t-1} + \mathbf{i}_t \odot \tilde{\mathbf{c}}_t$$

where $\mathbf{f}_t \in (0, 1)^N$ is the **forget gate** and $\odot$ denotes element-wise multiplication. The gradient of the cell state through time is:

$$\frac{\partial \mathbf{c}_t}{\partial \mathbf{c}_{t-1}} = \text{diag}(\mathbf{f}_t)$$

If the forget gate is close to 1 (i.e., $\mathbf{f}_t \approx \mathbf{1}$), then the gradient through the cell state is approximately $I$ — identity. Gradients flow backward without attenuation. This is the **constant error carousel** [Hochreiter1997], which provides a "highway" for gradient flow.

However, the gates themselves are functions of the state and input, and their gradients still flow through $W^{rec}$. LSTM does not eliminate the vanishing gradient problem; it creates bypasses around the worst of it. For very long sequences or tasks requiring very long-range memory, even LSTM eventually struggles.

## The Spectral Radius as a Design Parameter

In reservoir computing, the spectral radius of the recurrent weight matrix $\rho(W^{rec})$ becomes a **hyperparameter that the designer controls**. Rather than trying to train $W^{rec}$ to a value that balances gradient flow against dynamical stability, the reservoir computing paradigm simply fixes $W^{rec}$ at construction time, choosing $\rho(W^{rec})$ to be near — but below — 1.

The implications of this choice are profound, and they occupy much of Chapters 5 and 6. For now, it is enough to note that the spectral radius is the central knob in a tradeoff between memory (high $\rho$) and stability (low $\rho$) — and that managing this tradeoff through training is exactly the problem that BPTT struggles with.

## Summary

The vanishing/exploding gradient problem arises because BPTT requires computing products of $t - s$ Jacobian matrices. The spectral radius of these matrices determines the exponential rate of growth or decay. For typical stable networks ($\rho < 1$), gradients vanish exponentially, preventing learning of dependencies longer than $\tau_{\text{eff}} \approx -1/\ln \rho$ steps. For unstable networks ($\rho > 1$), gradients explode. Training near $\rho = 1$ is required for long-range learning, but this regime is dynamically unstable. This dilemma is fundamental, not incidental: it follows from the mathematics of deep networks with shared weights, and partial solutions (LSTM, gradient clipping) alleviate but do not eliminate it.

---

## References

- [Bengio1994] Bengio, Y., Simard, P., & Frasconi, P. (1994). Learning long-term dependencies with gradient descent is difficult. *IEEE Transactions on Neural Networks*, 5(2), 157–166.
- [Hochreiter1997] Hochreiter, S., & Schmidhuber, J. (1997). Long short-term memory. *Neural Computation*, 9(8), 1735–1780.
- [Pascanu2013] Pascanu, R., Mikolov, T., & Bengio, Y. (2013). On the difficulty of training recurrent neural networks. *Proceedings of ICML*, 1310–1318.
- [Hochreiter1991] Hochreiter, S. (1991). Untersuchungen zu dynamischen neuronalen Netzen. Diploma thesis, Technische Universität München.

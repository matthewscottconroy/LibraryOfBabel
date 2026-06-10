# 3.3.1 Backpropagation Through Time

## The Training Problem

We have a network defined by parameters $\theta = \{W^{rec}, W^{in}, W^{out}, \mathbf{b}\}$. We have a training sequence: inputs $\mathbf{u}_1, \mathbf{u}_2, \ldots, \mathbf{u}_T$ and target outputs $\hat{\mathbf{y}}_1, \hat{\mathbf{y}}_2, \ldots, \hat{\mathbf{y}}_T$. We have a loss function — say, mean squared error:

$$L = \frac{1}{T} \sum_{t=1}^{T} \|\mathbf{y}_t - \hat{\mathbf{y}}_t\|^2 = \frac{1}{T} \sum_{t=1}^{T} L_t$$

where $L_t = \|\mathbf{y}_t - \hat{\mathbf{y}}_t\|^2$ is the loss at step $t$.

We want to minimize $L$ over $\theta$ by gradient descent. This requires computing $\partial L / \partial W^{rec}$, $\partial L / \partial W^{in}$, and $\partial L / \partial W^{out}$.

The complication is that $L_t$ depends on $\mathbf{y}_t = W^{out}\mathbf{x}_t$, which depends on $\mathbf{x}_t$, which depends on $\mathbf{x}_{t-1}$, which depends on $\mathbf{x}_{t-2}$, and so on back to $\mathbf{x}_0$. The parameter $W^{rec}$ appears at every time step of this chain. Computing $\partial L / \partial W^{rec}$ requires accounting for all these indirect dependencies.

**Backpropagation Through Time (BPTT)** [Werbos1990] resolves this by a conceptual move: *unroll the recurrence*.

## Unrolling the Recurrence

The state update equation is:

$$\mathbf{x}_{t+1} = f\!\left(W^{rec}\mathbf{x}_t + W^{in}\mathbf{u}_t + \mathbf{b}\right)$$

This is a recurrence: the same equation, with the same weights, applied at every time step. Now imagine "unrolling" this recurrence in time — writing out the computation as a sequence of explicit steps:

$$\mathbf{x}_1 = f\!\left(W^{rec}\mathbf{x}_0 + W^{in}\mathbf{u}_0 + \mathbf{b}\right)$$
$$\mathbf{x}_2 = f\!\left(W^{rec}\mathbf{x}_1 + W^{in}\mathbf{u}_1 + \mathbf{b}\right)$$
$$\vdots$$
$$\mathbf{x}_T = f\!\left(W^{rec}\mathbf{x}_{T-1} + W^{in}\mathbf{u}_{T-1} + \mathbf{b}\right)$$

This is a **feedforward network** of depth $T$. It is not exactly a standard feedforward network — the weight matrix $W^{rec}$ is **shared** across all layers (this is weight tying). But the computation graph is acyclic: it flows from left to right in time, and backpropagation applies.

The unrolled network has one "layer" per time step. Layer $t$ receives the previous state $\mathbf{x}_{t-1}$ and the current input $\mathbf{u}_{t-1}$, and produces the next state $\mathbf{x}_t$. Each layer applies the same function $F(\mathbf{x}, \mathbf{u}) = f(W^{rec}\mathbf{x} + W^{in}\mathbf{u} + \mathbf{b})$.

## The Computational Graph

Let us introduce notation that makes the chain rule explicit. Define the **pre-activation** at time $t$:

$$\mathbf{a}_t = W^{rec}\mathbf{x}_{t-1} + W^{in}\mathbf{u}_{t-1} + \mathbf{b}$$

so that $\mathbf{x}_t = f(\mathbf{a}_t)$.

The computational graph is:

$$\mathbf{x}_0 \to \mathbf{a}_1 \to \mathbf{x}_1 \to \mathbf{a}_2 \to \mathbf{x}_2 \to \cdots \to \mathbf{a}_T \to \mathbf{x}_T \to \mathbf{y}_T$$

At each step, there are also "output branches" $\mathbf{x}_t \to \mathbf{y}_t \to L_t$.

The total loss is $L = \sum_t L_t$, so by linearity of differentiation:

$$\frac{\partial L}{\partial W^{rec}} = \sum_{t=1}^{T} \frac{\partial L_t}{\partial W^{rec}}$$

We now compute $\partial L_t / \partial W^{rec}$ for a fixed $t$.

## Deriving the Gradient: Step by Step

**Step 1: Gradient of the loss with respect to the output.**

$$\frac{\partial L_t}{\partial \mathbf{y}_t} = 2(\mathbf{y}_t - \hat{\mathbf{y}}_t)$$

This is a row vector (or column vector, depending on convention; we treat it as a column vector here, i.e., $\partial L_t / \partial \mathbf{y}_t \in \mathbb{R}^M$).

**Step 2: Gradient of the loss with respect to the state at time $t$.**

Since $\mathbf{y}_t = W^{out}\mathbf{x}_t$:

$$\frac{\partial L_t}{\partial \mathbf{x}_t} = (W^{out})^T \frac{\partial L_t}{\partial \mathbf{y}_t} \in \mathbb{R}^N$$

Call this $\boldsymbol{\delta}_t^{(t)} \in \mathbb{R}^N$ — the "immediate" gradient at time $t$ due to $L_t$.

**Step 3: Propagating the gradient backward through time.**

The loss $L_t$ also affects earlier states through the recurrence. Using the chain rule:

$$\frac{\partial L_t}{\partial \mathbf{x}_s} = \frac{\partial \mathbf{x}_t}{\partial \mathbf{x}_s} \cdot \frac{\partial L_t}{\partial \mathbf{x}_t}$$

for $s < t$. Here $\partial \mathbf{x}_t / \partial \mathbf{x}_s$ is a Jacobian matrix. By the chain rule applied to the recurrence:

$$\frac{\partial \mathbf{x}_t}{\partial \mathbf{x}_s} = \prod_{k=s+1}^{t} \frac{\partial \mathbf{x}_k}{\partial \mathbf{x}_{k-1}}$$

The local Jacobian at step $k$ is:

$$\frac{\partial \mathbf{x}_k}{\partial \mathbf{x}_{k-1}} = \text{diag}\!\left(f'(\mathbf{a}_k)\right) \cdot W^{rec} = D_k W^{rec}$$

where $D_k = \text{diag}(f'(\mathbf{a}_k)) \in \mathbb{R}^{N \times N}$ is the diagonal matrix of derivatives of the nonlinearity at the pre-activations at step $k$.

Therefore:

$$\frac{\partial \mathbf{x}_t}{\partial \mathbf{x}_s} = \left(\prod_{k=s+1}^{t} D_k W^{rec}\right) = D_t W^{rec} \cdot D_{t-1} W^{rec} \cdots D_{s+1} W^{rec}$$

This is a product of $(t - s)$ matrices, each of the form $D_k W^{rec}$.

**Step 4: The gradient of $L_t$ with respect to $\mathbf{x}_s$.**

$$\frac{\partial L_t}{\partial \mathbf{x}_s} = \left(\prod_{k=s+1}^{t} D_k W^{rec}\right)^T \boldsymbol{\delta}_t^{(t)}$$

where we have used the transpose because the gradient is a column vector and the Jacobian maps forward.

More carefully, using the convention that $\partial L / \partial \mathbf{x}_s$ is a column vector $\in \mathbb{R}^N$, and the Jacobian $\partial \mathbf{x}_t / \partial \mathbf{x}_s \in \mathbb{R}^{N \times N}$:

$$\frac{\partial L_t}{\partial \mathbf{x}_s} = \left(\frac{\partial \mathbf{x}_t}{\partial \mathbf{x}_s}\right)^T \frac{\partial L_t}{\partial \mathbf{x}_t}$$

Recursively, define the **error signal** $\boldsymbol{\delta}_s^{(t)}$ as the gradient of $L_t$ with respect to $\mathbf{x}_s$:

$$\boldsymbol{\delta}_s^{(t)} = (D_{s+1} W^{rec})^T \boldsymbol{\delta}_{s+1}^{(t)} = (W^{rec})^T D_{s+1} \boldsymbol{\delta}_{s+1}^{(t)}$$

with boundary condition $\boldsymbol{\delta}_t^{(t)} = (W^{out})^T \frac{\partial L_t}{\partial \mathbf{y}_t}$.

This is the backward pass: we start from the loss at time $t$ and propagate the error signal backward in time, step by step, through the transpose of the local Jacobians.

**Step 5: Gradient with respect to $W^{rec}$.**

The pre-activation at time $s$ is $\mathbf{a}_s = W^{rec}\mathbf{x}_{s-1} + W^{in}\mathbf{u}_{s-1} + \mathbf{b}$. The gradient of $L_t$ with respect to $W^{rec}$ comes from the explicit appearance of $W^{rec}$ at every time step $s \leq t$:

$$\frac{\partial L_t}{\partial W^{rec}} = \sum_{s=1}^{t} \frac{\partial L_t}{\partial \mathbf{a}_s} \mathbf{x}_{s-1}^T$$

Here $\partial L_t / \partial \mathbf{a}_s \in \mathbb{R}^N$ is the gradient with respect to the pre-activation at step $s$, given by:

$$\frac{\partial L_t}{\partial \mathbf{a}_s} = D_s \boldsymbol{\delta}_s^{(t)}$$

where $D_s = \text{diag}(f'(\mathbf{a}_s))$ and $\boldsymbol{\delta}_s^{(t)}$ is the error signal propagated back from time $t$ to time $s$.

The outer product $\frac{\partial L_t}{\partial \mathbf{a}_s} \mathbf{x}_{s-1}^T$ has shape $N \times N$, matching $W^{rec}$.

**Step 6: The full gradient.**

Summing over all time steps:

$$\boxed{\frac{\partial L}{\partial W^{rec}} = \sum_{t=1}^{T} \sum_{s=1}^{t} D_s \boldsymbol{\delta}_s^{(t)} \mathbf{x}_{s-1}^T}$$

This double sum accounts for every path through the computation graph: for each output time $t$, and for each step $s \leq t$ at which $W^{rec}$ acts.

## Recursive Reformulation

The double sum can be reorganized into a single backward pass by accumulating the total error signal at each time step. Define:

$$\boldsymbol{\delta}_s = \sum_{t \geq s} \boldsymbol{\delta}_s^{(t)}$$

This is the total error signal at time $s$, summing contributions from all future losses $L_t$ for $t \geq s$. Then:

$$\frac{\partial L}{\partial W^{rec}} = \sum_{s=1}^{T} D_s \boldsymbol{\delta}_s \mathbf{x}_{s-1}^T$$

The total error signal $\boldsymbol{\delta}_s$ can be computed recursively:

$$\boldsymbol{\delta}_s = (W^{out})^T \frac{\partial L_s}{\partial \mathbf{y}_s} + (W^{rec})^T D_{s+1} \boldsymbol{\delta}_{s+1}$$

with boundary condition $\boldsymbol{\delta}_T = (W^{out})^T \frac{\partial L_T}{\partial \mathbf{y}_T}$.

The first term is the "direct" gradient from the loss at time $s$. The second term is the propagated error from all future steps, flowing backward through $(W^{rec})^T D_{s+1}$.

## Reading the Algorithm

The BPTT algorithm can now be stated cleanly:

1. **Forward pass:** Starting from $\mathbf{x}_0$, compute and store $\mathbf{x}_1, \mathbf{a}_1, \mathbf{x}_2, \mathbf{a}_2, \ldots, \mathbf{x}_T, \mathbf{a}_T$ and all outputs $\mathbf{y}_t$.

2. **Backward pass:** Starting from $\boldsymbol{\delta}_T = (W^{out})^T \frac{\partial L_T}{\partial \mathbf{y}_T}$, propagate backward: for $s = T-1, T-2, \ldots, 1$:
   $$\boldsymbol{\delta}_s = (W^{out})^T \frac{\partial L_s}{\partial \mathbf{y}_s} + (W^{rec})^T D_{s+1} \boldsymbol{\delta}_{s+1}$$

3. **Gradient accumulation:** At each step $s$, accumulate:
   $$\frac{\partial L}{\partial W^{rec}} \mathrel{+}= D_s \boldsymbol{\delta}_s \mathbf{x}_{s-1}^T$$

4. **Update:** $W^{rec} \leftarrow W^{rec} - \eta \frac{\partial L}{\partial W^{rec}}$.

## Computational Cost and Memory

BPTT requires:
- Storing the entire state trajectory $\mathbf{x}_0, \ldots, \mathbf{x}_T$ and all pre-activations $\mathbf{a}_1, \ldots, \mathbf{a}_T$ during the forward pass: $O(TN)$ memory.
- One backward pass of length $T$: $O(TN^2)$ operations (dominated by the matrix-vector products $(W^{rec})^T D_{s+1} \boldsymbol{\delta}_{s+1}$).

For long sequences ($T$ large) or large networks ($N$ large), this becomes expensive. For $T = 1000$ and $N = 1000$, the backward pass requires $10^9$ floating-point operations and $10^6$ stored values. More troublingly, the backward pass must run sequentially: you cannot compute $\boldsymbol{\delta}_{s}$ without first computing $\boldsymbol{\delta}_{s+1}$.

These computational costs are real but manageable with modern hardware. The deeper problem — which we address in Section 3.4 — is not computational but mathematical: what happens to the gradient as $T$ grows large.

## Truncated BPTT

In practice, full BPTT is often too expensive for very long sequences. **Truncated BPTT** addresses this by partitioning the sequence into chunks of length $\tau$ and running BPTT only within each chunk. The gradient approximation is:

$$\frac{\partial L}{\partial W^{rec}} \approx \sum_{s=t-\tau}^{t} D_s \boldsymbol{\delta}_s \mathbf{x}_{s-1}^T$$

This ignores gradient contributions from more than $\tau$ steps in the past. It is computationally tractable but biased: it systematically underestimates the contribution of long-range dependencies. For tasks that require memory longer than $\tau$ steps, truncated BPTT will fail to learn the necessary structure.

Choosing $\tau$ is a practical design decision that encodes an implicit assumption about the relevant time scale of the task. This is an uncomfortable position: we are asking the algorithm to learn the time scale of its task, but we are forcing it to truncate its gradients at a fixed time scale $\tau$ that we chose in advance.

---

## References

- [Werbos1990] Werbos, P. J. (1990). Backpropagation through time: What it does and how to do it. *Proceedings of the IEEE*, 78(10), 1550–1560.
- [Rumelhart1986] Rumelhart, D. E., Hinton, G. E., & Williams, R. J. (1986). Learning internal representations by error propagation. In *Parallel Distributed Processing*, Vol. 1, MIT Press.
- [Goodfellow2016] Goodfellow, I., Bengio, Y., & Courville, A. (2016). *Deep Learning*. MIT Press. Chapter 10.

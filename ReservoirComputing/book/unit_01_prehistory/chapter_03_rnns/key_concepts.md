# Chapter 3 Key Concepts

---

## 1. Recurrent Neural Network (RNN)

A **recurrent neural network** is a neural network architecture in which neurons receive connections from other neurons at the same or previous time step, allowing the network to maintain an internal state that persists across the temporal sequence of inputs. The defining equation is:

$$\mathbf{x}_{t+1} = f\!\left(W^{rec}\mathbf{x}_t + W^{in}\mathbf{u}_t + \mathbf{b}\right)$$

where $\mathbf{x}_t \in \mathbb{R}^N$ is the state, $\mathbf{u}_t \in \mathbb{R}^K$ is the input, and $f$ is a pointwise nonlinearity. The recurrent weight matrix $W^{rec} \in \mathbb{R}^{N \times N}$ captures the network's internal dynamics. Unlike feedforward networks, an RNN is a **dynamical system**: its state at time $t$ depends on all previous inputs, not just the current one. RNNs are, in principle, Turing-complete [Siegelmann1995] and universal approximators for temporal functions.

---

## 2. Backpropagation Through Time (BPTT)

**Backpropagation through time** is the standard algorithm for computing gradients of a loss function with respect to the parameters of an RNN. It works by "unrolling" the recurrent computation in time, producing an equivalent feedforward network of depth $T$ (the sequence length), and then applying standard backpropagation to this unrolled network. The central complication is that the recurrent weight matrix $W^{rec}$ appears at every time step, so its gradient receives contributions from every depth in the unrolled network. The gradient with respect to $W^{rec}$ involves products of Jacobians:

$$\frac{\partial L_t}{\partial \mathbf{x}_s} = \left(\prod_{k=s+1}^{t} D_k W^{rec}\right)^T \boldsymbol{\delta}_t$$

where $D_k = \text{diag}(f'(\mathbf{a}_k))$. BPTT requires $O(TN)$ memory and $O(TN^2)$ computation, and must be run sequentially. **Truncated BPTT** restricts the backward pass to windows of length $\tau$, sacrificing gradient accuracy for computational feasibility.

---

## 3. Vanishing Gradient Problem

The **vanishing gradient problem** [Bengio1994, Hochreiter1991] refers to the exponential decay of gradient signals as they are propagated backward through many time steps in an RNN. Because the gradient of the loss at time $t$ with respect to the state at time $s$ involves a product of $(t - s)$ Jacobians $J_k = D_k W^{rec}$, the norm of this product decays as approximately $\rho(J)^{t-s}$ when $\rho(J) < 1$. For typical stable RNNs, this means that the gradient for time steps more than $\tau_{\text{eff}} \approx -1/\ln \rho(J)$ steps in the past is effectively zero. The network can learn only short-range dependencies, regardless of the training sequence length. This is the fundamental barrier to learning long-range temporal structure with standard gradient-based training.

---

## 4. Spectral Radius

The **spectral radius** of a matrix $A$, denoted $\rho(A)$, is the largest absolute value of its eigenvalues:

$$\rho(A) = \max_i |\lambda_i(A)|$$

In the context of RNNs, the spectral radius of the recurrent weight matrix $W^{rec}$ plays a dual role. Dynamically: if $\rho(W^{rec}) < 1$ (for a linear system), the state converges to zero for any initial condition, and the network has fading memory. If $\rho(W^{rec}) > 1$, the state can grow without bound. In terms of gradient flow: the spectral radius of the Jacobian $J_t = D_t W^{rec}$ (which is bounded above by $\rho(W^{rec})$) determines whether gradients vanish or explode during BPTT. The spectral radius reappears as the central hyperparameter of reservoir computing in Chapter 5, where it is set at construction time rather than learned.

---

## 5. Long Short-Term Memory (LSTM)

The **Long Short-Term Memory** network [Hochreiter1997] is a recurrent architecture that addresses the vanishing gradient problem through explicit memory cells and gating mechanisms. The LSTM maintains a **cell state** $\mathbf{c}_t$ and a **hidden state** $\mathbf{h}_t$, updated as:

$$\mathbf{f}_t = \sigma(W_f [\mathbf{h}_{t-1}; \mathbf{u}_t] + \mathbf{b}_f) \quad \text{(forget gate)}$$
$$\mathbf{i}_t = \sigma(W_i [\mathbf{h}_{t-1}; \mathbf{u}_t] + \mathbf{b}_i) \quad \text{(input gate)}$$
$$\tilde{\mathbf{c}}_t = \tanh(W_c [\mathbf{h}_{t-1}; \mathbf{u}_t] + \mathbf{b}_c) \quad \text{(cell candidate)}$$
$$\mathbf{c}_t = \mathbf{f}_t \odot \mathbf{c}_{t-1} + \mathbf{i}_t \odot \tilde{\mathbf{c}}_t \quad \text{(cell update)}$$
$$\mathbf{o}_t = \sigma(W_o [\mathbf{h}_{t-1}; \mathbf{u}_t] + \mathbf{b}_o) \quad \text{(output gate)}$$
$$\mathbf{h}_t = \mathbf{o}_t \odot \tanh(\mathbf{c}_t) \quad \text{(hidden state)}$$

The key insight is the **constant error carousel**: the gradient through $\mathbf{c}_t$ is $\text{diag}(\mathbf{f}_t)$, which is close to the identity when $\mathbf{f}_t \approx 1$. This provides a pathway for gradient flow that avoids repeated application of $W^{rec}$.

---

## 6. Gated Recurrent Unit (GRU)

The **Gated Recurrent Unit** [Cho2014] is a simplified variant of the LSTM that uses two gates (update and reset) instead of three, and no separate cell state. The GRU typically achieves comparable performance to LSTM with fewer parameters, making it more sample-efficient on smaller datasets. The update equations are:

$$\mathbf{z}_t = \sigma(W_z [\mathbf{h}_{t-1}; \mathbf{u}_t]) \quad \text{(update gate)}$$
$$\mathbf{r}_t = \sigma(W_r [\mathbf{h}_{t-1}; \mathbf{u}_t]) \quad \text{(reset gate)}$$
$$\tilde{\mathbf{h}}_t = \tanh(W_h [\mathbf{r}_t \odot \mathbf{h}_{t-1}; \mathbf{u}_t]) \quad \text{(candidate)}$$
$$\mathbf{h}_t = (1 - \mathbf{z}_t) \odot \mathbf{h}_{t-1} + \mathbf{z}_t \odot \tilde{\mathbf{h}}_t$$

Like LSTM, the GRU's update gate can preserve gradients through time by keeping $\mathbf{z}_t$ close to 0 (which lets $\mathbf{h}_t \approx \mathbf{h}_{t-1}$).

---

## 7. Teacher Forcing

**Teacher forcing** [Williams1989tf] is a training strategy for sequence-to-sequence RNNs in which the ground-truth output $\hat{\mathbf{y}}_{t-1}$ is fed as the input at time $t$ during training, regardless of the network's actual output $\mathbf{y}_{t-1}$. This prevents compounding errors during the forward pass and dramatically stabilizes training — if the network makes a mistake, it does not cascade through the rest of the sequence. However, teacher forcing creates an **exposure bias**: at test time, the ground truth is unavailable, so the network is fed its own (potentially incorrect) outputs. The distribution of inputs at test time differs from training, which can lead to unstable rollouts. Various **scheduled sampling** strategies [Bengio2015samp] interpolate between teacher forcing and autonomous mode during training to mitigate this issue.

---

## 8. Real-Time Recurrent Learning (RTRL)

**Real-Time Recurrent Learning** [Williams1989] is an alternative to BPTT that computes exact gradients online, without unrolling. RTRL maintains, at each time step, the full sensitivity matrix $\partial \mathbf{x}_t / \partial W^{rec} \in \mathbb{R}^{N \times N^2}$, which is updated recursively:

$$\frac{\partial \mathbf{x}_t}{\partial W^{rec}} = D_t W^{rec} \frac{\partial \mathbf{x}_{t-1}}{\partial W^{rec}} + D_t \frac{\partial (W^{rec}\mathbf{x}_{t-1})}{\partial W^{rec}}$$

Unlike BPTT, RTRL is causal (no future information required) and can run indefinitely. However, its computational cost is $O(N^4)$ per time step — far more expensive than BPTT's $O(N^2)$ per step. Approximations to RTRL that reduce this cost at the expense of gradient accuracy (e.g., [Tallec2017]) are an active research area with connections to biological plausibility.

---

## 9. Biological Plausibility

**Biological plausibility** refers to the extent to which a learning algorithm could, in principle, be implemented by the synaptic plasticity mechanisms of biological neural circuits. BPTT is widely regarded as biologically implausible on several grounds: it requires storing activations from the entire forward pass in memory, propagating error signals backward in time (requiring knowledge of future events), and performing weight updates via the transpose of the forward weight matrix (requiring "weight symmetry" between forward and backward pathways). These objections motivate the search for alternatives: spike-timing-dependent plasticity (STDP), predictive coding, local contrastive Hebbian learning, and — most relevant to this textbook — the reservoir computing hypothesis, in which only the output layer is trained, using a local and biologically feasible Hebbian or delta rule.

---

## 10. The Reservoir Hypothesis

The **reservoir hypothesis** is the foundational idea of reservoir computing: that a large, fixed, randomly-connected recurrent network (the "reservoir") provides a rich enough representation of the temporal input history that only a simple linear readout — trained by ordinary regression — is needed to compute useful outputs. The recurrent dynamics are not trained; only the output weights are learned. This hypothesis implies that:

1. Random recurrent connectivity, properly scaled, generates diverse and information-rich state trajectories.
2. The linear span of the reservoir states can approximate the target output function.
3. Learning reduces to a convex optimization (least squares), with guaranteed global optimality.

The conditions under which the reservoir hypothesis holds — separation, fading memory, and approximation — are formalized in the **echo state property** (Jaeger, Chapter 5) and the **liquid state machine** formalism (Maass, Chapter 6). This hypothesis is the conceptual bridge from Unit I (the prehistory of temporal computation) to Unit II (the reservoir computing paradigm).

---

## References

- [Siegelmann1995] Siegelmann, H. T., & Sontag, E. D. (1995). Computational power of neural nets. *JCSS*, 50(1), 132–150.
- [Bengio1994] Bengio, Y., Simard, P., & Frasconi, P. (1994). Learning long-term dependencies. *IEEE Trans. Neural Networks*, 5(2), 157–166.
- [Hochreiter1997] Hochreiter, S., & Schmidhuber, J. (1997). Long short-term memory. *Neural Computation*, 9(8), 1735–1780.
- [Hochreiter1991] Hochreiter, S. (1991). Untersuchungen zu dynamischen neuronalen Netzen. Diploma thesis, TU München.
- [Williams1989] Williams, R. J., & Zipser, D. (1989). A learning algorithm for continually running fully recurrent neural networks. *Neural Computation*, 1(2), 270–280.
- [Cho2014] Cho, K., et al. (2014). Learning phrase representations using RNN encoder–decoder for statistical machine translation. *EMNLP*.
- [Bengio2015samp] Bengio, S., et al. (2015). Scheduled sampling for sequence prediction with recurrent neural networks. *NeurIPS*.
- [Tallec2017] Tallec, C., Ollivier, Y. (2017). Unbiased online recurrent optimization. *arXiv:1702.05043*.

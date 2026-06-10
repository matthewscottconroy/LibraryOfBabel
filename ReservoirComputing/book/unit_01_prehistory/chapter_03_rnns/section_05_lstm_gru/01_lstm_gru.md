# 3.5 LSTM and GRU: Gating as the Solution

## The Problem to Be Solved

The vanishing gradient analysis of Section 3.4 identified the core pathology precisely: the gradient of $\mathbf{x}_t$ with respect to $\mathbf{x}_{t-k}$ is a product of $k$ Jacobian matrices, and if the spectral radius of those Jacobians is less than 1, the gradient shrinks exponentially with $k$. If it is greater than 1, it explodes.

There is a simple case where this does not happen: if the Jacobian at every step is the identity matrix, then $\partial \mathbf{x}_t / \partial \mathbf{x}_{t-k} = I^k = I$ for all $k$, and gradients neither vanish nor explode. The gradient flows backward through time as if time does not exist.

This observation is the germ of the Long Short-Term Memory (LSTM) [Hochreiter1997]. The idea is to build a recurrent unit with a **cell state** $\mathbf{c}_t$ that passes through time with only controlled, multiplicative modifications — a "constant error carousel" through which gradients can flow without decay.

The practical costs: more parameters, more complex dynamics, and a model that is harder to interpret theoretically. But the empirical benefits are substantial, which is why LSTMs and their simplified variant, the Gated Recurrent Unit (GRU) [Cho2014], dominated sequence modeling before the transformer era, and remain important benchmarks.

## The LSTM Architecture

The LSTM introduces a distinction between two types of recurrent state:

- **Cell state** $\mathbf{c}_t \in \mathbb{R}^N$: the long-term memory. Flows through time with only additive updates.
- **Hidden state** $\mathbf{h}_t \in \mathbb{R}^N$: the short-term output state. Derived from the cell state.

Three **gates** — forget, input, and output — control information flow. Each gate is a sigmoid function of the current input and previous hidden state, producing values in $(0, 1)$. A gate output near 1 means "let through"; near 0 means "block."

**Notation:** Let $\mathbf{u}_t \in \mathbb{R}^M$ be the input, $\mathbf{h}_{t-1} \in \mathbb{R}^N$ the previous hidden state. Define $[\mathbf{h}_{t-1}; \mathbf{u}_t] \in \mathbb{R}^{N+M}$ as their concatenation.

### The Gate Equations

**Forget gate** — decides what fraction of the old cell state to retain:
$$\mathbf{f}_t = \sigma(W_f [\mathbf{h}_{t-1}; \mathbf{u}_t] + \mathbf{b}_f)$$

**Input gate** — decides which new information to add:
$$\mathbf{i}_t = \sigma(W_i [\mathbf{h}_{t-1}; \mathbf{u}_t] + \mathbf{b}_i)$$

**Candidate cell state** — the proposed new content:
$$\tilde{\mathbf{c}}_t = \tanh(W_c [\mathbf{h}_{t-1}; \mathbf{u}_t] + \mathbf{b}_c)$$

**Cell state update** — the core recurrence:
$$\mathbf{c}_t = \mathbf{f}_t \odot \mathbf{c}_{t-1} + \mathbf{i}_t \odot \tilde{\mathbf{c}}_t$$

**Output gate** — decides what to expose to the hidden state:
$$\mathbf{o}_t = \sigma(W_o [\mathbf{h}_{t-1}; \mathbf{u}_t] + \mathbf{b}_o)$$

**Hidden state output:**
$$\mathbf{h}_t = \mathbf{o}_t \odot \tanh(\mathbf{c}_t)$$

Here $\odot$ denotes elementwise multiplication (Hadamard product). All weight matrices $W_f, W_i, W_c, W_o \in \mathbb{R}^{N \times (N+M)}$ and bias vectors $\mathbf{b}_f, \mathbf{b}_i, \mathbf{b}_c, \mathbf{b}_o \in \mathbb{R}^N$ are learned parameters.

## The Constant Error Carousel

The critical insight is the cell state update equation:
$$\mathbf{c}_t = \mathbf{f}_t \odot \mathbf{c}_{t-1} + \mathbf{i}_t \odot \tilde{\mathbf{c}}_t$$

This is an **additive** update — the new cell state is the old cell state scaled by the forget gate, plus new content. Compare this to the standard RNN hidden state:
$$\mathbf{x}_t = \tanh(W^{rec} \mathbf{x}_{t-1} + W^{in} \mathbf{u}_t)$$

In the standard RNN, $\mathbf{x}_{t-1}$ is squashed through $\tanh$ at every step, compressing it and making gradients vanish. In the LSTM, $\mathbf{c}_{t-1}$ passes through unchanged except for elementwise multiplication by $\mathbf{f}_t$.

**The gradient through the cell state:**

$$\frac{\partial \mathbf{c}_t}{\partial \mathbf{c}_{t-k}} = \prod_{j=0}^{k-1} \text{diag}(\mathbf{f}_{t-j})$$

This is a product of diagonal matrices — specifically, the forget gate values along the way. If $\mathbf{f}_t \approx \mathbf{1}$ (forget gate open), the product is approximately $I^k = I$: **the gradient flows backward without decay**. If $\mathbf{f}_t \approx \mathbf{0}$ (forget gate closed), the gradient is zeroed out — deliberately, controlled forgetting.

Hochreiter and Schmidhuber called this the **constant error carousel**: a pathway through which error signals can circulate without decay for as long as the forget gate allows. The name captures the mechanism: errors ride the carousel round and round without fading.

**What the gates provide:** The forget gate allows the LSTM to decide, at each step, whether to preserve or discard each dimension of the cell state. The input gate controls what new information to write in. The output gate controls what to read out. Together, they give the LSTM learned, input-dependent memory management.

## Gradient Flow Analysis

Consider a time interval $[t-k, t]$ where the forget gate is open ($\mathbf{f} \approx \mathbf{1}$) and the input gate is closed ($\mathbf{i} \approx \mathbf{0}$). Then:

$$\mathbf{c}_{t} \approx \mathbf{c}_{t-1} \approx \cdots \approx \mathbf{c}_{t-k}$$

The cell state is preserved exactly, and the gradient is:
$$\frac{\partial \mathcal{L}}{\partial \mathbf{c}_{t-k}} = \frac{\partial \mathcal{L}}{\partial \mathbf{c}_t} \cdot \prod_{j=0}^{k-1} \mathbf{f}_{t-j} \approx \frac{\partial \mathcal{L}}{\partial \mathbf{c}_t}$$

The error signal at time $t$ is faithfully propagated to time $t-k$. This is qualitatively different from the standard RNN, where the gradient would be exponentially small.

**The limits of LSTM gradient flow:**
1. If the forget gate values are less than 1 (which they always are in practice, since $\sigma(\cdot) \in (0,1)$), there is still exponential decay — just slower.
2. The gate activations themselves are computed from $\mathbf{h}_{t-1}$, which involves $\tanh$ activations, so there is still vanishing in those pathways.
3. The LSTM does not solve the vanishing gradient problem, it ameliorates it by providing a pathway with slower decay.

The practical effect is that LSTMs can reliably learn dependencies over 100–1000 time steps, whereas standard RNNs struggle beyond 10–20 steps.

## The Gated Recurrent Unit

The GRU [Cho2014] is a simplification of the LSTM that combines the forget and input gates into a single **update gate** and eliminates the output gate and the explicit cell state. The result has fewer parameters and is faster to compute, with similar empirical performance on many tasks.

**GRU equations:**

**Reset gate:**
$$\mathbf{r}_t = \sigma(W_r [\mathbf{h}_{t-1}; \mathbf{u}_t] + \mathbf{b}_r)$$

**Update gate:**
$$\mathbf{z}_t = \sigma(W_z [\mathbf{h}_{t-1}; \mathbf{u}_t] + \mathbf{b}_z)$$

**Candidate hidden state:**
$$\tilde{\mathbf{h}}_t = \tanh(W_h [\mathbf{r}_t \odot \mathbf{h}_{t-1}; \mathbf{u}_t] + \mathbf{b}_h)$$

**Hidden state update:**
$$\mathbf{h}_t = (1 - \mathbf{z}_t) \odot \mathbf{h}_{t-1} + \mathbf{z}_t \odot \tilde{\mathbf{h}}_t$$

The update gate $\mathbf{z}_t$ interpolates between the old hidden state and the candidate: when $\mathbf{z}_t \approx \mathbf{0}$, the hidden state is preserved; when $\mathbf{z}_t \approx \mathbf{1}$, it is fully updated. The reset gate $\mathbf{r}_t$ controls how much of the previous hidden state is used to compute the candidate — with $\mathbf{r}_t \approx \mathbf{0}$, the candidate is computed entirely from the current input.

**GRU vs. LSTM:**
| Aspect | LSTM | GRU |
|--------|------|-----|
| State | Cell $\mathbf{c}_t$ + hidden $\mathbf{h}_t$ | Hidden $\mathbf{h}_t$ only |
| Gates | 3 (forget, input, output) | 2 (reset, update) |
| Parameters | $4N(N+M)$ | $3N(N+M)$ |
| Cell state | Explicit, additive | Implicit, interpolative |
| Performance | Often marginally better on long sequences | Often comparable, faster |

## What Gating Does Not Solve

LSTMs and GRUs are practical engineering solutions, not theoretical resolutions of the vanishing gradient problem. Several fundamental limitations remain:

**1. The optimization landscape remains complex.** Even with gating, training LSTMs requires careful initialization, appropriate learning rates, and gradient clipping for stability. The landscape is not convex; multiple local optima exist.

**2. Interpretability is limited.** The gate values are dense, input-dependent functions that interact nonlinearly. Understanding what an LSTM has learned — what is in the cell state, what the forget gate is doing — requires careful analysis (see [Karpathy2015] for a celebrated visualization study).

**3. Parallelization is limited.** The LSTM recurrence is sequential: $\mathbf{c}_t$ depends on $\mathbf{c}_{t-1}$, so the computation cannot be parallelized across time. This is a fundamental bottleneck that transformers overcome by replacing recurrence with attention, at the cost of quadratic memory in sequence length.

**4. The inductive bias is not always appropriate.** The additive cell state update embeds an assumption that memory should be approximately linear over long time spans. For tasks requiring sharply nonlinear temporal integration, this bias may not be helpful.

## The Reservoir Alternative

LSTMs solve the gradient problem by architectural complexity. Reservoir computing solves it by not training the recurrent weights at all. The comparison illuminates what each approach is buying:

| Property | LSTM | Echo State Network |
|----------|------|--------------------|
| Gradient through time | Controlled, slow decay | Not needed (fixed weights) |
| Training | Gradient-based (BPTT on gates) | Linear regression (readout only) |
| Memory capacity | Learned, task-specific | Geometric, task-agnostic |
| Interpretability | Low | High (state-space analysis) |
| Initialization sensitivity | High | Low (typical random works) |
| Computational cost (training) | $O(T \cdot N^2)$ backprop | $O(N^2 T + N^3)$ regression |
| Theoretical guarantees | Limited | Echo state property, Boyd-Chua |

The key insight: LSTMs learn memory structure from data. ESNs impose memory structure through the fixed reservoir and assume the linear readout can extract what is needed. For tasks where the required memory structure is complex and data is plentiful, LSTMs have an advantage. For tasks where labeled data is scarce, the memory structure is simple, or the temporal structure of the input is rich, ESNs often outperform.

---

## References

- [Hochreiter1997] Hochreiter, S. & Schmidhuber, J. (1997). Long short-term memory. *Neural Computation*, 9(8), 1735–1780. **[The original LSTM paper. The constant error carousel idea and full gate derivation.]**
- [Hochreiter1991] Hochreiter, S. (1991). *Untersuchungen zu dynamischen neuronalen Netzen* (diploma thesis, TU München). **(Translated: "Investigations on dynamic neural networks" — the vanishing gradient analysis predating the LSTM paper.)**
- [Cho2014] Cho, K. et al. (2014). Learning phrase representations using RNN encoder-decoder for statistical machine translation. *EMNLP 2014*. **[Introduces the GRU.]**
- [Gers2000] Gers, F.A., Schmidhuber, J., & Cummins, F. (2000). Learning to forget: Continual prediction with LSTM. *Neural Computation*, 12(10), 2451–2471. **[Adds the forget gate to the original LSTM architecture.]**
- [Karpathy2015] Karpathy, A., Johnson, J., & Fei-Fei, L. (2015). Visualizing and understanding recurrent networks. *arXiv:1506.02078*. **[Empirical analysis of what LSTM gates learn.]**
- [Greff2017] Greff, K. et al. (2017). LSTM: A search space odyssey. *IEEE Transactions on Neural Networks and Learning Systems*, 28(10), 2222–2232. **[Systematic comparison of LSTM variants — finding that the forget gate is the most important component.]**
- [Jaeger2001] Jaeger, H. (2001). The 'echo state' approach to analysing and training recurrent neural networks. GMD Technical Report 148. **[The reservoir alternative — compare to LSTM approach.]**

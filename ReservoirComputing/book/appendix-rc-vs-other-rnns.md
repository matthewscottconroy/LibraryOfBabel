# Appendix: Reservoir Computing in the Landscape of Recurrent Neural Networks

---

## Overview

Reservoir computing did not emerge in a vacuum. It arrived in 2001-2002, twelve years after Williams and Zipser formalised backpropagation through time (BPTT), four years after Hochreiter and Schmidhuber's LSTM had established itself as the best practical approach to long-range temporal dependencies, and in the same year that the first transformers would eventually displace both. Understanding reservoir computing requires knowing what problem each architecture was designed to solve, what mathematical mechanism it exploits, and what task regime makes each approach dominant.

This appendix provides that comparison. We cover five architectures in the roughly chronological order they addressed the vanishing gradient problem: vanilla RNNs, LSTMs, GRUs, echo state networks (the primary reservoir computing architecture), and transformers. We then present a decision framework and a mathematical analysis of computational capacity.

---

## 1. The Baseline: Vanilla Recurrent Neural Networks

### Architecture

The simplest RNN computes

$$\mathbf{x}_t = f(W^{rec}\mathbf{x}_{t-1} + W^{in}\mathbf{u}_t + \mathbf{b}), \qquad \hat{\mathbf{y}}_t = W^{out}\mathbf{x}_t$$

where $f$ is typically $\tanh$, $\mathbf{x}_t \in \mathbb{R}^N$ is the hidden state, $\mathbf{u}_t \in \mathbb{R}^K$ is the input at time $t$, and $\hat{\mathbf{y}}_t \in \mathbb{R}^M$ is the output. All weight matrices $W^{rec}, W^{in}, W^{out}$ are trained by gradient descent via BPTT.

### The Vanishing Gradient Problem

The gradient of the loss at time $T$ with respect to the hidden state at time $s < T$ involves the product of Jacobians:

$$\frac{\partial \mathbf{x}_T}{\partial \mathbf{x}_s} = \prod_{k=s+1}^{T} \frac{\partial \mathbf{x}_k}{\partial \mathbf{x}_{k-1}} = \prod_{k=s+1}^{T} D_k W^{rec}$$

where $D_k = \text{diag}(f'(a_k^{(1)}), \ldots, f'(a_k^{(N)}))$ is the Jacobian of the nonlinearity at time $k$.

For $\tanh$, all diagonal entries satisfy $D_k^{(ii)} = 1 - \tanh^2(a_k^{(i)}) \in (0, 1]$, with equality only at $a_k^{(i)} = 0$. Consequently:

$$\left\|\frac{\partial \mathbf{x}_T}{\partial \mathbf{x}_s}\right\|_2 \leq \|D\|_2^{T-s} \|W^{rec}\|_2^{T-s} \leq \rho(W^{rec})^{T-s}$$

where the last inequality uses $\|D\|_2 \leq 1$. For $\rho(W^{rec}) < 1$ (necessary for the echo state property), this vanishes exponentially with $T - s$. The network cannot learn to use information from more than roughly $\tau_{eff} = -1/\ln(\rho)$ time steps ago.

Bengio, Simard, and Frasconi (1994) proved that this is not merely a practical difficulty but a fundamental structural feature: any recurrent network with fixed-point attractors must have vanishing gradients through the stable fixed-point directions. The network faces a dilemma: to have long memory, it must have near-unit eigenvalues; but near-unit eigenvalues cause training to be slow or unstable.

### What Vanilla RNNs Can Do Well

Despite the vanishing gradient problem, vanilla RNNs (with careful initialisation such as orthogonal initialisation, or with gradient clipping) perform well when:
- The task requires only short-range memory ($T \lesssim 20$).
- The task involves smooth, regular dynamics where the implicit regularisation of small recurrent weights is beneficial.
- Speed of inference is critical and model size is constrained.
- The training sequence length is short (avoiding the explosion of memory required by BPTT over long sequences).

---

## 2. Long Short-Term Memory (LSTM)

### Historical Context

Hochreiter and Schmidhuber (1997) directly diagnosed the vanishing gradient problem and designed a solution: a memory cell $\mathbf{c}_t$ whose gradient pathway bypasses the nonlinearity. The cell state update is:

$$\mathbf{c}_t = \mathbf{f}_t \odot \mathbf{c}_{t-1} + \mathbf{i}_t \odot \tilde{\mathbf{c}}_t$$

where:
- $\mathbf{f}_t = \sigma(W_f [\mathbf{h}_{t-1}; \mathbf{u}_t] + \mathbf{b}_f)$ — forget gate
- $\mathbf{i}_t = \sigma(W_i [\mathbf{h}_{t-1}; \mathbf{u}_t] + \mathbf{b}_i)$ — input gate
- $\tilde{\mathbf{c}}_t = \tanh(W_c [\mathbf{h}_{t-1}; \mathbf{u}_t] + \mathbf{b}_c)$ — candidate cell state
- $\mathbf{o}_t = \sigma(W_o [\mathbf{h}_{t-1}; \mathbf{u}_t] + \mathbf{b}_o)$ — output gate
- $\mathbf{h}_t = \mathbf{o}_t \odot \tanh(\mathbf{c}_t)$ — hidden state

The key observation is that the partial derivative of $c_t^{(i)}$ with respect to $c_{t-1}^{(i)}$ is simply $f_t^{(i)}$, the forget gate value. The product of these over multiple steps:

$$\frac{\partial c_t^{(i)}}{\partial c_s^{(i)}} = \prod_{k=s+1}^{t} f_k^{(i)}$$

is not filtered through $W^{rec}$ at all. Hochreiter and Schmidhuber called this the **constant error carousel (CEC)**: if the forget gate maintains $f_k^{(i)} \approx 1$, the gradient flows back through time without decay.

### Mathematical Properties

**Gradient control.** The cell-state gradient product $\prod_k f_k^{(i)}$ can be controlled by the forget gate's own weights. Unlike the vanilla RNN, the network can learn to keep this product near 1 for relevant inputs and near 0 for irrelevant inputs.

**Expressivity.** LSTMs are Turing complete (Siegelmann & Sontag 1995 proved this for RNNs generally, and LSTMs are a special case). They can in principle represent any computable sequence-to-sequence function.

**Effective memory.** The effective memory of a well-trained LSTM is not bounded by a fixed $\tau_{eff}$; it is task-dependent and can in principle span the entire training sequence length. In practice, LSTMs reliably solve tasks requiring memory spans of hundreds to thousands of steps.

### When LSTMs Beat ESNs

LSTMs outperform echo state networks when:
1. **Long-range dependencies are critical.** Tasks requiring memory spans $T > 100$ strongly favour LSTMs over ESNs with typical spectral radii ($\rho \leq 0.99$).
2. **The training set is large enough.** LSTMs have four weight matrices per layer, requiring substantial data to train reliably. For $N$ hidden units, an LSTM has $\sim 4N^2 + 4N K + 4N$ parameters versus an ESN's $N$ output parameters.
3. **Gradient information is available.** When the loss function is differentiable and the task can be expressed as supervised learning, the LSTM's trained gating gives it a structural advantage.
4. **Discrete, structured sequences.** Natural language, formal grammars, and other tasks with sharp token boundaries strongly favour LSTMs (and later transformers) over ESNs.

### When LSTMs Lose to ESNs

LSTMs are slower to train (by a factor of $O(N)$ compared to ESNs), require more data, and are harder to analyse theoretically. For real-time processing, small datasets, or physical substrates, ESNs dominate.

---

## 3. Gated Recurrent Units (GRUs)

### Architecture

Cho et al. (2014) proposed a simplified gating mechanism that retains the core insight of LSTMs with fewer parameters:

$$\mathbf{z}_t = \sigma(W_z [\mathbf{h}_{t-1}; \mathbf{u}_t])$$
$$\mathbf{r}_t = \sigma(W_r [\mathbf{h}_{t-1}; \mathbf{u}_t])$$
$$\tilde{\mathbf{h}}_t = \tanh(W [\mathbf{r}_t \odot \mathbf{h}_{t-1}; \mathbf{u}_t])$$
$$\mathbf{h}_t = (1 - \mathbf{z}_t) \odot \mathbf{h}_{t-1} + \mathbf{z}_t \odot \tilde{\mathbf{h}}_t$$

The update gate $\mathbf{z}_t$ replaces both the forget and input gates of the LSTM; the reset gate $\mathbf{r}_t$ controls how much of the previous hidden state is used in computing the candidate. There is no separate cell state.

### Comparison with LSTM

The GRU has approximately $3N^2$ parameters per layer versus $4N^2$ for the LSTM, a 25% reduction. Empirically, on most sequence tasks, GRUs match LSTM performance within noise, though LSTMs maintain an advantage on tasks with very long-range dependencies where the separate cell state provides additional memory capacity.

The GRU can be viewed as a leaky integrator with learned, input-dependent time constants: $\mathbf{h}_t = (1 - \mathbf{z}_t) \odot \mathbf{h}_{t-1} + \mathbf{z}_t \odot \tilde{\mathbf{h}}_t$ is formally identical to the leaky ESN update $\mathbf{x}_t = (1 - \alpha)\mathbf{x}_{t-1} + \alpha \tanh(\cdots)$, but with the leak rate $\mathbf{z}_t$ computed from the input rather than fixed.

### Relationship to Leaky ESNs

The leaky ESN uses a fixed scalar leak rate $\alpha \in (0, 1]$. The GRU replaces this with a learned, input-dependent vector-valued gating $\mathbf{z}_t$. This is precisely the difference between a reservoir (fixed, random dynamics) and a GRU (trained, adaptive dynamics). When the task involves changing dynamical regimes — which require different time constants in different parts of the input — the GRU's learned gating is a significant advantage. When the task is stationary, the fixed reservoir is competitive and far cheaper to train.

---

## 4. Echo State Networks: No Gradient Computation Required

### The Key Innovation

Jaeger (2001) and Maass, Natschläger & Markram (2002, Liquid State Machines) independently recognised that the difficulty of training recurrent weights by gradient descent could be completely bypassed if the recurrent part were fixed and only the readout were trained. This is the reservoir computing paradigm.

The ESN state update:

$$\mathbf{x}_t = \tanh(W^{rec}\mathbf{x}_{t-1} + W^{in}\mathbf{u}_t + \mathbf{b})$$

is run with $W^{rec}, W^{in}$ **frozen**. Only $W^{out}$ is trained, by ridge regression:

$$W^{out} = (X^T X + \lambda I)^{-1} X^T Y$$

This is a convex optimisation problem with a unique solution, solvable in $O(N^3)$ time (versus $O(T \cdot N^2)$ for BPTT over $T$ time steps). It requires no gradient computation through time.

### Why This Works: The Universal Approximation Argument

The echo state network works because a sufficiently large, random, nonlinear reservoir can approximate any fading-memory functional to arbitrary precision, given a sufficiently flexible readout (Maass & Markram 2004). Formally:

**Theorem (Boyd & Chua 1985, Maass & Markram 2004).** Let $F: \ell^\infty(\mathbb{Z}^-) \to \mathbb{R}$ be a causal, time-invariant functional with fading memory. Then for any $\varepsilon > 0$, there exists a reservoir computer $(W^{rec}, W^{in}, W^{out})$ such that $\sup_{\mathbf{u}} |F(\mathbf{u}) - \hat{F}(\mathbf{u})| < \varepsilon$.

The randomness of $W^{rec}$ is not a limitation but a feature: the reservoir provides a diverse library of nonlinear basis functions of the input history, from which the linear readout selects the appropriate combination.

### Computational Advantages

**Speed.** Training an ESN requires: (1) one forward pass to collect states — $O(T N^2)$, and (2) one ridge regression — $O(N^3)$ or $O(TN^2)$ depending on method. Total: $O(T N^2 + N^3)$. Training an LSTM requires $O(T N^2)$ per gradient step, times the number of training epochs. For $T = 10^4$ steps and $N = 500$, ESN training is typically 100-1000x faster than LSTM training.

**Stability.** The ridge regression for $W^{out}$ is a strongly convex problem with a unique global minimum. There are no local minima, saddle points, or vanishing gradients. The training loss decreases monotonically with iteration count (in the exact solver).

**Real-time inference.** The ESN state update is $O(N^2)$ per time step, identical to an LSTM or GRU. At inference time, all architectures have similar computational cost.

**Physical implementation.** The frozen recurrent weights can be realised as a fixed physical system — an optical cavity, a mechanical resonator, a water bucket — while only the output layer requires learning. This enables hardware reservoir computers that operate at physical speed limits (GHz for photonic RC).

### Limitations

**Fixed nonlinear basis.** The random reservoir cannot selectively gate long-range dependencies. For tasks requiring memory spans of $T > -1/\ln(\rho)$ steps, the ESN must be run near $\rho = 1$, which trades stability for memory.

**Poor sample efficiency for complex tasks.** The ESN requires enough training data to estimate $N$ readout weights reliably. For $N = 1000$ and short training sequences ($T_{train} \ll N$), the ridge regression is ill-conditioned regardless of regularisation.

**No representational learning.** The ESN's $W^{rec}$ and $W^{in}$ are fixed at initialisation. Unlike the LSTM, the ESN cannot adapt its internal representations to the structure of the task. For tasks with complex discrete structure (natural language, formal languages), this is a significant disadvantage.

---

## 5. Transformers and the Attention Mechanism

### Architecture

The transformer (Vaswani et al. 2017) abandons recurrence entirely. It processes the entire input sequence in parallel using self-attention:

$$\text{Attention}(Q, K, V) = \text{softmax}\left(\frac{QK^T}{\sqrt{d_k}}\right)V$$

where $Q = XW_Q$, $K = XW_K$, $V = XW_V$ are query, key, and value projections of the input sequence $X \in \mathbb{R}^{T \times d}$.

The self-attention mechanism computes, for each position $t$, a weighted average of all other positions' value vectors, where the weights are determined by the compatibility (dot product) of the query at position $t$ with the keys at all positions. This is an $O(T^2 d)$ operation per layer.

### What Transformers Do Differently

Unlike RNNs, transformers have **no notion of time or distance**. The attention weights for position $t = 100$ to position $t' = 1$ and to position $t' = 99$ are computed by the same mechanism. There is no built-in notion of recency or fading memory. This is both a strength (no vanishing gradient over sequence length) and a limitation (requires positional encodings to distinguish order, quadratic in sequence length).

From a reservoir computing perspective, a transformer layer is a (learned) nonlinear mixing operation on the sequence, followed by a pointwise feedforward network. There is no recurrent state; instead, all information propagates through the attention mechanism.

### When Transformers Win

Transformers dominate when:
1. **Sequence length is moderate** (up to $T \approx 100K$ with efficient attention variants). They can attend to any position with equal computational cost, unlike RNNs that must propagate information through time.
2. **Parallel training is available.** The entire sequence can be processed in a single forward pass, enabling massive parallelism on GPU/TPU hardware.
3. **Large datasets exist.** Transformers are data-hungry; pretraining on billions of tokens dramatically improves performance.
4. **The task involves long-range discrete dependencies.** Coreference resolution in language, document-level question answering, and protein structure prediction all require long-range reasoning that transformers handle naturally.

### When ESNs Still Compete with Transformers

Despite the rise of transformers, echo state networks remain competitive in several regimes:

1. **Real-time streaming inference.** A transformer must buffer the entire context window to compute attention; the ESN state $\mathbf{x}_t$ is updated with $O(N^2)$ operations per step, regardless of history length.

2. **Continuous-time, analogue signals.** Transformers are designed for discrete tokens. Physical reservoir computers can process continuous-time analogue signals (optical, acoustic, mechanical) without discretisation.

3. **Edge computing and low-power devices.** Transformer inference requires storing the full KV cache; ESN inference requires only $O(N)$ memory for the current state.

4. **Scientific time series with unknown structure.** For chaotic time series prediction, reservoir computing achieves results comparable to transformers (e.g., Lorenz VPT of 5-6 Lyapunov times) with orders of magnitude less compute.

5. **Few-shot or no-training scenarios.** A random reservoir provides a useful feature representation without any training. A transformer requires significant compute just for inference.

---

## 6. Computational Efficiency Comparison

The following table summarises the asymptotic computational costs for a network with $N$ hidden units processing a sequence of length $T$ with $K$-dimensional input:

| Architecture | Training time | Inference per step | Memory (training) | Parameters |
|-------------|---------------|-------------------|-------------------|------------|
| Vanilla RNN | $O(T \cdot N^2 \cdot E)$ | $O(N^2)$ | $O(T \cdot N)$ | $O(N^2 + NK + NM)$ |
| LSTM | $O(T \cdot N^2 \cdot E)$ | $O(N^2)$ | $O(T \cdot N)$ | $O(4N^2 + 4NK + 4N)$ |
| GRU | $O(T \cdot N^2 \cdot E)$ | $O(N^2)$ | $O(T \cdot N)$ | $O(3N^2 + 3NK + 3N)$ |
| ESN | $O(TN^2 + N^3)$ | $O(N^2)$ | $O(T \cdot N)$ | $O(NM)$ |
| Transformer | $O(T^2 d L)$ | $O(T d L)$ | $O(T^2 + Td)$ | $O(L d^2)$ |

Here $E$ = number of training epochs, $d$ = model dimension, $L$ = number of layers.

Key observations:
- ESN training is $O(E)$ faster than gradient-based methods (no epochs needed).
- Transformer inference scales with context length $T$; all RNN-style architectures are $O(1)$ in $T$.
- ESN parameter count grows only as $O(NM)$ rather than $O(N^2)$ for trained architectures.

---

## 7. Mathematical Analysis of Expressivity: ESN Capacity vs. LSTM Capacity

### ESN Capacity Bound

The **memory capacity** (MC) of a linear ESN is bounded above by its reservoir size:

$$MC = \sum_{k=1}^{\infty} MC_k \leq N$$

where $MC_k = R^2_k$ is the coefficient of determination of the optimal linear predictor of the input $k$ steps ago from the current reservoir state. This bound is tight: a linear ESN driven by white noise achieves $MC = N$ (Jaeger 2002).

For a nonlinear (tanh) ESN, the total MC is typically less than $N$ because some representational capacity is consumed by nonlinear mixing. The remaining capacity is available for **nonlinear** input-output functions, but cannot be precisely quantified in terms of a single number.

The **separation property** (Maass & Markram 2004) provides a different expressivity measure: the reservoir $\mathbf{x}_t = F(\mathbf{u}_{t:t-\infty})$ separates input histories $\mathbf{u}^{(1)} \neq \mathbf{u}^{(2)}$ if and only if the infinite-history functions are distinguishable. For a generic (full-rank, dense) random reservoir, the separation property holds for any two histories that differ at some finite time in the past.

### LSTM Capacity

LSTMs, being Turing complete, have in principle unbounded computational power given sufficient hidden dimension. The practical capacity for a given $N$ is determined by the richness of the gating functions the network can learn. An LSTM with $N = 100$ units has $\sim 40000$ trainable parameters; an ESN with $N = 100$ has $100 M$ readout parameters (where $M$ is the output dimension). The LSTM's extra parameters are used to learn task-specific gating, which is the source of its advantage on complex structured tasks.

**For chaotic time series:** The ESN's random reservoir implicitly approximates the required nonlinear basis functions via the random kitchen-sink argument (Rahimi & Recht 2007). The LSTM must learn these basis functions from data. For low-dimensional chaotic systems (Lorenz, Mackey-Glass), the random reservoir is nearly as expressive as the learned LSTM representation, at a fraction of the training cost.

---

## 8. A Decision Framework

Given a new sequence learning problem, the following decision framework guides architecture selection.

### Step 1: What is the required memory span?

- **Short memory ($T_{mem} < 20$ steps):** Any architecture works. Prefer ESN for speed, vanilla RNN for simplicity.
- **Medium memory (20 to 100 steps):** ESN with $\rho \in [0.9, 0.99]$ is competitive with LSTM. Compare empirically.
- **Long memory ($> 100$ steps):** LSTM or GRU preferred. ESN requires $\rho \to 1$, degrading stability.
- **Arbitrary memory (formal languages, document understanding):** Transformer or LSTM. ESNs are inappropriate.

### Step 2: What is the training data volume?

- **Very small ($T_{train} < 500$ steps):** ESN strongly preferred (requires only $N$ parameters to fit). LSTM will overfit.
- **Moderate (500 to 10K steps):** ESN and LSTM competitive depending on task.
- **Large ($> 10K$ steps):** LSTM and transformer become attractive, especially if the task is complex.

### Step 3: Is real-time or physical processing required?

- **Real-time streaming (latency per step):** Any RNN-style architecture. Transformer is prohibitive at $O(T)$ per step.
- **Physical substrate (hardware reservoir computing):** ESN is the only option. The reservoir is a physical device; only the readout is trained.
- **Batch processing (offline):** All architectures viable; transformer dominates on large datasets.

### Step 4: Is the task structure known?

- **Known polynomial structure (e.g., chaotic ODE):** NVAR (polynomial reservoir) or ESN. Transformer likely overkill.
- **Unknown nonlinear structure, continuous signals:** ESN is the most appropriate starting point.
- **Discrete structured sequences (language, code, formal grammars):** Transformer or LSTM. ESN is structurally limited.

### Summary Table

| Criterion | ESN | LSTM | GRU | Transformer |
|-----------|-----|------|-----|-------------|
| Short memory | ✓✓ | ✓✓ | ✓✓ | ✓✓ |
| Long memory | ✗ | ✓✓ | ✓✓ | ✓✓ |
| Small dataset | ✓✓ | ✗ | ✗ | ✗✗ |
| Real-time inference | ✓✓ | ✓ | ✓ | ✗ |
| Physical implementation | ✓✓ | ✗✗ | ✗✗ | ✗✗ |
| Training speed | ✓✓ | ✗ | ✗ | ✗ |
| Chaotic time series | ✓✓ | ✓ | ✓ | ✗ |
| Discrete sequences | ✗ | ✓✓ | ✓✓ | ✓✓ |

---

## 9. Combining Reservoirs with Modern Architectures

The apparent opposition between reservoirs and trained architectures is dissolving in recent work. Several hybrid approaches have emerged.

### Reservoir-Augmented Transformers

Several papers (2021-2024) have proposed using a fixed random reservoir as a positional encoding replacement or as a source of multi-scale temporal features for transformer inputs. The reservoir provides inductive biases about timescale and temporal locality that transformers lack natively. This is particularly useful for long time-series transformers where the quadratic attention cost is mitigated by providing reservoir states instead of raw inputs.

### Trained Reservoirs (Echo State Networks as Initialisers)

Gallicchio & Micheli (2017) showed that the random initialisation strategy of ESNs provides a particularly good starting point for gradient-based training. "Contractiveness" analysis shows that an ESN with $\rho < 1$ satisfies the conditions for stable gradient flow that orthogonal initialisation attempts to achieve but at a lower computational cost to verify.

### Reservoir Computing Meets State Space Models

The modern S4 (Gu et al. 2021) and Mamba (Gu & Dao 2023) architectures are, at their core, learned structured state space models — essentially LSTMs whose recurrent matrices are constrained to diagonal-plus-low-rank form for computational efficiency. The reservoir computing insight — that fixed, structured recurrent matrices can provide rich temporal bases — directly informed the design of these architectures. S4 uses a fixed random Hippo matrix (a structured matrix designed to approximate all monomials of the input history); this is philosophically identical to a carefully designed reservoir.

### Physical Reservoir + Digital Readout

The most immediate hybrid is the physical reservoir computer: a physical dynamical system (optical cavity, mechanical array, neuromorphic chip) provides the nonlinear temporal mixing, and a digital trained network handles the readout. This combination achieves the energy efficiency of analogue computing for the compute-intensive state evolution, while retaining the flexibility of digital computation for the task-specific output layer.

---

## 10. Historical Summary

| Year | Event | Significance |
|------|-------|--------------|
| 1989 | Williams & Zipser: BPTT | First systematic training algorithm for RNNs |
| 1994 | Bengio, Simard, Frasconi | Proved vanishing gradient as structural problem |
| 1997 | Hochreiter & Schmidhuber: LSTM | Gating mechanism bypasses vanishing gradient |
| 1998 | LeCun et al.: gradient clipping | Practical solution for exploding gradients |
| 2001 | Jaeger: Echo State Networks | Fixed reservoir, trained readout only |
| 2002 | Maass et al.: Liquid State Machines | Independent reservoir computing discovery, biological motivation |
| 2002 | Jaeger: Memory capacity bound | $MC \leq N$ — fundamental limit proved |
| 2014 | Cho et al.: GRU | Simplified LSTM with competitive performance |
| 2017 | Vaswani et al.: Transformer | Attention replaces recurrence for sequence modelling |
| 2021 | Gauthier et al.: NVAR | Polynomial features match ESN on Lorenz with D=27 |
| 2022 | Gu et al.: S4 | Structured state space models — reservoir insight in deep learning |
| 2023 | Gu & Dao: Mamba | Selective state space model with hardware-efficient implementation |

The convergence visible in this table is significant: the deep learning community, after a decade of transformer dominance, is rediscovering the importance of inductive biases about time — the same biases that motivated reservoir computing in 2001. The circle is closing.

---

## References

- Bengio, Y., Simard, P., & Frasconi, P. (1994). Learning long-term dependencies with gradient descent is difficult. *IEEE Transactions on Neural Networks*, 5(2), 157–166.
- Boyd, S., & Chua, L. (1985). Fading memory and the problem of approximating nonlinear operators with Volterra series. *IEEE Transactions on Circuits and Systems*, 32(11), 1150–1161.
- Cho, K., van Merriënboer, B., Gulcehre, C., Bahdanau, D., Bougares, F., Schwenk, H., & Bengio, Y. (2014). Learning phrase representations using RNN encoder–decoder for statistical machine translation. *EMNLP 2014*.
- Gallicchio, C., & Micheli, A. (2017). Echo state property of deep reservoir computing networks. *Cognitive Computation*, 9(3), 337–350.
- Gauthier, D. J., Bollt, E., Griffith, A., & Barbosa, W. A. S. (2021). Next generation reservoir computing. *Nature Communications*, 12, 5564.
- Gu, A., Goel, K., & Ré, C. (2022). Efficiently modeling long sequences with structured state spaces. *ICLR 2022*.
- Gu, A., & Dao, T. (2023). Mamba: Linear-time sequence modeling with selective state spaces. *arXiv:2312.00752*.
- Hochreiter, S., & Schmidhuber, J. (1997). Long short-term memory. *Neural Computation*, 9(8), 1735–1780.
- Jaeger, H. (2001). The echo state approach to analysing and training recurrent neural networks. *GMD Report 148*. German National Research Center for Information Technology.
- Jaeger, H. (2002). Short-term memory in echo state networks. *GMD Report 152*.
- Maass, W., Natschläger, T., & Markram, H. (2002). Real-time computing without stable states: A new framework for neural computation based on perturbations. *Neural Computation*, 14(11), 2531–2560.
- Maass, W., & Markram, H. (2004). On the computational power of circuits of spiking neurons. *Journal of Computer and System Sciences*, 69(4), 593–616.
- Rahimi, A., & Recht, B. (2007). Random features for large-scale kernel machines. *NeurIPS 20*.
- Siegelmann, H. T., & Sontag, E. D. (1995). On the computational power of neural nets. *Journal of Computer and System Sciences*, 50(1), 132–150.
- Vaswani, A., Shazeer, N., Parmar, N., Uszkoreit, J., Jones, L., Gomez, A. N., Kaiser, L., & Polosukhin, I. (2017). Attention is all you need. *NeurIPS 30*.
- Williams, R. J., & Zipser, D. (1989). A learning algorithm for continually running fully recurrent neural networks. *Neural Computation*, 1(2), 270–280.

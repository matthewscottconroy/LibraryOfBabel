# Transformers vs. Reservoirs: A Principled Comparison

## 30.1.1 Two Architectures for Sequential Data

The transformer [Vaswani et al. 2017] and the echo state network represent two fundamentally different approaches to sequential computation. Both take sequences as input and produce outputs; both have been applied successfully to temporal prediction, language modeling, and dynamical system emulation. Yet their architectural principles are nearly opposite:

- The **transformer** processes the entire input sequence at once, computes attention over all positions, and has no recurrence.
- The **reservoir** processes the input sequence one step at a time, maintains a fixed-size state, and has strong recurrence but no learned internal weights.

Understanding when each architecture is appropriate, and what mathematical relationship connects them, is essential for situating reservoir computing within the modern deep learning landscape.

## 30.1.2 Transformer Architecture

The core operation of the transformer is **scaled dot-product self-attention** [Vaswani et al. 2017]. For an input sequence $X = [x_1, \ldots, x_T] \in \mathbb{R}^{T \times d}$, the attention output is:

$$
\mathrm{Attention}(Q, K, V) = \mathrm{softmax}\!\left(\frac{QK^T}{\sqrt{d_k}}\right)V,
$$

where $Q = XW_Q$, $K = XW_K$, $V = XW_V$ are learned linear projections (query, key, value matrices). The output at position $t$ is a weighted average of the values at all positions:

$$
y_t = \sum_{s=1}^T \alpha_{ts} v_s, \quad \alpha_{ts} = \frac{\exp(q_t^T k_s / \sqrt{d_k})}{\sum_{s'}\exp(q_t^T k_{s'}/\sqrt{d_k})}.
$$

For causal (autoregressive) transformers, the attention is masked: $\alpha_{ts} = 0$ for $s > t$.

**Complexity.** Self-attention requires $O(T^2 d)$ computation and $O(T^2)$ memory (for the attention matrix). This quadratic cost in sequence length $T$ is the transformer's primary scaling limitation.

## 30.1.3 Reservoir Architecture

The ESN processes input sequentially:

$$
\mathbf{x}(t) = \tanh\!\left(W^{\text{rec}}\mathbf{x}(t-1) + W^{\text{in}}u(t)\right), \quad y(t) = \mathbf{w}^T\mathbf{x}(t).
$$

Complexity: $O(N^2)$ per time step for the reservoir state update, $O(N)$ for the readout. Total cost over $T$ steps: $O(N^2 T)$.

## 30.1.4 Memory: The Fundamental Difference

The most important architectural difference is in how the two models handle memory:

**Transformer:** Self-attention with causal masking gives the model access to **all previous tokens** within the context window of length $T_{\max}$. Token $t$ can attend to any token $s \leq t$, with attention weight $\alpha_{ts}$ determined by the similarity of $q_t$ and $k_s$. Memory is **perfect within the context window** and zero outside it.

**Reservoir:** The ESN state $\mathbf{x}(t)$ is a finite-dimensional summary of the input history. By the fading memory property, the influence of $u(t-k)$ on $\mathbf{x}(t)$ decays approximately as $\rho^k$ (where $\rho$ is the spectral radius). Memory is **exponentially fading** but **unbounded in principle** (the whole history is accessible, just progressively less so).

**Which is better?** Neither is universally superior. For tasks requiring long-range attention (e.g., understanding a reference made at the beginning of a long document), the transformer's uniform access to the full context window is advantageous. For tasks with smooth temporal dependencies (e.g., physical dynamics, audio signals, control), the reservoir's exponential memory is appropriate and much cheaper.

## 30.1.5 Transformers as Associative Memory

[Ramsauer et al. 2021] showed that the modern Hopfield network (with exponential storage capacity) is equivalent to the attention mechanism of transformers. Specifically, the attention update

$$
y = V\,\mathrm{softmax}(K^T q)
$$

can be interpreted as a **single step of energy minimization** in a Hopfield network with stored patterns $K$ and query $q$. This equivalence reveals that transformers implement a form of **content-addressable memory**: the query $q_t$ retrieves the most relevant stored pattern from $K$.

In contrast, reservoir computing implements **time-ordered memory**: past states are retrievable based on their temporal position (via the fading memory structure), not their content similarity to the current query.

## 30.1.6 Linear Attention and the Recurrent Form

A crucial bridge between transformers and reservoirs is **linear attention** [Katharopoulos et al. 2020], obtained by replacing the softmax in attention with a kernel function $\phi$:

$$
y_t = \frac{\sum_{s=1}^t \phi(q_t)^T \phi(k_s) v_s}{\sum_{s=1}^t \phi(q_t)^T \phi(k_s)}.
$$

Writing $S_t = \sum_{s=1}^t \phi(k_s) v_s^T$ and $z_t = \sum_{s=1}^t \phi(k_s)$, the linear attention can be computed **recurrently**:

$$
S_t = S_{t-1} + \phi(k_t) v_t^T, \quad z_t = z_{t-1} + \phi(k_t), \quad y_t = S_t \phi(q_t) / (z_t^T \phi(q_t)).
$$

This is a linear recurrent system! The state $(S_t, z_t)$ is the "reservoir state" of the linearized transformer. The linear attention approximation therefore recovers a form of reservoir computing from the transformer, at the cost of approximation quality for long-range attention.

This connection suggests a continuum between reservoirs and transformers, parameterized by the degree to which attention is "localized" (short-range: reservoir-like) vs. "global" (long-range: transformer-like).

## 30.1.7 Comparison Summary

| Property | Transformer | Reservoir |
|---|---|---|
| Training | Full backprop (expensive) | Readout only (ridge regression) |
| Memory | $O(1)$ per token after context | Exponentially fading |
| Context window | Fixed $T_{\max}$; perfect recall within | Unlimited; fading beyond $\sim 1/\lambda$ |
| Parallelism | Full (training on entire sequence) | Sequential (step-by-step) |
| Compute per step | $O(T)$ (attention over history) | $O(N^2)$ (matrix multiply) |
| Streaming / online | Poor (must buffer context) | Natural |
| Theoretical guarantees | Limited (empirical mostly) | Universal approximation (Boyd-Chua) |
| Scale | Billions of parameters | Thousands to millions of neurons |

## 30.1.8 When Does the Reservoir Win?

1. **Streaming, online computation.** The reservoir processes inputs as they arrive; the transformer requires buffering a context window.

2. **Resource-constrained settings.** A physical reservoir (optical, mechanical, electrochemical) can perform computation at the speed of the physical substrate with zero power for inference, compared to the megawatts consumed by large transformers.

3. **Systems with smooth temporal structure.** Physical dynamics (fluids, electronics, neural circuits) have smooth, decaying temporal correlations — exactly what the reservoir is designed for. Transformers may overfit the "memory" to non-existent long-range structure.

4. **Tasks requiring interpretability.** The linear readout of the reservoir provides a direct, interpretable mapping from reservoir state features to outputs. Transformer attention maps provide limited interpretability.

## References

- Katharopoulos, A., Vyas, A., Pappas, N., and Fleuret, F. (2020). Transformers are RNNs: Fast autoregressive transformers with linear attention. In *Proceedings of the 37th ICML*, 5156–5165.
- Ramsauer, H., Schäfl, B., Lehner, J., et al. (2021). Hopfield networks is all you need. In *International Conference on Learning Representations*.
- Vaswani, A., Shazeer, N., Parmar, N., et al. (2017). Attention is all you need. In *Advances in Neural Information Processing Systems*, 30.

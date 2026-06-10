# 1.1.3 Fixed Context Versus Adaptive Memory

## The Design-Time Commitment

Every feedforward architecture that attempts to handle temporal data makes a silent commitment at design time: it decides, before seeing a single data point, how much of the past to look at. This is the window size $W$, and it is fixed.

This commitment is a form of inductive bias — a prior assumption about the structure of the task. For some tasks, this assumption is well-matched. For others, it is a procrustean bed that either leaves relevant past information on the table (if $W$ is too small) or wastes capacity on irrelevant ancient history (if $W$ is too large).

**Adaptive memory**, by contrast, is a memory system that adjusts what it remembers based on what the task requires. It does not discard the past at a fixed horizon; it lets the past fade at a rate determined by its relevance. A skilled human listener, for example, remembers the beginning of a sentence for exactly as long as it takes to resolve the sentence's syntactic structure — no longer, but no shorter.

The question is: how do we build a machine that does this?

## What Fixed Context Can Model

A fixed-context system — any system that looks at a window of size $W$ — can in principle model any task whose relevant memory fits within $W$. If the longest dependency in the task is $D$ time steps, then $W \geq D$ is sufficient. The feedforward network then learns which lags matter and how to weight them.

This is not nothing. Many tasks have short, well-defined memory requirements. ARMA time series have exact, finite-order representations. Some classification tasks depend only on the most recent observation. For these tasks, the sliding window is appropriate and efficient.

The difficulty is that:
1. We rarely know $D$ in advance.
2. $D$ varies across instances within the same task.
3. The most interesting temporal tasks have $D$ that is effectively infinite (or at least very large compared to what is computationally tractable).

## The Failure Mode: Aliasing in Time

A subtle but important failure mode of fixed-context systems deserves attention: **temporal aliasing**.

Consider a system where two different histories lead to the same window content. For example:

$$\mathbf{x}_t = [u_t, u_{t-1}, \ldots, u_{t-W+1}] = [0.5, 0.3, 0.7, \ldots]$$

This window content is the same regardless of what happened at time $t-W-1, t-W-2, \ldots$ — even if those distant events are highly relevant to the current output. The feedforward network cannot distinguish between a trajectory that had $u_{t-W-1} = 10$ (a large spike) and one that had $u_{t-W-1} = 0.01$, because both produce identical windows.

This is a strict limitation: the window creates an equivalence class over all histories that agree on the last $W$ steps, regardless of what they did before. For tasks sensitive to events beyond $W$ steps ago, this equivalence class contains examples that should have different outputs — and no learning algorithm can separate them from the given input representation.

## Adaptive Memory in Practice

What would truly adaptive memory look like? Several desiderata:

**1. Variable effective horizon.** The system should "remember" long-range events when they are relevant and "forget" them when they are not. This requires the memory to be task-sensitive, not just input-sensitive.

**2. Compression of the past.** Rather than storing every past input verbatim, the system should maintain a compressed summary of the past that preserves task-relevant information and discards the rest. This is the concept of a **sufficient statistic** for the temporal prediction problem.

**3. Graceful decay.** When information from the past is no longer relevant, it should fade naturally rather than be abruptly cut off. Hard window boundaries create artifacts — the system's representation changes discontinuously as events cross the window boundary.

**4. Online adaptability.** The memory representation should be computable incrementally: given $\mathbf{x}_t$ and the new input $u_{t+1}$, the system should compute $\mathbf{x}_{t+1}$ in $O(N)$ time without reprocessing the entire history.

All four of these desiderata are met by a well-designed recurrent system — whether a trained RNN, an LSTM, or a reservoir. The state $\mathbf{x}_t$ is a compressed summary of the past that evolves incrementally. The rate at which information decays from the state is controlled by the system's dynamics, not fixed externally. And the state can, in principle, encode long-range dependencies if the dynamics are appropriately chosen.

## The Continuum Between Fixed and Adaptive

It is worth noting that fixed and adaptive memory are not a binary choice but endpoints of a continuum.

- **Pure fixed context ($W$ steps):** The most restrictive. No decay, hard cutoff.
- **Exponential decay:** $x_t = (1-\alpha) x_{t-1} + \alpha u_t$. Soft cutoff, geometric decay. Simple, online, but no adaptability to task structure.
- **ARIMA models:** Adaptive in the sense that the decay rate is estimated from data, but linear and with fixed order.
- **Reservoir computers:** Nonlinear mixing of past inputs with a decay rate determined by reservoir dynamics. The effective memory depends on the spectral radius. Can represent long-range and nonlinear dependencies.
- **Trained RNNs (LSTM, GRU):** Fully adaptive — the gating mechanisms learn task-specific forgetting rates from data. Maximum flexibility, but require end-to-end training.

Reservoir computing occupies a middle ground that turns out to be enormously practical: the dynamics are fixed (not trained), but rich enough to represent a wide variety of temporal dependencies. The training burden falls entirely on the linear readout, which is trivial. This is the key insight we will develop throughout the book.

---

## References

- [Rumelhart1986] Rumelhart, D.E., Hinton, G.E., & Williams, R.J. (1986). Learning representations by back-propagating errors. *Nature*, 323(6088), 533–536.
- [Hochreiter1997] Hochreiter, S. & Schmidhuber, J. (1997). Long Short-Term Memory. *Neural Computation*, 9(8), 1735–1780.
- [Jaeger2001] Jaeger, H. (2001). The 'echo state' approach to analysing and training recurrent neural networks. GMD Technical Report 148.

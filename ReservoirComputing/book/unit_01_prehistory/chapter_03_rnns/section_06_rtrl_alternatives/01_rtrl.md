# 3.6 RTRL and Online Gradient Alternatives

## The BPTT Bottleneck

Backpropagation through time (Section 3.3) computes exact gradients but requires storing the full computational history — all activations at all time steps — before performing updates. This is **offline** learning: the entire sequence must be observed before any weights change. For long sequences or streaming data, this is impractical.

The question is whether gradients can be computed **online** — one step at a time, as the sequence unfolds — without storing the full history.

The answer is yes, and the algorithm is **Real-Time Recurrent Learning (RTRL)** [Williams1989]. The catch is that RTRL is computationally expensive: its per-step cost is $O(N^4)$ in the number of neurons, compared to $O(N^2)$ for BPTT averaged over $T$ steps. This cost has largely prevented RTRL from being practical for large networks, but it remains theoretically important and has recently inspired more efficient approximations.

## Forward-Mode Automatic Differentiation

BPTT uses **reverse-mode** automatic differentiation: it propagates gradients backward from loss to parameters. RTRL uses **forward-mode** differentiation: it tracks how parameter perturbations propagate forward through time.

**The key object:** Define the sensitivity matrix $\mathbf{S}_t^{ij} = \frac{\partial x_t^i}{\partial W^{jk}} \cdot \text{(some index)}$... more precisely, for each parameter $\theta_k$, define:

$$\mathbf{p}_t^{(k)} = \frac{\partial \mathbf{x}_t}{\partial \theta_k} \in \mathbb{R}^N$$

The collection $\{\mathbf{p}_t^{(k)}\}$ for all parameters $\theta_k$ is the **sensitivity state** — it tracks how the current hidden state responds to infinitesimal changes in each parameter.

For the standard RNN $\mathbf{x}_t = \tanh(W^{rec} \mathbf{x}_{t-1} + W^{in} \mathbf{u}_t)$, let $\theta = W^{rec}_{pq}$ (a single recurrent weight). The sensitivity satisfies the recurrence:

$$\frac{\partial \mathbf{x}_t}{\partial W^{rec}_{pq}} = D_t \left( W^{rec} \frac{\partial \mathbf{x}_{t-1}}{\partial W^{rec}_{pq}} + \mathbf{e}_p x^q_{t-1} \right)$$

where $D_t = \text{diag}(\tanh'(W^{rec}\mathbf{x}_{t-1} + W^{in}\mathbf{u}_t))$ is the diagonal Jacobian of the activation function, and $\mathbf{e}_p$ is the $p$-th standard basis vector.

This recurrence is initialized as $\partial \mathbf{x}_0 / \partial W^{rec}_{pq} = \mathbf{0}$ and updated **at every time step** — hence "real-time." The gradient of the loss with respect to each parameter can then be computed immediately:

$$\frac{\partial \mathcal{L}_t}{\partial W^{rec}_{pq}} = \frac{\partial \mathcal{L}_t}{\partial \mathbf{x}_t}^\top \frac{\partial \mathbf{x}_t}{\partial W^{rec}_{pq}}$$

The parameter update can happen after each step: $W^{rec} \leftarrow W^{rec} - \eta \nabla_{W^{rec}} \mathcal{L}_t$.

## Computational Cost Analysis

The sensitivity state $\{\partial \mathbf{x}_t / \partial \theta_k\}$ has one $N$-dimensional vector per parameter. For a fully connected RNN with $N$ neurons:

- Number of recurrent parameters: $N^2$
- Size of sensitivity state: $N^2$ vectors of size $N = N^3$ scalars
- Cost to update sensitivity state at each step: $O(N^4)$ (matrix-vector product for each of the $N^2$ parameters)
- Cost per step for BPTT (amortized): $O(N^2)$

This $O(N^4)$ cost is catastrophic for large $N$. For $N = 100$, RTRL requires $10^8$ operations per time step, compared to $10^4$ for BPTT. For $N = 1000$, RTRL needs $10^{12}$ per step.

In practice, this means RTRL is only feasible for small networks (typically $N \leq 50$) or for approximate variants that reduce the sensitivity state.

## Equivalence to BPTT

Despite the very different computational structure, RTRL and BPTT compute exactly the same gradients [Williams1989]. The difference is purely algorithmic:

| Property | BPTT | RTRL |
|----------|------|------|
| Mode | Reverse (backward) | Forward |
| When gradients available | After full sequence | After each step |
| Storage | $O(NT)$ activations | $O(N^3)$ sensitivity state |
| Cost per step (amortized) | $O(N^2)$ | $O(N^4)$ |
| Online learning | No | Yes |
| Exact gradients | Yes | Yes |
| Truncation | Truncated BPTT (approx.) | Natural truncation (exact) |

The fundamental tension: forward-mode is online but expensive; reverse-mode is cheap but offline. A middle path — truncated BPTT — runs BPTT on fixed-length windows, giving approximate online learning at $O(N^2)$ cost per step. Truncated BPTT is the dominant practical algorithm, at the cost of discarding gradients older than the window length.

## Approximate RTRL Algorithms

Recent work has sought to approximate RTRL's online exact gradients at reduced cost, motivated partly by biological plausibility concerns (brains do not run backward passes) and partly by the desire for online learning in streaming applications.

**Random feedback alignment** [Lillicrap2016]: Replace the backward-pass weight matrix with a fixed random matrix. Biologically motivated and surprisingly effective, though gradients are approximate.

**Unbiased Online Recurrent Optimization (UORO)** [Tallec2017]: Approximates the sensitivity matrix as a rank-1 outer product $\mathbf{a} \mathbf{b}^\top$, updated online with random sign flips to maintain unbiasedness. Cost: $O(N^2)$ per step. The approximation is noisy but unbiased — variance decreases with more neurons.

**Kernel-based RNN (KBNN)** and **Local Learning Rules**: Several approaches decompose the sensitivity propagation to only use local information (presynaptic activity, postsynaptic activity, and a third factor), achieving biologically plausible updates.

**e-prop** [Bellec2020]: A theoretically motivated approximation to RTRL that retains only the locally accessible portion of the sensitivity state — the part computable from local synaptic information. e-prop is local in space (each synapse updates using only its own pre- and postsynaptic signals) and online in time.

## Biological Plausibility Analysis

The vanishing gradient problem and the BPTT algorithm raise fundamental questions about how biological neural networks could learn. BPTT requires:

1. **A backward pass:** Error signals must flow backward through the network, in the opposite direction to forward propagation. There is no known biological mechanism for this.
2. **Weight transport:** The backward pass uses the same weights as the forward pass, transposed. Synapses in the brain are directional; retrograde propagation of error using the same weights requires a precise symmetry with no known biological basis [Grossberg1987].
3. **Temporal nonlocality:** BPTT computes gradients using activations from the entire past. Neurons cannot store their own activation history over long timescales.

RTRL partially addresses (3) by being online, but retains the problems of non-locality in space (the full sensitivity matrix involves all-to-all interactions) and the backward weight transport problem.

**What would biological plausibility require?**

A biologically plausible learning rule must be:
- **Local in space:** The update to synapse $(i, j)$ depends only on the activity of neurons $i$ and $j$, plus a global modulatory signal (like a reward signal or neuromodulator).
- **Local in time:** Updates depend on recent activity, not the entire past trajectory.
- **Online:** Updates happen continuously, not after a full forward-backward pass.
- **Causal:** Learning depends on signals that are available in real time, not on future information.

Classic Hebbian learning $\Delta W_{ij} = \eta x_i x_j$ satisfies all four criteria, but does not implement error backpropagation.

**e-prop** [Bellec2020] is the most principled current approach: it approximates RTRL using only local signals plus a global "learning signal" (analogous to a neuromodulatory broadcast). The key insight is that the full RTRL sensitivity matrix can be decomposed into a locally computable "eligibility trace" plus a non-local component. e-prop discards the non-local component, resulting in approximate gradients that are nonetheless sufficient for learning many tasks.

**The three-factor learning rule:** The most common formulation of biologically plausible learning involves three factors:

$$\Delta W_{ij} = \eta \cdot \underbrace{e_{ij}(t)}_{\text{eligibility trace}} \cdot \underbrace{m(t)}_{\text{modulator}}$$

The eligibility trace $e_{ij}(t)$ is computed locally from pre- and postsynaptic activity and captures recent correlation. The modulator $m(t)$ is a global signal (reward, prediction error, or a teacher signal) that gates plasticity. This structure maps onto known neuroscience: dopaminergic reward signals modulate synaptic plasticity that has been "tagged" by recent correlated activity [Redondo2011, Schultz1997].

## Reservoir Computing as the Biological Alternative

The biological plausibility problems of BPTT are one motivation for reservoir computing, though not the primary one historically.

If the recurrent weights are fixed (as in an ESN or LSM), the only weights to learn are the readout weights $W^{out}$. The readout update is:

$$\Delta W^{out}_i = \eta \cdot e_t \cdot x_t^i$$

where $e_t = y_t^* - \hat{y}_t$ is the output error and $x_t^i$ is the $i$-th reservoir state — a perfectly local, online, three-factor rule. No backward pass, no weight transport, no temporal non-locality.

Maass, Natschläger, and Markram [Maass2002] argued explicitly that LSMs provide a model for cortical computation in which the readout corresponds to downstream neurons that receive and linearly combine the outputs of a fixed cortical microcircuit. This is the biological reading of reservoir computing: the cortex is the reservoir, and learning happens in the output connections, not inside the reservoir.

Whether this is accurate neuroscience is a separate question — one engaged throughout Unit VII. But as a theoretical framework, it transforms the learning problem from an intractable one (online training of recurrent weights) to a tractable one (online linear regression), while retaining the full nonlinear temporal processing power of the recurrent substrate.

---

## References

- [Williams1989] Williams, R.J. & Zipser, D. (1989). A learning algorithm for continually running fully recurrent neural networks. *Neural Computation*, 1(2), 270–280. **[The original RTRL paper. Full derivation, proof of equivalence to BPTT.]**
- [Werbos1990] Werbos, P.J. (1990). Backpropagation through time: What it does and how to do it. *Proceedings of the IEEE*, 78(10), 1550–1560. **[BPTT formal derivation and tutorial.]**
- [Tallec2017] Tallec, C., Ollivier, Y., & Charpiat, G. (2017). Unbiased online recurrent optimization. *arXiv:1702.05043*. **[UORO: $O(N^2)$ unbiased approximation to RTRL.]**
- [Bellec2020] Bellec, G. et al. (2020). A solution to the learning dilemma for recurrent networks of spiking neurons. *Nature Communications*, 11, 3625. **[e-prop: biologically plausible RTRL approximation with eligibility traces.]**
- [Lillicrap2016] Lillicrap, T.P. et al. (2016). Random synaptic feedback weights support error backpropagation for deep learning. *Nature Communications*, 7, 13276. **[Feedback alignment: random backward weights work.]**
- [Grossberg1987] Grossberg, S. (1987). Competitive learning: From interactive activation to adaptive resonance. *Cognitive Science*, 11(1), 23–63. **[Early analysis of biological implausibility of backpropagation.]**
- [Maass2002] Maass, W., Natschläger, T., & Markram, H. (2002). Real-time computing without stable states: A new framework for neural computation based on perturbations. *Neural Computation*, 14(11), 2531–2560. **[LSMs as biologically plausible reservoir computing.]**
- [Redondo2011] Redondo, R.L. & Morris, R.G.M. (2011). Making memories last: The synaptic tagging and capture hypothesis. *Nature Reviews Neuroscience*, 12(1), 17–30. **[Synaptic tagging and three-factor learning rules — the neuroscience basis for eligibility traces.]**
- [Schultz1997] Schultz, W., Dayan, P., & Montague, P.R. (1997). A neural substrate of prediction and reward. *Science*, 275(5306), 1593–1599. **[Dopaminergic reward prediction error signals — the biological modulator in three-factor rules.]**

# Why Go Deep with Reservoirs?

## The Single-Reservoir Ceiling

A single-layer echo state network with $N$ neurons and leak rate $\alpha$ processes input through a characteristic effective time constant of approximately

$$\tau_{\text{eff}} \approx \frac{1}{\alpha(1 - \rho)},$$

where $\rho$ is the spectral radius. For a fixed reservoir, all $N$ neurons share this single timescale (up to unit-to-unit variation from random initialization). This imposes a fundamental ceiling: a single reservoir is well-suited to tasks with a single characteristic temporal scale, but struggles when the task requires simultaneous processing of multiple timescales.

Many real-world signals are inherently hierarchical. Consider speech: the signal contains phonemic structure at the 10–50 ms scale, syllabic structure at the 100–300 ms scale, word-level structure at the 500–2000 ms scale, and sentence-level structure over seconds. A single reservoir with a fixed effective time constant cannot simultaneously track all four levels without a radical compromise. Setting $\alpha$ to track phonemes means the reservoir forgets word-level context; setting $\alpha$ to track sentences means phonemic details are blurred away.

The same hierarchy appears in motor control (muscle twitches → joint movements → limb trajectories → behavioral sequences), in visual processing (edges → contours → objects → scenes), and in natural language (tokens → phrases → clauses → discourse). The commonality is that information at coarser levels depends on integration over longer timescales, while information at finer levels depends on local temporal detail [Gallicchio & Micheli 2017].

## Deep Feedforward Networks as the Inspiration

In feedforward deep learning, each layer extracts increasingly abstract features from its input. The first layer detects low-level patterns (edges in images, phonemes in audio); subsequent layers combine these into higher-order abstractions (textures, syllables; objects, words). This hierarchy of abstraction enables deep networks to represent functions that shallow networks can only approximate with exponentially more units [LeCun et al. 2015].

The reservoir analog of this insight is: each layer should operate at a different timescale, with lower layers (closer to input) tracking rapid fluctuations and upper layers tracking slow, long-term context. A hierarchical reservoir would then provide, at its various layers, simultaneous representations of the input at multiple temporal resolutions — exactly what complex temporal tasks require.

## Gallicchio et al. 2017: Empirical Motivation

The systematic investigation of deep echo state networks was carried out by Gallicchio & Micheli [2017], who introduced the DeepESN architecture (detailed in Section 13.2) and evaluated it on a hierarchy of benchmark tasks. Their key finding was that deep ESNs with geometrically spaced leak rates ($\alpha_\ell \propto 1/\ell$) consistently outperformed single-layer ESNs of equal total size on tasks with hierarchical temporal structure, while remaining comparable or slightly worse on purely single-timescale tasks.

Specifically, on a polyphonic music prediction task — which involves simultaneous harmonic structure (fast) and melodic structure (slow) — a 5-layer DeepESN achieved 15–25% lower NRMSE than a single-layer ESN with the same total neuron count. The performance gain was directly attributable to the timescale separation: removing it (by setting all $\alpha_\ell$ equal) reduced the deep ESN to the performance of the shallow baseline [Gallicchio & Micheli 2017].

## The Core Idea: Timescale Hierarchy by Layer

The fundamental design principle of deep reservoir computing is that the leak rate $\alpha_\ell$ of layer $\ell$ should decrease with depth:

$$\alpha_1 > \alpha_2 > \cdots > \alpha_L.$$

Layer 1 receives the raw input and has a high leak rate, meaning it forgets quickly and tracks rapid fluctuations. Layer 2 receives the output of layer 1 and has a lower leak rate, integrating over a longer effective time window. Each successive layer acts as a low-pass filter on the output of the layer below, with a lower cutoff frequency [Gallicchio & Micheli 2017].

The readout is trained on the concatenated states from all layers simultaneously:

$$\mathbf{y}_t = \mathbf{W}^{\text{out}} [\mathbf{x}_t^{(1)}; \mathbf{x}_t^{(2)}; \cdots; \mathbf{x}_t^{(L)}].$$

This gives the readout access to the full temporal hierarchy, allowing it to combine fast features (from layer 1) with slow features (from layer $L$) as needed for the task.

## Why This Approach is Principled

Stacking reservoirs with decreasing $\alpha$ is not merely an engineering trick. It has a principled basis in the theory of dynamical systems: the effective time constant of a leaky integrator with spectral radius $\rho$ and leak rate $\alpha$ is $\tau_{\text{eff}} = 1/(\alpha(1-\rho))$ (derived in Section 13.3). By choosing $\alpha_\ell$ to decrease geometrically with $\ell$, the time constants increase geometrically:

$$\tau_{\text{eff}}^{(\ell)} \approx \ell \cdot \tau_{\text{eff}}^{(1)},$$

covering the hierarchy of timescales in the input with systematic spacing. This matches the structure of natural signals, where temporal scales are often related by multiplicative rather than additive factors [Gallicchio & Micheli 2017].

---

## References

- Gallicchio, C., & Micheli, A. (2017). Echo state property of deep reservoir computing networks. *Cognitive Computation*, 9(3), 337–350.
- LeCun, Y., Bengio, Y., & Hinton, G. (2015). Deep learning. *Nature*, 521(7553), 436–444.

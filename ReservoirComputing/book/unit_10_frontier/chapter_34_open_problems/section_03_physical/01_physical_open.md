# Open Problems in Physical Reservoir Computing

## 34.3.1 The Theory-Hardware Gap

Physical reservoir computing has produced impressive experimental demonstrations: optoelectronic reservoirs for speech recognition [Larger et al. 2017], spintronic reservoirs for chaotic time series [Torrejon et al. 2017], mechanical reservoirs for robotic control [Nakajima et al. 2013], and microfluidic reservoirs for pattern recognition. Yet the relationship between the theoretical properties of a reservoir (nonlinearity, fading memory, high dimensionality, separation) and the physical properties of a specific substrate remains poorly understood.

A researcher who wants to build a physical reservoir for a specific task faces a fundamental question: what physical system should they use, and how should they configure it? The answer is currently: try several systems empirically and pick the one that works. This is not satisfying. The open problems below represent the gap between theoretical understanding and engineering practice.

## 34.3.2 Problem 1: Closing the Theory-Hardware Loop

**Problem.** How do the four standard reservoir conditions (nonlinearity, high dimensionality, fading memory, separation) manifest in specific physical substrates?

**Current state.** The four conditions are stated abstractly in terms of the input-output map of the reservoir. For an ESN, each condition can be checked computationally:
- *Nonlinearity:* Measure NARMA task performance.
- *Dimensionality:* Compute the effective dimensionality of the state space via PCA of recorded states.
- *Fading memory:* Measure the linear memory capacity $C_L$.
- *Separation:* Measure the Fisher memory curve or the kernel quality metric.

For physical reservoirs, computing these quantities requires instrumenting the system: recording internal states, applying controlled inputs, and analyzing the outputs. For many substrates (spintronic, soft-body, microfluidic), internal state recording is partial or destructive.

**Open question.** Can the four conditions be verified from input-output measurements alone (without internal state access)? Is there a black-box characterization of reservoir quality?

**Formal version.** Given access only to input-output pairs $(u(t), y(t))$ from an unknown physical reservoir, can we compute (or bound) the IPC, memory capacity, and separation quality? If so, at what sample complexity?

## 34.3.3 Problem 2: Task-Substrate Matching

**Problem.** What physical substrate is optimal for which task class?

**Current evidence.** Different physical substrates have different dynamical timescales:
- Optoelectronic (nanosecond timescales): optimal for high-bandwidth temporal tasks
- Spintronic (nanosecond–microsecond): optimal for classification with short temporal context
- Soft-body/mechanical (millisecond–second): optimal for locomotion control
- Chemical/microfluidic (second–minute): optimal for slow pattern recognition

But timescale matching (physical dynamics timescale $\approx$ task timescale) is only one dimension of task-substrate matching. Other dimensions include:
- Dimensionality: how many independent state variables can be read out?
- Noise level: how does thermal/shot/fabrication noise affect generalization?
- Nonlinearity type: saturation (sigmoid-like) vs. threshold (ReLU-like) vs. oscillatory

**Open question.** Is there a quantitative theory that maps task complexity (e.g., Volterra kernel order, memory depth, input bandwidth) to required substrate properties? Can we predict, a priori, whether substrate A will outperform substrate B on task C?

**Partial result.** [Nakajima & Fischer 2021 (review)] catalog the performance of different physical substrates on NARMA and spoken digit tasks, but no systematic theory connects substrate properties to task performance.

## 34.3.4 Problem 3: Scalability

**Problem.** Can physical reservoirs scale to $N > 10^4$ nodes without decoherence, crosstalk, or fabrication defects?

**Current state.** Most physical RC demonstrations use effective reservoir sizes of $N = 50$–$1000$ (via time-multiplexing). The largest time-multiplexed reservoir (single node, delay-feedback) has $N \sim 10^4$ virtual nodes [Larger et al. 2017], but increasing $N$ also increases the delay length $\tau = N \cdot \Delta t$, which may exceed the coherence time of the physical system.

For multi-node physical reservoirs (e.g., photonic integrated circuits, spintronic arrays), fabrication variability limits scalability: each node has slightly different parameters, and systematic parameter mismatch can degrade reservoir performance compared to a homogeneous random ESN.

**Open questions.**
(a) What is the maximum $N$ achievable with current fabrication technology for each physical substrate, given a specified performance degradation tolerance $\delta\text{NRMSE}$?
(b) Is there a computational advantage to large physical reservoirs ($N > 10^4$) over digital simulation of ESNs of the same size?

## 34.3.5 Problem 4: Learning the Physical Output Mask

**Problem.** How can the readout weights $\mathbf{w}$ be learned in situ on physical hardware, without digitizing and transferring all readout channel signals?

**Current state.** Standard physical RC trains the readout offline: all recorded signals are transferred to a digital computer, and ridge regression is performed digitally. This is acceptable for laboratory demonstrations but impractical for deployed systems with limited bandwidth between the physical reservoir and the digital processor.

**The fundamental challenge.** In-situ readout learning requires implementing gradient descent (or ridge regression) directly on the physical hardware. For a linear readout, this requires:
1. Computing the gradient $\nabla_\mathbf{w}\mathcal{L} = -2\sum_t \mathbf{x}(t)(y_t - \mathbf{w}^T\mathbf{x}(t))$.
2. Updating $\mathbf{w}$ by a small step in the gradient direction.
3. Repeating until convergence.

Step 1 requires recording all $N$ state signals and computing their covariance with the target. For large $N$ and high bandwidth signals, this is a bottleneck.

**Proposed solutions:**
- **Analog weight storage** (memristors, ferroelectric devices): store $\mathbf{w}$ in analog memory, update with local Hebbian rules [Torrejon et al. 2017].
- **Optical matrix multiplication:** implement the readout as a free-space optical transformation; adjust using spatial light modulators.
- **On-chip digital-analog interface:** use a small FPGA to receive analog signals and perform ridge regression in real time [Duport et al. 2016].

**Open question.** Is there a learning rule that converges to the optimal readout using only locally available signals (input, output, error), without needing access to all $N$ reservoir states simultaneously?

## 34.3.6 Problem 5: Hybrid Physical-Digital Reservoirs

**Problem.** What is the optimal combination of physical and digital reservoir components for a given task and hardware budget?

**Motivation.** Pure physical reservoirs are fast but hard to configure; pure digital reservoirs are flexible but consume power and require specialized hardware. Hybrid systems that combine a physical "front-end" reservoir with a digital "back-end" readout (or vice versa) may offer the best of both.

**Current approaches.** [Du et al. 2017] demonstrated a hybrid system combining a physical (spintronic) reservoir for nonlinear feature extraction with a digital (FPGA) linear readout. The physical layer provided fast, low-power feature extraction; the digital layer provided precise, reconfigurable readout training.

**Open question.** What is the optimal "cut point" between physical and digital processing? How much of the reservoir state should be processed physically vs. digitally? Can the cut point be learned adaptively?

## 34.3.7 Problem 6: Fabrication Variability as a Resource

**Problem.** Physical reservoirs are not precisely controlled: physical parameters (node coupling, nonlinearity thresholds, noise level) vary from device to device. Can fabrication variability be exploited as a computational resource rather than treated as a limitation?

**Background.** In digital computing, fabrication variability is a defect to be minimized. In analog/physical computing, variability may be beneficial: it creates diversity among nodes (similar to the effect of different spectral radii in a heterogeneous ESN) and prevents synchronization (which would reduce effective dimensionality).

**Open question.** Is there a formal connection between fabrication variability and reservoir diversity (e.g., measured by the effective rank of the state covariance)? Can fabrication processes be designed to produce "optimally varied" physical reservoirs?

## References

- Du, C., et al. (2017). Reservoir computing using dynamic memristors for temporal information processing. *Nature Communications*, 8(1), 2204.
- Duport, F., Smerieri, A., Akrout, A., Haelterman, M., and Massar, S. (2016). Fully analogue photonic reservoir computer. *Scientific Reports*, 6, 22381.
- Larger, L., Baylón-Fuentes, A., Martinenghi, R., et al. (2017). High-speed photonic reservoir computing using a time-delay-based architecture: Million words per second classification. *Physical Review X*, 7(1), 011015.
- Nakajima, K. and Fischer, I. (Eds.) (2021). *Reservoir Computing: Theory, Physical Implementations, and Applications*. Springer.
- Tanaka, G., Yamane, T., Héroux, J. B., et al. (2019). Recent advances in physical reservoir computing: A review. *Neural Networks*, 115, 100–123.
- Torrejon, J., Riou, M., Araujo, F. A., et al. (2017). Neuromorphic computing with nanoscale spintronic oscillators. *Nature*, 547(7664), 428–431.

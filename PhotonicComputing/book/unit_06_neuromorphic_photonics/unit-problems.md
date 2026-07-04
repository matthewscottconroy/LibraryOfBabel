# Unit VI Problem Set: Neuromorphic Photonics

*These problems span Chapters 15–16 of Unit VI, covering neuromorphic computing concepts — spiking neurons, spike-timing codes, and STDP — and their photonic realization in excitable lasers, phase-change synapses, and WDM spiking architectures. Problems are labelled: [Easy], [Medium], or [Hard]. Hints are provided for Hard problems.*

---

## Chapter 15: Neuromorphic Computing Concepts

**Problem 15.1** [Medium]
A leaky integrate-and-fire (LIF) neuron has membrane time constant $\tau_m = 10$ ms, membrane resistance $R = 10$ MΩ, threshold $V_{th} - V_{\text{rest}} = 15$ mV, reset to rest ($V_{\text{reset}} = V_{\text{rest}}$), and absolute refractory period $t_{\text{ref}} = 2$ ms. It is driven by a constant current $I$.

(a) Find the rheobase current $I_{th}$ (the minimum constant current that makes the neuron fire).
(b) Show that the firing rate is $f(I) = \left[\,t_{\text{ref}} + \tau_m \ln\!\dfrac{I}{I - I_{th}}\,\right]^{-1}$ for $I > I_{th}$.
(c) Evaluate $f$ at $I = 3$ nA (twice rheobase).
(d) Find the maximum firing rate as $I \to \infty$, and state what sets it. If this neuron were re-implemented as an excitable laser with $\tau_m \to 0.1$ ns, roughly what maximum rate would you expect?

**Problem 15.2** [Medium]
A photonic neuron operates with a coding window $T = 1$ ns and a spike-timing jitter $\Delta t = 10$ ps.

(a) In a *temporal* (time-to-first-spike) code, how many distinguishable time slots fit in the window, and how many bits does a single spike therefore carry?
(b) In a *rate* code, how many spikes must be counted to reach the same information content?
(c) Compare the energy cost of the two codes if each spike costs the same fixed energy. State the ratio.
(d) Explain why the picosecond timing precision of photonic neurons makes temporal coding attractive, whereas biological neurons (millisecond jitter) lean on rate coding.

**Problem 15.3** [Medium]
A synapse is updated by the STDP rule $\Delta w = A_+ e^{-\Delta t/\tau_+}$ for $\Delta t = t_{\text{post}} - t_{\text{pre}} > 0$ and $\Delta w = -A_- e^{\,\Delta t/\tau_-}$ for $\Delta t < 0$, with $A_+ = 0.010$, $A_- = 0.012$, $\tau_+ = 17$ ms, $\tau_- = 34$ ms.

(a) Compute $\Delta w$ for a causal pairing $\Delta t = +5$ ms.
(b) Compute $\Delta w$ for an anti-causal pairing $\Delta t = -10$ ms.
(c) Compute the integrated potentiation area $A_+\tau_+$ and depression area $A_-\tau_-$. Which dominates, and why does a net-depressive window keep uncorrelated weights from running away?
(d) Argue qualitatively that repeated causal pairing potentiates the predictive synapse toward saturation.

**Problem 15.4** [Medium]
Compare spike latencies across substrates. Take a biological action potential to last $\sim 1$ ms and a photonic spike $\sim 10$ ps. A photonic network routes spikes through 1 cm of waveguide per layer ($n_g = 4.2$).

(a) Compute the ratio of biological to photonic spike duration.
(b) Compute the layer-to-layer latency of the photonic network (spike duration plus time-of-flight through 1 cm).
(c) Estimate the end-to-end latency of a 10-layer photonic network, and compare with a 10-layer biological network at $\sim 1$ ms per layer.
(d) List the physical effects that ultimately limit photonic per-layer latency (do not assume it can be made arbitrarily small).

**Problem 15.5** [Medium]
A WDM broadcast bus fans one neuron's optical output out to $N$ receiving neurons. The source emits $+10$ dBm, each receiver needs at least $-20$ dBm to register a spike, and the passive path (routing, filters) costs $10$ dB before splitting.

(a) Compute the power budget available for splitting, in dB.
(b) Assuming an ideal $1{:}N$ splitter (splitting loss $10\log_{10} N$), find the maximum fan-out $N$.
(c) Now each branch also passes a microring drop with $3$ dB loss. Recompute $N$. Compare with the $\sim 49$-node silicon weight-bank network demonstrated by Tait et al. (2017).
(d) Contrast this passive optical fan-out with electronic fan-out, where each branch needs its own driver and the interconnect is RC-limited.

**Problem 15.6** [Hard]
*Hint: Separate the two kinds of energy. Event-driven ("dynamic") energy scales with the number of spikes actually emitted; static ("hold") energy is paid continuously regardless of activity. Compare them over the actual inference time.*

Estimate the energy of one forward pass of a 10-layer, fully connected spiking network with 1000 neurons per layer.

(a) Count the synaptic operations per forward pass (full connectivity between adjacent layers).
(b) *GPU baseline:* at an effective $20$ pJ per multiply–accumulate (memory movement included), find the energy per inference.
(c) *Photonic SNN:* weights are stored passively in non-volatile PCM (weighting is "free"), each neuron fires once per inference, and each spike costs $1$ pJ to generate and detect. Find the energy per inference and the ratio to (b).
(d) Now include a hold power of $1$ mW per microring for $10^4$ rings. Over a $1$ ns inference, how much energy is that, and how does it compare to the spike energy in (c)? Over a $1$ μs inference? State clearly which regime makes the photonic advantage real and which erodes it.

---

## Chapter 16: Photonic Neurons and Synapses

**Problem 16.1** [Hard]
*Hint: Evaluate the $2\times 2$ Jacobian at the fixed point. The sign of the trace gives stability; a negative discriminant ($\text{tr}^2 < 4\det$) gives a complex-conjugate eigenvalue pair — damped relaxation oscillations. The relaxation-oscillation frequency is $\omega_R \approx \sqrt{\det}$ and the damping is $|\text{Re}\,\lambda| = |\text{tr}|/2$.*

An excitable laser neuron obeys, in photon number $S$ and carrier number $N$,

$$\dot{S} = \Big[g(N - N_{tr}) - \tfrac{1}{\tau_p}\Big]S, \qquad \dot{N} = \frac{I}{e} - \frac{N}{\tau_s} - g(N - N_{tr})\,S,$$

with $\tau_p = 2$ ps and $\tau_s = 1$ ns.

(a) Find the above-threshold fixed point: show $N_0 = N_{tr} + 1/(g\tau_p)$ and express $S_0$ in terms of $I$, $e$, $N_0$, $\tau_s$.
(b) Write the Jacobian at $(S_0, N_0)$ and show its entries are $\begin{pmatrix} 0 & gS_0 \\ -1/\tau_p & -(1/\tau_s + gS_0) \end{pmatrix}$.
(c) Taking $gS_0 = 2\times 10^{9}\ \text{s}^{-1}$, compute the trace, determinant, and eigenvalues. Show the eigenvalues form a complex-conjugate pair and give the relaxation-oscillation frequency $f_R$ (in GHz) and the envelope decay time.
(d) The excitable-versus-self-pulsating boundary is the Hopf condition $\text{Re}\,\lambda \to 0$. Using the trace, state what would drive the neuron across that boundary into continuous oscillation, and explain why excitable spike processing requires operation on the damped (stable-focus) side.

**Problem 16.2** [Medium]
A phase-change synapse is programmed to $L = 34$ analog levels between transmissions $T_{\min} = 0.1$ and $T_{\max} = 0.9$. Read-out noise is Gaussian with $\sigma = 0.005$.

(a) What weight precision, in bits, do 34 levels represent?
(b) Compute the level spacing $\Delta T$ and the per-level SNR $\Delta T/\sigma$ in dB.
(c) Compute the full-scale SNR in dB and the effective number of bits $\text{ENOB} = (\text{SNR}_{\text{dB}} - 1.76)/6.02$. How much margin does the 34-level programming leave against the noise floor?
(d) Suppose ageing/drift doubles the noise to $\sigma = 0.01$. Requiring a level spacing of at least $6\sigma$ for reliable read-out, how many levels remain usable, and what bit-depth is that?

**Problem 16.3** [Medium]
Cascade insertion loss decides how many synapses can sit on one optical path. Assume a fully crystalline GST synapse cell adds $1.0$ dB of insertion loss, and a GSST cell $0.1$ dB (order-of-magnitude values).

(a) Compute the total loss of an 8-cell chain, all crystalline, for GST and for GSST.
(b) For a $3$ dB loss budget, how many crystalline cells can each material cascade?
(c) A weighted sum needs 16 synapses on one waveguide. Compare the output-power penalty for GST versus GSST, and state the factor by which more input power GST would demand.
(d) Explain, in terms of the crystalline extinction coefficient, why GSST (Zhang et al., 2019) is cascadable where GST is not, and why this makes material choice an architectural decision.

**Problem 16.4** [Medium]
A broadcast-and-weight neuron sums four WDM inputs. Optical powers are $P = [0.5, 0.3, 0.8, 0.2]$ mW at $\lambda_1 \ldots \lambda_4$, and the signed weights are $w = [+0.5, -0.7, +0.3, -1.0]$. The balanced photodiode pair has responsivity $R = 1$ A/W.

(a) Compute the weighted-sum photocurrent $I = R\sum_i w_i P_i$.
(b) Explain how balanced detection of the drop and through ports produces the negative weights, and why signed weights are essential for a spiking network.
(c) The microring weight bank has $Q = 10^4$ at $193$ THz and a free spectral range of $3.2$ THz. Requiring channel spacing $\ge 4\times$ the ring linewidth to bound crosstalk, find the maximum number of WDM channels. Compare with the demonstrated network size of Tait et al. (2017).
(d) Name two effects (beyond FSR) that ultimately cap the channel count in practice.

**Problem 16.5** [Medium]
In an all-optical STDP synapse, a pre-spike and a post-spike meet at a PCM cell; tunable delay lines ($n_g = 4.0$) set their relative arrival $\Delta t = t_{\text{post}} - t_{\text{pre}}$, which programs the weight through the window $\Delta w = A_+ e^{-\Delta t/\tau_+}$ ($\Delta t > 0$) or $\Delta w = -A_- e^{\,\Delta t/\tau_-}$ ($\Delta t < 0$), with $A_+ = 0.020$, $A_- = 0.024$, $\tau_+ = \tau_- = 700$ ps.

(a) Compute the propagation delay of a $3.0$ mm delay line.
(b) The pre-path is $3.0$ mm and the post-path is $1.5$ mm. Compute $\Delta t$ and state whether the update is potentiation or depression.
(c) Compute $\Delta w$ for that pairing.
(d) Design the path-length *difference* needed to produce a target $\Delta t = +50$ ps (potentiation).

**Problem 16.6** [Hard]
*Hint: The Heaviside spike function $S = \Theta(V - V_{th})$ has zero derivative almost everywhere and an ill-defined one at threshold, so it cannot pass gradients. In the backward pass only, replace $\Theta'(V - V_{th})$ with a smooth surrogate — here the fast-sigmoid derivative $\sigma'(u) = (\beta|u| + 1)^{-2}$ with $u = V - V_{th}$. The forward pass still emits true binary spikes.*

A spiking neuron in a network trained by surrogate-gradient backpropagation has $V - V_{th} = +2$ (normalized units), presynaptic activity $x = \partial V/\partial w = 0.7$, upstream gradient $\partial L/\partial S = -0.4$, learning rate $\eta = 0.05$, and surrogate steepness $\beta = 1$.

(a) State the forward-pass output $S$ of this neuron.
(b) Explain why the exact derivative $dS/dV$ cannot be used to train the weight, and what pathology it produces in backpropagation.
(c) Using the surrogate, compute $\sigma'(u)$, then the gradient $\partial L/\partial w = (\partial L/\partial S)\,\sigma'(u)\,x$, and finally the weight update $\Delta w = -\eta\,\partial L/\partial w$.
(d) Describe the role of $\beta$: what happens to the gradient flow as $\beta \to \infty$ (narrow surrogate) versus $\beta \to 0$ (broad surrogate)? Note that the trained weights are subsequently written to the non-volatile PCM synapses of the photonic hardware.

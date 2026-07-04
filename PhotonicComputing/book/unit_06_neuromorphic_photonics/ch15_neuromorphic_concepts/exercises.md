# Exercises — Chapter 15: Neuromorphic Computing Concepts

Unless stated otherwise, use the leaky integrate-and-fire (LIF) neuron
$$\tau_m \frac{dV}{dt} = -(V - V_{rest}) + R\,I(t), \qquad \text{spike + reset to } V_{reset} \text{ when } V \ge V_{th},$$
with the reference parameters $\tau_m = 15$ ms, $R = 100$ MΩ, $V_{rest} = V_{reset} = -70$ mV, and $V_{th} = -55$ mV, so that the threshold sits $\Delta V = V_{th} - V_{rest} = 15$ mV above rest.

---

## Mathematical Exercises

**15.1** (Subthreshold Response and Time-to-Spike) A constant current $I$ is switched on at $t = 0$ with the membrane initially at rest.

(a) Solve the LIF equation to obtain $V(t)$ below threshold, and identify the steady-state potential $V_\infty$.

(b) Show that a spike is possible only if $I$ exceeds the *rheobase* current $I_{rh} = \Delta V / R$. Evaluate $I_{rh}$ in nA.

(c) For $I = 0.20$ nA, compute the time to the first spike measured from $t = 0$.

(d) A photonic LIF neuron has the *same* dimensionless parameters but $\tau_m = 15$ ps (a factor $10^9$ faster). What is its time-to-first-spike for the analogous drive? State the general scaling law relating the two neurons.

---

**15.2** (The LIF $f$–$I$ Curve) With a hard reset to $V_{reset} = V_{rest}$ and an absolute refractory period $t_{ref}$, the firing rate under constant current is
$$f(I) = \left[\, t_{ref} + \tau_m \ln\!\frac{R I}{R I - \Delta V} \,\right]^{-1}, \qquad R I > \Delta V. \tag{1}$$

(a) Derive Eq. (1) from the subthreshold solution of Exercise 15.1.

(b) With $t_{ref} = 2$ ms, evaluate the maximum firing rate as $I \to \infty$. Why does the refractory period, not the membrane time constant, set this ceiling?

(c) Compute $f$ at $I = 0.30$ nA (still using $t_{ref} = 2$ ms).

(d) Sketch $f(I)$ and mark the rheobase and the saturation rate. Contrast this thresholded, saturating curve with the smooth ReLU/sigmoid activation of the analog ONNs of Unit V.

---

**15.3** (STDP Weight Updates) Use the pairwise STDP rule with $\Delta t = t_{post} - t_{pre}$,
$$\Delta w = \begin{cases} +A_+\, e^{-\Delta t/\tau_+}, & \Delta t > 0 \\[2pt] -A_-\, e^{+\Delta t/\tau_-}, & \Delta t < 0, \end{cases}$$
and the biologically motivated parameters $A_+ = 0.010$, $A_- = 0.012$, $\tau_+ = 17$ ms, $\tau_- = 34$ ms.

(a) A presynaptic spike at $t = 10$ ms precedes a postsynaptic spike at $t = 15$ ms. Compute $\Delta w$ and state whether the synapse potentiates or depresses.

(b) A presynaptic spike at $t = 20$ ms follows a postsynaptic spike at $t = 12$ ms. Compute $\Delta w$.

(c) In a triplet, one presynaptic spike arrives 2 ms *before* and a second presynaptic spike arrives 6 ms *after* a single postsynaptic spike. Using the additive (pairwise-independent) approximation, compute the net $\Delta w$.

(d) Compute the areas $A_+\tau_+$ (integrated potentiation) and $A_-\tau_-$ (integrated depression). Explain why the inequality $A_-\tau_- > A_+\tau_+$ makes STDP *competitive* and keeps runaway potentiation in check for uncorrelated inputs.

---

**15.4** (Rate versus Temporal Coding: Information and Energy) A neuron fires at up to $f_{max} = 200$ Hz and is read out in a window $T = 100$ ms.

(a) *Rate code.* The maximum spike count in the window is $N = f_{max} T$. Assuming Poisson statistics (standard deviation $\sqrt{N}$), estimate the number of reliably distinguishable rate levels and convert to bits.

(b) *Temporal code.* If instead the *latency* of a single spike is resolved to $\Delta t = 1$ ms within the same window, how many bits does one spike carry? 

(c) Compare bits per *spike* for the two schemes. Which scheme extracts more information per unit of spiking energy, and by roughly what factor?

(d) Argue qualitatively why a photonic neuron, with picosecond spikes and picosecond timing jitter, is naturally suited to temporal coding rather than rate coding.

---

**15.5** (WDM Fan-out Power Budget) A photonic neuron emits $P_0 = 1$ mW (0 dBm) at its wavelength $\lambda_i$ onto a shared bus waveguide that broadcasts to $N$ downstream neurons. Each receiver requires at least $P_{min} = 1$ μW ($-30$ dBm) at its photodetector to register a spike, and the network carries a fixed excess loss of 3 dB (waveguide plus microring drop).

(a) For an ideal passive $1{:}N$ power split, write the power delivered per branch. Evaluate it (in dBm) for $N = 50$.

(b) Including the 3 dB excess loss, compute the maximum fan-out $N_{max}$ that still delivers $P_{min}$ to every receiver.

(c) In a broadcast-and-weight network, $M$ source neurons instead use $M$ *distinct* wavelengths on the same bus, each addressed at a receiver by its own tuned add–drop microring. Explain why WDM raises the aggregate synaptic throughput without imposing the $1/N$ splitting penalty on the *weighting* step.

(d) Estimate how many 100-GHz-spaced channels fit in the ~4.4 THz C-band, i.e. the practical ceiling on $M$ for a single bus.

---

**15.6** (Energy per Spike versus GPU MAC) A photonic spiking network completes a task that requires $S = 10^{9}$ spike events.

(a) At an optimistic $E_{spike} = 1$ fJ per spike, compute the total spiking energy.

(b) A GPU performing the equivalent computation as dense multiply–accumulates needs $M = 10^{12}$ MACs at an effective energy on the order of $E_{MAC} \approx 1$ pJ per MAC (including on-chip data movement; the bare arithmetic unit is lower). Compute the GPU energy.

(c) Form the ratio and comment. Then list three reasons the comparison is *not* apples-to-apples (e.g. event sparsity, analog precision, and the cost of optical-to-electrical conversion and laser wall-plug efficiency).

---

**15.7** (Latency Scaling of Photonic versus Electronic SNNs) A feedforward spiking network has $L = 10$ layers, and a spike must traverse them in sequence.

(a) An electronic neuromorphic core has a per-layer spike latency of about 1 μs. Compute the total forward latency.

(b) On a photonic chip, the per-layer latency is dominated by the on-chip time-of-flight plus the neuron response. For a 2 mm interconnect at group index $n_g = 4.2$, compute the propagation delay $t = n_g \ell / c$, and add a 10 ps neuron response.

(c) Compute the total photonic forward latency for 10 layers and the speedup over the electronic case.

(d) Name two applications in which end-to-end *latency* (not throughput) is the binding constraint, and explain why picosecond-scale spikes matter there.

---

## Conceptual Exercises

**15.8** (Neuromorphic Hardware Tradeoffs) Consider five platforms: Intel Loihi (digital, asynchronous, on-chip learning), IBM TrueNorth (digital, ~70 mW for 1 M neurons), SpiNNaker (ARM-based real-time software simulation), BrainScaleS (analog, $10^3$–$10^4\times$ accelerated), and a hypothetical integrated *photonic* spiking chip.

(a) Rank the platforms qualitatively along four axes: spike timescale, energy per spike, native on-chip learning, and analog precision.

(b) Match each platform to the application it best serves among: real-time low-power robotics, accelerated large-scale brain simulation, and ultra-low-latency radio-frequency (RF) front-end processing. Justify each match.

(c) What single capability do all of the electronic platforms share that the photonic platform is designed to break, and what new problem does the photonic approach introduce in exchange?

---

**15.9** (Choosing a Neural Code) The retina can encode the onset of a bright flash in the *latency* of the first spike, whereas many cortical areas use sustained firing *rates*.

(a) Why is time-to-first-spike coding especially attractive for a neuron that can fire in picoseconds?

(b) What robustness does rate coding buy — against jitter, dropout, and noise — that pure temporal coding sacrifices?

(c) For a photonic neuron capable of picosecond spikes but limited to GHz sustained rates, argue which coding scheme better exploits the hardware, and describe a hybrid scheme that hedges.

---

**15.10** (Why Fan-out Favors Photonics) An electronic neuron driving $N$ downstream synapses must charge $N$ wire capacitances through a driver; a photonic neuron splits its output power $N$ ways.

(a) Explain why the electronic fan-out cost grows with $N$ (RC charging, driver sizing, interconnect energy) while optical fan-out is essentially "free" in bandwidth.

(b) Optical fan-out is not free in *power*: each branch receives $\sim P_0/N$. Using a 1 mW source and a 1 μW per-receiver sensitivity, at roughly what $N$ does the optical power penalty force either higher launch power or on-chip amplification?

(c) How does WDM change this accounting, and what physical resource does it ultimately spend instead of power?

---

## Laboratory / Programming Exercises

**Lab 15.1: Simulating a Leaky Integrate-and-Fire Neuron**

Build a numerical LIF neuron and characterize it.

(a) Integrate the LIF equation with the forward-Euler method (time step $\ll \tau_m$) using the reference parameters above. Inject a constant current plus a small Gaussian noise term and plot $V(t)$, marking each spike and reset.

(b) Sweep the input current $I$ and measure the steady-state firing rate. Plot the empirical $f$–$I$ curve and overlay the analytical curve from Eq. (1) in Exercise 15.2. Identify the rheobase on the plot.

(c) Add an absolute refractory period $t_{ref}$ and confirm that the firing rate saturates at $1/t_{ref}$ for large $I$.

(d) Rescale *all* time constants by $10^{-9}$ (the photonic regime) and verify numerically that the *shape* of the $f$–$I$ curve is invariant while absolute rates scale from Hz to GHz. Comment on what this dimensionless invariance means for porting biological models to photonic hardware.

**Skills practiced**: ODE integration, threshold-and-reset event logic, $f$–$I$ characterization, dimensional scaling.

---

**Lab 15.2: STDP and Competitive Learning**

Implement spike-timing-dependent plasticity and observe emergent selectivity.

(a) Implement the pairwise STDP rule from Exercise 15.3 and reproduce the STDP window: plot $\Delta w$ versus $\Delta t \in [-100, 100]$ ms and confirm the asymmetry between the potentiation and depression lobes.

(b) Drive a single LIF neuron with 100 Poisson-spiking input synapses, a subset of which are temporally correlated with each other. Apply STDP online with a weight clip $w \in [0, w_{max}]$. Show that the synapses carrying correlated input strengthen while uncorrelated synapses weaken — the hallmark of competitive, Hebbian self-organization.

(c) Present a fixed input spike *pattern* repeatedly and show that, over training, the neuron learns to fire *earlier* in the pattern (latency reduction). Relate this to time-to-first-spike coding from Exercise 15.9.

(d) Discuss which parts of this simulation map cleanly onto an all-optical PCM synapse (weight storage, timing-dependent update) and which do not (unbounded precision, instantaneous global reset), previewing Chapter 16.

**Skills practiced**: event-driven simulation, Hebbian/STDP weight updates, emergence of feature selectivity, weight normalization and stability.

---

*These exercises span numerical computation, physical estimation, conceptual reasoning, and system design — the same four modes of thinking that Chapter 16 will demand when these neurons and synapses become lasers and phase-change films.*

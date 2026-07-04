# Subsection 12.4.2: The Broadcast-and-Weight Architecture

## Orientation

A weight bank computes one neuron's weighted sum. To build a *network*, the neurons' outputs must become other neurons' inputs — including, for recurrent networks, their own. Broadcast-and-weight, proposed by Tait, Nahmias, Shastri, and Prucnal at Princeton in 2014, solves the interconnection problem with a single shared medium: every neuron transmits on its own wavelength into a common bus waveguide, every neuron receives the entire WDM ensemble, and each neuron's private weight bank decides how much of everyone else it listens to. It is Ethernet for photonic neurons — an all-to-all network whose wiring complexity grows linearly, not quadratically, with neuron count.

---

## 12.4.2.1 The Protocol

The architecture assigns each of $N$ neurons a unique wavelength $\lambda_j$ and four functions:

1. **Broadcast:** neuron $j$'s output signal modulates a carrier at $\lambda_j$, injected onto the shared bus. All $N$ wavelengths co-propagate; a broadband splitter taps the full WDM spectrum off to every node.
2. **Weight:** node $i$'s microring weight bank (Subsection 12.4.1) applies its private weight vector $\{w_{ij}\}_{j=1}^{N}$, one ring per incoming wavelength.
3. **Sum:** node $i$'s balanced photodetector converts the weighted WDM ensemble into a single photocurrent $\propto \sum_j w_{ij} x_j$.
4. **Activate and retransmit:** the photocurrent drives node $i$'s own modulator (at $\lambda_i$), whose electro-optic transfer function supplies the nonlinearity, closing the loop back onto the bus.

Formally the network computes the recurrent dynamics

$$x_i(t + \tau) = f\!\left(\sum_{j} w_{ij}\, x_j(t) + b_i\right)$$

where $\tau$ is the loop latency and $f$ is the modulator's transfer function — for a Mach-Zehnder modulator biased at quadrature, $f(v) \propto \cos^2(\pi v/2V_\pi + \pi/4)$, a smooth sigmoid-like curve; for a ring modulator, a sharper Lorentzian-derived nonlinearity. The **modulator neuron** is a crucial economy: the same O/E/O stage that regenerates the signal (providing gain, fan-out, and cascadability — the properties whose absence doomed all-optical logic in Chapter 11) also provides the activation function for free. Chapter 13 (Section 13.2) returns to this point; Unit VI develops its spiking cousin.

---

## 12.4.2.2 Scaling and Power Budget

**Channel count.** As in Subsection 12.4.1, all $N$ wavelengths must fit in one ring FSR with acceptable crosstalk: $N \lesssim \text{FSR}/\Delta\lambda_{\text{ch}} \approx$ 10–30 for a single bus. Larger networks require hierarchies of buses bridged by wavelength converters or O/E/O gateways, or spatial parallelism (multiple buses).

**Worked power budget** for one $N = 16$ node at 5 GHz signal bandwidth:

| Element | Value | Note |
|---|---|---|
| Laser power per channel (at chip) | 1 mW | DFB array or comb, 10% wall-plug |
| Splitter loss (1:16 broadcast) | 12 dB | fundamental $1/N$ share |
| Weight bank + routing insertion loss | 3 dB | rings near-resonance + waveguide loss |
| Power at detector pair per channel | $\approx 32$ μW | |
| Photocurrent (16 ch, $\mathcal{R} = 1$ A/W) | $\lesssim 0.5$ mA | comfortably above TIA noise floor at 5 GHz |
| Heater power (16 rings × 3 mW) | 48 mW | dominant static term |
| Modulator + TIA + driver | $\approx 30$–50 mW | CMOS-class analog front end |

Total: $\sim$100 mW per neuron, dominated not by light but by thermal tuning and analog electronics — the recurring theme of this chapter. At $16^2 = 256$ MACs per loop iteration and $\sim$5 GHz iteration rates, this is $\sim$0.1 pJ/MAC at the system level for a small network, improving with $N$ since electronics amortize over $N^2$ MACs. The $1/N$ broadcast split, however, works against scaling: doubling $N$ halves per-channel detected power (3 dB) while doubling summed photocurrent terms, so SNR per weight falls and the shot-noise precision analysis of Section 12.1 must be redone at each scale.

**Latency.** The loop time $\tau$ — modulator, few-mm bus, weight bank, detector, TIA, driver — is of order 100 ps to 1 ns. For *recurrent* processing this is the figure of merit: a photonic recurrent network iterates 10–100× faster than any electronic implementation of the same dynamics, which is why the architecture's early wins were in real-time control and RF signal processing rather than static image classification.

---

## 12.4.2.3 Demonstrations

**Recurrent dynamics on silicon (Tait et al. 2017).** The first silicon broadcast-and-weight system — a 4-node network with 16 tunable weights — was programmed to emulate a continuous-time recurrent neural network solving ordinary differential equations, verifying that autonomous recurrent optical dynamics follow the programmed weight matrix quantitatively.

**Fiber nonlinearity compensation (Huang et al. 2021).** The most compelling application demonstration to date: a silicon photonic-electronic neural network that equalizes nonlinear distortion in fiber-optic communication signals. Compensating the Kerr-induced distortion of a long-haul link requires a nonlinear model executed at tens of gigasamples per second — infeasible in real-time DSP, but natural for a photonic network whose inputs are already optical and whose loop latency is sub-nanosecond. The Princeton system improved signal quality on signals that had accumulated nonlinear distortion over a 10,000 km-class link emulation, operating directly at communication rates. This exemplifies the *strategic* use case for broadcast-and-weight: signals that are born analog, wideband, and optical, processed in-flight — not competing with GPUs on batched matrix arithmetic, but doing what electronics cannot do at all.

**RF fingerprinting and channelization (Princeton, ongoing).** Weight banks driven as blind source separators and RF classifiers, exploiting the same wideband advantage; these applications connect back to the microwave photonics of Chapter 11.

---

## 12.4.2.4 Assessment

Broadcast-and-weight occupies a distinctive niche in the design space. Its strengths: incoherence (no phase stabilization), recurrence for free (the bus is a loop), neuron-level modularity, mature WDM componentry, and the highest demonstrated weight precision in photonics ($>$9 bits, Subsection 12.4.1). Its structural constraints: neuron count per bus bounded by FSR ($N \sim$ tens), static heater power, O/E/O energy per neuron per pass, and weights that update at kHz–MHz rather than GHz rates. It is best read not as a GPU rival but as the physical layer for *neuromorphic* photonics — the spiking laser neurons and learning synapses of Unit VI plug directly into this fabric.

---

## References

[1] Tait, A.N., Nahmias, M.A., Shastri, B.J., & Prucnal, P.R. (2014). "Broadcast and weight: An integrated network for scalable photonic spike processing." *Journal of Lightwave Technology*, 32(21), 4029–4041. [The architecture paper: protocol, scaling analysis, spiking compatibility.]

[2] Tait, A.N., et al. (2017). "Neuromorphic photonic networks using silicon photonic weight banks." *Scientific Reports*, 7, 7430. [First silicon implementation; recurrent ODE emulation.]

[3] Huang, C., et al. (2021). "A silicon photonic–electronic neural network for fibre nonlinearity compensation." *Nature Electronics*, 4, 837–844. [The flagship application demonstration at communication line rates.]

[4] Prucnal, P.R., & Shastri, B.J. (2017). *Neuromorphic Photonics*. CRC Press. [Book-length treatment of broadcast-and-weight and its spiking extensions; the standard reference for this architecture.]

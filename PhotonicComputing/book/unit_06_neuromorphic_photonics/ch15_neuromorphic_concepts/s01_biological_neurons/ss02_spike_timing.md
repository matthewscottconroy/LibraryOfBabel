# 15.1.2 Spike Timing and Rate Coding

A spike train is a sequence of near-identical events; all of a neuron's output information must therefore live in *which* neurons fire and *when*. Exactly how a downstream neuron — or an engineer — should read that information is the **neural coding** question, and the answer shapes how a photonic spiking system is designed and clocked.

## Rate coding

The classical view, dating to Adrian's recordings of sensory receptors, is **rate coding**: the signal is the mean firing rate $f$, estimated by counting spikes $n$ in a window $T$, so $f = n/T$. A stronger stimulus produces a higher rate. Rate codes are attractive because they are **robust**: averaging over many spikes (or many neurons) suppresses the effect of jitter, dropped spikes, and stochastic synaptic release. The cost is **time and energy** — resolving a rate to fine precision requires many spikes, hence a long integration window and many signaling events.

## Temporal and latency codes

**Temporal coding** places information in the precise timing of spikes rather than their count. The most economical variant is **time-to-first-spike (TTFS)**, or **latency coding**: the interval between a stimulus onset (or a reference clock) and a neuron's first spike encodes the analog value, with stronger inputs firing sooner. A single well-timed spike can then carry several bits. **Population coding** distributes a value across the joint activity of many neurons — for example the relative latencies, or the identity of the first units to fire — combining speed with graceful degradation.

The trade-off is fundamental. Temporal codes are **fast and sparse** (few spikes, low latency, low energy) but demand **precise, low-jitter timing** and are correspondingly fragile; rate codes are **slow and dense** but **noise-tolerant**. Real nervous systems use both, and the fastest behaviors — the ~100–150 ms from photons striking the retina to object recognition, across roughly ten processing stages — leave time for only about one spike per neuron per stage, implying the brain leans on latency codes when speed matters (Gerstner & Kistler 2002).

## Why photonics favors temporal coding

Maass (1997) classified networks of spiking neurons as the "third generation" of neural models, strictly more powerful per unit than rate-based sigmoidal networks precisely because they can exploit spike times. Photonic neurons make this attractive in hardware: optical pulses can be a few picoseconds wide with picosecond-scale timing jitter, so a temporal code packs many resolvable time bins into a nanosecond. Where a biological latency code is limited by millisecond jitter, a photonic one is limited by picosecond jitter — three orders of magnitude finer — and the decision can be taken the instant the first pulse arrives. Fast temporal coding thus aligns naturally with the picosecond, event-driven strengths of the photonic neurons developed in the next section.

In practice the two regimes are endpoints of a continuum rather than a strict dichotomy. **Synchrony** or **coincidence coding** sits between them: a downstream neuron with a short integration window (small $\tau_m$) fires only when several inputs arrive within a narrow interval, reading correlations in spike times while staying insensitive to slow rate fluctuations. The engineering lesson for photonics is direct — pushing $\tau_m$ toward the picosecond regime turns the neuron itself into a fast coincidence detector, so temporal precision is not merely available in the hardware, it is the resource the hardware is best at exploiting.

## Worked Example: rate versus latency for a 4-bit value

Suppose a neuron must transmit an analog quantity to **4-bit resolution** (16 distinguishable levels) to a downstream decision unit.

**Rate code.** Encoding 16 levels as spike counts requires up to $n = 15$ spikes in the counting window. At a biological maximum rate $f_{max} = 100$ Hz, the window must be

$$T = \frac{n}{f_{max}} = \frac{15}{100\ \text{Hz}} = 150\ \text{ms}.$$

The message is $\log_2 16 = 4$ bits, delivered using up to 15 spikes: an efficiency of $4/15 \approx 0.27$ bits per spike, with a decision latency of 150 ms.

**Latency code.** Encode the same 4 bits in the timing of a **single** spike within a window, divided into bins of width equal to the timing jitter $\Delta t$. With biological jitter $\Delta t \approx 1$ ms, resolving 16 levels needs a window

$$T_w = 16 \times \Delta t = 16\ \text{ms},$$

carrying 4 bits in 1 spike — $4$ bits per spike, roughly **15× fewer spikes** and, because the decision is made on the first spike, about **10× lower latency** than the rate code. The price is a hard requirement for millisecond spike-time precision.

**Photonic scaling.** Replace the biological timescale with a photonic one. With pulse jitter $\Delta t \approx 10$ ps and a window $T_w \approx 1$ ns, the number of resolvable bins is $T_w/\Delta t = 100$, so a single optical spike carries $\log_2 100 \approx 6.6$ bits and the decision follows within about 1 ns. Because energy scales with the number of spikes (Section 15.2.3), the latency code's ~15× spike reduction is also a proportional energy saving at equal energy per spike — the reason temporal coding is so appealing for photonic spiking networks.

## References

Gerstner, W. & Kistler, W.M. (2002). *Spiking Neuron Models: Single Neurons, Populations, Plasticity*. Cambridge University Press.

Maass, W. (1997). "Networks of spiking neurons: the third generation of neural network models." *Neural Networks*, 10(9), 1659–1671.

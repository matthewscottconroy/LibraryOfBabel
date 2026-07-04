# Chapter 15: Important Concepts

---

## 1. The Leaky Integrate-and-Fire (LIF) Model

The LIF neuron is the workhorse abstraction of the entire unit. It treats the neuron's membrane as a leaky capacitor charged by the input current:
$$\tau_m \frac{dV}{dt} = -(V - V_{rest}) + R\,I(t),$$
where $\tau_m = RC$ is the membrane time constant ($R$ the membrane resistance, $C$ its capacitance). Sub-threshold, the potential relaxes toward $V_\infty = V_{rest} + R I$ with the characteristic leak $\propto e^{-t/\tau_m}$; when $V$ reaches the threshold $V_{th}$, the neuron emits a spike and $V$ is reset to $V_{reset}$. Biological values: $V_{rest} \approx -70$ mV, $V_{th} \approx -55$ mV, spike peak $\approx +30$–$40$ mV, $\tau_m \approx 10$–$20$ ms, refractory period $\approx 1$–$2$ ms. The LIF model is a deliberate reduction of the full Hodgkin–Huxley biophysics: it discards the ionic-channel dynamics that shape the spike itself and keeps only *integration to threshold*. Its steady-state $f$–$I$ curve, with refractory period $t_{ref}$,
$$f(I) = \left[\, t_{ref} + \tau_m \ln\!\frac{RI}{RI - (V_{th}-V_{rest})} \,\right]^{-1},$$
is a thresholded, saturating nonlinearity — the neuromorphic counterpart of the smooth activation functions used in the ONNs of Unit V.

---

## 2. Biophysical Roots: Hodgkin–Huxley and the Excitable Reductions

The Hodgkin–Huxley model (1952) describes the action potential with four coupled variables (membrane voltage plus sodium activation/inactivation and potassium activation gating variables) and is quantitatively faithful but analytically heavy. Two reductions matter for photonics. The LIF model (Concept 1, ultimately traceable to Lapicque, 1907) keeps only sub-threshold integration. The **FitzHugh–Nagumo** system (1961/1962) keeps two variables — a fast "voltage-like" activator and a slow "recovery" inhibitor — and captures the essential geometry of *excitability*: a stable rest state, an all-or-nothing pulse for super-threshold perturbations, and a refractory return. This two-timescale excitable structure is exactly what a semiconductor laser biased near threshold provides (a fast photon field and a slow carrier population), which is why the excitable lasers of Chapter 16 map so naturally onto FitzHugh–Nagumo / LIF dynamics.

---

## 3. Neural Coding: Rate, Temporal, and Population Codes

How does a spike train carry information? Three schemes recur:

| Coding scheme | Information carried in | Strength | Weakness |
|---|---|---|---|
| Rate code | mean firing rate $f$ over a window | robust to jitter and dropout | slow; needs many spikes → more energy |
| Temporal / latency code | precise spike times (e.g. time-to-first-spike) | fast; high bits per spike | fragile to timing noise |
| Population code | joint activity across many neurons | robust, high capacity | needs many neurons/channels |

A rate code integrating $N$ spikes resolves roughly $\sqrt{N}$ Poisson-distinguishable levels, so information grows only slowly with spike count and energy. A temporal code can pack many bits into the latency of a *single* spike: resolving one spike to $\Delta t$ within a window $T$ carries $\sim\log_2(T/\Delta t)$ bits. Because photonic neurons produce picosecond spikes with picosecond jitter, they are naturally matched to fast temporal and time-to-first-spike codes rather than to rate codes.

---

## 4. Spike-Timing-Dependent Plasticity (STDP)

STDP is the canonical *local, unsupervised* learning rule, established experimentally by Markram (1997) and Bi & Poo (1998). With $\Delta t = t_{post} - t_{pre}$:
$$\Delta w = \begin{cases} +A_+\, e^{-\Delta t/\tau_+}, & \Delta t > 0 \ \ (\text{pre before post} \to \text{potentiation, LTP}) \\[3pt] -A_-\, e^{+\Delta t/\tau_-}, & \Delta t < 0 \ \ (\text{post before pre} \to \text{depression, LTD}). \end{cases}$$
It is Hebbian ("cells that fire together wire together") but *causal*: only inputs that help *cause* a spike are strengthened. Biological time constants are $\tau_+ \approx 15$–$20$ ms and $\tau_- \approx 20$–$35$ ms, with amplitudes $A_\pm$ of a few percent. The usual asymmetry $A_-\tau_- > A_+\tau_+$ (integrated depression exceeds integrated potentiation) makes learning *competitive*: uncorrelated synapses drift downward while causally correlated ones strengthen, preventing runaway potentiation. Chapter 16 shows how this rule is realized all-optically by the timing overlap of pulses at a phase-change synapse.

---

## 5. Spiking Networks as the "Third Generation"

Maass (1997) organized neural models into three generations: (1) McCulloch–Pitts threshold gates, (2) analog units with continuous (sigmoidal) activations — the basis of conventional deep learning and of the ONNs in Unit V — and (3) *spiking* networks that compute with the timing of individual pulses. Maass showed the third generation is, for a given number of units, computationally *at least as powerful* as the second, and can be strictly more efficient for temporally coded problems. This result is the intellectual license for treating spiking photonic computing as its own paradigm rather than a slower way of doing matrix multiplies.

---

## 6. The Neuromorphic Hardware Landscape

Electronic neuromorphic engineering — the field Carver Mead named in 1990 — has produced several flagship platforms, each a different bet. The comparison below is qualitative; exact throughput and efficiency figures depend heavily on the workload.

| Platform | Style | Scale (per chip) | Distinctive trait | Spike timescale |
|---|---|---|---|---|
| Loihi / Loihi 2 | digital, asynchronous | ~130k → ~1M neurons | programmable on-chip learning | μs–ns |
| TrueNorth | digital, fixed-function | 1M neurons, 256M synapses | ~70 mW ultra-low power | ~μs |
| SpiNNaker | ARM software | (defined in software) | real-time biological simulation | ~ms (real-time) |
| BrainScaleS | analog, waferscale | waferscale | $10^3$–$10^4\times$ accelerated | sub-μs |
| Photonic (Unit VI) | analog optical | research-scale | ps spikes, WDM fan-out | ps |

What unites the electronic platforms is that their spikes are transistor-gated events moved over metal interconnect — bounding their timescale at microseconds to nanoseconds. Photonics aims to break exactly that bound.

---

## 7. The Photonic Speed Advantage

A biological spike lasts $\sim$1 ms; a photonic spike from an excitable laser can be $\sim$1–10 ps — roughly eight orders of magnitude faster, and the ultrafast-cognitive-computing argument first quantified for the leaky integrate-and-fire laser neuron (Nahmias et al., 2013). The physical basis is that the optical carrier at 1550 nm oscillates at $\sim$193 THz, and per-channel modulation bandwidths reach tens of GHz, so the dynamics that gate a spike (photon lifetime $\sim$ps, carrier lifetime $\sim$ns) are themselves fast. The payoff is not merely raw speed but *latency*: a multilayer photonic spiking network can pass a spike front to front in hundreds of picoseconds, where an electronic neuromorphic core would need microseconds — decisive for closed-loop control and RF front ends.

---

## 8. WDM Fan-out and Broadcast-and-Weight

Neurons fan out: one output must reach many synapses. Electronically, fan-out to $N$ targets means charging $N$ wire capacitances through a driver, an RC- and energy-limited operation that worsens with $N$. Optically, a single output can be split passively to many receivers, and — crucially — **wavelength-division multiplexing (WDM)** lets one waveguide carry $N$ independent channels, each wavelength addressing a different synapse. This is the substrate of the **broadcast-and-weight** architecture (Unit V, §12.4): each neuron emits at a distinct $\lambda_i$ onto a shared bus; each receiver has a bank of add–drop microrings, one tuned to each $\lambda_i$, whose drop fraction sets the synaptic weight; balanced photodetection of drop versus through ports yields *signed* ($\pm$) weights, and the summed photocurrent drives the neuron's spiking element. The C-band alone (~4.4 THz) holds on the order of forty 100-GHz channels, setting the practical fan-in per bus. Optical fan-out spends *power* ($\sim P_0/N$ per branch) rather than *bandwidth*.

---

## 9. The Energy-per-Spike Budget and the Sub-Femtojoule Goal

Because spiking computation is event-driven, the total energy of a photonic SNN is, to first order,
$$E_{total} \approx (\text{number of spikes}) \times (\text{energy per spike}).$$
Present laser-neuron demonstrations sit in the femtojoule-to-picojoule per spike range, with a widely cited target of **$<1$ fJ per spike**; optical-to-electrical-to-optical (O-E-O) conversion adds on the order of sub-pJ–pJ. Non-volatile PCM synapses complicate the ledger usefully: a *write* costs relatively much (on the order of nanojoules to crystallize/amorphize the film), but once set the weight holds at *zero* static power, so for deployed, fixed-weight inference the write energy amortizes to nearly nothing. The design goal for a competitive photonic spiking processor is therefore a rare combination: sub-fJ dynamic energy per event, near-zero weight-hold power, and enough sparsity that few neurons fire at once.

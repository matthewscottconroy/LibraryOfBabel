# Chapter 16: Important Concepts

---

## 1. Excitability and the FitzHugh–Nagumo / LIF Mapping

An *excitable* system rests at a stable fixed point, but a perturbation exceeding a threshold sends it on a large, stereotyped excursion — a spike — before it returns. This is precisely the behavior of a neuron, captured in reduced form by the two-variable FitzHugh–Nagumo model (a fast activation variable and a slow recovery variable) and, in its integrate-and-fire limit, by the LIF model

$$\tau_m \frac{dV}{dt} = -(V - V_{\text{rest}}) + R\,I(t), \qquad V \ge V_{th} \Rightarrow \text{spike, reset to } V_{\text{reset}}.$$

The central claim of Section 16.1 is that a semiconductor laser biased just below threshold is an excitable system of exactly this class: the optical field plays the role of the fast variable and the carrier density the slow variable. Nahmias et al. (2013) showed the saturable-absorber laser maps *rigorously* onto LIF, so that decades of computational neuroscience transfer directly to the photonic device — but with a membrane-like time constant set by picosecond-to-nanosecond laser physics rather than the $\sim 10$ ms of biology.

---

## 2. The Laser-Neuron Rate Equations and Fast/Slow Timescales

Excitability in a laser arises from a *separation of timescales*. In field–carrier form,

$$\dot{E} = \tfrac{1}{2}(G-1)E + F_E, \qquad \dot{N} = \frac{I}{e} - \frac{N}{\tau_s} - G|E|^2, \qquad G = g(N - N_{tr}),$$

the photon lifetime $\tau_p$ (on the order of picoseconds) is the fast timescale and the carrier lifetime $\tau_s$ (on the order of nanoseconds) is the slow, recovery timescale — with $F_E$ a Langevin noise term. The wide ratio $\tau_s/\tau_p \sim 10^2$–$10^3$ is what produces the sharp, all-or-nothing pulse followed by a slow refractory recovery. For a saturable-absorber neuron the standard model is the three-variable Yamada system (gain $G$, absorber $Q$, intensity $I$), whose gain–absorber competition gives clean excitable pulses. Linearizing about the sub-threshold fixed point yields a complex-conjugate eigenvalue pair (damped relaxation oscillations); the fixed point loses stability at a Hopf bifurcation, which marks the boundary between excitable (isolated-spike) and self-pulsating operation.

---

## 3. VCSEL Polarization Spiking

A vertical-cavity surface-emitting laser (VCSEL) supports two orthogonal linear polarization modes. Near threshold these modes compete, and a suitable optical injection can flip the laser abruptly from one polarization to the other and back — producing a fast optical spike in the target polarization (Hurtado & Javaloyes, 2015). Long-wavelength (1300 nm) VCSELs generate such spikes at sub-nanosecond timescales. The practical appeal is substantial: VCSELs are directly modulated, low-threshold, cheap, tested in surface-normal geometry, and — crucially — fabricable as dense two-dimensional arrays, which suits them to spiking convolutional layers. Robertson et al. (2020) demonstrated integration, inhibition, and pattern classification with such neurons.

---

## 4. PCM Synapse SET/RESET Physics and Non-Volatility

A phase-change synapse is a small patch of chalcogenide glass (canonically GST, Ge₂Sb₂Te₅) clad on a waveguide; the evanescent tail of the guided mode samples the film. The *amorphous* phase is low-loss and low in imaginary index; the *crystalline* phase is strongly absorbing. The waveguide transmission therefore encodes the weight, set by the crystalline fraction. Switching is thermal and is driven by optical pulses:

- **SET (amorphous → crystalline):** a longer, moderate-power pulse heats the film above its crystallization temperature ($\sim 150$–$200\,°$C) but below melting, giving atoms time to order. Increases absorption (lowers weight).
- **RESET (crystalline → amorphous):** a short, intense pulse briefly melts the film (exceeding $\sim 600\,°$C) and the subsequent rapid quench freezes it into the disordered amorphous state. Decreases absorption (raises weight).

The decisive property is **non-volatility**: once written, the phase persists for years with *zero* static power. The energy cost is paid once, at write time (on the order of nanojoules), not continuously — the opposite of a thermally tuned ring, which pays $\sim 1$ mW forever to hold a weight (Ríos et al., 2015).

---

## 5. GST vs. GSST Cascadability

GST's Achilles heel is that its crystalline state is strongly absorbing, so a chain of GST synapses accumulates large insertion loss — limiting how many can be cascaded on one optical path. GSST (Ge₂Sb₂Se₄Te₁, selenium-alloyed) is engineered to be low-loss in *both* states while retaining a large refractive-index contrast, enabling nearly phase-only, cascadable, low-insertion-loss synapses (Zhang et al., 2019).

| Property | GST (Ge₂Sb₂Te₅) | GSST (Ge₂Sb₂Se₄Te₁) |
|---|---|---|
| Amorphous loss @ 1550 nm | Low | Low |
| Crystalline loss @ 1550 nm | High (absorbing) | Low |
| Index contrast $\Delta n$ | Large | Large (on the order of $\sim 2$) |
| Dominant modulation | Amplitude (absorptive) | Phase-dominated |
| Cascadability | Limited by crystalline loss | Many cells (low per-cell loss) |
| Best role | Compact single-cell weight | Long weighted chains, switches |

The lesson: material choice is an architectural decision. Amplitude-modulating GST suits compact single-weight cells; phase-modulating GSST suits long cascaded paths and low-loss switch fabrics.

---

## 6. In-Memory Computing and the von Neumann Bottleneck

In a conventional (von Neumann) processor, weights live in memory and must be fetched to a separate arithmetic unit — data movement that dominates the energy budget of neural-network inference. A PCM photonic synapse collapses this: the weight is stored *in* the cell, and the multiplication happens *as light traverses the cell*, automatically and passively (Ríos et al., 2019; Wright et al., 2013). There is no read, no fetch, no bus traffic — the memory *is* the multiplier. Summation is equally physical: co-propagating or wavelength-multiplexed signals add on a shared waveguide or at a photodetector. The 4×4 photonic tensor core of Feldmann et al. (2021) is the archetype — a WDM-fed PCM matrix performing multiply–accumulate at very high throughput with the weights held non-volatile in place.

---

## 7. Multi-Level Analog Storage: Precision, Drift, and Endurance

Partial crystallization gives *analog* weights. With $N$ reliably distinguishable crystalline-fraction levels, a synapse stores $\log_2 N$ bits; devices in the Feldmann-2019/2021 regime demonstrate on the order of $\sim 5$ bits ($\sim 34$ levels). Precision is bounded not by the number of programmed levels but by the noise and instability that blur them:

| Impairment | Physical origin | Mitigation |
|---|---|---|
| Programming stochasticity | Random nucleation kinetics | Closed-loop program-and-verify |
| Transmission drift | Slow structural relaxation | Drift-aware encoding, periodic refresh |
| Cycle-to-cycle variability | Non-identical melt-quench | Averaging, guard-banding levels |
| Finite endurance | Material fatigue over $\sim 10^6$–$10^9$ cycles | Reserve write cycles for infrequent updates |

The usable bit-depth is the number of levels whose spacing exceeds a few times the read noise *after* drift over the deployment lifetime — typically fewer than the freshly programmed count, which is why fixed-weight inference (few writes) is the natural application.

---

## 8. WDM Broadcast-and-Weight and Signed Weights via Balanced Detection

To wire many neurons together, each neuron $i$ emits at a distinct wavelength $\lambda_i$; a shared bus broadcasts all wavelengths to every receiver. At a receiving neuron, a bank of add–drop microrings — one tuned to each $\lambda_i$ — sets the synaptic weight by its drop fraction. Reading the *drop* and *through* ports with a **balanced photodetector pair** yields a photocurrent proportional to the *difference*, so the effective weight is

$$w_i \propto \eta_{\text{drop},i} - \eta_{\text{through},i} \in [-1, +1],$$

i.e. genuinely **signed** — essential for excitation *and* inhibition. The summed photocurrent $I \propto \sum_i w_i P_i$ is the synaptic weighted sum, which then drives the neuron's nonlinear/spiking element (an excitable laser or a modulator neuron). This is the spiking use of the broadcast-and-weight fabric introduced for matrix–vector products in Unit V (§12.4). The number of WDM channels is set by the ring FSR divided by the per-channel spacing needed to keep crosstalk low; Tait et al. (2017) demonstrated a 49-node silicon weight-bank network.

---

## 9. Optical STDP vs. Surrogate-Gradient Training

Photonic SNNs can be trained by two fundamentally different philosophies:

| | Optical STDP | Surrogate-gradient (BPTT) |
|---|---|---|
| Locality | Local (pre/post coincidence) | Global (backprop through network) |
| Supervision | Unsupervised (Hebbian) | Supervised (labeled targets) |
| Where it runs | On the hardware, online | Offline, in software |
| Mechanism | Pulse timing sets PCM SET/RESET | Replace Heaviside $\Theta'$ with smooth $\sigma'$ |
| Photonic demo | Feldmann et al. (2019) | Weights deployed after training |

**Optical STDP** exploits the physics directly: a pre-synaptic pulse (at $\lambda_{\text{pre}}$) and a post-synaptic pulse (at $\lambda_{\text{post}}$) arrive at a PCM synapse with a relative delay $\Delta t$ set by propagation; their overlap determines whether the deposited energy crystallizes (potentiate) or amorphizes (depress) the film, so $\Delta w$ depends on $\Delta t$ — an all-optical Hebbian rule.

**Surrogate-gradient learning** confronts the fact that the spike function $S = \Theta(V - V_{th})$ is non-differentiable: its derivative is zero almost everywhere. Neftci et al. (2019) and Zenke & Ganguli (2018) replace that derivative in the *backward pass only* with a smooth surrogate — e.g. the fast-sigmoid derivative $\sigma'(V - V_{th}) = (\,\beta\,|V-V_{th}| + 1)^{-2}$ — so that backpropagation-through-time can train the SNN offline. The trained weights are then written to the photonic hardware. STDP is local and physical; surrogate gradients are global and accurate — and in practice the two are complementary.

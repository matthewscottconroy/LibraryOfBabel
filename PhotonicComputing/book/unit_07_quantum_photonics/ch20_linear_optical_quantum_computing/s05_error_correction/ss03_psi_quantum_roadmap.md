# 20.5.3 PsiQuantum's Fault-Tolerant Roadmap

## A Bet on Manufacturability

PsiQuantum, founded in 2016 by Jeremy O'Brien, Terry Rudolph, Mark Thompson, and Pete Shadbolt, made a wager that distinguishes it from most of the field: that the path to a *useful* quantum computer runs not through improving a few exquisite qubits but through **manufacturing billions of mediocre ones** in an existing semiconductor foundry. The reasoning follows this chapter. A utility-scale machine will need on the order of a million physical qubits (Section 20.5.2); no laboratory technique assembles a million of anything; therefore the physical layer must be built in a commercial CMOS process, on 300 mm silicon wafers, using the fabrication, yield management, and metrology that the electronics industry already possesses. PsiQuantum partnered with GlobalFoundries to fabricate its photonic and electronic chips in a standard line, and in 2025 reported in *Nature* a manufacturable, foundry-produced platform integrating the required single-photon sources, switches, and detectors at wafer scale — the demonstration that the components can be made by the million rather than the dozen.

## The Stack

The architecture is FBQC (Section 20.3.3) end to end, and every layer is chosen for foundry compatibility:

- **Sources.** Multiplexed spontaneous parametric down-conversion in silicon or silicon-nitride waveguides. Individual heralded pair sources are probabilistic and low-yield, so many are multiplexed — spatially and temporally — with fast, low-loss switches to synthesize near-deterministic single photons (Chapter 19). Determinism is bought from probabilism by redundancy, exactly the multiplexing logic of Section 19.1.4.
- **Circuits.** Silicon-photonic interferometers and switch networks route and interfere photons, performing the boosted fusions of Section 20.3.3. Every element is a line item in the loss budget of Section 20.5.1, which is why the platform lives or dies on sub-decibel components.
- **Detectors.** Superconducting nanowire single-photon detectors ($>98\%$ efficiency), which force the whole chip into a cryostat at a few kelvin — cold, but far warmer than the millikelvin of superconducting qubits, and set by the detectors rather than the qubits themselves.
- **Logic.** Fusion networks whose outcomes extract the checks of a foliated surface code (Section 20.5.2), correcting loss-as-erasure and residual Pauli errors in the same measurement pattern that runs the algorithm.

## Interleaving: Trading Space for Time

A million physical qubits' worth of *simultaneous* hardware is infeasible, so PsiQuantum leans on **interleaving** (Bombin et al., 2021): time-multiplexing the fault-tolerant lattice through a comparatively small bank of components using fiber delay lines. A resource state generated now is stored in a length of fiber and fused with a state generated later, so a single physical module, reused thousands of times per second, emulates a large block of the code lattice. Interleaving trades chip area for optical delay and clock speed — a few thousand fast components can host a logical qubit that a naive layout would spend millions on — and is the architectural lever that makes a million-qubit machine a question of throughput rather than of chip count. It also raises the loss stakes, since stored photons accrue fiber loss while they wait, tightening the budget of Section 20.5.1 further.

## Worked Example: The Photon Throughput of a Utility Machine

Estimate the raw photon rate a fault-tolerant photonic computer must sustain. Target $\sim 10^3$ logical qubits at code distance $d \approx 30$. From Section 20.5.2 each logical qubit occupies $\approx d^2 \approx 900$ physical qubits, so the machine holds

$$N_{\text{phys}} \approx 10^3 \times 900 \approx 10^6 \text{ physical qubits.}$$

In FBQC each physical qubit is not a stored object but a *stream*: every code cycle it is rebuilt from a fresh resource state (say $\sim 6$ photons) and consumed by fusion. Running the surface-code cycle at a clock $f_{\text{cyc}} \sim 1$ MHz, the aggregate photon-generation rate is

$$R_\gamma \sim N_{\text{phys}} \times (\text{photons/qubit/cycle}) \times f_{\text{cyc}} \approx 10^6 \times 6 \times 10^{6} \approx 6\times 10^{12}\ \text{photons/s},$$

and pushing the fusion clock toward GHz, or the logical count toward the $\sim 10^4$ qubits a cryptographically relevant algorithm needs, drives the requirement into the $10^{13}$–$10^{15}$ photons/s range. With per-source rates of $\sim 10^{9}$–$10^{10}$ Hz, delivering this demands $10^3$–$10^6$ multiplexed sources firing in concert — an unprecedented photonic throughput, and the crux of why the whole enterprise reduces to foundry-scale manufacturing. For scale, that photon flux is comparable to the number of transistor switching events in a large microprocessor each second — which is precisely the analogy the foundry bet is built on. (These are order-of-magnitude figures; the architectural specifics are laid out in Bartolucci et al. (2023) and Bombin et al. (2021).)

## Contrast: Xanadu's Continuous-Variable Road

PsiQuantum's is the discrete-variable path — single photons, dual-rail qubits, fusion, surface codes. Xanadu pursues the **continuous-variable** alternative: encode qubits in the quadratures of the field (GKP states) built from the squeezed light of Gaussian boson sampling (Section 20.4.2), stitched into cluster states by deterministic Gaussian operations. The two programmes share silicon photonics and SNSPDs but differ in what a qubit *is* and in how loss is fought — CV codes battle finite squeezing where DV codes battle photon loss. Chapter 21 develops the continuous-variable route in full; the point here is that photonic fault tolerance is not a single design but a genuine fork, and PsiQuantum's discrete, foundry-first bet is one of its two leading branches. The two roads even fail differently: the discrete route must drive photon loss below the erasure threshold of Section 20.5.1, while the continuous route battles finite squeezing, since a GKP qubit is only as sharp as the squeezing that defines its grid. Which constraint proves the softer is, as of this writing, an open experimental question.

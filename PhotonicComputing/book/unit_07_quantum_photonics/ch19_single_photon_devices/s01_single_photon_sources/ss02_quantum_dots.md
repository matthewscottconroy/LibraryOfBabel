# 19.1.2 Semiconductor Quantum Dots: Artificial Atoms on Demand

## The Device

A self-assembled quantum dot (QD) is a nanoscale island of low-bandgap semiconductor — typically InAs or InGaAs, ~20 nm across and a few nm tall — embedded in a higher-bandgap matrix (GaAs). Grown by molecular beam epitaxy in the Stranski-Krastanov mode (strain-driven island formation) or by droplet etching (which yields more symmetric dots), the island confines carriers in all three dimensions. Confinement on the scale of the exciton Bohr radius discretizes the electronic spectrum: the QD is an *artificial atom*, with sharp optical transitions at cryogenic temperature (typically 4 K), but one that is lithographically locatable, electrically contactable, and embeddable inside photonic structures.

Emission wavelengths are set by composition and size: 900–950 nm for InGaAs/GaAs dots (the most mature system), and 1300–1550 nm for InAs dots on InP — directly in the telecom bands — at the cost of somewhat less mature performance. Quantum frequency conversion (difference-frequency generation in PPLN waveguides) is routinely used to shift 900-nm QD photons to 1550 nm with conversion efficiencies above 50%.

## Single-Photon Emission Mechanism

Optical or electrical excitation creates an exciton (bound electron-hole pair) in the dot. The exciton recombines radiatively with lifetime $T_1 \approx 1$ ns (free space), emitting exactly one photon at the transition energy. A second photon at the same frequency cannot be emitted because the doubly excited state — the biexciton — is shifted by the Coulomb interaction by several meV, far outside the transition linewidth. This *Coulomb blockade of the optical transition* is what makes the QD a turnstile: one trigger, one photon. The first demonstration of triggered single photons from a QD (Michler et al., *Science*, 2000) launched the field; the demonstration that consecutive QD photons interfere (Santori et al., *Nature*, 2002) made it relevant to quantum computing.

The biexciton-exciton cascade (XX → X → ground) is itself useful: the two photons of the cascade are polarization-entangled if the intermediate exciton levels are degenerate, making QDs on-demand entangled-pair sources — a capability SPDC provides only probabilistically.

## Excitation Schemes and Purity

How the dot is pumped determines the purity and coherence of what comes out:

- **Above-band excitation:** pump the GaAs matrix; carriers relax into the dot. Simple, but relaxation timing jitter and charge noise degrade indistinguishability ($M \sim 0.5$–0.8).
- **Resonant excitation (resonance fluorescence):** drive the exciton transition itself with a $\pi$-pulse. Preparation fidelity >98%, near-transform-limited photons, $g^{(2)}(0)$ down to $7.5\times10^{-5}$ (Schweickert et al., 2018). The experimental price: separating the emitted photons from the resonant pump requires polarization rejection (~$10^7$:1) or spatially orthogonal excitation.
- **Phonon-assisted and two-photon (cascade) excitation:** detuned schemes that avoid pump rejection while retaining high fidelity; two-photon excitation of the biexciton is standard for entangled-pair generation.

Re-excitation during a single pulse is the residual purity limit: the dot can emit, be re-excited by the tail of the same pulse, and emit again. Short pulses (relative to $T_1$) suppress this to the $10^{-4}$ level.

## The Cavity: From Good Emitter to Great Source

A bare QD in bulk GaAs delivers ~2% of its photons to a collection lens (total internal reflection, Section 19.1.1) and dephases before it emits (charge and phonon noise give $T_2 < 2T_1$). Both problems have one solution: couple the dot to a microcavity and exploit the Purcell effect (Section 19.3.3).

With Purcell factor $F_P$, the emission rate into the cavity mode is $F_P \Gamma_0$, so the fraction of photons funneled into that single collectable mode is

$$\beta = \frac{F_P}{F_P + 1} \xrightarrow{F_P = 10} 0.91,$$

and the shortened lifetime ($T_1 \to T_1/F_P \approx 100$ ps for $F_P = 10$) outruns slow dephasing, pushing $M$ toward 1. The workhorse geometries:

- **Micropillar cavities** (distributed Bragg reflector stacks etched into pillars a few μm across): $Q \sim 10^3$–$10^4$, $V \sim 10\,(\lambda/n)^3$. Somaschi et al. (2016) and Ding et al. (2016) used electrically tuned QD-micropillar devices to demonstrate, simultaneously, $g^{(2)}(0) < 0.01$, $M > 0.98$, and first-lens brightness >0.6 — the "near-optimal" single-photon sources.
- **Open fiber-based microcavities:** tunable in situ; Tomm et al. (2021) achieved 57% end-to-end efficiency into a single-mode fiber with $M \approx 0.97$ at 76 MHz repetition — the current benchmark for delivered brightness.
- **Photonic crystal waveguides:** instead of a cavity, a slow-light waveguide captures emission with $\beta > 0.98$ (near-unity coupling demonstrated by the Lodahl group), naturally suited to on-chip architectures.
- **Bullseye (circular Bragg grating) cavities:** broadband, modest $F_P$, well matched to the biexciton cascade for entangled pairs.

## Worked Example: A Micropillar Source Budget

Take a QD with free-space lifetime $T_1 = 1$ ns in a micropillar with $F_P = 12$ on resonance.

- Cavity-enhanced lifetime: $T_1' = T_1/F_P \approx 83$ ps, so the source can in principle run at $\sim$1 GHz repetition.
- Mode coupling: $\beta = 12/13 = 0.92$.
- With pillar out-coupling efficiency 0.85, preparation fidelity 0.98, and fiber coupling 0.8: $\eta \approx 0.98 \times 0.92 \times 0.85 \times 0.8 \approx 0.61$ — matching the scale of the best reported devices.
- If pure dephasing at 4 K contributes $\Gamma^* = (2\,\text{ns})^{-1}$, then $M = \Gamma'/(\Gamma' + 2\Gamma^*)$ with $\Gamma' = 1/T_1' = (83\,\text{ps})^{-1}$: $M \approx 0.92$ — and increasing $F_P$ raises it further. Purcell enhancement is doing double duty: brightness *and* indistinguishability.

## The Scaling Problem

Every quantum dot is different. Random size and composition fluctuations spread transition energies over tens of meV, while photonic quantum computing needs many *identical* photons from *many* sources. Mitigations: local tuning (Stark shift via electrical gates, strain tuning via piezoelectric substrates) to bring dots into mutual resonance — remote two-dot HOM visibilities of ~93% have been achieved this way — or the "single source, many photons" strategy: demultiplex one excellent dot's pulse train into $n$ spatial modes with fast switches. The 20-photon boson sampling experiments of the USTC group (Wang et al., 2019) used exactly this approach. Deterministic positioning (in-situ lithography, as in the Senellart group's devices) addresses spatial randomness; spectral randomness remains the central obstacle to wafer-scale QD source arrays.

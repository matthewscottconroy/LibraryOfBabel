# Chapter 19: Important Concepts

---

## 1. The Source Trilemma: Bright, Pure, Indistinguishable

A single-photon source is graded on end-to-end efficiency $\eta$ (photon delivered per trigger), purity $g^{(2)}(0)$ (multi-photon contamination; $<0.01$ required for quantum computing), and indistinguishability $M = |\langle\psi_1|\psi_2\rangle|^2$ (HOM visibility between successive photons). Each is separately achievable; physics couples them — filtering buys purity with brightness, pump power buys brightness with purity, and dephasing steals indistinguishability unless emission is faster than the noise ($M = T_2/2T_1$). Record values: $\eta \approx 0.57$ fiber-coupled (Tomm 2021), $g^{(2)}(0) < 10^{-4}$, $M > 0.99$ — but not all in one device.

---

## 2. $\eta^n$: The Exponential Cost of Loss

An $n$-photon protocol succeeds at rate $R\,\eta^n$. At $n = 20$, improving $\eta$ from 0.5 to 0.98 gains a factor of ~$10^6$ — more than any conceivable increase in clock rate. This single scaling law explains the field's obsession with percent-level efficiency improvements, why detector SDE >98% matters, and why fault-tolerant architectures demand per-component losses in the $10^{-3}$ range.

---

## 3. Heralded SPDC and $g^{(2)}_h(0) \approx 2\mu$

Parametric pair sources fire randomly with mean pair number $\mu$ per pulse; detecting the idler *heralds* the signal photon. With threshold heralding the purity is $g^{(2)}_h(0) = 2\mu/(1+\mu)$, so $g^{(2)} < 0.01$ caps the firing probability near 0.5% — the fundamental brightness-purity trade-off of all parametric sources. Photon-number-resolving heralds veto multi-pair events and break the trade-off; group-velocity-matched crystals (factorable joint spectral amplitude) give heralded spectral purity >99% without filtering.

---

## 4. Multiplexing: Determinism from Probabilism

$N$ heralded sources (or time bins) with firing probability $p$ deliver a photon with probability $1-(1-p)^N$, routed by a $\log_2 N$-deep switch network. The scheme's viability is set entirely by switch loss: at 0.09 dB/switch a 500-fold multiplexer delivers ~83%; at 0.46 dB it delivers ~39% and loses to a good quantum dot. Fast, ultra-low-loss switches — not sources — are the critical component of multiplexed photonics.

---

## 5. The Quantum Dot as Artificial Atom

Three-dimensional confinement discretizes the spectrum; Coulomb shift of the biexciton enforces one-photon-at-a-time emission. Resonant $\pi$-pulse excitation gives preparation fidelity >98% and $g^{(2)}(0)$ down to $7.5\times10^{-5}$. The unsolved scaling problem is inhomogeneity: no two dots are identical, so architectures either tune dots into resonance (Stark, strain) or demultiplex one excellent dot into many modes.

---

## 6. SNSPD Performance Envelope

A current-biased superconducting nanowire (NbN, NbTiN, WSi/MoSi; ~5 nm × 100 nm) converts one absorbed photon into a resistive hotspot and a voltage pulse. State of the art: SDE >98% at 1550 nm (cavity-integrated), dark counts <1 cps (down to $10^{-4}$ with cold filtering), jitter ~15 ps system / ~3 ps record, reset 10–50 ns set by kinetic inductance $\tau = L_k/R_{load}$, wavelength coverage UV to mid-IR, arrays to 400,000 pixels. SNSPDs are the reason "detection" is no longer the weak link of quantum photonics — at the price of 0.8–4 K operation.

---

## 7. SDE Factorization

System detection efficiency multiplies three terms: $\eta_{SDE} = \eta_{couple}\,\eta_{absorb}\,\eta_{internal}$. Cavity stacks solve absorption for fiber-coupled devices; traveling-wave evanescent absorption ($1-e^{-\alpha L}$, ~1 dB/μm) solves it for waveguide-integrated devices (91–99% on-chip, Pernice 2012). Amorphous superconductors give saturated internal efficiency over wide bias ranges. The same decomposition organizes every detector datasheet you will ever read.

---

## 8. Photon-Number Resolution

Threshold detectors answer "any photons?"; PNR detectors answer "how many?" — required for vetoing multi-pair heralds, for KLM ancilla measurements, and for Gaussian boson sampling statistics. True PNR: the transition-edge sensor (energy-proportional readout, 95–98% efficiency, ~μs recovery, ~100 mK). Quasi-PNR: segmented SNSPD arrays, with collision error $\sim\binom{n}{2}/N$ for $n$ photons on $N$ segments, at full nanowire speed.

---

## 9. The Jaynes-Cummings Ladder

One emitter + one mode: $\hat{H}_{JC} = \hbar\omega_c\hat{a}^\dagger\hat{a} + \tfrac{\hbar\omega_a}{2}\hat{\sigma}_z + \hbar g(\hat{a}^\dagger\hat{\sigma}_- + \hat{a}\hat{\sigma}_+)$, with $g \propto 1/\sqrt{V}$. Dressed states split by $2g\sqrt{n+1}$: vacuum Rabi splitting ($2g$), reversible spontaneous emission (Rabi oscillation between $|e,0\rangle$ and $|g,1\rangle$), collapse-and-revival as a witness of field quantization, and — because the ladder is anharmonic — a nonlinearity that operates photon by photon.

---

## 10. Strong Coupling, Cooperativity, and Photon Blockade

Strong coupling ($g \gg \kappa, \gamma$) makes the dressed doublet spectrally resolvable; cooperativity $C = 4g^2/\kappa\gamma > 1$ is the device-relevant threshold. The anharmonic ladder detunes the second photon: photon blockade, demonstrated with one atom ($g^{(2)}(0) \approx 0.13$, Birnbaum 2005), enabling deterministic atom-photon and photon-photon gates (Duan-Kimble protocol, Hacker 2016) — the deterministic alternative to Chapter 20's probabilistic linear-optics gates.

---

## 11. The Purcell Effect and the β-Factor

Spontaneous emission is environment-dependent: $F_P = \frac{3}{4\pi^2}(\lambda/n)^3 Q/V$ enhances the decay rate into a resonant cavity mode ($\Gamma_c = 4g^2/\kappa$ in the bad-cavity limit), subject to spectral alignment, spatial alignment, and emitter linewidth narrower than the cavity's. The collectable fraction $\beta = F_P/(F_P+1)$ saturates fast ($F_P = 10 \Rightarrow \beta = 0.91$), and the shortened lifetime simultaneously raises indistinguishability and repetition rate. Every leading single-photon source is a weak-coupling Purcell device — strong coupling is for gates, Purcell is for sources.

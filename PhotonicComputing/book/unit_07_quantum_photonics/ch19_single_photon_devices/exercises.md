# Chapter 19: Exercises

---

## Mathematical Exercises

**M19.1 — Purcell Factor and Cavity-Enhanced Lifetime**

A quantum dot ($\lambda = 930$ nm, free-space lifetime $T_1 = 1$ ns, host index $n = 3.5$) sits at the antinode of a photonic crystal cavity with $Q = 30{,}000$ and mode volume $V = 0.05\,\mu\text{m}^3$.

(a) Express $V$ in units of $(\lambda/n)^3$ and compute the ideal Purcell factor $F_P = \frac{3}{4\pi^2}(\lambda/n)^3\,Q/V$.

(b) Compute the cavity-enhanced lifetime and the β-factor $\beta = F_P/(F_P+1)$.

(c) The dot is detuned from the cavity by $\delta/2\pi = 50$ GHz. Using the Lorentzian reduction factor $1/(1 + 4\delta^2/\kappa^2)$ with $\kappa = \omega/Q$, recompute the effective $F_P$. Comment on the tuning precision this device demands.

**M19.2 — Detector Budget for a Clocked Photonic Processor**

An SNSPD has SDE = 95%, DCR = 10 cps, timing jitter 15 ps (FWHM), and reset time 30 ns. A photonic circuit delivers one photon per clock cycle at 100 MHz to this detector.

(a) What fraction of photons is missed due to detection inefficiency? Due to dead time (assume Poissonian arrivals at the full clock rate and estimate the probability that a photon arrives within 30 ns of a previous detection)?

(b) What is the probability per clock cycle that a dark count lands inside a 100 ps acceptance window? How many false events per second is that?

(c) If the protocol requires distinguishing photons in time bins separated by 50 ps, is the jitter adequate? Quantify the overlap error assuming Gaussian timing response.

**M19.3 — Jaynes-Cummings Eigenvalues**

Starting from $\hat{H}_{JC}$ with detuning $\Delta = \omega_a - \omega_c$, restrict to the one-excitation subspace $\{|e,0\rangle, |g,1\rangle\}$.

(a) Write the 2×2 Hamiltonian block and diagonalize it. Show that the eigenvalue splitting is $\hbar\sqrt{4g^2 + \Delta^2}$, reducing to the vacuum Rabi splitting $2\hbar g$ on resonance.

(b) Show that for $|\Delta| \gg g$ the eigenstates reduce to weakly dressed atom and photon states, with energy shifts $\pm\hbar g^2/\Delta$ (the dispersive shift). Why is this regime useful for *nondestructive* photon detection?

**M19.4 — Multiplexed SPDC Source**

Each of $N$ heralded SPDC sources fires (heralds a signal photon) with probability $p = 0.01$ per clock, with heralded $g^{(2)}_h(0) = 2\mu \approx 0.01$.

(a) Compute the probability $P_N = 1 - (1-p)^N$ that at least one source fires, for $N$ = 100, 300, 500.

(b) The winning photon traverses a $\lceil\log_2 N\rceil$-deep switch tree with transmission $\eta_{sw}$ per stage. For $\eta_{sw} = 0.98$ and $0.90$, compute the delivered single-photon probability for $N = 500$.

(c) Show that the multiplexed source's $g^{(2)}(0)$ remains that of a single source (to first order), and explain physically why multiplexing boosts brightness without degrading purity.

**M19.5 — Heralded $g^{(2)}(0)$ of an SPDC Source**

The two-mode squeezed vacuum has pair-number distribution $p_n = \mu^n/(1+\mu)^{n+1}$.

(a) An ideal threshold detector heralds on the idler. Show that the conditional signal state gives $g^{(2)}_h(0) = 2\mu/(1+\mu)$. (Compute $\langle n\rangle$ and $\langle n(n-1)\rangle$ over the renormalized distribution $p_n/(1-p_0)$, $n \geq 1$.)

(b) What $\mu$ keeps $g^{(2)}_h(0) < 0.01$? At a 1 GHz pulse rate with heralding efficiency 0.8, what is the heralded single-photon rate at that $\mu$?

(c) Now herald with an ideal photon-number-resolving detector that accepts only $n = 1$. What is $g^{(2)}_h(0)$, and what imperfection reintroduces multi-photon contamination in practice?

**M19.6 — HOM Visibility with Imperfect Photons**

Two photons with identical Gaussian spectral amplitudes of bandwidth $\sigma$ (rms) arrive at a 50/50 beam splitter with relative delay $\tau$.

(a) Show that the coincidence probability is $P_c(\tau) = \frac{1}{2}\left(1 - e^{-\sigma^2\tau^2}\right)$, i.e. a HOM dip of visibility 1 and width $\sim 1/\sigma$.

(b) A detector-timing jitter of 15 ps cannot resolve the dip directly for $1/\sigma = 3$ ps photons. Explain why the dip is nevertheless measurable, and what *does* limit the measurement.

(c) If the two photons additionally have indistinguishability $M = 0.95$ in their other degrees of freedom, what is the observed dip visibility?

**M19.7 — The Exponential Tyranny of $\eta^n$**

An $n$-photon experiment runs at repetition rate $R = 76$ MHz with per-photon system efficiency $\eta$.

(a) Tabulate the $n$-fold coincidence rate for $n = 10$ and $n = 20$ at $\eta = 0.3, 0.6, 0.9, 0.98$.

(b) A team can spend one year either doubling the repetition rate or improving $\eta$ from 0.60 to 0.66. For $n = 20$, which purchase wins, and by what factor?

(c) Derive the general rule: a fractional efficiency improvement $\delta\eta/\eta$ is worth the same as a repetition-rate factor of $(1 + \delta\eta/\eta)^n$.

**M19.8 — Dark Counts vs. Distance in a Heralded Link**

A single-photon signal at 1550 nm travels through fiber with 0.2 dB/km loss to a detector with SDE 90%, gated with a 200 ps window at 1 MHz.

(a) Write the signal rate as a function of distance $L$ and find the distance at which the signal rate equals the dark-count rate in the gate for DCR = 1 cps (SNSPD) and DCR = 3000 cps (InGaAs SPAD).

(b) Relate your result to the maximum range of trusted-node-free QKD (Chapter 22 preview): what does the detector choice buy in kilometers?

---

## Conceptual Exercises

**C19.9 — The Source Trilemma**

Why must a source for photonic quantum computing be simultaneously bright, pure, and indistinguishable? For each *pair* of metrics, identify a physical mechanism that improves one at the other's expense (e.g., spectral filtering, pump power, cavity linewidth), and explain why Purcell enhancement is the rare intervention that improves several at once.

**C19.10 — Choosing a Detector**

You are architecting (i) a room-temperature handheld QKD receiver at 850 nm, (ii) a 100-channel fusion-based quantum computer module at 1550 nm, and (iii) a Gaussian boson sampler whose output statistics must be photon-number-resolved. Choose a detector technology for each, and defend the choice against the scorecard metrics (SDE, DCR, jitter, PNR, operating temperature, channel count).

**C19.11 — Threshold Heralds and Hidden Photons**

Explain why a threshold ("click/no-click") herald on an SPDC idler cannot distinguish one pair from two, and trace the consequence through to a two-qubit photonic gate error. Then explain how (a) a TES and (b) a segmented SNSPD array mitigate this, and estimate the residual two-photon acceptance of a 10-segment array.

**C19.12 — Strong Coupling vs. Purcell for Devices**

The best single-photon sources operate in the *weak*-coupling (Purcell) regime, while deterministic photon-photon gates require the *strong*-coupling regime. Explain the physical reason for each preference, using the rate hierarchy $g$, $\kappa$, $\gamma$, $\gamma^*$ and the fate of a photon re-absorbed by the emitter.

---

## Programming Projects

**P19.1 — Purcell Design Optimizer**

For cavity families spanning (Fabry-Pérot: $Q = 10^3$–$10^5$, $V = 10^2$–$10^4\,(\lambda/n)^3$; micropillar: $Q = 10^3$–$3\times10^4$, $V = 5$–$50$; photonic crystal: $Q = 10^3$–$10^6$, $V = 0.3$–$2$), compute $F_P$, $\beta$, enhanced lifetime, and expected indistinguishability $M = \Gamma'/(\Gamma' + 2\gamma^*)$ for a quantum dot with $T_1 = 1$ ns and $\gamma^* = (2\,\text{ns})^{-1}$. Map the region of design space achieving $\beta > 0.95$, $M > 0.99$, and repetition rate > 1 GHz simultaneously.

**P19.2 — Jaynes-Cummings Dynamics in QuTiP**

Simulate the driven, dissipative Jaynes-Cummings model ($g/2\pi = 5$ GHz, vary $\kappa$, $\gamma$). Reproduce: (a) vacuum Rabi oscillations and their damping across the strong-to-weak coupling transition; (b) collapse and revival for a coherent-state field with $\bar{n} = 15$ (closed system); (c) photon blockade — compute $g^{(2)}(0)$ of the cavity output vs. drive detuning and locate the antibunching dip at the dressed-state resonance.

**P19.3 — Waveguide SNSPD Efficiency Model**

Model a waveguide-integrated SNSPD as cascaded efficiencies: fiber-chip coupling (grating vs. edge coupler, 0.5–3 dB), evanescent absorption $1 - e^{-\alpha L}$ with $\alpha(\lambda)$ from a simple mode-overlap model, and internal efficiency as a sigmoid in bias current. Optimize nanowire length against kinetic-inductance-limited reset time ($L_k \propto$ length), and plot the SDE/count-rate Pareto frontier at 1550 nm.

**P19.4 — Monte Carlo Multiplexed Source**

Simulate $N$-fold spatial multiplexing of heralded SPDC sources including threshold vs. PNR heralds, switch loss per stage, and detector dark counts. Reproduce the delivered-probability curves of M19.4 and find, for $\eta_{sw} \in [0.9, 0.999]$, the switch transmission at which multiplexing beats a single quantum-dot source with $\eta = 0.57$.

# Chapter 22: Important Concepts

---

## 1. Non-Orthogonality, Information–Disturbance, and No-Cloning

The security of quantum communication rests on one geometric fact: non-orthogonal states cannot be reliably distinguished — $|\langle H|+\rangle|^2 = \tfrac12$, so a photon in an unknown conjugate basis carries information no measurement can fully extract. The no-cloning theorem (Wootters–Zurek) makes this structural: unitarity forbids $\hat U|\psi\rangle|b\rangle = |\psi\rangle|\psi\rangle$ for arbitrary $|\psi\rangle$ (else $\langle\phi|\psi\rangle = \langle\phi|\psi\rangle^2$, forcing states to be identical or orthogonal). Hence any eavesdropper's information gain implies disturbance; the optimal universal cloner reaches only fidelity $5/6$, inflicting a $\sim 16.7\%$ QBER — above BB84's abort threshold.

---

## 2. BB84: Conjugate Bases, Sifting, and the Secret-Key Fraction

Alice encodes random bits in randomly chosen $Z$/$X$ bases; Bob measures randomly; public *sifting* keeps the $\sim\!50\%$ of events with matched bases. A sacrificed sample estimates the QBER $Q$, and privacy amplification distils the secret key. Intercept-resend forces $Q = 25\%$; the one-way secret fraction is $r \ge 1 - 2h(Q)$, with $h$ the binary entropy, vanishing at the famous $Q = 11\%$ threshold ($r \approx 0.61$ at $Q = 3\%$, $r \approx 0.06$ at $Q = 10\%$). Security is information-theoretic (Shor–Preskill). E91/BBM92 recast the same logic with entangled pairs and a CHSH test, extending to device independence.

---

## 3. Decoy States and the Defeat of the PNS Attack

Real sources are attenuated lasers with Poissonian photon number, $P(n) = e^{-\mu}\mu^n/n!$; multiphoton pulses ($P(n\!\ge\!2)\approx\mu^2/2$) let Eve split off a photon and learn bits losslessly (the photon-number-splitting attack), collapsing naive rates to $O(\eta^2)$. Decoy-state QKD interleaves pulses of different intensities $\mu, \nu, 0$; since Eve cannot tell a signal photon from a decoy photon, comparing their measured yields bounds the single-photon yield $Y_1$ and error $e_1$, exposing PNS and restoring $R = O(\eta)$. It is the deployed workhorse protocol.

---

## 4. The PLOB Bound and the Rate–Distance Wall

Loss is exponential — $\eta = 10^{-\alpha L/10}$ at $\alpha = 0.2$ dB/km gives $10^{-2}$, $10^{-4}$, $10^{-20}$ at 100, 200, 1000 km. The PLOB bound caps the secret key of *any* repeaterless protocol: $K \le -\log_2(1-\eta) \approx 1.44\,\eta$ bits per channel use. At 500 km ($\eta = 10^{-10}$) even a perfect 10 GHz system yields $\lesssim 1.4$ bit/s. This protocol-independent ceiling is the reason satellites and repeaters exist — and precisely what twin-field QKD's altered topology evades.

---

## 5. MDI-QKD: Structural Detector-Attack Immunity

Detectors are QKD's soft underbelly — blinding attacks fake clicks on real hardware. MDI-QKD removes *all* detector side channels: Alice and Bob are both transmitters, sending weak decoy-state pulses to an untrusted relay that performs a Bell-state measurement (Hong–Ou–Mandel interference, Chapter 20) and publicly announces the outcome. The announcement correlates their bits without revealing them; the relay learns nothing even if Eve owns it. MDI-QKD reached 404 km in fibre and underlies untrusted-node (Stage-2) networks; twin-field QKD inherits the same untrusted-relay property.

---

## 6. Twin-Field QKD and $\sqrt{\eta}$ Scaling

Twin-field QKD beats PLOB by changing topology: Alice and Bob each send dim phase-locked fields to an untrusted midpoint where they interfere, and a single detector click heralds a key bit. Because only *one* photon need survive *half* the path, the rate scales as $\sqrt{\eta}$ rather than $\eta$ — overtaking the PLOB bound beyond a few hundred kilometres. The price is interferometric phase stability between lasers hundreds of km apart; records reached 1,002 km of fibre, the first key exchange across 1,000 km.

---

## 7. CV-QKD: Coherent States and Homodyne Detection

CV-QKD (the GG02 protocol) encodes key data in the Gaussian-modulated quadratures of coherent states, read by homodyne or heterodyne detection (Chapter 21) — no single-photon detectors, room-temperature telecom receivers, silicon-photonics-compatible, and able to coexist with classical WDM traffic on lit fibre. Security follows from the uncertainty principle: Eve cannot tap both quadratures without adding detectable excess noise, and Gaussian attacks are provably optimal. Costs: computationally heavy reconciliation of low-SNR data and shorter range (tens of km in the field, $\sim 200$ km in the laboratory).

---

## 8. Entanglement Swapping, Repeaters, and $F' \approx F^2$

A Bell measurement at a middle node entangles two outer qubits that never interacted — teleportation of entanglement itself (Section 22.2.3). Chaining swaps over short, survivable segments distributes entanglement with *polynomial* rather than exponential overhead: the repeater principle (Briegel–Dür–Cirac–Zoller). Fidelity multiplies down the chain, $F' \approx F^2$ per swap (two 95% pairs $\to \sim\!90\%$), so entanglement *purification* (BBPSSW/DEJMPS) — trading copies and two-way classical communication for higher fidelity — is mandatory. Linear-optics Bell measurements cap each swap's success at 50%.

---

## 9. The Quantum Memory Scorecard

A repeater memory must combine efficiency ($>90\%$), storage exceeding the classical heralding time (ms–s), matched bandwidth, multimode capacity, telecom compatibility, and $>99\%$ fidelity — no platform yet has all six. DLCZ atomic ensembles unify source, herald, and memory via collective spin waves; EIT/GEM reach 87–92% efficiency. Rare-earth AFC crystals offer $10^2$–$10^3$-mode storage and record spin coherence (6 h in Eu:YSO). Single emitters (trapped ions, NV/SiV centres) add on-node qubit logic at the cost of multimode capacity. The memory is the pacing technology of the entire quantum internet.

---

## 10. Satellite QKD: Diffraction Beats Absorption

Fibre loses photons exponentially (240 dB over 1,200 km); free space above the atmosphere suffers only *diffraction*, which is polynomial. A $\theta \approx \lambda/D_t$ divergence from a 30 cm telescope spreads to $\sim 3.4$ m over 1,200 km — tens of dB of loss versus fibre's 240 dB, a $\sim 10^{20}$ advantage. Micius exploited this for kHz-rate QKD over 645–1,200 km, a 1,203 km entanglement Bell test ($S = 2.37$), ground-to-satellite teleportation, and a 7,600 km trusted-relay link — at the price of microradian pointing between platforms moving at 7.6 km/s.

---

## 11. The Staged Quantum-Internet Roadmap

Wehner–Elkouss–Hanson rank networks by *end-node capability*: (1) trusted-repeater (deployed — Beijing–Shanghai's 32 nodes); (2) prepare-and-measure (untrusted MDI relays); (3) entanglement distribution (device-independent QKD; Micius's 1,120 km BBM92); (4) quantum memory (teleportation between nodes, blind computing — the Delft NV network); (5) few-qubit fault-tolerant; (6) full quantum-computing networks. Each rung enables strictly more and demands strictly more; the chief payoff of climbing is a shrinking trust boundary — from trusting every relay to trusting none.

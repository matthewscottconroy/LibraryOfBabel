# 22.1.2 — Photonic QKD Implementations

## The Weak-Coherent-Pulse Compromise and the PNS Attack

Ideal BB84 wants one photon per slot. Real transmitters are attenuated lasers emitting coherent states, whose photon number is Poissonian (Chapter 18):

$$P(n) = e^{-\mu}\frac{\mu^n}{n!}$$

with mean $\mu$ typically $0.1$–$0.7$. A pulse then contains two or more photons with probability $P(n\geq 2) \approx \mu^2/2$ — and every multiphoton pulse is a security hole. In the **photon-number-splitting (PNS) attack**, Eve performs a quantum non-demolition measurement of photon number (allowed in principle), blocks single-photon pulses, and for multiphoton pulses keeps one photon in quantum memory while forwarding the rest through a *lossless* channel of her own. She waits for the basis announcement, measures her stored photons correctly, and learns those bits *without inducing any errors*. On a lossy channel she can hide entirely: she suppresses singles and compensates with her lossless forwarding so Bob's detection rate looks normal. Against PNS, the secure key rate of naive weak-pulse BB84 collapses to the rate of pulses that were *provably* single-photon — scaling as $O(\eta^2)$ with channel transmission $\eta$ once $\mu$ is optimized ($\mu \sim \eta$), a crippling penalty at long distance.

## Decoy States: Statistics as a Weapon

The fix costs one modulator. In **decoy-state QKD** (Hwang, 2003 [1]; made rigorous by Lo, Ma & Chen and by Wang, 2005 [2, 3]), Alice randomly interleaves pulses of different mean photon numbers — e.g., signal $\mu = 0.5$, decoy $\nu = 0.1$, and vacuum — announcing afterward which was which. Eve cannot tell a signal pulse from a decoy pulse (a photon is a photon), so any photon-number-dependent channel she implements acts *identically* on both populations. But Alice and Bob can then solve for the single-photon yield $Y_1$ and single-photon error rate $e_1$ from the observed detection statistics of the different intensities — the two-intensity linear system over-determines Eve's strategy. PNS becomes visible as an inconsistency between signal and decoy detection rates. With decoys, the secure rate returns to $R = O(\eta)$ — the same linear scaling as ideal single-photon BB84 — using the GLLP-decoy key formula

$$R \;\geq\; q\left\{ Q_1\left[1 - h(e_1)\right] - Q_\mu f\, h(E_\mu) \right\}$$

where $Q_\mu, E_\mu$ are the overall gain and QBER, $Q_1$ the bounded single-photon gain, $f \approx 1.1$ the error-correction inefficiency, and $q$ the sifting factor. Decoy-state BB84 is *the* deployed workhorse protocol; true single-photon sources (quantum dots, Chapter 19) remove the multiphoton issue at the source and have demonstrated QKD, but attenuated lasers with decoys remain cheaper and faster in practice.

## Rate versus Distance: The Exponential Wall

Everything now reduces to the photon budget. Standard fiber attenuates $\alpha = 0.2$ dB/km at 1550 nm — transmission halves every 15 km:

$$\eta_{\text{ch}} = 10^{-\alpha L / 10}: \qquad 100\ \text{km} \to 10^{-2}, \quad 200\ \text{km} \to 10^{-4}, \quad 400\ \text{km} \to 10^{-8}, \quad 1000\ \text{km} \to 10^{-20}$$

The key survives while single-photon detections outpace dark counts; with SNSPDs (system efficiency $\sim 70\text{–}90\%$, dark counts $< 1$ Hz, Chapter 19) and ultra-low-loss fiber (0.16 dB/km), decoy-state BB84 has reached **421 km** (Geneva, 2018 [4]), delivering a few bits per second at the extreme. At metropolitan distances rates are enormous by comparison: Toshiba demonstrated $\sim 13.7$ Mb/s of secret key over a 10 dB channel ($\approx 50$ km) in 2018, and multi-band systems have since pushed metropolitan rates into the tens of Mb/s — enough for one-time-pad encryption of video.

There is a fundamental ceiling, not just a technological one. The **PLOB bound** (Pirandola-Laurenza-Ottaviani-Banchi, 2017 [5]) caps the secret bits per channel use of *any* repeaterless protocol over a loss channel:

$$K \leq -\log_2(1 - \eta) \;\approx\; 1.44\,\eta \quad (\eta \ll 1)$$

At 500 km ($\eta = 10^{-10}$), even a perfect 10 GHz system cannot beat $\sim 1.4$ bit/s. Direct transmission is doomed at continental scale — hence repeaters (Section 22.2) and satellites (22.1.3).

## Twin-Field QKD: Beating PLOB with One Interference

One protocol family slips past the bound by changing the topology. In **twin-field QKD** (Lucamarini et al., 2018 [6]), Alice and Bob each send dim phase-encoded pulses to an *untrusted middle station*, where the two fields interfere on a beam splitter and a single detector clicks. A key bit requires only *one* photon to survive *half* the path, so the rate scales as $\sqrt{\eta}$ rather than $\eta$ — the square root of the total transmission — overtaking PLOB beyond a few hundred kilometers. (The middle node measures only interference, learning nothing about the key: the arrangement is measurement-device-independent by construction.) The price is interferometric phase stability between lasers hundreds of kilometers apart — solved with locked lasers and phase tracking. Records followed rapidly: 605 km (Toshiba, 2021), 833 km (2022), and **1,002 km** of fiber (USTC, 2023 [7]) — the first key exchange across 1,000 km of fiber, albeit at $\sim 10^{-3}$ bit/s at the extreme, and $\sim$ kb/s at 500 km-class distances where PLOB-limited systems produce essentially nothing.

## Closing the Detector Door: MDI-QKD

Security proofs model devices; hackers attack the hardware itself. The most damaging demonstrated attacks target detectors — e.g., *detector blinding*: bright light forces an APD into linear mode, where tailored classical pulses fake "clicks" of Eve's choosing, letting her run intercept-resend invisibly (demonstrated against commercial systems in 2010 [8]). **Measurement-device-independent QKD** (Lo, Curty & Qi, 2012 [9]) removes *all* detector side channels structurally: Alice and Bob both act as transmitters, sending BB84-encoded weak pulses to an untrusted relay that performs a Bell-state measurement (Hong-Ou-Mandel interference of the two pulses, Chapter 20) and publicly announces the outcome. The announcement correlates Alice's and Bob's bits without revealing them; the relay — even if Eve owns it — learns nothing. MDI-QKD with decoy states has reached 404 km in fiber and is deployed in Chinese metropolitan networks. Its conceptual descendant, twin-field QKD, inherits the untrusted-relay property.

## CV-QKD: Keys from Homodyne Detection

Chapter 21's toolbox yields a wholly different implementation. In **CV-QKD** (the GG02 protocol, Grosshans & Grangier, 2002 [10]), Alice sends coherent states whose $x$ and $p$ displacements are drawn from a Gaussian distribution; Bob homodynes (or heterodynes) a randomly chosen quadrature. The shared Gaussian-correlated variables are reconciled into bits; security rests on the uncertainty principle — Eve cannot tap both quadratures without adding detectable excess noise, and Gaussian attacks are provably optimal among collective attacks. The attractions are industrial: no single-photon detectors at all — just the coherent receivers of classical coherent telecom (Chapter 9), operating at room temperature, compatible with wavelength-multiplexed traffic on lit fiber, and integrable in silicon photonics. The drawbacks: reconciliation of low-SNR Gaussian data is computationally heavy, excess-noise requirements are stringent, and range is shorter — field systems run tens of kilometers, with laboratory records around 200 km. CV-QKD's composable finite-size security proofs matured later than DV's but are now established for Gaussian-modulated protocols.

## From Components to Products

QKD is a real industry, if a niche one: ID Quantique (Geneva, founded 2001; its systems secured Swiss ballot transfers as early as 2007), Toshiba (Cambridge), QuantumCTek (Hefei), and a growing field of startups sell rack-mounted QKD links; the ETSI ISG-QKD and ITU-T produce interface and security-certification standards. Deployment realities temper the physics: beyond point-to-point reach, today's networks chain **trusted nodes** — intermediate stations where the key is decrypted and re-encrypted, each a physical-security liability (the 2,000 km Beijing–Shanghai backbone uses 32 of them). Integration with classical infrastructure means QKD keys typically feed AES encryptors rather than one-time pads, and national security agencies (e.g., NSA, NCSC) currently recommend *post-quantum cryptography* (PQC) — quantum-resistant classical algorithms — as the primary quantum-threat mitigation, with QKD in hybrid QKD+PQC roles where its physical-layer guarantees justify the hardware. The frank engineering summary: QKD's security argument is unique and its photonics superb, but distance limits, trusted nodes, cost per bit, and authentication bootstrapping define its commercial boundary — and motivate the repeater technology of Section 22.2.

## Summary

- Weak coherent pulses have Poissonian photon number; multiphoton pulses enable the PNS attack, reducing naive rates to $O(\eta^2)$.
- Decoy states (extra intensities, post-announced) bound single-photon yields and restore $R = O(\eta)$; decoy BB84 is the deployed standard, reaching 421 km and Mb/s-class metropolitan rates.
- The PLOB bound $K \leq -\log_2(1-\eta)$ caps all repeaterless QKD; twin-field QKD's untrusted-relay interference achieves $\sqrt{\eta}$ scaling and has crossed 1,000 km of fiber.
- MDI-QKD eliminates detector side channels (the demonstrated blinding attacks) via an untrusted Bell-measurement relay.
- CV-QKD encodes in Gaussian-modulated quadratures read by telecom homodyne receivers — no single-photon detectors — at the cost of range and reconciliation complexity.
- Commercial systems exist under ETSI/ITU standards; trusted nodes and PQC coexistence define current practice.

---

*References*

[1] Hwang, W.-Y. (2003). Quantum key distribution with high loss: Toward global secure communication. *Physical Review Letters*, 91(5), 057901. [DOI: 10.1103/PhysRevLett.91.057901]

[2] Lo, H.-K., Ma, X., & Chen, K. (2005). Decoy state quantum key distribution. *Physical Review Letters*, 94(23), 230504. [DOI: 10.1103/PhysRevLett.94.230504]

[3] Wang, X.-B. (2005). Beating the photon-number-splitting attack in practical quantum cryptography. *Physical Review Letters*, 94(23), 230503. [DOI: 10.1103/PhysRevLett.94.230503]

[4] Boaron, A., et al. (2018). Secure quantum key distribution over 421 km of optical fiber. *Physical Review Letters*, 121(19), 190502. [DOI: 10.1103/PhysRevLett.121.190502]

[5] Pirandola, S., Laurenza, R., Ottaviani, C., & Banchi, L. (2017). Fundamental limits of repeaterless quantum communications. *Nature Communications*, 8, 15043. [DOI: 10.1038/ncomms15043] [The PLOB bound.]

[6] Lucamarini, M., Yuan, Z.L., Dynes, J.F., & Shields, A.J. (2018). Overcoming the rate–distance limit of quantum key distribution without quantum repeaters. *Nature*, 557, 400–403. [DOI: 10.1038/s41586-018-0066-6] [Twin-field QKD.]

[7] Liu, Y., et al. (2023). Experimental twin-field quantum key distribution over 1000 km fiber distance. *Physical Review Letters*, 130(21), 210801. [DOI: 10.1103/PhysRevLett.130.210801]

[8] Lydersen, L., Wiechers, C., Wittmann, C., Elser, D., Skaar, J., & Makarov, V. (2010). Hacking commercial quantum cryptography systems by tailored bright illumination. *Nature Photonics*, 4(10), 686–689. [DOI: 10.1038/nphoton.2010.214]

[9] Lo, H.-K., Curty, M., & Qi, B. (2012). Measurement-device-independent quantum key distribution. *Physical Review Letters*, 108(13), 130503. [DOI: 10.1103/PhysRevLett.108.130503]

[10] Grosshans, F. & Grangier, P. (2002). Continuous variable quantum cryptography using coherent states. *Physical Review Letters*, 88(5), 057902. [DOI: 10.1103/PhysRevLett.88.057902] [GG02.]

[11] Xu, F., Ma, X., Zhang, Q., Lo, H.-K., & Pan, J.-W. (2020). Secure quantum key distribution with realistic devices. *Reviews of Modern Physics*, 92(2), 025002. [DOI: 10.1103/RevModPhys.92.025002] [Authoritative review of practical security.]

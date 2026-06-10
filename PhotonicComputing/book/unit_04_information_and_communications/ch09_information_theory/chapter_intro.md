# Chapter 9: Information Theory and Optical Channels

## The Question Shannon Asked

In 1948, Claude Shannon at Bell Laboratories published "A Mathematical Theory of Communication" in two parts in the Bell System Technical Journal [1]. It is, by any measure, one of the most consequential scientific papers of the 20th century. In 27 pages of precise mathematical reasoning, Shannon answered a question that had not been clearly asked: *how much information can a noisy channel carry?*

The answer was both surprising and useful. Shannon showed that information has a precise mathematical definition — entropy — and that every channel with noise has a fundamental capacity $C$ (in bits per second) such that:
- For any rate $R < C$, reliable communication is possible — errors can be made arbitrarily rare by using a sufficiently good code.
- For any rate $R > C$, reliable communication is impossible — errors accumulate faster than they can be corrected.

The existence of this capacity limit — the **Shannon limit** — is remarkable because it says that noise is not the enemy of communication. Noise can always be overcome by proper coding, as long as the rate is below capacity. The enemy is exceeding the capacity.

For optical communications, Shannon's work has a direct and quantitative application. The optical channel — a photon stream in a fiber, modulated by a laser and detected by a photodetector — is a specific type of channel whose capacity can be calculated from its noise model. The result, as we will derive in Section 9.1, is the **optical channel capacity**:

$$C = B\log_2\left(1 + \frac{S}{N}\right) \text{ bits/s}$$

where $B$ is the bandwidth, $S$ is the signal power, and $N$ is the noise power. This is the Shannon-Hartley theorem, applied to the optical channel.

Current long-haul optical fiber systems operate at spectral efficiencies within a factor of 2–3 of the Shannon limit for a linear (amplifier-noise-limited) channel. At shorter distances, systems approach within 10–20% of capacity. The Shannon limit has, in the span of 70 years, been transformed from an abstract mathematical result to a practical engineering benchmark.

## Chapter Structure

This chapter develops the theory and practice of information in optical systems in four sections:

**Section 9.1 — Shannon Theory and the Optical Channel**: Entropy, mutual information, the Shannon-Hartley theorem, and the capacity of optical channels. The photon-number-resolving limit (the quantum limit of optical communication) and how it relates to the classical Shannon limit.

**Section 9.2 — Modulation Formats**: OOK (on-off keying) and PAM4 (pulse amplitude modulation, 4 levels) for direct detection; BPSK, QPSK, and QAM-16/64 for coherent systems. The spectral efficiency and SNR requirement of each format, and why the industry has shifted from OOK to 400G DP-16QAM.

**Section 9.3 — Wavelength-Division Multiplexing (WDM)**: The ITU frequency grid, DWDM channel plans, WDM system design including EDFA cascades, dispersion compensation, and nonlinear limits. ROADMs (reconfigurable optical add-drop multiplexers) as the switching nodes of the optical network.

**Section 9.4 — Forward Error Correction**: The FEC principle, hard-decision vs. soft-decision decoding, LDPC codes and polar codes, and FEC overhead. The role of FEC in photonic computing: optical decoding as a potential photonic computing application.

---

## References

[1] Shannon, C.E. (1948). "A mathematical theory of communication." *Bell System Technical Journal*, 27(3), 379–423. [The paper that defined the field.]

[2] Essiambre, R.-J., Kramer, G., Winzer, P.J., Foschini, G.J., & Goebel, B. (2010). "Capacity limits of optical fiber networks." *Journal of Lightwave Technology*, 28(4), 662–701. [The comprehensive treatment of Shannon limits for optical fiber, including nonlinear effects.]

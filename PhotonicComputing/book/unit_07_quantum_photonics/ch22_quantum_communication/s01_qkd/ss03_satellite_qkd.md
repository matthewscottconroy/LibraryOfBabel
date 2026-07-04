# 22.1.3 — Satellite QKD: Micius and the Global Scale

## Changing the Loss Law

Fiber loses photons *exponentially*: $0.2$ dB/km compounds to 240 dB across 1,200 km — a transmission of $10^{-24}$ that no source brightness or detector heroics can overcome. Free space plays by different rules. Above the atmosphere there is no absorption or scattering at all; the only unavoidable loss is **diffraction**, and diffraction is polynomial, not exponential.

**Worked link budget.** A transmitter telescope of aperture $D_t$ emits a beam with diffraction-limited divergence $\theta \approx \lambda / D_t$. For Micius's $\sim 30$ cm transmitter at $\lambda \approx 850$ nm, $\theta \approx 2.8\ \mu$rad. Over $L = 1{,}200$ km the spot has grown to $\theta L \approx 3.4$ m. A ground telescope of diameter $D_r = 1.2$ m intercepts roughly $(D_r/3.4\,\text{m})^2 \approx 0.12$ of the beam — about 9 dB of geometric loss; adding pointing jitter, atmospheric turbulence and transmission (the beam crosses the equivalent of only $\sim 10$ km of horizontal air), and receiver optics, real downlinks at maximum range cost a few tens of dB. Compare: **tens of dB versus 240 dB**. The satellite advantage at this distance is roughly twenty orders of magnitude — which is precisely the enhancement the Micius team reported relative to direct fiber transmission [1]. The engineering price is acquisition, pointing, and tracking: holding two moving telescopes locked to microradian precision while one of them crosses the sky at 7.6 km/s, using beacon lasers and fast steering mirrors, with quantum operations restricted (initially) to cloudless nights to keep background photons below the signal.

## Micius: The Experiment Series

*Micius* (named for the ancient philosopher Mozi; also "QUESS" — Quantum Experiments at Space Scale) was launched by the Chinese Academy of Sciences in August 2016 into a $\sim 500$ km sun-synchronous orbit, carrying a decoy-state BB84 transmitter, a Sagnac-based entangled-photon-pair source, and precision pointing payloads. Led by Jian-Wei Pan's USTC group, it executed, within two years, essentially the entire textbook of long-distance quantum communication:

1. **Satellite-to-ground decoy-state QKD** (2017). Keys delivered to the Xinglong station over pass distances of 645–1,200 km at kilohertz secret-key rates — $\sim 20$ orders of magnitude beyond what the same distance of fiber could deliver [1].
2. **Entanglement distribution over 1,203 km** (2017). Pairs from the onboard down-conversion source were split between the Delingha and Lijiang ground stations, 1,203 km apart; the measured CHSH violation ($S = 2.37 \pm 0.09$) confirmed that entanglement survives dual downlinks totaling 64–82 dB of loss [2] — the longest-distance Bell test yet performed.
3. **Ground-to-satellite quantum teleportation** (2017). Single-photon states from the Ngari station in Tibet were teleported to the satellite over up-links of up to 1,400 km, using entanglement shared through the moving channel [3].
4. **Intercontinental quantum-secured communication** (2018). Using Micius as a *trusted relay* — the satellite holds keys exchanged separately with ground stations in Xinglong and Graz — a quantum-secured image exchange and a 75-minute videoconference linked Beijing and Vienna, 7,600 km apart [4].
5. **Entanglement-based QKD without trusting the satellite** (2020). Running BBM92 (Section 22.1.1) between two ground stations 1,120 km apart, with the satellite merely distributing pairs: since security rests on the measured entanglement, the source need not be trusted at all. The rate was meager ($\sim 0.1$ bit/s) but the security model is the strongest deployed at that scale [5].
6. **An integrated space-to-ground network** (2021). Micius was joined to the 2,000 km Beijing–Shanghai fiber backbone (32 trusted nodes) and metropolitan networks, forming a 4,600 km-span infrastructure serving over 150 institutional users [6].

The sequence is worth memorizing as a case study in *system* physics: every element — SPDC pair sources (Chapter 18), decoy-state modulation (22.1.2), single-photon detectors (Chapter 19), adaptive optics, and Bell-measurement protocols (Chapter 20) — had existed in laboratories; Micius's contribution was integrating them onto a 635 kg platform that survives launch and thermal cycling in orbit.

## Limits and the Constellation Future

A single LEO satellite is a *store-and-forward* key courier: it sees each ground station for a few hundred seconds per night pass, accumulates keys, and relays them — with the satellite itself trusted in most operating modes (the 2020 entanglement-based mode removes that trust at heavy rate cost). Cloud cover, daylight background, and pass geometry limit availability; keys arrive in bursts of $10^5$–$10^6$ bits per pass. Scaling the vision means: constellations of dozens of satellites for continuous coverage; daylight operation (demonstrated in principle using 1550 nm downlinks and tight spectral-spatial filtering); smaller, cheaper platforms — China's Jinan-1 microsatellite (2022) demonstrated real-time satellite QKD from a 100 kg-class platform; and, eventually, *entanglement* distribution with onboard or ground quantum memories, making satellites segments of a true quantum repeater chain rather than trusted couriers. National and commercial programs (ESA's Eagle-1, Canada's QEYSSat, and several startups) are converging on the same architecture. For global reach, satellites and fiber are complements, not competitors: satellites cross oceans; fiber delivers density within metropolitan areas; repeaters (next section) are the missing piece that would fuse both into an untrusted end-to-end quantum network.

## Summary

- Free space replaces fiber's exponential attenuation with polynomial diffraction loss: $\sim$ tens of dB from LEO versus 240 dB for 1,200 km of fiber — a $\sim 10^{20}$ advantage.
- Micius (2016) demonstrated the full canon: decoy-state QKD at kHz rates over 645–1,200 km; entanglement distribution and a CHSH violation across 1,203 km; ground-to-satellite teleportation; a trusted-relay Beijing–Vienna link over 7,600 km; trust-free entanglement-based QKD over 1,120 km; and integration with the 2,000 km fiber backbone into a 4,600 km network.
- Pointing (microradians between platforms moving at 7.6 km/s), background light, and weather are the binding constraints; current satellites are mostly trusted couriers.
- The roadmap: constellations, daylight operation, cheaper microsatellites, and memory-equipped satellites acting as repeater nodes in a global quantum network.

---

*References*

[1] Liao, S.-K., et al. (2017). Satellite-to-ground quantum key distribution. *Nature*, 549, 43–47. [DOI: 10.1038/nature23655]

[2] Yin, J., et al. (2017). Satellite-based entanglement distribution over 1200 kilometers. *Science*, 356(6343), 1140–1144. [DOI: 10.1126/science.aan3211]

[3] Ren, J.-G., et al. (2017). Ground-to-satellite quantum teleportation. *Nature*, 549, 70–73. [DOI: 10.1038/nature23675]

[4] Liao, S.-K., et al. (2018). Satellite-relayed intercontinental quantum network. *Physical Review Letters*, 120(3), 030501. [DOI: 10.1103/PhysRevLett.120.030501] [Beijing–Vienna.]

[5] Yin, J., et al. (2020). Entanglement-based secure quantum cryptography over 1,120 kilometres. *Nature*, 582, 501–505. [DOI: 10.1038/s41586-020-2401-y]

[6] Chen, Y.-A., et al. (2021). An integrated space-to-ground quantum communication network over 4,600 kilometres. *Nature*, 589, 214–219. [DOI: 10.1038/s41586-020-03093-8]

[7] Lu, C.-Y., Cao, Y., Peng, C.-Z., & Pan, J.-W. (2022). Micius quantum experiments in space. *Reviews of Modern Physics*, 94(3), 035001. [DOI: 10.1103/RevModPhys.94.035001] [Comprehensive review of the program.]

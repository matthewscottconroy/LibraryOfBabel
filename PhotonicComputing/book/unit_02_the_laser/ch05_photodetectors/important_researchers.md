# Important Researchers: Chapter 5 — Photodetectors

---

## Albert Einstein (1879–1955)

*(Profiled in Chapter 3 for stimulated emission; relevant here for the photoelectric effect.)*

Einstein's 1905 paper explaining the photoelectric effect [1] established the photon concept and the relation $E = h\nu$. He showed that the kinetic energy of ejected electrons is $K_{max} = h\nu - \phi$ (where $\phi$ is the work function), independent of light intensity — a result inexplicable classically, where more intense light should eject faster electrons. This paper is the founding document of quantum optics and of all photon detection physics. **It is, paradoxically, the paper for which Einstein received his only Nobel Prize, not relativity.**

[1] Einstein, A. (1905). *Annalen der Physik*, 322(6), 132–148.

---

## John B. Johnson (1887–1970) and Harry Nyquist (1889–1976)

**What they did**: In 1928, Johnson (at Bell Labs) measured and Nyquist (also at Bell Labs) theoretically explained thermal noise in resistors [2,3]. Johnson measured that the mean-square voltage noise in a resistor is proportional to temperature and resistance; Nyquist derived this from thermodynamic equilibrium via the fluctuation-dissipation theorem, giving the formula $S_V(f) = 4k_BTR$.

**Why it matters for photonic computing**: Johnson noise in the TIA feedback resistor is typically the dominant noise source in photonic computing output receivers at signal powers below ~0.1 mW. Understanding Nyquist's derivation — that thermal noise is a consequence of equilibrium, not a circuit flaw — clarifies why the only way to reduce it is to reduce temperature, reduce bandwidth, or increase the feedback resistance (at the cost of bandwidth).

[2] Johnson, J.B. (1928). *Physical Review*, 32, 97–109.
[3] Nyquist, H. (1928). *Physical Review*, 32, 110–113.

---

## Walter Schottky (1886–1976)

**What he did**: Schottky predicted and explained shot noise in electron tubes in 1918 [4]: the discrete nature of electron emission from a cathode produces current fluctuations with spectrum $S_i = 2eI$. This is the **shot noise formula**, which applies equally to photodetection. Schottky also invented the Schottky barrier (metal-semiconductor junction) and contributed to the theory of transistors.

**Why it matters for photonic computing**: Every photonic computing system operating on optical signals is ultimately limited by shot noise in the detectors. Schottky's result is the irreducible quantum noise floor of optical measurement. An analyst evaluating the precision of a photonic matrix processor must first compute whether the system is thermally or shot-noise limited.

[4] Schottky, W. (1918). "Über spontane Stromschwankungen in verschiedenen Elektrizitätsleitern." *Annalen der Physik*, 362(23), 541–567.

---

## Yevgeny Tager (1922–2004) and Roman Alabedra (1930–2010)

**What they did**: Tager (Soviet physicist) and Alabedra (French physicist, LIRMM) independently developed the theory of the optimum ionization ratio for avalanche photodiodes in the 1960s–70s. Their work established that low ionization ratio $k = \alpha_h/\alpha_e$ minimizes excess noise in APDs, explaining why silicon APDs (with $k \approx 0.01$, electron-dominated ionization) have superior noise performance to InGaAs APDs ($k \approx 0.4$).

**Why it matters**: The APD figure of merit $k$ governs whether an APD provides a net SNR benefit over a p-i-n detector in a given receiver. This analysis is critical for choosing the detector type in optical receiver design for photonic computing output stages.

---

## Gerd Keiser (b. 1941)

Not a researcher per se, but worth acknowledging: Keiser's textbook *Optical Fiber Communications* (now in its 5th edition) and his review articles on optical receiver design have been formative for generations of photonics engineers. The receiver SNR analysis, TIA noise models, and sensitivity calculations that appear in this chapter follow the conventions established in Keiser's pedagogical framework.

---

## Roman Sobolewski and collaborators (SNSPD Development)

**What they did**: Goltsman et al. (2001, Moscow State Pedagogical University) and Korneev et al. demonstrated the first ultrafast superconducting nanowire single-photon detector (SNSPD) operating at 1550 nm [5]. The original device used NbN nanowires at 4.2 K, achieving single-photon sensitivity with picosecond timing. Subsequent work by groups at NIST (Verma, Lita, Mirin, Nam), Delft (Zwiller), and MIT Lincoln Laboratory (Berggren, Rosenberg) pushed efficiency above 90% and timing jitter below 10 ps.

**Why it matters for photonic computing**: The SNSPD's near-unity detection efficiency (> 90%) and sub-picosecond timing jitter are prerequisites for photonic quantum computing protocols (boson sampling, quantum teleportation, quantum error correction) that require coincidence detection of many photons. Without SNSPD performance levels, most demonstrations of quantum photonic advantage would be experimentally impossible.

[5] Goltsman, G.N., et al. (2001). "Picosecond superconducting single-photon optical detector." *Applied Physics Letters*, 79(6), 705–707.

---

## Jurgen Michel and Lionel Kimerling (MIT)

**What they did**: Michel, Kimerling, and their group at MIT pioneered the integration of germanium on silicon for telecom photodetection. Their work in the 2000s established the epitaxial growth conditions, doping strategies, and waveguide integration techniques that enable high-performance Ge-on-Si photodetectors in silicon photonic platforms [6]. The key insight was using cyclic annealing and selective area growth to reduce threading dislocation densities to practical levels.

**Why it matters for photonic computing**: Ge-on-Si photodetectors are the standard output element of silicon photonic computing chips. Without practical Ge integration, every silicon photonic chip would require off-chip detectors (expensive, bulky, lossy coupling), making chip-scale photonic computing systems impractical.

[6] Yin, T., et al. (2007). "31 GHz Ge n-i-p waveguide photodetectors on silicon-on-insulator substrate." *Optics Express*, 15(21), 13965–13971.

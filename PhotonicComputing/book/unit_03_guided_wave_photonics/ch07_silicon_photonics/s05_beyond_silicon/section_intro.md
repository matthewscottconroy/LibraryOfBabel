# Section 7.5: Beyond Silicon — Alternative Photonic Platforms

Silicon photonics is extraordinary. A 220-nm-thick silicon waveguide can guide light around a 5-μm bend. A silicon ring resonator the size of a red blood cell can select one frequency from a terahertz-wide spectrum with a linewidth of 25 picometers. Silicon photonic chips can be manufactured by the billions, using the same fabs that make the processors in smartphones.

But silicon is not sufficient. We have catalogued its limitations throughout this chapter: no direct bandgap (no on-chip laser), two-photon absorption at modest power levels (~10 mW), centrosymmetric structure (no Pockels effect), thermal sensitivity ($dn/dT$ so large that temperature fluctuations detune resonators), and a transparency window that closes at ~2.5 μm on the long-wavelength end and at ~1.1 μm on the short-wavelength end.

When any of these limitations matter — and for photonic computing, several of them matter simultaneously — the system designer must look beyond silicon. The three platforms developed in this section each address different subsets of silicon's limitations:

**Subsection 7.5.1 — Silicon Nitride (Si₃N₄)**: The low-power, wide-bandwidth complement to silicon. Si₃N₄ has no two-photon absorption at 1550 nm, an extremely low thermo-optic coefficient (10× smaller than Si), near-zero anomalous dispersion achievable with waveguide geometry, and transparency extending from ultraviolet to mid-infrared. Its nonlinear coefficient is lower than silicon's (no TPA, but also less $n_2$), making it ideal for precision photonic circuits where power must be kept low or where the thermo-optic drift of silicon is unacceptable. Microresonator frequency combs on Si₃N₄ are the leading platform for massively parallel WDM photonic computing.

**Subsection 7.5.2 — Lithium Niobate on Insulator (LNOI)**: We encountered LNOI in the modulator section (7.3.4) as a modulator platform. Here we treat it as a complete photonic platform: its waveguide properties, passive components, and the full range of $\chi^{(2)}$ phenomena it enables — electro-optic modulation, second-harmonic generation, parametric amplification, and photon pair generation. LNOI is the platform where the Pockels effect meets chip-scale integration.

**Subsection 7.5.3 — Indium Phosphide (InP)**: The only platform that integrates everything. InP and its alloys (InGaAsP, InAlGaAs) are direct-bandgap semiconductors that can be used for lasers, amplifiers, modulators, and photodetectors all on a single substrate. InP foundries offer monolithic photonic integrated circuits (PICs) with hundreds of components. The key limitation is cost and integration density: InP wafer sizes are smaller (100–150 mm vs. 300 mm for Si), and the cost per chip is much higher. But for applications requiring the highest integration and performance — coherent optical communications transceivers, quantum photonic circuits with integrated single-photon emitters — InP remains essential.

---

## The Platform Decision in Photonic Computing

Choosing a photonic platform for a computing application is not a simple optimization. Each platform has a different set of available building blocks, different co-integration options with electronics, and different manufacturing cost structures. A useful framework is to ask:

1. **What do you need to generate the light?** (Laser: only InP or heterogeneous III-V/Si; Si and Si₃N₄ and LNOI require external sources)
2. **What do you need to modulate the light?** (Fast EO: LNOI or Si plasma; slow thermal: Si or Si₃N₄; non-volatile: PCM on any platform)
3. **What do you need to detect the light?** (1550 nm: Ge-on-Si (Si PIC), InGaAs (InP), or external)
4. **What do you need for the linear optical network?** (Dense passive routing: Si or Si₃N₄; fiber-scale device: LNOI or InP)
5. **What nonlinearities do you need?** ($\chi^{(2)}$: LNOI or AlGaAs; low-loss $\chi^{(3)}$: Si₃N₄; high-speed TPA management: Si₃N₄ over Si)

No single platform answers all five questions optimally. The mature photonic computing architectures are therefore heterogeneous: they combine multiple platforms (Si, Si₃N₄, III-V, LNOI) through wafer bonding, edge coupling, or flip-chip attachment. Understanding what each platform contributes — and what its limitations are — is the knowledge required to evaluate the engineering tradeoffs in any specific photonic computing proposal.

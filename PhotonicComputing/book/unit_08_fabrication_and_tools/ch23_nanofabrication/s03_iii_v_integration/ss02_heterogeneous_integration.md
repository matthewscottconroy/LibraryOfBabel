# 23.3.2 Heterogeneous III-V-on-Silicon Platforms and Monolithic Growth

## From Bonded Film to Working Laser

After bonding and substrate removal (Section 23.3.1), the SOI wafer carries a few-μm III-V epitaxial film — n-cladding, multiple quantum wells, p-cladding — directly above its patterned silicon waveguides, separated by at most tens of nanometers of oxide or BCB. The laser is now built by ordinary wafer-scale processing: mesa etch, implantation or regrowth for current confinement, p and n contacts, heaters.

The optical design problem is characteristic and elegant: the structure supports **hybrid supermodes** shared between the III-V gain region and the silicon waveguide, with a confinement split controlled by the silicon waveguide's width beneath the mesa. A wide silicon waveguide pulls the mode down into silicon (low gain overlap, low loss); a narrow one pushes it up into the quantum wells (high gain). **Adiabatic tapers** on the silicon layer transform between these regimes, so a single device can have its gain section strongly coupled to the wells and its passive ends deliver light into a purely silicon mode with sub-dB transition loss. Mirrors, gratings (DBR/DFB), rings, and wavelength-selective feedback are all patterned in the *silicon*, where lithography is precise — the III-V supplies photons, silicon supplies brains. Beyond lasers, the same platform yields SOAs, high-speed III-V-on-Si modulators (including MOSCAP devices exploiting the III-V/Si interface), and photodetectors; see the platform review by Komljenovic et al. [*Journal of Lightwave Technology*, 2016].

An underappreciated bonus of heterogeneous lasers for coherent and analog applications: the silicon external-cavity approach (gain chip locked to a high-Q silicon or Si₃N₄ ring) delivers fiber-laser-class linewidths (kHz and below) from a chip — directly relevant to the coherent photonic computing architectures of Unit V and the quantum systems of Unit VII, whose phase references live or die by laser linewidth.

Heterogeneous integration is not laboratory exotica: Intel's silicon photonics transceivers, with bonded III-V lasers defined by wafer-scale lithography, have shipped in the millions of units — the strongest existence proof that the approach survives high-volume manufacturing economics.

## Thermal Reality

The heterogeneous laser's chief physical liability is thermal: the active region sits above the **buried oxide, whose thermal conductivity (~1.4 W/m·K) is two orders of magnitude worse than silicon's (~150 W/m·K)**. Junction heat must detour through the mesa and metals, so thermal impedances are several times higher than for the same epi on native InP, degrading efficiency and maximum output power at elevated temperature — and photonic computing chips, co-packaged with hot electronics, run warm. Mitigations: thermal shunts (metal or poly vias through the BOX), substrate thinning, placing amplifiers over BOX openings, and system-level choices (external laser, on-chip everything else) when the power budget demands it.

## Monolithic Growth: The Endgame

The conceptually cleanest solution — grow the III-V laser directly on the silicon wafer by epitaxy — collides with three materials facts:

1. **Lattice mismatch**: 4.1% for GaAs/Si, 8.1% for InP/Si. Mismatched growth relaxes by generating **threading dislocations** (typically 10⁸–10¹⁰ cm⁻² uncontrolled), each a nonradiative recombination center that murders laser efficiency and lifetime.
2. **Thermal expansion mismatch** (~2.5× for GaAs/Si): cracks and wafer bow accumulate on cooldown from growth temperature.
3. **Polar-on-nonpolar growth**: III-V on (001) Si nucleates **antiphase domains** unless the surface is prepared (offcut substrates or patterned nucleation).

The countermeasures, layered together in modern demonstrations:

- **Buffer engineering**: thick graded GaAs/Ge or GaAs buffers with strained-layer superlattices that bend threading dislocations sideways, filtering the density down to ~10⁶–10⁷ cm⁻².
- **Aspect-ratio trapping / selective-area growth**: grow in narrow oxide trenches so dislocations terminate on the trench sidewalls; V-groove silicon variants (GaAs-on-V-grooved-Si) suppress antiphase domains as well.
- **Quantum dot active regions** — the breakthrough enabler. Unlike quantum wells, where carriers diffuse freely to any dislocation within a diffusion length, carriers in InAs quantum dots are *localized*: a dislocation kills only the dots it touches. QD lasers tolerate dislocation densities that would extinguish QW lasers, and additionally offer low threshold, temperature-stable operation, and small linewidth-enhancement factor (hence low feedback sensitivity — attractive for isolator-free integration). Electrically pumped, continuous-wave 1.3 μm InAs/GaAs QD lasers grown on silicon, with long extrapolated lifetimes, were demonstrated by Chen et al. [*Nature Photonics*, 2016], and the UCSB program has driven the approach toward foundry relevance.

Status, honestly stated: monolithic QD-on-Si lasers are a rapidly maturing research technology — impressive lifetimes and wafer-scale growth demonstrated; still ahead: routine coupling into the 220 nm SOI device layer (the grown material sits on thick buffers, μm above the waveguide plane — coupling structures or growth-in-recess schemes must bridge this), CMOS-pilot-line qualification, and yield economics. Bonded heterogeneous integration is *production*; monolithic growth is the credible successor.

## What This Means for Photonic Computing

Return to the system-level accounting that Unit IX will formalize. The laser is the photonic processor's power supply, and its **wall-plug efficiency (WPE)** — electrical watts in per optical watts out, typically 10–25% for integrated III-V lasers, degraded further by on-chip coupling and distribution losses — multiplies *every* optical energy figure in Units V–VI. A matrix engine whose optics dissipate femtojoules per MAC but whose laser runs at 15% WPE, feeding a distribution network with 6 dB of loss, pays $\sim 26×$ the naive optical energy at the wall. Integration choice is therefore not a packaging detail but an energy-efficiency term: on-chip gain (heterogeneous/monolithic) shortens the lossy path between photon generation and computation, distributed amplification can re-level deep meshes, and comb-based architectures amortize one efficient pump across many wavelength channels. When you read a photonic-computing energy claim, your first question should be Chapter 25's: *where is the laser, and who is paying for it?*

# 23.4.2 Electronic-Photonic Packaging, Thermal Management, and Testing

## The Electrical Interface Problem

A photonic computing chip is electrically voracious in two distinct currencies. **DC/low-speed lines** bias and tune: every thermo-optic phase shifter, ring heater, and monitor photodiode needs a wire — a 64×64 Clements mesh carries 2016 MZIs and ~4000+ tuning elements, far beyond any perimeter wire-bond budget. **RF lines** carry data: modulators and photodetectors at tens of GHz, each wanting a controlled 50 Ω environment. The packaging hierarchy exists to serve both.

**Wire bonding** — μm-scale gold or aluminum wires from chip pads to a carrier — is cheap, universal, and perimeter-limited (pads at ~100 μm pitch around the die edge: a few hundred connections at best). Its RF liability is inductance, ~1 nH per mm of wire. Into a 50 Ω system, a 1 mm bond gives a single-pole rolloff near

$$f_{3dB} \approx \frac{R}{2\pi L} = \frac{50}{2\pi \times 10^{-9}} \approx 8\ \text{GHz}$$

— marginal for modern modulators. Short double bonds and ground-signal-ground geometries stretch this, but the message stands: wire bonds are for DC and modest speeds.

**Flip-chip bumping** inverts the die onto a substrate or directly onto an electronic IC, connecting through solder microbumps or **copper pillars** at ≲100 μm pitch (and, in advanced/hybrid bonding, down to μm-scale pitch). Thousands of area-array connections with tens of pH inductance and tens of fF capacitance: this is how a photonic mesh gets its thousands of driver lines, and how a modulator sees its driver with essentially no parasitic penalty. The natural embodiment for photonic computing is the **electronic-photonic 2.5D/3D stack**: photonic die as an *interposer* carrying the electronic die(s) on its face (photonics-as-substrate, with through-silicon vias if needed), or both dies side-by-side on a shared interposer. This is also the architecture of **co-packaged optics** for datacenter switches — optical tiles surrounding a switch ASIC on a common substrate to replace lossy electrical SerDes reach with fiber — whose manufacturing ecosystem photonic computing inherits essentially for free.

The end of the line is **monolithic** integration — photonics and transistors in one process — demonstrated persuasively in zero-change CMOS ("a single-chip microprocessor communicating directly using light," Sun et al., *Nature*, 2015) and in bulk-CMOS polysilicon photonics [Atabaki et al., *Nature*, 2018]. Monolithic wins on interconnect density and parasitics, but marries the photonics to an electronics node's design rules and economics; the industry's center of gravity today remains 2.5D/3D assembly of separately optimized dies.

## Thermal Management

Photonics' thermal problem is not primarily heat *quantity* but heat *sensitivity*: silicon's $dn/dT$ shifts a ring resonance ~0.07–0.08 nm/K (Chapter 7), so the same package that hosts a 50 W electronic die must hold photonic structures to sub-kelvin *stability* (absolute or relative to their tuners' capture range).

The toolkit, in escalating order of cost:

- **Passive design**: place rings far from hot electronics; exploit common-mode drift (tune to a reference resonance, not to absolute wavelength); athermal waveguide designs (polymer or nitride compensation).
- **Local closed-loop tuning**: every ring's heater servoed to a monitor photodiode — converting a temperature problem into a control-and-power problem (the multi-watt budgets of Section 23.2.3).
- **Thermoelectric coolers (TEC)**: hold a laser or the full assembly at fixed temperature. Effective and standard for lasers (wavelength stability), but a TEC's own consumption (often 1–3 W to move a few watts of heat) can dominate a module's power — anathema for computing accelerators chasing pJ/MAC figures. "Uncooled" operation is the design goal wherever architecture permits.
- **Thermal isolation vs. thermal shunting**, chosen per device: undercut trenches isolate heaters (5–10× tuning-power savings, slower response); thermal shunts through the BOX cool heterogeneous lasers (Section 23.3.2). Note the direct conflict — the same BOX that insulates your heater insulates your laser — one more reason floorplanning is physics.

## Testing: From Wafer to Qualified Part

**Wafer-level optical test** is the economic linchpin, made possible by grating couplers (Section 23.4.1): a probe station steps a fiber array plus RF/DC probe card across every reticle site, running per-die measurements in seconds to minutes — swept-wavelength transmission (tunable laser + detector, resolving ring spectra and filter shapes), insertion-loss cutbacks, phase-shifter $P_\pi$/$V_\pi$ curves, photodiode responsivity and dark current. The output is the wafer map (Section 23.2.3) and a known-good-die inventory *before* the expensive packaging steps — essential when packaging dominates cost: packaging a bad die is the most expensive way to find it.

**High-speed characterization** happens on sampled dies or packaged parts: S-parameters (electro-optic $S_{21}$) with a lightwave component analyzer to tens of GHz; **eye diagrams** and bit-error-rate curves for data paths (the eye's openness integrating every impairment: bandwidth, chirp, reflections, noise); and for computing parts specifically, *analog* metrics with no datacom analogue — weight-setting precision and repeatability (effective bits), crosstalk matrices between tuners, matrix-operation fidelity against programmed targets (Chapter 12's benchmarks).

**Reliability qualification** borrows telecom's framework (Telcordia GR-468 for optoelectronics): high-temperature operating life, temperature cycling, damp heat, mechanical shock. Lifetimes are inferred from **accelerated aging** via the Arrhenius model,

$$AF = \exp\left[\frac{E_a}{k_B}\left(\frac{1}{T_{use}} - \frac{1}{T_{stress}}\right)\right]$$

with activation energy $E_a$ fitted per failure mechanism ($E_a \approx 0.4\text{–}0.7$ eV covers many III-V degradation modes: a 0.5 eV mechanism ages ~17× faster at 125 °C than at 55 °C, so a 1000-hour burn-in emulates ~2 years). Lasers dominate the failure budget — hence burn-in screens for infant mortality and derating in system design. Passive silicon is comparatively immortal; the reliability question for photonic *computing* modules concentrates on lasers, epoxied fiber joints, and the electronics.

## The Packaging Bottom Line

For photonic computing, packaging is where three chapter-level threads converge and get priced: fabrication variation (tuning power), integration strategy (where the laser lives and how its heat leaves), and I/O architecture (how many fibers and how many thousand electrical lines). A recurring failure mode of the literature — flagged again in Unit IX — is quoting the energy or cost of the photonic die alone. The package *is* the computer.

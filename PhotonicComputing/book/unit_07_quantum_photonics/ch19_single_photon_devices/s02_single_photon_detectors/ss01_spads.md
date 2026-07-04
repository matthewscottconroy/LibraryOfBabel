# 19.2.1 SPADs and Avalanche Photodiodes: Semiconductor Click Detectors

## Geiger-Mode Operation

An avalanche photodiode (APD) is a reverse-biased p-n junction with an internal multiplication region: a photogenerated carrier accelerated by the field ionizes further carriers, producing gain. Biased *below* breakdown, the APD is a linear amplifier (gain 10–100) — useful for classical receivers, but its excess noise swamps single-photon signals. Biased *above* breakdown — **Geiger mode** — the junction becomes metastable: a single absorbed photon triggers a self-sustaining avalanche of $\sim10^5$–$10^6$ carriers, a macroscopic current pulse. A Geiger-mode APD is called a **single-photon avalanche diode (SPAD)**.

The avalanche must then be stopped and the diode re-armed:

- **Passive quenching:** a series resistor drops the bias as current flows, choking the avalanche. Simple, but recovery is RC-slow (~μs).
- **Active quenching:** a fast circuit senses the avalanche onset, yanks the bias below breakdown, then restores it after a controlled hold-off time. Modern SPAD modules use active quenching with dead times of tens of nanoseconds, supporting count rates of 10–40 MHz.

Because the avalanche is triggered identically by one photon or ten, a SPAD is a **threshold ("click/no-click") detector** with no intrinsic photon-number resolution — a limitation shared, as we will see, by SNSPDs, and one with real consequences for heralding and boson sampling.

## Silicon SPADs: Visible and Near-IR

Silicon SPADs cover ~400–1000 nm (bandgap 1.12 eV) and are superbly mature:

- **Detection efficiency:** 50–70% peak in the 600–800 nm range (thick-junction devices such as the classic SPCM modules); thin-junction devices trade some efficiency for speed.
- **Dark counts:** 25–500 cps at Peltier temperatures; the best selected devices reach a few cps.
- **Timing jitter:** ~350 ps FWHM for thick-junction, high-efficiency devices; 30–50 ps for thin-junction designs — the efficiency/jitter trade-off follows from where in the depletion region the photon is absorbed.
- **Afterpulsing:** carriers trapped in deep levels during an avalanche are released later, causing correlated false counts (~0.5–3%). This is why the hold-off time exists, and it contaminates correlation measurements at short delays — an artifact to remember when interpreting $g^{(2)}(\tau)$ data.

Silicon SPADs are the natural partner for quantum-dot photons at 900–950 nm (with reduced efficiency, ~15–30%) and for the NV/hBN emitters of Section 19.1.3. Large monolithic arrays — SiPMs (analog-summed SPAD arrays) and CMOS SPAD imagers with $>10^5$ pixels — power LiDAR and fluorescence-lifetime imaging, and give a taste of quasi-photon-number resolution by spatial multiplexing.

## InGaAs/InP SPADs: The Telecom Problem

At 1550 nm, silicon is transparent; the absorber must be InGaAs (bandgap 0.75 eV) with the multiplication region in InP. The narrow gap raises the price of Geiger operation:

- **Efficiency:** 10–30% typically (with ~55% reported in optimized devices).
- **Dark counts:** $10^3$–$10^4$ cps free-running at 220–250 K — orders of magnitude above silicon — driven by trap-assisted tunneling in the high-field InP.
- **Afterpulsing:** severe, forcing either long (μs) hold-offs or **gated operation**: the diode is armed only in narrow windows synchronized to expected photon arrivals. Self-differencing and sine-gating techniques (developed largely by Toshiba's Cambridge group for QKD) allow GHz gating with sub-ns windows, recovering usable count rates for clocked protocols.
- **Negative-feedback avalanche diodes (NFADs)** with integrated quench resistors enable free-running telecom SPADs with ~100 cps DCR when cooled to ~193 K — the standard choice for fielded QKD receivers where a 2–4 K cryostat is impractical.

## Photomultipliers, in One Paragraph

The photomultiplier tube — a photocathode followed by electron-multiplying dynodes — was the original single-photon detector and still owns niches needing large areas and blue/UV sensitivity (neutrino detectors, scintillation counting). For quantum photonics its numbers (QE 10–40% in the visible, ~2% in the near-IR, ns-scale jitter for standard tubes) have been overtaken on every axis, and we set it aside.

## Where SPADs Stand

| Metric | Si SPAD | InGaAs SPAD | (For contrast) SNSPD |
|---|---|---|---|
| Wavelength | 400–1000 nm | 950–1650 nm | UV–mid-IR |
| SDE | 50–70% (peak) | 10–30% | >90%, up to 98% |
| DCR | 25–500 cps | $10^3$–$10^4$ cps (free-run) | <1–100 cps |
| Jitter | 35–350 ps | 50–200 ps | 3–50 ps |
| Dead time | 20–100 ns | μs-scale or gated | 10–50 ns |
| Operating T | 250–300 K | 200–250 K | 0.8–4 K |

The verdict for quantum computing at telecom wavelengths is unambiguous: InGaAs SPADs are the detector you use when you cannot afford a cryostat; SNSPDs are the detector you use when you cannot afford the photons. Since Chapter 20's architectures cannot afford the photons, we turn to superconductors.

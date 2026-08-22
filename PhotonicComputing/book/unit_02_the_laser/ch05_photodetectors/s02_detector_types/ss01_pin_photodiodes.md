# 5.2.1 p-i-n Photodiodes

## Structure and Operation

The p-i-n photodiode is the standard detector for analog and digital optical communications and for photonic computing output stages. Its structure is a sandwich: a **p**-type layer, an **i**ntrinsic (lightly doped or undoped) absorbing layer, and an **n**-type layer. A reverse bias voltage ($V_R \approx 3$–10 V) is applied, creating a strong electric field across the intrinsic layer that fully depletes it.

When a photon with $\hbar\omega > E_g$ is absorbed in the intrinsic layer, an electron-hole pair is created. The electric field sweeps the electron toward the n-contact and the hole toward the p-contact, at near-saturation drift velocities ($v_s \sim 10^7$ cm/s). The carrier transit produces a photocurrent pulse that flows in the external circuit.

**Advantages of the p-i-n structure over a simple p-n junction**:
1. The intrinsic layer thickness can be independently chosen to optimize QE vs. bandwidth tradeoff
2. Full depletion at low reverse bias (small field needed to deplete intrinsic layer)
3. Low junction capacitance (thick intrinsic layer → low $C_j = \varepsilon A/d$)
4. Low dark current (low doping in intrinsic region → low thermal carrier generation)

## Performance Parameters

| Parameter | Typical value (InGaAs, 1550 nm) | Notes |
|---|---|---|
| Quantum efficiency $\eta$ | 70–95% | With AR coating |
| Responsivity $\mathcal{R}$ | 0.9–1.1 A/W | Near theoretical max |
| Dark current $I_d$ | 1–100 nA | Limited by thermal generation |
| Bandwidth $f_{3\text{dB}}$ | 3–60 GHz | Depends on area and bias |
| Capacitance | 0.1–1 pF | For $r$ = 25 μm at bias |
| Operating wavelength | 900–1700 nm | Set by InGaAs bandgap |
| Reverse bias | 3–10 V | |

**InGaAs (In$_{0.53}$Ga$_{0.47}$As) lattice-matched to InP** is the standard detector material at 1310 nm and 1550 nm. Its bandgap is 0.74 eV ($\lambda_c = 1.67$ μm), it absorbs efficiently at both telecom wavelengths ($\alpha \approx 7000$ cm$^{-1}$ at 1550 nm), and its device performance is well-established.

## Linear Mode Operation

For analog photonic computing, linearity of $I_{ph}$ vs. $P_{in}$ is critical. p-i-n photodiodes are highly linear over many orders of magnitude:

- **Low power**: Shot noise limited, but the responsivity is constant: $\mathcal{R} =$ const
- **High power**: Saturation occurs when the space-charge of the photogenerated carriers screens the internal electric field, reducing sweep-out velocity. The 1-dB compression point (where responsivity drops by 1 dB from small-signal value) is typically at photocurrents of 10–100 mA for standard detectors.

For photonic matrix processors operating at powers of 0.1–10 mW, corresponding to photocurrents of 0.1–10 mA, standard p-i-n detectors remain in the linear regime. The nonlinearity-induced precision loss is negligible compared to shot noise and thermal noise.

## Integrated p-i-n Detectors

In silicon photonic chips, p-i-n detectors are integrated directly with waveguides using butt-coupling or evanescent coupling. The light enters the detector chip in-plane, traveling along the waveguide axis. This waveguide-photodetector geometry (as discussed in Section 5.1.2) decouples the absorption length from the depletion width, enabling simultaneous high QE and high bandwidth. Standard silicon photonic foundry offerings (IMEC iSiPP50G, GlobalFoundries Fotonix) include Ge-on-Si waveguide p-i-n detectors as standard process design kit (PDK) components.

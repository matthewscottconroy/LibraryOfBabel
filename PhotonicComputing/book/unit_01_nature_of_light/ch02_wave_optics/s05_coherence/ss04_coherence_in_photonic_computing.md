# 2.5.4 — Coherence in Photonic Computing

## The Fundamental Choice: Coherent vs. Incoherent

Photonic computing architectures divide into two broad classes based on whether they exploit optical coherence:

**Coherent architectures** use the complex amplitude of the optical field as the computational variable. Computation is performed via interference — constructive or destructive combination of fields with controlled phase relationships. The MZI mesh (Section 2.2.4) is the canonical example: the output depends on $e^{i\phi}$, the complex field amplitude, not just $|\cdot|^2$, the intensity. Coherent architectures can implement complex-valued matrix multiplications, including unitary transformations and (via SVD decomposition) arbitrary linear maps. They require:
- A coherent laser source (coherence length $\gg$ all path differences on chip)
- Phase-stable waveguide routing (low temperature drift, low vibration)
- Phase calibration procedures
- Polarization control (one polarization mode)

**Incoherent (intensity-based) architectures** use optical intensity as the computational variable. No phase control is needed; computation is performed by weighting and summing intensities. The basic operation is: weight each input signal $I_k$ by a non-negative real coefficient $w_k$ and sum: $I_\text{out} = \sum_k w_k I_k$. Incoherent architectures can implement *non-negative real-valued* matrix multiplications. They can tolerate:
- Broadband or multi-mode illumination
- Phase drift in waveguides
- Mixed polarization states
But they are limited to non-negative weights (you cannot subtract intensities) and cannot exploit interference for more general computations.

Both types of architecture have been demonstrated experimentally. The choice depends on the application, available hardware, and the acceptable noise level.

## Coherence Requirements for MZI Meshes

An $N \times N$ MZI mesh implementing the Clements decomposition [1] has path lengths ranging from the minimum (a single waveguide crossing) to the maximum (a path traversing the entire mesh). For an $N = 16$ mesh with minimum path $L_\text{min} \approx 1$ mm and maximum $L_\text{max} \approx N \cdot L_\text{min} \approx 16$ mm, the maximum path difference is $\Delta L = L_\text{max} - L_\text{min} \approx 15$ mm.

For coherent operation, the source coherence length must satisfy $L_c \gg \Delta L$. For a DFB laser with $\Delta\nu = 1$ MHz: $L_c = c/\Delta\nu = 300$ m. This is $300/0.015 = 20,000$ times larger than the required coherence length — no problem.

However, if the path length differences include a thermally unstable waveguide that drifts by $\delta L \sim 1$ nm (from temperature changes of $\sim 0.1°$C and thermo-optic coefficient $dn/dT \approx 2 \times 10^{-4}$ /K for silicon): the corresponding phase drift is $\delta\phi = 2\pi \delta L n/\lambda \approx 2\pi \times 10^{-9} \times 3.48 / 1550 \times 10^{-9} \approx 0.014$ rad $\approx 0.8°$. This is a non-negligible phase error for a circuit requiring sub-millirad precision. Thermal management is a major engineering challenge in large-scale photonic computing chips [2].

## Precision and Noise in Coherent Optical Computing

The precision of a coherent optical computation is limited by several noise sources:

**Shot noise**: Detecting $N_\text{ph}$ photons introduces an uncertainty of $\sqrt{N_\text{ph}}$ in the photon count (Poisson statistics). The signal-to-noise ratio $\text{SNR} = N_\text{ph}/\sqrt{N_\text{ph}} = \sqrt{N_\text{ph}}$. For 6-bit precision ($\text{SNR} \geq 2^6 = 64$): $N_\text{ph} \geq 64^2 = 4096$ photons per computation. At 1550 nm, each photon carries energy $E = hc/\lambda \approx 1.28 \times 10^{-19}$ J. For a 1 GHz computation rate: power needed $= 4096 \times 10^9 \times 1.28 \times 10^{-19} \approx 0.5$ nW per output — negligibly small. Shot noise is not currently the limiting noise source in analog photonic computing.

**Detector noise**: Photodetectors have *noise-equivalent power* (NEP), dark current, and amplifier noise. These set the minimum detectable signal and limit analog precision at low photon flux.

**Phase noise**: As discussed above, thermal drift and laser phase noise contribute phase errors. Calibration (periodic measurement and correction of phase settings) can reduce systematic errors, but random phase fluctuations are harder to mitigate.

**Crosstalk**: In silicon photonic circuits, light in one waveguide can couple to adjacent waveguides via evanescent fields or radiative scattering. This introduces unwanted additions to the intended signals, limiting the matrix element precision.

## Number of Effective Bits

The *number of effective bits* (ENOB) measures the actual analog precision of an optical computation, accounting for all noise sources. For a well-designed MZI mesh, ENOB of 5–8 bits has been demonstrated [3], compared to the theoretical precision limits of 8–12 bits from shot noise alone at moderate power levels. Achieving higher precision requires lower crosstalk, better thermal stability, and more accurate phase calibration.

This is directly relevant to machine learning applications: neural network inference can often tolerate 4–8 bit precision (matching ENOB achievable with current photonic hardware), while training requires higher precision (16–32 bit, not currently achievable with analog photonic systems). This is why photonic computing is most competitive for inference workloads.

## Lasers for Photonic Computing

The choice of laser source affects coherence, power efficiency, and integration:

**External cavity lasers (ECL)**: Very narrow linewidth ($< 1$ kHz), tunable, but bulky and expensive. Used in laboratory demonstrations.

**Distributed feedback lasers (DFB)**: Single-frequency, $\Delta\nu \sim 1$ MHz, compact, chip-scalable. Standard in telecommunications. Appropriate for most photonic computing applications.

**Vertical-cavity surface-emitting lasers (VCSELs)**: Low power, low cost, high-speed, but broader linewidth and multimode. Used in short-distance optical interconnects; less suitable for coherent computation.

**On-chip lasers**: Silicon is an indirect-bandgap semiconductor and does not lase efficiently in bulk. Current approaches to on-chip lasing for photonic computing include:
- Wafer-bonded III-V lasers on silicon photonic chips [4]
- Quantum dot lasers on silicon (grown directly)
- Off-chip laser sources with fiber coupling

The integration of the laser source is one of the key unsolved engineering challenges in practical photonic computing systems.

## Summary

- Coherent photonic computing: complex field amplitudes as variables; can implement any linear map; requires phase control.
- Incoherent photonic computing: intensities as variables; non-negative weights only; tolerates phase drift.
- MZI meshes require coherence length $L_c \gg$ path length differences ($\sim$ mm-scale on chip) — easily met by DFB lasers.
- Precision limited by shot noise, detector noise, phase drift, and crosstalk; ENOB of 5–8 bits currently achievable.
- Laser integration on silicon is a key open engineering challenge.

---

*References*

[1] Clements, W.R. et al. (2016). Optimal design for universal multiport interferometers. *Optica*, 3(12), 1460–1465.

[2] Bandyopadhyay, S. et al. (2022). Hardware error correction for programmable photonics. *Optica*, 9(10), 1168–1175. [DOI: 10.1364/OPTICA.455864]

[3] Hamerly, R. et al. (2022). Accurate self-configuration of rectangular multiport interferometers. *Physical Review Applied*, 18(2), 024019. [DOI: 10.1103/PhysRevApplied.18.024019]

[4] Tanaka, S. et al. (2012). High-output-power, single-wavelength silicon hybrid laser using precise flip-chip bonding technology. *Optics Express*, 20(27), 28057–28069. [DOI: 10.1364/OE.20.028057]

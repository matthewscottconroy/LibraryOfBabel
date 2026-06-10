# 7.2.4 Mach-Zehnder Interferometers (On-Chip)

## From Free Space to Silicon Photonics

The Mach-Zehnder interferometer developed in Chapter 2 (Section 2.2.4) translates directly to silicon photonics. The two beamsplitters become directional couplers or MMI couplers; the two arms are silicon waveguides with adjustable lengths (for the static phase bias) and phase shifters (for dynamic modulation).

The transfer matrix of a balanced MZI (50:50 input and output couplers):

$$U_{MZI} = e^{i(\theta+\phi)/2}\begin{pmatrix}\cos(\Delta\phi/2) & i\sin(\Delta\phi/2) \\ i\sin(\Delta\phi/2) & \cos(\Delta\phi/2)\end{pmatrix}$$

where $\Delta\phi = \phi_1 - \phi_2$ is the differential phase between the two arms (from Chapter 2, Section 2.2.4).

## Physical Implementation

A standard silicon photonic MZI:
- Input MMI 50:50 splitter (~25 μm)
- Arm 1: waveguide with phase shifter (thermo-optic heater or p-n junction for electro-optic)
- Arm 2: reference waveguide (possibly with static $\pi/2$ bias)
- Output MMI 50:50 combiner (~25 μm)
- Total device: ~200–500 μm (dominated by the arm length needed for full $\pi$ phase shift)

## Phase Shifter Efficiency: $V_\pi L$ and $V_\pi$

For a carrier-depletion (reverse-biased p-n junction) phase shifter in a 450 × 220 nm Si waveguide:
- Phase shift per length: $\Delta\phi/L = \Gamma \cdot 2\pi \Delta n_{eff}/\lambda$
- Typical plasma dispersion: $\Delta n_{eff} \approx 10^{-4}$ per volt in a lateral p-n junction
- $\pi$ phase shift requires: $L = \lambda/(2\Gamma\Delta n_{eff}) \approx 1.55\times10^{-6}/(2 \times 0.8 \times 10^{-4}) \approx 10$ mm

For a 1 V drive voltage, $V_\pi L \approx 10$ V·mm — comparable to current state-of-the-art (~1–2 V·cm for silicon p-n modulators). Reducing $V_\pi L$ requires either:
1. Stronger overlap (doping the waveguide core more heavily, increasing loss)
2. Resonant enhancement (use ring modulator instead of MZI)
3. Different material (LiNbO₃ has $V_\pi L \approx 0.2$ V·cm via Pockels effect — 50× better)

## MZI as Matrix Element

In a photonic neural network, each MZI implements a 2×2 unitary rotation. A mesh of $N(N-1)/2$ MZIs implements an arbitrary $N \times N$ unitary (Reck decomposition) or $N^2/2$ MZIs for the symmetric Clements decomposition. For $N = 64$: Reck requires 2016 MZIs; Clements requires 2048 MZIs (more balanced, equal depth). Each MZI needs 2 phase shifters for full generality ($\theta$ and $\phi$).

**Chip area**: At ~200 μm per MZI, a 64×64 Clements mesh is ~2048 × 200 μm² = ~40 cm² — far too large for a single chip. Real designs use shorter MZIs (with smaller phase shifters, accepting higher $V_\pi$) and more compact topology. The state-of-the-art 64×64 MZI mesh fits in ~1–2 cm² chip area using compact phase shifters.

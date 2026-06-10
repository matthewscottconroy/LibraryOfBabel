# 4.2.1 Longitudinal Modes and Mode Spacing

## Standing Wave Condition

A Fabry-Pérot resonator of length $L$ with refractive index $n$ (or more precisely, group index $n_g$) supports resonant modes at frequencies where the round-trip optical path length is an integer number of wavelengths:

$$2 n_g L = m \lambda_m = m \frac{c}{\nu_m}$$

$$\nu_m = \frac{mc}{2n_g L}$$

The integer $m$ is the longitudinal mode number. For a 300 μm semiconductor laser at 1550 nm ($n_g \approx 3.7$):

$$m = \frac{2 n_g L}{\lambda} = \frac{2 \times 3.7 \times 300 \times 10^{-4} \text{ cm}}{1.55 \times 10^{-4} \text{ cm}} \approx 1432$$

The mode spacing (free spectral range) is:

$$\Delta\nu_{FSR} = \frac{c}{2n_g L}$$

For the 300 μm laser: $\Delta\nu_{FSR} = 3 \times 10^{10}/(2 \times 3.7 \times 300 \times 10^{-4}) \approx 135$ GHz, corresponding to a wavelength spacing of $\Delta\lambda \approx \lambda^2\Delta\nu/c \approx 1.1$ nm.

## Multi-Mode vs. Single-Mode Operation

A simple Fabry-Pérot semiconductor laser supports multiple longitudinal modes simultaneously: any mode that falls within the gain bandwidth experiences net gain. The gain bandwidth of a typical InGaAsP quantum well laser is approximately 30–50 nm, which contains ~30–50 longitudinal modes of a 300 μm cavity. All of these modes can lase if the cavity provides no frequency-selective feedback beyond the Fabry-Pérot condition.

Multi-mode operation is problematic for:

1. **Coherent photonic computing**: an MZI matrix multiplier requires the source to be coherent over the path length difference $\Delta L$. Multiple modes with spacing 135 GHz give a coherence length of only $c/\Delta\nu_{FSR} \approx 2.2$ mm — less than typical MZI arm length differences in chip-scale systems.

2. **WDM coherent communications**: wavelength-division multiplexed channels must be distinguishable; a multi-mode laser would occupy multiple WDM channels simultaneously.

3. **Phase noise**: multiple modes beat with each other and with the spontaneous emission background, generating intense low-frequency noise.

Single-frequency operation is achieved by adding frequency-selective feedback to the cavity — most commonly via a Bragg grating that preferentially reflects a single wavelength. This is the distributed feedback (DFB) architecture.

## Photon Lifetime and Q Factor

The quality factor of a resonator is defined as the energy stored divided by the energy lost per radian of oscillation:

$$Q = \frac{\omega_0 \tau_p}{1} = \omega_0 \tau_p$$

where $\tau_p$ is the photon lifetime. The photon lifetime is related to the round-trip losses:

$$\frac{1}{\tau_p} = v_g(\alpha_i + \alpha_m)$$

where:
- $\alpha_i$ = internal (distributed) loss coefficient (cm$^{-1}$)
- $\alpha_m = \frac{1}{2L}\ln\frac{1}{R_1 R_2}$ = mirror (output coupling) loss

For a semiconductor laser with $\alpha_i = 20$ cm$^{-1}$, $R = 0.32$ (cleaved facet, $n = 3.5$), $L = 300$ μm:

$$\alpha_m = \frac{1}{2 \times 300 \times 10^{-4}} \ln\frac{1}{0.32^2} \approx 37 \text{ cm}^{-1}$$

$$\frac{1}{\tau_p} = \frac{c}{n_g}(\alpha_i + \alpha_m) = \frac{3 \times 10^{10}}{3.7}(20 + 37) \approx 4.6 \times 10^{11} \text{ s}^{-1}$$

$$\tau_p \approx 2.2 \text{ ps}$$

The photon lifetime is the time scale on which photons escape the cavity. Together with the carrier lifetime $\tau_s \approx 1$–3 ns, the photon lifetime sets the relaxation oscillation frequency and the direct modulation bandwidth of the laser (Section 4.3.1).

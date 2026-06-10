# 4.3.4 Laser Noise

## The Two Fundamental Noise Sources

A laser has two fundamental noise sources, both arising from the quantum nature of light-matter interaction:

1. **Phase noise**: Spontaneous emission events inject photons with random phases into the cavity mode, causing the phase of the coherent field to perform a random walk. This gives the laser a finite linewidth.

2. **Intensity noise (amplitude fluctuations)**: The discrete nature of photon emission and the coupled carrier-photon dynamics produce fluctuations in the output power. The relevant figure of merit is the **relative intensity noise (RIN)**.

## Relative Intensity Noise (RIN)

RIN is defined as the mean-square power fluctuation normalized to the square of the average power:

$$\text{RIN}(f) = \frac{\langle|\delta P(f)|^2\rangle}{P_{avg}^2} \quad \text{[dB/Hz]}$$

The RIN spectrum has three regimes:

1. **Low frequency ($f \ll f_R$)**: Flat, dominated by 1/f (flicker) noise and spontaneous emission. Typical level: −140 to −160 dBc/Hz for a good DFB laser.

2. **Near relaxation oscillation ($f \approx f_R$)**: Large RIN peak due to the resonant carrier-photon coupling. Can be 10–30 dB above the low-frequency floor.

3. **High frequency ($f \gg f_R$)**: RIN rolls off as $f^{-4}$ above the relaxation resonance.

**Photonic computing implication**: RIN noise on the optical carrier sets the noise floor of any analog photonic computing system. If the laser has RIN = −140 dBc/Hz, and the analog computation is performed over a bandwidth $B$ = 10 GHz, the signal-to-noise ratio contribution from RIN alone is approximately:

$$\text{SNR}_{RIN} = -\text{RIN}(\text{dBc/Hz}) - 10\log_{10}(B) = 140 - 100 = 40 \text{ dB}$$

This corresponds to effective number of bits (ENOB) of approximately $\text{ENOB} \approx (40 - 1.76)/6.02 \approx 6.4$ bits. Current analog photonic processors are RIN-limited to approximately 5–8 bits of precision [1], consistent with this estimate.

Reducing RIN requires operating well above threshold ($P \gg P_{th}$, which increases photon density and suppresses the relative effect of spontaneous emission), using low-noise drive electronics, and in some cases using optical feedback or injection locking to stabilize the laser.

## Phase Noise and Laser Linewidth

The phase of the laser field $\phi(t)$ undergoes a random walk due to spontaneous emission. Each spontaneous emission event adds a phase step of order $1/\sqrt{S}$, where $S$ is the photon number. The phase diffusion coefficient is:

$$D_\phi = \frac{R_{sp}}{2S^2}$$

where $R_{sp}$ is the spontaneous emission rate into the mode. The resulting phase noise power spectral density has a $1/f^2$ spectrum (Wiener process), and the optical spectrum has a Lorentzian lineshape with FWHM:

$$\Delta\nu = \frac{D_\phi}{\pi} = \frac{R_{sp}}{2\pi S^2} = \frac{\hbar\omega v_g^2 \alpha_{tot}^2 n_{sp}}{4\pi P_{out}}$$

which is the Schawlow-Townes formula derived earlier (Section 4.2.3), now derived from the rate equation / Langevin perspective rather than the quantum optics perspective.

## Frequency Noise and Phase Noise Spectrum

The one-sided power spectral density of frequency noise is:

$$S_{\nu\nu}(f) = \frac{\Delta\nu_{ST}}{\pi} + \frac{f^2}{2\pi^2 \tau_p^2 P} |\delta N(f)|^2 + S_{flicker}(f)$$

where the three terms are: (1) white frequency noise (Schawlow-Townes floor), (2) technical noise from carrier density fluctuations (dominant near $f_R$), and (3) low-frequency 1/f flicker noise.

The laser linewidth measured over a time scale $T$ (integral of the frequency noise spectrum up to $1/T$) includes contributions from all these components:

$$\Delta\nu(T) = 2\pi \int_0^{1/T} S_{\nu\nu}(f)\,df$$

For a DFB laser with Schawlow-Townes floor at 100 kHz and $1/f$ noise knee at 1 MHz, the intrinsic linewidth (integration time < 1 μs) is ~100 kHz, but the measured linewidth at 1 ms integration time may be ~10 MHz due to $1/f$ noise.

## Optical Frequency Noise and Photonic Computing

For coherent photonic matrix multiplication, the critical parameter is the phase coherence time $\tau_{coh} = 1/(\pi\Delta\nu_{free-running})$, which determines the maximum allowed path length difference in the MZI circuit. For a 10 MHz DFB laser: $\tau_{coh} \approx 32$ ns, corresponding to a coherence length of $\sim$10 m — more than sufficient for chip-scale photonic circuits (path differences of mm to cm).

However, some photonic computing architectures route signals through cascaded MZI networks where the total accumulated path length difference can grow. For a $64 \times 64$ Clements MZI mesh with 4 mm mean arm-length difference: total coherence budget is ~16 cm, requiring coherence length $> 16$ cm, i.e., $\Delta\nu < c/(16 \text{ cm}) \approx 2$ GHz — easy for any DFB laser.

## References

[1] Miscuglio, M., & Sorger, V.J. (2020). "Photonic tensor cores for machine learning." *Applied Physics Reviews*, 7(3), 031404.

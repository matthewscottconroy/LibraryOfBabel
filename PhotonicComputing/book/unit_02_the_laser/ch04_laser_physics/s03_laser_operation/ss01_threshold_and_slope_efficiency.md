# 4.3.1 Threshold, Slope Efficiency, and Direct Modulation

## The LI Curve

The most fundamental characterization of a semiconductor laser is its light-current (LI) curve: output power $P$ vs. injection current $I$. Below threshold, spontaneous emission produces incoherent light that increases slowly with current. Above threshold, stimulated emission takes over and $P$ grows linearly with $I - I_{th}$:

$$P = \eta_d \frac{\hbar\omega}{e}(I - I_{th}), \quad I > I_{th}$$

The slope $\eta_d \hbar\omega/e$ (in W/A) is the *differential quantum efficiency* times the photon energy per electron charge. A typical 1550 nm DFB laser has $\eta_d \approx 0.25$ mW/mA, giving 2.5 mW output at 10 mA above threshold.

The kink at $I = I_{th}$ is the laser transition — not a phase transition in the thermodynamic sense (it is smoothed by spontaneous emission), but a sharp change in operating regime that separates the incoherent LED-like regime from the coherent laser regime.

## Direct Modulation Bandwidth

For photonic computing and optical communications, it is often desirable to modulate the laser output directly by modulating the drive current $I(t) = I_0 + \delta I \cos(2\pi f_m t)$. The small-signal response of the rate equations to this perturbation gives the modulation transfer function:

$$\left|\frac{\delta P}{\delta I}\right|^2 \propto \frac{f_R^4}{(f_R^2 - f_m^2)^2 + f_m^2 \Gamma_R^2 / (4\pi^2)}$$

where:
- $f_R$ = relaxation oscillation frequency
- $\Gamma_R$ = damping rate of relaxation oscillations

The relaxation oscillation frequency is:

$$f_R = \frac{1}{2\pi}\sqrt{\frac{v_g \partial g/\partial N \cdot S_{th}}{\tau_p}}$$

For typical semiconductor laser parameters, $f_R \approx 5$–15 GHz at moderate bias above threshold, scaling approximately as $\sqrt{I - I_{th}}$.

**Key result**: The −3 dB modulation bandwidth is approximately $f_{-3\text{dB}} \approx 1.55 f_R$ in the absence of parasitic effects. With careful laser design and package optimization, bandwidths of 20–35 GHz have been demonstrated for directly modulated DFB lasers [1]. This corresponds to data rates of 25–50 Gbit/s per wavelength using simple on-off keying.

**Limitation for photonic computing**: Direct modulation causes simultaneous amplitude and frequency modulation (chirp), because the carrier density $N$ modulates the gain and the refractive index simultaneously (via the linewidth enhancement factor $\alpha_H$). The instantaneous frequency chirp is:

$$\delta\nu(t) = -\frac{\alpha_H}{4\pi}\left(\frac{1}{P}\frac{dP}{dt} + \kappa P\right)$$

where the first term is transient chirp (from the leading/trailing edge of the pulse) and the second term is adiabatic chirp (from the shift in carrier density with power level). This chirp converts to amplitude noise after propagation in dispersive fiber and limits the transmission distance. For photonic computing on a chip (short distances), chirp causes spectral broadening that limits the wavelength-division multiplexing channel spacing.

For high-precision analog photonic computing (where each MZI implements a matrix multiplication and the precision matters), direct modulation with its associated chirp and nonlinear LI response is typically inadequate. External modulation (using Mach-Zehnder or ring modulators, discussed in Chapter 7) provides better linearity, lower chirp, and higher bandwidth.

## Relaxation Oscillations

Relaxation oscillations are the natural response of the coupled carrier-photon system to perturbation. When the laser is turned on from below threshold, or perturbed by a change in current, the photon and carrier densities ring at frequency $f_R$ before settling to the new steady state.

Physically: when photon density is high, stimulated emission depletes carriers, reducing gain, which reduces photon density, allowing carriers to recover, increasing gain and photon density — an oscillating feedback loop. The damping rate $\Gamma_R$ determines how quickly the oscillation decays.

Relaxation oscillations can cause significant intensity noise at frequency $f_R$ even in nominally cw operation, appearing as a peak in the relative intensity noise (RIN) spectrum. For photonic computing systems using analog signal levels, this noise peak sets a constraint on the operating current and the design of electronic driver circuits.

## References

[1] Matsui, Y., Schatz, R., Che, D., Khan, F., Kwakernaak, M., & Sudo, T. (2021). "55 GHz bandwidth distributed reflector laser." *Journal of Lightwave Technology*, 39(2), 520–527.

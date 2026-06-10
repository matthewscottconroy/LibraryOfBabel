# 6.3.1 Self-Phase Modulation in Fiber Links

## SPM in Transmission

A pulse propagating through fiber accumulates a nonlinear phase $\phi_{NL} = \gamma P(t) L_{eff}$ where $L_{eff} = (1-e^{-\alpha L})/\alpha$ is the effective interaction length. For $L \gg L_{att} = 1/\alpha$: $L_{eff} \to 1/\alpha \approx 22$ km at 1550 nm.

The maximum nonlinear phase accumulated in a single span is:

$$\phi_{NL,max} = \gamma P_{launch} L_{eff} = 1.3 \times P_{launch}[\text{W}] \times 22000 = 28600 \times P_{launch}$$

For $P_{launch} = 1$ mW: $\phi_{NL} = 28.6$ mrad — negligible. For $P_{launch} = 100$ mW: $\phi_{NL} = 2.86$ rad — significant spectral broadening.

**Rule of thumb for WDM systems**: Nonlinear impairments become significant when $\phi_{NL} > 1$ rad, giving maximum launch power:

$$P_{max,NL} = \frac{1}{\gamma L_{eff}} = \frac{1}{1.3 \times 22000} \approx 35 \text{ mW} \approx +15.4 \text{ dBm}$$

In practice, WDM systems launch each channel at +5 to +15 dBm, staying below the nonlinear threshold.

## SPM-Induced Spectral Broadening

SPM converts phase modulation to frequency modulation: the instantaneous frequency is $\delta\omega(t) = -d\phi_{NL}/dt = -\gamma L_{eff} dP/dt$. For a Gaussian pulse, this adds a quadratic chirp that interacts with fiber dispersion — either compressing or broadening the pulse, depending on the dispersion sign. In anomalous dispersion (C-band), SPM-induced positive chirp partially compensates dispersion-induced pulse spreading, improving transmission in some regimes (this is the basis of quasi-linear soliton transmission).

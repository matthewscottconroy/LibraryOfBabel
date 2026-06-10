# 6.3.2 Cross-Phase Modulation and WDM Crosstalk

## XPM Mechanism

When two optical channels co-propagate in a fiber, the Kerr nonlinearity of one channel modulates the phase of the other. The XPM-induced phase shift on channel 2 due to channel 1 is:

$$\phi_{XPM} = 2\gamma P_1(t) L_{eff}$$

The factor of 2 (compared to SPM) arises because cross-phase modulation from a co-polarized wave is twice as strong as self-phase modulation (the third-order susceptibility tensor: $\chi^{(3)}_{xyxy} = \chi^{(3)}_{xxyy} = \chi^{(3)}_{xxxx}/3$, but for co-polarized: $\chi_{eff} = \chi_{xxxx}$, while for SPM: same; for cross-polarized XPM the coefficient is 2/3).

XPM converts amplitude modulation on channel 1 into phase modulation (and eventually, through dispersion, into amplitude modulation) on channel 2. This is a source of inter-channel crosstalk in WDM photonic systems.

## XPM Reduction via Dispersion

XPM is weakest when channels walk off from each other rapidly — when dispersion makes the pulses from different channels travel at different speeds, reducing the effective interaction length. The walk-off length:

$$L_W = \frac{T_{pulse}}{|D||\Delta\lambda|}$$

For $T_{pulse} = 10$ ps, $|D| = 17$ ps/(nm·km), $|\Delta\lambda| = 0.8$ nm (100 GHz): $L_W = 10/(17 \times 0.8) = 0.74$ km. Channels walk off within < 1 km, limiting XPM interaction to this effective length instead of 22 km. This is why WDM transmission requires finite dispersion (not $D = 0$): dispersion suppresses both FWM and XPM via walk-off.

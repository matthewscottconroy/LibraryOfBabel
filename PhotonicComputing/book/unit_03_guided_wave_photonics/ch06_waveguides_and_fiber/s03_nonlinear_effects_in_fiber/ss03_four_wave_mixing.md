# 6.3.3 Four-Wave Mixing and Channel Crosstalk

## FWM in WDM Systems

Four-wave mixing in a WDM system occurs when three channels at frequencies $\omega_1, \omega_2, \omega_3$ generate a fourth frequency $\omega_4 = \omega_1 + \omega_2 - \omega_3$ (or permutations). If this fourth frequency coincides with another WDM channel, it adds coherently and creates crosstalk.

**Phase matching condition**: FWM is efficient only when $|\Delta k| = |\beta(\omega_1) + \beta(\omega_2) - \beta(\omega_3) - \beta(\omega_4)| \ll 1/L$. For equally spaced channels with spacing $\Delta\omega$:

$$\Delta k = \beta_2 \Delta\omega^2$$

FWM is maximally efficient when $\Delta k \to 0$, i.e., near $\lambda_{ZD}$ where $\beta_2 \approx 0$.

**FWM power**: In the undepleted pump approximation, the FWM product power is:

$$P_{FWM} \approx \frac{(\gamma L_{eff})^2 P_1 P_2 P_3}{1 + (\Delta k L_{eff}/2)^2}$$

For SMF-28 at 1550 nm with 100 GHz channel spacing: $\Delta k \approx \beta_2 (2\pi \times 100\text{GHz})^2 = (-26.9\text{ps}^2/\text{km})(4\pi^2\times10^{22}\text{s}^{-2}) \approx -10.6 \times 10^3$ m$^{-1}$. With $L_{eff} = 22$ km: $\Delta k L_{eff}/2 \approx 116 \gg 1$. FWM power is suppressed by a factor of $1 + 116^2 \approx 10^4$ compared to $\Delta k = 0$. This is the reason SMF-28 (non-zero dispersion) is used for WDM rather than DSF (zero dispersion at 1550 nm): FWM is negligible.

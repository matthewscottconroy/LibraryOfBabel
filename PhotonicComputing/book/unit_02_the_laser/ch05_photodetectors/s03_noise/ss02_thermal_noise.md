# 5.3.2 Thermal (Johnson) Noise

## The Fluctuation-Dissipation Theorem

Thermal noise — also called Johnson noise or Nyquist noise — arises from the thermal agitation of charge carriers in any resistive element. Unlike shot noise (quantum origin), thermal noise is a classical phenomenon: it results from the thermodynamic equilibrium between a resistor and the thermal bath.

Johnson's experimental measurement (1928) and Nyquist's theoretical derivation (1928) established the fundamental result [1,2]: a resistor $R$ at temperature $T$ produces a voltage noise with one-sided power spectral density:

$$S_V(f) = 4k_B T R \quad \text{[V²/Hz]}$$

Equivalently, the current noise when the resistor is loaded is:

$$S_i(f) = \frac{4k_BT}{R} \quad \text{[A²/Hz]}$$

In bandwidth $B$:

$$\langle i^2_{thermal}\rangle = \frac{4k_BTB}{R}$$

This is the **Johnson noise** formula. For a 50 Ω load resistor at $T = 300$ K, $B = 10$ GHz:

$$i_{thermal,rms} = \sqrt{\frac{4\times1.38\times10^{-23}\times300\times10^{10}}{50}} = \sqrt{3.3\times10^{-10}} \approx 18 \text{ μA}$$

This is 10× larger than the signal shot noise at 1 mW input computed in Section 5.3.1 — the 50 Ω receiver is thermally noise dominated, not shot-noise limited.

## Transimpedance Amplifier Noise and the Input-Referred Noise

In practice, the detector is not loaded by a simple 50 Ω resistor. A **transimpedance amplifier (TIA)** provides a virtual ground at the detector output (low input impedance) and converts the photocurrent to a voltage with gain $Z_T = R_F$ (feedback resistor). The effective load resistance for noise purposes is $R_F$, which can be much larger than 50 Ω:

$$\langle i^2_{thermal,TIA}\rangle \approx \frac{4k_BTB}{R_F} + S_{i,amp}B$$

where $S_{i,amp}$ is the input-referred current noise of the amplifier (typically $\sim 10$ pA/√Hz for a good CMOS TIA). For $R_F = 1$ kΩ and $B = 10$ GHz:

$$i_{thermal,TIA,rms} = \sqrt{\frac{4\times1.38\times10^{-23}\times300\times10^{10}}{1000}} \approx 4 \text{ μA}$$

This is 4.5× smaller than with 50 Ω but still dominates over signal shot noise at moderate signal levels.

## When Is Shot-Noise-Limited Operation Achievable?

Comparing shot noise and thermal noise:

$$\langle i^2_{shot}\rangle = 2eI_{ph}B > \langle i^2_{thermal}\rangle = \frac{4k_BTB}{R_F}$$

$$I_{ph} > \frac{2k_BT}{eR_F}$$

For $R_F = 1$ kΩ, $T = 300$ K:

$$I_{ph} > \frac{2\times1.38\times10^{-23}\times300}{1.6\times10^{-19}\times10^3} = 52 \text{ μA} \Rightarrow P_{in} > 52 \text{ μW at } \mathcal{R} = 1 \text{ A/W}$$

So for signal powers above ~50 μW with a 1 kΩ TIA, shot noise dominates over Johnson noise. Below this level, the receiver is thermally noise limited (Johnson-noise dominated), and increasing $R_F$ or cooling the detector improves SNR.

**For coherent detection** (Section 5.4.2), a balanced receiver eliminates laser RIN and allows shot-noise-limited operation at even lower signal powers.

## References

[1] Johnson, J.B. (1928). "Thermal agitation of electricity in conductors." *Physical Review*, 32(1), 97–109.

[2] Nyquist, H. (1928). "Thermal agitation of electric charge in conductors." *Physical Review*, 32(1), 110–113.

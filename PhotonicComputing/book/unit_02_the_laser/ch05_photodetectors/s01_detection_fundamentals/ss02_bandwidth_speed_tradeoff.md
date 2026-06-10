# 5.1.2 Bandwidth and the Speed-Efficiency Tradeoff

## Two Limits on Detector Bandwidth

A photodetector's bandwidth (the frequency range over which it responds accurately to modulated optical signals) is limited by two physical mechanisms:

1. **Transit time**: The time for photogenerated carriers to traverse the depletion region and be collected. Faster collection → higher bandwidth.

2. **RC time constant**: The capacitance of the depletion region combined with the series resistance creates an RC lowpass filter. Smaller capacitance (thinner depletion region, smaller area) → higher bandwidth.

These two effects pull in opposite directions:
- **Thicker depletion region**: longer transit time (slower) but lower capacitance per unit area (faster) and higher absorption efficiency (more QE)
- **Thinner depletion region**: shorter transit time (faster) but higher capacitance (slower) and lower absorption (less QE)

The optimum depletion thickness balances these effects.

## Transit-Time Limit

For a depletion region of width $d$, carriers travel at the saturation drift velocity $v_s \approx 10^7$ cm/s (GaAs, InP) or $\sim 6 \times 10^6$ cm/s (Ge) in a strong electric field. The transit time is $\tau_{tr} = d/v_s$, and the transit-time-limited bandwidth is:

$$f_{tr} \approx \frac{0.45 v_s}{d}$$

(The factor 0.45 comes from the shape of the carrier transit waveform — it is not a simple step function.) For a Ge detector with $d = 1$ μm and $v_s = 6 \times 10^6$ cm/s:

$$f_{tr} = \frac{0.45 \times 6 \times 10^6 \text{ cm/s}}{10^{-4} \text{ cm}} = 27 \text{ GHz}$$

## RC-Time-Constant Limit

The junction capacitance of a circular-aperture p-i-n detector with depletion width $d$ and radius $r$ is:

$$C_j = \varepsilon \frac{\pi r^2}{d}$$

where $\varepsilon = \varepsilon_r \varepsilon_0$ is the semiconductor permittivity. With series resistance $R_s$ (contact resistance + substrate) and load resistance $R_L$ (typically 50 Ω):

$$\tau_{RC} = (R_s + R_L) C_j$$

$$f_{RC} = \frac{1}{2\pi(R_s + R_L) C_j}$$

For a Ge detector with $r = 5$ μm, $d = 1$ μm, $\varepsilon_r = 16$, $R_s + R_L = 50$ Ω:

$$C_j = 16 \times 8.85 \times 10^{-12} \times \frac{\pi (5\times10^{-6})^2}{10^{-6}} = 11 \text{ fF}$$

$$f_{RC} = \frac{1}{2\pi \times 50 \times 11\times10^{-15}} = 290 \text{ GHz}$$

In this case, the RC limit is much higher than the transit time limit — the bandwidth is dominated by transit time.

## Combined Bandwidth and the Tradeoff

The combined −3 dB bandwidth is approximately:

$$\frac{1}{f_{3\text{dB}}^2} \approx \frac{1}{f_{tr}^2} + \frac{1}{f_{RC}^2}$$

The optimization problem: for a fixed area (fixed power coupling, fixed RC), choose $d$ to maximize $f_{3\text{dB}}$. Setting $f_{tr} = f_{RC}$ gives the optimum depletion width. For Ge at 1550 nm with 5 μm radius:

$$d_{opt} \approx \sqrt{\frac{0.45 v_s \varepsilon_r \varepsilon_0 A}{\pi f \cdot \ldots}}$$

Numerically, optimum Ge detectors for 1550 nm achieve > 50 GHz bandwidth at moderate efficiency (QE ~60–80%).

## The Quantum Efficiency-Bandwidth Product

The fundamental figure of merit combining QE and bandwidth:

$$\eta \cdot f_{3\text{dB}} = (1-R)(1-e^{-\alpha d}) \cdot f_{3\text{dB}}(d)$$

As $d$ increases: $\eta$ increases (more absorption) but $f_{tr}$ decreases. As $d$ decreases: $\eta$ falls but $f_{tr}$ increases. The product $\eta \cdot f_{3\text{dB}}$ has a maximum at an intermediate $d$.

For Ge at 1550 nm ($\alpha \approx 10^4$ cm$^{-1}$), the optimum depletion width for maximizing $\eta \cdot f_{3\text{dB}}$ is approximately $d \approx 0.5$–1 μm, giving products of 30–50 GHz with reasonable QE.

**Waveguide photodetectors**: The quantum efficiency-bandwidth tradeoff can be circumvented by using a waveguide geometry where light travels *parallel* to the depletion region (along the waveguide axis) rather than perpendicular. In this case, absorption length is set by the waveguide length (can be made long for high QE), while the depletion width sets only the bandwidth (can be made thin for high $f$). Waveguide-integrated Ge photodetectors in silicon photonic platforms routinely achieve > 40 GHz bandwidth with > 0.9 A/W responsivity — near the theoretical limit.

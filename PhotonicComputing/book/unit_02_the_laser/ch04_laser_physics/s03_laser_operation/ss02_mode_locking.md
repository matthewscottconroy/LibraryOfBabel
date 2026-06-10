# 4.3.2 Mode-Locking

## From Continuous Wave to Pulses

A Fabry-Pérot laser cavity supports many longitudinal modes. In free-running operation, these modes oscillate with independent, random phases — the total output is incoherent, noisy, and cw. If instead the modes are forced to maintain a fixed phase relationship, they interfere constructively at one point in time and destructively elsewhere, producing a short pulse that travels back and forth in the cavity.

This is mode-locking: the deliberate synchronization of phases across many longitudinal modes to produce ultrashort optical pulses.

## Interference of $N$ Phase-Locked Modes

Consider $N$ modes equally spaced by $\Delta\nu_{FSR} = c/(2n_g L)$, each with amplitude $E_0$ and phase locked to zero:

$$E(t) = E_0 \sum_{k=-(N-1)/2}^{(N-1)/2} e^{i 2\pi(\nu_0 + k\Delta\nu)t}$$

This is a geometric series. The total field is:

$$E(t) = E_0 \frac{\sin(N\pi\Delta\nu t)}{\sin(\pi\Delta\nu t)} e^{i2\pi\nu_0 t}$$

The intensity envelope is:

$$I(t) \propto \left[\frac{\sin(N\pi\Delta\nu t)}{\sin(\pi\Delta\nu t)}\right]^2$$

This is a train of pulses with:
- **Repetition period**: $T_{rep} = 1/\Delta\nu = 2n_g L/c$ (the round-trip time)
- **Peak power**: $N^2 \times$ average power (modes constructively interfere)
- **Pulse duration**: $\Delta t_{pulse} \approx 1/(N \cdot \Delta\nu) = 2n_g L/(cN)$

For $N = 100$ locked modes in a 1 cm semiconductor laser at 1550 nm: $T_{rep} = 2 \times 3.5 \times 10^{-2}/(3 \times 10^{10}) \approx 2.3$ ps $\times$ ... correction: $T_{rep} = 2n_g L/c = 2 \times 3.5 \times 0.01 / (3\times10^8) = 233$ ps (repetition rate ~4.3 GHz). Pulse duration $\approx 2.3$ ps.

## Methods of Mode-Locking

**Active mode-locking**: A modulator inside the cavity (electro-optic or acousto-optic) is driven at frequency $\Delta\nu_{FSR}$. The modulator creates sidebands on each mode at $\pm\Delta\nu$, which overlap with adjacent modes and phase-lock them. This requires the modulator frequency to be precisely matched to the round-trip frequency, making it sensitive to environmental perturbations.

**Passive mode-locking**: A saturable absorber inside the cavity preferentially absorbs low-intensity light (spontaneous emission, noise) while bleaching (becoming transparent) for high-intensity pulses. This creates a mechanism that favors pulsed operation: a circulating pulse bleaches the absorber as it passes, allowing itself to be amplified, while cw light experiences continuous absorption. Passive mode-locking produces shorter pulses than active mode-locking and requires no external modulation frequency reference.

**Colliding pulse mode-locking** and **SESAM** (semiconductor saturable absorber mirror) mode-locking: Advanced variants of passive mode-locking used in high-repetition-rate semiconductor lasers. SESAM-based fiber lasers can produce pulses as short as 50–100 fs at repetition rates of 100 MHz–10 GHz.

## Pulse Parameters: The Time-Bandwidth Product

For a transform-limited pulse (no chirp), the time-bandwidth product is a constant determined by the pulse shape:

| Pulse shape | $\Delta t \cdot \Delta\nu$ |
|---|---|
| Gaussian | 0.4413 |
| Sech² (soliton) | 0.3148 |
| Square | 0.8859 |

A 1 ps sech² pulse has $\Delta\nu \approx 0.315/T_0 \approx 315$ GHz. This sets the minimum spectral width of a mode-locked pulse train.

## Relevance to Photonic Computing

Mode-locked lasers have two primary relevance areas for photonic computing:

1. **WDM comb sources**: A mode-locked laser produces equally spaced frequency tones — a frequency comb — that can serve as the multi-wavelength source for WDM photonic matrix processors. The comb lines are mutually coherent (they all derive from the same optical oscillator), which is important for coherent photonic computing. The spacing between comb lines ($\Delta\nu_{FSR}$) sets the WDM channel spacing.

2. **Optical sampling and time-stretch ADC**: For analog photonic computing applications involving radio-frequency signal processing, mode-locked lasers enable photonic analog-to-digital conversion (the time-stretch ADC technique) at bandwidths far exceeding electronic ADCs.

The microresonator Kerr comb (Section 4.4.4) is an alternative to mode-locked lasers for on-chip WDM comb generation — it offers much higher repetition rates (25–500 GHz) in a chip-scale form factor.

# 5.4.2 Coherent vs. Direct Detection

## Direct Detection: Squaring the Field

Standard photodetection is **direct detection**: the detector responds to optical intensity (power):

$$I_{ph} = \mathcal{R}P = \mathcal{R}|E|^2$$

The detector squares the field amplitude — it destroys phase information. The output current represents only the amplitude of the optical signal. This is sufficient for many photonic computing architectures (MZI-based matrix processors where the output of each MZI is an intensity), but it discards potentially useful phase information.

**Limitations of direct detection**:
1. Cannot measure optical phase (important for coherent photonic computing)
2. Receives full laser RIN (intensity fluctuations of the source)
3. Thermally noise limited at low powers (50 Ω receiver)

## Coherent Detection: Mixing with a Local Oscillator

In **coherent detection**, the signal field $E_s$ is mixed with a strong local oscillator (LO) field $E_{LO}$ using a 2×2 optical coupler (a 50:50 beamsplitter in integrated form), and the two outputs are detected by a **balanced pair** of photodetectors:

$$E_1 = \frac{1}{\sqrt{2}}(E_s + E_{LO}), \quad E_2 = \frac{1}{\sqrt{2}}(E_s - E_{LO})$$

The difference current:

$$I_{diff} = I_1 - I_2 = \mathcal{R}|E_1|^2 - \mathcal{R}|E_2|^2 = 2\mathcal{R}\text{Re}[E_s E_{LO}^*]$$

$$= 2\mathcal{R}\sqrt{P_s P_{LO}} \cos(\phi_s - \phi_{LO})$$

This is **homodyne detection**: the output current is proportional to the cosine of the phase difference between signal and LO. The output is *linear* in the signal amplitude $\sqrt{P_s}$ (rather than quadratic in direct detection), and it measures phase.

## Benefits of Balanced Coherent Detection

1. **LO gain**: The factor $2\mathcal{R}\sqrt{P_{LO}}$ amplifies the signal current. A strong LO ($P_{LO} \gg P_s$) provides gain before the TIA, suppressing the effect of TIA thermal noise.

2. **RIN cancellation**: Since both detectors see the same LO and signal, common-mode intensity fluctuations (including laser RIN) cancel in the difference $I_1 - I_2$.

3. **Shot-noise-limited sensitivity**: With balanced detection and LO gain, the dominant noise is the shot noise of the LO current $I_{LO} = \mathcal{R}P_{LO}$:

$$\langle i^2_{noise,balanced}\rangle = 2 \times 2e I_{LO} B = 4e\mathcal{R}P_{LO}B$$

SNR:

$$\text{SNR}_{coherent} = \frac{(2\mathcal{R}\sqrt{P_sP_{LO}})^2}{4e\mathcal{R}P_{LO}B} = \frac{\mathcal{R}P_s}{eB}$$

This is **2× better** than the direct detection shot-noise limit ($\mathcal{R}P_s/2eB$), corresponding to a 3 dB sensitivity advantage. The factor of 2 comes from the balanced subtraction recovering all the signal power.

4. **Phase sensitivity**: The $\cos(\phi_s - \phi_{LO})$ dependence allows measurement of the signal optical phase, enabling phase-encoded computation or QKD.

## 90° Optical Hybrid: Full IQ Detection

A 90° hybrid (four-port 90° optical coupler) separates the in-phase (I) and quadrature (Q) components of the signal field, allowing simultaneous measurement of both quadratures:

$$I_{diff,I} = 2\mathcal{R}\sqrt{P_sP_{LO}}\cos(\phi_s - \phi_{LO})$$
$$I_{diff,Q} = 2\mathcal{R}\sqrt{P_sP_{LO}}\sin(\phi_s - \phi_{LO})$$

From $I$ and $Q$, the complex amplitude $E_s$ can be fully reconstructed:

$$E_s \propto I_{diff,I} + i I_{diff,Q}$$

IQ coherent detection is the basis of 100/400/800 GBit/s coherent optical communications using advanced modulation formats (DP-QPSK, 16-QAM, 64-QAM). It is also used in some photonic computing architectures that encode matrix weights in the complex amplitude (both phase and amplitude) of optical fields.

## Comparison for Photonic Computing

| Property | Direct detection | Balanced coherent |
|---|---|---|
| Phase sensitivity | No | Yes |
| Sensitivity | Thermally limited (low $P$) | Shot-noise limited |
| RIN rejection | None | ~30–40 dB common-mode rejection |
| Complexity | Simple (1 detector + TIA) | Higher (2 detectors, hybrid, LO source) |
| Need for LO laser | No | Yes (must be phase-coherent with signal) |
| Effective SNR at 100 μW | ~40 dB (thermal limited) | ~50–55 dB (shot-noise limited) |

For current photonic matrix processors (operating at milliwatt power levels with modest precision requirements), direct detection is typically adequate and simpler to implement. For future systems requiring > 8-bit precision or for quantum photonic processors, coherent detection becomes necessary.

# 5.4.1 Transimpedance Amplifiers

## The TIA Circuit

The standard optical receiver front-end is the shunt-feedback transimpedance amplifier:

```
      R_F
   ┌──/\/\/──┐
   │         │
   ├──────[A]──── V_out
   │
   └── (photodetector)
```

An inverting amplifier with gain $-A$ and feedback resistor $R_F$ presents an input impedance $Z_{in} = R_F/(A+1) \approx R_F/A$ (a virtual ground) to the detector. The transimpedance gain is:

$$Z_T = \frac{V_{out}}{I_{ph}} = -A \cdot Z_{in} = \frac{-A R_F}{A+1} \approx -R_F$$

The bandwidth of the TIA is set by the gain-bandwidth product of the amplifier and the feedback network. The closed-loop −3 dB bandwidth is:

$$f_{3\text{dB}} \approx \sqrt{\frac{A}{2\pi R_F C_T}}$$

where $C_T = C_j + C_{in}$ is the total capacitance at the input node (detector capacitance + amplifier input capacitance). This is the classic **TIA bandwidth-gain tradeoff**: larger $R_F$ gives higher gain (lower noise at low signal levels) but lower bandwidth. For fixed $A$ and $C_T$:

$$R_F \cdot f_{3\text{dB}}^2 = \text{const} = \frac{A}{2\pi C_T}$$

**Example**: $A = 40$ dB = 100, $C_T = 100$ fF, target $f_{3\text{dB}} = 25$ GHz:

$$R_F = \frac{100}{2\pi \times 100\times10^{-15} \times (25\times10^9)^2} = \frac{100}{3.93\times10^{-1}} \approx 254 \text{ Ω}$$

This relatively small $R_F$ (254 Ω) gives reasonable bandwidth but not the lowest noise. Achieving 25 GHz with $R_F = 1$ kΩ would require $A > 400$ (> 52 dB), demanding a higher-gain-bandwidth product amplifier.

## Input-Referred Noise of a TIA

The TIA adds noise beyond the Johnson noise of $R_F$. The dominant noise sources in a CMOS TIA are:

1. **$R_F$ Johnson noise**: $4k_BTB/R_F$ (current noise at input)
2. **Amplifier input transistor thermal current noise**: $S_{i,FET} = 4\gamma k_B T g_m$ (where $\gamma \approx 2/3$ for long-channel FET, $g_m$ is transconductance)
3. **Amplifier input capacitance noise**: Noise current that charges $C_{in}$ through the transistor

The total input-referred noise current spectral density of a well-designed CMOS TIA is approximately:

$$S_{i,TIA}(f) \approx \frac{4k_BT}{R_F} + S_{i,FET} + (2\pi f)^2 \cdot C_T^2 \cdot S_{v,FET}$$

The third term (rising with frequency) reflects the fact that for high-frequency signals, the noise voltage of the input transistor is amplified by the $C_T f$ factor — this is the "capacitive noise peaking" that ultimately limits TIA sensitivity at high bit rates.

## TIA for Photonic Computing: Design Considerations

For analog photonic computing (MZI matrix processors), the TIA requirements differ from digital communications:

1. **Linearity**: The output voltage must accurately represent the photocurrent over the full dynamic range of the matrix output. Unlike digital receivers (where you only care about 0 or 1), analog TIAs need linearity over the full output range.

2. **Dynamic range**: The matrix output at different operating points can vary by 20–40 dB. The TIA must handle this range without saturation or clipping.

3. **Bandwidth vs. precision tradeoff**: For a photonic accelerator running at 1 GHz matrix-vector products, the TIA bandwidth needs to be ~1 GHz. Reducing bandwidth from 10 GHz to 1 GHz reduces Johnson noise by 10× (noise ∝ √B) and allows larger $R_F$, improving SNR by ~10 dB.

4. **Co-integration with silicon photonics**: The TIA must be integrated on the same chip as the Ge photodetector (or at minimum co-packaged to minimize parasitic capacitance). Most state-of-the-art silicon photonic computing chips use monolithically integrated CMOS TIAs in the same 45 nm or 16 nm process node as the photonic components.

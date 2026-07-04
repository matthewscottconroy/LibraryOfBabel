# 25.1.1 DACs, ADCs, and TIAs: The Conversion Tax

## The Conversion Chain

Every analog photonic computation is wrapped in the same electronic envelope:

```
 digital       ┌─────┐   ┌────────┐   ┌──────────┐   ┌─────┐   ┌─────┐      digital
 weights   ──► │ DAC │──►│ driver │──►│ photonic │──►│ PD +│──►│ ADC │──►   result
 & inputs      └─────┘   └────────┘   │   core   │   │ TIA │   └─────┘
                  ▲                   └──────────┘   └─────┘      ▲
                  │                        ▲                      │
                  └──────────── clock distribution ───────────────┘
```

Each stage costs energy per sample, adds noise, and limits precision. This subsection puts numbers on each stage — numbers we will reuse for every system-level budget in this chapter.

## Digital-to-Analog Converters

A photonic accelerator needs DACs in two roles: **input DACs**, which encode each element of the incoming activation vector as a modulator drive voltage at the full clock rate (one per input channel, e.g. 8–12 bits at 10–50 GS/s), and **weight DACs**, which program the phase shifters or ring biases that hold the matrix (one per weight, but usually updated slowly and multiplexed).

The energy of a high-speed CMOS DAC is dominated by switching its output network and driving its load. As a rule of thumb from surveyed designs, current-steering DACs at 8–10 bits and tens of GS/s consume tens to a few hundred milliwatts, i.e.

$$E_{\text{DAC}} \approx 0.5\text{–}2\ \text{pJ per sample (8–10 bit, 10–50 GS/s class)}$$

Slower, lower-resolution DACs are cheaper (well below 100 fJ/sample at MS/s rates), but a photonic accelerator running its inputs at gigahertz rates cannot use them for activations.

To the DAC we must add the **modulator driver**. For a lumped ring modulator with capacitance $C \approx 10$ fF driven at $V_{pp} = 1$ V, the switching energy is $E = C V_{pp}^2 \approx 10$ fJ — nearly free. For a 50-Ω-terminated traveling-wave MZM driven at $V_{pp} = 1$ V, the termination dissipates continuously:

$$P_{\text{term}} \approx \frac{V_{pp}^2}{4R} = \frac{1}{4 \times 50} = 5\ \text{mW} \;\Rightarrow\; 0.5\ \text{pJ/sample at 10 GS/s}$$

This factor-of-50 spread between ring and MZM drive energy (consistent with the device numbers of Chapter 7) is one reason dense WDM ring architectures dominate recent energy-oriented designs.

## Analog-to-Digital Converters

The ADC is usually the single most expensive element of the chain. Its energy is characterized by the **Walden figure of merit**,

$$\text{FOM}_W = \frac{P}{2^{\text{ENOB}} \cdot f_s} \quad [\text{J per conversion-step}]$$

where ENOB is the effective number of bits and $f_s$ the sample rate. Murmann's long-running ADC survey [2, 3] shows a stable envelope: the best converters below ~100 MS/s reach a few fJ per conversion-step, while designs in the multi-GS/s regime needed here pay a substantial speed penalty, achieving roughly

$$\text{FOM}_W \approx 10\text{–}100\ \text{fJ/step at } f_s \gtrsim 10\ \text{GS/s}$$

The per-sample energy follows as $E_{\text{ADC}} = \text{FOM}_W \cdot 2^{\text{ENOB}}$:

| ENOB | $f_s$ | FOM$_W$ | Energy per sample |
|------|-------|---------|-------------------|
| 4 | 10 GS/s | 30 fJ/step | 0.5 pJ |
| 6 | 10 GS/s | 30 fJ/step | 1.9 pJ |
| 8 | 10 GS/s | 15 fJ/step (optimistic) | 3.8 pJ |
| 8 | 50 GS/s | 50 fJ/step | 12.8 pJ |

Two scalings in this table drive the entire architecture debate. First, energy is *exponential in bits*: each additional effective bit doubles the ADC energy. This is why photonic accelerator proposals cluster at 4–8 bit precision, and why "peak TOPS" quoted at 4 bits cannot be compared against GPU TOPS at 8 or 16 bits without adjustment. Second, energy is *linear in sample count*: an $N$-output accelerator clocked at $f$ pays $N f E_{\text{ADC}}$ watts in digitization no matter how efficient its optics.

### The Jitter Ceiling

Beyond energy, ADCs impose a hard precision ceiling. Sampling a signal of input frequency $f_{\text{in}}$ with RMS aperture jitter $\sigma_j$ limits the SNR to

$$\text{SNR}_{\text{jitter}} = -20 \log_{10}\!\left(2\pi f_{\text{in}} \sigma_j\right)$$

With excellent integrated clocking, $\sigma_j \approx 100$ fs. At $f_{\text{in}} = 10$ GHz this gives $\text{SNR} = 44$ dB, or

$$\text{ENOB} = \frac{44 - 1.76}{6.02} \approx 7.0\ \text{bits}$$

and only ~5.7 bits at 25 GHz. **No electronic converter delivers 8+ effective bits on multi-tens-of-GHz analog signals**; this is a limit of clock physics, not of design skill [1, 2]. Any photonic accelerator claiming simultaneously ">8 bits" and ">25 GHz analog symbol rate" at its digitized output has claimed something the ADC literature does not know how to build — the first of many audit checks this chapter will accumulate. (The converse opportunity — using photonics itself to beat electronic jitter, as in time-stretch ADCs — is taken up in Section 25.3.2.)

## Transimpedance Amplifiers

The photodetector output is a current — microamps for the optical power levels of Chapter 5 — and must be converted to a voltage large enough for the ADC's input range. The transimpedance amplifier that does this contributes:

- **Noise.** A good high-speed TIA has input-referred current noise density $i_n \approx 2\text{–}20\ \text{pA}/\sqrt{\text{Hz}}$. Over a 10 GHz bandwidth, $\sigma_I = i_n \sqrt{B} \approx 0.2\text{–}2\ \mu\text{A}$ RMS. For $b$-bit output precision the peak signal photocurrent must exceed $2^b \sigma_I$: at 6 bits and $\sigma_I = 0.5\ \mu$A, that is 32 μA — about 40 μW of optical power at 0.8 A/W responsivity. TIA noise, not shot noise, typically sets the receiver-side optical power requirement (Section 25.2.2).
- **Power.** Broadband analog gain is expensive: tens of milliwatts for a 25–100 GHz-class TIA, i.e. roughly 0.5–5 pJ per sample after amortizing over the symbol rate.

## Clock Distribution

Converters at gigahertz rates require low-jitter clocks distributed across the die and the photonic-electronic boundary with picosecond skew. PLLs, clock buffers, and clock-data recovery typically add 10–30% on top of the converter power itself — a line item that vanishes from most published budgets but not from the wall plug.

## The Tax Table and the Amortization Principle

Collecting the working numbers (per sample, order-of-magnitude, early-2020s CMOS):

| Interface element | Energy per sample |
|-------------------|-------------------|
| Input DAC (8 bit, ≥10 GS/s) | 0.5–2 pJ |
| Modulator drive (ring / MZM) | 0.01 / 0.1–0.5 pJ |
| TIA + receiver analog front end | 0.5–5 pJ |
| ADC (6–8 ENOB, ≥10 GS/s) | 1–10 pJ |
| Clocking overhead | +10–30% |
| **Total conversion tax** | **≈ 2–15 pJ per input-output sample pair** |

Compare: a complete 8-bit digital MAC costs ~0.2–0.3 pJ in 45 nm CMOS [4], and a few tens of femtojoules in leading-edge nodes. Sample-for-sample, the conversion chain is 10–100× *more* expensive than simply doing the arithmetic digitally. The photonic core only wins because it does not pay the tax per operation. In an $N \times N$ photonic matrix-vector multiplier, one vector of $N$ conversions on each edge yields $N^2$ MACs in the interior:

$$\boxed{E_{\text{interface per MAC}} = \frac{E_{\text{DAC}} + E_{\text{mod}} + E_{\text{TIA}} + E_{\text{ADC}}}{N}}$$

With a 5 pJ tax and $N = 64$: 78 fJ/MAC. With $N = 8$: 625 fJ/MAC — worse than a mobile NPU, regardless of how perfect the optics are. **Small photonic accelerators lose before the light is even turned on.** This single equation explains the field's obsession with large $N$, and Sections 25.2.1–25.2.2 will quantify the loss, noise, and static-power penalties that push back against it.

One further term belongs in the accounting: weight updates. If the matrix is reprogrammed at rate $f_{\text{up}}$ while computing at clock $f$, each MAC carries an added burden $E_{\text{DAC,weight}} \cdot (f_{\text{up}}/f)$ — negligible for static inference weights, ruinous if the matrix changes every cycle. Weight-stationary dataflow is not merely a scheduling preference in photonics; it is an energy requirement (Section 25.3.1).

---

## References

[1] Walden, R.H. (1999). "Analog-to-digital converter survey and analysis." *IEEE Journal on Selected Areas in Communications*, 17(4), 539–550. [The original ADC survey; source of the jitter-limited SNR analysis and the FOM formulation.]

[2] Murmann, B. "ADC Performance Survey 1997–2023." Online: https://web.stanford.edu/~murmann/adcsurvey.html [Continuously updated dataset of published ADCs; the empirical envelope for energy per conversion-step quoted in this subsection.]

[3] Murmann, B. (2015). "The race for the extra decibel: a brief review of current ADC performance trajectories." *IEEE Solid-State Circuits Magazine*, 7(3), 58–66. [Readable summary of ADC energy scaling trends and their physical limits.]

[4] Horowitz, M. (2014). "1.1 Computing's energy problem (and what we can do about it)." *IEEE International Solid-State Circuits Conference (ISSCC) Digest of Technical Papers*, 10–14. [Canonical energy-per-operation numbers for digital arithmetic and memory access in 45 nm CMOS.]

[5] Al-Qadasi, M.A., Chrostowski, L., Shastri, B.J., & Shekhar, S. (2022). "Scaling up silicon photonic-based accelerators: challenges and opportunities." *APL Photonics*, 7(2), 020902. [Full-system energy analysis of MZI- and microring-based accelerators, itemizing DAC, ADC, TIA, laser, and tuning power; the model behind the amortization arguments here.]

[6] Nahmias, M.A., Ferreira de Lima, T., Tait, A.N., Peng, H.-T., Shastri, B.J., & Prucnal, P.R. (2020). "Photonic multiply-accumulate operations for neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 7701518. [Defines the photonic MAC and its energy scaling, including the O(N) conversion / O(N²) computation asymmetry.]

[7] Miller, D.A.B. (2017). "Attojoule optoelectronics for low-energy information processing and communications." *Journal of Lightwave Technology*, 35(3), 346–396. [The device-physics roadmap for reducing every optoelectronic energy in this table toward femtojoule and attojoule scales.]

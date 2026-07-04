# Important Concepts — Chapter 25: Co-Design and Benchmarking

## The Central Thesis

No photonic computer is purely photonic. Data arrives and leaves as electronic bits; the optical core only does linear algebra in the middle. **In nearly every photonic accelerator built to date, the majority of energy, latency, area, and cost lives in the electronics.** Whether photonics wins is decided not by the elegance of the optics but by how many cheap optical operations are extracted per expensive electronic conversion.

## The Conversion Tax

Every analog optical computation is wrapped in the same envelope: DAC → driver → **photonic core** → photodetector + TIA → ADC, all clocked. Each stage costs energy per sample and spends from the precision budget.

| Interface element | Energy per sample (early-2020s CMOS, ≥10 GS/s) |
|-------------------|-----------------------------------------------|
| Input DAC (8-bit) | 0.5–2 pJ |
| Modulator drive (ring / MZM) | 0.01 / 0.1–0.5 pJ |
| TIA + receiver front end | 0.5–5 pJ |
| ADC (6–8 ENOB) | 1–10 pJ |
| Clocking overhead | +10–30% |
| **Total tax** | **≈ 2–15 pJ per input–output sample pair** |

A full 8-bit digital MAC costs only ~0.2–0.3 pJ (45 nm) to a few tens of fJ (leading node). **Sample-for-sample, the conversion chain is 10–100× more expensive than just doing the arithmetic digitally.** Photonics wins only because it does not pay the tax per operation.

## The Amortization Principle (Why Large $N$)

An $N\times N$ optical multiplier does $N^2$ MACs per pass but pays only $O(N)$ conversions at its edges:

$$E_{\text{interface per MAC}} = \frac{E_{\text{DAC}} + E_{\text{mod}} + E_{\text{TIA}} + E_{\text{ADC}}}{N}$$

A 5 pJ tax gives 78 fJ/MAC at $N=64$ but 625 fJ/MAC at $N=8$ (worse than a mobile NPU). Small photonic accelerators lose before the light is turned on.

## The Master Equation for Energy per MAC

$$E_{\text{MAC}} = \underbrace{\frac{E_{\text{laser}}}{N}}_{\div\,\text{WPE}} + \underbrace{\frac{E_{\text{DAC}}+E_{\text{mod}}+E_{\text{TIA}}+E_{\text{ADC}}}{N}}_{\text{conversion tax}} + \underbrace{\frac{P_{\text{static}}}{N^2 f}}_{\text{weight hold + control}} + \underbrace{E_{\text{digital}}}_{\text{accumulate, requantize, move}}$$

Marketing conversion: $\text{TOPS/W} = 2/E_{\text{MAC}}[\text{pJ}]$, so **1 pJ/MAC ↔ 2 TOPS/W** and **100 fJ/MAC ↔ 20 TOPS/W**.

**Worked-example lessons (64×64, 10 GHz, 6-bit):** the optical computation itself is ~5% of the budget; static thermo-optic hold (732 fJ/MAC → 2.3 TOPS/W) can single-handedly drop the system *below* an H100, while non-volatile weights (156 fJ/MAC → ~13 TOPS/W) put it several-fold above. The choice of phase-shifter technology outweighs every optical innovation combined.

## Operation Counting

By convention a multiply-accumulate is **two operations**: $1\ \text{TMAC/s} = 2\ \text{TOPS}$. Both conventions appear in the literature. Peak throughput of an $N\times N$ core at vector rate $f$: $R_{\text{peak}} = N^2 f\ \text{MAC/s} = 2N^2 f\ \text{OPS}$ (410 TOPS for $N=64$, $f=50$ GHz).

**Peak vs delivered.** Utilization $= R_{\text{delivered}}/R_{\text{peak}}$; report it or the benchmark is astrology. GPUs run 40–70% of peak on large GEMMs, single-digit percent on small kernels — photonic cores have their own utilization discounts, so compare delivered-to-delivered on a named workload.

## Precision Is a Derived Quantity

Analog optics has no bit width, only a signal-to-noise-and-distortion ratio:

$$\text{ENOB} = \frac{\text{SNDR}_{\text{dB}} - 1.76}{6.02}$$

**Shot-noise photon budget:** to resolve $2^b$ levels, $n_{\max} \gtrsim 2^{2b}$ photons per output symbol — each extra bit costs 4× the light. This confines optical computing to 4–8 bits and is the fundamental energy–precision exchange rate.

| $b$ | Photons $2^{2b}$ | Detected energy/symbol (1550 nm) |
|-----|------------------|----------------------------------|
| 4 | 256 | 33 aJ |
| 6 | 4,096 | 0.52 fJ |
| 8 | 65,536 | 8.4 fJ |
| 10 | 1.05 × 10⁶ | 134 fJ |

Practical floors usually beat the shot limit: **RIN** caps SNR at $1/(\text{RIN}\cdot B)$ (≈8 ENOB at −150 dB/Hz, ≈6.4 at −140 dB/Hz over 10 GHz); **TIA thermal noise** typically sets the receiver optical power; **phase error** accumulates with mesh depth as a frozen matrix error requiring recurring calibration.

## The Jitter Ceiling (a Hard ADC Limit)

$$\text{SNR}_{\text{jitter}} = -20\log_{10}(2\pi f_{\text{in}}\sigma_j)$$

With $\sigma_j \approx 100$ fs, ENOB $\approx 7.0$ at 10 GHz and only ~5.7 at 25 GHz. **No electronic converter delivers 8+ effective bits on multi-tens-of-GHz analog signals** — a limit of clock physics, and a standing audit check.

## The $O(N)$/$O(N^2)$ Asymmetry and Its Three Penalties

Amortization ($1/N$) fights three countervailing forces as $N$ grows: **loss** in dB grows linearly with mesh depth (laser power grows exponentially); **static power** grows as $N^2$ (thermo-optic hold ~30 W at $N=64$); **area** grows as $N^2$ at photonic density, capping single-die meshes near $N\sim100$–300 (reticle limit). Escapes: WDM channel counts, spatial tiling, free-space optics, non-volatile weights.

## Hybrid Integration

| Approach | Die-to-die energy | Main risk |
|----------|-------------------|-----------|
| Wirebond 2.5D | 1–10 pJ/bit | bandwidth ceiling |
| Interposer 2.5D | 0.1–1 pJ/bit | cost, routing length |
| 3D stack | 0.01–0.1 pJ/bit | thermal crosstalk |
| Monolithic | ~0.01 pJ/bit | legacy (inefficient) logic node |

Interface parasitic capacitance multiplies the entire conversion budget ($E \sim C_{\text{total}}V^2$); integration technology is the first co-design decision. Figure of merit: **die-to-die bandwidth density at a given pJ/bit**.

## Co-Design: Partition by Comparative Advantage

| Task | Domain | Reason |
|------|--------|--------|
| Dense linear algebra (MVM, conv, FFT) | Photonic | passive interference; $N^2$ MACs/pass; WDM |
| Fan-out / interconnect | Photonic | loss- not $CV^2$-dominated |
| Nonlinear activation | Electronic | a transistor beats any fJ-scale optical nonlinearity |
| Accumulation at precision | Electronic (digital) | analog summation grows noise as $\sqrt{K}$ |
| Memory (weights, activations) | Electronic | no optical SRAM/DRAM exists |
| Control, scheduling, calibration | Electronic | general-purpose logic |

Amdahl governs: accelerating the linear 90% by 100× caps end-to-end speedup near 9×.

## Fair Comparison: Five Normalizations

1. Same operation counting (MAC = 2 OP; exclude sparsity multipliers).
2. Same precision, or explicit precision normalization (state TOPS *at* ENOB).
3. Same boundary — the wall plug (laser ÷ WPE, all converters, tuning/TEC, control, cooling PUE).
4. Delivered, on a named workload, at stated accuracy (the MLPerf discipline).
5. Same economic axes (TOPS/mm², cost/part, energy per *inference*).

## The Pitfall Catalog (Audit Vocabulary)

Core-vs-system boundary swap · peak-for-delivered substitution · precision arbitrage · MAC/OP factor-of-two · sparsity "up-to" multipliers · accuracy silence · hero-device extrapolation · amortization by assumption · benchtop laundering (AWGs/EDFAs/scopes doing the DAC/ADC/laser job for free) · baseline sandbagging.

## The Auditor's Checklist

**Boundary** (what's in the watts?) · **Numerator** (measured? utilization? precision? MAC or OP?) · **Accuracy** (hardware vs identical digital model?) · **Scale** (largest array actually measured? yield, calibration, drift?) · **Latency** (at what batch size, including conversion?) · **Reproducibility** (MLPerf-class or bespoke?).

## Where Each Domain Wins

**Photonics wins:** fixed-weight low-latency inference (batch-1 economics = batch-N economics; deterministic ns latency); WDM-parallel and optics-native tasks (signals already optical, or too fast for any ADC); interconnect (already in production, e.g. TPU v4 optical circuit switches).

**Electronics wins (today):** guaranteed/error-corrected precision (>8–12 bit); cheap abundant nonlinearity; **memory** (no photonic SRAM/DRAM — the field's deepest limitation); integration density (4–5 orders); reconfiguration and training; general-purpose control; and a 50-year, trillion-dollar manufacturing ecosystem.

**The honest summary:** with best current practice (large $N$, non-volatile weights, 3D-integrated conversion, 4–6 bit workloads, noise-aware training), photonic systems project to **several-fold, approaching an order of magnitude** better energy per delivered MAC than contemporary GPUs on favorable workloads — not the 1000× that compares an optical *core* against an electronic *system*.

## Key Numbers

| Quantity | Value |
|----------|-------|
| Conversion tax (in/out sample pair) | 2–15 pJ |
| Digital 8-bit MAC (45 nm) | 0.2–0.3 pJ |
| DRAM access energy | ~1 nJ per 64-bit word |
| Photon budget for $b$ bits | $2^{2b}$ photons/symbol |
| Photon energy at 1550 nm | 0.128 aJ |
| Laser wall-plug efficiency (typical) | 10–25% |
| ADC FOM$_W$ at ≥10 GS/s | 10–100 fJ/conversion-step |
| Jitter-limited ENOB (100 fs, 10 GHz) | ≈ 7.0 bits |
| Analog optical ENOB (practical) | 4–8 bits |
| Thermo-optic hold, 64×64 mesh | ~30 W (732 fJ/MAC at 10 GHz) |
| Non-volatile (PCM/MEMS) hold | ~40 mW (~1 fJ/MAC) |
| Photonic weight area (MZI) | ~5000 μm² |
| SRAM 6T cell area (leading node) | ~0.03 μm² |
| A100 (INT8) / H100 (INT8) peak efficiency | 1.6 / 2.8 TOPS/W |
| Photonic system, co-designed (projected) | ~13–25 TOPS/W |
| End-to-end on-chip inference latency (demo) | < 570 ps |
| TOPS/W ↔ energy per MAC | $\text{TOPS/W} = 2/E_{\text{MAC}}[\text{pJ}]$ |

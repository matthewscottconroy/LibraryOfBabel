# 25.2.3 Fair Comparison with GPUs, TPUs, and Analog Electronics

## The Machines to Beat

Any photonic accelerator claim is implicitly a comparison, so fix the reference points first (early-2020s flagships; datasheet peak, dense — i.e., without the "up to 2×" structured-sparsity multipliers marketing prefers):

| System | Peak arithmetic | Power | Peak efficiency | Memory |
|--------|----------------|-------|-----------------|--------|
| NVIDIA A100 (SXM) | 312 TFLOPS FP16 / 624 TOPS INT8 | 400 W | 0.78 / 1.6 TOPS/W | 80 GB HBM2e, ~2 TB/s |
| NVIDIA H100 (SXM) | ~990 TFLOPS FP16 / 1979 TOPS INT8 | 700 W | 1.4 / 2.8 TOPS/W | HBM3, ~3.35 TB/s |
| Google TPU v1 (2015, 28 nm) | 92 TOPS INT8 | 75 W | ~1.2 TOPS/W | 8 GiB DDR3 |
| Edge NPUs (phone-class) | 1–50 TOPS INT8 | 0.5–5 W | ~1–10 TOPS/W | LPDDR |
| Analog in-memory electronic (PCM/RRAM/flash prototypes) | claims 10–400 TOPS/W at 4–8 bit | — | same caveats as photonics | on-array weights |

Three observations calibrate expectations. First, the electronic frontier moves: dense INT8 efficiency improved roughly 2× per generation across recent flagships, so a photonic part promising "10× the A100" three years out is promising ~2–3× its actual contemporary. Second, GPUs deliver 40–70% of peak on the large GEMMs that matter and far less on small kernels — but photonic cores have their own utilization discounts (Section 25.2.1), so this cuts both ways. Third, the closest cousins of photonic accelerators are not GPUs at all but **analog-electronic in-memory accelerators** [6], which share the DAC/ADC tax, the ENOB ceiling, the noise-aware-training dependence, and — instructively — the same history of core-only efficiency claims later deflated by system measurements.

## What "Fair" Means: Five Normalizations

1. **Same operation counting.** MAC = 2 OPs, sparsity multipliers excluded, and "operations" that are actually fixed passive transforms (a lens performing an FFT) flagged as such — a diffractive element performing $N \log N$ "operations" per frame is real physics but not fungible with programmable MACs.
2. **Same precision — or explicit precision normalization.** Comparing 4-bit-equivalent analog throughput against FP16 GPU throughput inflates the photonic number by the very factor (energy ∝ exponential in bits, Sections 25.1.1/25.2.2) that makes low precision cheap. State TOPS *at* ENOB; compare like with like (the GPU also gets faster at INT8/INT4/FP8).
3. **Same boundary: the wall plug.** Laser (÷ wall-plug efficiency), TECs and thermal tuning, every converter, control FPGA/CPU share, and — for datacenter claims — cooling overhead (PUE ~1.1–1.5). If the baseline GPU is charged for its HBM and voltage regulators (it is; that's the 400/700 W), the photonic system must be charged for its periphery too.
4. **Delivered, on a named workload, at stated accuracy.** Peak-to-peak comparisons are astrology. The MLPerf discipline: fixed models and datasets, hardware result within 99% (or 99.9%) of reference accuracy, defined serving scenarios — single-stream (batch-1 latency), server (latency-bounded throughput), offline (pure throughput) — and measured system power under load [4, 5]. Nothing about this methodology is electronic-specific; photonic systems should be run through it unmodified, and mostly have not been.
5. **Same economic axes.** TOPS/mm² of silicon (photonic weights are areally expensive — Section 25.3.3), cost per part, and energy per *inference* rather than per operation. A datacenter buys throughput per dollar per watt at SLA latency; it does not buy TOPS/W.

## The Pitfall Catalog

The recurring failure modes, assembled for reuse as an audit vocabulary:

1. **Core-vs-system boundary swap** — femtojoule optics headline, picojoule system reality; the omitted items are almost always laser WPE, conversion, tuning, control.
2. **Peak-for-delivered substitution** — $N^2 f$ arithmetic presented as achieved throughput, no utilization given.
3. **Precision arbitrage** — analog 4-ish bits compared against digital 8/16 bits without normalization.
4. **MAC/OP factor-of-two gymnastics.**
5. **Sparsity and "up to" multipliers** on either side of the comparison.
6. **Accuracy silence** — throughput reported, hardware task accuracy (vs. identical digital model) not.
7. **Hero-device extrapolation** — one calibrated 4×4 tile projected to 512×512 without loss, noise, yield, or calibration-time scaling (Sections 25.2.1–25.2.2 quantify why this fails).
8. **Amortization by assumption** — weight-programming energy and calibration time divided by an unstated, effectively infinite run length.
9. **Benchtop laundering** — AWGs, EDFAs, oscilloscopes, and temperature-stabilized tables performing the DAC/ADC/laser/packaging functions for free.
10. **Baseline sandbagging** — comparing against an old GPU, a datasheet number the GPU never achieves, or a GPU running an unoptimized model.

## The Auditor's Checklist

For any published or pitched photonic-accelerator claim, ask:

> **Boundary:** What exactly is in the watts? Laser wall-plug? DAC/ADC/TIA? Thermal tuning and TEC? Control processor? Cooling?
> **Numerator:** Measured or computed? Sustained on what workload, at what utilization, what precision, MACs or OPs?
> **Accuracy:** Hardware task accuracy versus the identical model run digitally? Trained with hardware noise in the loop?
> **Scale:** Largest array actually measured? Element yield and calibration time? Drift interval between recalibrations?
> **Latency:** At what batch size? Including conversion and post-processing?
> **Reproducibility:** Standard benchmark (MLPerf-class) or bespoke task? Enough numbers disclosed to recompute the claim?

A claim that survives all six lines is rare and valuable. A claim that fails several is not necessarily fraudulent — early-stage research legitimately reports device-level physics — but it is a *device* result, and pricing it as a *system* result is the category error this chapter exists to prevent.

## Case Studies in Reading Claims

**Coherent MZI mesh (Shen et al. 2017) [1].** Landmark demonstration: a 56-MZI programmable silicon mesh executing the linear layers of a small neural network. What the paper actually established: programmable coherent linear optics works, and hardware accuracy (76.7% on vowel classification) trails the digital baseline (91.7%) for identifiable noise reasons. The widely quoted femtojoule-and-below energies from its discussion section are explicitly *forward-looking device projections*, not measurements of the reported system — the correct citation practice is to say so.

**Microcomb convolutional accelerator, "11 TOPS" (Xu et al. 2021) [2].** The headline is a genuine aggregate throughput obtained by time-wavelength interleaving many comb lines through effectively one modulator-detector chain — a triumph of WDM parallelism (Section 25.3.2). Audit notes: the demonstration ran on laboratory instrumentation (AWG, EDFAs, oscilloscope-class digitization), so no meaningful TOPS/W exists for it; MNIST accuracy on hardware (≈88%) was honestly reported against ≈90% in silico. Quoting "11 TOPS" is fair; deriving an efficiency from it is not.

**In-memory photonic tensor core (Feldmann et al. 2021) [3].** Phase-change weights hold the matrix at zero static power directly in the optical path, with comb-driven WDM parallelism — architecturally, this deletes two of the worst line items in the 25.2.1 budget (weight hold and weight fetch). Audit notes: array scale small; write endurance and analog PCM level precision (~4–5 bits) bound the workload class; the demonstration is an existence proof of in-memory photonic computing rather than a benchmarked accelerator.

**End-to-end on-chip classifier (Ashtiani et al. 2022) [7].** A complete photonic pipeline — detection, weighting, nonlinearity — classifying low-resolution images in under 570 ps end-to-end. This is the *right* latency reporting (whole task, on chip, batch 1), and a preview of Section 25.3.1's argument. Audit notes: few-class, tiny-image task; scaling the approach to useful model sizes is the open question, and the paper does not pretend otherwise.

**Commercial photonic accelerators (Hot Chips-class disclosures) [8].** Full systems — photonic tensor cores with on-package SRAM, digital control, and vendor-measured ResNet-class demos at reduced precision. These are the claims most worth auditing carefully, because they *do* draw a system boundary: the questions become which GPU mode the baseline used (FP16? INT8? measured or datasheet?), at what batch size and model, and what accuracy was achieved. Vendor numbers that survive those questions — some do, at the several-× level — are the field's most meaningful evidence to date.

**The control case: TPU v4's optical switches [5].** Photonics already ships inside benchmark-verified production AI infrastructure — as reconfigurable optical *interconnect* (circuit switches in TPU v4 pods), where its advantages need no conversion tax at all. The contrast is the chapter's thesis in miniature: photons transport and transform; electrons count and remember; systems win by assigning each its comparative advantage.

## The Honest Summary

Run through the master equation of 25.2.1 with best current practice — large $N$, non-volatile weights, 3D-integrated conversion, 4–6 bit workloads, noise-aware training — and photonic systems project to **several-fold, approaching an order of magnitude**, better energy per delivered MAC than contemporaneous GPUs on favorable (large, dense, static-weight, latency-sensitive) workloads, with a genuinely unmatched batch-1 latency story. The three-orders-of-magnitude figures in circulation compare a photonic core against an electronic system. Both facts fit in one sentence, and this book's advice is to always say the sentence whole.

---

## References

[1] Shen, Y., Harris, N.C., Skirlo, S., Prabhu, M., Baehr-Jones, T., Hochberg, M., Sun, X., Zhao, S., Larochelle, H., Englund, D., & Soljačić, M. (2017). "Deep learning with coherent nanophotonic circuits." *Nature Photonics*, 11(7), 441–446.

[2] Xu, X., Tan, M., Corcoran, B., Wu, J., Boes, A., Nguyen, T.G., Chu, S.T., Little, B.E., Hicks, D.G., Morandotti, R., Mitchell, A., & Moss, D.J. (2021). "11 TOPS photonic convolutional accelerator for optical neural networks." *Nature*, 589, 44–51.

[3] Feldmann, J., Youngblood, N., Karpov, M., Gehring, H., Li, X., Stappers, M., Le Gallo, M., Fu, X., Lukashchuk, A., Raja, A.S., Liu, J., Wright, C.D., Sebastian, A., Kippenberg, T.J., Pernice, W.H.P., & Bhaskaran, H. (2021). "Parallel convolutional processing using an integrated photonic tensor core." *Nature*, 589, 52–58.

[4] Reddi, V.J., et al. (2020). "MLPerf inference benchmark." *Proceedings of ISCA 2020*, 446–459; and Mattson, P., et al. (2020). "MLPerf training benchmark." *Proceedings of MLSys 2020*. [The accuracy-constrained, scenario-based methodology advocated throughout this subsection.]

[5] Jouppi, N.P., et al. (2017). "In-datacenter performance analysis of a tensor processing unit." *ISCA 2017*; and Jouppi, N.P., et al. (2023). "TPU v4: an optically reconfigurable supercomputer for machine learning with hardware support for embeddings." *ISCA 2023*. [Delivered-versus-peak accounting, and the production deployment of optical circuit switching.]

[6] Sebastian, A., Le Gallo, M., Khaddam-Aljameh, R., & Eleftheriou, E. (2020). "Memory devices and applications for in-memory computing." *Nature Nanotechnology*, 15, 529–544. [The analog-electronic accelerator landscape whose benchmarking lessons transfer directly to photonics.]

[7] Ashtiani, F., Geers, A.J., & Aflatouni, F. (2022). "An on-chip photonic deep neural network for image classification." *Nature*, 606, 501–506.

[8] Ramey, C. (2020). "Silicon photonics for artificial intelligence acceleration." *IEEE Hot Chips 32 Symposium*. [Representative commercial system disclosure; a productive target for the auditor's checklist.]

[9] Reuther, A., et al. (2019–2022). "Survey and benchmarking of machine learning accelerators" series. *IEEE HPEC*. [Cross-vendor peak/power landscape used to locate any new claim.]

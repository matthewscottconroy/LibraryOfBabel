# 25.3.1 Fixed-Weight Inference at Low Latency

## Why "Fixed" Changes Everything

Freeze the weight matrix, and three of the ugliest terms in the Section 25.2.1 master equation vanish or collapse:

1. **Weight-programming energy → 0.** The one-time cost of setting $N^2$ weights is amortized over every vector the matrix ever processes. A matrix resident for one second at a 10 GHz vector rate serves $10^{10}$ inputs; even a millijoule of programming energy contributes 0.1 fJ/MAC-vector... nothing.
2. **Static holding power → 0, if the technology allows.** Non-volatile weights (phase-change materials, Chapter 16; MEMS latching) hold the matrix at zero power — deleting the 732 fJ/MAC thermo-optic penalty that wrecked the worked example of 25.2.1 [3].
3. **Weight memory traffic → 0.** This is the in-memory-computing argument: in a von Neumann accelerator every MAC implies fetching a weight from SRAM (~pJ) or DRAM (~nJ) unless reuse is engineered; in a photonic fabric the weight *is* the hardware, permanently in the datapath. The only per-sample costs left are the activation-side conversions.

What remains is the accounting identity that defines this subsection: **for static weights and streaming inputs, energy per MAC ≈ (activation conversion tax)/N + laser + digital** — the most favorable regime the physics offers, plausibly 50–150 fJ/MAC (10–40 TOPS/W) with early-2020s components, several-fold better than flagship GPUs *at* far lower latency.

## The Latency Budget

Latency is where the fixed-weight case stops being merely competitive and becomes categorically different. An optical mesh is a combinational circuit: input to output in one transit, no instruction fetch, no scheduler, no cache hierarchy, no contention. Budget for one $64 \times 64$ layer:

| Stage | Time |
|-------|------|
| Input DAC settling | ~50–100 ps |
| Modulator response | ~10–20 ps |
| Optical transit (5 mm, $n_g = 4.24$) | ~70 ps |
| Photodetector + TIA | ~20–50 ps |
| ADC (pipelined; latency, not throughput) | ~0.5–2 ns |
| Digital accumulate/activation | ~1 ns |
| **End-to-end per layer** | **~2–3 ns** |

Fully analog variants that keep the nonlinearity in the optical/optoelectronic domain skip the converter stages between layers: the measured exemplar is an on-chip photonic classifier completing its entire task — detection, weighting, nonlinearity, decision — in **under 570 ps** [1].

Now the electronic comparison, honestly drawn. A GPU serving the same small model at batch 1 pays kernel-launch and memory-traffic overheads that put per-inference latency in the 0.1–1 ms range for ResNet-class models — and, critically, a GPU reaches its efficiency numbers only by *batching*, trading latency for utilization because its weights must be re-fetched from memory and amortized across many samples. The photonic fabric has nothing to amortize: **batch 1 runs at the same energy per sample and the same utilization as batch 10,000.** Where electronics offers a latency-throughput trade, fixed-weight photonics simply declines the trade. Its latency is also *deterministic* — set by picosecond-stable physical delays rather than schedulers — which safety-critical and real-time systems value as much as speed.

The comparison is least flattering to the GPU precisely where the GPU is least replaceable — enormous models — because a photonic core of size $N$ holds only an $N \times N$ tile at a time, and tiling large layers re-imports scheduling, buffering, and conversion between passes. The fixed-weight photonic sweet spot is therefore **small-to-moderate models with punishing latency or energy constraints**, not hyperscale language models.

## Where Nanosecond Inference Is Worth Money

- **Wireless physical layer and beamforming.** Massive-MIMO detection and precoding are dense matrix-vector products against slowly varying channel matrices, on microsecond deadlines, at radio bandwidths — nearly the exact specification of a photonic mesh, and the weights (channel state) update at kHz rates, well within thermo-optic reach.
- **Radar/electronic warfare and LIDAR front-ends.** Classification or filtering inside the sensor loop, where the signal is wideband and the decision deadline is physical (Section 25.3.2 continues this thread on the input side).
- **Real-time control of fast physical systems.** Plasma stabilization, accelerator and free-electron-laser feedback, optical-network equalization, high-frequency trading risk checks: loops whose plant dynamics outrun software inference.
- **Scientific triggering.** Collider and telescope first-level triggers discard petabytes by the microsecond; a fixed, occasionally retrained classifier at nanosecond latency is the job description.
- **Datacenter inference serving (tail latency).** Less exotic but larger: serving fleets provision for p99 latency; a device whose batch-1 economics equal its batch-N economics attacks the exact constraint that forces GPU overprovisioning.

## What Must Still Be Conceded

Honesty about the boundaries of the win:

- **The input must be paid for.** Unless the signal is already optical (next subsection), activations still cross a DAC per element per cycle — the irreducible ~1–2 pJ that dominates the residual budget. "Fixed-weight" removes the matrix's costs, not the vector's.
- **Precision ceilings apply unchanged.** 4–8 effective bits, noise-aware training required; workloads needing certified numerics are excluded (Section 25.3.3).
- **Amdahl governs multi-layer models.** Only the linear algebra accelerates; nonlinearities, normalization, and control remain electronic, and O/E/O boundaries between layers reintroduce conversions unless optical nonlinearity is used — with its own power and cascadability problems.
- **Retraining cadence must be slow.** The regime assumes weights static over $\gg 10^5$ vectors. Continual-learning and training workloads violate this by construction and belong to electronics.

Within those boundaries, however, fixed-weight low-latency inference is the cleanest answer this book can give to "where does photonic computing win *first*?" — the regime where every term of the master equation bends in light's favor, and where the demonstrations already published [1, 3] look less like laboratory curiosities and more like early products.

---

## References

[1] Ashtiani, F., Geers, A.J., & Aflatouni, F. (2022). "An on-chip photonic deep neural network for image classification." *Nature*, 606, 501–506. [End-to-end sub-nanosecond (<570 ps) on-chip inference; the latency exemplar for this subsection.]

[2] Shen, Y., Harris, N.C., et al. (2017). "Deep learning with coherent nanophotonic circuits." *Nature Photonics*, 11(7), 441–446. [The programmable-mesh inference architecture whose fixed-weight operating mode this subsection analyzes.]

[3] Feldmann, J., Youngblood, N., Karpov, M., et al. (2021). "Parallel convolutional processing using an integrated photonic tensor core." *Nature*, 589, 52–58. [Non-volatile phase-change weights held in the optical datapath: zero static hold power and zero weight fetch, demonstrated.]

[4] Nahmias, M.A., Ferreira de Lima, T., Tait, A.N., Peng, H.-T., Shastri, B.J., & Prucnal, P.R. (2020). "Photonic multiply-accumulate operations for neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 7701518. [Energy accounting under weight-stationary operation.]

[5] Wetzstein, G., Ozcan, A., Gigan, S., Fan, S., Englund, D., Soljačić, M., Denz, C., Miller, D.A.B., & Psaltis, D. (2020). "Inference in artificial intelligence with deep optics and photonics." *Nature*, 588, 39–47. [Survey of inference-oriented optical computing and its application niches.]

[6] Prucnal, P.R., & Shastri, B.J. (2017). *Neuromorphic Photonics*. CRC Press. [RF and real-time processing applications of weight-bank photonic processors.]

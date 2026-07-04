# Subsection 13.1.3: The Hardware Bottleneck for AI

## Orientation

Photonic neural networks are pitched as the answer to a crisis in AI hardware. Before accepting the pitch, a physicist should locate the crisis precisely. Where, in joules and seconds, does a GPU running a neural network actually spend its budget? The answer is more interesting than "arithmetic is expensive" — arithmetic is, in fact, nearly free — and it defines exactly which photonic value propositions are physically sound and which are marketing.

---

## 13.1.3.1 The Scale of the Demand

Training a frontier language model requires on the order of $10^{23}$–$10^{25}$ floating-point operations — GPT-3 (175 billion parameters, 2020) took $\approx 3\times10^{23}$ FLOP, and each successive frontier generation has multiplied that by an order of magnitude or more. At the $\sim$10$^{11}$ FLOP-per-joule efficiency of modern accelerator clusters, $10^{24}$ FLOP is a few gigawatt-hours: training runs are measured in megawatt-months of datacenter power. Inference, though cheaper per query ($\sim$10$^{9}$–10$^{12}$ FLOP per LLM response), runs continuously at global scale and now dominates total AI energy consumption. Datacenter electricity demand attributable to AI is measured in tens of terawatt-hours per year and rising steeply — the macroscopic pressure behind every "beyond-CMOS computing" research program, this book's subject included.

## 13.1.3.2 Where the Joules Go

Decompose the energy of one INT8 MAC on a modern (5 nm-class) accelerator:

| Component | Energy | Notes |
|---|---|---|
| The multiply-add itself | $\sim$0.1–1 fJ | 8-bit datapath logic |
| Register/local SRAM access | $\sim$1–10 fJ/byte | on-tile |
| On-chip network traversal | $\sim$10–100 fJ/byte | millimeters of wire |
| Off-chip HBM access | $\sim$3–10 pJ/byte | through the package |
| DRAM (DDR) access | $\sim$20–100 pJ/byte | off package |

The table contains the whole story: **moving a byte costs between 10 and 10⁵ times more than computing with it.** Wire charging energy ($CV^2$, Chapter 10) — not transistor switching — is the binding constraint. Digital architects respond by maximizing *data reuse*: systolic arrays, large batches, weight-stationary dataflows, ever-larger on-chip SRAM. When reuse is high (large batched matrix-matrix products), GPUs approach their arithmetic limit of a few fJ/MAC and are genuinely hard to beat. When reuse is low — **batch-1 inference, where each weight fetched from HBM is used exactly once** — the effective energy balloons to hundreds of fJ/MAC and throughput collapses to the memory bandwidth. This memory-bound regime is precisely where large-model interactive inference lives (every generated token must stream all the model's weights past the arithmetic units), and it is the soft target for alternative architectures.

**The photonic claims, restated against this decomposition:**

1. **Weight-stationary analog compute.** A photonic processor holding weights in phases, detunings, or PCM states *never fetches them*. It eliminates the HBM line of the table for weights, competing instead with digital SRAM-resident schemes. The honest comparison is thus photonics vs. wafer-scale/SRAM accelerators, not vs. worst-case DRAM traffic.
2. **$O(1)$-latency linear algebra.** The transit-time latency of the optical multiply ($\sim$100 ps) is $10^2$–$10^4\times$ shorter than a GPU kernel launch, enabling applications (control, RF, recurrent iteration) that are latency-bound rather than throughput-bound.
3. **$O(N)$ conversion overhead amortized over $O(N^2)$ work.** DAC/ADC/laser energy per vector scales with $N$; MACs scale with $N^2$. Beyond $N \sim 100$, conversion overhead per MAC falls below digital arithmetic energy (Nahmias et al. 2020; Hamerly et al. 2019 project sub-fJ system-level MACs at large $N$). The advantage is real but *conditional on large, well-utilized $N$* — and on workloads dominated by dense linear layers.

## 13.1.3.3 What Photonics Does Not Fix

- **Training weight updates.** Gradient descent writes every weight every step. Photonic weight technologies write in μs–ms (Section 13.1.2); the gap to electronic SRAM is orders of magnitude and is a device problem, not an architecture problem.
- **Nonlinear, branching, and pointwise work.** Softmax, normalization, sampling, tokenization — small in FLOPs, but they force domain crossings whose conversion energy must be amortized (Section 13.2).
- **Precision-critical computation.** Optimizer state, loss scaling, scientific workloads: analog ENOB does not reach them (Section 12.1).
- **The economics of maturity.** A photonic accelerator competes not with today's GPU but with the GPU shipping when it reaches production, backed by a compiler ecosystem measured in thousands of engineer-years. Architectural advantage must therefore be *structural* (physics electronics cannot follow), not incremental.

The disciplined conclusion — argued quantitatively in the energy analyses of Nahmias, Hamerly, Demirkiran, and McMahon — is that photonic neural hardware earns its place in three regimes: (i) large-$N$, weight-stationary, batch-insensitive inference; (ii) ultra-low-latency recurrent/feedback processing; (iii) signals already in the optical domain. Every credible demonstration in this unit sits in one of these regimes; every disappointing one tried to fight the GPU on its home ground.

---

## References

[1] Brown, T., et al. (2020). "Language models are few-shot learners." *Advances in Neural Information Processing Systems*, 33. [GPT-3: the paper whose Appendix compute accounting ($3.14\times10^{23}$ FLOP) anchors the training-cost figures above.]

[2] Horowitz, M. (2014). "Computing's energy problem (and what we can do about it)." *IEEE International Solid-State Circuits Conference (ISSCC)*, 10–14. [The canonical energy-per-operation table for logic, SRAM, and DRAM; the source of the data-movement hierarchy quoted here.]

[3] Nahmias, M.A., et al. (2020). "Photonic multiply-accumulate operations for neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 7701518. [System-level photonic MAC energy accounting, including conversion overheads and their $O(N)/O(N^2)$ amortization.]

[4] Hamerly, R., Bernstein, L., Sludds, A., Soljačić, M., & Englund, D. (2019). "Large-scale optical neural networks based on photoelectric multiplication." *Physical Review X*, 9, 021032. [The large-$N$ limit: standard-quantum-limited optical MACs approaching attojoule optical energies.]

[5] McMahon, P.L. (2023). "The physics of optical computing." *Nature Reviews Physics*, 5, 717–734. [A critical assessment of which optical-computing advantages survive contact with the full system energy budget.]

# 25.3.3 Where Electronics Wins (Today)

## The Complement, Stated Without Flinching

The two preceding subsections mapped the territory light holds. This one maps the far larger territory it does not. The honest reader of Sections 25.1 and 25.2 already senses the shape of the argument: everything photonics is bad at, electronics is superb at, and most of a real computation is made of exactly those things. A photonic accelerator is a specialized coprocessor for dense low-precision linear algebra sitting inside a machine that is otherwise — of necessity — electronic. This subsection enumerates why, without softening any of it, because a role is only defensible once its boundaries are drawn in ink.

Five capabilities keep electronics dominant: **precision**, **nonlinearity**, **memory**, **density**, and **general-purpose control** — buttressed by a sixth, non-technical, and possibly decisive advantage: a **manufacturing ecosystem** refined over fifty years and trillions of dollars.

## Precision and Error-Corrected Arithmetic

A digital adder is exact and its errors are correctable. ECC memory, checksummed interconnect, and redundant arithmetic make an electronic computer's numeric result reproducible to the bit across machines and decades. Analog optics offers no such guarantee: Section 25.2.2 fixed its ceiling at roughly 4–8 effective bits, drifting, and recoverable toward the digital baseline only through noise-aware training. Whole workload classes live above that ceiling by definition — FP64 scientific simulation, cryptography, exact accumulation in long reductions, financial and database arithmetic where the last bit is legally load-bearing. Even within machine learning, *training* — which needs wide dynamic range and precise gradient accumulation — stays overwhelmingly digital; photonics' inference case (25.3.1) is a case *precisely because* the hard, high-precision work of producing the weights was already done in electronics.

## Nonlinearity: Cheap in Charge, Expensive in Light

Neural networks are interleaved linear maps and pointwise nonlinearities, and only the linear half is photonics' home ground. A CMOS ReLU is a comparator: a few transistors, sub-square-micrometer, femtojoules, perfectly cascadable. An *optical* nonlinearity at comparable energy and cascadability does not exist. Practical optical nonlinear effects (Kerr, two-photon absorption, saturable absorption) demand high intensities — high-$Q$ resonators or long interaction lengths — and, lacking gain, generally cannot drive the next stage without amplification. The field's standard workaround is therefore optoelectronic: detect, apply the nonlinearity in electronics, re-modulate. But that is an O/E/O crossing (Section 25.1.1) at *every activation, every layer* — reintroducing the full conversion tax precisely where a deep model spends most of its layer boundaries. Amdahl's law (Section 25.1.2) then bites: accelerating only the linear 90% caps end-to-end speedup near 9×, and the nonlinear 10% is electronic no matter how the optics improve.

## Memory: The Field's Deepest Limitation

There is no photonic SRAM, no photonic DRAM, no photonic HBM — no optical device that stores a bit statically, at high density, and returns it on demand at low energy. Light does not hold still. This is not an engineering gap awaiting a better modulator; it is a consequence of photons being delocalized, non-interacting bosons, and it is arguably the single deepest limitation in the field. Every photonic accelerator borrows its memory from electronics, and the borrowing sets the terms.

### Worked Example: Can a Model Live in the Light?

Take a modest modern model — a 7-billion-parameter transformer (Llama-2-7B class), 7 GB of INT8 weights — and ask what it would cost to hold its weights *resident* in an optical fabric, the regime where 25.3.1's amortization argument works.

A photonic weight is areally enormous. An MZI mesh cell runs ~50 μm × 100 μm ≈ **5000 μm²** (Chapter 12); an aggressively dense microring weight, perhaps **100 μm²**. Holding all $7\times10^9$ weights at once:

$$A_{\text{MZI}} = 7\times10^9 \times 5000\ \mu\text{m}^2 = 3.5\times10^{13}\ \mu\text{m}^2 = 35\ \text{m}^2$$
$$A_{\text{ring}} = 7\times10^9 \times 100\ \mu\text{m}^2 = 7\times10^{11}\ \mu\text{m}^2 = 0.7\ \text{m}^2$$

Against an ~858 mm² reticle, that is **~41,000 reticles** (MZI) or **~800 reticles** (ring) — hundreds of full wafers of nothing but weights, before a single laser, driver, or detector. Now the electronic comparison: the same 7 GB fits inside a *fraction of one* HBM3 stack (24 GB in a ~100 mm² footprint), because a 6T SRAM cell is ~0.03 μm² and a DRAM cell smaller still. Per stored weight, electronics is **four to five orders of magnitude** denser (5000 μm² versus ~0.24 μm² for eight SRAM cells ≈ $2\times10^4$).

The consequence is structural, not incidental. A practical photonic core ($N \sim 64$–300, i.e. $10^4$–$10^5$ resident weights) holds a *tile* of this model measured in parts per hundred-thousand. Running the model means **streaming weights from electronic memory and tiling the computation** — which resurrects exactly the costs 25.3.1 claimed to delete: a weight-programming/conversion tax per tile, DRAM traffic at ~nJ per access [1] (the "weight *is* the hardware" argument evaporates when the hardware can hold only a sliver of the weights), and scheduling. The memory wall that electronic accelerators spend most of their silicon fighting is *worse* for photonics, because photonics cannot even store the operands it computes on.

This is why photonics' inference case is strongest for **small-to-moderate, static-weight** models and structurally weakest for hyperscale LLMs — and why self-attention is doubly hostile: its $QK^\top$ product is formed from activations, so *both* operands change every token, forfeiting the weight-stationary amortization on which every favorable budget in this chapter depended.

## Density, Reconfiguration, and General-Purpose Control

The density gap generalizes beyond memory. A logic transistor in a leading node is ~$10^{-2}\ \mu\text{m}^2$; a photonic component is $10^3$–$10^4\ \mu\text{m}^2$. Diffraction sets a hard floor — you cannot make a waveguide much smaller than the wavelength — so no process node will close four-to-five orders of magnitude. **Reconfiguration** is a second electronic monopoly: an SRAM weight rewrites in nanoseconds with effectively unlimited endurance, whereas thermo-optic tuning settles in microseconds, MEMS in milliseconds, and phase-change writes carry finite endurance ($\sim10^6$–$10^9$ cycles). Workloads whose weights change every step — training, adaptive filtering, online learning — are electronic by construction. And the entire scaffolding of a real computation — control flow, branching, scheduling, calibration, the operating system — is general-purpose logic, a domain in which photonics has never claimed to compete.

## The Ecosystem Argument

The final advantage is not physics. CMOS is the most refined manufacturing technology in human history: EUV lithography, yields measured in defects per billion transistors, a global supply chain, mature EDA tools, and standard interfaces. Silicon photonics is a young beneficiary of that ecosystem, not a peer to it; it inherits the fabs but not the decades of volume, yield learning, and tooling. A photonic accelerator that is 3× more efficient in principle can still lose to an electronic part that is manufacturable, yieldable, and buyable today. Benchmarking that ignores this — comparing a hero photonic device against a shipping GPU as though they carried equal product risk — commits the mirror image of the boundary error this chapter has warned against throughout.

## The Conclusion Is a Role, Not a Verdict

None of this is a case against photonic computing; it is the case *for* using it correctly. Electrons count, remember, decide, and control; photons transport and transform at bandwidths and latencies electrons cannot match. The stable conclusion of honest benchmarking is therefore not "photonics wins" or "photonics loses" but **role assignment**: photonics as a domain accelerator for dense low-precision linear algebra, and as an interconnect and routing fabric, embedded inside systems that remain — in their memory, their nonlinearity, their precision, and their control — electronic. TPU v4's optical circuit switches (Section 25.3.2) are the template already in production: photonics doing the one job the accounting unambiguously awards it, inside a machine that is otherwise silicon. The chapter's discipline exists to find, defend, and price exactly that boundary — and to keep both halves of the sentence honest.

---

## References

[1] Horowitz, M. (2014). "1.1 Computing's energy problem (and what we can do about it)." *IEEE International Solid-State Circuits Conference (ISSCC) Digest of Technical Papers*, 10–14. [The nanojoule-scale DRAM-access energies that dominate any tiled/streamed accelerator, photonic or electronic.]

[2] Nahmias, M.A., Ferreira de Lima, T., Tait, A.N., Peng, H.-T., Shastri, B.J., & Prucnal, P.R. (2020). "Photonic multiply-accumulate operations for neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 7701518. [Weight-stationary energy accounting whose assumptions this subsection stress-tests against large-model memory demands.]

[3] Shastri, B.J., Tait, A.N., Ferreira de Lima, T., Pernice, W.H.P., Bhaskaran, H., Wright, C.D., & Prucnal, P.R. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15(2), 102–114. [Review that frames memory, nonlinearity, and reconfiguration as the standing challenges for photonic AI hardware.]

[4] Miller, D.A.B. (2017). "Attojoule optoelectronics for low-energy information processing and communications." *Journal of Lightwave Technology*, 35(3), 346–396. [Why interface capacitance and the absence of low-energy optical nonlinearity/memory bound the achievable system energy.]

[5] Sebastian, A., Le Gallo, M., Khaddam-Aljameh, R., & Eleftheriou, E. (2020). "Memory devices and applications for in-memory computing." *Nature Nanotechnology*, 15, 529–544. [The electronic in-memory-computing landscape — the closest analog to a photonic weight fabric, and the memory technologies photonics must borrow.]

[6] Wetzstein, G., Ozcan, A., Gigan, S., Fan, S., Englund, D., Soljačić, M., Denz, C., Miller, D.A.B., & Psaltis, D. (2020). "Inference in artificial intelligence with deep optics and photonics." *Nature*, 588, 39–47. [Positions optical computing as inference-side acceleration within electronic systems — the role-assignment thesis of this subsection.]

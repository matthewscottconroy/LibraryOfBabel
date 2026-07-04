# 25.1.2 Hybrid Integration and Co-Design Methodology

## Why Packaging Is an Energy Problem

Subsection 25.1.1 priced each conversion. But the price depends on a hidden variable: the parasitic capacitance and inductance between the electronic circuit and the photonic device it drives. A modulator driver's dynamic energy is $E \sim C_{\text{total}} V^2$, where $C_{\text{total}}$ includes everything between the driver output and the device — bond pads, wirebonds or bumps, ESD structures, routing. The same CMOS driver that spends 10 fJ switching a 10 fF ring through a monolithic via spends a picojoule switching the same ring through a 1 pF pad-and-wirebond path. Integration technology therefore multiplies the entire interface budget, and choosing it is the first co-design decision.

## The Three Integration Options

**2.5D integration** places the photonic die and the electronic ASIC side by side in one package — on an organic substrate with wirebonds at the cheap end, or on a silicon interposer with fine-pitch microbumps at the expensive end.

- *Wirebond:* ~1 nH of inductance per millimeter of wire and picofarad-scale pad capacitance limit clean signaling to a few GHz and push drive energies toward picojoules per transition. Adequate for slow weight-programming lines; a bottleneck for data-rate signals.
- *Interposer/microbump:* bump capacitance of tens of femtofarads, signal pitches of tens of micrometers, and die-to-die energies of order 0.1–1 pJ/bit. This is the workhorse of current commercial photonic accelerators and of co-packaged optics generally, in the same packaging family as the CoWoS-style silicon-interposer assemblies used for GPU+HBM.

**3D integration** stacks the electronic die directly on the photonic die (or vice versa), connecting them with microbumps, through-silicon vias, or — at the state of the art — direct hybrid bonding at sub-10 μm pitch. Parasitics drop to a few femtofarads and link energies toward 10–100 fJ/bit, with two decisive architectural consequences: (i) every ring, MZI, and detector can have its own driver or TIA directly above it, so the $O(N)$ edge-converter model of 25.1.1 can actually be laid out without kilometer-years of routing; and (ii) analog signals travel micrometers, not millimeters, preserving bandwidth and SNR. The costs are thermal coupling (a hot ASIC sitting on temperature-sensitive rings that drift ~10 GHz/K, Chapter 7) and manufacturing complexity.

**Monolithic integration** fabricates transistors and photonics in the same process — historically in specialized platforms such as IBM's CMOS-integrated nanophotonics and GlobalFoundries' 45 nm monolithic electronic-photonic process, and in Intel's closely coupled (though not fully monolithic) transceiver flows. Parasitics become on-chip wire loads (femtofarads); the driver-modulator energy approaches the naked $CV^2$ limit. The price is that photonics inherits a mature but *old* transistor node (45–90 nm class): the accompanying digital logic is 10–20× less energy-efficient than 7 nm logic, which can cost more at the system level than the interface savings gained. Monolithic platforms shine when interface count dominates (transceivers, dense sensor interfaces); leading photonic-accelerator designs have instead favored 3D/2.5D hybrids that pair a modern CMOS node with a dedicated photonic process.

| Approach | Interface parasitics | Die-to-die energy | Photonics next to which CMOS? | Main risk |
|----------|---------------------|-------------------|-------------------------------|-----------|
| Wirebond 2.5D | ~1 nH + ~1 pF | 1–10 pJ/bit | Any (loose coupling) | Bandwidth ceiling |
| Interposer 2.5D | 10s of fF | 0.1–1 pJ/bit | Any | Cost, routing length |
| 3D stack | few–10s of fF | 0.01–0.1 pJ/bit | Any | Thermal crosstalk |
| Monolithic | ~fF (wire) | ~0.01 pJ/bit | Legacy node only | Inefficient logic |

The figure of merit to watch in datasheets is **die-to-die bandwidth density** (Tb/s per mm of die edge, or per mm²) **at a given pJ/bit** — this, more than any optical parameter, determines how large an $N$ the package can feed.

## Co-Design Methodology: Partition by Physical Comparative Advantage

Integration answers *how* to connect the domains; co-design answers *what to put in each*. The methodology, common to every successful photonic accelerator and formalized in the systems literature [1, 2, 5], proceeds top-down:

**Step 1 — Start from the algorithm.** Profile the workload. A transformer or CNN inference is dominated (typically >90% of operations) by dense matrix-vector and matrix-matrix products, connected by cheap-but-essential nonlinearities, normalizations, and data movement. Amdahl's law governs everything that follows: accelerating the linear 90% by 100× yields at most $1/(0.1 + 0.9/100) \approx 9.2\times$ end-to-end. The nonlinear 10% and the conversions are not details; they are the asymptote.

**Step 2 — Partition by comparative advantage.**

| Task | Domain | Reason |
|------|--------|--------|
| Dense linear algebra (MVM, convolution, FFT) | Photonic | Passive interference computes; $O(N^2)$ MACs per pass; WDM parallelism |
| Fan-out / broadcast / interconnect | Photonic | Loss-dominated, not $CV^2$-dominated; distance-independent on chip |
| Nonlinear activation | Electronic | A transistor is a better nonlinearity than any practical optical effect at fJ energies |
| Accumulation at precision | Electronic (digital) | Analog summation accumulates noise; digital adders are exact |
| Weight and activation storage | Electronic | No optical SRAM/DRAM exists (Section 25.3.3) |
| Control flow, scheduling, calibration | Electronic | General-purpose logic |

**Step 3 — Close the loop across the stack.** Each layer's choices constrain the others, so iterate: the *algorithm* is retrained noise-aware and quantized to the precision the analog core can deliver (Section 25.2.2); the *architecture* chooses $N$, WDM channel count, and weight-update rate to amortize the 25.1.1 tax; the *circuits* co-optimize DAC resolution against the mesh's usable ENOB (an 8-bit DAC feeding a 5-ENOB optical path wastes energy exponentially); the *devices* trade $V_\pi L$, loss, and capacitance against the driver's supply voltage. A design that optimizes any single layer in isolation — the all-too-common "record modulator + off-the-shelf everything else" — lands outside the narrow regime where photonics wins.

**The canonical partition** for one neural-network layer, used by essentially every serious system design:

```
weights (static, electronic memory) ──► weight DACs ──► photonic N×N core
activations ──► input DACs ──► modulators ──►      (optical MVM, ~10s of ps)
photodetectors ──► TIAs ──► ADCs ──► digital: accumulate partial sums,
bias + batch-norm (folded), ReLU/softmax, requantize ──► next layer
```

Photonics performs the multiply and the physical summation within one pass; electronics owns everything stateful, nonlinear, or precise. Variants move the boundary — detector nonlinearity as a free activation function, analog electronic accumulation to skip an ADC stage, optical fan-out of shared activations to multiple cores — but the partition principle is invariant.

## What Co-Design Buys: A Before/After Sketch

Consider the 64×64 accelerator whose interface tax we computed as ~5 pJ per sample pair. Naive design (2.5D wirebond, 8-bit converters everywhere, weights updated per layer at kHz rates, thermo-optic hold): the budget of Section 25.2.1 will show conversion plus static tuning dominating at >85% of wall-plug power. Co-designed version: 3D-stacked drivers (interface tax ×0.5), 6-bit input path matched to the measured optical ENOB (ADC energy ×0.25), noise-aware retraining to hold accuracy at 6 bits (algorithm), PCM or MEMS weight hold (static power ~0), and batch scheduling that keeps each programmed matrix resident for thousands of vectors (weight DACs amortized away). Same optics; roughly 4–6× lower energy per MAC. None of the gain came from photonic devices — which is precisely the point of this subsection.

---

## References

[1] Al-Qadasi, M.A., Chrostowski, L., Shastri, B.J., & Shekhar, S. (2022). "Scaling up silicon photonic-based accelerators: challenges and opportunities." *APL Photonics*, 7(2), 020902. [System-level co-design analysis across DACs, ADCs, lasers, tuning, and array size; the quantitative backbone of this subsection.]

[2] Shastri, B.J., Tait, A.N., Ferreira de Lima, T., Pernice, W.H.P., Bhaskaran, H., Wright, C.D., & Prucnal, P.R. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15(2), 102–114. [Review framing the electronic-photonic partition for AI workloads.]

[3] Miller, D.A.B. (2017). "Attojoule optoelectronics for low-energy information processing and communications." *Journal of Lightwave Technology*, 35(3), 346–396. [Quantifies why interface capacitance dominates optoelectronic energy and what integration must achieve.]

[4] Bogaerts, W., Pérez, D., Capmany, J., Miller, D.A.B., Poon, J., Englund, D., Morichetti, F., & Melloni, A. (2020). "Programmable photonic circuits." *Nature*, 586, 207–216. [Programmability, calibration, and control-plane requirements of large photonic meshes — the electronic burden co-design must budget for.]

[5] Sze, V., Chen, Y.-H., Yang, T.-J., & Emer, J.S. (2017). "Efficient processing of deep neural networks: a tutorial and survey." *Proceedings of the IEEE*, 105(12), 2295–2329. [The electronic-accelerator dataflow taxonomy — weight-stationary, output-stationary — that photonic architectures inherit.]

[6] Ramey, C. (2020). "Silicon photonics for artificial intelligence acceleration." *IEEE Hot Chips 32 Symposium*. [Industrial description of a 3D-integrated photonic tensor core with its electronic control stack; a concrete instance of the integration choices discussed here.]

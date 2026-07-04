# Subsection 12.3.3: Commercial Architectures

## Orientation

Within two years of the Shen et al. paper, its lead authors had founded competing companies: Nicholas Harris started **Lightmatter** and Yichen Shen started **Lightelligence**, both aiming to commercialize MZI-mesh matrix processors for AI inference. A decade on, these companies — together with a broader ecosystem (Luminous Computing, Salience Labs, iPronics, Celestial AI) — constitute a natural experiment: what happens when the physics of Chapter 12 meets the economics of the AI accelerator market? This subsection describes the announced architectures, teaches you how to read an analog "TOPS" specification critically, and draws the sober lesson from the industry's partial pivot toward optical *interconnect* rather than optical *compute*.

---

## 12.3.3.1 How to Count Photonic TOPS

A photonic matrix unit with $N$ input modes and $N$ output modes, accepting new input vectors at modulation rate $f_{\text{mod}}$, performs

$$R = 2N^2 f_{\text{mod}} \quad \text{operations per second}$$

(the factor 2 counts a multiply and an add per MAC, following digital-accelerator convention). This number grows quadratically with mesh size and is easily made spectacular: a $64 \times 64$ mesh clocked at 1 GHz delivers $2 \times 4096 \times 10^9 = 8.2$ TOPS from a few mm$^2$ of silicon; at 10 GHz modulation, 82 TOPS.

Three caveats convert marketing arithmetic into engineering truth:

1. **Precision qualifier.** Digital TOPS are quoted at a specified word length (INT8, FP16). Analog photonic operations carry an effective precision set by the noise and error analysis of Sections 12.1–12.2, typically 4–8 ENOB, and precision *degrades* as $f_{\text{mod}}$ rises (fewer photons per symbol — the shot-noise tradeoff of Section 12.1). A photonic TOPS figure without a stated ENOB and optical power is not comparable to a digital one.
2. **Utilization.** The $N^2$ scaling is realized only when the workload offers matrices at least as large as the mesh, batched to keep the modulators busy. Layers smaller than the mesh strand hardware; layers larger must be tiled, with electronic accumulation between tiles.
3. **Conversion overhead.** Every input element must pass through a DAC and modulator; every output through a photodetector, TIA, and ADC. At $\sim$1 pJ per high-speed conversion, an $N = 64$ tile amortizes its $2 \times 64$ conversions over $64^2$ MACs — about 0.03 pJ/MAC of overhead — while an $N = 8$ tile pays $8\times$ more per MAC. Conversion overhead is the reason photonic accelerators *must* be large to win, which is also exactly the regime where phase error control is hardest. This tension is the central engineering dilemma of the field.

---

## 12.3.3.2 Lightmatter: Mars, Envise, and Passage

Lightmatter (founded 2017, MIT lineage) disclosed its **Mars** device at Hot Chips 32 in 2020: a 3D-integrated package combining a photonic die — an MZI-mesh matrix core in the tens-of-modes class — with a CMOS die stacked on top containing the DACs, ADCs, SRAM, and control logic. The 3D stacking matters: analog photonics is only as good as the electronic plumbing around it, and vertical integration minimizes the capacitance (hence energy) of every DAC-to-modulator connection. Weights are held on analog phase shifters (updated at kHz–MHz rates), while activations stream through at GHz rates — a *weight-stationary* dataflow, in accelerator taxonomy.

**Envise** is the productized version: a server blade combining photonic tensor cores with electronic pre/post-processing, targeted at transformer and CNN inference. Company materials have claimed multi-fold throughput-per-watt advantages over contemporary GPUs on BERT-class inference; peer-reviewed, independently audited benchmarks have not been published, and the honest summary as of this writing is that the claims are plausible in the weight-stationary, batch-rich regime but unverified publicly.

The company's largest commercial traction, however, came from **Passage** — a wafer-scale programmable photonic *interconnect* that routes light between electronic chiplets, addressing the data-movement wall of Chapter 10 rather than the arithmetic itself. The market signal is worth registering: the first venture-scale returns in commercial photonics-for-AI came from moving bits, not from multiplying them.

## 12.3.3.3 Lightelligence: Comet and PACE

Lightelligence (founded 2017) demonstrated **PACE** (Photonic Arithmetic Computing Engine) in 2021: an integrated photonic accelerator with roughly ten thousand photonic devices co-packaged with driving electronics, executing recurrent matrix-vector iterations for combinatorial optimization (Ising/Max-Cut heuristics) rather than neural network inference. The choice of workload is astute: an Ising solver iterates $\mathbf{x}_{t+1} = f(W\mathbf{x}_t)$ with a *fixed* matrix $W$ thousands of times, so the slow weight-update problem disappears and the picosecond matrix-multiply latency compounds over iterations. Against this recurrent workload the company reported orders-of-magnitude latency advantage over a GPU executing the same algorithm — a fair comparison only for algorithms genuinely bound by low-latency small-matrix recurrence. The academic ancestor of this line of work is the recurrent photonic Ising machine of Prabhu et al. (2020), discussed again in Unit X.

## 12.3.3.4 The Rest of the Field, Briefly

- **Luminous Computing** pursued laser-neuron-based spiking architectures (Princeton lineage; Chapter 16) before pivoting toward interconnect and subsequently winding down — an early data point on the difficulty of the full-stack approach.
- **Salience Labs** (Oxford/Münster lineage) pursues WDM crossbars with phase-change-material weights, the Feldmann architecture of Subsection 12.4.3.
- **iPronics** (Valencia, Capmany group) sells general-purpose field-programmable photonic meshes — the "FPGA of photonics" — for signal processing rather than deep learning.
- **Celestial AI**, like Lightmatter's Passage, targets the photonic-fabric interconnect market.

---

## 12.3.3.5 What the Market Is Telling Us

Reading the sector as a physicist, three conclusions stand out. First, the *matrix engine itself works*: multiple independent teams have silicon that multiplies at useful fidelity. Second, the hard problems are at the boundaries — DAC/ADC energy, weight-update bandwidth, calibration at scale, and the software stack that hides analog error from the ML user — precisely the topics of Sections 12.2.4 and 13.3. Third, wherever photonics competes against a *physical* limitation of electronics (distance-bandwidth-energy in interconnect; latency in recurrent analog iteration), adoption is fast; wherever it competes against Moore's-law-hardened digital multipliers on their home turf (dense batched GEMM at INT8), the bar is brutally high. The architectures that survive will be the ones designed around that asymmetry.

---

## References

[1] Ramey, C. (2020). "Silicon photonics for artificial intelligence acceleration." *IEEE Hot Chips 32 Symposium*. [Lightmatter's Mars disclosure: 3D-stacked photonic core with CMOS control.]

[2] Shen, Y., et al. (2017). "Deep learning with coherent nanophotonic circuits." *Nature Photonics*, 11, 441–446. [The academic ancestor of both Lightmatter's and Lightelligence's architectures.]

[3] Prabhu, M., et al. (2020). "Accelerating recurrent Ising machines in photonic integrated circuits." *Optica*, 7(5), 551–558. [Recurrent photonic matrix iteration for combinatorial optimization; the research basis for the PACE workload.]

[4] Demirkiran, C., et al. (2023). "An electro-photonic system for accelerating deep neural networks." *ACM Journal on Emerging Technologies in Computing Systems*, 19(4). [ADEPT: a careful architectural study of what a full electro-photonic accelerator system costs, including all conversion overheads — an antidote to bare-TOPS marketing.]

[5] Shastri, B.J., Tait, A.N., Ferreira de Lima, T., Pernice, W.H.P., Bhaskaran, H., Wright, C.D., & Prucnal, P.R. (2021). "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15, 102–114. [Survey of the architecture landscape spanning the academic and commercial efforts named here.]

# Chapter 27: Exercises

*The exercises for this chapter are weighted toward analysis and discussion, as befits a survey of open research. The mathematical problems use order-of-magnitude figures to expose where a frontier's real bottleneck sits; the discussion problems ask you to separate demonstrated physics from computing claims; and the lab problems ask you to practice the evaluative discipline the chapter preaches. Where you can find better numbers than those given, use them and say so.*

---

## Mathematical / Estimation Exercises

**M27.1 — The Measurement-Feedback Cost of a Coherent Ising Machine**

A measurement-feedback CIM (Section 27.1.2) computes the coupling vector $\mathbf{J}\boldsymbol{\sigma}$ once per round trip, where $\mathbf{J}$ is dense and $N$ is the number of spins.

(a) Show that the feedback step costs $O(N^2)$ multiply-accumulates per round trip. For the 100,000-spin regime [Honjo et al., *Science Advances*, 2021], how many MACs is that per round trip?

(b) A time-multiplexed machine circulates its pulses in a fiber loop of length $L$ (refractive index $n \approx 1.45$); the round-trip time is $t_{\rm rt} = nL/c$. For $L = 1$ km, compute $t_{\rm rt}$. Over a run of $R = 1000$ round trips, what is the total wall-clock time set by the loop?

(c) Suppose the FPGA performing the feedback sustains $10^{12}$ MAC/s. Compare the electronic feedback time per round trip against $t_{\rm rt}$ from part (b). Which dominates, and what does your answer say about calling this machine "optical"?

**M27.2 — Microcomb Line Count versus Laser-Bank Power**

A WDM tensor core (Section 27.2.2) requires $K$ mutually coherent wavelength channels.

(a) Option A supplies them with $K$ individually packaged DFB lasers at wall-plug power $P_L = 250$ mW each. Option B uses one pump laser at $P_p = 2$ W driving a soliton microcomb of pump-to-comb conversion efficiency $\eta = 2\%$. Write the total electrical power for each option as a function of $K$.

(b) Above what channel count $K^*$ does the microcomb consume less wall-plug power than the laser bank, on this simple accounting?

(c) List three overheads this estimate omits (consider soliton initiation and stabilization, per-line power available at the modulators, thermal control, and comb flatness/equalization), and state for each whether it helps or hurts the comb's case.

**M27.3 — Space-Bandwidth Product and Camera-Limited Throughput**

A free-space processor (Section 27.3) uses an SLM of $N = 1000 \times 1000$ pixels and a detector array read out at $F$ frames per second.

(a) Treating one frame as a single matrix-vector product of dimension $10^6$, how many MACs are performed per frame?

(b) The effective throughput is (MACs per frame) $\times F$. Evaluate it for $F = 100$ fps and for $F = 10{,}000$ fps.

(c) Compare both figures against a GPU sustaining $\sim 10^{14}$–$10^{15}$ MAC/s. What must $F$ become for the optical processor to be competitive on raw throughput, and which physical component sets the ceiling — the light or the readout?

**M27.4 — Graphene Modulator Energy per Bit**

Model the graphene electro-absorption modulator (Section 27.5.1) as a parallel-plate capacitor of active area $A = 10\ \mu\text{m} \times 0.5\ \mu\text{m}$ with a gate-dielectric capacitance per unit area $c_{\rm ox} = 5\ \text{fF}/\mu\text{m}^2$, driven by a voltage swing $V = 2$ V.

(a) Compute the capacitance $C = c_{\rm ox} A$ and the switching energy $\tfrac{1}{2}CV^2$ per bit.

(b) At a line rate of $B = 50$ Gb/s, what dynamic power does this represent?

(c) Compare the per-bit energy to the femtojoule-scale interface budget flagged in Chapter 25. Then estimate qualitatively how a metal-graphene contact resistance $R_c$ degrades the achievable speed (through the $R_c C$ time constant) and the energy — and explain why contact resistance, not the capacitor, is often the real limiter.

---

## Conceptual / Discussion Exercises

**C27.1 — Is a Coherent-Ising-Machine Advantage Established?**

Using Section 27.1.3, the standard survey [Mohseni, McMahon & Byrnes, *Nature Reviews Physics*, 2022], and the simulated-bifurcation result [Goto et al., *Science Advances*, 2019]:

(a) State precisely what "computational advantage" would have to mean for a photonic Ising machine — over what baseline, on what instance class, under what metric.

(b) List the baselines a credible claim must beat (name at least three, spanning heuristics and exact solvers).

(c) Describe a single experiment whose outcome you would accept as settling the question, and say what result would change your current view.

**C27.2 — Physics Result versus Computing Claim: Topological Devices**

For each of (i) the topological insulator laser [Bandres et al., *Science*, 2018] and (ii) a "disorder-immune" topological delay line:

(a) Separate the demonstrated physics from the engineering or computing claim being made on its behalf.

(b) Identify the *competent trivial baseline* — the well-engineered non-topological device the result must be compared against.

(c) State explicitly which disorder the topological protection suppresses and which it does not (Section 27.4; Concept 8). Is the advantage established, plausible, or open?

**C27.3 — Fixed D²NN versus Reconfigurable SLM Processor**

(a) Contrast a fixed diffractive network [Lin et al., *Science*, 2018] with a reconfigurable SLM-based processor [Zhou et al., *Nature Photonics*, 2021] along four axes: reconfiguration energy, latency, per-inference energy, and manufacturability.

(b) Map the comparison onto the ASIC-versus-FPGA trade in digital hardware. Where does the analogy hold and where does it break?

(c) Name one deployed workload for which each choice is clearly the right one, and justify.

**C27.4 — Why 2D Materials Attack Interfaces, Not Arithmetic**

(a) Explain why graphene and TMD devices (Section 27.5) target modulation, detection, and emission rather than the matrix multiply itself.

(b) Connect this to the interface-dominated energy budget of Chapter 25: which specific line items in a photonic accelerator's budget would a good graphene modulator or detector improve?

(c) What would have to become true — physically and in manufacturing — for a 2D material to perform the arithmetic rather than serve the interface? Argue whether that is a promising direction or a category error.

**C27.5 — Synthetic Dimensions: Computing, Simulation, or Spectroscopy?**

(a) Explain how a modulated ring resonator realizes a synthetic frequency lattice [Yuan et al., *Optica*, 2018; Dutt et al., *Science*, 2020], and what plays the role of a gauge field.

(b) Name two lattice phenomena that are easier to study in a synthetic dimension than in real space.

(c) Classify the activity: is a synthetic-dimension experiment best described as computing, as quantum/physics simulation, or as engineered spectroscopy? Defend your classification against the other two.

---

## Lab / Research Exercises

**L27.1 — Design a Matched-Baseline Benchmark**

For a photonic Ising machine of your choice, write a one-page benchmark protocol (Section 27.1.3). Specify: the instance class and sizes; the classical baselines (at minimum simulated annealing, simulated bifurcation, and a commercial exact solver such as a branch-and-cut MIP); the time-to-target-accuracy metric; and — crucially — how you will account for the electronic measurement-feedback cost in the machine's own timing. Predict the outcome and state one result that would genuinely surprise you.

**L27.2 — Reproduce a Synthetic-Dimension Band Structure**

In simulation, build a driven ring resonator with nearest-neighbor coupling between adjacent frequency modes (Section 27.2.3). Compute and plot the band structure of the resulting synthetic lattice as a function of the modulation phase. Then introduce an effective gauge field by making the coupling phase position-dependent, and show its effect on the bands. Compare your result qualitatively with [Yuan et al., *Optica*, 2018].

**L27.3 — Audit a Frontier Paper's End-to-End Energy Claim**

Choose one recent photonic-computing result — a microcomb accelerator [Xu et al., *Nature*, 2021], a photonic tensor core [Feldmann et al., *Nature*, 2021], or a sub-photon-per-multiplication network [Wang et al., *Nature Communications*, 2022]. Reconstruct the full end-to-end energy budget in the style of Chapter 25: pump and source lasers, modulators, detectors, DAC/ADC conversion, and digital control and calibration. Identify precisely which of these the headline efficiency number includes and which it excludes, and write a one-page memo distinguishing what is demonstrated, at what scope, against what baseline.

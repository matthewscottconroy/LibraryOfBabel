# Section 14.4: Physical Limits and Trade-offs

## What This Section Is About

Everything that makes a diffractive deep neural network attractive — computation at the speed of light, in a passive slab of engineered matter, drawing essentially no power in the inference path — flows from a single physical choice: let free-space diffraction do the arithmetic and let a photodetector read the answer as intensity. That same choice writes the network's limits. A passive, phase-only, intensity-read optical system cannot be scaled, aligned, or signed with impunity, and this section makes those constraints quantitative. The organizing point is that a D2NN's power and its limitations are not separate lists — they are two readings of the same physics.

**14.4.1: Scalability, Depth, and Width** — how many neurons a layer can hold is set by the space-bandwidth product, itself capped by diffraction at roughly $(2A\cdot\mathrm{NA}/\lambda)^2$ degrees of freedom per aperture. Width buys throughput and depth buys representational reach, but with the crucial caveat that a stack of *linear* diffractive layers composes to a single linear operator. Footprint grows as layers × spacing, and total optical power scales with input intensity × aperture area.

**14.4.2: Noise and Fabrication Sensitivity** — a D2NN is trained in a computer and then frozen into matter, so every misalignment, thickness error, phase-quantization step, and detector-noise electron widens the sim-to-real gap. Tolerances scale with wavelength, which is why terahertz demonstrations succeed with millimeter features and modest micron alignment, while visible and metasurface designs demand nanometer control.

**14.4.3: The Non-Negativity Constraint** — a detector measures $|U|^2 \ge 0$, discarding sign and phase. We examine why a passive D2NN nonetheless computes at all, and how class-specific differential detection and complex-field readout restore the missing sign axis.

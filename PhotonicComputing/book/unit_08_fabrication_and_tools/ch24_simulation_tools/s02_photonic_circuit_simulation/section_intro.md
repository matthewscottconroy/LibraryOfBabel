# Section 24.2: Photonic Circuit Simulation

You cannot simulate a photonic processor with Maxwell's equations. A 64×64 Mach-Zehnder mesh contains thousands of directional couplers, phase shifters, and waveguide segments, and a rigorous FDTD run of even one microring already fills a GPU. The gap between what rigorous solvers can afford and what a system contains is many orders of magnitude, and the field bridges it exactly as electronic design automation did decades ago: by abstraction. Each component is simulated rigorously *once*, its behavior distilled into a compact **scattering-parameter model**, and thousands of those models composed in a fast circuit simulator that never touches a field again.

This section builds that layer from the bottom. First, the mathematics of composition: the scattering matrix that describes a component as a relation between incoming and outgoing wave amplitudes, and the transfer-matrix technique that cascades components by matrix multiplication — with the microring resonator worked through analytically as the canonical example. Second, the circuit simulators themselves — the photonic analogues of SPICE — that solve arbitrary interconnections of compact models in the frequency and time domains, scaling to the thousands of components a real processor needs. Third, the full-stack methodology that ties a rigorous field simulation, a circuit model, the driving electronics, the thermal environment, and the physical layout into one verifiable flow ending in a tape-out.

The unifying idea is **compact-model thinking**: the discipline of leaving behind, from every expensive simulation, a cheap and reusable model. It is what makes system-scale photonic design possible at all.

- **24.2.1** — The Transfer Matrix Method: scattering parameters, cascading, and the ring resonator
- **24.2.2** — Photonic SPICE: circuit simulators for thousands of components
- **24.2.3** — The Full-Stack Simulation Flow: from field solver to co-designed system

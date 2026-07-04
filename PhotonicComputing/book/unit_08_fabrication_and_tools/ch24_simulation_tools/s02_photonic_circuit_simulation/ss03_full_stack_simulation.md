# 24.2.3 The Full-Stack Simulation Flow

Circuit simulation predicts what a photonic processor does *if* every component matches its compact model, the electronics are ideal, and the chip is isothermal. None of those hold. A design is only trustworthy when the omitted physics — the driving electronics, the thermal environment, and the statistical spread of a real foundry process — is put back. The full-stack flow is the methodology that does this, and it is the connective tissue of the entire unit: it consumes the fabrication statistics of Chapter 23 and the solvers of this chapter and produces a single, verifiable, manufacturable design.

## The Vertical Flow

The flow runs top to bottom and then closes a loop.

1. **Component simulation.** Each distinct building block is characterized rigorously once — a directional coupler and grating coupler by FDTD (Section 24.1.1), a phase shifter by a mode solve plus a thermal FEM analysis (Section 24.1.2), a taper by EME (Section 24.1.3).
2. **Compact-model extraction.** Each result is reduced to a frequency-dependent scattering matrix, tagged with the process parameters (width, thickness, temperature) it depends on.
3. **Circuit assembly.** The models are wired into a netlist and solved at system scale by a photonic circuit simulator (Section 24.2.2).
4. **Co-simulation.** Electronics and heat are coupled in.
5. **Statistical analysis.** The whole thing is run many times over the foundry's variability distribution.
6. **Layout and verification.** The verified circuit is turned into a DRC-clean GDSII file and checked against the schematic before tape-out.

## Electronic–Photonic Co-Simulation

A photonic modulator is useless without a driver, and a photodetector without a transimpedance amplifier; the interesting behavior — settling time, bit depth, power, jitter — lives at the boundary. **Electronic–photonic design automation (EPDA)** addresses this by exporting the photonic compact models into a form an electronic simulator understands, typically **Verilog-A**, so that the optics and the CMOS are solved together in an industry SPICE engine such as Cadence Spectre. A modulator's optical response is then driven by the *actual* nonlinear output of its driver; a receiver's sensitivity is evaluated against the *actual* TIA noise. For a photonic computing mesh, co-simulation is how the digital-to-analog converters that set the phase shifters, the control loop that calibrates them, and the optical mesh are verified as one mixed-signal system rather than three optimistic abstractions.

## Thermal and Statistical Coupling

Two further couplings are re-injected. **Thermal co-simulation** takes the dissipated power of every heater, doped junction, and absorbing region as sources for an FEM heat solve, obtains the on-chip temperature map, converts it to local index shifts through $dn/dT$, and detunes the affected components in the circuit model — capturing the thermal crosstalk that makes a heater intended for one MZI perturb its neighbors. **Statistical (Monte Carlo) analysis** draws correlated width, thickness, and temperature variations from the PDK's statistical models — using the spatially correlated fields motivated in Chapter 23, not naïve independent draws — evaluates the circuit for each realization, and builds the distribution of a system metric. The output is **parametric yield**: the fraction of chips whose performance stays within spec once fabrication scatter is accounted for.

## Worked Example: Full-Stack Sign-Off of a 64×64 Mesh

Bring the whole flow to bear on a 64×64 Clements mesh — 2016 MZIs and 4032 thermo-optic phase shifters.

**Component library.** The mesh reduces to a handful of distinct parts: a 3-dB directional coupler, a thermo-optic phase shifter, a waveguide crossing, and an edge or grating coupler. Characterizing them rigorously is on the order of ten FDTD/FEM runs — a few GPU-hours, done once.

**Circuit + statistics.** Assembling 2016 MZIs from those models gives a sparse system of $\sim10^4$ ports, solved in seconds per wavelength. A Monte Carlo campaign of $N=1000$ realizations — each drawing a correlated $(\delta w,\delta t)$ field with $\sigma_w=2$ nm, $\sigma_t=1.5$ nm and a millimeter correlation length, then evaluating the mesh's implemented unitary — runs in tens of minutes. Uncalibrated, the realized unitaries are essentially random, exactly as the 0.6 rad-per-nm-per-100 μm phase sensitivity of Chapter 23 predicts. With per-MZI phase calibration applied (the calibrate-then-compensate doctrine of Chapter 12), the fidelity recovers above 0.99 — *at a static tuning power* that the FSR/2 tuning-budget argument of Chapter 23 puts in the watts for a mesh this size. That number is not a footnote: it is the quantity Chapter 25 weighs against an electronic accelerator, and the full-stack simulation is where it is first computed honestly.

**Co-design and sign-off.** Exporting the phase-shifter-plus-heater model to Verilog-A lets the DAC array and calibration controller be co-simulated with the mesh, verifying that the control electronics can reach and hold the required phase precision and settling time. Only then does the design proceed to layout.

## Layout, Verification, and Tape-Out

The last stage reconciles the simulated circuit with a physical mask. **Schematic-driven layout** generates the mask from the verified netlist so the two cannot drift apart. In the open flow, **gdsfactory** builds the mesh layout parametrically in Python and emits both the GDSII and a netlist that **SAX** re-simulates with the *as-laid-out* waveguide lengths back-annotated — catching routing-induced phase errors a schematic never sees. **KLayout** runs the foundry **design-rule check** (minimum width, gap, bend radius, density) and serves as the mask editor, while **layout-versus-schematic (LVS)** confirms that what was drawn is what was simulated. A DRC-clean, LVS-verified GDSII is the deliverable that Chapter 23's MPW run consumes.

This closed loop — simulate, compose, co-design, randomize, lay out, verify — is the professional practice of photonic design. Its purpose is singular: to spend every dollar of insight *before* the four-to-nine-month foundry cycle begins, so that the dies that come back work, and work as predicted.

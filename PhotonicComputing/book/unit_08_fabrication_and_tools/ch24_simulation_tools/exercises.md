# Chapter 24: Exercises

---

## Mathematical Exercises

**M24.1 — The Courant Limit, Derived and Applied**

(a) Consider the 1D FDTD update for $E_x$ and $H_y$ on a grid of spacing $\Delta x$ with time step $\Delta t$. By substituting a numerical plane-wave trial solution $e^{i(\tilde{k} m\Delta x - \omega n\Delta t)}$ into the leapfrog update equations, derive the numerical dispersion relation and show that real $\tilde k$ requires $\Delta t \le \Delta x/c$. Interpret this as "information cannot cross a cell in one step."

(b) Generalize to a 3D grid with cubic cells and show $\Delta t \le \Delta x/(c\sqrt{3})$. Where does the $\sqrt{3}$ come from?

(c) A cubic-cell 3D simulation uses $\Delta x = 20$ nm. Compute the largest stable $\Delta t$. How many time steps are needed to simulate $50$ ps of physical time?

**M24.2 — FDTD Resource Budget**

A directional-coupler simulation spans a domain of $20\times 6\times 3\ \mu\text{m}$ in a silicon-on-insulator stack, meshed at $\Delta x = 25$ nm (cubic cells).

(a) Compute the number of Yee cells. Estimate the field memory (6 single-precision components per cell) and a realistic total footprint (take $\sim$100 bytes/cell).

(b) Verify the grid resolves the in-silicon wavelength ($n_\text{Si}=3.47$, $\lambda_0=1550$ nm) to at least 15 points per wavelength.

(c) You must resolve the transmission spectrum over 1500–1600 nm. Explain why a *single* pulsed FDTD run suffices, and describe the source and monitors you would use. Contrast the run count with a frequency-domain solver.

**M24.3 — Ring Resonator from the Transfer Matrix**

An all-pass silicon microring has radius $R = 8\ \mu\text{m}$, group index $n_g = 4.2$, self-coupling $r$, and round-trip amplitude transmission $a$, at $\lambda = 1550$ nm.

(a) Compute the free spectral range from $\text{FSR} = \lambda^2/(n_g L)$, $L = 2\pi R$.

(b) Starting from the through-port transmission $T = (a^2 - 2ra\cos\theta + r^2)/(1 - 2ra\cos\theta + r^2 a^2)$, show that resonances occur at $\theta = 2\pi m$ and that critical coupling ($T_\text{min}=0$) requires $a = r$.

(c) Using the finesse $\mathcal{F} = \pi\sqrt{ra}/(1-ra)$, find the $ra$ product needed for a loaded $Q$ of $20{,}000$. Report the corresponding FWHM in nm and pm.

(d) A width error $\delta w = 2$ nm perturbs $n_\text{eff}$ with $\partial n_\text{eff}/\partial w = 1.5\times10^{-3}\,\text{nm}^{-1}$. Compute the resonance shift $\delta\lambda = \lambda\,\delta n_\text{eff}/n_g$ and express it in linewidths for the $Q$ of part (c).

**M24.4 — Adjoint Gradient of a Power Splitter**

A design region of $N$ permittivity pixels is optimized so that an input mode splits 50/50 into two output waveguides. Define the objective $F = -\big(|t_1|^2 - \tfrac12\big)^2 - \big(|t_2|^2 - \tfrac12\big)^2$, where $t_{1,2}$ are the modal transmission amplitudes into the two outputs.

(a) Explain why finite-difference estimation of $\nabla F$ costs $N+1$ simulations, while the adjoint method costs 2, independent of $N$.

(b) Describe the adjoint source: where is it placed, and what is its relation to $\partial F/\partial \mathbf{E}$?

(c) Write the pixel gradient in the schematic form $\partial F/\partial\varepsilon_i \propto \text{Re}\{\mathbf{E}_\text{adj}(\mathbf r_i)\cdot\mathbf{E}_\text{fwd}(\mathbf r_i)\}$ and interpret it physically.

(d) For $N = 4\times10^4$ pixels, an objective at 3 wavelengths, and 3 robustness corners, how many simulations does one optimization iteration cost by adjoint? How many would finite differences require?

**M24.5 — EME versus FDTD for a Taper**

An adiabatic taper widens from 180 nm to 450 nm over a length $L$.

(a) In EME the taper is sliced into $S$ sections, each keeping $M$ local modes. Argue that the compute cost scales as $\sim S\,M^2$ (interface overlaps) and is essentially independent of $L$. Why does making the taper twice as long barely change the EME cost?

(b) In FDTD, express how the cell count *and* the Courant-limited step count scale with $L$, and conclude how the total FDTD cost scales with $L$.

(c) You need to sweep $L$ over 20 values to find the shortest adiabatic taper. Quantify (order of magnitude) why EME is the right tool. Under what circumstance (name one device feature) would you be forced back to FDTD?

---

## Conceptual Exercises

**C24.1 — Pick the Solver**

For each task, choose FDTD, FEM/FDE mode solver, EME, BPM, or a circuit simulator, and defend the choice in one sentence: (a) the effective and group index of a 450 nm strip waveguide; (b) the back-reflection of an abrupt waveguide junction; (c) the insertion loss of a 300 μm spot-size-converter taper; (d) the transmission spectrum of a 64-ring CROW filter given each ring's compact model; (e) the temperature map and tuning efficiency of a TiN heater over a waveguide; (f) the field profile in a wavelength-scale inverse-designed demultiplexer; (g) the propagation through a 2 mm titanium-diffused lithium-niobate modulator.

**C24.2 — Why Inverse Designs Fail on Silicon**

(a) Explain, with reference to the density method, why an unconstrained pixel optimization tends to produce checkerboards and single-pixel features, and how filtering plus projection removes them while enforcing a minimum feature size.

(b) Define the robust (eroded/nominal/dilated) formulation and connect it quantitatively to the $\pm$few-nanometer process bias of Chapter 23.

(c) A colleague trains a neural network to map "target spectrum → device geometry" with an MSE loss on geometry and finds it produces devices that work poorly. Diagnose the failure using the non-uniqueness of inverse problems, and explain how a tandem network fixes it.

**C24.3 — The Modeling Hierarchy**

(a) State the compact-model methodology (rigorous solve once → S-parameters → circuit composition) and estimate, for a 32×32 MZI mesh, how many *distinct* component simulations are actually required.

(b) Give two concrete physical effects that a circuit simulator built from isolated compact models will miss, and name the full-stack technique that re-injects each.

(c) Argue why a differentiable circuit simulator (photontorch, SAX) is qualitatively more useful for photonic computing than a non-differentiable one.

---

## Lab / Programming Exercises

**L24.1 — Meep FDTD of Waveguide, Coupler, and Ring**

Using the Meep Python API: (a) compute the fundamental TE mode profile and $n_\text{eff}$ of a 450×220 nm silicon waveguide with the mode solver, and compare with an FDE/COMSOL result if available; (b) simulate a directional coupler and extract the power-coupling ratio versus gap for gaps 150–300 nm; (c) simulate an add-drop ring ($R=5\ \mu$m) and extract its transmission spectrum, FSR, and loaded $Q$ using `harminv`. Verify the FSR against $\lambda^2/(n_g L)$ and report the run time and memory actually used.

**L24.2 — Adjoint Optimization of a Y-Junction**

Using `meep.adjoint` or ceviche, inverse-design a compact 50/50 power splitter in a $2.5\times2.5\ \mu$m design region. (a) Implement the objective of M24.4; (b) run topology optimization with density filtering (radius $\approx$90 nm) and $\beta$-annealed projection; (c) impose the eroded/nominal/dilated robust formulation at $\pm10$ nm; (d) report transmission, balance, and bandwidth, and compare footprint and performance with a conventional adiabatic Y-junction. Confirm the final geometry is binary and passes a minimum-feature check.

**L24.3 — MZI Mesh Simulation and Programming**

Build a 4×4 Clements MZI mesh in a transfer-matrix simulator (or SAX/photontorch). (a) Given a target 4×4 unitary, solve for the phase-shifter settings and verify the mesh implements it; (b) inject independent Gaussian phase errors ($\sigma_\phi = 0.05, 0.1, 0.2$ rad) and plot the implemented-unitary fidelity distribution over 1000 trials; (c) if using a differentiable simulator, *retrain* the phase settings by gradient descent to compensate a fixed random error pattern and report the recovered fidelity; (d) simulate one matrix–vector multiply and relate the fidelity to inference accuracy for a small neural-network layer (link to Chapter 12).

**L24.4 — Deep-Learning Surrogate and Layout Hand-off**

(a) Generate a dataset of $\sim10^4$ $(w, h)\to(n_\text{eff}, n_g)$ pairs with a mode solver; train a small neural network surrogate and report its accuracy on a held-out set. (b) Measure the inference speedup over a direct mode solve and compute the break-even query count (data-cost ledger of Section 24.3.3). (c) Use the surrogate to scan a $200\times200$ grid of $(w,h)$ for a target $n_g$, then (d) instantiate the chosen waveguide in **gdsfactory**, route it to a grating-coupler pair, run a **KLayout** DRC (min width 180 nm, min gap 200 nm, min bend radius 5 μm), and export a GDSII. Comment on how the simulated circuit and the drawn layout are kept consistent (schematic-driven layout / LVS).

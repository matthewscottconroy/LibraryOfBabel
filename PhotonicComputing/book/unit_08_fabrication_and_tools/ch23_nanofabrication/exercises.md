# Chapter 23: Exercises

---

## Mathematical Exercises

**M23.1 — Lithography Resolution and Tool Choice**

(a) Using the Rayleigh criterion $CD = k_1\lambda/NA$, compute the minimum feature size for: (i) 248 nm KrF with $NA = 0.6$, $k_1 = 0.5$; (ii) 193 nm dry ArF with $NA = 0.93$, $k_1 = 0.35$; (iii) 193 nm immersion with $NA = 1.35$, $k_1 = 0.3$; (iv) EUV at 13.5 nm with $NA = 0.33$, $k_1 = 0.4$.

(b) Compute the corresponding depths of focus, $DOF = k_2\lambda/NA^2$ with $k_2 = 0.5$. Which tools can tolerate 200 nm of wafer topography without CMP?

(c) A silicon photonic layout needs: 450 nm waveguides, 180 nm coupler gaps, and a photonic crystal with 90 nm holes. Assign the cheapest adequate lithography to each and justify.

**M23.2 — E-Beam Write Time**

A 100 keV e-beam tool writes with current $I$ at areal dose $D$; write time is $t = DA_{exposed}/I$ (ignore stage overhead).

(a) A 10 × 10 mm chip has a waveguide layer with 25% pattern density. With ZEP resist ($D = 250$ μC/cm²) at $I = 20$ nA, find the write time.

(b) The same chip's photonic-crystal region (1 mm², 30% density) needs HSQ at $D = 1500$ μC/cm² and $I = 1$ nA for resolution. Find its write time, and the total.

(c) A 200 mm wafer carries 200 such chips. Estimate the wafer write time in days and explain why e-beam is a prototyping tool, not a production tool — and why photomasks (written once, printed millions of times) change the economics.

**M23.3 — Sidewall Roughness and Loss**

A strip waveguide's scattering loss scales approximately as $\alpha \propto \sigma^2/d^3 \times E_{sidewall}^2$, with $\sigma$ the RMS sidewall roughness.

(a) A process improvement (resist reflow plus optimized etch chemistry) reduces $\sigma$ from 2.5 nm to 1.2 nm. If the waveguide loss was 2.8 dB/cm and is dominated by sidewall scattering, estimate the new loss.

(b) For a 3 cm total on-chip path, how many dB of link budget does the improvement recover? Convert this to the equivalent change in required laser power (in %) for fixed detector power.

(c) Widening the routing waveguide from 450 nm to 800 nm reduces the sidewall field intensity by roughly 10×. Estimate the routing loss in the improved process, and state the design rule this justifies (wide waveguides + tapers for long routes). What new hazard does the 800 nm width introduce? (Hint: how many TE modes does it support?)

**M23.4 — Fabrication Sensitivity of a Ring Weight Bank**

Use $\partial n_{eff}/\partial w = 1.5\times10^{-3}$ nm⁻¹, $\partial n_{eff}/\partial t = 5\times10^{-3}$ nm⁻¹, $n_g = 4.2$, $\lambda = 1550$ nm.

(a) Derive $\delta\lambda_{res} = (\lambda/n_g)\,\delta n_{eff}$ from the ring resonance condition, and compute the resonance shift per nm of width error and per nm of thickness error.

(b) A wafer exhibits independent Gaussian variations $\sigma_w = 2$ nm and $\sigma_t = 1.5$ nm. Compute $\sigma_\lambda$ for a ring's resonance. How many resonance linewidths is this for $Q = 15{,}000$?

(c) A 32-ring WDM weight bank targets a 100 GHz (0.8 nm) grid with FSR 18 nm. With tuning efficiency 0.25 nm/mW and tune-to-nearest-resonance strategy (mean correction FSR/2 without pre-bias), estimate the mean static tuning power for the bank. Recompute with undercut heaters (1.2 nm/mW).

(d) The same $\sigma_w$ applies to a 100 μm MZI arm. Compute the arm phase error $\sigma_\phi$ and compare with the $\sim 0.05$ rad phase accuracy needed for high-fidelity mesh operation (Chapter 12). What does this imply about calibration?

**M23.5 — Poisson Yield and Die-Size Economics**

(a) A photonic die has critical area 0.25 cm² and sees 4 critical layers with $D_0 = 0.08$ cm⁻² each. Compute the defect-limited yield.

(b) A 200 mm wafer (usable radius 95 mm, ignore edge exclusion subtleties) is diced into square dies of side $s$. Approximating dies per wafer as $N \approx \pi r^2/s^2 - 2\pi r/s$ (edge-loss correction), compute good dies per wafer for $s = 5, 10, 20$ mm with the yield model of (a) scaled to each area ($D_{total} = 0.32$ cm⁻²).

(c) A photonic matrix processor wants the largest monolithic mesh possible. Using (b), discuss the trade between mesh size $\propto s$ and good-die count, and connect to the chiplet/redundancy strategies of Section 23.2.3.

**M23.6 — Grating Coupler Design**

(a) From the grating equation $n_{eff} - n_c\sin\theta = \lambda/\Lambda$, find $\Lambda$ for $\lambda = 1310$ nm, $n_{eff} = 2.6$ (shallower etch, O-band mode), $\theta = 12°$, $n_c = 1.44$ (glued fiber, index-matched epoxy).

(b) Differentiate the grating equation to show $d\lambda/d\theta = -\Lambda\, n_c\cos\theta$ and evaluate it (nm per degree) at the design point. If the fiber array is placed with ±0.5° angular error, what wavelength shift results?

(c) The coupler's 1-dB lateral alignment tolerance is ±2.5 μm. If a pick-and-place machine has ±1 μm (3σ) accuracy, estimate the fraction of passive attachments landing within tolerance, and the expected worst-case insertion-loss penalty (assume a Gaussian overlap model $\eta(\delta) = e^{-\delta^2/w^2}$ with $w = 5.2$ μm).

---

## Conceptual Exercises

**C23.1 — Choose the Fabrication Route**

For each project, choose e-beam prototyping, passive DUV MPW, full active MPW, or dedicated run — and defend the choice with cost, turnaround, and risk arguments: (a) a PhD student's first inverse-designed 3-dB splitter; (b) an 8×8 thermo-optic MZI mesh with on-chip detectors for a neural-network demo; (c) a startup's 64×64 accelerator heading to customer sampling in 18 months; (d) a photonic crystal cavity requiring 60 nm holes with ±2 nm control.

**C23.2 — The III-V Integration Debate**

You are the integration architect for a photonic accelerator needing 16 distributed on-chip gain sites (inter-stage amplification) and one low-linewidth laser. Argue the choice among flip-chip, die-to-wafer bonding, micro-transfer printing, and monolithic QD growth, addressing: alignment and coupling loss; thermal paths through the BOX; known-good-die economics at 16 sites/chip; and technology maturity timelines. There is a defensible hybrid answer — find it.

**C23.3 — The Electrical Escape Problem**

A 64×64 Clements mesh has 2016 MZIs, each with two tuners (say 4032 DC lines plus grounds) and needs 64 RF input modulators and 64 RF output receivers. (a) Show that perimeter wire bonding at 100 μm pitch cannot escape this die. (b) Sketch a 2.5D/3D packaging solution, assigning each signal class (DC tuning, RF data, monitor photocurrents) to an interconnect technology with justification. (c) Identify which co-integration choice (driver ASIC location) most reduces total tuning-network power, and why.

**C23.4 — Why the PDK Says No**

Explain, using the physics of Sections 23.1–23.2, why a foundry PDK: (a) forbids waveguide-layer polygons below 150 nm even though the scanner "can" print 100 nm; (b) requires tiling fill but demands exclusion zones around rings; (c) guarantees only PDK components; (d) quotes ring resonance wavelength as a distribution, never a value. For each, name the process step responsible.

---

## Lab/Programming Exercises

**L23.1 — GDS Layout with gdsfactory (Python)**

Using gdsfactory: (a) generate a chip layout containing a grating-coupler loopback pair, three cutback spirals (1, 3, 6 cm), and a 5-ring resonator array with radii swept 5.00–5.04 μm in 10 nm steps; (b) auto-route all devices to a 127 μm-pitch grating coupler array; (c) run KLayout DRC with rules: min width 180 nm, min gap 200 nm, min bend radius 5 μm; (d) export GDSII and verify the ring polygons' vertex discretization contributes <0.1 nm RMS radius error.

**L23.2 — Process-Variation Monte Carlo**

Model a 2×2 directional coupler with coupling ratio $\kappa(w, g)$ from a simple exponential-overlap model, $\kappa \propto e^{-\gamma g}$ with $\gamma = 5$ μm⁻¹. (a) Draw $N = 2000$ samples with $\sigma_w = 2$ nm (affecting both guides, correlated 0.9) and $\sigma_g = 2$ nm; plot the splitting-ratio distribution at nominal 50/50. (b) Propagate into a 4×4 Clements mesh (use the transfer-matrix simulator from Chapter 12's projects): compute the distribution of matrix fidelity for a fixed target unitary, without and with per-MZI phase recalibration. (c) Report the fidelity percentiles and the recalibration budget (mean phase correction in rad).

**L23.3 — Yield and Cost Model**

Build a spreadsheet/Python model of an MPW-to-production path: inputs — die area, defect densities per layer, wafer cost, MPW seat cost, packaging cost per die, test cost per die, packaging yield. Outputs — cost per good packaged die vs. volume for (i) MPW seats, (ii) dedicated 25-wafer lots. Find the crossover volume, and the sensitivity of cost-per-die to packaging yield (vary 80–99%). Explain why "package known-good dies only" (wafer-level test) shifts the curves.

**L23.4 — Thermal Crosstalk Map (Simulation)**

Using a 2D finite-difference solution of the steady heat equation (write ~50 lines of Python; fixed-temperature bottom boundary as heat sink), model two TiN heaters 1.5 μm above a silicon layer, separated by pitch $p$. (a) Compute the temperature at each waveguide for unit power in heater 1; define crosstalk $X(p) = \Delta T_2/\Delta T_1$. (b) Plot $X$ for $p$ = 10–200 μm and find the pitch for $X < 1\%$. (c) Recompute with a 20 μm-deep isolation trench between them, and comment on the area-vs-crosstalk trade in dense MZI meshes.

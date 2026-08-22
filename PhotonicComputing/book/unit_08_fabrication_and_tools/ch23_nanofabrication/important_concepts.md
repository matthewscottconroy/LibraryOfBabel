# Chapter 23: Important Concepts

---

## 1. The Rayleigh Criterion: $CD = k_1\lambda/NA$

One equation prices the entire lithography menu. Projection lithography resolves features down to $CD = k_1\lambda/NA$, with $k_1 \approx 0.25$–0.4 at the aggressive end; the companion relation $DOF = k_2\lambda/NA^2$ taxes high NA with a shrinking focus window, which is what makes CMP mandatory. Photonics' 100–500 nm features sit comfortably within 193 nm DUV (dry: ~100 nm; immersion: ~40 nm), which is why every major photonics foundry standardized on ArF scanners and none needs EUV.

---

## 2. E-Beam Lithography: Resolution Without Throughput

Electron-beam direct write achieves 5–10 nm resolution with zero mask cost, making it the vehicle for research prototyping, photonic crystals, and inverse-designed geometries. Its costs are serial write time ($t = DA/I$; hours per chip, weeks per wafer), proximity-effect correction, and write-field stitching errors (10–30 nm) that become scattering and phase defects when a waveguide crosses a field boundary. E-beam also writes the photomasks — optical lithography is e-beam amortized over a million exposures.

---

## 3. Anisotropic Dry Etch and the Origin of Waveguide Loss

ICP-RIE combines reactive chemistry (rate, selectivity) with directional ion bombardment (anisotropy) to carve near-vertical waveguide sidewalls. Resist line-edge roughness plus stochastic etch chemistry leave 1–3 nm RMS sidewall roughness, and via the Payne–Lacey scaling $\alpha \propto \sigma^2 (n_1^2-n_2^2)^2 E_{sidewall}^2/d^3$ this — not material absorption — is why silicon strip waveguides lose 1–3 dB/cm. Corollaries: loss improves with lithography generation, wide waveguides route with ~10× less loss, and oxidation/H₂-anneal smoothing buys loss where thermal budget allows.

---

## 4. Thermal Budget: The Process Flow's Arrow of Time

Once metal (and germanium) are on the wafer, subsequent steps must stay below ~400–450 °C. This single constraint orders every flow — oxidation and LPCVD nitride first, implant anneals next, Ge epitaxy, then metals, then only PECVD/ALD — and explains material quirks like the N–H absorption overtone near 1520 nm that haunts low-temperature back-end nitride.

---

## 5. The PDK Contract

The process design kit is the foundry-designer interface: layer map, DRC deck (minimum features, enclosures, density/tiling rules), validated pcells with *measured* performance distributions, compact models, and verification decks. Its deepest clause: only PDK components are guaranteed. Photonic design freedom is real but priced — every custom polygon carries your risk, on a 4–9 month feedback loop.

---

## 6. MPW Economics

Multi-project wafers amortize mask sets (tens of masks, thousands of dollars each) across many customers: \$10k–75k buys 5–50 dies of a few-mm² design, in 4–9 months. Consequences: designs sweep parameters and carry heavy test-structure overhead; e-beam rapid prototyping (weeks) de-risks custom devices before MPW commitment; and one MPW's dies sample only local process variation — never lot-to-lot statistics.

---

## 7. Fabrication Sensitivity: $\delta\lambda \approx 0.55\,\delta w + 1.8\,\delta t$ (nm per nm)

Through $\delta\lambda = (\lambda/n_g)\,\delta n_{eff}$ with $\partial n_{eff}/\partial w \approx 1.5\times10^{-3}$/nm and $\partial n_{eff}/\partial t \approx 5\times10^{-3}$/nm (500×220 nm strip), routine ±2–5 nm dimensional scatter displaces ring resonances by tens of linewidths and shifts MZI arms by ~0.6 rad per nm·(100 μm). *No photonic computing architecture may assume as-drawn dimensions*: tuning, calibration, and their power budgets (tens of mW per ring; watts per weight bank) are architectural constants, not afterthoughts.

---

## 8. Correlation Is the Designer's Friend

Process variation is spatially correlated over hundreds of μm to mm, so *differences* between nearby, identically drawn, identically oriented devices are far tighter than absolute values. Hence: match critical pairs closely (MZI arms, balanced detectors), architect for common-mode rejection, and model variability with correlated Monte Carlo rather than independent draws. Foundry PCM structures and on-die monitor rings turn the wafer map into calibration data.

---

## 9. Yield: Functional vs. Parametric

Defect (functional) yield follows $Y = e^{-A\sum_i D_i}$ — benign for mm-scale photonic dies, murderous for reticle-scale photonic processors (an 8 cm² die at $D_{tot} = 0.3$ cm⁻² yields ~9%), motivating chiplets and redundant/reroutable meshes. Parametric yield — the fraction of devices whose *continuous* deviations stay within tuning capture range — is the binding constraint for analog photonics and must be computed from the sensitivity statistics of Concept 7.

---

## 10. The III-V Integration Ladder

Silicon cannot lase; the gain must be attached. Options, in increasing intimacy: flip-chip known-good dies (±1 μm assembly, serial); die/wafer bonding of unpatterned III-V films with laser cavities defined afterwards by silicon lithography (the Fang/Bowers insight — placement precision becomes irrelevant; Intel-scale production); micro-transfer printing (parallel, sparse, material-efficient); monolithic quantum-dot epitaxy on Si (dislocation-tolerant dots; CW 1.3 μm demonstrated, production pending). The choice sets coupling loss, thermal path (the BOX is a thermal blanket), and the system's wall-plug efficiency.

---

## 11. Coupling and Packaging Dominate Cost

Fiber-chip coupling bridges a 10.4 μm fiber mode to a 0.5 μm waveguide mode: grating couplers (−2 to −4 dB, ~35 nm bandwidth, wafer-testable, polarization-selective) versus edge couplers (0.5–2 dB, broadband, facet and sub-μm alignment required). Active alignment time, fiber attach, and thousands of electrical escapes (wire bonds: ~8 GHz/nH-limited and perimeter-bound; flip-chip/2.5D: area-array, tens of pH) make packaging and test — not the die — the cost center. Budget systems, not chips.

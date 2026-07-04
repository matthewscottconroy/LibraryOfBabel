# Chapter 26: Exercises

*The exercises for this chapter are weighted toward analysis and discussion, as befits its subject. Numerical exercises use representative public figures; where you can find better numbers, use them and say so.*

---

## Mathematical / Estimation Exercises

**M26.1 — The Startup Fab Budget**

A photonic computing startup plans its first two years around foundry MPW runs: 4 tape-outs at $60k each (shared-wafer cost for ~25 mm² in an advanced photonic process), plus packaging and fiber attach at $40k per run, test infrastructure at $500k (one-time), and a 6-person team at a fully loaded $250k/person/year.

(a) Compute the two-year technical budget. What fraction is silicon, and what fraction is people?

(b) A full custom wafer run costs $2–5M. At what team size does the MPW-versus-full-run cost ratio stop dominating the budget?

(c) Discuss: why does the MPW model (Section 26.3 of Chapter 7) effectively determine the *minimum viable seed round* for a photonic hardware company?

**M26.2 — Co-Packaged Optics Power Arithmetic**

A 51.2 Tb/s Ethernet switch uses 64 × 800G pluggable transceivers at 14 W each. A CPO implementation replaces them with in-package optical engines at 5 pJ/bit plus 2 external laser modules at 15 W each.

(a) Compute total optics power for both configurations and the percentage saving.

(b) A large AI data center operates 10,000 such switches. At $0.08/kWh and PUE 1.3, what is the annual electricity cost difference?

(c) List two costs the CPO configuration *adds* (consider serviceability and yield) and estimate their sign and rough magnitude qualitatively.

**M26.3 — Interconnect Demand Sanity Check**

Suppose 5 million AI accelerator packages ship in a year, and the accelerator roadmap calls for 10 Tb/s of off-package optical bandwidth each within a few years.

(a) At 1 Tb/s per optical I/O chiplet, how many chiplets per year does this imply? Compare with total annual shipments of datacenter optical transceivers (order 10⁷ units).

(b) At 8 wavelengths × 100 Gb/s per fiber pair, how many fiber attaches per package? Why does packaging, not photonics, become the bottleneck?

---

## Conceptual / Discussion Exercises

**C26.1 — Classify the Company**

For each of: Lightmatter (Envise), Lightmatter (Passage), Lightelligence (PACE), Lightelligence (Hummingbird), Ayar Labs, Celestial AI, PsiQuantum, Xanadu — place the product on the compute-interconnect axis, identify the customer, and state the dominant technical risk in one sentence each. Which placements changed over the company's life, and why?

**C26.2 — Explain the Pivot from First Principles**

Using only material from Chapters 25 and 28 (precision-energy trade-off, DAC/ADC overhead, data-movement energy), construct the argument that a rational photonic computing company in the early 2020s *should* have pivoted to interconnect. Then construct the strongest counter-argument (steelman the compute-first position). Which argument do you find stronger, and what evidence would change your mind?

**C26.3 — Reading a Cap Table**

Ayar Labs' investors came to include the venture arms of Intel, NVIDIA, and AMD simultaneously. What does each incumbent gain from this position even if it never acquires the company? What does the *absence* of comparable strategic investment in a sector (e.g., optical compute accelerators) signal — and what are two alternative explanations that do not imply technical skepticism?

**C26.4 — Failure Autopsies**

Compare the failure modes of Luminous Computing (vertical overreach) and Rockley Photonics (market timing via SPAC). For each, identify: the thesis, the number of simultaneously unsolved problems, the revenue bridge (or absence of one), and one decision that could plausibly have changed the outcome. Then state the general lesson as a single design rule for deep-tech companies.

**C26.5 — Why Quantum Raises More**

PsiQuantum raised more private capital by 2021 than most of the photonic AI sector combined, plus government partnerships in 2024, despite having no interim product by design. Analyze this using expected-value reasoning: discontinuous versus incremental payoffs, government co-investment incentives, and the role of a legible, falsifiable critical path (the loss budget). When is "no interim product" a fundable feature rather than a bug?

**C26.6 — The Ecosystem Map**

Draw the dependency graph for one company in this chapter: foundry, laser supplier, packaging/OSAT, detector supplier (if quantum), design tools, standards bodies (UCIe, OIF), and university pipeline. Identify the single point of failure. Who else depends on that same node?

---

## Lab / Research Exercises

**L26.1 — Claims Audit**

Choose one photonic computing company. Collect (i) its most technical marketing document and (ii) its most recent peer-reviewed or refereed-conference publication. Build a two-column table matching each marketing claim to its published support: supported / partially supported / unsupported. Write a one-page memo in the style of Chapter 25: what is demonstrated, at what scope, against what baseline?

**L26.2 — PDK Exploration**

Obtain an openly accessible photonic design kit or open-source photonic layout toolchain (e.g., the SiEPIC educational PDK, or the open-source GDSFactory ecosystem). Enumerate the available component library. Identify three components a photonic *computing* chip needs that the standard (communications-oriented) library does not provide, and sketch how you would qualify them.

**L26.3 — Group Atlas**

Pick one research group from Section 26.3 whose work you have not yet read. From its last three years of publications, produce: the group's central claim, its platform of choice, its industrial ties (spin-offs, funders), and the one paper you would assign a new student. Present in five minutes to your class or group.

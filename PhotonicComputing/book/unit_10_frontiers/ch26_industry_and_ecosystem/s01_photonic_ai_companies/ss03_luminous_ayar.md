# 26.1.3 Luminous, Ayar Labs, Celestial AI, and the Interconnect-First Thesis

The third subsection of this survey pairs a cautionary tale with two companies that never needed the pivot — because they started where the others ended up.

## Luminous Computing: The Maximal Bet

Luminous Computing, founded in 2018 and backed by prominent investors including Bill Gates, raised roughly \$115M (a \$105M Series A in 2022 on top of seed funding) toward the most ambitious framing in the sector: build an entire AI supercomputer — compute, memory, and interconnect — around silicon photonics, vertically integrated from chip to system, and compete head-on with GPU clusters for training the largest models.

The ambition compressed every hard problem in this book into one product: analog optical arithmetic (Chapter 25's precision and conversion taxes), photonic memory access (Chapter 28's unsolved memory problem), massive laser and packaging scale-up (Chapter 23), and a software stack — all before first revenue, and all against an incumbent improving on a yearly cadence. By public accounts the company had wound down its supercomputer ambitions by around 2023–2024. The lesson is not that the physics was wrong; it is that *the number of simultaneously unsolved problems a startup can carry is small*. Luminous carried nearly all of them at once.

## Ayar Labs: Optical I/O as a Product, Not a Revolution

Ayar Labs (founded 2015, Santa Clara) is the counter-example, and its intellectual pedigree is worth knowing. It commercializes the "zero-change" monolithic electronic-photonic integration lineage of Vladimir Stojanović, Rajeev Ram, and Miloš Popović — the research that produced the first microprocessor communicating directly with light [Sun et al., *Nature*, 2015] and monolithic photonics in a commercial 45 nm CMOS node [Atabaki et al., *Nature*, 2018].

Ayar sells exactly two things:

- **TeraPHY**, an optical I/O chiplet that sits in-package beside a customer's ASIC or GPU and converts its die-to-die electrical interface (including the UCIe standard) to dense WDM optical fiber I/O — terabit-per-second-class bandwidth per chiplet at single-digit pJ/bit [Wade et al., *IEEE Micro*, 2020];
- **SuperNova**, an external multi-wavelength laser source supplying the (deliberately off-chip, field-replaceable) optical power.

Note what Ayar does *not* claim: no optical arithmetic, no new programming model, no displacement of the customer's compute. Its strategic investors — which by its 2024 round included the venture arms of Intel, NVIDIA, and AMD simultaneously, with total funding around \$370M — are the incumbents themselves. When all three leading compute vendors invest in the same optical I/O supplier, you are looking at the industry's own forecast of where photonics enters the computer first.

## Celestial AI: Interconnect Aimed at the Memory Wall

Celestial AI (founded 2020) sharpened the interconnect thesis into a specific architectural claim with its **Photonic Fabric**: optical interconnect implemented as chiplets and interposers that link accelerators not only to each other but to *disaggregated memory* — attacking the memory bandwidth and capacity wall (Chapter 25) rather than the FLOPS budget. The company raised several hundred million dollars across 2023–2025 rounds from financial and strategic investors, on the argument that the binding constraint for large-model inference is bytes per second to memory, not operations per second — precisely the diagnosis our benchmarking unit reached.

## The Interconnect-First Thesis, Stated

These case studies justify a thesis we will elevate to a principle in Chapter 28:

> **Photonics enters computing systems in order of decreasing communication distance and increasing integration intimacy — long-haul, then data-center network, then rack, then package, then interposer — and optical *computation* becomes commercially viable, if it does, only after optical *connectivity* has industrialized the underlying component supply chain.**

| | Optical compute startup | Optical interconnect startup |
|---|---|---|
| Customer relationship | Competes with incumbents | Supplies incumbents |
| Tolerance for analog error | Low (precision-bound) | High (BER-bound, with FEC) |
| Software burden | New stack required | None (transparent bits) |
| Adoption unit | Whole accelerator | One chiplet / one link |
| Failure mode | Must win outright | Can win incrementally |

Nothing in this table says optical compute is impossible. It says optical compute is *late in the order of battle* — and the companies that survived the mid-2020s are the ones that understood the ordering.

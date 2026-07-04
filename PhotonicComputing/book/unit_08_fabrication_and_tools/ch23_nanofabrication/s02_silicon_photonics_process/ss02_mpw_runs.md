# 23.2.2 Multi-Project Wafer Runs

## The Economics of Sharing a Mask Set

A dedicated wafer run in a modern photonic foundry costs hundreds of thousands of dollars, dominated by the mask set (a full active flow uses 20–40 mask levels, each a DUV reticle costing thousands to tens of thousands of dollars) and by the fixed cost of occupying the line. No graduate student, and few startups, can buy that per design iteration.

The **multi-project wafer (MPW)** amortizes those fixed costs: the foundry (or a broker) aggregates many customers' designs onto one shared reticle, runs one wafer lot, dices it, and distributes to each customer only their own dies. Typical parameters of a silicon photonics MPW:

- **Block size**: a few mm² up to ~25–100 mm² (e.g., 5 × 5 mm is a common quantum), priced roughly per mm² — order of $500–1500/mm² for full active flows, much less for passive-only.
- **Cost per seat**: **$10k–75k** depending on foundry, flow (passive vs. active), and area.
- **What you receive**: typically 5–50 identical dies.
- **Turnaround**: **4–9 months** from submission deadline to chips in hand; submission deadlines run a few times per year per platform.

For academic groups this is transformative — state-of-the-art fabrication for the price of a conference budget — and it imposes an equally transformative discipline: with two or three tape-out windows per year, *a design error costs a season*. The simulation-heavy methodology of Chapter 24, and the test-structure habits described below, are the community's adaptation to that clock.

## Who Runs MPWs

The landscape (stable in shape, evolving in detail — check current offerings):

| Organization | Platform | Notes |
|---|---|---|
| IMEC (Belgium, via Europractice) | iSiPP full active SOI, 200 mm | The broadest standard PDK; long-running reference platform |
| CEA-Leti (France) | Active SOI, heterogeneous options | Strong III-V-on-Si research links |
| AIM Photonics (USA) | Full active SOI, 300 mm | US-government-backed; PIC + interposer options |
| AMF (Singapore) | Active SOI, 200 mm | Popular for cost-effective active runs |
| GlobalFoundries | 45/90 nm monolithic photonics + CMOS | Photonics beside advanced electronics |
| Tower Semiconductor | PH18 active SOI | Commercial production emphasis |
| CORNERSTONE (Southampton, UK) | Passive/active SOI, SiN | Research-friendly, frequently subsidized |
| LioniX, Ligentec | Si₃N₄ (TriPleX / thick nitride) | Ultra-low-loss nitride specialists |
| SMART Photonics, Fraunhofer HHI | InP | Full active III-V: lasers, SOAs, modulators monolithically |
| Applied Nanotools (Canada) | E-beam SOI/SiN rapid prototyping | Weeks, not months; passive; no masks |

Brokers — **Europractice** in Europe, **CMC Microsystems** in Canada — aggregate university access, handle NDAs and PDK licensing, and subsidize academic seats. E-beam-based rapid-prototyping services (and university shuttles built on them, such as the SiEPIC program's fabrication runs) fill the gap between simulation and a foundry MPW: 2–8 week turnaround for passive silicon devices, ideal for de-risking custom components before committing them to a 6-month active run.

## Using an MPW Well

Hard-won practice, compressed:

**1. Spend area on test structures.** A returned die that cannot be measured is a paperweight. Standard practice devotes 20–50% of a research block to: cutback structures (identical paths of different lengths, to extract dB/cm loss by linear fit), de-embedding pairs for every custom component, isolated copies of each novel device with their own I/O, and process-monitoring rings whose resonance shifts read out local width/thickness (Section 23.2.3).

**2. Sweep what you cannot predict.** Fabrication bias (Section 23.1.2) means your directional coupler's gap will not print as drawn. Lay out the coupler at gap − 20 nm, gap, gap + 20 nm; the length-sweep and gap-sweep matrix is cheap insurance. Inverse-designed devices (Section 24.3) go out with eroded/dilated variants for the same reason.

**3. Standardize your I/O for the test bench you actually have.** Grating-coupler arrays at 127 μm pitch matched to commercial fiber arrays, with identical loopback pairs at known positions, allow automated wafer-scale probing (Section 23.4). The community's shared templates (e.g., those in the SiEPIC and gdsfactory ecosystems) encode these conventions; use them.

**4. Respect the deadline mechanics.** MPW aggregation means late designs wait months for the next window; DRC violations discovered by the foundry at merge time can silently bump you. Internal tape-out should complete a week before the real one.

**5. Plan the experiment for n = 5–50 dies.** You will get enough chips for statistics on one wafer's local variation, but *not* across-lot statistics; conclusions about "the process" from one MPW are conclusions about one wafer neighborhood on one run. Production-bound designs eventually need dedicated engineering runs precisely to see lot-to-lot spread.

## What MPWs Mean for Photonic Computing Research

Nearly every photonic computing demonstration in Units V–VII — MZI meshes, ring weight banks, photonic tensor cores, quantum interference chips — was fabricated on an MPW or a university e-beam line. The consequences shape the literature you read: chip areas cluster near MPW block sizes (which bounds mesh sizes near the reticle-scale limits discussed in Chapter 12's scaling exercises); demonstrations favor passive meshes with off-chip light and detection (passive runs are cheap and fast); and results are reported for the handful of dies that came back, with calibration absorbing die-to-die spread. Reading a photonics paper well means asking the MPW questions: which foundry, which flow, how many dies, and how much tuning did it take to make the physics visible?

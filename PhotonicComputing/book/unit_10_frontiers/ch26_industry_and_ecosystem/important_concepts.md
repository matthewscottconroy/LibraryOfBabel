# Chapter 26: Important Concepts

---

## 1. The Compute-versus-Interconnect Axis

Every photonic computing product either performs arithmetic optically (compute) or moves bits optically (interconnect). The two categories differ in customer relationship (compete with vs. supply the incumbents), error tolerance (analog precision vs. BER with forward error correction), software burden (new stack vs. none), and adoption granularity (whole accelerator vs. single chiplet). This axis is the fastest way to understand any company in the sector.

---

## 2. The Great Pivot

The central empirical fact of the photonic AI industry's first decade: companies founded on optical compute (Lightmatter, Lightelligence) migrated their flagship products to optical interconnect (Passage; Hummingbird oNOC), while interconnect-native companies (Ayar Labs, Celestial AI) drew strategic investment from incumbent chipmakers. The physics didn't change; the market revealed that bandwidth between digital chips was the binding constraint of the AI build-out.

---

## 3. Co-Packaged Optics (CPO) as Beachhead

Moving transceiver optics from faceplate pluggables (~12–15 pJ/bit) into the ASIC package (~5 pJ/bit) roughly halves switch optics power at 51.2 Tb/s scale and eliminates the long electrical trace at 100+ Gb/s lanes. CPO industrializes lasers, fiber attach, packaging, and test — the same supply chain any optical compute product would need — which is why the compute agenda benefits even when only interconnect ships.

---

## 4. The Foundry Layer and MPW Access

No photonic startup owns a fab. AIM Photonics (US), imec (EU), GlobalFoundries Fotonix, TSMC, CEA-Leti, AMF, and Tower provide the CMOS-photonics processes; LioniX/Ligentec (Si₃N₄), HyperLight (thin-film LiNbO₃), and SMART Photonics (InP) provide specialty platforms. Multi-project wafer runs put credible prototypes under $100k per tape-out, which is the economic enabler of the entire startup wave — and the reason photonic hardware startups can exist on venture timescales at all.

---

## 5. Strategic Capital as a Forecast

Read cap tables as predictions: when the venture arms of Intel, NVIDIA, and AMD simultaneously back an optical I/O supplier (Ayar Labs), the incumbents are telling you where they expect photonics to enter the computer. Financial-only capital chases narrative; strategic capital chases roadmaps.

---

## 6. The Two Quantum Bets

PsiQuantum: discrete-variable, fusion-based, fault-tolerance-or-nothing, manufactured in a commercial 300 mm fab, with photon loss (an erasure error, threshold ~10%) as the single tracked figure of merit. Xanadu: continuous-variable, squeezed-light and GBS, with GKP-state generation as the acknowledged critical step, hedged by the PennyLane software franchise. The two programs are near-perfect experimental controls for each other.

---

## 7. Component Specialists as Architecture-Neutral Infrastructure

QuiX (Si₃N₄ interferometers), Quandela (quantum-dot sources), ORCA (memory-based multiplexing), Sparrow Quantum, Single Quantum, Pixel Photonics: the European cluster sells the subsystems every architecture needs. Whoever wins the full-stack race buys from — or replicates — the specialists.

---

## 8. Vertical Overreach as the Canonical Failure Mode

Luminous Computing attempted photonic compute, memory access, packaging scale-up, and a software stack simultaneously, pre-revenue; it wound down by ~2023–2024 despite ~$115M raised. Rockley Photonics (health-sensing photonics via SPAC) reached bankruptcy in 2023. The survivors share one property: a single, narrowly scoped, incrementally adoptable product.

---

## 9. Press Release vs. Peer Review

The sector's claims span a credibility gradient: peer-reviewed journal papers > refereed system conference talks (Hot Chips, ISSCC, OFC post-deadline) > whitepapers > press releases. The evaluation discipline of Chapter 25 — end-to-end energy, matched-baseline comparisons, workload specificity — applies at every level, but can only be *checked* at the first two.

---

## 10. The Talent Graph

Essentially all companies in this chapter descend from a few dozen academic groups (Sections 26.3.1–26.3.3), densely linked by co-authorship: MIT's photonics groups begat Lightmatter and Lightelligence; Bristol/Imperial begat PsiQuantum; C2N begat Quandela; Oxford begat ORCA; Berkeley/MIT's zero-change CMOS program begat Ayar Labs. For students, the practical corollary: choosing a group is choosing a region of the industry graph.

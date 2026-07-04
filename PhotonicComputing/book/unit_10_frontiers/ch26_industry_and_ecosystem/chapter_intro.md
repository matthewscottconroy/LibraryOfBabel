# Chapter 26: The Photonic Computing Industry and Research Ecosystem

> *"The road from a Nature paper to a data-center product is longer than it looks from either end."*
>
> — Paraphrase of a sentiment common among photonic computing founders

---

## From Laboratory to Industry

Photonic computing stopped being a purely academic subject sometime around 2017. In that year, the demonstration of a coherent nanophotonic neural network at MIT [Shen et al., *Nature Photonics*, 2017] spawned two venture-backed companies within months (Lightmatter and Lightelligence, founded by different authors of the same paper); PsiQuantum was already assembling the case that a fault-tolerant photonic quantum computer should be built in a commercial CMOS foundry; and the silicon photonics transceiver industry — the field's economic bedrock — was shipping at a scale that made "photonics in the data center" an ordinary fact rather than a speculation.

This chapter maps the resulting ecosystem as it stood in the mid-2020s. It has three parts: the photonic AI hardware companies (Section 26.1), the quantum photonics companies (Section 26.2), and the academic research landscape that feeds both (Section 26.3). Around these we describe the connective tissue — foundries, funding, and product categories — that determines which ideas become products.

---

## Two Product Categories: Compute and Interconnect

Every photonic computing company can be located on a single axis, and locating it there is the most clarifying act of analysis you can perform.

**Optical compute** products perform arithmetic in the optical domain: matrix-vector products in MZI meshes or ring weight banks, Ising energy minimization in oscillator networks, convolutions in free space. Their promise is the energy and latency physics of Units V–VI; their burden is everything in Chapter 25 — analog precision, calibration, the DAC/ADC tax, and competition with a digital industry that improves relentlessly.

**Optical interconnect** products move bits, not operations: optical I/O chiplets, co-packaged optics, photonic interposers and switch fabrics. Their promise is more modest and more certain — photons beat electrons at *communication* long before they beat them at *logic* (a theme made quantitative in Chapter 28) — and their customers are the incumbent compute vendors rather than their competitors.

The single most important structural fact about this industry is the direction of migration along this axis: company after company founded on optical compute has shifted its lead products toward optical interconnect. Lightmatter's flagship became Passage (a photonic interposer), Lightelligence's became an optical network-on-chip, and the companies founded directly on interconnect (Ayar Labs, Celestial AI) attracted strategic investment from the largest chipmakers. Section 26.1 examines why.

---

## The Foundry Ecosystem

No photonic computing startup builds its own fab. The industry rests on a foundry layer, introduced in Chapter 7, whose principal actors deserve restating in this commercial context:

| Foundry / institute | Role in the computing ecosystem |
|---|---|
| **AIM Photonics** (US) | DoD-backed Manufacturing USA institute (founded 2015); MPW access, test and packaging facility in Rochester; a US-government-accessible supply chain |
| **imec** (Belgium) | iSiPP silicon photonics MPW; the default European prototyping route; deep ties to Ghent University research |
| **GlobalFoundries** (US) | Fotonix: monolithic RF-CMOS + photonics on 300 mm wafers; manufacturing partner for PsiQuantum, Lightmatter, and Ayar Labs |
| **TSMC** (Taiwan) | The advanced-CMOS leader; publicly developing compact silicon photonics engines for co-packaged optics with major AI-hardware customers |
| **CEA-Leti, AMF, Tower, VTT** | Regional platforms: III-V heterogeneous integration, low-cost prototyping, high-power handling |
| **LioniX / Ligentec** (EU) | Si₃N₄ specialty foundries — ultra-low loss, visible-to-IR transparency; quantum and frequency-comb applications |
| **HyperLight; SMART Photonics** | Thin-film lithium niobate and InP respectively — the high-performance modulator and laser platforms |

Two ecosystem developments of the early 2020s matter for computing specifically. First, *lowered design barriers* — university MPW programs, open-source layout tools, and PDK-based design flows — pushed the cost of a credible photonic prototype below $100k per tape-out, enabling the startup wave. Second, *co-integration roadmaps* — monolithic photonics in a 45 nm CMOS process [Atabaki et al., *Nature*, 2018], and 3D hybrid bonding of electronic dies onto photonic interposers — turned "photonics and electronics in one package" from a research aspiration into a product path.

---

## Co-Packaged Optics: The Beachhead

The commercial beachhead for photonics *inside* the computer — as opposed to between computers — is **co-packaged optics (CPO)**: placing the optical transceiver silicon in the same package as a switch ASIC or processor, replacing pluggable modules at the faceplate. The arithmetic is straightforward. At 51.2 Tb/s per switch, pluggable optics at roughly 12–15 pJ/bit consume on the order of a kilowatt per switch; co-packaged optics at ~5 pJ/bit roughly halve that figure and remove the increasingly painful electrical trace between ASIC and faceplate at 100+ Gb/s per lane. By 2024–2025 the largest switch and GPU vendors had announced or shipped CPO-based products, and optical I/O chiplets conforming to die-to-die interface standards (UCIe) were sampling. Every photonic *computing* company benefits from this beachhead: it industrializes the lasers, fiber attach, packaging, and test infrastructure that an optical compute product would also need.

---

## The Funding Landscape

Venture and government capital in this sector has displayed three stable patterns. (1) **AI interconnect attracts strategic money**: the venture arms of the major chip and systems companies appear repeatedly on the cap tables of optical I/O startups — the incumbents hedging their own interconnect roadmaps. (2) **Quantum photonics raises the largest single rounds**: PsiQuantum alone had raised more private capital by 2021 (roughly $665M) than most of the photonic AI sector combined, later supplemented by government partnerships in Australia and the United States (2024) — because the prize is discontinuous rather than incremental. (3) **The sector has real failures**: SPAC-era disappointments (Rockley Photonics' 2023 bankruptcy) and the quiet wind-down of ambitious compute-first startups (Luminous Computing) discipline the survivors' claims. After 2022, the AI capital cycle redirected attention toward anything that relieves the data-center bandwidth and energy walls — which is to say, toward interconnect.

---

## Chapter Structure

**Section 26.1 — Photonic AI Hardware Companies**: Lightmatter, Lightelligence, and the interconnect-first companies (Luminous, Ayar Labs, Celestial AI), read as case studies in the compute-versus-interconnect migration.

**Section 26.2 — Quantum Photonics Companies**: PsiQuantum, Xanadu, and the European quantum photonics cluster (QuiX, Quandela, ORCA), read as three distinct architectural and commercial bets.

**Section 26.3 — The Academic Research Landscape**: the university and national-laboratory groups, organized geographically, whose work populates the reference lists of this book.

A standing caveat applies to every named company, number, and product in this chapter: the snapshot is circa 2025, and the durable content is the analysis, not the roster.

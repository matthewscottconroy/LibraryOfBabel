# 26.1.1 Lightmatter: From Photonic Compute to the Photonic Interposer

## Origins

Lightmatter was founded in Cambridge, Massachusetts in 2017 by Nicholas Harris, Darius Bunandar, and Thomas Graham. Harris and Bunandar came out of MIT's quantum photonics community; Harris was a lead author on the programmable nanophotonic processor work in Dirk Englund's group and a co-author of the 2017 coherent nanophotonic neural network demonstration [Shen et al., *Nature Photonics*, 2017] that is, as much as any single paper, the founding document of the photonic AI industry. The company's premise followed directly from Unit V of this book: an MZI mesh performs an $N \times N$ matrix-vector multiplication in a single optical transit, so build a commercial accelerator around one.

## The Compute Products: Mars and Envise

Lightmatter's first public silicon, a test chip presented at the Hot Chips 32 conference [Ramey, 2020], demonstrated a photonic matrix engine co-packaged with digital control — the architecture pattern of Chapter 25: photonic core, electronic everything-else. Its productization, **Envise**, was announced as a full inference accelerator: photonic tensor cores for the matrix products, SRAM and digital SIMD units for the nonlinearities, activations, and data marshaling that photonics cannot do. Envise embodied both halves of the photonic computing argument. The optical matrix engine delivered the promised density and energy characteristics for large matrix products; and the surrounding digital machinery — DACs, ADCs, memory, control — reproduced exactly the system-level costs that Chapter 25 warns dominate end-to-end budgets. Publicly reported deployments of Envise remained limited, and the company's emphasis visibly shifted.

## The Pivot: Passage

**Passage** is a wafer-scale programmable photonic interconnect — an interposer: a large photonic die on which multiple electronic chips (GPUs, accelerators, memory) are mounted and interconnected by dense waveguides and WDM links rather than by electrical traces. The value proposition requires no claims about optical arithmetic at all: it is bandwidth density, reach, and energy per bit for chip-to-chip communication, sold to the very companies whose digital accelerators a photonic computer would otherwise have to beat. By 2025 the Passage line — culminating in announced multi-hundred-terabit-per-second interposer and 3D co-packaged optics products — had become the company's flagship, manufactured with GlobalFoundries.

The migration from Envise to Passage is the cleanest single example of the industry-wide pattern named in this chapter's introduction, and its logic is worth stating explicitly:

1. **Interconnect composes with the incumbent; compute competes with it.** A photonic interposer makes NVIDIA-class silicon better. A photonic accelerator must beat it.
2. **Interconnect tolerates analog imperfection; compute does not.** A link needs a bit error rate; a matrix engine needs calibrated analog precision across thousands of devices (Chapter 25).
3. **The AI market's binding constraint moved.** After 2022, the scarce resources in AI data centers were bandwidth between chips and power delivery — precisely what optical interconnect addresses.

## Funding and Scale

Lightmatter's capital trajectory tracked the AI cycle: a $154M Series C in 2023, extensions later that year, and a reported $400M Series D in late 2024 at a valuation of roughly $4.4B — approximately $850M raised in total by that point, making it the most heavily funded photonic AI company. Note what the market priced: by the time of the largest rounds, the pitch was interconnect.

## How to Read Lightmatter

For a student of this book, Lightmatter is a live experiment in three of our recurring themes: the MZI mesh as a computing primitive (Chapter 11), the electronic-photonic co-design problem (Chapter 25), and the interconnect-first thesis (Chapter 28). The discipline to apply when reading any of its claims — or any competitor's — is the one this book has practiced throughout: locate the peer-reviewed or conference-published system data (Hot Chips presentations, journal papers), reconstruct the end-to-end energy and throughput accounting, and treat marketing numbers as hypotheses rather than results.

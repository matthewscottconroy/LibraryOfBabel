# Prokaryotic vs. Eukaryotic Cell Architecture

Here is one of the most consequential boundaries in all of biology: the line between the prokaryotes and the eukaryotes. On one side sit bacteria and archaea — cells without nuclei, tiny, ancient, and metabolically versatile beyond imagination. On the other side sit all cells with a nucleus: yeast, protists, fungi, plants, animals, and us. This boundary was crossed approximately 1.5–2 billion years ago in a single endosymbiotic event, and virtually every decision you will make as a computational or synthetic biologist — which model organism to use, which modeling assumptions are valid, what timescales govern your circuit's dynamics — flows from which side of this boundary your organism of interest inhabits.

Understanding the fundamental differences between prokaryotic and eukaryotic cells is essential for every practitioner of computational biology. The organisms we most commonly engineer (bacteria, yeast), the organisms whose genomes we most commonly sequence (all of them), and the organisms whose cells we most commonly model (both) fall across this divide. The architectural differences profoundly affect gene regulation, gene circuit design, and modeling assumptions.

## Prokaryotes: Bacteria and Archaea

**Bacteria** are the primary organisms of synthetic biology. Their defining features:

- **No nucleus:** DNA is not membrane-bound; the chromosome is in the **nucleoid** (a loosely defined region of the cytoplasm)
- **Single circular chromosome:** *E. coli* K-12 has a 4.6 Mb chromosome with ~4400 protein-coding genes
- **70S ribosomes:** Smaller subunit (30S) and large subunit (50S). Target of many antibiotics (chloramphenicol, erythromycin, tetracycline, streptomycin — each targeting a specific ribosomal component)
- **No membrane-bound organelles:** No mitochondria, ER, Golgi
- **Coupled transcription-translation:** mRNA is translated by ribosomes even while still being transcribed — there is no nuclear export step, no splicing
- **Cell wall:** Gram-positive (thick peptidoglycan), gram-negative (thin peptidoglycan + outer membrane with LPS)
- **Typical size:** 1–5 µm, volume ~1 fL (femtoliter = $10^{-15}$ L)
- **Doubling time:** 20 min (*E. coli* in rich media) to hours (slow-growing soil bacteria)

**Archaea** resemble bacteria morphologically but have:
- Ether-linked lipids (instead of ester-linked in bacteria and eukaryotes)
- Transcription machinery more similar to eukaryotes (TATA-box promoters, TBP, TFIIB homologs)
- Histones (some archaea) — chromatin-like organization
- Ability to survive extreme environments (thermophiles, halophiles, methanogens)

The CRISPR-Cas system was discovered in archaea and bacteria as an adaptive immune system; its eukaryotic absence made it ideal for re-engineering as a genome editing tool.

## Eukaryotes: A Different Design Space

Eukaryotes (yeast, animals, plants, fungi) share:

- **True nucleus:** DNA is enclosed in a double-membrane envelope with nuclear pore complexes (NPCs) — each pore complex is a ~120 MDa, 120 nm diameter machine that controls all nucleocytoplasmic transport
- **Linear chromosomes:** Multiple chromosomes with centromeres and telomeres
- **80S ribosomes:** Large (60S) and small (40S) subunits; target of different antibiotics from prokaryotes (cycloheximide, emetine)
- **Membrane-bound organelles:** Mitochondria, ER, Golgi, lysosomes (see organelles section)
- **Splicing:** Pre-mRNA splicing is required for most genes before export and translation — a major regulatory layer absent in prokaryotes
- **Histone-packaged chromatin:** DNA is wrapped around histone octamers, dramatically affecting gene accessibility
- **Typical size:** 10–100 µm

## The Fundamental Consequence for Gene Circuit Timing

The absence of a nucleus in bacteria means transcription and translation are coupled in time and space:
- Ribosomes attach to mRNA while it is still being transcribed
- There is no nuclear export delay
- Regulation can act at any point from transcription initiation through translation

In eukaryotes, the sequence is:
1. Transcription (nucleus) — minutes to hours
2. Pre-mRNA processing: 5' capping, splicing, 3' polyadenylation (nucleus) — minutes
3. Nuclear export through NPCs — minutes to hours
4. mRNA localization (often) — minutes
5. Translation (cytoplasm) — minutes per protein molecule
6. Post-translational modification (ER, Golgi, cytoplasm) — minutes to hours

This temporal separation creates **regulatory layers** absent in bacteria. Importantly for gene circuit modeling: the effective delay from gene activation to protein appearance is much longer in eukaryotes (~30–60 min total) than in bacteria (~5–10 min). This time delay affects the dynamics of gene circuits — specifically, delay in negative feedback loops can cause oscillations (the NF-$\kappa$B oscillator depends in part on the nuclear-cytoplasmic transport delay). You might think that more steps between gene and protein would simply slow things down uniformly — but in fact, the delay creates qualitatively new dynamical behaviors. This is a theme you will encounter repeatedly: architectural differences between cell types are not just a catalog of parts, but generators of distinct dynamic regimes.

## Cell Size and Its Constraints

**Surface area-to-volume ratio (SA:V):** For a sphere of radius $r$: SA:V = $3/r$. Larger cells have lower SA:V. Bacteria ($r \approx 1\ \mu$m, SA:V $\approx 3\ \mu$m$^{-1}$) can exchange nutrients and waste through diffusion across their surface much faster relative to their volume than a human cell ($r \approx 10\ \mu$m, SA:V $\approx 0.3\ \mu$m$^{-1}$).

**Diffusion times scale as $r^2/D$:** In *E. coli*, a transcription factor with $D = 10\ \mu$m$^2$/s reaches any point in the cell in $\sim (0.5)^2 / (2 \times 10) \approx 0.01$ s — effectively instantaneous. In a large eukaryotic cell (radius 10 µm), the same protein takes $100/20 \approx 5$ s — still fast, but not instantaneous. Compartmentalization in organelles further restricts diffusion. The practical upshot is that, for most bacterial gene circuit models, the well-mixed assumption is justified: any protein produced anywhere in the cell is functionally available everywhere. For eukaryotic models, compartment-specific concentrations matter quantitatively.

**Minimal gene number:** The smallest known free-living bacteria (*Mycoplasma genitalium*: 485 protein-coding genes) provide a lower bound on the gene set needed for cellular life. Obligate intracellular parasites like *Rickettsia* have ~830 genes. These systems inform the design of minimal synthetic cells.

## Why This Matters for Computational Biology

When you build an ODE model of a gene circuit, the choice of organism determines your model structure:
- Bacteria: simpler models; transcription and translation often lumped; typically 5–10 min response time
- Yeast/mammalian: nuclear-cytoplasmic transport must be modeled explicitly if dynamics on the timescale of 10–60 min are important; mRNA maturation adds delay; chromatin state adds a regulatory dimension

When you design a synthetic circuit, the chassis (host organism) determines:
- Available molecular tools (restriction-modification systems, recombinases, inducers)
- Codon usage bias (affects heterologous protein expression)
- Resource competition (ribosomes, RNAP, chaperones are shared among all genes)
- Metabolic background (existing pathways that interact with your circuit)

The basic architectural principles introduced here will be elaborated throughout the curriculum.

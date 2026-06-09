# scRNA-seq Technology Overview

To sequence the RNA of a single cell, you face an immediate problem of scale. A typical mammalian cell contains roughly 200,000–500,000 mRNA molecules. But sequencing libraries require micrograms of DNA — roughly a billion-fold more material than a single cell provides. This means every scRNA-seq technology must solve the same core engineering challenge: capture the mRNA from one cell, mark it with a unique cellular identity tag, amplify it without losing the relative proportions of different transcripts, and do this for thousands of cells simultaneously — without contaminating any cell's molecules with another's.

How different platforms solve this problem determines their throughput, their resolution, their artifacts, and the computational corrections you will need to apply. Understanding the technical principles of each platform is essential for interpreting data artifacts and choosing the right platform for a given experiment.

## 10x Genomics Chromium: Droplet-Based Sequencing

The dominant solution to the cell-isolation problem, pioneered commercially by 10x Genomics, is microfluidics. Cells and barcoded beads flow through narrow channels and are co-encapsulated, one cell per bead, in nanoliter-scale oil droplets. The physics of the droplet size and flow rate is tuned so that, statistically, most droplets contain exactly zero or one cell. The bead serves as both the identifier and the capture reagent for that cell's mRNA.

The **10x Genomics Chromium** system is the dominant platform for high-throughput scRNA-seq. Cells and barcoded beads are co-encapsulated in nanoliter-scale oil droplets called **GEM wells** (Gel Bead-in-Emulsion). Each bead carries ~750,000 copies of a single barcode — a unique 16-nt sequence that tags all cDNA molecules derived from that cell.

The bead also carries a **UMI** (Unique Molecular Identifier) — a random 10–12 nt sequence attached to each individual poly-A capture oligo. Since each mRNA molecule is captured by a distinct oligo with a distinct UMI, PCR duplicates (which share the same UMI) can be collapsed during analysis. Without UMIs, PCR amplification would introduce severe count biases — a gene captured by 10 initial molecules but amplified 1,000-fold would appear to have 10,000 copies, indistinguishable from a gene that truly had 10,000 copies at the start.

Typical specifications for 10x Chromium v3.1:
- Throughput: 500–10,000 cells per GEM well
- Genes detected per cell: ~2,000–5,000
- Reads per cell: 20,000–50,000 recommended
- Library prep: 3' end capture (mRNA 3' bias)
- Cell capture efficiency: ~65% of loaded cells

## UMI Collision Probability

UMIs are finite-length random sequences, which means two different mRNA molecules in the same cell could, by chance, receive the same UMI — a **collision**. Collapsed as a duplicate, two real molecules would be counted as one, underestimating expression. The probability of collision depends on the number of UMI positions and the number of molecules. For a 10-nt UMI (4^10 = ~1 million possibilities) and ~20,000 mRNA molecules per cell, the expected collision rate is approximately:

$$P(\text{collision}) \approx 1 - e^{-N/4^L}$$

where $N$ is the number of captured molecules and $L$ is UMI length. For $N = 20{,}000$ and $L = 10$, this is ~2%, which is acceptable. Shorter UMIs (6 nt, 4^6 = 4,096 possible) would produce unacceptably high collision rates in real experiments. This is why UMI lengths have progressively increased in newer platform versions.

## Ambient RNA Contamination

When cells are lysed during library preparation, their mRNA is released into the surrounding solution. This **ambient RNA** (also called "soup") can be encapsulated into droplets that do not contain a viable cell, or added on top of the true signal from a cell-containing droplet. Tools like **SoupX** or **DecontX** estimate and correct for ambient RNA contamination by modeling the composition of empty droplet barcodes.

Ignoring ambient RNA produces subtle but consequential errors. Hemoglobin genes, for example, are extremely highly expressed in red blood cells and will be present at high concentrations in the ambient RNA of any blood dataset. Without correction, every other cell type in the dataset will appear to weakly express hemoglobin — not because they transcribe it, but because their droplets contain traces of ambient soup. This has led to incorrect biological conclusions about cell types where hemoglobin expression was never expected.

## Doublets

When two cells are co-encapsulated in a single GEM, the resulting barcode will have the expression profile of both cells — a **doublet**. The doublet rate increases with cell loading density (at 5,000 cells loaded, ~3.9% doublets; at 10,000 cells, ~7.7% doublets). Doublets appear in UMAP as hybrid clusters between two genuine cell types, or cause artificial trajectories that seem to show one cell type transitioning into another when in fact both signatures simply came from a doublet.

Tools like **Scrublet** or **DoubletFinder** identify likely doublets by simulating doublet transcriptomes and comparing observed cells to the simulated population. Any cell that looks more like two cells superimposed than a genuine single cell gets flagged. Removing doublets before clustering is now standard practice.

## Smart-seq2: Full-Length, Plate-Based Sequencing

Droplet-based sequencing has a fundamental limitation: it captures only the 3' end of each mRNA, because the capture oligo binds the poly-A tail. This means you cannot determine which isoform of a gene is expressed from 10x data. **Smart-seq2** solves this problem by taking a completely different approach.

**Smart-seq2** physically sorts individual cells into wells of a 96- or 384-well plate via FACS. mRNA is captured using an oligo-dT primer and reverse transcribed using a template-switching mechanism. The result is full-length cDNA coverage of each transcript — unlike 3' capture in 10x, Smart-seq2 enables isoform analysis. Limitations: much lower throughput (hundreds of cells vs. tens of thousands), higher cost per cell, and no UMIs (requiring alternative normalization). Smart-seq2 is preferred for rare sorted cell types and isoform-level analysis. If you need to know whether T cells express the Treg isoform of *FOXP3* vs. a non-Treg isoform, Smart-seq2 is the tool.

## Other Notable Technologies

**MARS-seq** (Massively Parallel RNA Single-Cell Sequencing) uses liquid handling robotics for plate-based sorting with early barcoding, enabling higher throughput than Smart-seq2 while maintaining plate format. Developed for immune cell studies.

**Split-seq** (Parse Biosciences SPLiT-seq) uses combinatorial barcoding without requiring microfluidics: cells are fixed, then mRNA is barcoded through sequential rounds of split-pool ligation — each round adds one barcode component. The final cell identity is the combination of all barcodes. This approach scales to millions of cells and is cost-effective for very large experiments, though the fixed-cell protocol limits some downstream applications.

## Platform Comparison

| Platform | Throughput | Genes/Cell | Coverage | Cost/Cell | UMI | Isoforms |
|---|---|---|---|---|---|---|
| 10x Chromium | High (thousands) | ~2,000–5,000 | 3' end | ~$0.15–0.30 | Yes | No |
| Smart-seq2 | Low (hundreds) | ~5,000–8,000 | Full-length | ~$3–10 | No | Yes |
| MARS-seq | Medium | ~2,000–4,000 | 3' end | ~$0.50–1.00 | Yes | No |
| Split-seq | Very high (millions) | ~2,000–4,000 | 3' end | ~$0.01–0.05 | Yes | No |

## Why This Matters

The platform choice determines what biological questions the data can answer, which artifacts will appear in the analysis, and what computational tools are appropriate. Mismatched analysis assumptions (e.g., treating 3'-capture data as if it were full-length) produce incorrect results about isoform usage and gene body coverage. More importantly, the choice is irreversible: once you have sequenced with 10x, you cannot go back and ask isoform-level questions. The time to think carefully about platform selection is before the experiment is designed, not after the data is in hand.

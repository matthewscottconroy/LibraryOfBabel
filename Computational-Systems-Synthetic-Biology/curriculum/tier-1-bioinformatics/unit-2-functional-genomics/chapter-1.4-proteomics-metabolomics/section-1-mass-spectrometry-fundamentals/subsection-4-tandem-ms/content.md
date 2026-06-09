# Tandem Mass Spectrometry (MS/MS)

Here is the problem that motivates this entire subsection: you are looking at a peak at m/z 850.4 in your mass spectrum. Is it the peptide YGGFLR from your protein of interest? Or is it one of the dozens of other tryptic peptides from various proteins that happen to share that mass? Accurate mass helps — perhaps there is only one peptide with that exact molecular formula — but in a complex biological sample, co-eluting isobars are routine, and even a 1 ppm mass error window can still contain multiple candidate sequences.

What you need is a way to break the molecule apart in a controlled fashion and read the resulting fragments. If you fragment a specific peptide, the pieces spell out the sequence directly, because each fragment carries a defined portion of the original chain. This is tandem mass spectrometry — and it transforms the mass spectrometer from a weighing device into a molecular sequencer.

## The MS/MS Workflow

**Step 1 — Precursor selection**: The mass spectrometer's first analyzer (Q1 in a triple quadrupole, or the quadrupole in a Q-Orbitrap) selects a narrow m/z window (typically ±1–2 Da) centered on the precursor ion of interest, isolating it from co-eluting ions.

**Step 2 — Fragmentation**: The isolated precursor ions are accelerated into a collision cell filled with inert gas (nitrogen or argon). Collisions transfer kinetic energy to the ion, converting it to internal vibrational energy and causing bond cleavage. Common fragmentation methods:

- **CID (Collision-Induced Dissociation)**: Low energy (tens of eV). For peptides, cleavage occurs preferentially at the amide bonds of the peptide backbone, producing b and y ions (see below). This is the most widely used method.
- **HCD (Higher-energy Collisional Dissociation)**: A higher-energy variant of CID optimized for Orbitrap instruments. Produces more complete b/y ion series and enables detection of small reporter ions (e.g., TMT tags at m/z 126–134).
- **ETD (Electron Transfer Dissociation)**: Transfers electrons to multiply charged cations, causing backbone N-Cα bond cleavage to produce c and z• ions. Particularly useful for phosphopeptides (preserves labile phosphate groups) and longer peptides.

**Step 3 — Product ion detection**: Fragment ions are analyzed by the second mass analyzer (Q3, or Orbitrap in high-resolution instruments), producing the **MS/MS spectrum** — a plot of fragment ion intensity vs. m/z.

The choice of fragmentation method is not a purely technical matter — it connects directly to the biology you want to interrogate. CID and HCD work beautifully for standard tryptic peptides but strip labile post-translational modifications like phosphorylation during the collision process. ETD, by contrast, uses a fundamentally different fragmentation chemistry that preserves these modifications, making it the method of choice for phosphoproteomics, O-glycoproteomics, and intact protein sequencing.

## The b/y Ion Series for Peptides

Peptide backbone fragmentation at the amide C-N bond during CID produces two complementary series of ions:
- **b ions**: Contain the N-terminus of the peptide. The b1 ion is the first residue + H⁺ (as an oxazolone); b2 contains residues 1–2, etc.
- **y ions**: Contain the C-terminus of the peptide, terminating in –OH. The y1 ion is the last residue + H₂O + H⁺.

**Worked example** — Peptide PEPTIDE (7 residues, molecular formula):

Sequence: P-E-P-T-I-D-E

| Fragment | Residues | m/z (singly charged) |
|---|---|---|
| b2 | P-E | 227.1 |
| b3 | P-E-P | 324.2 |
| y1 | E | 148.1 |
| y2 | D-E | 263.1 |
| y3 | I-D-E | 376.2 |
| y5 | T-I-D-E | 574.3 |

Reading b ions from left to right (N → C terminus) or y ions from right to left (C → N terminus), each mass difference corresponds to a residue mass, directly spelling out the amino acid sequence.

What makes this so powerful is that you can read the sequence even without a reference spectrum. The mass difference between consecutive b ions (or consecutive y ions) directly gives you the residue mass: 113 Da = leucine or isoleucine; 131 Da = methionine; 163 Da = tyrosine. A well-resolved MS/MS spectrum from a high-resolution instrument is essentially a direct read-out of the peptide sequence, at least for short peptides with good fragmentation efficiency.

## Neutral Losses

Post-translational modifications produce characteristic **neutral losses** in MS/MS spectra. Phosphorylation is the most important example: a phosphoserine or phosphothreonine residue loses H₃PO₄ (98 Da) under CID conditions, producing a prominent [M−98]²⁺ ion from the precursor. This neutral loss is diagnostic for phosphopeptide identification (though site localization requires observing the phosphorylated b or y ions directly). For phosphotyrosine, the major neutral loss is HPO₃ (80 Da).

## Database Search Algorithms

Peptide identification from MS/MS spectra uses **database searching**: the measured spectrum is compared against predicted spectra for all peptides in a protein database. The algorithm:

1. For each peptide in the database (generated in silico by applying the enzyme cleavage rules, e.g., trypsin cleaves after K or R), generate the theoretical b/y ion series.
2. Score the match between theoretical and observed spectra.
3. Assign the best-matching peptide to each spectrum.

Common algorithms:
- **Sequest** (Thermo): Normalized dot product cross-correlation score (XCorr)
- **Mascot** (Matrix Science): Probability-based Mowse score
- **Andromeda** (MaxQuant): Combinatorial scoring used by MaxQuant for high-resolution data

In practice, a typical proteomics experiment generates 100,000 to 1,000,000 MS/MS spectra per run, and the database search must score each spectrum against potentially millions of candidate peptides. This is not a trivial computational problem — it is one of the reasons that bioinformatics is inseparable from modern proteomics. The algorithms must be fast enough to process gigabytes of data in hours, but sophisticated enough to distinguish true matches from high-scoring noise.

## FDR Control via Target-Decoy Strategy

No search algorithm is perfect, and incorrect peptide-spectrum matches (false positives) arise from spectral noise, co-fragmented peptides, or database mismatches. The **target-decoy strategy** controls the PSM-level FDR:

1. Construct a **decoy database** of "fake" peptide sequences (reversed or shuffled protein sequences).
2. Search the spectra against both target and decoy databases simultaneously.
3. The fraction of accepted decoy PSMs at any score threshold estimates the false positive rate among accepted target PSMs.
4. Set the score threshold to achieve FDR = 1% (for every 100 accepted PSMs, ~1 is expected to be a false match).

The 1% PSM-level FDR is the standard in proteomics; protein-level FDR (also 1%) is then applied separately, since a protein is reported as detected only if enough confidently identified peptides support it.

The elegance of the target-decoy strategy is that it requires no external validation set or hand-curated gold standard. The decoy database itself serves as a built-in control for false discovery, because the decoy sequences are guaranteed to be absent from your sample — any spectrum that "matches" a decoy is by definition a false positive. This self-referential quality is what has made the strategy the universal standard in proteomics, despite its conceptual simplicity.

## Why This Matters

Tandem MS is the fundamental identification engine of proteomics and metabolomics — without it, mass spectrometry could only measure masses, not identities; understanding the fragmentation logic, database search process, and FDR control enables critical evaluation of proteomics datasets and avoids misinterpreting search artifacts as biological signals. Every protein identification in a shotgun proteomics dataset was generated by exactly this pipeline: fragmentation, matching, and FDR control. When you examine a protein list and its supporting evidence, you are auditing a chain of probabilistic inferences, each of which has characteristic failure modes. Knowing those failure modes — incomplete b/y series for proline-containing peptides, neutral losses masking modifications, shared peptides confounding protein inference — is what separates a critical reader of proteomics data from a passive consumer.

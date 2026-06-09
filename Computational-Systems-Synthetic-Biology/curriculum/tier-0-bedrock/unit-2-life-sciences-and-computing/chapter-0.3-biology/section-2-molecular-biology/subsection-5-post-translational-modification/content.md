# Post-Translational Modification

You might expect that once a protein is synthesized, its identity is fixed — that the sequence of amino acids fully determines what the protein does. In many ways this is true, but reality is considerably richer. The proteome is vastly more complex than the genome. A cell with ~20,000 protein-coding genes can generate hundreds of thousands of distinct protein species through alternative splicing and, even more pervasively, through **post-translational modifications (PTMs)** — enzymatic chemical alterations of protein residues after synthesis. PTMs regulate activity, localization, stability, and protein-protein interactions, often serving as the "language" of cellular signaling. Understanding them quantitatively is essential for modeling signal transduction networks and interpreting proteomics data.

Think of it this way: if genes are words and proteins are sentences, PTMs are the punctuation marks, the italics, and the margin notes that determine how those sentences should be read in context.

## Phosphorylation: The Master Signaling Switch

**Phosphorylation** — addition of a phosphate group to Ser, Thr, or Tyr — is the most abundant and studied PTM. It is catalyzed by **kinases** (using ATP as phosphate donor) and reversed by **phosphatases**. The human genome encodes ~520 kinases (the "kinome") and ~150 phosphatases.

Phosphorylation changes protein function by:
1. **Electrostatic effects**: the phosphate group (pKa ~1, so doubly negatively charged at physiological pH) alters local charge, changing structure and interaction surfaces
2. **Conformational change**: phosphorylation of the activation loop of many kinases opens the catalytic cleft, activating the kinase
3. **Creating docking sites**: phospho-Ser/Thr is recognized by 14-3-3, FHA, WD40, and BRCT domains; phospho-Tyr is recognized by SH2 and PTB domains

**Example — receptor tyrosine kinase activation:** EGF binding to EGFR induces receptor dimerization and trans-autophosphorylation of cytoplasmic tyrosines. The phospho-Tyr sites recruit SH2 domain-containing proteins (Grb2, PLCγ, PI3K), initiating downstream cascades (RAS/MAPK, PI3K/AKT). Phosphorylation thus acts as a digital switch: 0 or 1 phosphate determines whether a docking site is present.

The kinetics can be modeled: for a substrate S being phosphorylated by kinase K and dephosphorylated by phosphatase P,

$$\frac{d[Sp]}{dt} = k_1 [K][S] - k_2 [P][Sp]$$

At steady state: $[Sp]/[S] = k_1[K] / (k_2[P])$ — the ratio depends on the balance between kinase and phosphatase activities.

## Ubiquitination: Protein Degradation Tags and Signaling Scaffolds

**Ubiquitin** is a highly conserved 76-amino acid protein that is covalently attached to substrate proteins via a three-enzyme cascade (E1 activating, E2 conjugating, E3 ligating). Ubiquitin is attached via its C-terminal Gly to a Lys residue in the substrate. Chains can be built on any of ubiquitin's 7 Lys residues or its N-terminal Met, with distinct chain types having distinct fates:

| Chain type | Linkage | Functional consequence |
|---|---|---|
| K48-linked chains | Lys48 | Proteasomal degradation |
| K63-linked chains | Lys63 | DNA damage signaling, endosomal sorting |
| K27-linked chains | Lys27 | Mitophagy |
| Linear chains (M1) | N-terminal Met | NF-κB signaling |
| Monoubiquitination | Single Ub | Histone H2A/H2B (transcription), receptor internalization |

The **26S proteasome** — a barrel-shaped complex of 19S regulatory and 20S catalytic subcomplexes — recognizes and degrades K48-polyubiquitinated proteins. The 20S barrel contains three proteolytic activities (trypsin-like, chymotrypsin-like, caspase-like) that cleave proteins into 7–25 aa peptides.

Protein half-life is determined by the balance between synthesis rate and degradation rate. In synthetic biology, **degrons** (short sequences that recruit E3 ligases) are engineered into proteins to tune degradation rate and reduce the time for protein levels to change after transcriptional reprogramming.

## Acetylation

**N-terminal acetylation** (by NATs, N-terminal acetyltransferases) modifies ~85% of human proteins cotranslationally, affecting protein stability (acetylated N-termini are not recognized by N-degron E3 ligases).

**Lys acetylation** (by HATs, histone acetyltransferases; reversed by HDACs, histone deacetylases) is the primary regulatory modification on histones. Acetylation of Lys neutralizes its positive charge, weakening electrostatic interaction with the negatively charged DNA backbone — this opens chromatin and correlates with transcriptional activation. H3K27ac marks active enhancers; H4K16ac is a general activating mark.

Beyond histones, Lys acetylation regulates metabolic enzymes: acetylation of ~20% of mitochondrial proteins regulates metabolic flux.

## Glycosylation: Extracellular Identity Codes

**Glycosylation** — covalent attachment of sugar moieties — is the most chemically diverse PTM. Over half of all human proteins are glycosylated.

- **N-linked glycosylation**: A pre-assembled oligosaccharide (Glc3Man9GlcNAc2) is transferred en bloc from a dolichol-PP carrier to the Asn in the sequon **Asn-X-Ser/Thr** (X ≠ Pro) in the ER lumen. This is subsequently trimmed and elaborated in the ER and Golgi. N-glycosylation is essential for protein folding quality control: **calnexin/calreticulin** in the ER bind monoglucosylated glycans and act as folding chaperones
- **O-linked glycosylation**: GalNAc (or other sugars) is added to Ser/Thr hydroxyl groups in the Golgi by a family of 20 GalNAc-Ts; no specific sequon

Glycosylation affects protein stability, trafficking (glycans are sorting signals), cell adhesion (selectins bind sialylated glycans), and immune recognition (ABO blood group antigens are glycan modifications of surface proteins).

## Other Key PTMs

- **Methylation**: Arg and Lys methylation by PRMTs/PKMTs (methyltransferases). Histone H3K4me3 marks active gene promoters; H3K27me3 marks Polycomb-repressed loci. Methylation is more stable than acetylation and phosphorylation.
- **Lipidation**: Myristoylation, palmitoylation, and GPI anchoring attach fatty acid/lipid moieties that tether proteins to membranes. Ras proteins require palmitoylation for membrane association and signaling.
- **Proteolytic cleavage**: Many proteins are synthesized as inactive precursors (zymogens). Trypsin is activated from trypsinogen by removal of a propeptide; insulin is cleaved from proinsulin; viral coat proteins are processed by viral proteases. Caspases (apoptosis executioners) are activated by proteolytic cleavage.

## Why This Matters for Computational Biology

PTMs are the primary language of signal transduction, and modeling signaling pathways requires explicit representation of phosphorylation/dephosphorylation cycles. Mass spectrometry-based proteomics identifies PTMs at proteome scale (phosphoproteomics, ubiquitinomics) — computational analysis requires tools like MaxQuant, Mascot, and specialized peptide fragment databases. In synthetic biology, protein degradation rate is a key design parameter: tagged degrons (LVA, AAV in bacteria; PEST sequences, auxin-inducible degrons in eukaryotes) are used to accelerate protein turnover. The interplay between ubiquitination and deubiquitination creates PTM-based logic gates that can be incorporated into synthetic signaling circuits. And stepping back: PTMs are the reason that correlations between mRNA and protein levels are often surprisingly weak. If you want to understand what a cell is actually doing, you need to know not just what proteins are present, but what modifications those proteins carry. That is the challenge and the opportunity of quantitative proteomics.

# Epigenetics

Consider two cells in your body with identical DNA sequences: a neuron and a liver cell. They occupy different organs, perform completely different functions, and will never exchange their identities — even though they contain the same genome. What determines which genes each cell expresses? Not the DNA sequence itself, but a layer of chemical information written on top of the DNA and around it: methylation marks on cytosines, modification patterns on the histone proteins that DNA is wrapped around, and the three-dimensional organization of chromatin in the nucleus. This information — heritable, functionally significant, and independent of DNA sequence — is epigenetic information.

**Epigenetics** refers to heritable changes in gene expression that do not alter the DNA sequence. Epigenetic information is encoded in patterns of DNA methylation, histone modifications, and higher-order chromatin structure. These marks determine which genes are accessible and expressed in each cell type — explaining how cells with identical genomes (a liver cell and a neuron) maintain profoundly different identities across many cell divisions. Epigenetics is also at the center of cancer biology, developmental biology, and the emerging field of epigenome editing.

## DNA Methylation

In mammals, DNA methylation occurs predominantly at the cytosine in **CpG dinucleotides** (C followed by G on the same strand), producing **5-methylcytosine (5mC)**, sometimes called the "fifth base." CpG methylation is symmetric: both strands are methylated at the CpG, enabling faithful inheritance through replication.

**Enzymes:**
- **DNMT3A/DNMT3B**: de novo methyltransferases; establish new methylation patterns during development
- **DNMT1**: maintenance methyltransferase; preferentially methylates hemimethylated CpG (produced after replication), copying the parental methylation pattern to the daughter strand
- **TET1/TET2/TET3**: iteratively oxidize 5mC → 5-hydroxymethylcytosine (5hmC) → 5-formylcytosine (5fC) → 5-carboxylcytosine (5caC); the latter two are excised by TDG and BER, completing active demethylation

**Gene regulation by methylation:**
- CpG-methylated promoters are generally silenced; methylation repels transcription factors and recruits methyl-CpG binding proteins (MeCP2, MBDs) which in turn recruit HDACs and condensing factors
- **CpG islands** — 200 bp to several kb stretches with elevated CpG content and high GC% — are found at ~70% of human gene promoters. CpG islands are normally unmethylated at active promoters. Aberrant hypermethylation of CpG island promoters silences tumor suppressor genes in cancer (e.g., *MLH1*, *CDKN2A/p16*).
- Conversely, gene bodies of actively transcribed genes are often methylated — this suppresses spurious internal promoters and is associated with Pol II elongation

**Genomic imprinting:** ~80–100 human genes are expressed from only one parental allele, determined by differential methylation at **imprint control regions (ICRs)** established in the germline. The two alleles carry different methylation patterns throughout development. Classic example: *IGF2* (insulin-like growth factor 2) is expressed only from the paternal allele; *H19* (a lncRNA) from the maternal allele; a shared ICR controls both via differential methylation.

## Histone Modifications

Histone tails (unstructured N-terminal extensions of each histone) are sites of extensive covalent modification. The combination of marks constitutes a **histone code** that is read by effector proteins ("readers") to alter chromatin state.

### Key Active Marks

**H3K4me3** (trimethylation of histone H3 Lys4) is the most reliable mark of **active gene promoters**. Written by the COMPASS/MLL methyltransferase complexes. Read by PHD finger domains in chromatin remodelers (e.g., NURF, CHD1) that facilitate transcription. In ENCODE chromatin state annotations, H3K4me3 is the primary mark defining active TSSs.

**H3K27ac** (acetylation of H3 Lys27) marks **active enhancers** and active promoters. Written by CBP/p300 acetyltransferases. Read by BRD4 and other bromodomain proteins that recruit P-TEFb (the kinase complex that phosphorylates RNAP II CTD Ser2 for productive elongation). The distinction between **typical enhancers** and **super-enhancers** (unusually large, highly acetylated enhancer clusters) is defined by H3K27ac ChIP-seq signal density.

**H3K36me3** marks the **bodies of transcribed genes** (written by SETD2 as RNAP II elongates). Recruits DNMT3B (maintaining gene body methylation) and LEDGF (retroviral integration targeting).

### Key Repressive Marks

**H3K27me3** (trimethylation of H3 Lys27) marks **Polycomb-repressed chromatin**. Written by PRC2 (EZH2/EZH1 as the catalytic subunit; SUZ12, EED as essential cofactors). Read by the chromodomain of CBX proteins in PRC1, which ubiquitinates H2AK119. Polycomb domains cover hundreds of developmental genes in undifferentiated cells; they are progressively removed as lineage-specific genes are activated during differentiation.

**H3K9me3** marks **constitutive heterochromatin** (pericentromeric repeats, retrotransposons). Written by SUV39H1/2 in mammals. Read by the HP1 chromodomain — HP1 binding HP1 creates a self-reinforcing heterochromatic state that can spread along the chromosome. This is a paradigm for **epigenetic memory**: once established, the H3K9me3-HP1 state can be inherited through replication because HP1 recruits SUV39H to re-methylate the newly deposited H3 after replication.

### Mark Inheritance Through Replication

After replication, parental nucleosomes are distributed randomly to both daughter strands (~50% each). The remaining gaps are filled with newly synthesized H3-H4 tetramers bearing no modifications. How are marks restored?

For Polycomb: EED (a component of PRC2) has a WD40 domain that binds H3K27me3 on parental nucleosomes, allosterically activating PRC2 to methylate adjacent newly incorporated H3 — a **reader-writer** mechanism for epigenetic inheritance.

For heterochromatin: HP1 on parental nucleosomes recruits SUV39H, which methylates adjacent new H3K9 → new HP1 binding → propagation.

The reader-writer mechanism is conceptually elegant: a mark begets the enzyme that writes more of the same mark. This positive feedback is what makes epigenetic states stable and heritable. But it also means that epigenetic states are not unconditionally permanent — disrupt the feedback strongly enough (for example, by inhibiting EZH2) and the state can be erased.

## Chromatin Remodelers

**ATP-dependent chromatin remodelers** use energy from ATP hydrolysis to move, eject, or restructure nucleosomes, controlling DNA accessibility:

| Family | Members | Function |
|---|---|---|
| SWI/SNF | BAF (mammalian), RSC (yeast) | Nucleosome sliding, ejection; tumor suppressors |
| ISWI | NURF, CHRAC, ACF | Nucleosome spacing and positioning |
| CHD | CHD1, NuRD | Deacetylation and compaction (NuRD), spacing (CHD1) |
| INO80 | INO80, SRCAP | Histone variant exchange (H2A.Z insertion) |

SMARCB1/SNF5 (SWI/SNF) is a tumor suppressor deleted in ~100% of pediatric rhabdoid tumors — chromatin remodeling is a critical anti-cancer mechanism.

## Why This Matters for Computational Biology

Epigenome profiling generates the most data-rich view of the regulatory genome. Standard pipelines include: FASTQ → alignment (Bowtie2/BWA) → peak calling (MACS2 for ChIP-seq/ATAC-seq) → differential binding (DiffBind, DESeq2) → motif analysis (HOMER, FIMO). Chromatin state models (ChromHMM) integrate multiple histone marks across cell types to define combinatorial regulatory states. Epigenome editing using **dCas9** fused to DNMT3A, TET1, p300, KRAB, or LSD1 allows targeted writing or erasing of epigenetic marks for synthetic biology applications — this requires knowing which mark to target at which genomic position to achieve the desired gene expression change. Single-cell ATAC-seq (scATAC-seq) profiles chromatin accessibility per cell, revealing cell-type heterogeneity and regulatory dynamics that bulk assays obscure.

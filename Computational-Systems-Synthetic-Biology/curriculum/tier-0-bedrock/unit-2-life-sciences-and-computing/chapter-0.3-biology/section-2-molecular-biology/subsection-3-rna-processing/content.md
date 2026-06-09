# RNA Processing

Here is one of the more counterintuitive facts about eukaryotic genomes: a typical human gene has nine exons interspersed among nine introns, and the introns are typically much longer than the exons they separate. When you ask why the genome is organized this way, the answer touches on some of the deepest questions in evolutionary biology. But the immediate consequence is practical: the raw transcript of a eukaryotic gene is not directly translatable. Before the ribosome can read it, the cell must cap the 5' end, remove every intron, stitch the exons together with near-perfect fidelity, and add a poly(A) tail to the 3' end. All of this happens in minutes, co-transcriptionally, in the nucleus.

In eukaryotes, the primary transcript (pre-mRNA) is not directly translated. Before reaching the ribosome, it must be capped, spliced, and polyadenylated — a suite of processing events that occur co-transcriptionally in the nucleus. These steps add regulatory layers unavailable to prokaryotes and expand proteomic complexity far beyond what the gene count alone would suggest. For computational biology, RNA processing is the source of the mRNA isoform landscape that RNA-seq tools must resolve.

## 5' Capping

Within seconds of transcription initiation, the 5' end of the nascent pre-mRNA receives a **7-methylguanosine (m7G) cap**. This is an unusual 5'→5' triphosphate linkage (not the standard 3'→5' phosphodiester), added by three enzymatic activities:

1. **RNA triphosphatase**: removes the γ-phosphate from the 5' triphosphate
2. **Guanylyl transferase**: adds GMP in a 5'→5' orientation
3. **Guanine-N7-methyltransferase**: methylates position N7 of the guanine

The m7G cap serves multiple functions:
- Protects mRNA from 5'→3' exonuclease degradation
- Is recognized by the **cap-binding complex (CBC)** for nuclear export
- Enables **cap-dependent translation initiation** via eIF4F recognition of eIF4E (a rate-limiting step in translation that is regulated by mTOR signaling)
- Marks the mRNA as self (vs. viral dsRNA, which is often uncapped) for innate immune discrimination

## 3' Polyadenylation

Near the 3' end of most eukaryotic mRNAs, a **polyadenylation signal** (`AAUAAA`, ~10–30 nt upstream of the cleavage site) is recognized by **CPSF (Cleavage and Polyadenylation Specificity Factor)**. Cleavage occurs ~15–30 nt downstream, and **poly(A) polymerase (PAP)** adds ~200 adenosine residues without a template.

The **poly(A) tail** is bound by **poly(A) binding protein (PABP)**, which:
- Protects the 3' end from exonuclease attack
- Participates in the closed-loop mRNA structure (PABP interacts with eIF4G, promoting translation)
- Is progressively removed during mRNA degradation (deadenylation is the first step in the major mRNA decay pathway)

mRNA half-life is largely determined by poly(A) tail length and the stability of the 3' UTR. ARE (AU-rich elements) in 3' UTRs recruit deadenylases, accelerating decay of unstable mRNAs (many cytokine and proto-oncogene mRNAs).

## Pre-mRNA Splicing

Most eukaryotic genes are split into **exons** (expressed sequences) and **introns** (intervening sequences that are excised). Human introns range from ~60 bp to >1 Mb; the average gene has ~9 introns. Intron removal is carried out by the **spliceosome**, a 5-snRNP complex (U1, U2, U4, U5, U6) with ~150 associated proteins ($\sim$3.5 MDa total).

**Splice site consensus sequences:**
- **5' splice site (donor)**: exon|GU (intron begins with GU)
- **Branch point**: ~20–50 nt upstream of the 3' splice site; consensus `YNYURAY`, the A being the branch point adenosine
- **3' splice site (acceptor)**: pyrimidine tract + AG|exon

**The two-step splicing mechanism:**
1. **Step 1 (branching)**: The 2'-OH of the branch point A attacks the 5' splice site, forming a lariat intermediate (2'→5' phosphodiester bond) and freeing the 5' exon with a 3'-OH
2. **Step 2 (exon ligation)**: The free 3'-OH of the 5' exon attacks the 3' splice site, joining the exons and releasing the lariat intron

The spliceosome assembles de novo on each intron, with U1 snRNA base-pairing with the 5' splice site and U2 with the branch point (the snRNAs provide the recognition specificity). The actual chemistry is catalyzed by RNA (the snRNAs), making the spliceosome a ribozyme. This adds the spliceosome to a growing list of biological machines whose catalytic cores are made of RNA — a fact that continues to support the RNA world hypothesis.

## Alternative Splicing

The same pre-mRNA can be spliced in multiple ways to produce distinct protein isoforms. Four main types of alternative splicing:

| Type | Description | Example |
|---|---|---|
| **Exon skipping** | One or more exons are omitted | Most common type; ~40% of alternative splicing events |
| **Alternative 5' splice site** | Different donor site used | Altered N-terminus |
| **Alternative 3' splice site** | Different acceptor site used | Altered protein domain |
| **Intron retention** | Intron not removed | Common in plants; regulated in mammals |
| **Mutually exclusive exons** | Either exon A or exon B, never both | *Dscam* in Drosophila: 38,016 possible isoforms |

The **RNA-binding proteins (RBPs)** that regulate splicing are key regulatory nodes. **SR proteins** (Ser/Arg-rich) generally activate splicing by binding exonic splicing enhancers (ESEs); **hnRNP proteins** often repress splicing. The balance between these factors is tissue-specific, creating tissue-specific splicing programs.

## RNA Editing

A small but important fraction of mRNAs undergo **RNA editing**: enzymatic alteration of individual nucleotides after transcription.

- **A-to-I editing** (adenosine to inosine, read as G): carried out by **ADAR** (Adenosine Deaminase Acting on RNA) enzymes. Double-stranded RNA structure near the editing site is required. Major substrates include glutamate receptor GluR-B (AMPA subunit) — A-to-I editing at the Q/R site changes a codon from CAG (Gln) to CIG (read as Arg), dramatically reducing Ca²⁺ permeability. Failure to edit is lethal in mice.
- **C-to-U editing**: **Apolipoprotein B (ApoB)** mRNA is edited in intestinal cells only, converting a CAA (Gln) codon to a UAA (stop codon), producing the shorter ApoB-48 protein rather than the full-length liver-expressed ApoB-100.

## Why This Matters for Computational Biology

RNA processing creates the isoform landscape that computational tools must navigate. Transcript quantification tools (Salmon, kallisto) map reads to transcript-level reference sequences; differential isoform usage between conditions requires tools like **DEXSeq** or **SUPPA2**. Splice junction reads in RNA-seq data (reads spanning exon-exon junctions) are the evidence base for de novo splice site discovery. Understanding U1 snRNA base-pairing and splice site scoring matrices (MaxEntScan) enables computational prediction of splicing effects of mutations — critical for clinical variant interpretation. Alternative polyadenylation (APA) creates 3' UTR variation affecting mRNA stability and subcellular localization, increasingly measured by PAPERCLIP, MISO, and DaPars tools. The central computational challenge of RNA-seq — turning read counts into gene expression estimates — is inseparable from this biology: every alignment decision, every transcript isoform assigned to a gene, every uncertain multi-mapping read is a direct consequence of the processing events described here.

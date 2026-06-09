# Genome Editing and CRISPR: The Canonical Papers

In 2012, Jennifer Doudna and Emmanuelle Charpentier published a paper in *Science* showing that a bacterial immune protein called Cas9 could be programmed with a short RNA molecule to cut any DNA sequence you chose, with remarkable precision. Within a year, four independent groups had used it to edit the genomes of human cells. Within seven years, the first patients were being treated in clinical trials for sickle cell disease. Within eight years, Doudna and Charpentier had won the Nobel Prize in Chemistry. Few areas of science have compressed the distance from basic discovery to clinical application as dramatically as CRISPR — and understanding how and why requires reading the papers themselves.

The papers in this section span from the biochemical characterization of the Cas9 nuclease in 2012 to the development of base editing and prime editing by 2019 — a seven-year arc in which the tool set for precise genome modification was transformed entirely. These papers are essential reading not only for their scientific content but as a case study in how a basic science discovery becomes a transformative technology.

---

## Historical Context: Before CRISPR

Targeted genome editing existed before CRISPR — it was just difficult enough to be effectively inaccessible to most labs. Zinc finger nucleases (ZFNs, developed in the 1990s) and transcription activator-like effector nucleases (TALENs, developed in the late 2000s) both used engineered protein domains to target specific DNA sequences and create double-strand breaks (DSBs). Both technologies were functional but expensive, slow to engineer, and technically demanding: each new target sequence required designing and validating a new protein from scratch, a process that could take months and tens of thousands of dollars. CRISPR transformed genome editing by replacing the protein-based DNA recognition system with a guide RNA — programmable in days by any molecular biology lab, at negligible cost. The barrier to entry collapsed almost overnight.

---

## 1. Jinek et al. (2012) — Cas9 Biochemistry

**Full citation:** Jinek, M., Chylinski, K., Fonfara, I., Hauer, M., Doudna, J. A., & Charpentier, E. (2012). A programmable dual-RNA–guided DNA endonuclease in adaptive bacterial immunity. *Science*, 337, 816–821.

**What it contributes:** This paper characterizes the biochemistry of Cas9 from *Streptococcus pyogenes* and demonstrates that it can be programmed to cleave specific DNA sequences using a single guide RNA (sgRNA). The CRISPR-Cas9 system had been known as a bacterial adaptive immune system for several years, but this paper established: (1) Cas9 creates blunt-ended double-strand breaks at positions determined by the guide RNA; (2) a two-component system (tracrRNA + crRNA) can be simplified to a single chimeric guide RNA (sgRNA); (3) the system can cleave plasmid and linear dsDNA in vitro with programmable specificity.

**The Nobel Prize:** Doudna and Charpentier were awarded the 2020 Nobel Prize in Chemistry for this work. Notably, the prize was for characterizing the biochemistry of Cas9, not for demonstrating genome editing in eukaryotic cells — the latter was achieved by multiple groups simultaneously in 2013.

**Approach:** Biochemical reconstitution of Cas9 with guide RNA and target DNA. Cleavage assays using gel electrophoresis. Systematic mutagenesis of guide RNA components to identify the minimal requirements for activity. Demonstration that different guide RNA sequences redirect Cas9 to different target sites.

**How to read it:** Figure 1 (schematic of CRISPR-Cas9 system) and Figure 3 (demonstration of guide-RNA-directed cleavage) are the core. Figure 5 (single-guide RNA design) is the most practically important — this is the design that every CRISPR experiment since 2012 has used.

**Why it remains important:** This is the foundational biochemistry paper for the entire CRISPR field. Understanding what Cas9 does mechanistically — where it cuts, what the PAM requirement is, how the guide RNA directs specificity — requires reading this paper. Every subsequent CRISPR tool (base editors, prime editors, CRISPRi, CRISPRa, Cas12, Cas13) is a modification or extension of the system described here.

---

## 2. Cong et al. (2013) — CRISPR in Human Cells

**Full citation:** Cong, L., Ran, F. A., Cox, D., Lin, S., Barretto, R., Habib, N., ... & Zhang, F. (2013). Multiplex genome engineering using CRISPR/Cas systems. *Science*, 339, 819–823.

**What it contributes:** Demonstrates that CRISPR-Cas9 can edit the genome of human cells (HEK-293T, K562, iPSCs) with high efficiency. Published simultaneously with Mali et al. (2013, Science) and a companion paper by Church's group, these papers established **eukaryotic genome editing with CRISPR as a routine laboratory procedure**. Cong et al. also demonstrated multiplexed editing (targeting multiple loci simultaneously with different guide RNAs) and the use of paired Cas9 nickases to reduce off-target editing.

**Approach:** Co-transfection of Cas9 expression plasmid and sgRNA into human cell lines. Editing efficiency measured by T7 endonuclease I assay (detects mismatches from NHEJ-mediated repair) and Surveyor nuclease assay. Targeted knockin demonstrated using a homology-directed repair (HDR) template.

**How to read it:** Figure 1 (the conceptual jump from prokaryotic to eukaryotic genome editing) and Figure 2 (efficiency data across multiple human cell lines and target sites) are the core. Read alongside Mali et al. (2013) — both appeared in the same issue of Science and should be understood as simultaneous, independent demonstrations of the same capability.

**Why it remains important:** This paper — together with Mali et al. — marks the moment at which CRISPR became a clinical and commercial technology. Every human therapeutic application of CRISPR, every transgenic animal model, every CRISPR screen traces its experimental lineage to these papers.

---

## 3. Komor et al. (2016) — Base Editing

**Full citation:** Komor, A. C., Kim, Y. B., Packer, M. S., Zuris, J. A., & Liu, D. R. (2016). Programmable editing of a target base in genomic DNA without double-stranded DNA cleavage. *Nature*, 533, 420–424.

**What it contributes:** Introduces the first **base editor** — a fusion protein combining a catalytically impaired Cas9 (nCas9, which nicks only one strand) with a cytidine deaminase enzyme (APOBEC1). The base editor converts cytosine to uracil (read as thymine) within a 5-base editing window in the target sequence, without creating a double-strand break. Because no DSB is created, the edit does not require cellular repair machinery and occurs at higher efficiency and with lower toxicity than conventional CRISPR-Cas9 cutting.

**Why no DSB is important:** CRISPR-Cas9 creates DSBs that are repaired either by non-homologous end joining (NHEJ, which is error-prone and usually creates indels — insertions or deletions) or homology-directed repair (HDR, which is precise but requires a repair template and occurs inefficiently in non-dividing cells). Base editors bypass the DSB entirely, achieving precise single-base changes at efficiencies often 10–100-fold higher than HDR.

**Approach:** Fusion of APOBEC1 deaminase to the N-terminus of nCas9 (D10A nickase), optimization of linker length, validation in HEK-293 cells. Characterization of the editing window (positions 4–8 of the protospacer), efficiency, and purity (fraction of edits that are the intended cytosine→thymine vs. indels).

**How to read it:** Figure 1 (base editor architecture) and Figure 2 (editing efficiency vs. window position) are the core. The supplementary data contains the systematic optimization of the deaminase-Cas9 fusion — the kind of optimization data that rarely appears in main text but is essential for understanding why the final design works.

**Subsequent developments:** The Liu lab subsequently developed (1) a second generation of CBEs (cytosine base editors, CBE3 and CBE4) with improved efficiency; (2) adenine base editors (ABEs, Gaudelli et al. 2017 Nature), which convert adenine to inosine (read as guanine); and (3) glycosylase base editors (GBEs) for other transitions. Together, these tools enable installation of any of the four transition mutations (C→T, G→A, A→G, T→C) at a target site without DSBs.

**Why it remains important:** Base editors are now clinical tools — several base editing therapies are in Phase 1/2 clinical trials for sickle cell disease, beta-thalassemia, and hypercholesterolemia. Understanding the design and limitations of base editors is essential for any scientist working in therapeutic genome editing.

---

## 4. Anzalone et al. (2019) — Prime Editing

**Full citation:** Anzalone, A. V., Randolph, P. B., Davis, J. R., Sousa, A. A., Koblan, L. W., Levy, J. M., ... & Liu, D. R. (2019). Search-and-replace genome editing without double-strand breaks or donor DNA. *Nature*, 576, 149–157.

**What it contributes:** Prime editing is a genome editing technology that can install **any single-nucleotide substitution, small insertion, or small deletion** at a target site without creating a DSB and without requiring a separate donor DNA template. A prime editor (PE) consists of nCas9 fused to an engineered reverse transcriptase. A prime editing guide RNA (pegRNA) carries both the targeting sequence and the desired edit as an RNA template. After nicking the target strand, the reverse transcriptase uses the pegRNA template to synthesize a new DNA strand incorporating the edit, which is then incorporated into the genome.

**Why "search and replace":** Unlike base editors (which are limited to transition mutations in a specific window) or conventional CRISPR (which requires DSB and HDR), prime editing can install any of the 12 possible point mutations, insertions of up to ~44 bp, and deletions of up to ~80 bp. It is the most versatile precision editing tool yet developed.

**Approach:** Systematic engineering of nCas9-reverse transcriptase fusions with different linkers and RT variants. pegRNA design optimization. Validation across 175 distinct edits in HEK-293T cells. Demonstration of correction of disease-causing mutations (sickle cell, Tay-Sachs disease mutations in cell culture).

**How to read it:** Figure 1 (mechanism diagram) is essential — prime editing is mechanistically more complex than base editing, and understanding the pegRNA design requires understanding all the steps. Figure 3 (comparison of PE2, PE3, and PE3b variants) shows the systematic optimization. Supplementary Figure 6 (comparison to base editing and HDR across matched targets) contextualizes the advantages and limitations.

**Current limitations:** Prime editing efficiency is lower than Cas9-NHEJ (typically 10–50% in dividing cells, less in non-dividing cells). Large insertions (>44 bp) are inefficient. The pegRNA design adds complexity. Ongoing development (PE4, PE5, PE6, epegRNAs) continues to improve efficiency.

**Why it remains important:** Prime editing substantially expands the addressable disease space for genome editing therapeutics. Approximately 89% of known pathogenic point mutations are theoretically addressable by prime editing (based on ClinVar classification). It is the most powerful demonstration of the Liu lab's vision of precision genome editing as chemistry — rational design of molecular tools that accomplish specific, defined chemistry at a programmed genomic location.

---

## Connecting the Papers: The CRISPR Tool Progression

**Jinek et al. (2012)** establishes the biochemistry and the sgRNA design → **Cong et al. (2013)** translates it to human cells → **Komor et al. (2016)** eliminates the need for DSBs for transition mutations → **Anzalone et al. (2019)** generalizes precision editing to any mutation type. Each paper addresses a limitation of its predecessor. The progression shows a research program that is both scientifically deep and medically motivated.

## Takeaway

The CRISPR canon tells the story of a basic science discovery — bacterial adaptive immunity — that was rapidly engineered into a versatile precision tool and is now entering clinical use at scale. Reading these papers in order demonstrates how each successive tool addresses the specific limitations of the previous one: efficiency, precision, mutation type coverage, off-target effects, and delivery. Understanding the mechanistic basis of each tool — not just the results — is what enables the critical thinking needed to evaluate new CRISPR variants as they continue to appear in the literature at a remarkable pace. The story is still unfolding; these four papers give you the foundation to follow it.

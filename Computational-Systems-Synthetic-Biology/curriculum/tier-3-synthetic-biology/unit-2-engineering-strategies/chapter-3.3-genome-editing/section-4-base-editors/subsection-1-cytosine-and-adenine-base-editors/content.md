# Cytosine and Adenine Base Editors

About 58% of the pathogenic point mutations catalogued in ClinVar are transition mutations — cytosine flipping to thymine, adenine flipping to guanine, or their reverse complements. For years, correcting these mutations with CRISPR required introducing a double-strand break, providing a repair template, and coaxing the cell into using the HDR pathway — a process that was inefficient even in dividing cells and essentially impossible in neurons or cardiomyocytes. In 2016, David Liu's lab asked a different question: what if you could simply reach into the R-loop, find the mismatched base, and chemically convert it without cutting the DNA at all? The answer was base editing — a class of tools that achieves single-nucleotide precision, in non-dividing cells, at efficiencies that HDR could never approach.

Base editors achieve single-nucleotide changes in the genome without introducing double-strand breaks and without requiring a homology-directed repair template. They are more efficient than HDR for simple transition mutations (C→T or A→G), work in non-dividing cells, and have been applied to correct pathogenic point mutations in animal models. Understanding their mechanism is essential for designing experiments that require precise sequence alterations.

## The Core Concept

Both cytosine and adenine base editors consist of the same three-component architecture:
1. **nCas9 (D10A nickase)**: binds the target DNA via sgRNA and nicks the non-edited strand; does not create a DSB
2. **Deaminase domain**: catalyzes a chemical modification on the single-stranded DNA in the R-loop
3. **Uracil glycosylase inhibitor (UGI, CBE only)**: blocks base excision repair of the deaminated base

The key substrate for deamination is the **single-stranded DNA bubble** created when nCas9 forms an R-loop. In this bubble, the non-target strand remains single-stranded and is accessible to deaminase enzymes that would not act on duplex DNA.

## Cytosine Base Editor (CBE): C → T

### Mechanism

**Deaminase**: APOBEC1 (apolipoprotein B mRNA editing catalytic subunit 1), fused N-terminally to nCas9.

**Chemistry**:
$$\text{Cytosine} \xrightarrow{\text{APOBEC1}} \text{Uracil} \xrightarrow{\text{replication}} \text{Thymine}$$

APOBEC1 hydrolytically deaminates cytosine (C) to uracil (U) at the C4 position. Uracil is read by the replication machinery as thymine, so after one round of cell division, the edited strand contains a T:A base pair instead of the original C:G pair.

**Without UGI**: cellular uracil-DNA glycosylase (UDG/UNG) removes the uracil, creating an abasic site. This site is then repaired by base excision repair using the complementary strand (which still contains G), reverting to C:G. UGI blocks UDG, preventing this reversion.

**Nicking**: nCas9 D10A nicks the non-edited strand (the strand containing G opposite the C being edited). This signals the cell's mismatch repair system to use the U-containing strand as the repair template, increasing the efficiency with which U is permanently converted to T.

### Editing Window

The deaminase domain is tethered to Cas9 by a flexible linker, restricting its access to a specific window of the single-stranded DNA in the R-loop. For most CBE designs, this window spans **positions 4–8** of the protospacer, counting from the PAM-distal end (i.e., the 5′ end of the spacer in the guide RNA).

$$\text{Spacer: 5′-}[1][2][3]\underbrace{[4][5][6][7][8]}_{\text{editing window}}[9]...[20]\text{-3′ PAM}$$

Any cytosine within positions 4–8 is a potential editing target. **Bystander editing** occurs when multiple cytosines fall within the window — all are potential substrates for deamination, regardless of whether they are the target.

### CBE Variants

- **BE3** (original): nCas9-APOBEC1-UGI; ~15–40% editing efficiency
- **BE4**: adds second UGI copy; reduces indel frequency; ~25–55% efficiency
- **AncBE4max**: codon-optimized, nuclear-localized; current high-efficiency standard

## Adenine Base Editor (ABE): A → G

Cytosines can be deaminated naturally (APOBEC enzymes exist), but adenosine deaminases that act on DNA do not exist in nature. Liu lab solved this by **evolving an RNA adenosine deaminase (TadA) to act on DNA** through directed evolution — over 7 rounds of phage-assisted evolution, producing ABE7.10 and subsequently ABE8e.

### Mechanism

**Chemistry**:
$$\text{Adenine} \xrightarrow{\text{TadA}} \text{Inosine} \xrightarrow{\text{replication}} \text{Guanine}$$

TadA deaminates adenine (A) to inosine (I). Inosine is decoded as guanine by the replication machinery. After cell division: A:T → G:C.

**No UGI needed**: there is no inosine-specific glycosylase in mammalian cells that would revert this change. The nicking of the complementary strand guides mismatch repair toward G incorporation.

### Editing Window

For ABEs, the editing window is slightly PAM-proximal compared to CBEs: approximately **positions 4–7** of the protospacer (from the 5′/PAM-distal end). Window position can vary by 1–2 positions depending on the specific ABE variant and linker length.

### ABE Variants

- **ABE7.10**: first generation; 50 evolution cycles of TadA; editing efficiency ~35–65%
- **ABE8e**: adenosine deaminase with further evolved activity; efficiencies up to 80%; also has reduced off-target RNA editing

## Applications of Base Editing

Base editors are particularly valuable for:

**Correcting pathogenic SNPs**: approximately 58% of known human pathogenic point mutations are transition mutations (C→T or A→G, or their complements G→A, T→C), directly addressable by CBE or ABE without HDR.

*Example*: Sickle cell disease is caused by a single A→T transversion in HBB (hemoglobin beta, Glu6Val). This specific mutation is a transversion and not directly correctable by base editors; however, CBE can install a compensatory mutation to create hemoglobin variants (like HbF re-activation via BCL11A editing).

*Example*: Progeria (HGPS) is caused by a C→T mutation in LMNA, which activates a cryptic splice site. CBE has been used in mouse models to correct this mutation with ~90% efficiency in affected tissues.

**Creating stop codons**: CBE can convert a CAA (Gln), CAG (Gln), CGA (Arg), or TGG (Trp) codon to a stop codon (TAA, TAG, TGA) by deaminating a single cytosine. Used to introduce loss-of-function mutations without relying on NHEJ.

**Engineering phenotypes**: modify active-site residues, phosphorylation sites, or protein-protein interaction surfaces at specific amino acid positions.

## Limitations

**Bystander editing**: all cytosines (or adenines) in the editing window are potential targets. If multiple C's fall within positions 4–8 and only one is desired, bystander edits at the others are likely. Strategies to manage bystander editing: choose guides that position only the target C within the window; use narrow-window base editors (e.g., base editors with shorter linkers).

**Window constraints**: the fixed window position means some target bases cannot be reached. If the target C is at position 12, no current standard CBE can reach it.

**Genome-wide deamination**: base editors can cause off-target edits in two categories:
1. Cas9 guide-dependent off-target sites (same as standard Cas9)
2. Guide-independent, transcriptome-wide RNA deamination by the deaminase domain

ABE8e has reduced RNA off-target activity compared to earlier ABE versions. SECURE-CBE uses engineered APOBEC1 variants with lower intrinsic activity to reduce guide-independent DNA deamination.

**Only transitions**: base editors convert C→T (CBE) or A→G (ABE). They cannot install transversions (C→A, C→G, T→A, etc.) or insertions/deletions. Prime editing addresses this limitation.

## Delivery

Base editors are larger than SpCas9 (~5.6 kb for ABE8e-nCas9 fusion), exceeding single-AAV packaging limits. Delivery strategies:
- **Dual AAV**: split intein system delivers N-terminal and C-terminal halves separately; they splice together in the cell
- **LNP (lipid nanoparticle)**: mRNA encoding base editor + sgRNA; highly efficient for liver delivery; used in ongoing clinical trials
- **RNP delivery**: base editor protein + sgRNA electroporation for primary cells

## Why This Matters

Base editors expanded the scope of precise genome editing from a niche capability requiring optimized HDR conditions to a broadly applicable tool achieving 40–80% correction efficiency even in non-dividing primary cells. The demonstration that an evolved RNA deaminase can be converted into a DNA deaminase (ABE) illustrates the power of directed evolution to solve problems where no natural enzyme exists. For medicine, base editors are now in clinical trials for multiple genetic diseases — and the growing list of pathogenic SNPs correctable by this approach makes them among the most impactful biotechnologies developed in the 2010s.

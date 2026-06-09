# CRISPR-Cas9: Components

Buried in a 1987 paper on the *Escherichia coli* iap gene, Yoshizumi Ishino and colleagues noted something odd at the end of their sequence: a series of short repeated elements separated by equally short, apparently random spacers. They flagged it as a curiosity. Twenty years later, those spacers would turn out to be molecular memories — fragments of viral DNA captured by bacteria as part of an immune system nobody knew existed. And twenty-five years after that, the machinery behind those spacers would become the most powerful genome editing tool in the history of molecular biology.

To understand how CRISPR-Cas9 works — and to design experiments that use it wisely — you need to understand its components: what each piece does, how each one constrains your design choices, and why the system is built the way it is. The PAM requirement is not just a nuisance; it reflects how Cas9 physically reads DNA. The seed region asymmetry is not an arbitrary rule; it emerges from thermodynamics. Every design decision in a CRISPR experiment flows from these molecular details.

## Historical Origin and Adaptation

CRISPR (Clustered Regularly Interspaced Short Palindromic Repeats) arrays were first observed in *Escherichia coli* sequences in 1987 and their immune function confirmed in 2007. The landmark 2012 paper by Jinek, Doudna, and Charpentier reconstituted the *Streptococcus pyogenes* Cas9 system in vitro and demonstrated programmable, RNA-directed DNA cleavage. Within a year, Zhang and Church independently applied the system to mammalian cell genome editing. The speed of translation from biochemistry to genome editing technology is unparalleled in the history of molecular biology.

## The Cas9 Protein

**Cas9** from *Streptococcus pyogenes* (SpCas9) is a 1,368-amino-acid, 158-kDa bilobed endonuclease. Its two lobes are:

- **Recognition (REC) lobe**: binds the guide RNA scaffold and mediates initial DNA surveillance
- **Nuclease (NUC) lobe**: contains the two catalytic domains responsible for cutting each strand of the target DNA

### The Two Catalytic Domains

The NUC lobe contains two structurally distinct endonuclease domains:

**HNH domain**: cleaves the **strand complementary** to the spacer sequence in the guide RNA (called the target strand or non-template strand in the context of transcription). HNH is a single-strand nickase; its catalytic residue is H840.

**RuvC domain**: cleaves the **non-complementary strand** (the strand not base-paired to the guide RNA, often called the non-target strand). RuvC is also a single-strand nickase; its active-site residues are D10, E762, and D986.

Together, coordinated cleavage by HNH and RuvC produces a **blunt-ended double-strand break (DSB)** located 3 bp upstream of the PAM sequence. Either domain can be inactivated individually by point mutation to produce a **nickase** (nCas9), which cuts only one strand — an important variant for base editing and prime editing.

### PAM Recognition

The **PAM (Protospacer Adjacent Motif)** is a short DNA sequence required for Cas9 binding. SpCas9 recognizes the sequence **5′-NGG-3′** on the non-target strand (equivalently, CCN on the target strand). The PAM is read by the PAM-interacting (PI) domain at the C-terminus of Cas9.

Critically, the PAM is not part of the guide RNA — it is a fixed sequence requirement in the genomic DNA. This means:
- Not every genomic position can be targeted by SpCas9; a target site must have NGG within 1–2 bp of the desired cut
- The PAM determines which strand Cas9 searches: the protospacer is always 5′ of the NGG on the non-target strand
- SpCas9 has an NGG PAM frequency of approximately 1 per 8 bp in a random sequence, making most genomic regions accessible within a ~20 bp window

## The Guide RNA

The native CRISPR system uses a two-component RNA: a **crRNA (CRISPR RNA)** containing the spacer sequence, and a **tracrRNA (trans-activating crRNA)** providing the scaffold that Cas9 binds. For experimental use, Jinek et al. fused these into a single **sgRNA (single guide RNA)** of approximately 100 nucleotides — the standard format in virtually all genome editing applications.

### sgRNA Architecture

```
5'-[20 nt spacer]--[sgRNA scaffold]------3'
    NNNNNNNNNNNNNNNNNNNN|GTTTTAGAGCTAGAAATAGCAAGTTAAAATAAGGCTAGTCC...
    └──────────────────┘└────────────────────────────────────────────┘
       Protospacer          tracrRNA scaffold (binds Cas9)
```

**The spacer (protospacer region)**: 20 nucleotides complementary to the target DNA. This sequence determines where in the genome Cas9 will bind. It is the only component that changes between experiments — the scaffold is constant.

**The scaffold**: ~80 nucleotides forming a characteristic stem-loop structure. Key features include:
- Repeat:anti-repeat duplex: the junction between crRNA and tracrRNA
- Three stem-loops in the tracrRNA portion (nexus, hairpin 1, hairpin 2)
- The 3′ end of the scaffold is critical for Cas9 binding; truncations here abolish activity

### The Seed Region

Not all 20 nucleotides of the spacer contribute equally to target specificity. The **seed region** — approximately bases 1–12 counting from the PAM-proximal end of the protospacer (i.e., the 3′ end of the spacer sequence) — is most critical for accurate target recognition. Mismatches in the seed region strongly reduce cleavage efficiency, while mismatches in bases 13–20 (PAM-distal) are better tolerated. This asymmetry has direct implications for off-target activity, discussed in section 3.3.2.

## The PAM and R-Loop Formation

Before cleavage, Cas9 must locate the correct target through a process called **R-loop formation**:

1. Cas9-sgRNA complex diffuses along DNA, interrogating NGG sequences
2. PAM binding induces a local conformational change in Cas9, melting the adjacent DNA duplex
3. The sgRNA spacer invades and base-pairs with the complementary strand, forming the **R-loop** (RNA-DNA hybrid + displaced ssDNA)
4. If the full spacer is complementary, Cas9 undergoes an allosteric conformational change that activates both HNH and RuvC
5. Coordinated cleavage occurs

The requirement for both PAM recognition and spacer complementarity provides a two-factor authentication mechanism for specificity, though as discussed in section 3.3.2, it is not infallible.

## Component Delivery Formats

The Cas9 protein and guide RNA can be delivered to cells in three formats, each with different trade-offs:

**Plasmid DNA**: Cas9 CDS + sgRNA expression cassette encoded on a single plasmid. Persistent Cas9 expression increases editing efficiency but prolongs Cas9 activity and raises off-target risk. Standard format for initial experiments.

**mRNA + sgRNA**: Cas9 protein expressed from mRNA (transient); guide RNA delivered as in vitro-transcribed RNA. Faster clearance than plasmid; no integration risk; used in therapeutic contexts.

**Ribonucleoprotein (RNP)**: Pre-assembled Cas9 protein + sgRNA complex delivered directly by electroporation or lipid nanoparticles. Fastest clearance (hours vs. days), lowest off-target activity, most efficient for primary cells. RNP is now the preferred format for therapeutic genome editing.

## Why This Matters

Every design decision in a CRISPR experiment flows from understanding these components. The PAM requirement constrains target site selection. The seed region asymmetry motivates guide design rules and off-target prediction algorithms. The two-domain cleavage architecture enables the nickase variants used in base editing and prime editing. And the modular guide RNA design — where only the 20-nt spacer changes — is what makes CRISPR programmable at the scale of genome-wide libraries containing tens of thousands of distinct guides. The component-level understanding developed in this section is the foundation for every advanced editing strategy in the sections that follow.

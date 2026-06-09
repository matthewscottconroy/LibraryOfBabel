# DNA Repair Pathways After CRISPR Cleavage

Here is something that surprises most people when they first encounter it: the double-strand break Cas9 introduces is not, by itself, the edit. The break is just a wound. What matters is how the cell heals that wound — and cells have several ways of doing so, each producing a different outcome. If you want to knock out a gene, you need one repair pathway. If you want to install a precise single-nucleotide correction, you need another. And if you're working in a neuron rather than a dividing cell line, you may not have access to the pathway you want at all. Understanding DNA repair is not background material — it is what lets you design an editing strategy that achieves a specific genetic outcome rather than a random one.

The Cas9-induced double-strand break (DSB) is only the beginning of the editing event. What happens next — which DNA repair pathway the cell uses — determines the outcome: gene disruption, precise correction, or insertion. Understanding these pathways at a mechanistic level is essential for designing editing strategies that achieve specific genetic outcomes.

## The Three Major DSB Repair Pathways

Cells have multiple competing mechanisms for repairing DSBs. The choice between pathways is governed by cell cycle phase, cell type, and the availability of repair templates.

### 1. Non-Homologous End Joining (NHEJ)

**NHEJ** is the dominant DSB repair pathway in most mammalian cell types. It rejoins broken DNA ends without requiring a homologous template, making it fast but imprecise.

**Mechanism:**
1. Ku70/Ku80 heterodimer binds and protects DSB ends within seconds
2. DNA-PKcs is recruited and activated, phosphorylating downstream factors
3. Artemis nuclease may process the ends (removing damaged nucleotides or overhangs)
4. DNA Polymerase μ or λ may fill in or extend ends, introducing insertions
5. XRCC4/DNA Ligase IV complex ligates the ends

The end-processing steps (Artemis trimming, polymerase extension) introduce stochastic insertions and deletions (**indels**). Indel frequency and size distribution depend on the specific DNA sequence context, but typical outcomes are:
- 1-bp insertions (very common, ~50% of events): the templated insertion of one nucleotide
- Small deletions (1–10 bp): end resection before ligation
- Large deletions (rare but detectable): resection exposing microhomology sequences

**Genetic consequences of NHEJ:**
Indels in the coding sequence of a gene most commonly cause frameshift mutations. A 1-bp insertion or deletion shifts the reading frame, creating a premature stop codon in approximately 2/3 of cases (since 2 of 3 possible frameshifts terminate within a few codons). This is the basis for **gene knockout** by CRISPR: design a guide targeting an early exon, allow NHEJ repair, screen for frameshifted clones.

The InDelphi computational tool (Shen et al. 2018) predicts the distribution of indel outcomes from a given cut site sequence with high accuracy, enabling rational design of NHEJ outcomes.

### 2. Homology-Directed Repair (HDR)

**HDR** uses a homologous DNA sequence as a template to repair the break faithfully. In the context of CRISPR editing, an exogenous repair template can be supplied to direct the outcome.

**Mechanism:**
1. CtIP nuclease resects the DSB ends, generating 3′ single-stranded overhangs (this step requires CDK activity, restricting HDR to S and G2 phases)
2. RPA coats ssDNA; BRCA2 loads Rad51
3. Rad51-coated ssDNA invades a homologous duplex (the repair template), forming a displacement loop (D-loop)
4. DNA synthesis copies the template
5. The D-loop is resolved; both strands are ligated

**Repair template formats:**
- **ssODN (single-stranded oligodeoxynucleotide)**: 90–200 nt, with homology arms flanking the desired edit. Used for point mutations, small insertions (<20 bp).
- **dsDNA donor (plasmid or PCR product)**: larger templates for insertions of gene-sized sequences. Requires longer homology arms (200 bp – 1 kb).
- **rAAV (recombinant adeno-associated virus)**: efficient ssDNA template delivery; widely used for therapeutic editing.

**Cell cycle restriction**: HDR is essentially absent in G1 and post-mitotic cells because CtIP resection requires CDK-mediated phosphorylation, which only occurs in S and G2. This fundamentally limits HDR efficiency in neurons, muscle cells, and other terminally differentiated cells. In actively dividing cells (cancer lines, iPSCs), HDR efficiency is typically **1–10%** of total editing events.

**Worked example**: To introduce a specific point mutation changing Ala→Val at codon 72 of EGFR in HEK293 cells:
1. Design sgRNA targeting within 20 bp of codon 72
2. Synthesize 150-nt ssODN: 60-nt left homology arm + desired mutation + 60-nt right homology arm
3. Also include a silent mutation in the PAM or seed region to prevent re-cleavage of the edited allele
4. Deliver RNP + ssODN by electroporation
5. Expected HDR efficiency: 3–8% of alleles; NHEJ will also occur

### 3. Microhomology-Mediated End Joining (MMEJ)

**MMEJ** is an alternative to NHEJ that uses short regions of sequence homology (2–25 bp) flanking the cut site to join ends in a predictable fashion. Unlike NHEJ, MMEJ outcomes are more stereotyped and can be predicted from the sequences flanking the cut.

**Mechanism:**
1. End resection exposes sequences flanking the break
2. Short microhomology sequences (2–25 bp) on either side of the break anneal to each other
3. The sequences between the microhomology regions are deleted
4. Ligation completes the join

MMEJ always produces deletions, but because the deletion size is determined by the location of the flanking microhomologies, the outcomes are less random than NHEJ. This **predictability** has been exploited for precision deletion engineering: by designing guides adjacent to synthetic flanking microhomologies in a donor construct, specific deletions can be created with high frequency.

The **InDelphi** tool also models MMEJ outcomes. MMEJ frequency is cell-type dependent and generally lower than NHEJ.

## Pathway Choice Determinants

| Pathway | Cell Cycle | Template | Outcome | Typical Frequency |
|---------|-----------|----------|---------|-------------------|
| NHEJ | Any | None | Stochastic indels | 30–80% of alleles |
| HDR | S/G2 only | Required | Precise edit | 1–10% of alleles |
| MMEJ | S/G2 preferred | None (uses flanking seq.) | Predictable deletion | 2–15% of alleles |

Strategies to increase HDR frequency include:
- **Cell cycle synchronization**: arrest cells in S/G2 with nocodazole or other agents before editing
- **NHEJ inhibition**: small molecule inhibitors of DNA-PKcs (M3814, NU7441) block NHEJ, forcing repair into HDR pathway
- **HDR-enhancing small molecules**: RS-1 (Rad51 activator) reported to increase HDR
- **Optimizing donor template**: ssODN generally more efficient than plasmid for small edits; asymmetric homology arms (36 nt/91 nt) may improve efficiency
- **Timing delivery**: electroporate RNP in late S/G2 (use cell cycle markers)

## Reading Editing Outcomes

Distinguishing NHEJ, HDR, and unedited alleles requires sequencing. Common approaches:

**ICE (Inference of CRISPR Edits)**: Decompose Sanger sequencing traces from an edited population into a mixture of wild-type and mutant sequences. Estimates editing frequency and indel distribution without next-generation sequencing.

**Amplicon deep sequencing**: PCR amplify the target region; sequence on NGS platform. Each read is classified as WT, NHEJ indel, or HDR-corrected. Provides full indel distribution and HDR frequency at single-allele resolution.

**Allele-specific PCR**: use primers that distinguish edited and unedited alleles; quick screening of clonal populations.

## Why This Matters

The choice of repair pathway is not a detail — it determines the fundamental nature of the editing outcome. Gene disruption (NHEJ-mediated knockout) and precise correction (HDR-mediated knock-in) require different experimental strategies and succeed in different cell contexts. Therapeutic editing for monogenic diseases almost always requires HDR to restore the correct sequence, which is why HDR efficiency in non-dividing primary cells remains one of the central challenges in the field. Understanding the pathway mechanisms is what enables rational strategies to overcome these limitations — whether through cell cycle manipulation, NHEJ inhibition, or transitioning to base editors and prime editors that achieve precision without requiring HDR at all.

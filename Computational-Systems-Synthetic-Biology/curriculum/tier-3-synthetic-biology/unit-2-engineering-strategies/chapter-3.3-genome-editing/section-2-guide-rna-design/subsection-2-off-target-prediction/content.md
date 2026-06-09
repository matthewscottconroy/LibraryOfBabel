# Off-Target Prediction: Computational and Experimental Approaches

In 2013, a paper in Nature Biotechnology caused alarm across the young CRISPR field. Researchers had used Cas9 to edit a gene in mouse embryonic stem cells and, using whole-genome sequencing, identified mutations scattered across the genome that had not been there before. The CRISPR system — celebrated for its precision — was also cutting where it wasn't supposed to. That finding launched an entire subfield of CRISPR research devoted to a single question: how do you find off-target cuts before they cause problems, and how do you know when you've found them all?

Off-target cleavage — Cas9 cutting genomic sites other than the intended target — is one of the primary safety concerns in therapeutic editing and a source of confounding artifacts in research applications. Predicting, detecting, and mitigating off-target activity requires both computational prediction tools and experimental validation methods.

## The Nature of Off-Target Sites

Off-target sites arise because Cas9 tolerates mismatches between the guide RNA and genomic DNA. The tolerance is asymmetric: mismatches in the PAM-distal end of the spacer (positions 13–20) are substantially better tolerated than mismatches in the seed region (positions 1–12, PAM-proximal).

Key empirical observations:
- SpCas9 can cleave sites with up to 5 mismatches in some contexts
- Bulges (insertions or deletions in the RNA:DNA hybrid) at 1–2 positions are also tolerated
- Off-target activity is guide-specific: some guides are highly specific, others have dozens of off-target sites
- Off-target frequency at any given site is typically 0.01–1% of on-target frequency, but can be higher

## The CFD Score: Computational Off-Target Prediction

The **Cutting Frequency Determination (CFD) score** (Doench et al. 2016) is the most widely used computational metric for ranking off-target sites. It is derived from systematic measurements of how each type of mismatch (substitution at each of the 20 positions, for each possible substitution type) affects SpCas9 cleavage:

$$\text{CFD score} = \prod_{i=1}^{20} w_i^{mm} \times w_{PAM}$$

Where $w_i^{mm}$ is the weight for a mismatch of type $mm$ at position $i$, learned from experimental data, and $w_{PAM}$ is the PAM compatibility weight (1.0 for NGG, 0.107 for NAG, etc.). An on-target site has CFD = 1.0. A site with CFD > 0.2 is generally considered a meaningful off-target risk.

CFD scores are precomputed in CRISPOR and other tools for all genomic sites within a defined mismatch threshold. This enables rapid risk ranking without performing any experiments.

## Cas-OFFinder: Exhaustive Sequence Search

**Cas-OFFinder** (Bae et al. 2014) performs exhaustive enumeration of all genomic sequences that match the guide RNA with up to $n$ mismatches and/or bulges, for user-specified $n$.

Algorithm:
1. Take the 20-nt spacer + PAM as a query
2. Search the entire genome sequence using GPU-accelerated pattern matching
3. Report all matches with ≤ $n$ mismatches (and optionally RNA or DNA bulges)
4. Output: genome coordinates, mismatched positions, mismatch types

```python
# Example Cas-OFFinder invocation (conceptual)
# Find all sites with NGG PAM and up to 3 mismatches
cas_offinder_query = {
    'spacer': 'GGTGGCGTAGGCAAGAGTGCC',
    'PAM': 'NGG',
    'max_mismatches': 3,
    'genome': 'GRCh38',
    'include_bulges': True
}
# Returns: list of (chromosome, position, strand, mismatch_count, mismatch_positions)
```

Cas-OFFinder identifies candidate off-target sites but cannot predict which sites are actually cleaved in cells, because chromatin accessibility and local sequence context also influence cleavage.

## Chromatin Accessibility and Off-Target Activity

A critical insight is that off-target cleavage in cells is strongly influenced by chromatin state. Cas9 accesses closed chromatin inefficiently — nucleosome-occluded sites are cleaved rarely even if they have perfect sequence complementarity. This means:

- Sequence-based prediction tools overestimate the number of relevant off-target sites
- ATAC-seq or FAIRE-seq data (measuring chromatin accessibility) can be integrated to refine predictions
- The tool **CRISPR-ML** incorporates chromatin accessibility features from ENCODE data to produce more accurate cell-type-specific off-target predictions

## Experimental Off-Target Validation

Computational predictions identify candidate sites but cannot replace experimental validation. Three gold-standard experimental methods exist:

### GUIDE-seq (Tsai et al. 2015)

**Principle**: double-stranded oligonucleotide (dsODN) tags are integrated at DSB sites genome-wide. Sites with integrated tags are identified by sequencing.

**Protocol**:
1. Deliver Cas9 + sgRNA + dsODN (blunt-ended, 34 bp)
2. Wait for DSBs to occur; dsODN integrates at breaks via NHEJ
3. Extract genomic DNA; fragment; ligate adaptors
4. Amplify with one primer from dsODN, one from adaptor (directional amplification)
5. NGS sequencing identifies all genomic sites where dsODN integrated

**Sensitivity**: detects off-target sites cleaved at frequencies as low as 0.1% of on-target rate

**Limitation**: dsODN integration is itself mutagenic; requires careful controls; may miss very low-frequency sites

### CIRCLE-seq (Tsai et al. 2017)

**Principle**: circularized genomic DNA is cleaved in vitro by Cas9. Only linearized (cleaved) molecules are amplified. This enriches for cleavage sites over uncleaved background.

**Advantage over GUIDE-seq**: in vitro, so not limited by delivery efficiency or chromatin effects. Can detect virtually all sequence-accessible Cas9 off-target sites regardless of frequency.

**Limitation**: detects sites accessible in vitro but not necessarily cleaved in the cellular chromatin context. More sensitive than GUIDE-seq, but lower specificity for in-vivo-relevant sites.

### Digenome-seq (Kim et al. 2015)

**Principle**: treat naked genomic DNA with Cas9 in vitro; sequence the entire genome. Cas9 cleavage sites appear as clusters of reads with uniform 5′ ends at the cut position.

**Advantage**: genome-wide without special library preparation steps; scalable

**Limitation**: like CIRCLE-seq, detects sequence-accessible sites not necessarily cleaved in cells; whole-genome sequencing is expensive

## Integrating Computational and Experimental Data

A best-practice workflow for a therapeutic application:

1. **CRISPOR/Cas-OFFinder**: identify all candidate sites with ≤ 4 mismatches; compute CFD scores
2. **ATAC-seq integration**: filter to sites in accessible chromatin in the relevant cell type
3. **CIRCLE-seq or GUIDE-seq**: experimentally validate remaining candidates
4. **Amplicon deep sequencing** at top predicted and experimentally detected off-target sites: quantify actual editing frequencies

For research applications (not therapeutic), a simplified version is usually sufficient: CRISPOR design to select high-specificity guides (high on-target Doench score, low aggregate off-target score), plus Sanger sequencing at the top 3–5 predicted off-target sites.

## Example: Interpreting a CRISPOR Off-Target Report

For a guide targeting PCSK9, CRISPOR might report:

| Off-Target Site | Mismatches | CFD Score | Chromatin |
|----------------|-----------|----------|-----------|
| chr1:55039975 (on-target) | 0 | 1.00 | Open |
| chr7:12345678 | 2 | 0.45 | Open |
| chr12:87654321 | 3 | 0.18 | Closed |
| chr3:11223344 | 3 | 0.12 | Open |

The site on chr7 (CFD = 0.45, open chromatin) is the highest risk and should be validated experimentally. The chr12 site (CFD = 0.18, closed chromatin) is likely not cleaved in cells and may be deprioritized.

## Why This Matters

Off-target cleavage is not just a theoretical concern. In early CRISPR therapeutic trials, off-target sites were detected in edited cells at frequencies sufficient to cause regulatory concern, even with well-designed guides. In cancer cells, off-target DSBs can drive chromosomal rearrangements. In model organism experiments, off-target mutations can cause phenotypes falsely attributed to the intended knockout. Rigorous off-target analysis — combining computational prediction, chromatin-aware filtering, and experimental validation — is now a standard requirement for publication in high-impact journals and for regulatory submission of therapeutic editing products.

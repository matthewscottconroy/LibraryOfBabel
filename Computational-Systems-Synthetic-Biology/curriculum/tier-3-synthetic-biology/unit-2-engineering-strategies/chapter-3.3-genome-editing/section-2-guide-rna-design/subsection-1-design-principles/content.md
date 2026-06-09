# Guide RNA Design Principles

Imagine you have just identified a mutation in a patient's genome — a single cytosine that, if changed, would prevent a devastating disease. You know the target. You know the tool. What you need now is the right guide RNA to connect them. Designing that guide RNA is not a trivial lookup: there may be dozens of candidate 20-nucleotide sequences near your target, and they are not equivalent. Some will edit efficiently; others will barely work. Some will cut only where you want; others will cut in three additional places in the genome. The rules that separate good guides from poor ones emerge directly from the molecular mechanism you've just learned — and knowing them is what allows you to design an experiment that succeeds the first time rather than the third.

Selecting an effective guide RNA (sgRNA) for CRISPR-Cas9 editing is the first and most consequential experimental design decision. A well-designed guide achieves high on-target editing efficiency while minimizing unintended edits elsewhere in the genome. Guide design is governed by PAM constraints, sequence rules that predict Cas9 activity, and considerations specific to the editing goal.

## Step 1: Identifying Candidate Target Sites

For SpCas9, a targetable site is any 20-nucleotide sequence immediately 5′ of a 5′-NGG-3′ PAM on either genomic strand. For a gene of interest:

1. Extract the genomic sequence of the target exon (or regulatory region)
2. Scan both strands for NGG occurrences
3. The 20 nucleotides immediately 5′ of each NGG on that strand are candidate spacers

In practice, there are approximately 1 NGG-targetable site per 8 bp in a random sequence, giving roughly 2–3 target sites per codon in a coding sequence. The density is sufficient that design rules can be applied to choose among many candidates.

**Example**: For the sequence `5′-AGGCTATGCCGTACG[NGG]-3′`, the spacer would be `AGGCTATGCCGTACG` followed by 5 more nucleotides completing the 20-nt protospacer.

## Step 2: PAM Sequence Requirements

The NGG PAM is non-negotiable for SpCas9. Related constraints:
- **NAG** is a weak alternative PAM recognized by SpCas9 with ~5–10% efficiency; generally avoid as on-target sites
- The PAM must be present in the genomic DNA, not in the sgRNA
- PAM sequences in the supplied donor template can be silently mutated to prevent re-cleavage of edited alleles

## Step 3: GC Content

GC content of the 20-nt spacer affects both R-loop stability and on-target cleavage efficiency. Empirical rules from Doench et al. 2016 and Moreno-Mateos et al. 2015:

- **Optimal range: 40–70% GC**
- Below 40%: weak R-loop stability → reduced cleavage efficiency
- Above 70%: potential off-target binding to GC-rich genomic regions; possible secondary structure in the sgRNA itself

A 20-nt spacer with 40–70% GC contains 8–14 G/C bases. Calculate:
$$\text{GC fraction} = \frac{N_G + N_C}{20}$$

## Step 4: Avoiding Poly-T Sequences

RNA Polymerase III (Pol III), which transcribes most sgRNA expression cassettes, terminates at runs of 4 or more thymidines (TTTT) in the template. If the spacer sequence contains 4+ consecutive T's:
- Pol III terminates prematurely
- A truncated sgRNA is produced
- On-target editing efficiency drops to near zero

**Rule**: avoid spacers containing 5′-TTTT-3′ anywhere in the 20-nt protospacer sequence. Similarly avoid UUUU in the RNA: since the spacer RNA sequence mirrors the non-template strand, check for TTTT in the spacer DNA sequence.

Also avoid other homopolymer runs (AAAA, CCCC, GGGG) which can form hairpins or other secondary structures in the sgRNA.

## Step 5: Seed Region Rules

The seed region (positions 1–12 from the PAM-proximal end, i.e., the 3′ end of the spacer) is the primary specificity determinant. Design rules:
- **The seed region should match the target perfectly**: any mismatch here will reduce on-target efficiency significantly
- **Prioritize guides where seed region sequences are unique in the genome**: run BLAST or a dedicated tool with the seed region as query

## Step 6: Position-Specific Activity Rules

High-throughput guide efficacy screens (Doench et al. 2016, trained on ~1,800 guides) identified position-specific nucleotide preferences for SpCas9 on-target activity:

- **G at position 20** (immediately 5′ of PAM): associated with higher activity
- **G at positions 1 and 2**: slightly beneficial
- **A at position 3 (PAM-proximal)**: mildly negative
- **C at position 13–16**: mildly negative in some contexts

These rules are encoded in the **Doench Rule Set 2** score, available in CRISPOR and other tools. The score ranges 0–1, with higher values predicting better on-target efficiency.

## Step 7: Transcription Start Site Considerations

For gene knockout by promoter disruption or knockdown using CRISPRi, guide position matters:

- **Knockout**: target early exons to ensure frameshift disrupts the entire protein. Avoid the last exon (truncated protein may still be functional).
- **CRISPRi**: dCas9-KRAB must target within 200 bp upstream or within the first 200 bp of the coding sequence for maximal repression. Guides too far upstream or downstream are ineffective.
- **Avoid known functional domains in the guide's flanking sequence**: occasional indels in flanking sequence can disrupt splicing sites or regulatory elements

## Practical Guide Design Workflow

```python
# Pseudocode for guide design
target_gene = "EGFR"
genomic_sequence = fetch_sequence(target_gene, exon=2)

candidates = []
for i in range(len(genomic_sequence) - 22):
    if genomic_sequence[i+20:i+23] in ["AGG","CGG","GGG","TGG"]:  # NGG PAM
        spacer = genomic_sequence[i:i+20]
        gc = (spacer.count('G') + spacer.count('C')) / 20
        has_poly_t = 'TTTT' in spacer
        
        if 0.4 <= gc <= 0.7 and not has_poly_t:
            candidates.append({
                'spacer': spacer,
                'position': i,
                'gc': gc,
                'doench_score': compute_doench(spacer)  # Rule Set 2
            })

# Rank by Doench score; filter by off-target predictions
top_guides = sorted(candidates, key=lambda x: x['doench_score'], reverse=True)[:5]
```

## Recommended Tools

**CRISPOR** (crispor.tefor.net): the most comprehensive guide design tool. Outputs Doench scores, CFD off-target scores, and all predicted off-target sites with up to 4 mismatches. Best for research use.

**CHOPCHOP** (chopchop.cbu.uib.no): user-friendly interface; integrates multiple algorithms; supports multiple Cas proteins and organisms.

**Benchling**: integrated lab notebook platform; guide design module with off-target annotation; useful for teams.

**Cas-OFFinder**: not a design tool but used for exhaustive off-target site enumeration after the guide is chosen.

## Worked Example: Designing a Guide for KRAS G12D Knockout

Target: Knock out KRAS in codon 12 (hotspot mutation G12D in cancer cell lines).

Genomic context around codon 12 (KRAS exon 2):
```
5′-...GACTGAATATAAACTTGTGGTAGTTGGAGCT[GGTGGCGTAGGCAAGAGTGCC]-[TGG]-...3′
                                        ←── 20-nt spacer ──→      PAM
```

Spacer: `GGTGGCGTAGGCAAGAGTGCC`
- GC: 14/20 = 70% (borderline high; acceptable)
- No TTTT present
- PAM: TGG (valid NGG)
- Seed region (positions 1–12 from PAM): `GCAAGAGTGCC` — check genome for uniqueness
- Position: cuts within codon 12, disrupting KRAS function by NHEJ

This guide was used in published studies and achieves >50% editing in multiple cancer cell lines.

## Why This Matters

Guide RNA design is the most commonly performed task in CRISPR experiments, yet poorly designed guides waste weeks of experimental effort. The rules described here — GC content, poly-T avoidance, seed region uniqueness, position-specific preferences — each individually increase on-target efficiency by 1.5–3-fold. Together they separate guides that achieve 50–80% editing efficiency from guides that achieve 5–10%. For screens, where hundreds of guides must all work reliably, the difference between good and poor design practice is the difference between a successful and a failed experiment.

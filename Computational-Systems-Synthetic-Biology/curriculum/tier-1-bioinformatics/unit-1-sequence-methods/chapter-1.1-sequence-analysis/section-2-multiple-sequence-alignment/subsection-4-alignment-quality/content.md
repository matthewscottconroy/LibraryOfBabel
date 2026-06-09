# Alignment Quality Assessment

Computing a multiple sequence alignment is not sufficient — the result must be evaluated for quality before downstream analysis. A poor alignment used in phylogenetics, structure prediction, or functional annotation produces correspondingly poor results. This is not a hypothetical concern: benchmark studies have shown that alignment errors can change the topology of phylogenetic trees, alter which residues appear conserved, and degrade the coevolutionary signal that structural prediction methods depend on. Alignment quality assessment combines quantitative scoring metrics, reference benchmark comparison, and expert visual inspection.

You might expect that running a well-established tool like MAFFT or MUSCLE is sufficient, and that quality checking is optional housekeeping. It is not. Alignment quality degrades systematically in specific circumstances — sequences with very different lengths, proteins with long disordered regions, alignments near the twilight zone of sequence identity. Knowing when your alignment is trustworthy and when it needs scrutiny is a core skill.

## Quantitative Scoring Metrics

**Sum-of-Pairs (SP) Score**

The most commonly used internal metric. For an alignment $\mathcal{A}$ of $k$ sequences, the SP score sums all pairwise alignment scores across all columns:

$$SP(\mathcal{A}) = \sum_{1 \leq i < j \leq k} \sum_{c=1}^{L} \sigma(A_i[c], A_j[c])$$

where $L$ is the alignment length and $\sigma$ is the substitution matrix score (typically BLOSUM62 for proteins). Gap pairs contribute $\sigma(\text{gap}, \text{gap})$ or are excluded depending on the implementation.

Higher SP score = better alignment (given the same scoring scheme). SP is used internally by MUSCLE and MAFFT for convergence checks during iterative refinement.

**Column Score (CS)**

Measures the fraction of alignment columns perfectly recovered when compared to a reference alignment:

$$CS = \frac{\text{number of correctly aligned columns}}{\text{total columns in reference}}$$

CS requires a reference (gold standard) alignment, so it is used for benchmark evaluation rather than routine quality assessment.

**Sum-of-Pairs Score Against Reference (SPS)**

Measures the fraction of pairwise residue homologies correctly aligned:

$$SPS = \frac{\text{number of correctly aligned residue pairs}}{\text{number of residue pairs in reference}}$$

A correctly aligned residue pair $(a_i, b_j)$ means that residue $a_i$ from sequence $A$ and residue $b_j$ from sequence $B$ are aligned in the same column in both the test and reference alignments.

## Reference-Free Quality Indicators

In practice, reference alignments are not available. Several signals indicate alignment quality:

**Gap distribution**: Legitimate gaps cluster in structurally variable loop regions; gaps scattered randomly across an alignment indicate misalignment. A high fraction of gap-only columns suggests over-gapping.

**Consistency**: For any three sequences $A$, $B$, $C$ in the alignment, if $a_i$ is aligned to $b_j$, and $b_j$ is aligned to $c_k$, then $a_i$ should be aligned to $c_k$. Violations indicate regions where the progressive algorithm made inconsistent choices.

**Block structure**: Well-aligned regions typically show clear conserved blocks interspersed with variable loop regions. An alignment that looks "noisy" throughout — many short conserved stretches with many gaps — may reflect genuine sequence diversity or may indicate misalignment.

## Visual Inspection Tools

### Jalview

Jalview is a Java-based alignment editor and viewer with extensive features:
- Color schemes: ClustalX (conservation-based), BLOSUM62 (chemical character), Taylor (amino acid type)
- Conservation and consensus row: shows conservation percentage and consensus residue per column
- Sequence logo: visualizes position-specific residue frequencies
- Tree visualization: annotate sequences with phylogenetic relationships
- Secondary structure annotation overlaid on alignment

Visual inspection in Jalview is not optional for any alignment that will be used in a published analysis. Secondary structure annotations overlaid on the alignment are particularly valuable: gaps should appear in loops, not in helices or strands. A gap in the middle of a predicted alpha helix is almost certainly a misalignment.

### AliView

A fast, simple alignment editor suitable for large alignments:
- Handles alignments of thousands of sequences
- Allows manual editing of gaps
- Shows nucleotide/amino acid coloring
- Useful for final inspection and manual curation before phylogenetic analysis

### UGENE

Open-source bioinformatics suite with integrated alignment viewer, allowing alignment editing, consensus computation, and phylogenetic tree display alongside the alignment.

## Trimming Poorly Aligned Regions

Even a high-quality alignment typically contains regions of low confidence — often the highly variable N- and C-termini and loop insertions. Including these in phylogenetic analysis can mislead tree inference by introducing noise.

**trimAl**: the standard tool for automated alignment trimming:

```bash
# Remove columns with > 20% gaps
trimal -in msa.fasta -out msa_trimmed.fasta -gt 0.8

# Automated heuristic trimming (recommended for phylogenetics)
trimal -in msa.fasta -out msa_trimmed.fasta -automated1

# Remove sequences shorter than 60% of alignment length
trimal -in msa.fasta -out msa_trimmed.fasta -resoverlap 0.60 -seqoverlap 60
```

**Gblocks**: a more conservative trimmer that retains only blocks of conserved columns meeting length and flanking requirements. Produces shorter but cleaner alignments.

There is an ongoing debate in the field about whether trimming helps or hurts phylogenetic analysis. The concern with aggressive trimming is that it can discard phylogenetically informative positions along with the noisy ones. The `automated1` mode in trimAl is generally considered a reasonable balance — it removes columns that are both gappy and poorly conserved, while retaining columns that are gappy but conserved. For maximum caution, try both trimmed and untrimmed alignments and check whether tree topologies are robust to the choice.

## Worked Example: Evaluating an Alignment

Consider two alignments of the same five cytochrome c sequences:

**Alignment A** (ClustalW, no trimming):
```
Seq1  MGDVEKGKKIFIMKCSQCHTVEKGGKHKTGPNLHGLFGRKTGQAPGYSYTAANKNKGIIWGEDTLMEYLENPK
Seq2  MGDVEKGKKIFVQKCAQCHTVEKGGKHKTGPNLHGLFGRKTGQAPGFTYTDANKNKGITWKEETLMEYLENPK
Seq3  MGDVEKGKKI----CSQCHTVEKGGK-----NLHGLFGRKTGQ-------YTAANKNKGITWGEDTLMEYLENPK
```

**Alignment B** (MAFFT L-INS-i):
```
Seq1  MGDVEKGKKIFIMKCSQCHTVEKGGKHKTGPNLHGLFGRKTGQAPGYSYTAANKNKGIIWGEDTLMEYLENPK
Seq2  MGDVEKGKKIFVQKCSQCHTVEKGGKHKTGPNLHGLFGRKTGQAPGFTYTDANKNKGITWKEETLMEYLENPK
Seq3  MGDVEKGKKIF--KCSQCHTVEKGGK-----NLHGLFGRKTGQ-------YTAANKNKGITWGEDTLMEYLENPK
```

Alignment B places gaps in more biologically plausible positions (insertion in loops rather than core secondary structure) and achieves a higher SP score. The conserved heme-binding residues (His, Met) remain in the same columns across all sequences.

## Why This Matters

Alignment quality directly determines the reliability of every downstream analysis. In phylogenetics, poorly aligned columns contribute noise that can mislead tree topology inference. In structure prediction, misaligned MSA columns destroy the covariation signal that AlphaFold2 relies on for accurate contacts. In functional annotation, misalignment can place a functionally critical residue in a gap column, making it appear non-conserved. Investing effort in alignment quality assessment — using appropriate tools, checking visually, trimming uncertain regions — is one of the highest-return practices in computational biology.

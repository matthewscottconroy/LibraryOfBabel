# Substitution Matrices

A **substitution matrix** encodes the expected score for aligning each amino acid (or nucleotide) to each other, derived from evolutionary data. These matrices transform alignment from a pure string-matching problem into a biologically-informed measurement of evolutionary relatedness. Choosing the right substitution matrix is as important as choosing the right alignment algorithm.

Here is the key insight: when two protein sequences diverged from a common ancestor, the substitutions that actually happened were not random. A hydrophobic amino acid buried in a protein core was far more likely to be replaced by another hydrophobic amino acid than by a charged one. A cysteine forming a disulfide bond was under extreme pressure to stay a cysteine. These evolutionary preferences are encoded in the statistical patterns of substitutions observed across thousands of protein families — and substitution matrices distill those patterns into alignment scores.

## Why Not Use Simple Match/Mismatch Scoring?

A naive scoring scheme assigns +1 to every match and -1 to every mismatch, regardless of which amino acids are compared. This ignores two critical biological facts:

1. **Biochemical similarity**: leucine (L) and isoleucine (I) are both large, hydrophobic, branched-chain amino acids. They substitute for each other frequently in evolution without disrupting protein function. Aligning L to I should score more highly than aligning L to arginine (R), a positively charged amino acid.

2. **Evolutionary frequency**: some substitutions are intrinsically more common than others due to codon structure. A single nucleotide change (A→G, a transition) changes codons differently than a transversion, biasing the amino acid substitution space.

Substitution matrices capture both effects by being derived directly from observed evolutionary substitutions.

## PAM Matrices

**PAM (Point Accepted Mutation)** matrices were developed by Margaret Dayhoff in 1978, working at the National Biomedical Research Foundation with a collection of protein families that was modest by today's standards but revolutionary for its time. The approach starts from alignments of closely related proteins and infers a model of amino acid substitution.

**PAM1** represents 1 accepted mutation per 100 amino acid positions. The **mutation probability matrix** $M$ gives the probability that amino acid $j$ replaces amino acid $i$ in 1 PAM of evolution:

$$M_{ij} = \frac{f_{ij}}{\pi_j}$$

where $f_{ij}$ is the observed replacement frequency and $\pi_j$ is the background frequency of amino acid $j$.

PAM matrices for longer evolutionary distances are computed by matrix exponentiation:

$$\text{PAM}_k = M^k$$

The **log-odds score** entry for aligning residues $i$ and $j$ is:

$$s(i,j) = \log \frac{M_{ij}^{(k)}}{\pi_j}$$

Positive values indicate substitutions more frequent than expected by chance; negative values indicate substitutions rarer than expected.

**PAM250** (PAM1 applied 250 times) is designed for comparing distantly related sequences. At PAM250, each position has on average 2.5 substitutions, but many sites have had multiple hits — the sequences appear about 20% identical.

**Rule of thumb**: use smaller PAM numbers (PAM30, PAM70) for closely related sequences; use PAM250 for distant homologs.

## BLOSUM Matrices

**BLOSUM (BLOcks SUbstitution Matrix)** matrices were introduced by Henikoff and Henikoff in 1992 and largely superseded PAM matrices in practice. The key improvement was empirical rather than model-based: instead of extrapolating a model of evolution outward to distant sequences, BLOSUM matrices are derived directly from observed substitutions at specific levels of sequence divergence.

**Key difference**: BLOSUM matrices are derived directly from ungapped alignments of conserved sequence blocks in the **BLOCKS database**, not from an extrapolated evolutionary model. This makes them empirically grounded at specific identity levels.

The derivation:
1. Collect thousands of ungapped multiple alignments from the BLOCKS database
2. For BLOSUM$X$: cluster sequences with $\geq X$% identity; treat each cluster as one sequence (downweight redundancy)
3. Count all pairwise substitutions across aligned columns
4. Compute log-odds scores: $s(i,j) = 2 \log_2 \frac{q_{ij}}{e_{ij}}$

where $q_{ij}$ is the observed frequency of the $i$-$j$ pair and $e_{ij} = p_i \cdot p_j$ is the expected frequency under independence.

**BLOSUM62**: derived from alignments at $\geq 62$% identity. This is the **standard default** for most protein alignment tasks and BLAST searches.

**BLOSUM80**: for closely related sequences ($\geq 80$% identity). Scores matches more highly, penalizes mismatches more.

**BLOSUM50**: for more divergent sequences. More permissive of mismatches between similar amino acids.

The BLOSUM number is inversely related to evolutionary distance: BLOSUM80 was derived from closely related sequences, so it is appropriate when your query and target are closely related. BLOSUM45 was derived from more divergent sequences, so it is appropriate when you are searching for distant homologs. This is the opposite direction from PAM numbering, which trips up many students initially.

## Reading a Substitution Matrix

Here is a fragment of the BLOSUM62 matrix:

| | A | R | N | D | C | Q | E |
|---|---|---|---|---|---|---|---|
| A | 4 |-1 |-2 |-2 | 0 |-1 |-1 |
| R |-1 | 5 | 0 |-2 |-3 | 1 | 0 |
| N |-2 | 0 | 6 | 1 |-3 | 0 | 0 |
| D |-2 |-2 | 1 | 6 |-3 | 0 | 2 |
| C | 0 |-3 |-3 |-3 | 9 |-3 |-4 |

Key observations:
- Diagonal entries (self-alignment) are positive and larger for rare amino acids (C = 9) than common ones (A = 4), because rare amino acids carry more information when conserved.
- D/N score = +1 (asparagine/aspartate, similar chemistry)
- C/E score = -4 (cysteine/glutamate, very different)

The cysteine self-alignment score of 9 deserves special attention. Cysteine is both rare and biochemically distinctive — it can form disulfide bonds and coordinate metal ions. When you see a cysteine conserved at a position across many species, it is almost certainly critical for function or structure. The high self-alignment score reflects this evolutionary pressure.

## Nucleotide Substitution Matrices

For DNA alignment, the options are simpler:

- **Identity matrix**: +1 for match, -3 for mismatch (simple default)
- **EDNAFULL (NUC4.4)**: handles IUPAC ambiguity codes (N, R, Y, W, etc.)
- **For protein-coding regions**: translate first, then use protein matrix — more sensitive for detecting distant coding homologs

For evolutionary analysis, nucleotide substitution models (JC69, HKY85, GTR) describe substitution probabilistically rather than as a scoring matrix, and are discussed in the phylogenetics chapter.

## Choosing the Right Matrix

| Sequence identity (expected) | Recommended matrix |
|---|---|
| > 85% | BLOSUM80 or PAM30 |
| 50–85% | BLOSUM62 (default) |
| 30–50% | BLOSUM50 |
| < 30% | BLOSUM45, or profile HMMs |

For cross-species comparison of well-conserved protein families (e.g., histones, ubiquitin), BLOSUM80 is appropriate. For searching a database with an unknown query, BLOSUM62 is the robust default. For detecting remote homologs (e.g., threading a novel sequence onto a fold), PSI-BLAST's position-specific scoring matrix (PSSM) or profile HMMs outperform any single substitution matrix.

## Why This Matters

Substitution matrices operationalize the concept that evolutionary history is encoded in sequence. They transform alignment from pattern matching into probabilistic inference about common ancestry. An incorrect matrix choice systematically biases which alignments score highest, leading to false homology calls or missed true homologs. In drug discovery, identifying the correct distant homolog of a drug target in a pathogen may depend entirely on which substitution matrix is used. In phylogenetics, substitution matrices determine which alignment columns appear conserved and thus contribute to tree inference. No other single parameter choice in bioinformatics has such broad downstream consequences. When you accept the BLOSUM62 default and never question it, you are making an evolutionary assumption — the assumption that your sequences are diverged to the ~62% identity level. For very recently diverged sequences or very ancient ones, that assumption is wrong, and your alignments will reflect it.

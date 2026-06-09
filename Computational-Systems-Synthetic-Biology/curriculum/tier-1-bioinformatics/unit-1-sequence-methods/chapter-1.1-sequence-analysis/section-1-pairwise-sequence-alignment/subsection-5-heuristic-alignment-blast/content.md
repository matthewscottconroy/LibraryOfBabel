# Heuristic Alignment: BLAST

Smith-Waterman alignment is exact but slow: comparing a 500 amino acid query against the entire UniProt database (~250 million sequences) would require years of computation. **BLAST (Basic Local Alignment Search Tool)** achieves near-equivalent sensitivity at 100–1000× the speed through a seed-and-extend heuristic. BLAST is the most widely used tool in bioinformatics and arguably the most important single algorithm in the history of the field.

The story behind BLAST is worth knowing. When the human genome project was ramping up in the late 1980s, biologists faced a practical crisis: newly sequenced genes needed to be compared against growing databases, but the databases were growing faster than Smith-Waterman could search them. Stephen Altschul and colleagues at the NCBI published BLAST in 1990 — and within a few years it had transformed how biology was done. The paper has been cited more than 90,000 times, making it one of the most-cited papers in scientific history. Almost every newly discovered gene is BLASTed before anything else. The algorithm enabled the flood of genomic data in the 1990s to be functionally annotated at scale.

## The Core Heuristic: Seed-and-Extend

BLAST exploits the observation that significant alignments almost always contain a short, exact (or near-exact) matching word. Rather than computing the full dynamic programming table, BLAST:

1. **Seeds**: identifies short exact word matches between query and database
2. **Extends**: applies local alignment outward from seeds until the score drops below a threshold
3. **Reports**: returns all extended alignments with statistical significance above a cutoff

This sacrifices the mathematical guarantee of finding the optimal alignment but finds nearly all biologically significant alignments orders of magnitude faster.

The key biological assumption is worth examining: for two sequences to be meaningfully homologous, they almost certainly share at least one short stretch of high identity. A 30% identity protein has statistically unlikely chances of lacking any word of 3 amino acids in common. This assumption fails for extremely divergent sequences — which is why BLAST struggles at the edge of the "twilight zone" of sequence identity (< 20–25%) and why iterative methods like PSI-BLAST are necessary for deep homology detection.

## Step-by-Step Algorithm

**Step 1: Query word generation**

For protein BLAST (`blastp`), the default word size is $W = 3$. Every 3-mer in the query is enumerated. For each 3-mer, all possible 3-mer sequences scoring above a threshold $T$ (default T = 11 with BLOSUM62) are added to a **neighborhood word list**.

Example: for query word `LYS`, the neighborhood includes not just `LYS` itself but also similar 3-mers like `LFS`, `IYS`, `LYA`, etc. — all scoring $\geq T$ against `LYS` in BLOSUM62.

**Step 2: Database scanning**

The database is scanned for exact matches to any word in the neighborhood list. This uses a precomputed hash table or finite automaton, enabling $O(1)$ lookup per position.

**Step 3: Ungapped extension**

When a hit (exact word match) is found, BLAST extends the alignment diagonally (without gaps) in both directions. The score is tracked; extension stops when the score drops more than $X_{drop}$ below the maximum score seen so far. If the resulting high-scoring segment pair (HSP) exceeds a score threshold $S$, it is retained.

**Step 4: Gapped extension**

Retained HSPs undergo a full gapped alignment (Smith-Waterman) centered on the HSP region. This step recovers the biologically accurate alignment with indels.

**Step 5: Statistical evaluation**

Each alignment score $S$ is assigned an E-value.

## The E-value

The **E-value** is the expected number of alignments with score $\geq S$ that would be found by chance when searching a database of the given size. It is computed using extreme value distribution statistics:

$$E = K \cdot m \cdot n \cdot e^{-\lambda S}$$

where:
- $m$ = query length
- $n$ = total database size (number of residues)
- $K$ and $\lambda$ = statistical parameters determined by the scoring matrix and gap penalties
- $S$ = raw alignment score

**Interpretation**:
- E-value = 10: expect 10 random hits of this score — not significant
- E-value = 0.01: expect one such hit in 100 database searches — possibly significant
- E-value < $10^{-5}$: typically significant
- E-value < $10^{-10}$: high-confidence homology

The E-value scales with database size: searching a larger database increases E-values for the same alignment score, reflecting the increased chance of finding a false positive by chance. This is a critical practical point. If you search your query against a database of 1,000 sequences and get E = $10^{-6}$, and then search against a database of 100 million sequences, the same alignment score will return E = $10^{-1}$ — suddenly non-significant. The alignment has not changed; only the statistical context has.

## BLAST Variants

| Program | Query | Database | Use case |
|---------|-------|----------|----------|
| `blastn` | Nucleotide | Nucleotide | DNA/RNA similarity search |
| `blastp` | Protein | Protein | Protein homolog search |
| `blastx` | Nucleotide (6 frames) | Protein | Find protein-coding genes in DNA |
| `tblastn` | Protein | Nucleotide (6 frames) | Find protein relatives in genome |
| `tblastx` | Nucleotide (6 frames) | Nucleotide (6 frames) | Cross-species coding region comparison |
| `megablast` | Nucleotide | Nucleotide | Optimized for very similar sequences |

**DIAMOND**: an alternative protein aligner achieving 100-1000× speedup over BLASTP for large-scale metagenomics and pangenome analysis, with comparable sensitivity at typical E-value thresholds.

## PSI-BLAST: Iterative Profile Search

For detecting **remote homologs** (< 30% sequence identity), a single BLAST search is often insufficient. **PSI-BLAST (Position-Specific Iterated BLAST)** builds a **position-specific scoring matrix (PSSM)** from initial alignment results:

1. Run standard `blastp` with the query
2. Collect significant hits (E < 0.001), align them
3. Compute a PSSM: at each alignment position, the score for each amino acid is derived from the observed frequency in that column (biased toward conserved residues)
4. Search the database with the PSSM instead of the query sequence
5. Repeat until convergence

The PSSM effectively encodes conservation patterns: conserved active-site residues score very high for the correct residue; variable surface positions are more permissive. This makes PSI-BLAST far more sensitive than standard BLAST for distant homology detection.

**Warning**: PSI-BLAST can "blow up" — if a false positive is incorporated into the profile, it can bias subsequent iterations toward unrelated sequences. Always inspect the alignment at each iteration. The safest practice is to run only 2–3 iterations and manually review each new sequence added to the profile.

## Practical BLAST Usage

```bash
# Build a local database from a FASTA file
makeblastdb -in myproteins.faa -dbtype prot -out mydb

# Run blastp
blastp -query query.faa -db mydb \
       -evalue 1e-5 \
       -outfmt 6 \
       -num_threads 8 \
       -out results.txt

# tabular output (-outfmt 6) columns:
# qseqid sseqid pident length mismatch gapopen qstart qend sstart send evalue bitscore
```

## Interpreting BLAST Output

Key fields in BLAST results:
- **% identity**: fraction of aligned positions that are identical
- **alignment length**: number of columns in the aligned region (includes gaps)
- **E-value**: statistical significance (lower = more significant)
- **bit score**: normalized alignment score; comparable across different database sizes

**Do not confuse E-value with p-value**: the p-value is the probability of observing a score $\geq S$ by chance in a single comparison. E-value = p-value × database size. Both measure significance, but E-value is more intuitive for database searches.

A common mistake is to treat sequence identity percentage as the primary measure of significance. Two sequences may share 45% identity over just 20 amino acids — which could easily occur by chance — or 45% identity over 300 amino acids, which is almost certainly homologous. The E-value integrates both identity and alignment length into a single significance measure. Always look at E-value first, alignment length second, and identity percentage third.

## Why This Matters

BLAST transformed biology. Before BLAST (1990, Altschul et al.), comparing a new sequence to known sequences required either slow exact methods or fast but crude approximations. BLAST made database searching tractable at scale, enabling the flood of genomic data from the 1990s onward to be functionally annotated. Every newly sequenced gene is BLASTed before any other analysis. Every genome annotation pipeline relies on BLAST or its successors. Understanding BLAST's statistics — particularly E-values and their dependence on database size — is essential for interpreting the vast majority of comparative sequence data encountered in research.

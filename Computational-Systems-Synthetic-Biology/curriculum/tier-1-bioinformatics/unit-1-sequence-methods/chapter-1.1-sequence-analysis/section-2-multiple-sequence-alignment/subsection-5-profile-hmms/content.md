# Profile Hidden Markov Models

A **profile Hidden Markov Model (profile HMM)** is a probabilistic representation of a multiple sequence alignment that captures both the residue composition at each position and the patterns of insertions and deletions. Profile HMMs are the most sensitive method for detecting sequence homologs, particularly at low sequence identity, and underlie the Pfam protein family database and the database searches that power AlphaFold2.

To appreciate why profile HMMs are such a powerful step forward, consider what a single protein sequence can tell you about whether a new sequence is a member of the same family. Very little, if the new sequence is distantly related. At 25% identity, a pairwise BLAST search may produce a borderline E-value that could be a true homolog or a chance match. Now consider what happens if you have aligned 500 members of that protein family and built a detailed statistical model of what each position in the alignment looks like. Suddenly you have enormous discriminatory power: you know exactly which positions tolerate variation and which are invariant, which insertions are tolerated between specific positions and which never occur, and what the baseline rates of deletion look like across the family. That model is a profile HMM, and searching a new sequence against it is orders of magnitude more sensitive than any pairwise comparison.

## From Alignment to Profile

A multiple sequence alignment can be read column by column. Each column represents a homologous position shared across the aligned sequences. From any column, we can compute:

- The **residue frequency** at that position: what fraction of sequences have Ala, Val, Leu, etc.?
- The **gap frequency**: what fraction of sequences have a gap?

A simple profile stores these frequencies as a position-specific scoring matrix (PSSM). A profile HMM extends this to also model insertion and deletion patterns probabilistically.

## Profile HMM Structure

A profile HMM for an alignment of length $L$ (consensus columns) contains three types of states per position:

- **Match state $M_k$**: represents an aligned position. Has an **emission probability distribution** over the alphabet (20 amino acids or 4 nucleotides). Sequences are expected to emit one residue per match state.
- **Insert state $I_k$**: models residues inserted between consensus positions $k$ and $k+1$. Has emission probabilities (often flat, uniform distribution). Can emit zero or more residues.
- **Delete state $D_k$**: silent (emits nothing). Allows the HMM to skip a consensus position entirely.

Additionally, the model has Begin (B) and End (E) states.

**Transition probabilities** connect states:

From $M_k$:
- To $M_{k+1}$: probability of continuing to the next match state
- To $I_k$: probability of inserting before position $k+1$
- To $D_{k+1}$: probability of skipping position $k+1$

These transition probabilities are learned from the observed gap patterns in the multiple alignment. A family where deletions are rare will have high $M_k \to M_{k+1}$ transition probabilities. A family that commonly has an insertion between positions 50 and 51 will have high $M_{50} \to I_{50}$ and $I_{50} \to M_{51}$ probabilities. The model encodes the family's evolutionary history in its transition structure.

## Mathematical Formulation

The probability that a profile HMM $\lambda$ generates a sequence $x = x_1 x_2 \ldots x_n$ is:

$$P(x | \lambda) = \sum_{\pi} P(x, \pi | \lambda)$$

where the sum is over all possible state paths $\pi$ through the HMM. Each path corresponds to one way of aligning $x$ to the profile.

For alignment, we want the most probable path through the model given the sequence:

$$\pi^* = \arg\max_\pi P(x, \pi | \lambda)$$

This is computed efficiently by the **Viterbi algorithm**, a DP algorithm running in $O(nL)$ time where $n$ is the sequence length and $L$ is the model length.

### Viterbi Algorithm (simplified)

For each match state $k$ and position $i$ in the sequence:

$$V_M(k, i) = e_M(k, x_i) \cdot \max \begin{cases}
V_M(k-1, i-1) \cdot t(M_{k-1} \to M_k) \\
V_I(k-1, i-1) \cdot t(I_{k-1} \to M_k) \\
V_D(k-1, i-1) \cdot t(D_{k-1} \to M_k)
\end{cases}$$

where $e_M(k, x_i)$ is the emission probability of residue $x_i$ from match state $k$.

The structure of the Viterbi algorithm should look familiar: it is the same dynamic programming logic you have seen in sequence alignment, now applied to a probabilistic model rather than a scoring function. The recursion traverses the sequence and the model simultaneously, finding the path through the model that best explains the observed sequence.

## HMMER: The Standard Tool

**HMMER** implements profile HMM search with careful attention to statistical rigor:

```bash
# Build a profile HMM from a multiple alignment
hmmbuild myprotein.hmm msa.fasta

# Search a protein database with the profile
hmmsearch myprotein.hmm uniprot.fasta > results.txt

# Search a profile database with a single sequence
hmmscan Pfam-A.hmm query.fasta > results.txt

# Iterative profile search (like PSI-BLAST but HMM-based)
jackhmmer query.fasta uniprot.fasta > jackhmmer_results.txt
```

HMMER3 uses accelerated algorithms (the MSV filter, the Viterbi filter, and the Forward filter) applied in sequence to avoid the full $O(nL)$ computation for most sequences. This gives speeds approaching BLAST while maintaining profile HMM sensitivity.

## Pfam: A Database of Profile HMMs

**Pfam** is a database of protein family profile HMMs. As of recent releases, Pfam contains ~19,000 families, each represented by:
- A **seed alignment**: manually curated MSA of representative family members
- A **profile HMM** built from the seed alignment
- A **full alignment**: all UniProt sequences matching the profile

Pfam search (`hmmscan`) assigns protein domains to families:

```bash
# Annotate a protein with Pfam domains
hmmscan --domtblout domains.txt Pfam-A.hmm query.fasta
```

This identifies which known protein domains are present in a query, enabling function prediction by domain composition. When you annotate a new genome, the first question you typically ask about each protein is "does it contain any known domains?" Pfam and HMMER are the standard tools for answering that question.

## HH-suite: Profile-Profile Comparison

The most sensitive method for remote homology detection is **profile-profile alignment**, implemented in **HH-suite** (HHblits, HHpred):

- Instead of aligning a sequence to a profile, align **two profiles** against each other
- Two distantly related proteins may have very different sequences but similar position-specific amino acid preferences — profiles are more similar than raw sequences
- HHblits achieves ~10× greater sensitivity than HMMER for detecting remote homologs

AlphaFold2 uses HHblits to generate the MSA input that provides coevolutionary information for structure prediction.

The power of profile-profile comparison becomes clear at the extremes of evolutionary divergence. Two proteins that share a common structural fold but have no detectable sequence identity — such as distant members of the same protein superfamily — may still have similar profiles at key structural positions. HHpred has been remarkably successful at detecting these remote homologs, effectively extending the reach of sequence-based inference into the realm traditionally reserved for structural comparison.

## Why This Matters

Profile HMMs represent the state of the art in sequence-based function inference. They power Pfam, which annotates hundreds of millions of protein sequences. They underlie the iterative searches (Jackhmmer, HHblits) that build the MSAs used by AlphaFold2. HMMER's statistical framework provides well-calibrated E-values that accurately control false discovery rates in database searches. For any task involving function prediction from sequence, remote homology detection, or building comprehensive protein family models, understanding and using profile HMMs is essential.

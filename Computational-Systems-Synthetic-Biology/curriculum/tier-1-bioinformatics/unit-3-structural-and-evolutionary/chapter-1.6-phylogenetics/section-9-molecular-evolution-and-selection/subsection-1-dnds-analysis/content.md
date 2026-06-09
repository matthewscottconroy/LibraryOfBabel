# dN/dS Analysis (ω)

The genetic code has a remarkable property: because of its redundancy, some nucleotide changes alter the encoded amino acid and some do not. A change in the third codon position often leaves the amino acid unchanged — these are silent mutations. Changes in the first and second positions usually alter the amino acid — these are visible to natural selection. This asymmetry is the basis for one of the most powerful tests in molecular evolution.

If natural selection favors the current amino acid sequence (purifying selection), then amino acid-changing mutations will be removed from the population faster than silent ones. You will observe fewer nonsynonymous substitutions per nonsynonymous site than synonymous substitutions per synonymous site. Conversely, if selection is driving amino acid change — if new variants are actively beneficial — you will see an excess of nonsynonymous substitutions. The ratio of these two rates is ω, and it is the primary currency of molecular selection analysis.

Protein-coding sequences evolve under the pressure of natural selection: deleterious mutations are eliminated (purifying selection), neutral mutations fix at the neutral rate, and occasionally advantageous mutations are positively selected. **dN/dS analysis** quantifies the relative rates of synonymous and nonsynonymous substitutions to infer the action of natural selection on protein sequences.

## The dN/dS Ratio (ω)

**Synonymous substitutions** (dS, also called "silent" substitutions): nucleotide changes that do not alter the encoded amino acid (e.g., CGA → CGT, both encoding Arg). These are nearly neutral and accumulate at a rate approximately equal to the neutral substitution rate.

**Nonsynonymous substitutions** (dN): nucleotide changes that alter the amino acid (e.g., CGA → TGA, Arg → Stop; or CGA → CAA, Arg → Gln). These are subject to natural selection.

The **dN/dS ratio**, denoted **ω** (omega), normalizes these rates:

$$\omega = \frac{dN}{dS}$$

**Interpretation**:
- **ω < 1**: Nonsynonymous substitutions are less frequent than synonymous → **purifying (negative) selection** is removing deleterious mutations. Most proteins under functional constraint show ω in the range 0.05–0.5.
- **ω = 1**: Synonymous and nonsynonymous rates are equal → **neutral evolution** (no selection on the protein sequence). Pseudogenes often show ω ≈ 1.
- **ω > 1**: Nonsynonymous substitutions are MORE frequent than synonymous → **positive (Darwinian) selection** is actively driving amino acid change. Observed in adaptive immune genes, host-pathogen arms race genes, and genes under ecological diversification.

## PAML codeml: The Standard Tool

**PAML** (Phylogenetic Analysis by Maximum Likelihood, Yang 1997) implements a family of **codon substitution models** that explicitly model the synonymous and nonsynonymous substitution processes. The **GY94 model** (Goldman & Yang, 1994) is the foundational codon model: it has a $61 \times 61$ rate matrix over sense codons (excluding the 3 stop codons), with exchangeability parameters for transitions vs. transversions (κ) and for synonymous vs. nonsynonymous changes (ω).

**PAML codeml** implements several model categories for testing selection:

## Branch Models: Detect Rate Acceleration in Specific Lineages

**Branch models** allow different ω values on different branches of the phylogeny:
- **M0 (one-ratio model)**: One ω for all branches. Null model.
- **Two-ratio model**: Specify "foreground" branches (those where adaptive evolution is expected) and allow them to have a different ω (ω₁) from background branches (ω₀).
- **Free-ratio model**: Each branch has its own independently estimated ω. Used to identify which branches show ω > 1.

**Application**: Testing whether a specific lineage (e.g., a specific primate branch, or fast-evolving surface proteins of influenza) has elevated ω.

## Site Models: Detect Sites Under Positive Selection

**Site models** allow different ω values at different codon sites in the alignment:
- **M1a (nearly neutral)**: Two site classes: conserved sites (0 < ω₀ < 1) and neutral sites (ω₁ = 1). Null model.
- **M2a (positive selection)**: Three site classes: conserved (ω₀ < 1), neutral (ω₁ = 1), and positively selected sites (ω₂ > 1). Test: LRT comparing M2a vs M1a. If M2a fits significantly better and ω₂ > 1 with substantial probability mass, positive selection is supported.
- **M7 (beta)** vs. **M8 (beta + ω > 1)**: More flexible continuous beta distribution for site rate variation; M8 adds a class with ω > 1. LRT of M8 vs M7 is the recommended test for positive selection.

After a site model test, **BEB** (Bayes Empirical Bayes) analysis identifies the specific codon sites most likely to be in the positive selection class. These sites are candidates for functional analysis (e.g., sites at the receptor-binding domain of viral hemagglutinin show positive selection consistent with immune escape).

It turns out that site models are often more informative than branch models because they identify the actual amino acid positions that are under positive selection. If the receptor-binding site of influenza hemagglutinin shows ω > 1, this directly implicates those positions as sites of antigenic variation under immune pressure — pointing toward the exact residues that vaccine design should focus on.

## Branch-Site Models: Episodic Positive Selection

**Branch-site models** detect positive selection acting on specific sites in specific lineages — the most realistic scenario for episodic adaptive evolution:
- **Model A**: A subset of sites is allowed to shift from conservation/neutrality to positive selection (ω > 1) on foreground lineages. The LRT compares Model A to a null model with ω = 1 fixed for the positively selected class.

This is the most widely used test for detecting adaptive evolution in specific contexts (e.g., immune escape during HIV evolution within a host, or positive selection on a specific protein family in the primate lineage leading to humans).

## dN/dS Limitations

**Averaging problem**: dN/dS is averaged over all sites in the gene. A gene with 98% conserved sites and 2% positively selected sites will have an average ω < 1, even if those 2% of sites are under strong positive selection. Site models address this but assume each site is independent and ignore spatial clustering.

**Time averaging**: dN/dS measures average selection pressure over the entire phylogenetic history represented in the alignment. Episodic positive selection (acting briefly during an adaptive event) may not be detectable if the signal is diluted across millions of years of subsequent purifying selection.

**Requires diverged sequences**: dN/dS is unreliable when sequences are very similar (few substitutions, high variance in dN and dS estimates). Population-level analyses (McDonald-Kreitman test) are more appropriate for closely related sequences.

**Stop codons and selection**: The GY94 model assumes selection acts at the amino acid level and that synonymous changes are neutral. Synonymous sites can be under selection (codon usage bias, mRNA splicing signals, miRNA target sites), which violates the dS = neutral rate assumption.

## Why This Matters

dN/dS analysis is the primary tool for detecting natural selection acting on protein-coding genes — it has revealed the molecular arms races between host immune systems and pathogens, identified the functional sites in rapidly evolving viral surface proteins, and detected the gene pathways driving primate brain evolution — making it foundational for evolutionary genomics, virology, and the study of adaptation. Every time you read about "genes under positive selection in the human lineage," the underlying evidence almost always comes from dN/dS analysis.

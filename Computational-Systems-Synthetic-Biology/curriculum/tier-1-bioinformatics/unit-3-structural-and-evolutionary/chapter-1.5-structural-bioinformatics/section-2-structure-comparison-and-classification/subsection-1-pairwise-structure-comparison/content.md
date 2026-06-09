# Pairwise Structure Comparison

Here is a question that turns out to be harder than it looks: are these two proteins similar? You can ask this at the level of sequence — do they share amino acid identity? — but sequence similarity fades over evolutionary time, and two proteins can share a common ancestor, perform the same biochemical function, and occupy the same fold while sharing less than 15% sequence identity. At that point, sequence comparison tells you almost nothing. What you need is a way to directly compare three-dimensional shapes.

Comparing two protein structures quantitatively — asking how similar they are in three dimensions — is a fundamental operation in structural bioinformatics. It underlies function prediction, detection of remote evolutionary relationships, validation of computational models, and drug cross-reactivity analysis. Several metrics and algorithms have been developed, each with different properties and appropriate use cases. Understanding when to use which metric, and what its values mean biologically, is the skill this section develops.

## RMSD: Root Mean Square Deviation

The **RMSD** (root mean square deviation) is the most commonly reported measure of structural similarity:

$$\text{RMSD} = \sqrt{\frac{1}{N}\sum_{i=1}^{N} \|r_i - r_i'\|^2}$$

where $r_i$ and $r_i'$ are the coordinates of corresponding atom $i$ in the two structures after optimal rigid-body superposition (rotation + translation that minimizes RMSD), and $N$ is the number of aligned atoms (usually Cα atoms).

**Superposition** is performed using the Kabsch algorithm, which finds the rotation matrix $R$ minimizing RMSD via singular value decomposition of the cross-covariance matrix of the two coordinate sets.

```python
from Bio.PDB import Superimposer, PDBParser

parser = PDBParser()
structure1 = parser.get_structure("ref", "ref.pdb")
structure2 = parser.get_structure("mob", "mobile.pdb")

# Get Ca atoms for residues 1-100 from chain A:
ref_atoms = [structure1[0]['A'][i]['CA'] for i in range(1, 101)]
mob_atoms = [structure2[0]['A'][i]['CA'] for i in range(1, 101)]

sup = Superimposer()
sup.set_atoms(ref_atoms, mob_atoms)
print(f"RMSD: {sup.rms:.2f} Å")
```

**RMSD interpretation**:
- RMSD < 1 Å: Nearly identical (e.g., apo vs. ligand-bound conformations of a rigid protein)
- RMSD 1–3 Å: Structurally similar; same or highly related fold
- RMSD > 3 Å: Significant structural differences; may indicate different functional states or different folds

**Critical limitation**: RMSD is length-dependent. A 100-residue protein with RMSD = 2 Å is more similar than a 500-residue protein with RMSD = 2 Å, because the same absolute deviation is spread over more residues. Furthermore, a single flexible loop with large deviation inflates RMSD even if the rest of the structure is nearly identical. You might encounter a paper reporting RMSD = 4 Å between two structures that are, in fact, essentially the same fold — the difference is entirely localized in a disordered loop at the terminus. Always report the number of aligned residues alongside RMSD. An RMSD without an accompanying N is not a meaningful number.

## TM-score: Normalized Structural Similarity

The **TM-score** (Template Modeling score) addresses RMSD's length dependence by normalizing the distance contribution of each aligned residue pair:

$$\text{TM-score} = \max \left[ \frac{1}{L_{\text{target}}} \sum_{i=1}^{L_{\text{aligned}}} \frac{1}{1 + (d_i/d_0)^2} \right]$$

where $L_{\text{target}}$ is the length of the target protein, $d_i$ is the distance between aligned Cα pair $i$ after superposition, and $d_0 = 1.24(L_{\text{target}} - 15)^{1/3} - 1.8$ Å is a length-dependent normalization factor.

The $1/(1 + (d_i/d_0)^2)$ contribution per residue is a smooth function that approaches 1.0 for well-aligned residues and approaches 0 for residues far apart. This means large deviations in a small number of flexible residues don't dominate the score the way they dominate RMSD. The normalization by $L_{\text{target}}$ removes the length dependence. The result is a score between 0 and 1 that can be compared across proteins of different sizes.

TM-score ranges from 0 to 1:
- **TM-score > 0.5**: Structures are likely from the same fold (globally similar)
- **TM-score > 0.9**: Highly similar; useful for validating computational models
- **TM-score < 0.3**: Very different structures

TM-score is the standard metric for evaluating structure prediction accuracy (CASP) and is calculated by the **TM-align** server/program. When AlphaFold2 achieved a median TM-score of ~0.92 on CASP14 targets, it meant the predictions were, on average, matching the true structures at a level of similarity that previously indicated essentially the same protein from the same organism. That is why CASP14 was treated as a paradigm shift.

## Structural Alignment vs. Sequence Alignment

A critical concept: **structural alignment** can align structurally equivalent residues even when the sequence identity is very low (<20%) — in the "twilight zone" where sequence alignment fails. Two proteins with only 15% sequence identity may share the same fold (e.g., both are TIM-barrel enzymes), and structural alignment by DALI or CE will correctly align them while sequence alignment would give a meaningless alignment.

This is not merely a technical curiosity. It means that structure carries phylogenetic information that sequence has lost. Two enzymes in completely different metabolic pathways, in organisms separated by billions of years of evolution, may reveal a common ancestor through structural alignment even though their sequences have diverged beyond recognition. This observation is one of the central insights motivating structural classification databases — and it gives structural bioinformatics its power to reach further back in evolutionary time than sequence-based methods.

The **DALI** (Distance Alignment) algorithm compares 2D distance matrices — matrices of all pairwise Cα-Cα distances within each structure. Structural equivalence is detected by finding matching blocks of the distance matrix using Monte Carlo optimization. DALI is particularly powerful for detecting remote structural homologs because pairwise distances are invariant to rotation and translation, which means DALI does not need to first superpose the structures.

The **CE** (Combinatorial Extension) algorithm finds the optimal alignment by extending a seed alignment in the sequence direction while monitoring geometric compatibility. CE is faster than DALI and implemented in many structural analysis pipelines.

## Practical Application: Model Validation

When validating a homology model or AlphaFold2 prediction, TM-align the predicted structure against the experimental structure (when available) and report TM-score and RMSD. For CASP predictions, GDT_TS (Global Distance Test — fraction of aligned residues within 1, 2, 4, 8 Å) is also reported:

$$\text{GDT\_TS} = \frac{P_{<1\text{Å}} + P_{<2\text{Å}} + P_{<4\text{Å}} + P_{<8\text{Å}}}{4}$$

AlphaFold2 achieved average GDT_TS > 90 at CASP14, a breakthrough improvement over previous methods. To put that in context: a GDT_TS of 90 means that, on average, 90% of residues in the predicted structure are within 1–8 Å of their true position. Before AlphaFold2, the best methods were achieving GDT_TS around 50–60 on the hardest targets.

## Why This Matters

Pairwise structure comparison is the foundation of structural evolution analysis, model quality assessment, and cross-reactivity prediction for drug molecules. The ability to quantitatively answer "how similar are these structures?" enables computational biology applications from homology-based function prediction to off-target identification in drug discovery. And in the AlphaFold2 era, TM-score has become the primary currency of model evaluation — when a predicted structure is deposited or described, TM-score against the experimental structure (if known) is the headline number. Understanding what that number means, and what its limitations are, is no longer optional background knowledge. It is the standard language of structural bioinformatics.

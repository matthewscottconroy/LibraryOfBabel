# The Homology Modeling Process

Understanding how homology modeling works in detail — not just what button to press, but what each step is actually doing and why it can go wrong — pays dividends long after the field has moved on to AlphaFold2. The conceptual machinery of homology modeling is still operating inside every modern deep learning structure prediction method, just learned rather than hand-coded. When AlphaFold2 uses MSA information to extract evolutionary constraints, it is doing something structurally analogous to template-based modeling, but with billions of training examples rather than explicit rules. Knowing the classical workflow gives you a mental model for what the deep learning methods are implicitly learning to do.

Homology modeling follows a well-defined series of steps: template identification, alignment, model construction, and validation. Each step has specific tools and quality criteria. The process is described below in the order you would execute it, with attention to what goes wrong at each stage.

## Step 1: Template Identification

The starting point is identifying one or more protein structures in the PDB that share evolutionary relationship with the query protein. Two primary tools:

**PSI-BLAST**: Iterative sequence database search. Start with the query sequence, find homologs in nr (non-redundant database), build a Position-Specific Scoring Matrix (PSSM) from the aligned hits, use this profile to find more distant homologs, repeat. PSI-BLAST can detect templates down to ~25–30% identity in favorable cases.

**HHpred**: Profile-profile search that compares the query HMM (hidden Markov model) to HMMs of all proteins in the PDB. More sensitive than PSI-BLAST for the twilight zone. HHpred routinely detects templates with <20% sequence identity and is the recommended first-line tool for difficult targets. The key insight behind HHpred is that comparing the evolutionary variation patterns of two proteins (their HMMs) provides much more information than comparing single sequences — two proteins may differ at every position while still showing the same pattern of conservation and variation, a signature of common descent.

## Step 2: Template Selection

Multiple template structures may be available. Selection criteria:
- **% identity to query**: Higher is better.
- **Resolution**: Prefer higher-resolution structures (< 2.5 Å).
- **Coverage**: Template should cover the full-length query, or at least the domain of interest.
- **Ligand bound**: If modeling for drug docking, prefer a template structure with a ligand bound in the target site (induced-fit or holo structure).
- **Same species or closely related species**: Species-specific insertions/deletions may create alignment errors across very distant species.

Multiple templates can be used simultaneously (multi-template modeling), which generally improves accuracy by covering different regions with the best available template. You might use a high-identity template for the core domain and a separate template from a distantly related organism that captured a particular loop in a well-ordered conformation.

## Step 3: Alignment

The alignment between query sequence and template sequence(s) defines the correspondence between every residue in the query and either a residue in the template or an insertion/deletion. **The alignment is the rate-limiting step** — a single alignment error propagates through the model.

For targets with >40% identity, standard global or local pairwise alignment (MUSCLE, MAFFT) is appropriate. For difficult targets (<30% identity), HHpred profile-profile alignment is essential. Alignments should be manually inspected for:
- Gaps in secondary structure elements (especially in helices and strands — these are structurally implausible)
- Correct alignment of conserved catalytic residues
- Consistency with any known functional constraints

This is the step where expert knowledge makes the biggest difference. An alignment that places a gap in the middle of a helix is almost certainly wrong — helices don't simply stop and restart mid-sequence. If your automated aligner produces such an alignment, you should override it manually based on secondary structure prediction. The alignment is your interpretation of evolutionary history; don't let the algorithm make that interpretation without scrutiny.

## Step 4: Model Building

Given the alignment, the model is constructed by:

1. **Backbone transfer**: Aligned residue positions are assigned the Cα coordinates from the template. For insertions in the query (positions without a template residue), backbone coordinates must be generated de novo — this is **loop modeling**.

2. **Loop modeling**: Insertions and deletions relative to the template are the largest source of uncertainty. **MODELLER** uses a conjugate gradient/MD energy-based protocol to build loop regions. **Rosetta loop modeling** uses a fragment-based approach with a Rosetta energy function. For loops > 8 residues, accuracy is generally poor; this limits model usefulness for large inserted regions.

3. **Side chain placement**: Template side chain coordinates can be transferred where identity is conserved. For non-identical positions, rotamer libraries (Dunbrack backbone-dependent rotamer library) combined with energy optimization select the lowest-energy side chain conformation.

4. **Energy minimization**: Brief restrained minimization (gradient descent or short MD) removes steric clashes introduced during side chain placement, yielding the final model.

MODELLER generates multiple models (typically 5–50) with slightly different loop conformations and random number seeds, each receiving a **DOPE score** (Discrete Optimized Protein Energy) — a statistical potential derived from known structures. The model with the best (most negative) DOPE score is selected.

Loop modeling deserves particular attention because loops are often the most functionally important parts of a protein — active site loops, substrate-binding loops, dimerization interfaces. These are also, unfortunately, the regions that homology modeling handles worst. When you look at a homology model, the regions to trust least are the ones that matter most for function.

## Step 5: Model Validation

**MolProbity** validation: Check Ramachandran outliers, clashscore, rotamer outliers. A good homology model should approach the quality of an experimental structure of comparable resolution.

**DOPE score / ProSA Z-score**: ProSA plots the energy of each residue as a function of position; segments with positive energy (unfavorable) indicate likely modeling errors. The ProSA Z-score quantifies overall model quality relative to experimental structures of the same length (Z-score > −4 for models of short proteins is concerning).

**Structural check against template**: Superpose the model onto the template (TM-align); RMSD for well-aligned core regions should be < 1 Å.

**Active site geometry check**: If the protein is an enzyme, verify that catalytic residues are positioned correctly (appropriate hydrogen-bond distances, correct geometry for the proposed mechanism).

Validation is not just quality control — it often reveals problems with the alignment. If a region of the ProSA plot shows consistently positive energy, go back and look at the alignment in that region. The validation and alignment steps are iterative in practice, not sequential.

## Why This Matters

Understanding the homology modeling process provides mechanistic insight into how structure prediction works, what errors arise, and why template quality and alignment accuracy are so critical — insights directly applicable to interpreting AlphaFold2 predictions, assessing confidence in computationally predicted structures, and designing mutagenesis experiments. AlphaFold2 has automated and greatly improved the implicit equivalent of each step described here; knowing the classical version makes it easier to understand what AlphaFold2's confidence metrics (pLDDT and PAE) are actually measuring, and why low-pLDDT regions correspond so closely to the loop and disordered regions that classical homology modeling also failed to model reliably.

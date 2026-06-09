# When to Use Homology Modeling

For most of the history of structural bioinformatics, homology modeling was the answer to a perennial frustration: you know a protein's sequence, you know it does something important, but no experimental structure exists and crystallization has failed for three years. The solution, developed in earnest through the 1990s and 2000s, was to exploit the observation that protein structure is far more conserved than protein sequence over evolutionary time. If you can find a protein whose structure is known and whose sequence resembles your protein of interest, you can use that known structure as a template to build an approximate model.

Today, that calculus has shifted dramatically. AlphaFold2 has predicted structures for essentially every protein in UniProt — more than 200 million proteins — and for most single-chain proteins, its predictions are better than classical homology models at any sequence identity. You might reasonably ask: is homology modeling still relevant? The answer is yes, but the circumstances where it is the right tool have narrowed considerably. Understanding precisely when classical homology modeling adds value requires understanding what it does, how it works, and where it fails.

Homology modeling (also called comparative modeling) constructs a 3D model of a protein whose structure is unknown, using the experimentally determined structure of a related protein as a template. The approach exploits the principle that protein structure is more conserved than sequence over evolutionary time — proteins with similar sequences almost always have similar folds. However, the reliability of a homology model depends critically on the sequence similarity between query and template.

## The Sequence Identity Threshold for Reliable Modeling

The relationship between sequence identity and model quality is the most important practical consideration:

**> 50% sequence identity** ("safe zone"): The model is expected to be reliable at the backbone level (RMSD ~1 Å for the core) and can be used with reasonable confidence for:
- Visualizing the protein fold and functional sites
- Identifying and modeling bound ligands
- Predicting the consequences of mutations (especially in conserved regions)
- Guiding mutagenesis experiments

**30–50% sequence identity** ("usable zone"): The overall fold is likely correct, but some regions — especially variable loops and the packing of peripheral helices — may deviate substantially from the true structure. Functional site residues in conserved core regions remain reliable; peripheral regions are uncertain.

**< 30% sequence identity** ("twilight zone"): Fold recognition (threading) methods rather than classical alignment-based homology modeling are required. Even with the correct fold template, alignment errors introduce local structural errors that can misplace catalytic residues or misidentify binding pockets. Below ~20% identity ("midnight zone"), even the fold assignment may be incorrect without profile-profile methods (HHpred).

These thresholds were established empirically by comparing homology models to subsequently determined experimental structures and measuring how model quality degraded as sequence identity fell. They represent real knowledge about what the modeling procedure can and cannot accomplish.

## Protein Fold Conservation Beyond Sequence Conservation

Why can two proteins with <20% sequence identity share the same fold? The protein folding code is partially degenerate — many amino acid sequences encode the same thermodynamically stable fold. Only a few "core" positions (hydrophobic core, certain loop-anchoring residues) are strongly constrained to maintain structure; the majority of positions can accommodate diverse amino acids without destabilizing the fold.

This evolutionary principle was demonstrated dramatically by the **"chameleon" experiment**: artificial sequences with deliberately randomized compositions can fold into natural protein folds when key structural positions are maintained. It means that fold identification is the rate-limiting problem at very low sequence identity, not model building per se.

It also means that structure contains information about evolutionary history that sequence has long since lost. When a protein has diverged past 20% identity with all its relatives, its sequence alignment is noise — but its fold still tells a story. This is why structural methods can detect evolutionary relationships that are entirely invisible to BLAST.

## Practical Decision Tree: When to Use What Method

Before investing effort in homology modeling, consider the following workflow:

**Step 1: Search the AlphaFold Protein Structure Database (AlphaFold DB)**. DeepMind has predicted structures for >200 million proteins. If the protein is in AlphaFold DB (virtually all UniProt proteins), download the AlphaFold2 model directly — it is almost certainly better than a classical homology model except in edge cases.

**Step 2: If AlphaFold2 fails or provides low-confidence predictions** (pLDDT < 50 for large regions), and experimental structures are unavailable, classical homology modeling may still be valuable — particularly when modeling a specific variant or mutant relative to a known experimental structure of the wild type (where the AlphaFold2 model may not capture mutation effects).

**Step 3: For drug discovery applications**, experimental structures should be preferred when available (ideally co-crystal structures with a bound ligand). Homology models are used only when no suitable experimental structure exists, because docking accuracy decreases substantially with increasing model error.

**Step 4: For studying conformational changes** induced by ligand binding, neither AlphaFold2 nor homology modeling is reliable — use MD simulations on the experimental structure or seek experimental structures of both conformational states.

These four steps represent a genuine change in practice from the pre-AlphaFold2 world, where step 1 was "search the PDB for homologs" and step 2 was "build a homology model." The default has shifted: AlphaFold2 first, homology modeling only when AlphaFold2 is demonstrably insufficient.

## The Alignment Quality Problem

In homology modeling, **the quality of the sequence alignment between query and template is the rate-limiting step**. Even with a perfect template structure, an alignment error that shifts the query sequence by 1–2 positions in a loop region will misplace every residue in that region. At sequence identities above 40%, sequence alignment tools (Clustal, MUSCLE, MAFFT) produce reliable alignments. Below 30%, profile-profile methods (HHpred) are essential for generating alignments good enough for meaningful modeling.

This is counterintuitive. You might expect the model-building step — the complex 3D optimization — to be where things go wrong. But it turns out the alignment is more critical. A single alignment error in a helix can misplace 10–15 residues by a full turn of the helix. No amount of energy minimization will fix that; the minimization will find the nearest local minimum to the wrong starting point.

## Why This Matters

Understanding when homology modeling is appropriate — and when AlphaFold2, MD simulation, or experimental structure determination is needed instead — prevents wasting resources on low-quality models and ensures that computational structural biology efforts are directed toward questions where they can genuinely add value. In the post-AlphaFold2 era, the most important skill is not knowing how to build a homology model — it is knowing when to build one versus when to use an AlphaFold2 prediction, and in both cases, knowing how to interpret the confidence metrics that tell you which parts of the model to trust.

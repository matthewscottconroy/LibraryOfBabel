# AlphaFold2

In November 2020, at the fourteenth Critical Assessment of Structure Prediction competition (CASP14), a team from DeepMind presented results that most structural biologists initially refused to believe. Their system, AlphaFold2, was predicting protein structures with accuracy comparable to experimental methods — achieving median GDT_TS scores above 90, in a field where scores of 50–60 on hard targets had represented the state of the art. The reaction from the structural biology community ranged from astonishment to skepticism to, gradually, recognition that something profound had changed. Andrei Lupas, a structural biologist at the Max Planck Institute who had been working on a particular difficult target for over a decade, said that AlphaFold2 had solved it. "It would have taken us years to figure it out," he told Science, "but AlphaFold did it in a weekend."

The release of **AlphaFold2** at CASP14 in November 2020 constitutes arguably the most significant advance in computational biology since the sequencing of the human genome. It solved the "protein folding problem" — predicting a protein's three-dimensional structure from its amino acid sequence — with accuracy approaching that of experimental methods for many protein targets.

This section explains how AlphaFold2 works, what its outputs mean, and where it falls short. Understanding all three is essential for using it well.

## The Breakthrough: CASP14 Performance

The Critical Assessment of Structure Prediction (CASP) competition evaluates structure prediction methods against experimentally determined structures withheld from the community. At CASP14, AlphaFold2 achieved a **median GDT_TS score > 90** on 87 targets, and a **median TM-score of ~0.92**. The previous best methods achieved ~GDT_TS 50–60. AlphaFold2 solved structures that had resisted experimental determination for years, including the TBK1:OPTN complex (relevant to ALS) and several multi-domain proteins.

To put GDT_TS > 90 in biological context: this means that, on the hardest targets in the CASP14 set, the predicted structures had 90% of residues within atomic-distance accuracy of the experimentally determined structure. This is not "good enough for a rough model"; it is "sufficient to begin drug design, interpret mutations, and understand mechanism." The wall between "structure known" and "structure unknown" collapsed essentially overnight.

## Architecture

AlphaFold2 processes two input representations:

1. **Multiple Sequence Alignment (MSA)**: Sequences of evolutionarily related proteins found by searching UniRef90 and other databases with HHblits. The MSA captures evolutionary constraints — residue pairs that co-evolve must be in spatial proximity (see coevolutionary methods section).

2. **Template features**: 3D coordinates from structurally similar proteins found in the PDB, providing direct structural information.

These inputs feed into the core network:

**Evoformer**: A stack of ~48 transformer blocks that operate on both the MSA representation (residues × sequences matrix) and a pairwise representation (residues × residues matrix). The Evoformer uses **attention mechanisms** to mix information within and between these two representations. Critically, it uses **paired MSA attention** — attending simultaneously over the query sequence positions and the depth of the MSA — allowing it to extract coevolutionary signal as direct contacts.

If you are familiar with the DCA coevolutionary methods described in the next subsection, you can think of the Evoformer as learning an extraordinarily sophisticated, nonlinear version of direct coupling analysis. Where DCA fits a maximum entropy model to extract direct couplings, the Evoformer learns to extract whatever statistical structure in the MSA is predictive of 3D structure. It has effectively internalized the coevolutionary information that decades of method development had been trying to capture with increasingly sophisticated statistical models.

**Structure Module**: Takes the output of the Evoformer (updated residue pair representations) and produces 3D backbone frames using **Invariant Point Attention (IPA)** — an attention mechanism defined in 3D space that is equivariant to global rotation and translation. Side chains are placed by a torsion angle predictor. The full structure module runs 8 times iteratively.

**Recycling**: The predicted structure from one forward pass is fed back as input coordinates for the next pass, iterated 3–4 times. This iterative refinement substantially improves accuracy.

## Confidence Scores: pLDDT and PAE

AlphaFold2 provides per-residue confidence estimates, which are crucial for interpreting predictions. These are not optional metadata — they are as important as the coordinates themselves, and using an AlphaFold2 model without inspecting the confidence scores is like using a structure with 4 Å resolution for drug design without checking that resolution.

**pLDDT (predicted Local Distance Difference Test)**: Ranges 0–100. Computed per residue, it estimates the expected LDDT score for that residue if the structure were compared to the true structure.

- pLDDT > 90: **Very high confidence** — likely accurate within ~1 Å of true structure
- pLDDT 70–90: **Confident** — generally correct backbone; some uncertainty in side chains
- pLDDT 50–70: **Low confidence** — broadly correct but details uncertain; treat with caution
- pLDDT < 50: **Very low confidence** — the region is likely intrinsically disordered or the prediction is unreliable; do not use for structural analysis

Regions with consistently low pLDDT across the entire protein (not just flexible loops) suggest intrinsically disordered regions (IDRs) — regions that lack a stable 3D structure in isolation. AlphaFold2 does not reliably distinguish "disordered" from "I cannot model this region." This is an important limitation: low pLDDT might mean the protein is intrinsically disordered, or it might mean the MSA was too shallow to constrain the structure. Additional analysis is needed to distinguish these cases.

**PAE (Predicted Aligned Error)**: A residue × residue matrix where `PAE[i][j]` is the expected error (in Å) of residue j's position when the structure is aligned at residue i. Low PAE between two residues means they are confidently positioned relative to each other — they are in the same rigid body. High PAE between two domains means their relative orientation is uncertain. PAE is essential for interpreting multi-domain proteins and protein complexes.

The PAE matrix is one of AlphaFold2's most powerful features and one of its most underused. When you see a multi-domain protein with two globular domains connected by a linker, the PAE matrix tells you whether those two domains are confidently positioned relative to each other (low PAE across the domain boundary) or whether only the individual domain structures are reliable while their relative orientation is essentially guessed (high PAE across the boundary). This distinction is critical for any analysis that depends on the arrangement of multiple domains.

## Key Limitations

**Conformational changes**: AlphaFold2 predicts a single structure — typically close to the active/apo or ground state conformation. It cannot predict open/closed conformational changes, allosteric mechanisms, or alternative binding-induced conformations.

**Binding-induced folding**: Intrinsically disordered proteins that fold upon binding a partner are not modeled in their bound conformation; the model shows them as disordered (low pLDDT).

**Oligomeric states**: Standard AlphaFold2 predicts monomers. AlphaFold-Multimer extends this to predict homo- and hetero-oligomers, but accuracy for non-obligate complexes and large assemblies remains imperfect.

**Shallow MSAs**: For orphan proteins with few homologs (< 30 sequences in the MSA), AlphaFold2's accuracy drops substantially because the coevolutionary information is insufficient. This is not an edge case — many proteins from newly sequenced organisms or rapidly evolving pathogens have sparse MSAs, and for these the pLDDT is also a useful warning signal.

**PTMs and ligands**: AlphaFold2 predicts the apo protein structure; it does not model post-translational modifications or bound small molecules. The active site geometry predicted in the absence of substrate may differ from the true substrate-bound conformation.

## Practical Use

```bash
# Run AlphaFold2 locally (Google Colab version available for small proteins)
python run_alphafold.py \
    --fasta_paths=query.fasta \
    --output_dir=output/ \
    --model_preset=monomer \
    --db_preset=full_dbs \
    --max_template_date=2020-05-14
```

For most users, the **AlphaFold Database** (alphafold.ebi.ac.uk) and **ColabFold** (a faster, Colab-accessible implementation using MMseqs2 for MSA generation) are the most practical access points. ColabFold dramatically reduces MSA generation time (from hours to minutes) by using MMseqs2 precomputed databases, making it the practical choice for most users without access to the large computing resources required to run the full AlphaFold2 pipeline locally.

## Why This Matters

AlphaFold2 has made structural information available for essentially the entire known protein universe — >200 million predicted structures — democratizing structural biology and transforming how researchers design experiments, interpret mutations, and develop drugs; it is now the starting point for virtually every structural analysis where an experimental structure is unavailable. But democratization comes with responsibility: a field where everyone can generate a structure in minutes also needs everyone to understand what those structures mean and where they can be wrong. The confidence metrics described in this section are not decorative — they are the essential vocabulary for using AlphaFold2 responsibly.

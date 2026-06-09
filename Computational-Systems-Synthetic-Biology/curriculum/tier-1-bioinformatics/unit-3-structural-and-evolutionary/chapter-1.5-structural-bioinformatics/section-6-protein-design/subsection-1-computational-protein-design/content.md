# Computational Protein Design

In 2003, David Baker's group at the University of Washington published a paper in Science describing a computationally designed protein they called Top7. It had a fold not found in any natural protein. Its sequence had no detectable similarity to any known protein. And when they synthesized it and characterized it in the laboratory, it folded exactly as designed — the crystal structure matched the computational model at 1.17 Å RMSD. Nature had taken billions of years to explore protein sequence space by random mutation and selection. A computer had found a stable, novel protein in a different corner of that space in days.

Top7 was proof of concept. It demonstrated that the rules of protein folding were understood well enough not just to predict what a given sequence folds into, but to design sequences that fold into a specified novel structure. If structure prediction asks "given this sequence, what is the structure?", **computational protein design** inverts the problem: "given this desired structure and function, what sequences will fold into it?" This inverse folding problem has proven enormously fruitful — computational design has produced proteins with binding specificities, catalytic activities, and structural geometries not found in nature.

## The Rosetta Design Framework

**Rosetta** (Baker lab, University of Washington) is the most comprehensive framework for computational protein design. At its core, design proceeds by **Monte Carlo sampling of sequence space**: starting from a backbone structure, the program explores different amino acid identities at each designable position, accepting or rejecting changes based on the Rosetta all-atom energy function.

**The Rosetta energy function** (Ref2015) contains terms for:
- **Van der Waals interactions** (Lennard-Jones potential): Rewards close-packing; penalizes steric clashes.
- **Electrostatics** (implicit GB solvation): Long-range charge-charge interactions.
- **Solvation** (Lazaridis-Karplus): The cost of burying polar groups; reward for burying hydrophobic groups.
- **Hydrogen bonds**: Explicit directional hydrogen bond terms for backbone-backbone, backbone-sidechain, and sidechain-sidechain hydrogen bonds.
- **Rotamer entropy**: Statistical potential derived from the Dunbrack rotamer library; penalizes strained rotamer conformations.
- **Disulfide geometry**: For designed disulfides.

The energy function captures the same physical interactions that determine experimental binding affinity in docking, but here applied to the problem of finding sequences whose ground state is a specified fold. The logic is: a protein is stable if the designed sequence is in a lower free energy state in the designed fold than in any other conformation. Design is the search for sequences that satisfy this condition while also satisfying whatever functional constraints you impose.

## Fixed-Backbone vs. Flexible-Backbone Design

**Fixed-backbone design** keeps the protein backbone (Cα coordinates, φ/ψ angles) fixed while sampling amino acid identities and rotamers. This is appropriate when: the backbone is well-defined (high-resolution crystal structure); small stability improvements or interface optimization (antibody engineering); local active site redesign. Dead-end elimination (DEE) and integer linear programming (ILP) can find the global energy minimum sequence for fixed backbones.

**Flexible-backbone design** allows backbone conformation to change simultaneously with sequence. This is essential for: de novo design (no initial structure); large sequence changes that require backbone adaptation; loop design. RFdesign and RFdiffusion use this approach.

Fixed-backbone design is mathematically tractable but biologically limiting. Proteins are not rigid — they breathe, flex, and often undergo large conformational changes upon ligand binding. Flexible-backbone design captures this reality but at the cost of a much larger and more complex optimization problem. The development of methods that can handle backbone flexibility while maintaining design control is one of the central challenges of the field.

## De Novo Protein Design vs. Stability Engineering

**Stability engineering**: Starting from a natural protein, computational design predicts mutations that improve thermodynamic stability. **FoldX** and Rosetta ΔΔG prediction estimate stability changes for each possible single mutation; the top-ranked stability-improving mutations are introduced experimentally. This is routinely used to improve enzyme stability for industrial biotechnology and to stabilize antibodies for therapeutic development.

**De novo protein design**: Design a new protein from scratch — no natural protein template. The designer specifies a desired structural topology (e.g., a four-helix bundle with a metal-binding site), and Rosetta generates a backbone consistent with the topology, then designs sequences that fold into it. Remarkably successful examples include: designed mini-proteins with enhanced penetration into cells, computationally designed enzyme active sites installed into scaffold proteins, and entire de novo protein families not found in nature.

The distance between stability engineering and de novo design represents an enormous increase in ambition and difficulty. Stability engineering works because you are exploring a small neighborhood around a naturally stable protein — most mutations are small perturbations. De novo design requires finding a rare sequence in a vast space that folds stably into a specific novel topology. That Top7 succeeded at all was surprising; that subsequent designs have increasingly succeeded at harder targets is a testament to how far the energy functions and sampling algorithms have improved.

## RFdiffusion and ProteinMPNN: Generative Design

Recent deep learning tools have transformed protein design:

**RFdiffusion** (Watson et al., 2023) uses a diffusion model trained on PDB structures to generate novel protein backbones conditioned on user-specified constraints — a binding site geometry, a desired protein-protein interface, or a functional motif. The generated backbones represent new folds not found in the PDB.

**ProteinMPNN** (Dauparas et al., 2022) is an inverse folding neural network that designs amino acid sequences for a given backbone, massively outperforming Rosetta sequence design for most applications. The message-passing architecture explicitly models protein structure topology to generate sequences that fold into the target backbone.

Combined RFdiffusion (backbone generation) + ProteinMPNN (sequence design) + AlphaFold2 (validation) has achieved remarkable successes: designing binders to previously undruggable targets, generating novel enzymes, and creating protein nanoparticles for vaccine delivery.

This combination is worth pausing on. Three deep learning systems — a diffusion model, an inverse folding network, and a structure predictor — working together can design proteins with specified functions from scratch. The validation step (AlphaFold2 predicts the designed sequence, and you check whether the prediction matches the design) has become the standard in-silico filter before experimental synthesis. Designs where AlphaFold2 confidently predicts the intended structure are far more likely to fold correctly in the lab. This computationally-guided synthesis selection has dramatically improved the experimental success rate of designed proteins.

## Validation Pipeline

Computational design predictions must always be experimentally validated:

1. **Sequence synthesis**: The designed gene is synthesized (codon-optimized for the expression host).
2. **Expression**: Produced in *E. coli*, yeast, or mammalian cells, depending on the application.
3. **Purification**: Affinity chromatography (His-tag, Strep-tag) + SEC to confirm monodispersity.
4. **Structural validation**:
   - **Circular dichroism (CD) spectroscopy**: Secondary structure content (α-helical proteins show 208/222 nm minima); thermal denaturation provides Tm.
   - **NMR** (for small proteins): ¹H-¹⁵N HSQC shows whether the protein is folded and has the expected dispersion.
   - **X-ray crystallography or cryo-EM**: Confirms the designed structure at atomic resolution.
5. **Functional validation**: Binding affinity (SPR, ITC), enzyme activity assay.

## Why This Matters

Computational protein design has moved from curiosity to therapeutic pipeline — with designed antibodies, enzymes, and vaccines entering clinical trials — demonstrating that the rules of protein folding are sufficiently well understood to engineer new proteins for defined functions; this capacity is foundational to synthetic biology and the future of molecular medicine. The same deep understanding of structure-function relationships that AlphaFold2 exploits for prediction, computational design exploits in reverse for engineering. Both the success of AlphaFold2 and the success of de novo protein design are consequences of the same underlying truth: protein sequence space is navigable, and the map is the energy function.

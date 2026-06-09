# Structural and Interaction Proteomics

A protein's sequence tells you what it can potentially do. Its expression level tells you how much of it is available. But what a protein actually does in the cell depends almost entirely on its physical context: which other proteins it contacts, which ligands it binds, which regions are flexible versus rigid, and how its conformation changes in response to signals. Two proteins can be present at identical levels in two different cells but perform completely different functions because their interaction partners, their conformational states, and their post-translational decorations are different.

The techniques in this subsection address that physical context directly. They do not ask "how much protein X is there?" but rather "what does protein X touch, and what does it look like when it touches it?" Beyond identifying and quantifying proteins, mass spectrometry can characterize protein three-dimensional structure, conformational dynamics, and protein-protein interactions. These applications — collectively termed **structural and interaction proteomics** — use chemical probes, affinity purification, or hydrogen isotope exchange to report on the physical state of proteins in their native cellular context.

## Crosslinking Mass Spectrometry (XL-MS)

**XL-MS** uses bifunctional chemical crosslinkers to covalently link amino acid residues that are within a defined spatial distance in the native protein structure. The resulting crosslinked peptides — containing two peptides joined by the crosslinker — are detected by LC-MS/MS and provide distance constraints (the crosslinker defines a maximum Cα-Cα distance, typically 25–35 Å for NHS ester crosslinkers).

**Commonly used crosslinkers**:
- **DSS/BS3** (disuccinimidyl suberate/bis-sulfosuccinimidyl suberate): Reactive toward primary amines (lysine ε-amine and protein N-terminus). Spacer arm: 11.4 Å. BS3 is the water-soluble version.
- **DSSO** (disuccinimidyl sulfoxide): Cleavable crosslinker with two MS-labile bonds, enabling MS2/MS3 analysis for crosslink identification.
- **EDC** (carbodiimide): Zero-length crosslinker linking carboxylate to amine (aspartate/glutamate to lysine), reporting on very close contacts.

**Data analysis**: Crosslinked peptide pairs are detected as ions with masses equal to peptide A + peptide B + crosslinker mass. Tools like **pLink2**, **Kojak**, and **XlinkX** identify these complex species by database searching. The identified crosslinks are used as distance constraints for computational structural modeling (Rosetta, HADDOCK) or for validating AlphaFold2 predictions.

The combination of XL-MS with AlphaFold2 is particularly powerful. AlphaFold2 produces structure predictions for individual proteins with remarkable accuracy, but it struggles with protein complexes, especially transient or condition-dependent ones. XL-MS provides experimental distance constraints that can guide and validate complex structure modeling, catching cases where AlphaFold's predicted interface geometry disagrees with the actual crosslink pattern observed in living cells. This experimental-computational hybrid is increasingly the standard for characterizing large macromolecular assemblies.

## Affinity Purification Mass Spectrometry (AP-MS)

**AP-MS** identifies protein-protein interactions by pulling down a bait protein (typically tagged with GFP, FLAG, or streptavidin-binding peptide) along with any associated proteins (**prey**), then identifying the prey by shotgun proteomics.

The critical challenge is distinguishing **specific** interactions from **non-specific** background (proteins that bind to the bead, tag, or lysis conditions). **SAINT** (Significance Analysis of INTeractome) is the standard statistical tool: it models the distribution of bait-specific vs. background spectral counts using a Bayesian mixture model and assigns each bait-prey pair a **SAINT score** (probability of being a true interaction). A SAINT score ≥ 0.8 and requiring ≥2 unique peptides per prey are standard filters.

AP-MS is particularly powerful for mapping **protein complexes** in a cellular context, profiling interaction changes under different conditions, and identifying off-target interactors of drugs.

You might expect that the hardest part of AP-MS would be the mass spectrometry. In practice, the hardest part is the biology: ensuring that your tagged bait protein is expressed at near-endogenous levels (overexpression creates spurious interactions), that your lysis conditions preserve native complexes (harsh lysis can destroy transient interactions while also releasing nuclear proteins that promiscuously bind the bait), and that your controls adequately sample the background. The SAINT statistical framework helps, but it cannot compensate for a poorly controlled experiment.

## HDX-MS: Probing Protein Dynamics

**Hydrogen-deuterium exchange mass spectrometry (HDX-MS)** monitors the exchange of backbone amide hydrogens with deuterium from the solvent (D₂O). Amide hydrogens that are hydrogen-bonded (in secondary structures) or buried in the protein core exchange more slowly than those on exposed, flexible loops.

Workflow:
1. Incubate protein in D₂O buffer for various time points (seconds to hours).
2. Quench exchange (pH 2.5, 0°C) to slow back-exchange.
3. Digest with pepsin (active at pH 2) on a cold column.
4. Analyze peptides immediately by LC-MS to measure deuterium incorporation at each peptide.

The output is a **deuterium uptake map**: regions of high uptake = exposed/dynamic; regions of low uptake = structured/protected. HDX-MS is particularly powerful for:
- Mapping binding interfaces: A ligand that shields a surface reduces deuterium uptake at the binding site relative to unbound protein.
- Detecting allosteric changes: Binding at site A changes dynamics at remote site B.
- Comparing conformational states of the same protein under different conditions.

HDX-MS offers a window into protein dynamics that static structure methods cannot provide. A crystal structure of a kinase shows you the inactive conformation; HDX-MS on the same kinase in solution, with and without ATP, with and without its substrate, shows you how different binding events reorganize the flexibility of the entire protein. Regulatory mechanisms that are invisible in crystal structures — allosteric communication paths, disorder-to-order transitions, dynamic loops — are directly readable from differential deuterium uptake maps.

## CRISPR-Based Proximity Labeling: BioID, TurboID, APEX2

**Proximity labeling** identifies all proteins within ~10 nm of a bait protein in the living cell by using an engineered enzyme fused to the bait that biotinylates nearby proteins. After cell lysis, biotinylated proteins are captured on streptavidin beads and identified by LC-MS/MS.

- **BioID/BioID2**: Mutant biotin ligase from *Brevundimonas* that promiscuously biotinylates nearby proteins over 18–24 hours. Slow but specific; useful for identifying stable and transient interactions at specific subcellular locations.
- **TurboID**: Evolved biotin ligase with 100-fold higher activity; enables proximity labeling in 10 minutes, reducing background from protein diffusion.
- **APEX2**: Ascorbate peroxidase that generates biotin-phenol radicals in the presence of H₂O₂; labeling occurs in <1 minute, providing the highest temporal resolution.

Proximity labeling is particularly powerful for: protein interactions at membrane-proximal locations (which AP-MS misses due to solubilization requirements), dynamic interactions (transient complexes), and mapping the composition of organelles or nuclear bodies.

The APEX2 approach deserves special notice for its temporal resolution. With a labeling pulse of less than one minute triggered by adding H₂O₂, you can capture the protein neighborhood of your bait at a defined moment — during mitosis, immediately after receptor activation, in the seconds following a stress stimulus. This turns proximity labeling into a kind of spatiotemporal molecular snapshot, enabling experiments that were previously impossible: what proteins transiently visit the vicinity of a nuclear pore during the first 60 seconds of mitotic exit? Who are the neighbors of a misfolded protein in the first minutes after heat shock? These questions are now answerable.

## Why This Matters

Structural and interaction proteomics move beyond "what is expressed" to "how proteins physically interact and behave" — questions essential for understanding signal transduction, drug mechanism of action, and the structural basis of disease mutations — making these methods indispensable complements to genomics and standard expression proteomics. Drug targets are not just proteins that are differentially expressed; they are proteins whose interactions, conformational states, or enzymatic activities are altered in disease. Structural and interaction proteomics are the methods that connect the static map of protein expression to the dynamic machinery of cellular function.

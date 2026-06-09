# Homology Modeling Tools

The landscape of protein structure modeling tools has been transformed by a single paper — AlphaFold2 (Jumper et al., 2021) — but that transformation does not render the classical tools irrelevant. It does change how and when they are used, and understanding the full toolkit — its history, its strengths, and its current place in structural bioinformatics — prepares you to make good choices across a wide range of problems.

Several software tools and web servers implement the homology modeling process, each with different strengths in template selection, loop modeling, and scoring. The description below covers the major tools in roughly historical order, leading to the deep learning methods that now define the field's frontier.

## MODELLER: The Python-API Standard

**MODELLER** (Sali & Blundell, 1993; maintained by Andrej Sali's group at UCSF) is the most widely used standalone homology modeling program. It implements **spatial restraint satisfaction**: rather than building a model by energy minimization from scratch, MODELLER converts the aligned template coordinates and a statistical potential (derived from the PDB) into distance and dihedral angle restraints, then optimizes the model by conjugate gradient minimization and MD to satisfy all restraints simultaneously.

The elegance of this approach is that it treats the modeling problem as a constrained optimization: find the configuration of atoms that best satisfies all the information available — template geometry, stereochemical constraints, and the statistical potential. This is mathematically well-defined and produces models with good stereochemistry.

**Python API workflow**:

```python
from modeller import *
from modeller.automodel import *

log.verbose()
env = Environ()
env.io.atom_files_directory = ['.', 'pdbs/']

# automodel is the simplest class for standard modeling
a = automodel(env,
              alnfile='query_template.pir',    # PIR-format alignment
              knowns='1abc',                    # Template PDB code
              sequence='my_query')             # Query sequence name
a.starting_model = 1
a.ending_model = 5                             # Generate 5 models
a.make()

# Select the model with the best DOPE score
models = [x for x in a.outputs if x['failure'] is None]
best_model = min(models, key=lambda m: m['DOPE score'])
print(best_model['name'], best_model['DOPE score'])
```

**Loop refinement**: The `loopmodel` class in MODELLER performs additional Monte Carlo sampling specifically for loop regions, improving accuracy for difficult insertions relative to the template.

**Key strength**: Fine-grained control over modeling parameters; generates multiple models for ensemble analysis; widely validated benchmark performance.

## SWISS-MODEL: The Automated Web Server

**SWISS-MODEL** (Waterhouse et al., 2018; swissmodel.expasy.org) is the most popular web server for homology modeling, providing an automated pipeline accessible without programming knowledge:

1. Submit a sequence via the web interface.
2. SWISS-MODEL automatically searches for templates using HHpred and BLAST.
3. For the top templates (or user-selected ones), builds models using ProMod3 (an internal engine derived from MODELLER).
4. Reports GMQE (Global Model Quality Estimate, 0–1) and QMEAN (Qualitative Model Energy Analysis, a Z-score) for each model.

SWISS-MODEL is appropriate for quick, exploratory modeling where the user wants a rapid first-pass structure. For publication-quality models requiring loop refinement or difficult targets, MODELLER with manual alignment curation is preferred.

It turns out that the most common mistake with SWISS-MODEL is accepting the top-ranked template automatically without inspecting whether it makes biological sense. A template with 55% coverage and 30% identity to the query might rank higher by some metric than a template with 95% coverage and 25% identity, but the latter might be more appropriate for your question. Always look at what SWISS-MODEL chose before trusting the result.

## Rosetta CM: Fragment-Based Comparative Modeling

**Rosetta CM** (RosettaCM, Song et al., 2013) uses a hybrid strategy: conserved regions are transferred from templates (as in standard homology modeling), but unaligned regions (loops, N/C termini) are modeled using Rosetta's fragment insertion and full-atom energy minimization. RosettaCM can also recombine information from multiple templates by hybridizing segments from different PDB structures in an MC search. Its full-atom Rosetta energy function provides excellent side chain placement.

**Best use case**: Multi-template modeling where segments of a query can be modeled from different templates; difficult loop regions; when side chain accuracy is critical (e.g., for molecular docking).

## I-TASSER: Threading + Ab Initio

**I-TASSER** (Iterative Threading ASSEmbly Refinement, Yang & Zhang lab) combines:
1. Multiple threading algorithms (to detect templates even at very low sequence identity)
2. Ab initio modeling for regions lacking template coverage
3. Iterative structural refinement by fragment assembly

I-TASSER is particularly useful for targets with no easily detectable template (< 20% sequence identity) where pure homology modeling would fail. It provides C-score (confidence score) and TM-score estimates.

## RoseTTAFold and the Transition to Deep Learning

**RoseTTAFold** (Baek et al., 2021, Baker lab) was a landmark deep learning model for structure prediction released just before AlphaFold2. It uses a three-track architecture (1D sequence + 2D pairwise + 3D structure tracks) to simultaneously process multiple sequence alignments and predict 3D structure with AlphaFold2-comparable accuracy on most targets. RoseTTAFold is still valuable for multimer prediction and specialized applications.

RoseTTAFold represented a conceptual leap: instead of explicitly doing template selection, alignment, and model building as separate steps, the neural network learns to implicitly perform all of these operations simultaneously from MSA data. The success of RoseTTAFold and AlphaFold2 demonstrated that the right way to do template-free and template-based modeling is not to separate them at all — it is to let a deep network discover whatever combination of template-like and de novo reasoning is most useful for each part of the sequence.

## AlphaFold2: The New Baseline

**AlphaFold2** (Jumper et al., 2021, DeepMind) has fundamentally changed the homology modeling landscape. For single-chain protein structure prediction, AlphaFold2's median TM-score of >0.92 on CASP14 targets exceeds what classical homology modeling achieves even with excellent templates at >50% identity. The practical implication:

**For single-chain proteins**: Download the AlphaFold2 model from the AlphaFold Database first. Use classical homology modeling only when: (1) the protein is not in AFDB; (2) you need to model a specific mutant relative to an experimental structure of the wild type; (3) you are modeling ligand-induced conformational changes.

**For protein complexes**: AlphaFold-Multimer (Evans et al., 2022) and RoseTTAFold2 predict protein-protein and protein-DNA/RNA complexes with remarkable accuracy, increasingly superseding traditional docking for this application.

The practical workflow has changed: where once a researcher might spend weeks setting up MODELLER runs, curating alignments, and validating models, the same task now takes minutes with AlphaFold2. The time savings are real and substantial, and the quality improvement is equally real. But classical tools remain indispensable for the edge cases — mutant modeling, ligand-induced conformational change studies, and situations where you need fine-grained control over exactly how the model was built and what assumptions were made.

## Why This Matters

The practical toolkit for structural modeling has been transformed by AlphaFold2 — but understanding MODELLER, SWISS-MODEL, and their underlying principles remains necessary for cases where deep learning predictions are insufficient, for interpreting the limitations of any predicted structure, and for specialized applications like variant modeling and loop refinement. The tools described in this section will continue to be used in research and pharmaceutical development for years to come, not because AlphaFold2 failed, but because the range of structural modeling problems is broader than what any single method can solve optimally.

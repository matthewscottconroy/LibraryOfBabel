# The GEM Reconstruction Workflow

## Overview

Building a GEM for a new organism is not primarily a computational task — it is a scientific one. You are assembling evidence from genomics, biochemistry, and physiology into a coherent and testable quantitative model. The bioinformatics is the scaffolding; the scientific judgment is the structure.

Reconstructing a GEM for a new organism is a multi-stage process that combines automated bioinformatics with extensive manual curation. The process typically takes months to years for a high-quality, manually curated model, though automated tools can produce a draft in hours. Understanding the workflow is important both for building new models and for critically evaluating the quality of published ones.

## Stage 1: Genome Annotation

The starting point is a sequenced and annotated genome. Genes are assigned functions by:

- **Homology-based annotation**: BLAST against characterized proteins in databases (UniProt, RefSeq)
- **Domain annotation**: Pfam, TIGRFAM domain profiles to assign enzyme families
- **Pathway databases**: KEGG Orthology (KO) assignments; MetaCyc pathway membership
- **Ortholog groups**: eggNOG, COG assignments for functional classification

The quality of the genome annotation directly determines the completeness of the draft GEM. Poorly annotated genomes produce GEMs with many gaps.

## Stage 2: Draft Reconstruction

Automated tools generate a draft model from the genome annotation:

**CarveMe** (Machado et al. 2018): the current standard for automated GEM reconstruction.
```bash
# Command-line CarveMe usage
carve genome.faa -o draft_model.xml \
  --solver cplex \
  --refseq genome_refseq_id \
  --media M9  # optional: medium-specific gap filling
```

CarveMe uses a reference database of curated reactions (BiGG) and matches annotated proteins to known enzymes via BLAST. It returns a draft SBML model with GPR associations and initial flux bounds.

**ModelSEED**: alternative pipeline; starts from RAST annotation; includes automated gap filling; large collection of pre-built models available.

```python
# Alternative: build from KEGG annotation using COBRApy utilities
import cobra
from cobra.io import load_model

# Load a template model (closely related organism)
template = load_model('iJO1366')  # E. coli as template for a related species

# The actual reconstruction would involve:
# 1. Get gene list for new organism from KEGG/UniProt
# 2. Map to reaction identifiers via orthology
# 3. Build S matrix from mapped reactions
# 4. Manually curate GPR associations
```

## Stage 3: Manual Curation

The draft model requires substantial manual review. This is where the science happens — and where shortcuts lead to unreliable models.

**Mass and charge balance**: Check that every reaction conserves mass (every element balanced) and charge (total charge conserved). Automated tools often produce unbalanced reactions, particularly for:
- Proton-coupled transporters (H⁺ stoichiometry)
- Reactions involving CoA (complex molecular weight)
- Redox reactions (NAD⁺/NADH stoichiometry)

**Biomass reaction construction**: Measure the organism's dry weight composition (protein, RNA, DNA, lipid, carbohydrate fractions) using standard biochemical assays. Determine amino acid composition (proteomics), nucleotide composition (HPLC), lipid profile (lipidomics). Construct the biomass reaction stoichiometrically from these data.

**Directionality assignment**: Use eQuilibrator (Section 2.2.5.2) to compute $\Delta_r G^{\circ\prime}$ for each reaction and assign thermodynamically correct directionality. Reactions with $\Delta_r G^{\circ\prime} \gg 0$ are physiologically irreversible in the forward direction and should be set as irreversible.

**Consistency checks**:
```python
import cobra
model = cobra.io.read_sbml_model('draft_model.xml')

# Check 1: Can the model grow?
sol = model.optimize()
print(f"Growth rate: {sol.objective_value:.4f} h⁻¹")

# Check 2: Blocked reactions (no feasible flux)
from cobra.flux_analysis import find_blocked_reactions
blocked = find_blocked_reactions(model)
print(f"Blocked reactions: {len(blocked)}")
# -> These need gap filling

# Check 3: Reactions producing energy without substrates (energy-generating cycles)
from cobra.flux_analysis.loopless import loopless_solution
# Thermodynamically infeasible loops can inflate apparent growth

# Check 4: Mass balance
unbalanced = []
for rxn in model.reactions:
    if rxn.check_mass_balance():
        unbalanced.append(rxn.id)
print(f"Unbalanced reactions: {len(unbalanced)}")
```

## Stage 4: Gap Filling

A metabolic network with gaps cannot carry flux to produce biomass. **Gap filling** adds the minimum set of reactions needed to restore connectivity — either because the enzyme is present but unannotated, or because a reaction from a universal database is needed to bridge a metabolic gap.

```python
from cobra.flux_analysis import gapfill

# Gap fill using a universal database model
universal = cobra.io.load_model('bigg_universal')  # universal reaction database

# Find reactions to add to make model grow
solution = gapfill(model, universal, demand_reactions=False)
for i, solutions in enumerate(solution):
    print(f"Solution {i}: add reactions {[r.id for r in solutions]}")
```

Gap filling adds reactions, but biological validation is required: does the organism actually have this enzyme? Is there a unannotated gene with homology to the enzyme? Gap-filled reactions with no genomic support are flagged as "inferred" rather than "genomic."

## Stage 5: Phenotypic Validation

Validate the model against growth/no-growth data across different media conditions:

**Biolog phenotype microarray**: measures growth of the organism in 96-well plates with different carbon sources. Compare model predictions (FBA grow/no-grow) to experimental data.

```python
import pandas as pd

# Experimental Biolog data
biolog_data = pd.read_csv('biolog_phenotype.csv')  # columns: carbon_source, growth (1/0)

# Model predictions
predictions = []
for _, row in biolog_data.iterrows():
    with model:
        # Set single carbon source
        for rxn in model.reactions:
            if rxn.id.startswith('EX_') and rxn.lower_bound < 0:
                rxn.lower_bound = 0
        # Enable this carbon source
        try:
            carbon_rxn = model.reactions.get_by_id(f"EX_{row['carbon_id']}_e")
            carbon_rxn.lower_bound = -10
        except KeyError:
            predictions.append({'carbon_source': row['carbon_source'], 'predicted': 0})
            continue
        sol = model.optimize()
        predicted = 1 if sol.objective_value > 1e-6 else 0
        predictions.append({'carbon_source': row['carbon_source'], 'predicted': predicted})

pred_df = pd.DataFrame(predictions)
merged = biolog_data.merge(pred_df, on='carbon_source')
accuracy = (merged['growth'] == merged['predicted']).mean()
print(f"Phenotypic prediction accuracy: {accuracy:.1%}")
```

Target accuracy for a well-curated GEM: >90% on the conditions used for curation.

## Why This Matters

The reconstruction workflow illustrates that GEMs are not just computational tools — they are knowledge-intensive scientific objects that consolidate decades of biochemistry, genomics, and physiology into a consistent quantitative framework. The manual curation steps — mass balance, biomass composition, directionality, gap filling, validation — are where biological knowledge matters most and where errors have the largest impact. Understanding this workflow enables critical evaluation of published GEMs (how carefully was it curated?) and guides the reconstruction of new models for understudied organisms with potential biotechnological or medical importance.

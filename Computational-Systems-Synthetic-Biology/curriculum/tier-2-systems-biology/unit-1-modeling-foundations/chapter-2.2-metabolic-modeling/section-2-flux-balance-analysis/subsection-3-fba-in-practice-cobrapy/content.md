# FBA in Practice: COBRApy

## COBRApy: The Standard Python Interface

The stoichiometric matrix is constructed. The constraints are specified. The LP is formulated. Now you need to actually solve it. In the Python ecosystem, that means COBRApy.

**COBRApy** (Constraint-Based Reconstruction and Analysis for Python) is the primary software library for FBA and related analyses in Python. It provides:

- Model loading/saving in SBML, JSON, and MATLAB formats
- Reaction, metabolite, and gene object management
- FBA, pFBA, and FVA solvers
- Gene knockout simulations
- Medium management
- Integration with escher for flux map visualization

The COBRApy API is structured around three core objects: `Model`, `Reaction`, and `Metabolite`, forming an intuitive hierarchy that mirrors the biological concepts. Once you are comfortable with these three objects and how they relate, the entire library feels natural.

## Loading and Inspecting a Model

```python
import cobra
from cobra.io import read_sbml_model

# Load the E. coli GEM
model = read_sbml_model('iJO1366.xml')

# Basic statistics
print(f"Genes: {len(model.genes)}")
print(f"Reactions: {len(model.reactions)}")
print(f"Metabolites: {len(model.metabolites)}")

# Inspect a specific reaction
rxn = model.reactions.get_by_id('PFK')
print(f"\nPhosphofructokinase (PFK):")
print(f"  Equation: {rxn.reaction}")
print(f"  Bounds: [{rxn.lower_bound}, {rxn.upper_bound}]")
print(f"  GPR: {rxn.gene_reaction_rule}")

# List metabolites of a reaction
for met, coeff in rxn.metabolites.items():
    print(f"  {met.id}: {coeff} (compartment: {met.compartment})")
```

## Running FBA and Examining Results

```python
# Standard FBA under glucose aerobic conditions (default in iJO1366)
sol = model.optimize()

print(f"Objective value (growth rate): {sol.objective_value:.4f} h⁻¹")
print(f"Status: {sol.status}")

# Access individual fluxes
fluxes = sol.fluxes  # pandas Series indexed by reaction ID
print(f"\nKey fluxes (mmol/gDW/h):")
key = ['EX_glc__D_e', 'EX_o2_e', 'EX_co2_e', 'EX_ac_e',
       'PFK', 'CS', 'CYTBO3_4pp']
for rxn_id in key:
    if rxn_id in fluxes.index:
        print(f"  {rxn_id:20s}: {fluxes[rxn_id]:8.3f}")

# Shadow prices (marginal value of metabolites)
shadow = sol.shadow_prices
print(f"\nTop 5 most limiting metabolites (shadow prices):")
print(shadow.nsmallest(5))
```

## Context Manager Pattern for Temporary Changes

The context manager (`with model:`) is essential for temporary modifications — it automatically reverts all changes when the block exits. Without it, you would need to manually restore every bound you changed, which is error-prone and makes comparison experiments painful. Get in the habit of using context managers for any simulation that modifies the model transiently:

```python
# Compare multiple conditions without modifying the original model
print("Growth rate comparison across conditions:")
conditions = {
    'Glucose aerobic':   {'EX_glc__D_e': -10, 'EX_o2_e': -20},
    'Glucose anaerobic': {'EX_glc__D_e': -10, 'EX_o2_e': 0},
    'Acetate aerobic':   {'EX_ac_e': -10, 'EX_o2_e': -20},
    'Succinate aerobic': {'EX_succ_e': -10, 'EX_o2_e': -20},
}

for condition, media in conditions.items():
    with model:
        # First close all carbon uptakes
        for rxn in model.reactions:
            if rxn.id.startswith('EX_') and rxn.lower_bound < 0:
                rxn.lower_bound = 0
        # Set new media
        for rxn_id, lb in media.items():
            if rxn_id in [r.id for r in model.reactions]:
                model.reactions.get_by_id(rxn_id).lower_bound = lb
        sol = model.optimize()
        print(f"  {condition:25s}: {sol.objective_value:.4f} h⁻¹")
```

## Gene and Reaction Knockout Simulations

```python
from cobra.flux_analysis import single_gene_deletion, single_reaction_deletion

# Single gene knockout
with model:
    # Knock out pfkA (phosphofructokinase A)
    model.genes.get_by_id('b3916').knock_out()  # pfkA
    sol_ko = model.optimize()
    print(f"pfkA knockout growth: {sol_ko.objective_value:.4f} h⁻¹")

# Systematic single gene deletion screen
print("\nComputing essential genes...")
deletion_results = single_gene_deletion(model)
essential_genes = deletion_results[deletion_results['growth'] < 1e-6]['ids']
print(f"Essential genes: {len(essential_genes)} / {len(model.genes)}")

# Single reaction deletion
rxn_deletion = single_reaction_deletion(model)
essential_rxns = rxn_deletion[rxn_deletion['growth'] < 1e-6]
print(f"Essential reactions: {len(essential_rxns)}")
```

## Medium Management

```python
from cobra.medium import minimal_medium

# Find the minimal medium for growth on glucose
min_medium = minimal_medium(model, 0.1)  # minimum growth rate 0.1 h⁻¹
print("Minimal medium components:")
for rxn_id, flux in min_medium.items():
    print(f"  {rxn_id}: {flux:.3f}")

# Set a custom medium
model.medium = {
    'EX_glc__D_e': 10,  # glucose
    'EX_nh4_e': 1000,   # ammonium
    'EX_pi_e': 1000,    # phosphate
    'EX_so4_e': 1000,   # sulfate
    'EX_o2_e': 20,      # oxygen
    # ... other required inorganics
}
```

## Visualization with Escher

```python
import escher

# Build and display metabolic flux map
builder = escher.Builder(
    map_name='e_coli_core.Core metabolism',
    reaction_data=dict(sol.fluxes),
    # Color reactions by flux direction and magnitude
    reaction_scale=[
        {'type': 'min', 'color': '#c8c8c8', 'size': 4},
        {'type': 'value', 'value': 0, 'color': '#c8c8c8', 'size': 4},
        {'type': 'max', 'color': '#54a0e3', 'size': 12}
    ]
)
builder.display_in_notebook()
```

## Saving and Sharing Models

```python
import cobra

# Save in multiple formats
cobra.io.write_sbml_model(model, 'modified_model.xml')  # SBML XML
cobra.io.save_json_model(model, 'modified_model.json')   # JSON (faster loading)
cobra.io.save_matlab_model(model, 'modified_model.mat')  # MATLAB

# Load model from URL (BioModels database)
# model = cobra.io.load_model('MODEL1507180060')  # via BiGG identifiers
```

## Why This Matters

COBRApy is the practical tool through which all constraint-based metabolic modeling is conducted in Python. Mastering its API — model loading, flux optimization, gene knockouts, medium management — is the gateway to applying FBA to any metabolic engineering or systems biology question. The library's consistent use of context managers and pandas DataFrames for results makes it interoperable with the rest of the scientific Python ecosystem (NumPy, SciPy, pandas, matplotlib, seaborn), enabling seamless integration with data analysis pipelines. For anyone working in metabolic engineering, genome-scale modeling, or microbiome research, COBRApy proficiency is an essential computational skill.

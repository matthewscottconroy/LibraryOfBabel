# Incorporating Regulatory Constraints into Metabolic Models

## The Limitation of Pure Stoichiometric Models

Consider the *lac* operon. When *E. coli* is growing on glucose, the genes for lactose transport and cleavage are essentially silent — catabolite repression keeps them off. If you add lactose to a glucose-grown culture, nothing happens metabolically until the glucose is gone. The cell does not simultaneously metabolize both sugars; it actively suppresses lactose catabolism in the presence of glucose. This behavior is invisible to standard FBA, which sees both glucose and lactose as available carbon sources and happily routes flux through lactose pathways regardless.

Standard FBA is purely stoichiometric — it predicts fluxes based on mass balance and capacity constraints, without accounting for gene regulation. In reality, cells use transcriptional regulation to adjust enzyme levels (and thus effective flux capacities) in response to environmental conditions. FBA cannot distinguish between glucose and fructose as carbon sources beyond their stoichiometric differences, yet cells regulate their metabolic responses quite differently to these sugars.

**Regulatory FBA (rFBA)** and its variants incorporate gene regulatory logic to constrain which reactions are active under specific conditions.

## rFBA: Boolean Regulatory Rules

In **rFBA** (Shlomi et al. 2007), Boolean gene regulatory rules determine enzyme availability, which constrains reaction flux bounds:

1. Define Boolean state variables for environmental signals (glucose present/absent, oxygen present/absent, nitrogen limiting, etc.)
2. Encode transcriptional regulatory logic as Boolean functions of these signals: `lacY is expressed IF lactose present AND NOT glucose present`
3. Evaluate Boolean rules given current environmental state → determine which genes are expressed
4. Map gene expression to reaction bounds via GPR associations: if gene is OFF, set reaction flux = 0
5. Solve FBA with modified bounds

```python
import cobra

model = cobra.io.read_sbml_model('iJO1366.xml')

def apply_regulatory_constraints(model, glucose_present, oxygen_present, 
                                   lactose_present):
    """Apply simplified regulatory constraints."""
    # Catabolite repression: lac operon suppressed by glucose
    lac_expressed = lactose_present and not glucose_present
    
    if not lac_expressed:
        # Suppress lac transport and metabolism
        for rxn_id in ['LACt2pp', 'LACZ']:
            if rxn_id in [r.id for r in model.reactions]:
                model.reactions.get_by_id(rxn_id).bounds = (0, 0)
    
    # Anaerobic regulation: suppress aerobic reactions
    if not oxygen_present:
        # Cytochrome bo oxidase: aerobic only
        for rxn_id in ['CYTBO3_4pp', 'CYTBD2pp']:
            if rxn_id in [r.id for r in model.reactions]:
                model.reactions.get_by_id(rxn_id).bounds = (0, 0)
    
    return model

# Simulate glucose + lactose (catabolite repression)
with model:
    apply_regulatory_constraints(model, glucose_present=True, 
                                  oxygen_present=True, lactose_present=True)
    model.reactions.get_by_id('EX_glc__D_e').lower_bound = -10
    sol_glc = model.optimize()
    print(f"Glucose+Lactose (catabolite repression): {sol_glc.objective_value:.4f} h⁻¹")

# Simulate lactose only (lac operon expressed)
with model:
    apply_regulatory_constraints(model, glucose_present=False,
                                  oxygen_present=True, lactose_present=True)
    model.reactions.get_by_id('EX_lcts_e').lower_bound = -10  # lactose
    sol_lac = model.optimize()
    print(f"Lactose only (lac expressed): {sol_lac.objective_value:.4f} h⁻¹")
```

## TRFBA: Transcriptomic Constraints

**Transcriptional Regulatory FBA (TRFBA)** uses continuous transcriptomic data (RNA-seq, microarray) to constrain metabolic fluxes without discrete Boolean approximations:

**E-flux**: maps normalized mRNA expression levels to maximum flux bounds:

$$v_j^\text{max} = v_j^\text{default} \times \frac{\text{expression}_j}{\text{expression}^\text{reference}_j}$$

where the reference expression is from a well-characterized condition.

**SPOT (Simplified Pearson cOrrelation with Transcriptomics)**: correlates reaction fluxes with expression data and uses the correlation to weight flux penalties.

These methods are approximate — mRNA levels do not always predict protein levels or enzyme activity — but they can improve FBA predictions under conditions where regulatory changes are large (stationary phase, nutrient shifts, stress responses).

## GECKO: Enzyme-Constrained Models

**GECKO** (Sánchez et al. 2017) represents the state of the art in integrating enzymatic constraints. Instead of treating reactions as having unlimited capacity, GECKO explicitly models each enzyme as a protein with a specific turnover number ($k_\text{cat}$):

For each reaction $j$ catalyzed by enzyme $e_j$:

$$v_j \leq k_{\text{cat},j} \cdot [E_j]$$

where $[E_j]$ is the enzyme concentration (in g/gDW). Adding the constraint:

$$\sum_j \frac{v_j}{k_{\text{cat},j} \cdot M_{\text{w},j}} \leq \rho_\text{max}$$

where $\rho_\text{max}$ is the total cellular protein mass budget (estimated from proteomics, typically ~0.5 g/gDW).

```python
# GECKO is implemented for yeast8 model via the GECKO toolbox
# Python interface through gemsembler package (newer approach)

# Conceptual GECKO-like constraint:
from cobra import Model, Reaction, Metabolite

def add_enzyme_constraint(model, kcat_dict, total_enzyme_budget=0.5):
    """Add simplified GECKO-like proteome allocation constraint."""
    # Create a "total enzyme" pseudo-metabolite
    enzyme_pool = Metabolite('prot_pool', name='Enzyme pool', 
                              compartment='c')
    model.add_metabolites([enzyme_pool])
    
    # Add enzyme consumption to each reaction
    for rxn_id, kcat in kcat_dict.items():
        rxn = model.reactions.get_by_id(rxn_id)
        # Enzyme cost: 1/(kcat * MW) per unit of flux
        # Simplified: assume MW = 40 kDa for all enzymes
        enzyme_cost = 1.0 / (kcat * 40000)  # g_enzyme/mmol_flux
        rxn.add_metabolites({enzyme_pool: -enzyme_cost})
    
    # Add exchange reaction for enzyme pool
    pool_rxn = Reaction('enzyme_pool_supply')
    pool_rxn.add_metabolites({enzyme_pool: 1.0})
    pool_rxn.bounds = (0, total_enzyme_budget)
    model.add_reactions([pool_rxn])
    
    return model
```

GECKO predictions:
- Growth rate as a function of dilution rate
- Proteome composition (which enzymes dominate at different growth rates)
- Effect of kcat mutations on growth capacity
- Optimal carbon source given proteome budget constraints

## Context-Specific Models from Transcriptomics

**FASTCORE**, **GIMME**, **iMAT**: algorithms that build condition-specific models by removing reactions that are highly unlikely given transcriptomic evidence. These reduce a generic model to a context-specific one:

- GIMME: minimizes flux through reactions with low expression, subject to achieving a metabolic objective
- iMAT: maximizes consistency between high/low expression categories and non-zero/zero fluxes
- FASTCORE: keeps a minimal set of reactions that satisfies expressed reactions; faster for large models

These context-specific models are used for predicting tissue-specific metabolism in human models (Recon3D applied to liver vs. kidney vs. brain tissue). The same metabolic genes expressed at different levels in different tissues produce remarkably different predicted flux distributions — this is what makes the liver a metabolic workhorse while neurons prioritize a narrow, energy-efficient flux palette.

## Why This Matters

Regulatory constraints transform GEMs from descriptions of metabolic potential into predictors of context-specific behavior. A cell does not simultaneously express all enzymes in its genome — it regulates which enzymes are synthesized based on environmental signals, and these regulatory decisions shape its metabolic phenotype. By incorporating transcriptional logic, expression data, or enzyme capacity constraints, constraint-based models can distinguish between conditions where stoichiometry alone cannot: the glucose-fed vs. acetate-fed cell, the aerobic vs. anaerobic state, the exponentially growing vs. stationary phase cell. This integration is essential for metabolic engineering (predict the effect of overexpressing a transcription factor) and for personalized medicine (predict metabolism in a patient with specific gene expression changes).

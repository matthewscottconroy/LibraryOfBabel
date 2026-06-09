# ME-Models: Metabolism and Expression

## Beyond Standard Metabolic Models

Here is something standard metabolic models cannot explain: why does *E. coli* growing on glucose in the presence of abundant oxygen still secrete acetate? From a purely thermodynamic perspective, it should complete oxidative phosphorylation and extract maximum ATP from every glucose molecule. But it doesn't. At fast growth rates, it wastes carbon through overflow metabolism — and this turns out to be a rational strategy once you account for a constraint that GEMs ignore entirely: making enzymes is expensive.

Standard genome-scale metabolic models (GEMs) predict optimal flux distributions under the implicit assumption that enzymes are present in unlimited quantity — only their biochemical reaction stoichiometry matters. In reality, producing enzymes is itself a metabolic cost: ribosomes must synthesize every enzyme, RNA polymerases must transcribe every enzyme gene, and the proteome has a finite capacity (cells have limited volume and ribosome capacity).

**ME-models (Metabolism and Expression models)** (O'Brien et al. 2013, *Science*) extend GEMs by explicitly including the gene expression machinery as part of the optimization problem. This creates a fundamentally richer model that can predict not just fluxes but also **proteome composition** — what fraction of total protein is allocated to each enzyme and to the expression machinery itself.

## The Core Innovation: Ribosomes as a Resource

In a standard GEM, the biomass production reaction includes a fixed set of protein requirements per unit biomass. In a ME-model, each protein must be synthesized explicitly by the ribosomes, and ribosomes have a finite total capacity:

$$\sum_j \frac{v_j^{\text{syn}}}{k_{\text{rib}}} \leq c_{\text{rib}}$$

where $v_j^{\text{syn}}$ is the synthesis rate of protein $j$ (mmol/gDW/h), $k_{\text{rib}}$ is the ribosome elongation rate (amino acids per second), and $c_{\text{rib}}$ is the total ribosome capacity (determined by ribosome copy number, itself a model variable).

Similarly, RNA polymerase (RNAP) has a finite capacity:

$$\sum_j \frac{v_j^{\text{trans}}}{k_{\text{RNAP}}} \leq c_{\text{RNAP}}$$

These constraints create a **growth rate-dependent allocation problem**: at faster growth rates, the cell needs more ribosomes (to synthesize proteins at higher rates), but making ribosomes consumes ribosome capacity. This creates a self-referential constraint that determines the maximum growth rate.

## The Ribosome Allocation Problem

The ME-model optimization can be understood through a simple ribosome allocation argument. Define:

- $\phi_R$: fraction of total protein that is ribosomal
- $\phi_E$: fraction of total protein that is metabolic enzymes
- $\phi_R + \phi_E = 1$ (simplified; ignores other protein categories)

The growth rate $\mu$ is:
$$\mu = k_R \phi_R$$

where $k_R$ is the ribosomal productivity constant (growth produced per unit ribosome). However, ribosomes themselves must be synthesized by ribosomes:
$$\phi_R = \mu / k_R \quad \Rightarrow \quad \phi_E = 1 - \mu/k_R$$

Metabolic flux is proportional to metabolic enzyme allocation: $\mu \propto \phi_E \cdot k_E$ (where $k_E$ is enzyme productivity). Combining:

$$\mu = k_E (1 - \mu/k_R)$$
$$\mu (1 + k_E/k_R) = k_E$$
$$\mu_{\max} = \frac{k_E k_R}{k_E + k_R}$$

This simple model predicts a maximum growth rate determined by ribosome productivity ($k_R$) and metabolic enzyme productivity ($k_E$). When $k_R \gg k_E$: $\mu_{\max} \approx k_E$ (metabolically limited). When $k_E \gg k_R$: $\mu_{\max} \approx k_R$ (ribosomally limited).

## The *E. coli* ME-model: iOL1650-ME

The first ME-model for *E. coli* (O'Brien et al. 2013) is called **iOL1650-ME** (later extended to **iJL1678-ME** and subsequent versions). It includes:

- **4,695 reactions** (compared to 2,583 in iJO1366 GEM): metabolic reactions + transcription, translation, complex assembly, protein folding, mRNA and protein degradation
- **1,678 genes** explicitly represented
- **~70,000 variables** in the optimization problem

The additional reactions represent:
- Transcription of every gene: RNA polymerase + NTPs → mRNA
- Translation of every protein: ribosome + tRNA + amino acids → protein
- Complex assembly: multiple polypeptides → functional complex
- Protein folding: chaperone-dependent folding reactions
- tRNA charging: amino-acyl tRNA synthetases

## Mathematical Formulation

The ME-model is a **linear programming problem** (like FBA) but substantially larger. The constraint matrix includes the metabolic stoichiometry plus expression machinery stoichiometry:

$$\begin{bmatrix} \mathbf{S}_{\text{met}} & \mathbf{S}_{\text{expr}} \\ \mathbf{0} & \mathbf{S}_{\text{coupling}} \end{bmatrix} \begin{bmatrix} \mathbf{v}_{\text{met}} \\ \mathbf{v}_{\text{expr}} \end{bmatrix} = \begin{bmatrix} \mathbf{0} \\ \mathbf{0} \end{bmatrix}$$

where $\mathbf{S}_{\text{coupling}}$ encodes the coupling between protein production rates and their required metabolic reactions:

$$v_{\text{reaction}} \leq k_{\text{cat}} \cdot v_{\text{protein,syn}} / \mu$$

This coupling constraint says: the flux through a reaction is bounded by the enzyme concentration, which equals the enzyme synthesis rate divided by the dilution rate (growth rate $\mu$ dilutes all cellular components).

```python
# Conceptual code for ME-model structure using COBRAme
import cobrame

# Load base GEM
me_model = cobrame.MEModel()
me_model.load_me_model('iJO1366_me.json')

# ME-model optimization (non-linear: mu appears as parameter)
# Solved iteratively using bisection on mu

def optimize_me_model(me_model, mu_guess=0.5, tol=1e-6):
    """
    Iterative optimization of ME-model.
    Find maximum mu such that model is feasible.
    """
    mu_lo, mu_hi = 0, 2.0  # bounds on growth rate (h^-1)
    
    while mu_hi - mu_lo > tol:
        mu = (mu_lo + mu_hi) / 2
        me_model.reactions.get_by_id('biomass_dilution').upper_bound = mu
        
        # Update coupling constraints with current mu
        for rxn in me_model.reactions:
            if hasattr(rxn, 'kcat') and rxn.kcat > 0:
                # v_rxn <= kcat * [enzyme] = kcat * v_syn / mu
                rxn.upper_bound = rxn.kcat / mu  # simplified
        
        solution = me_model.optimize()
        if solution.status == 'optimal':
            mu_lo = mu
        else:
            mu_hi = mu
    
    return mu_lo
```

## Key Predictions of ME-Models

**Growth rate-dependent proteome composition**: ME-models predict that the ribosomal fraction of total protein increases linearly with growth rate — a relationship known as the "Schaechter-Maaløe-Kjeldgaard (SMK) law" observed experimentally in *E. coli*. The GEM alone cannot predict this because it treats the ribosome as a fixed cost.

**Overflow metabolism**: at high growth rates, ME-models predict that *E. coli* secretes acetate even under aerobic conditions ("acetate overflow"). This occurs because the metabolic bottleneck shifts from enzyme kinetics to ribosome capacity — producing ATP via the less efficient fermentative pathway is advantageous because it requires less enzyme and thus less ribosome allocation.

**Antibiotic sensitivity**: ME-models can predict which growth conditions make bacteria most sensitive to antibiotics targeting ribosomes or metabolic enzymes — because the ribosome allocation fraction determines intrinsic sensitivity.

**iOL1650-ME predictions vs. experiment**: the ME-model correctly predicted the proteome fraction allocated to ribosomes (from mass spectrometry) for *E. coli* grown at 12 different conditions, with R² > 0.9 — substantially better than GEM predictions.

## COBRAme: Software for ME-models

**COBRAme** is the Python package for constructing and solving ME-models, built on top of COBRApy:

```python
import cobrame

# Build ME-model from scratch
me = cobrame.MEModel('my_me_model')
me.add_metabolites(metabolites_list)
me.add_reactions(metabolic_reactions)
me.add_expression_machinery(genome_sequence, protein_sequences, 
                              trna_synthetases, ribosome_composition)
me.solve()
print(f"Predicted growth rate: {me.solution.objective_value:.3f} h⁻¹")
print(f"Ribosomal fraction: {me.solution['ribosome_fraction']:.2f}")
```

## Why This Matters

ME-models represent the natural extension of GEMs toward a more complete cellular model — one that couples metabolism with the gene expression machinery that produces the enzymes that enable metabolism. This coupling predicts a large class of metabolic phenotypes (proteome composition, overflow metabolism, antibiotic sensitivity, growth rate-dependent phenotypes) that GEMs fundamentally cannot access. As single-cell proteomics becomes routine, ME-model predictions can be tested against quantitative proteome measurements across growth conditions, metabolic perturbations, and genetic knockouts — making ME-models an increasingly testable and predictively powerful component of the systems biology toolkit.

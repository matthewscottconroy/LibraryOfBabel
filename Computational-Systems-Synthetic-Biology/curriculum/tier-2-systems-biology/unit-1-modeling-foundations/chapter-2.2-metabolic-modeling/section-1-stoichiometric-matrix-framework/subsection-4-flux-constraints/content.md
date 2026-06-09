# Flux Constraints: Bounding the Feasible Space

## From Null Space to Feasible Polytope

The null space alone is a vast, undisciplined space — it includes flux distributions where enzymes run at infinite speed, irreversible reactions run backwards, and a cell absorbs unlimited glucose. Biology, of course, does none of these things. The gap between the mathematical null space and real cellular behavior is bridged by **flux constraints**: inequalities that translate biological limits into the language of linear programming.

The null space $\mathcal{N}(\mathbf{S})$ defines which flux distributions are stoichiometrically feasible. However, the null space alone is unbounded — it includes infinitely large and infinitely small fluxes. Biological reality imposes additional constraints on what fluxes are achievable:

- Reactions cannot proceed at infinite speed (enzyme capacity limits)
- Irreversible reactions cannot run backwards (thermodynamics)
- Nutrient uptake is limited by transporter capacity and environmental availability
- Some fluxes may be directly measured and therefore fixed

These constraints define the **feasible flux polytope** (also called the flux cone or constraint-based feasible region):

$$\mathcal{F} = \{\mathbf{v} : \mathbf{S} \cdot \mathbf{v} = \mathbf{0},\; \mathbf{v}_\text{min} \leq \mathbf{v} \leq \mathbf{v}_\text{max}\}$$

This is a **convex polytope** — a bounded convex set defined by linear inequalities. Every feasible metabolic state is a point in $\mathcal{F}$.

## Types of Constraints

### Irreversibility Constraints

Thermodynamic constraints require that irreversible reactions carry non-negative flux:

$$v_j \geq 0 \quad \text{for all irreversible reactions } j$$

Reversible reactions allow negative flux (reaction running in reverse):

$$-v_j^\text{max} \leq v_j \leq v_j^\text{max}$$

In practice, large values ($\pm 1000$ mmol/gDW/h) are used as "infinity" for unconstrained reversible reactions.

Thermodynamic directionality can be determined from the sign of the standard Gibbs free energy $\Delta_r G^{\circ\prime}$ (Section 2.2.5.1). Reactions with $\Delta_r G^{\circ\prime} \ll 0$ are strongly irreversible; those with $\Delta_r G^{\circ\prime} \approx 0$ are reversible under physiological conditions.

### Nutrient Uptake Constraints

Exchange fluxes at the system boundary represent the flow of nutrients from the medium into the cell (negative sign = uptake) and waste products from cell to medium (positive sign = secretion):

$$v_{\text{EX\_glc}} \geq -10 \; \text{mmol/gDW/h} \quad \text{(glucose uptake limited to 10)}$$

$$v_{\text{EX\_o2}} \geq -20 \; \text{mmol/gDW/h} \quad \text{(oxygen uptake limited)}$$

Constraints on exchange fluxes define the **growth medium** computationally: a minimal medium restricts all carbon sources to a single defined substrate; a rich medium allows uptake of many amino acids and cofactors.

**Anaerobic growth** is modeled by setting $v_{\text{EX\_o2}} = 0$ (no oxygen allowed). The model will then redistribute fluxes to fermentation pathways (ethanol production in yeast, mixed acid fermentation in bacteria). With a single constraint change, you transform the simulated environment from aerobic to anaerobic — this is the computational analog of moving a culture from a shaker flask to a sealed anaerobic chamber.

### Enzyme Capacity Constraints (GECKO)

Standard FBA assumes unlimited enzyme capacity — any flux is achievable given the stoichiometric constraints. The **GECKO framework** (GEM with Enzymatic Constraints using Kinetic and Omics data) adds a proteome allocation constraint:

$$\sum_j \frac{v_j}{k_\text{cat,j} / M_\text{w,j}} \leq \rho_\text{max}$$

where $k_\text{cat,j}$ is the catalytic rate constant of the enzyme for reaction $j$, $M_\text{w,j}$ is its molecular weight, and $\rho_\text{max}$ is the total enzyme mass budget (estimated from proteomics). This constraint captures the trade-off between flux and enzyme investment.

### Measured Flux Constraints

When specific fluxes are measured experimentally (by $^{13}$C MFA, isotope tracing, or gas exchange), they can be fixed:

$$v_{\text{biomass}} = \mu_\text{measured}$$

$$v_{\text{EX\_CO2}} = v_\text{measured} \pm \sigma$$

Fixing biomass flux to the measured growth rate and then using FBA to predict all other fluxes is a common use case.

```python
import cobra
from cobra.io import read_sbml_model

model = read_sbml_model('iJO1366.xml')

# Aerobic growth on glucose
model.reactions.get_by_id('EX_glc__D_e').lower_bound = -10   # mmol/gDW/h
model.reactions.get_by_id('EX_o2_e').lower_bound = -20

# Block all other carbon sources (minimal medium)
# (In iJO1366, all other EX_... exchanges are already 0 by default for minimal medium)

# Simulate
sol = model.optimize()
print(f"Growth rate (aerobic glucose): {sol.objective_value:.4f} h⁻¹")

# Anaerobic
with model:
    model.reactions.get_by_id('EX_o2_e').lower_bound = 0
    sol_anaerobic = model.optimize()
    print(f"Growth rate (anaerobic glucose): {sol_anaerobic.objective_value:.4f} h⁻¹")
    eth_flux = sol_anaerobic.fluxes.get('EX_etoh_e', 0)
    print(f"Ethanol secretion: {eth_flux:.4f} mmol/gDW/h")
```

## The Geometry of the Feasible Polytope

The feasible polytope $\mathcal{F}$ is a multi-dimensional polyhedron. Its vertices (extreme points) correspond to unique solutions where the maximum number of constraints are active (at their bounds). In metabolic terms, vertices represent metabolic states where the maximum number of reactions are either fully on (at their upper bound) or fully off (zero flux).

FBA identifies a vertex (or face) of this polytope that maximizes the chosen objective. The polytope can be explored more completely by:
- **Flux variability analysis** (Section 2.2.2.4): find the range of each reaction across the optimal face
- **Sampling** (optGpSampler, ACHRsampler): randomly sample interior points of the polytope to characterize the full distribution of feasible metabolic states
- **Elementary flux modes**: enumerate the extreme rays of the cone

## Why This Matters

Flux constraints transform an abstract mathematical null space into a biologically interpretable feasible region. The constraints encode our knowledge of the growth environment (which nutrients are available), thermodynamic limits (which reactions are irreversible), and enzyme capacities (how fast each reaction can proceed). Adding or modifying constraints is how we simulate different experimental conditions in silico: changing the carbon source, adding a gene knockout, simulating a drug treatment. Mastering constraint specification is therefore mastering the art of computational metabolic experiment design.

# Multi-Organism Community Metabolic Modeling

## Why Model Microbial Communities?

Your gut contains roughly 38 trillion microbial cells — comparable to the number of human cells in your body. These microbes are not passive passengers; they produce short-chain fatty acids that fuel your colon epithelium, synthesize vitamins, metabolize drugs, and educate your immune system. They do all of this through an intricate web of metabolic cross-feeding: one species's waste product is another's nutrient, and the collective metabolic output of the community bears little resemblance to what any single species would produce in isolation. To understand the gut microbiome — or any other microbial community — you cannot analyze organisms one at a time.

Microorganisms rarely exist in isolation. The gut microbiome contains 100–1000 species living in a complex metabolic ecosystem. Soil communities drive the global carbon and nitrogen cycles. Industrial bioprocesses use co-cultures for division of metabolic labor. Understanding these communities requires modeling the exchange of metabolites between organisms.

Multi-organism community metabolic modeling extends the single-organism FBA framework to networks of interacting GEMs sharing a common extracellular environment.

## The Community FBA Framework

The simplest approach **stacks** multiple GEMs into a single stoichiometric matrix, connected through a shared extracellular compartment:

$$\mathbf{S}_\text{community} = \begin{pmatrix} \mathbf{S}_1 & & \\ & \mathbf{S}_2 & \\ & & \mathbf{S}_3 \\ \hline -\mathbf{E}_1 & -\mathbf{E}_2 & -\mathbf{E}_3 \end{pmatrix}$$

where $\mathbf{S}_i$ is the stoichiometric matrix of organism $i$ and $\mathbf{E}_i$ connects organism $i$'s internal exchange reactions to the shared extracellular compartment.

At steady state, every metabolite in the shared compartment must be balanced:

$$\sum_i v_{ij}^\text{exchange} \cdot X_i = 0 \quad \forall j \in \text{shared metabolites}$$

(for chemostat-like conditions where the environment is at steady state).

## SteadyCom: Equal Growth Rate Community Modeling

**SteadyCom** (Chan et al. 2017) implements a key constraint for a stable microbial community at steady state: all organisms must grow at the same rate $\mu$ (otherwise faster-growing organisms would displace slower ones, violating the steady-state assumption):

$$\mu_1 = \mu_2 = \ldots = \mu_N = \mu$$

Combined with the steady-state mass balance on extracellular metabolites, this determines both the species abundances and their metabolic fluxes.

```python
# SteadyCom is available in COBRApy
from cobra.community import SteadyCom
import cobra

# Load two organisms
ecoli = cobra.io.load_model('iJO1366')
klebsiella = cobra.io.read_sbml_model('iKP1289_Kpneumoniae.xml')

# Define community
community = cobra.Model()
# Add organisms with species prefixes
# ... (SteadyCom handles this internally)

# Run SteadyCom
result = SteadyCom.SteadyCom(
    [ecoli, klebsiella],
    medium={'EX_glc__D_e': 10, 'EX_nh4_e': 100, 'EX_pi_e': 100}
)
print(f"Community growth rate: {result.objective_value:.4f} h⁻¹")
print(f"E. coli abundance: {result.abundance['iJO1366']:.3f}")
print(f"K. pneumoniae abundance: {result.abundance['iKP1289']:.3f}")
```

## MICOM: Multi-Organism Community Modeling

**MICOM** (Diener et al. 2020) is the standard tool for gut microbiome community modeling. It uses AGORA2 models (7,302 gut microbiome species) and allows:

- **Cooperative tradeoff**: organisms are not forced to maximize growth rate; instead, they cooperate to maximize community growth while each individual uses a fraction of its maximum growth capacity (controlled by a tradeoff parameter)
- **Diet simulation**: defined nutrient compositions from human dietary databases (USDA food database)
- **Flux sampling**: probabilistic predictions of metabolite exchanges between organisms and host

```python
import micom

# Build community from taxonomy data
taxonomy = {
    'id': ['Bacteroides_thetaiotaomicron', 'Faecalibacterium_prausnitzii'],
    'abundance': [0.3, 0.2],
    'genus': ['Bacteroides', 'Faecalibacterium']
}

# Set Western diet
diet = micom.db.western_diet()

# Create community
com = micom.Community(taxonomy, model_db='agora2')

# Grow community and get fluxes
result = com.cooperative_tradeoff(fraction=0.5)
production = result.members[result.members.reaction == 'EX_but_e']
print("Butyrate producers:")
print(production[['organism', 'flux']])
```

## Cross-Feeding and Metabolic Syntrophy

Community modeling reveals **cross-feeding** relationships that are not apparent from single-organism analysis:

**Syntrophic communities**: organism A produces compound X (which is thermodynamically unfavorable at high X concentration) only because organism B consumes X (keeping its concentration low). Without B, A cannot grow.

**Example — hydrogen syntrophy**: many anaerobic bacteria produce H₂ during fermentation, but H₂ production becomes thermodynamically unfavorable above ~1 Pa H₂. Methanogenic archaea consume H₂ (producing CH₄), keeping H₂ partial pressure below threshold and enabling fermentative H₂ production.

**Metabolic complementation**: two auxotrophic strains can grow in co-culture by cross-feeding the missing metabolite. This is both observed in natural communities and engineered in synthetic consortia for metabolic division of labor.

Community FBA can predict which cross-feeding patterns are stoichiometrically feasible and which organisms in a community must interact for all to survive.

## Applications

**Gut microbiome metabolism**:
- Predict short-chain fatty acid (SCFA) production (butyrate, propionate, acetate) from dietary fiber
- Identify species responsible for drug biotransformation (activation/inactivation)
- Model the effect of antibiotics on community metabolism

**Synthetic consortia design**:
- Design two-strain communities for production of compounds requiring long biosynthetic pathways
- Strain A: upstream pathway → intermediate
- Strain B: consumes intermediate → final product
- Balance growth rates for stable co-culture

**Ecological predictions**:
- Competitive exclusion: which species cannot coexist due to identical metabolic niches
- Cooperative growth: which species combinations enable growth on minimal medium
- Mutualism: which pairs of species each increase the other's growth rate

## Why This Matters

Microbial communities drive planetary-scale biogeochemical cycles, determine human health outcomes through the gut microbiome, and are central to next-generation biotechnology (co-cultures, microbiome engineering). Single-organism GEMs cannot address these questions. Community metabolic models provide the computational framework for understanding how metabolic complementarity, cross-feeding, and competition shape the composition and function of microbial ecosystems. The MICOM/AGORA2 platform in particular represents a transformative capability: predicting how diet and microbiome composition jointly determine metabolite production affecting host health, from SCFA production supporting intestinal epithelium to secondary bile acid metabolism influencing immune function.

# Knockout of Competing Pathways

The metabolic network is not a linear pipeline leading from glucose to your product. It is an interconnected web, and the very same intermediates your new pathway needs are simultaneously being consumed by pathways that evolution installed long before your engineering goals existed. Pyruvate gets siphoned to lactate. FPP gets stolen by the sterol pathway. 2-ketoisovalerate gets diverted to valine. Every precursor your synthetic pathway requires is a contested resource in an ongoing competition — and in wild-type cells, your pathway loses. After introducing a heterologous biosynthetic pathway, the next phase of strain optimization is eliminating competing reactions that divert precursors, consume cofactors, or degrade the product. Strategic knockouts redirect metabolic flux toward the target compound, increasing yield without requiring additional enzyme expression.

## Identifying Targets for Knockout

The rational approach to identifying knockout candidates uses **flux balance analysis (FBA)** combined with genome-scale metabolic models:

### OptKnock

OptKnock (Burgard et al. 2003) formulates knockout selection as a bilevel optimization problem:

**Inner problem**: maximize biomass production (cell's objective) subject to stoichiometric constraints and the knockouts specified by the outer problem.

**Outer problem**: find the set of knockouts that maximizes product yield at the growth optimum.

The key insight: by knocking out competing reactions, growth and production can be **coupled** — the cell can only grow well by also producing the target compound. This genetic coupling means evolutionary pressure favors the producing strain.

```python
from cobra.flux_analysis import OptKnock

model = load_model("iJO1366")  # E. coli genome-scale model
results = OptKnock(model).run(
    target="EX_isobutanol_e",
    biomass="BIOMASS_Ec_iJO1366_WT_53p95M",
    max_knockouts=4
)
# Returns knockout combinations that couple growth to isobutanol production
```

### Manual Analysis by Flux Topology

For simpler cases, inspect the metabolic network around the precursor:
1. Draw all reactions consuming the key precursor metabolite
2. Identify which consumption pathways are competing with your target
3. Evaluate whether eliminating each competitor is viable (i.e., the knockout doesn't kill the cell)

## Common Knockout Targets and Rationale

### Eliminating Competing Fermentation Products

In anaerobic or mixed-acid fermentation, *E. coli* produces a spectrum of byproducts from pyruvate:
- Lactate (D-lactic acid): ldhA encodes D-lactate dehydrogenase
- Acetate: pta-ackA (phosphotransacetylase + acetate kinase) or poxB (pyruvate oxidase)
- Formate: pflB (pyruvate formate lyase) under anaerobic conditions
- Succinate: competing TCA cycle usage

**For ethanol production**: Δpta ΔackA reduces acetate overflow; Δldh eliminates lactate competing for pyruvate; Δpflb (anaerobic) increases pyruvate availability.

**For isobutanol**: isobutanol is derived from 2-ketoisovalerate (2-KIV), a valine biosynthesis intermediate. Competing pathways consuming 2-KIV:
- IlvE (branched-chain aminotransferase): 2-KIV → valine. Knockout reduces valine production, increasing 2-KIV for isobutanol.
- But: ΔilvE may cause valine auxotrophy. Solution: add valine to medium during first stages; adaptive evolution to recover growth.

### Eliminating Precursor Consumption

For terpenoid production (lycopene, artemisinin), the precursor FPP (farnesyl pyrophosphate) is consumed by:
- Squalene synthase (erg9 in yeast): FPP → squalene → sterols. This is essential in yeast. Solution: downregulate erg9 using methionine-repressible promoter (CYC1-CYC3 hybrid, repressed by methionine) — reduces sterol synthesis during production phase without killing the cell.
- Farnesylated proteins: small amount; cannot eliminate
- Heme synthesis: small amount; cannot eliminate

### Product-Degrading Pathway Knockouts

If the target product is a natural substrate for cellular enzymes, these must be eliminated:
- Mucoic acid degradation: catA (catechol 1,2-dioxygenase) can degrade muconic acid. Δcat prevents product degradation.
- Lactate oxidation: knockout aldehyde dehydrogenases that might oxidize alcohol products
- Glycerol production in yeast: gpd1/gpd2 (glycerol-3-phosphate dehydrogenase) knockout reduces glycerol byproduct formation, increasing carbon flux to ethanol or other products

## Essential vs. Non-Essential Knockouts

Not all knockouts are viable. Before attempting a knockout:

1. Check Keio collection (comprehensive *E. coli* single-gene knockout library): determine if the gene is essential for aerobic growth in LB
2. Check condition-specific essentiality: a gene non-essential in rich medium may be essential in minimal medium (e.g., amino acid biosynthesis genes)
3. Use sgRNA-based CRISPRi for partial knockdown of essential genes where partial reduction is sufficient

**Example**: overexpressing HMG-CoA reductase for terpenoid production increases mevalonate pathway flux, but excess HMG-CoA can be toxic. Full ERG13 knockout (HMG-CoA synthase) is lethal, but ERG10 (acetyl-CoA acetyltransferase) can be partially downregulated to tune mevalonate pathway input.

## Combinatorial Knockout Strategies

A single knockout rarely dramatically improves titer; combinations are needed. The number of possible knockout combinations grows exponentially:
- 1 knockout: $n$ combinations
- 2 knockouts: $\binom{n}{2}$ combinations
- 3 knockouts: $\binom{n}{3}$ combinations

For *E. coli* with ~4,300 non-essential genes, testing all triple knockouts is computationally and experimentally intractable (>10^10 combinations). FBA-guided selection reduces the candidate list to a tractable set by predicting which combinations improve production without eliminating growth.

**Phenotype microarray screens**: Biolog PM plates can profile growth of knockout strains across 2,000 carbon and nitrogen sources simultaneously, identifying unexpected effects of knockouts on metabolic flexibility.

## Worked Example: Succinate Production Knockout Strategy

Target: high-yield succinate production from glucose in *E. coli* under anaerobic conditions

Competing pathways consuming oxaloacetate or succinyl-CoA (succinate precursors):
- Phosphoenolpyruvate carboxylase (ppc): key for oxaloacetate synthesis; cannot knock out
- Succinyl-CoA synthetase (sucCD): consumes succinate → succinyl-CoA (reverse of what we want)
- Lactate dehydrogenase (ldhA): diverts pyruvate to lactate
- Pyruvate formate lyase (pflB): diverts pyruvate to acetate + formate (anaerobic)
- Acetate kinase (ackA): diverts acetyl-CoA to acetate

**Optimized knockout set for succinate**: Δ*ldhA* Δ*pflB* Δ*ptsG* (replace glucose PTS with GalP+Glk to increase PEP availability) + Δ*pck* (phosphoenolpyruvate carboxykinase, reverse reaction competitor).

This combination, validated in Lee et al. 2005, produced 14.2 g/L succinate with 1.04 mol/mol yield from glucose — near the theoretical maximum of 1.12 mol/mol.

## Why This Matters

Competing pathway knockouts are among the highest-leverage interventions in metabolic engineering because they simultaneously increase precursor supply and reduce byproduct formation — often improving titer, rate, and yield together. The FBA-guided approach to knockout identification transforms what would otherwise be a trial-and-error process into a rational design exercise: the engineer can predict which knockouts improve production before investing experimental time in their construction. In the genomic editing era, constructing a strain with 5–10 chromosomal knockouts takes a few weeks rather than months, making iterative knockout optimization a practical strategy for routine metabolic engineering projects.

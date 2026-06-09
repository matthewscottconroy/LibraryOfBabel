# Cathedral I: A Predictive Metabolic Engineering Campaign

---

## The Question

Can a genome-scale metabolic model guide the rational design of a microbial strain to produce a target compound at commercially relevant levels?

---

## What You're Building

A complete metabolic engineering campaign that starts with a computational prediction and ends with an experimentally validated strain — or a rigorous in silico campaign benchmarked against published experimental data.

---

## Prerequisites

Before attempting this cathedral, you need:
- [Tier 0.2](../curriculum/tier-0-bedrock/0.2-chemistry.md): Biochemistry (enzyme kinetics, cofactor chemistry, thermodynamics)
- [Tier 0.3](../curriculum/tier-0-bedrock/0.3-biology.md): Microbiology (growth kinetics, central metabolism)
- [Tier 1.2](../curriculum/tier-1-bioinformatics/1.2-genomics.md): Genome annotation
- [Tier 2.2](../curriculum/tier-2-systems-biology/2.2-metabolic-modeling.md): FBA, GEMs, COBRApy (required)
- [Tier 3.4](../curriculum/tier-3-synthetic-biology/3.4-metabolic-engineering.md): Metabolic engineering principles

---

## The Project

### Phase 1: Pathway and Host Selection

1. Choose a target molecule with industrial or medical relevance
   - Starter options: muconic acid, lycopene, farnesene, 3-hydroxypropionic acid, mevalonate
   - Harder: a terpenoid natural product with a known biosynthetic pathway

2. Choose a chassis organism
   - *E. coli*: best characterized GEM (iJO1366 or iML1515); easiest genetic tools
   - *S. cerevisiae*: better for P450-requiring pathways; yeast8 GEM
   - Document: why this organism for this molecule

3. Map the biosynthetic pathway from central metabolism to target
   - Use: KEGG, MetaCyc, RetroPath
   - Identify: which reactions are native to the chassis, which are heterologous
   - Thermodynamic check: use eQuilibrator to verify all reactions are feasible

### Phase 2: Computational Strain Design

4. Load the chassis GEM in COBRApy

5. Add heterologous pathway reactions to the model
   - Specify stoichiometry, reversibility, cofactor requirements
   - Add exchange reaction for target product

6. Run baseline FBA
   - Maximize biomass: what is the predicted growth rate?
   - Maximize product: what is the maximum theoretical yield?
   - Compare: growth-coupled production? Decoupled?

7. OptKnock analysis
   - Identify gene knockout combinations that couple growth to product formation
   - Test knockouts: does the model predict viable knockout strain?

8. Flux Variability Analysis
   - For each competing pathway: flux range under optimal production
   - Identify which reactions must be upregulated

9. Thermodynamic analysis
   - MDF: what is the maximum minimum driving force for your pathway?
   - Identify thermodynamic bottleneck reaction
   - Does ΔG analysis suggest any reactions need coupling to ATP hydrolysis?

10. GECKO analysis (optional but valuable)
    - Enzyme-constrained model
    - Identify which enzyme has the highest cost per unit flux

### Phase 3: Strain Design Synthesis

11. Write a detailed engineering plan:
    - List of gene knockouts with justification
    - List of gene overexpressions with promoter recommendations
    - Codon-optimize heterologous genes for expression in chassis
    - RBS design for each heterologous gene

12. Predict expected performance:
    - Titer (g/L) — requires assuming production rate and culture time
    - Yield (g product / g glucose) as percentage of maximum theoretical
    - Rate (g/L/h) under batch conditions

### Phase 4: Comparison and Validation (Two Options)

**Option A (Experimental)**: Build the strain, measure performance, compare to predictions
- This is the full project; requires wet lab access and ~3-6 months

**Option B (In Silico Validation)**: Find published experimental data for this or a similar strain
- Compare model predictions to published titers and knockouts
- Explain discrepancies: what did the model get right? What did it miss?
- This is a publishable analysis — identifying why models fail is scientifically valuable

---

## Expected Output

A manuscript-quality analysis containing:
- Complete genome-scale model analysis (code in GitHub repository)
- Comparative analysis of computational predictions vs. experimental data
- Clear mechanistic explanation of why model predictions agree or disagree with experiment
- Proposed model refinements that would improve predictive accuracy
- A design recommendation for an improved strain

---

## Stretch Goals

- Dynamic FBA: model batch fermentation time course; predict growth and production kinetics
- Multi-enzyme kinetic model: replace FBA with kinetic model for the bottleneck pathway segment
- Robustness analysis: how sensitive is production to parameter uncertainty?
- Comparison of multiple chassis organisms for the same target

---

## Key Tools

- COBRApy: FBA, FVA, OptKnock, flux sampling
- eQuilibrator-api: thermodynamic analysis
- GECKO: enzyme-constrained modeling
- Salis RBS Calculator: translation rate prediction
- Pathway visualization: Escher (flux maps), MetDraw

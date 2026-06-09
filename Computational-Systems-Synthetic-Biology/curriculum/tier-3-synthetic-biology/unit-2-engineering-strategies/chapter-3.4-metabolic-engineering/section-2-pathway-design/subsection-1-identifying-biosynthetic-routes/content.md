# Identifying Biosynthetic Routes

Imagine you are handed a molecule — muconic acid, say, or lycopene — and asked to produce it in *E. coli*. The cell has never made this compound. Where do you even begin? The answer is to work backward: start from your target and ask what enzymatic reactions could have produced it, then what reactions could produce those precursors, and so on, until you arrive at metabolites the cell already makes. This process, called **retrobiosynthesis**, is the first and often most consequential design decision in all of metabolic engineering. A route requiring twelve heterologous enzymes and a toxic intermediate will fail; a route requiring three native-adjacent enzymes and favorable thermodynamics may succeed on the first attempt. Pathway design begins with the fundamental question: what sequence of enzymatic reactions can convert an available cellular metabolite into the desired product?

## The Retrobiosynthesis Approach

Retrobiosynthesis reverses the direction of synthesis: starting from the target compound, one identifies what known enzymatic reactions could produce it, what precursors those reactions require, and whether those precursors are available in the chosen chassis organism.

**Algorithm (conceptual)**:
```
1. Start: target molecule T
2. Query reaction databases for all reactions that produce T
3. For each precursor P of T:
   a. If P is a native metabolite in chassis → feasible path found
   b. If P is not native → repeat recursively for P
4. Collect all feasible multi-step routes
5. Rank routes by: number of heterologous steps, thermodynamic feasibility, 
   enzyme availability, and cofactor requirements
```

The challenge is the size of the reaction space: KEGG contains >10,000 reactions; MetaCyc contains >50,000. Manual navigation is impractical for more than 2–3 steps. Computational tools automate the search.

## Key Metabolic Databases

**KEGG (Kyoto Encyclopedia of Genes and Genomes)**:
- Contains metabolic pathways, reactions, compounds, and genes across >5,000 organisms
- Reaction database: ~11,000 reactions with EC number classification
- KEGG PATHWAY: visual pathway maps showing reactions in metabolic context
- KEGG REACTION: chemical equation, substrates, products, enzyme classes
- Use for: finding known pathways, identifying enzymes from specific organisms

**MetaCyc**:
- Experimentally validated pathways from >3,000 organisms
- Curated from primary literature: each pathway has associated publications
- ~3,000 metabolic pathways; emphasis on natural product biosynthesis
- Better than KEGG for secondary metabolite pathways (polyketides, alkaloids, terpenoids)

**BRENDA (Braunschweig Enzyme Database)**:
- Enzyme kinetic data: Km, Vmax, kcat, inhibitors, activators
- Temperature, pH optima
- Organism-specific parameters
- Essential for selecting the best enzyme for each step (section 3.4.2.2)

**BiGG Models**:
- Standardized metabolite and reaction IDs for use with genome-scale models
- Connects genome annotation to metabolic reactions
- Used for FBA modeling

## Automated Retrobiosynthesis Tools

### RetroPath2.0

RetroPath2.0 (Delepine et al. 2018) is a KNIME-based workflow that explores chemical space around a target molecule using biochemical transformation rules extracted from reaction databases.

**Concept**: biochemical reactions can be abstracted as **reaction rules** (SMARTS strings) that describe the chemical transformation without reference to a specific substrate. For example, the rule for a decarboxylase removes a carboxyl group from any compound bearing it. Applying all decarboxylase rules to all substrates generates all possible decarboxylation products.

**Algorithm**:
1. Start from the target molecule (represented as a SMARTS/SMILES string)
2. Apply all possible rules from the reaction rule library (extracted from KEGG or MetaCyc) in reverse (synthetic direction = retro-application)
3. Each application generates a precursor and a set of cofactors
4. Repeat for each new precursor until native chassis metabolites are reached
5. Output: network of possible routes from chassis metabolites to target

**Scope**: RetroPath2.0 can explore hundreds of steps and thousands of intermediate compounds in one run, generating a comprehensive map of possible routes.

**Example**: Designing a route to **muconic acid** (adipic acid precursor for nylon biosynthesis):
- Target: cis,cis-muconic acid
- RetroPath2.0 identifies routes from chorismate (shikimate pathway intermediate) via catechol or protocatechuic acid
- Best route: glucose → PEP/E4P → shikimate → chorismate → catechol → muconic acid (3 heterologous steps)
- Enzymes needed: AroY (catechol 1,2-dioxygenase from Klebsiella), CatA (catechol 1,2-dioxygenase from Pseudomonas)

### BNICE.ch (Biochemical Network Integrated Computational Explorer)

BNICE predicts **novel** enzymatic transformations by applying enzymatic reaction rules to substrate molecules, identifying reactions not yet observed in nature that are chemically plausible. It expands the design space beyond what is in any database.

### novoStoic

novoStoic (Kumar et al. 2018) uses mixed-integer linear programming to design novel pathways with optimal stoichiometry. Unlike retrobiosynthesis tools that enumerate routes one step at a time, novoStoic directly solves for pathways that are thermodynamically and stoichiometrically balanced, including cofactor regeneration.

## Evaluation Criteria for Route Selection

Not all computationally identified routes are equally viable. Evaluation criteria:

**Number of heterologous steps**: each heterologous enzyme must be expressed, may have toxicity, and adds metabolic burden. Prefer routes requiring the fewest non-native enzymes.

**Thermodynamic feasibility**: all steps must have negative $\Delta_r G'$ at relevant concentrations, or unfavorable steps must be coupled to favorable ones. The eQuilibrator tool calculates $\Delta_r G'$ values for any reaction.

$$\Delta_r G' = \Delta_r G'^\circ + RT \ln Q$$

Where $Q$ is the reaction quotient from intracellular metabolite concentrations. A step with $\Delta_r G'^\circ = +10 \text{ kJ/mol}$ may be feasible if products are maintained at low concentrations.

**Enzyme availability**: does a characterized enzyme exist for each step? BRENDA and UniProt document characterized enzymes. Preference for enzymes with published Km and kcat values in the relevant substrate range.

**Cofactor balance**: does the complete route consume and regenerate cofactors in a balanced manner? A route requiring 3 NADPH and regenerating 1 NADPH is net NADPH-negative — it will deplete the cellular NADPH pool and reduce flux unless NADPH regeneration is engineered.

**Intermediate toxicity**: some pathway intermediates are toxic (e.g., malonyl-ACP, cinnamate, some aldehydes). Routes that minimize the accumulation of toxic intermediates are preferred.

## Worked Example: Route Selection for Lycopene in *E. coli*

Target: lycopene (red carotenoid; antioxidant; food colorant)

Precursor in *E. coli*: IPP (isopentenyl pyrophosphate) — available from both MEP pathway (native in E. coli) and mevalonate pathway (from S. cerevisiae, 7 genes)

Native route (MEP pathway → carotenoids):
- IPP + DMAPP → GPP (C10) → FPP (C15) → GGPP (C20) → phytoene → lycopene
- Steps 4–6 require heterologous carotenoid biosynthesis genes (crtE, crtB, crtI from Pantoea)
- Native IPP supply from MEP pathway may be insufficient → overexpress dxs (rate-limiting MEP step)

Alternative (mevalonate pathway for IPP supply):
- Introduce complete mevalonate pathway from *S. cerevisiae* (acetyl-CoA → mevalonate → IPP)
- Provides higher IPP flux
- 7 heterologous enzymes + 3 carotenoid genes = 10 total heterologous steps

Decision: start with MEP pathway (fewer heterologous genes) + overexpress dxs + crtEBI from Pantoea. If titer is insufficient, consider mevalonate pathway for IPP.

## Why This Matters

Identifying the right biosynthetic route before beginning any wet-lab work is the most impactful design decision in metabolic engineering. A route that requires 12 heterologous enzymes, generates a toxic intermediate, and consumes 4 net NADPH may be theoretically possible but practically unachievable in a reasonable timeframe. Retrobiosynthesis tools like RetroPath2.0, combined with thermodynamic evaluation and enzyme availability assessment, compress weeks of literature search into hours of computation and allow comparison of dozens of candidate routes before committing to any single one. The quality of the initial route selection is often the primary determinant of whether a metabolic engineering project succeeds.

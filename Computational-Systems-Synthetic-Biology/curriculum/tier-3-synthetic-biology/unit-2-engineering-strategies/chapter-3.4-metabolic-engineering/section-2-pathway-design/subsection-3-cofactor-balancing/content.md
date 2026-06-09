# Cofactor Balancing

Here is a scenario that will frustrate you if you encounter it unprepared: you have designed a beautiful pathway, selected enzymes with good kcat values and reasonable Km values, expressed them all at high levels — and the titer is still disappointing. Every enzyme in the pathway is there, active, and well-expressed. What went wrong? In many cases, the answer is cofactor imbalance. Most biosynthetic pathways consume or produce redox cofactors (NADH, NADPH, FADH₂) and energy carriers (ATP). If these cofactors are not regenerated at the same rate they are consumed, pathway flux stalls regardless of how well-expressed the enzymes are. Cofactor balancing — ensuring that cofactor supply matches demand throughout the cell — is often the decisive engineering challenge in high-flux metabolic engineering.

## The Major Cofactors and Their Roles

### NADH

**Production**: glycolysis (GAPDH: 2 NADH per glucose), pyruvate dehydrogenase (1 NADH), TCA cycle (3 NADH per acetyl-CoA)

**Consumption**: electron transport chain (ETC): NADH → NAD⁺ + 2e⁻ → ATP via oxidative phosphorylation. In anaerobic conditions, NADH is consumed by fermentation reactions (lactate dehydrogenase, alcohol dehydrogenase).

**Intracellular [NADH]/[NAD⁺] ratio**: typically 0.01–0.05 in aerobic cells (NAD⁺ maintained in excess to drive glycolysis); 0.1–0.5 in anaerobic cells.

**For metabolic engineering**: NADH is relatively abundant in aerobically growing cells. Biosynthetic pathways requiring NADH as reductant can typically access sufficient supply without specific engineering.

### NADPH

**Production**: pentose phosphate pathway (PPP) — G6PDH and 6PGDH (2 NADPH per glucose entering PPP); isocitrate dehydrogenase (ICDH) in TCA (1 NADPH per turn); malic enzyme (in some organisms).

**Consumption**: biosynthesis — fatty acid synthesis (~14 NADPH per C16 fatty acid), terpenoid synthesis, amino acid biosynthesis (Glu, Pro, Arg pathways), nucleotide biosynthesis.

**Intracellular [NADPH]/[NADP⁺] ratio**: 10–100 in *E. coli* (NADPH kept highly reduced; it's a biosynthetic cofactor, not an energy carrier).

**For metabolic engineering**: NADPH is frequently limiting in high-flux biosynthetic pathways because:
- Only ~30% of glucose flux normally enters PPP (the NADPH-generating pathway)
- High flux through biosynthetic pathways exceeds normal NADPH supply

### ATP

**Production**: glycolysis (substrate-level phosphorylation: 2 ATP/glucose), oxidative phosphorylation (up to 30 ATP/glucose aerobically), TCA cycle (1 GTP per turn ≈ 1 ATP equivalent).

**Consumption**: biosynthesis (amino acid activation, nucleotide synthesis, cell wall), transport (ABC transporters, flagellar rotation), cell maintenance.

**For metabolic engineering**: ATP is rarely limiting unless the pathway has an unusual ATP stoichiometry (e.g., multiple ATP-activating steps with no ATP-generating steps), or production occurs at very high flux in anaerobic conditions where ATP yield is limited.

## Identifying Cofactor Imbalances

**Method 1: Stoichiometric analysis of the target pathway**

Write the balanced equation for your target pathway from precursor to product, including all cofactor transformations:

*Example: isobutanol from pyruvate*
$$2 \text{Pyruvate} \rightarrow \text{Acetolactate} \xrightarrow{+2H} \text{2,3-DHIV} \rightarrow \text{2-KIV} \rightarrow \text{Isobutyraldehyde} \xrightarrow{+2H} \text{Isobutanol}$$

Balance: 2 pyruvate + 2 NADPH → 1 isobutanol + CO₂ + H₂O

From glucose: glycolysis produces 2 NADH, PPP produces ~0.6 NADPH per glucose equivalent entering glycolysis.

**Net cofactor demand**: isobutanol synthesis requires 2 NADPH per isobutanol = 2 NADPH per 2 pyruvate consumed. Under standard fermentation conditions, NADPH supply is ~0.4–0.6 NADPH per pyruvate — insufficient. Engineering is required.

**Method 2: FBA with cofactor constraints**

In a genome-scale model, set the target pathway flux to the desired rate and solve FBA. Inspect cofactor metabolite fluxes: if NADPH uptake fluxes are near maximum, NADPH is limiting.

## Cofactor Engineering Strategies

### Strategy 1: Increase NADPH Production

**Overexpress PPP enzymes**:
- glucose-6-phosphate dehydrogenase (zwf): the first committed PPP step; overexpression diverts more glucose through PPP → more NADPH
- 6-phosphogluconate dehydrogenase (gnd): second NADPH-generating PPP step

*Effect*: can increase NADPH production by 2–5-fold in *E. coli*; reduces biomass slightly (less carbon through lower glycolysis)

**Express NADH kinase**: converts NADH to NADPH at the expense of ATP:
$$\text{NADH} + \text{ATP} \rightarrow \text{NADPH} + \text{ADP}$$
This is energetically costly but increases NADPH from the abundant NADH pool.

**Malic enzyme overexpression**: malate → pyruvate + CO₂ + NADPH. Useful when malate accumulates.

### Strategy 2: Reduce NADPH Consumption by Competing Pathways

Identify biosynthetic pathways that consume NADPH but are not required at maximum rate:
- Fatty acid synthesis consumes large amounts of NADPH; reducing growth rate (nitrogen limitation) reduces fatty acid synthesis
- Eliminate competing biosynthetic pathways if the products are supplied in the medium

### Strategy 3: Switch Cofactor Specificity

If an NADPH-requiring enzyme in your pathway has an NADH-dependent isozyme with comparable kinetics, switch to the NADH-dependent version:

**Example**: for terpenoid synthesis in yeast, HMG-CoA reductase (HMGR) uses NADPH. An engineered HMGR variant with NADH preference reduces NADPH consumption and can use the more abundant NADH.

**Transhydrogenase expression**: PntAB (membrane-bound transhydrogenase from *E. coli*, or its equivalent) converts NADH + NADP⁺ ⇌ NAD⁺ + NADPH at the cost of a proton gradient:
$$\text{NADH} + \text{NADP}^+ + \text{H}^+_{out} \rightleftharpoons \text{NAD}^+ + \text{NADPH} + \text{H}^+_{in}$$

Direction depends on proton gradient and [NADH]/[NADPH] ratio. In aerobic *E. coli*, this reaction runs in the NADPH-producing direction, providing additional NADPH from NADH.

### Strategy 4: Redesign Pathway Stoichiometry

In some cases, an alternative pathway using a different cofactor exists. The Liao group demonstrated this for isobutanol: replacing the NADPH-dependent ketol-acid reductoisomerase (IlvC) with an NADH-dependent variant (engineered), making the entire isobutanol pathway NADH-dependent — matching the abundant NADH supply from glycolysis.

## Worked Example: Lycopene Production and NADPH

Lycopene biosynthesis via MEP pathway:

MEP pathway summary (condensed):
- 3 NADPH consumed per IPP produced (in the DXR, HDS, HDR steps)
- 2 IPP + 1 DMAPP → GPP; 2 IPP + GPP → FPP; 2 FPP → GGPP → phytoene → lycopene
- Net: ~12 NADPH per lycopene molecule

For 1 g/L lycopene production (≈ 0.001 mol lycopene from 0.055 mol glucose):
- NADPH demand: 0.012 mol NADPH
- PPP NADPH supply at 30% flux diversion: ~0.033 mol NADPH per 0.055 mol glucose

In this case, NADPH supply exceeds demand at low titers, but at target titers of 3–5 g/L, NADPH becomes limiting. Standard engineering response: overexpress zwf + gnd to increase PPP flux.

## Why This Matters

Cofactor imbalance is among the most common reasons a metabolically engineered pathway produces disappointing titers and rates despite all individual enzymes being well-expressed and individually active. Understanding the cofactor stoichiometry of a pathway before construction enables rational engineering of the regeneration machinery alongside the biosynthetic pathway — rather than discovering the limitation only after rounds of optimization. The systematic analysis of NADPH/NADH budgets, combined with targeted engineering of PPP flux, transhydrogenase expression, or cofactor-specificity switching, is what separates expert metabolic engineers from novices attempting pathway construction without systems-level thinking.

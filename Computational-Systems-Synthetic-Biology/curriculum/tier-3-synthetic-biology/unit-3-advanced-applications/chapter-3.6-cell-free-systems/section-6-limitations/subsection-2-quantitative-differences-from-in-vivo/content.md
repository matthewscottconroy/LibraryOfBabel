# Quantitative Differences Between Cell-Free and In Vivo Systems

Suppose you measure a promoter in cell-free and find that it drives GFP to 400 µg/mL. You then clone the same construct into E. coli and induce expression. The GFP comes out at 0.4 µg/mL. You haven't made an error — you've encountered one of the most important quantitative facts about cell-free biology. Beyond the categorical limitations of cell-free systems — processes they simply cannot perform — there are quantitative differences in measurable parameters between cell-free and in vivo contexts. These differences matter because they determine how accurately cell-free measurements can predict in vivo behavior. Understanding these quantitative gaps enables researchers to interpret cell-free data correctly and to build calibration models that translate cell-free observations into predictions for cellular experiments.

## Expression Level Differences

**Cell-free expression levels** are dramatically higher per molecule of DNA than in vivo, due to the absence of competing reactions and the concentrated ribosome/RNAP pool.

Typical comparison for GFP expressed from a T7 promoter:

| System | [GFP] at plateau |
|---|---|
| E. coli cell-free (S30 extract) | 100–500 µg/mL = 3–15 µM |
| PURE system | 50–200 µg/mL = 1.5–6 µM |
| E. coli BL21 (DE3) in vivo, 1 mM IPTG induction | 0.1–1 µg/mL = 3–30 nM |

The difference is approximately **100–1000×** higher concentration in cell-free than in cells under typical conditions. This has several practical consequences:

1. Cell-free cannot be used to predict absolute protein concentrations in cells
2. Protein-protein interactions measured by co-expression in cell-free occur at much higher concentrations — weak interactions (Kd > 1 µM) that would not occur in cells may appear significant in cell-free
3. Reporter signal calibration must account for concentration differences when comparing cell-free PoPS measurements to in vivo promoter activity

**Why the large difference?** In an E. coli cell, the total ribosome pool is shared among ~4000 mRNA species from the endogenous genome plus the plasmid-borne transgene. The transgene captures typically <1–5% of total ribosome flux. In cell-free, the transgene DNA is the primary (or only) template; essentially 100% of ribosome flux goes to the exogenous circuit.

## Timescale Differences

Gene expression kinetics in cell-free are fundamentally faster than in vivo, and for a mechanistic reason:

**In vivo dynamics** are dominated by **dilution by cell growth**:
$$\frac{d[\text{protein}]}{dt} = \beta_{synthesis} - (\delta_{degradation} + \mu_{growth}) \times [\text{protein}]$$

Where $\mu_{growth}$ (E. coli growth rate, typically 0.7–1.2 h⁻¹) is a major term. At steady state:
$$[\text{protein}]_{ss} = \frac{\beta_{synthesis}}{\delta_{degradation} + \mu_{growth}}$$

For a stable protein, $\delta_{degradation} \approx 0$, so dilution by growth is the primary "effective degradation" rate, with time constant $\tau = 1/\mu \approx 1$ hour.

**Cell-free dynamics** have no dilution by growth:
$$\frac{d[\text{protein}]}{dt} = \beta_{synthesis}(t) - \delta_{degradation} \times [\text{protein}]$$

Protein accumulates until either (a) substrates are exhausted or (b) degradation (by extract proteases) brings it to a new steady state. Without growth dilution, the effective time constant is longer — protein persists for hours without being diluted.

The practical consequence: dynamic behaviors that occur on the timescale of hours in cells (bistable switch transitions, oscillations with period ~2 hours) occur on the timescale of minutes in cell-free. A toggle switch measured to have 4-hour switching kinetics in E. coli may appear to switch in 30–60 minutes in cell-free. The **qualitative behavior** (bistability, oscillation) is preserved, but the **quantitative timescale** is compressed.

**Rough scaling rule**: cell-free kinetics are approximately 4–10× faster than in vivo kinetics for the same circuit, primarily due to the absence of growth dilution.

## Resource Competition Differences

In cells, expressing a new gene diverts ribosomes and RNAP from endogenous genes, causing global expression changes — **resource competition** or **metabolic burden**.

Classic example: overexpressing a toxic or metabolically expensive protein in E. coli from a strong promoter on a high-copy plasmid causes:
- Reduction in growth rate (sometimes 20–50% reduction)
- Reduction in expression of endogenous genes
- Induction of stress response (heat shock, SOS response)
- Plasmid instability through selection for mutations reducing expression

In cell-free, there is no endogenous genome to compete with. Resource competition exists but takes a different form:
- Multiple gene circuits in the same cell-free reaction compete with each other for ribosomes
- The total protein synthesis capacity of the extract is finite (~0.5–1 mg/mL/hour)

**Quantitative model for cell-free resource sharing** (Gyorgy et al., 2015):

If two circuits are expressed in the same cell-free reaction, each consuming ribosome resource $R$ at rates $k_1$ and $k_2$:
$$[\text{circuit 1 output}] = \frac{k_1 \cdot R}{k_1 + k_2}$$
$$[\text{circuit 2 output}] = \frac{k_2 \cdot R}{k_1 + k_2}$$

This zero-sum competition means that a highly active circuit A will reduce the expression of circuit B — which can be mistaken for regulatory interaction between A and B if controls are not included. This type of **indirect coupling through shared resources** occurs in both cell-free and in vivo, but the quantitative parameters differ because the pool sizes and competing demands differ.

## Correlating Cell-Free to In Vivo: Empirical Models

Given these systematic differences, calibration models can be built to translate cell-free measurements to in vivo predictions:

**For promoter rank ordering** (relative promoter strength):

The correlation between cell-free and in vivo promoter rankings is well-established empirically:
$$\text{Rank}_{in vivo} \approx f(\text{Rank}_{cell-free})$$

Spearman ρ ≈ 0.7–0.9 for T7 promoter variants, sigma-70 promoter variants, and synthetic constitutive promoter libraries.

**For transfer function parameters** (Hill function for repressors):

The qualitative parameters are transferable:
- $K_{1/2}$ (repressor concentration for half-maximum repression): in vivo $K_{1/2}$ ≈ 10–50× smaller than cell-free $K_{1/2}$, reflecting the lower absolute protein concentrations in cells
- $n$ (Hill coefficient): generally conserved between cell-free and in vivo (within 20%)
- ON/OFF ratio ($y_{max}/y_{min}$): sometimes lower in vivo due to leaky expression from spurious sigma factor recognition of non-cognate promoters

A simple calibration:
$$K_{1/2}^{in vivo} = K_{1/2}^{cell-free} \times f_{dilution}$$

where $f_{dilution}$ is the ratio of absolute expression levels (cell-free to in vivo), empirically determined for the protein of interest. This requires measuring the same repressor's expression level in both systems — typically done once for a given repressor/host combination.

## Protease Activity and Protein Stability Differences

**In cell-free extracts**: proteases Lon, ClpXP, and others from the extract continue to degrade proteins, but their concentrations and activities differ from the in vivo situation (diluted by the lysis process, operating outside their normal cellular context).

**Consequence**: a protein with a 20-minute half-life in E. coli cells (due to Lon or ClpXP recognition) may be more stable in cell-free extract (proteases diluted, possibly less substrate recognition in the diluted context) or less stable (if the extract inadvertently concentrates protease activity relative to the protection afforded by chaperones in vivo).

For circuit design using destabilized proteins (e.g., ssrA-tagged reporters for fast dynamics), the ssrA-ClpXP degradation rate in cell-free must be calibrated separately from the in vivo rate. ClpXP activity in extracts depends strongly on how the extract is prepared.

## Energy and ATP Availability Differences

**In cells**: ATP is continuously generated by metabolism coupled to growth and maintenance. At mid-log growth, ATP concentration is maintained at ~3–5 mM with high flux.

**In cell-free with 3-PGA energy system**: ATP starts at ~3 mM and is maintained for 4–6 hours, then depletes. As ATP drops, translation slows, circuit dynamics slow, and eventually the reaction stops.

This creates an important artifact: reactions that depend on energy-sensitive processes (kinases, chaperones that hydrolyze ATP) may behave differently late in the cell-free reaction as energy depletes. Always verify that the measurement is taken during the energy-replete phase (typically within the first 4 hours for a 3-PGA system).

## Practical Calibration Protocol

For a research group using cell-free to predict in vivo behavior of new genetic circuits:

1. **Establish a calibration library**: characterize 10–20 known circuits (from published work) in your cell-free system. Collect the same measurements for those circuits in your in vivo system (same host, same induction conditions).

2. **Compute calibration factors**: for each parameter of interest (promoter strength, K_1/2, Hill coefficient), compute the cell-free-to-in-vivo ratio across the calibration library.

3. **Apply calibration**: for a new circuit characterized in cell-free, apply the calibration factors to predict in vivo parameters. Report both the raw cell-free measurement and the calibrated prediction.

4. **Test calibration with held-out circuits**: validate the calibration on circuits not used to build it. If calibration is accurate, in vivo measurements for 80% of held-out circuits should fall within 2-fold of the calibrated prediction.

## Why This Matters

The quantitative differences between cell-free and in vivo systems are not bugs — they are features of the system that must be understood to use the technology correctly. A researcher who assumes that cell-free promoter activity directly predicts in vivo expression level will draw incorrect quantitative conclusions. A researcher who understands that cell-free correctly predicts rank order but systematically overestimates absolute levels — and who has built a calibration model to correct for this — will extract accurate in vivo predictions from fast, cheap cell-free experiments. This calibration-based workflow is the mature application of cell-free to the DBTL cycle: use cell-free for high-throughput screening, build a calibration model from a small number of in vivo measurements, and apply the model to predict in vivo behavior for the screened variants. This workflow is already standard practice in the most productive synthetic biology laboratories and is increasingly being codified in published computational tools.

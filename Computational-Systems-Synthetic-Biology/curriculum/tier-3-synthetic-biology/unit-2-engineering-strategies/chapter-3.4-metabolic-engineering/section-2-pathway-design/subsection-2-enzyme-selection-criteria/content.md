# Enzyme Selection Criteria

You have your pathway on paper: a clean retrobiosynthetic route from a native precursor to your target compound, six steps, each catalyzed by a known enzyme class. But "enzyme class" is not the same as "working enzyme." For each step in your pathway, dozens of homologs may exist across the tree of life — some fast, some slow, some with Km values that make them useless under intracellular conditions, some dependent on cofactors your chassis can barely supply. Enzyme selection is where the gap between a textbook pathway and a functioning strain is first established. Once a biosynthetic route has been identified, every enzymatic step must be populated with a specific enzyme from a real organism. Multiple candidate enzymes often exist for each step — homologs from different organisms with different kinetic parameters, substrate specificities, and expression characteristics. Selecting the best enzyme for each step is a critical design decision that substantially affects pathway productivity.

## The Key Selection Criteria

### 1. Substrate Specificity and Selectivity

The enzyme must accept the actual substrate in your pathway, not just a structurally similar compound. BRENDA reports substrate specificity data (relative activity on various substrates), but this data is often sparse or absent for the exact substrate of interest.

**Practical approach**:
- Search BRENDA for enzymes with activity on the substrate of interest; filter by Km ≤ likely intracellular substrate concentration
- If no well-characterized enzyme exists, use structural homology to predict specificity: enzymes sharing >40% identity with a characterized enzyme likely share substrate specificity
- Check for **substrate promiscuity**: some enzymes with broad specificity may be more useful than highly specific ones that lack activity on your substrate

**Selectivity**: for chiral substrates or stereospecific reactions, the enzyme must produce the correct stereoisomer. Cofactor identity (NADH vs. NADPH) is also a selectivity criterion: using an NADH-dependent enzyme in a NADPH-limited context reduces pathway efficiency.

### 2. Catalytic Rate (kcat)

**kcat** (turnover number) is the maximum number of substrate molecules converted per enzyme active site per second. Units: s⁻¹.

$$v_{max} = k_{cat} \times [E]_{total}$$

A higher kcat means less enzyme is needed to sustain a given flux. In metabolic engineering, where each heterologous enzyme competes for cellular resources (ribosomes, amino acids), minimizing enzyme expression burden while maintaining sufficient activity is a key optimization objective.

Typical kcat values range from 0.001 s⁻¹ (slow enzymes: complex natural products) to 10,000 s⁻¹ (fast enzymes: carbonic anhydrase). Most metabolic enzymes: 1–100 s⁻¹.

**Rule of thumb**: prefer enzymes with kcat > 1 s⁻¹ for the first round of pathway construction. Very slow enzymes (kcat < 0.01 s⁻¹) may need protein engineering to be useful in vivo.

### 3. Michaelis Constant (Km)

**Km** is the substrate concentration at which the reaction proceeds at half-maximal velocity. A low Km means the enzyme has high affinity for its substrate — it operates near Vmax even at low substrate concentrations.

$$v = \frac{v_{max}[S]}{K_m + [S]}$$

In metabolic engineering, intracellular metabolite concentrations are typically 0.1–1 mM (for central metabolites) or lower (for pathway intermediates). An enzyme with Km = 10 mM would operate at only 1–10% of Vmax at these concentrations — a 10–100-fold reduction in effective rate.

**Rule**: select enzymes with Km comparable to or lower than the expected intracellular substrate concentration. If the only available enzyme has Km >> expected concentration, engineer the strain to accumulate the substrate (by blocking downstream consumption) or engineer the enzyme to reduce Km.

### 4. Catalytic Efficiency (kcat/Km)

The **specificity constant** $k_{cat}/K_m$ captures both affinity and rate in a single parameter. It represents the rate of reaction at low substrate concentrations (when $[S] \ll K_m$):

$$v \approx \frac{k_{cat}}{K_m}[S][E] \quad \text{when } [S] \ll K_m$$

For selecting between enzyme candidates when substrate concentration is limiting, maximize kcat/Km. The diffusion limit for enzyme-substrate encounters is ~10⁸–10⁹ M⁻¹s⁻¹; "perfect enzymes" like triose phosphate isomerase operate near this limit.

### 5. Thermodynamic Favorability

The reaction must be thermodynamically spontaneous under intracellular conditions. The **Gibbs free energy of reaction** ($\Delta_r G'$) must be negative for the reaction to proceed in the desired direction.

$$\Delta_r G' = \Delta_r G'^\circ + RT \ln \left(\frac{\prod [products]}{\prod [substrates]}\right)$$

Reactions with $\Delta_r G'^{\circ} > 0$ (endergonic under standard conditions) can still be thermodynamically favorable if products are rapidly consumed (low product concentration) or substrates accumulate. However, reactions with $\Delta_r G'^{\circ} > +15$ kJ/mol are practically infeasible unless coupled to an ATP-hydrolyzing step.

**eQuilibrator**: web-based tool that calculates $\Delta_r G'^{\circ}$ and $\Delta_r G'$ at physiological metabolite concentrations for any reaction drawn from known metabolites.

### 6. Cofactor Requirements

Most oxidoreductases require either NADH or NADPH as cofactor. The intracellular availability of these cofactors differs substantially:

- **NADH**: primarily produced in glycolysis (GAPDH) and TCA cycle; used mainly in energy metabolism and electron transport chain. Relatively abundant in aerobic growth.
- **NADPH**: primarily produced in pentose phosphate pathway (G6PDH, 6PGDH) and TCA (ICDH). Used primarily in biosynthesis (fatty acid synthesis, amino acid biosynthesis). Often limiting in high-flux biosynthetic pathways.

**When NADPH is limiting**:
- Prefer NADH-dependent enzymes if available and kinetically comparable
- Overexpress NADPH-regenerating enzymes (glucose-6-phosphate dehydrogenase, NADH kinase)
- Express transhydrogenase (converts NADH to NADPH at cost of proton gradient)

### 7. Expression Compatibility

**Codon usage**: genes from organisms with different codon usage tables express poorly in the chassis. Codon-optimize the coding sequence for the chassis organism using tools like IDT Codon Optimization or JCat. Modern gene synthesis companies offer codon optimization as a standard service.

**GC content**: very high (>75%) or very low (<25%) GC content can cause problems: high GC sequences fold in structured mRNA that reduces translation; extremely low GC sequences may have poor promoter recognition.

**Protein folding**: some enzymes require specific chaperones or co-factors for proper folding that may be absent in the chassis. Screen for soluble expression by Western blot or activity assay.

## A Decision Framework for Enzyme Selection

```python
def rank_enzyme_candidates(candidates, expected_substrate_conc, chassis="E. coli"):
    """
    Score enzyme candidates for a metabolic engineering pathway step.
    """
    scores = []
    for enzyme in candidates:
        # Kinetics score: higher is better
        km_score = 1.0 / (1.0 + enzyme.km / expected_substrate_conc)
        kcat_score = min(enzyme.kcat / 10.0, 1.0)  # normalize to 10 s-1
        
        # Thermodynamics: must be feasible
        if enzyme.delta_g_prime > 15:  # kJ/mol
            continue  # skip thermodynamically unfavorable enzymes
        
        # Cofactor match
        cofactor_score = 1.0 if enzyme.cofactor_matches(chassis) else 0.5
        
        # Expression compatibility
        codon_score = 1.0 if enzyme.organism == chassis else 0.7  # assumes codon opt needed
        
        total_score = km_score * kcat_score * cofactor_score * codon_score
        scores.append((enzyme, total_score))
    
    return sorted(scores, key=lambda x: x[1], reverse=True)
```

## Example: Selecting an Amorphadiene Synthase for Artemisinin Production

For artemisinin production in yeast, the first committed step is cyclization of farnesyl pyrophosphate (FPP) → amorphadiene, catalyzed by **amorphadiene synthase (ADS)** from *Artemisia annua*.

ADS characterization:
- Km for FPP: 1.2 μM (lower than typical intracellular FPP: ~50 μM → well saturated)
- kcat: 0.008 s⁻¹ (very slow — requires high expression level or protein engineering)
- No cofactor required (class I terpene cyclase)
- Plant enzyme: requires codon optimization for yeast expression
- $\Delta_r G'^{\circ}$: highly negative (cyclization reactions are thermodynamically favored)

Decision: use ADS but overexpress it (strong promoter, high-copy integration) to compensate for low kcat. Keasling group achieved this by expressing ADS from GAL1 promoter at ~30 copies per cell.

## Why This Matters

Enzyme selection is where the gap between a textbook pathway on paper and a functional strain in a flask is established. An enzyme with excellent kcat but Km 100-fold above intracellular substrate concentrations will operate at 1% of its potential — rate-limiting the entire pathway regardless of how well everything else is optimized. Conversely, choosing an enzyme with the correct cofactor specificity can eliminate the need for expensive cofactor engineering interventions. The systematic evaluation of kcat, Km, thermodynamics, cofactor, and expression compatibility before synthesis orders are placed is what separates modern rational metabolic engineering from the trial-and-error approach that characterized the field's early years.

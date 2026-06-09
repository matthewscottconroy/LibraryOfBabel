# Metabolic Burden: Resource Competition Between Circuits and the Host Cell

A circuit that works beautifully in a four-hour experiment can fail catastrophically in a twenty-four-hour fermentation. Not because anything in the DNA changes, not because the circuit logic is wrong — but because the cell itself changes. Cells carrying heavy synthetic loads grow slower than cells that do not. Slower-growing cells encounter mutations and selection pressures that reward anyone who loses the circuit. After enough generations, the culture is dominated by cells that have deleted, silenced, or restructured the construct. The experiment worked; the fermentation failed. The underlying problem is **metabolic burden**: the drain that synthetic gene expression imposes on the shared resources of a living cell.

Every gene expressed in a cell consumes shared cellular resources: ribosomes, RNA polymerase, amino acids, nucleotides, ATP, and molecular chaperones. In a cell running at near-maximum growth rate, these resources are already largely allocated to essential housekeeping functions. A synthetic genetic circuit that demands a substantial fraction of cellular resources necessarily redirects those resources away from growth, reducing the growth rate — and can trigger cellular stress responses that further alter circuit behavior. This is metabolic burden, and it is one of the most pernicious failure modes in synthetic biology because it creates a coupling between circuit function and host fitness that is difficult to model or predict.

## The Sources of Metabolic Burden

### Ribosome Competition
Protein synthesis is the largest metabolic cost in a typical bacterium. In *E. coli* growing at maximal rate:
- ~80% of ribosome activity is devoted to synthesizing proteins needed for growth
- ~20% can potentially be allocated to heterologous protein production without measurable growth rate reduction
- Beyond ~20%, each additional % of ribosome allocation causes approximately 1% growth rate reduction

A high-expression synthetic gene consuming 5% of cellular ribosomes reduces growth rate by ~5%; this is often significant enough to cause evolutionary selection against the circuit.

### RNA Polymerase Competition
Each promoter creates a potential binding site for RNAP. Many strong promoters competing for limited RNAP (only ~2000–3000 molecules per *E. coli* cell) reduce the effective RNAP available for all other genes, including essential housekeeping genes. This effect is most severe for:
- T7 RNAP systems (T7 RNAP is ~5× faster than *E. coli* RNAP but competes for NTP pools)
- Multiple strong promoters on high-copy plasmids
- Circuits using the σ70 promoter class (same pool as most essential genes)

### Plasmid Replication and Maintenance
High-copy plasmids impose direct metabolic costs:
- ColE1 origin plasmid at 100 copies: replicon maintenance consumes ~2–3% of cellular DNA replication capacity
- Antibiotic resistance genes on plasmids: constitutive expression of resistance proteins (e.g., AmpR β-lactamase) ~200–500 molecules/cell of unnecessary protein
- Plasmid maintenance in dividing cells: cells that lose plasmids grow faster; selection for plasmid loss is constant without antibiotic

### ATP and Amino Acid Pools
Highly expressed synthetic genes may deplete the ATP pool (through translation) faster than cellular energy generation can replenish it, causing growth rate reduction and triggering stringent response (ppGpp accumulation). Amino acid depletion is less common but occurs with extremely high expression of proteins with unusual amino acid compositions.

## The Growth Rate-Production Trade-off

A core mathematical description relates growth rate to synthetic gene expression level:

$$\mu = \mu_0 \left(1 - \frac{B}{\lambda}\right)$$

Where:
- $\mu_0$: maximum growth rate in the absence of burden
- $B$: fraction of cellular resources consumed by the synthetic circuit
- $\lambda$: a constant representing the cell's total productive capacity (normalized to 1)

This model predicts that as $B$ increases (higher circuit expression), growth rate decreases. When $\mu$ drops below a threshold where cells without the plasmid grow faster, evolutionary pressure drives loss of the circuit.

In practice, burden can be measured by expressing GFP at different levels from a well-characterized promoter and measuring the growth rate at each expression level. The slope of the growth rate vs. expression level curve is the "burden coefficient" for that experimental system.

## Feedback Between Burden and Circuit Behavior

Metabolic burden creates a destructive feedback loop:
1. Circuit is expressed → consumes resources → growth rate reduced
2. In slower-growing cells, some global regulators (RpoS, ppGpp) are upregulated
3. These global regulators alter the activity of housekeeping promoters and σ factors
4. The alteration affects circuit promoters (which are recognized by the same σ70)
5. Circuit expression level changes — usually decreasing — in ways not predicted by isolated characterization

This means a circuit characterized in exponentially growing cells (low σS, low ppGpp) behaves differently in nutrient-limited or densely growing cells (high σS, high ppGpp), exactly the conditions where metabolic burden is most severe.

## Mitigation Strategies

### Use Tight Inducible Promoters
Express circuit components only when needed. A circuit that is induced only during a 2-hour production phase imposes burden for 2 hours, not continuously. Tight inducible promoters with low basal expression minimize the burden in the non-induced state.

### Minimize Protein Size
Smaller proteins consume fewer ribosomes. For circuits where the regulatory function can be provided by a short peptide or domain, minimizing the size of the circuit protein reduces the translation burden.

### Orthogonal Expression Systems
T7 RNAP-driven circuits are partially orthogonal to host RNAP pool, but they compete for NTP pools and ribosomes. Truly orthogonal systems (orthogonal ribosomes that translate only synthetic mRNAs) can decouple circuit translation from host translation — under development but not yet widely used.

### Chromosomal Integration
Integrating circuit genes into the chromosome at single copy per cell dramatically reduces replication and plasmid maintenance burden compared to high-copy plasmids. Expression from a chromosomal locus is also more stable against evolutionary loss (no plasmid-free cells can arise).

### Protein Degradation Tags
ssrA tags on circuit proteins ensure rapid turnover, reducing steady-state protein concentration and therefore the ribosome fraction needed to maintain circuit protein levels. Shorter protein half-lives also reduce the amount of biosynthetically costly amino acids tied up in circuit proteins at any given time.

### Growth-Decoupling: Production in Non-Growing Cells
Expressing circuit components only in stationary-phase or nutrient-limited cells, when growth is negligible, decouples the production from growth rate effects. Quorum sensing-based switching systems (section 3.4) implement this by transitioning from growth mode to production mode at high cell density.

## Worked Example: Quantifying Burden from a Metabolic Pathway

An *E. coli* strain was engineered with a 5-enzyme terpenoid biosynthesis pathway on a ColE1 plasmid, each enzyme under a strong constitutive promoter. Measured outcomes:

| Condition | Growth rate (h⁻¹) | Product titer (mg/L) | Burden fraction |
|---|---|---|---|
| No plasmid | 0.95 | 0 | 0% |
| Pathway plasmid (no inducer) | 0.82 | 2.1 | 14% |
| Pathway plasmid (full induction) | 0.61 | 45 | 36% |

At full induction, the pathway consumes ~36% of cellular resources — far above the threshold for evolutionary instability. After 50 generations without antibiotic selection, 80% of cells had lost the plasmid.

**Fix**: switch pathway enzymes to RBS variants with 5–10-fold lower expression, move rate-limiting enzyme to high-expression and others to low-expression. This reduced burden fraction to 12% while maintaining 78% of product titer, and stability improved dramatically (< 5% plasmid loss after 50 generations).

## Why This Matters

Metabolic burden is the reason that a circuit working perfectly in a 4-hour experiment may fail in a 24-hour fermentation. As circuit complexity grows — more genes, more regulatory proteins, more metabolic pathway enzymes — the aggregate burden increases until it triggers evolutionary selection against the circuit itself. Designing for low burden is not a constraint on circuit function but a prerequisite for circuit longevity. Understanding burden quantitatively allows practitioners to calculate whether a proposed circuit is in the "safe zone" (< 15–20% resource consumption in bacteria) before building it, and to choose the right mitigation strategy when it is not.

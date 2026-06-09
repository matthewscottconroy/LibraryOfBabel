# Dynamic Metabolic Engineering

There is a deep tension at the heart of metabolic engineering, one that no amount of careful pathway design can eliminate by itself: the conditions that are best for growing cells are not the conditions that are best for making product. During rapid growth, the cell's entire machinery — ribosomes, ATP, precursor metabolites — is aimed at making more cell. Redirecting those resources toward your target chemical is fighting evolution. Static metabolic engineering — designing a strain to produce a compound at a fixed gene expression state — cannot resolve this tension, only endure it. Dynamic metabolic engineering resolves this by programmatically switching cellular gene expression between growth and production modes as the fermentation progresses.

## The Growth-Production Tradeoff

In exponential growth, cellular resources (ribosomes, ATP, carbon) are directed toward biomass production — synthesizing new DNA, RNA, protein, and membrane lipids. A strain forced to simultaneously produce a target chemical competes with growth for the same resource pool.

At high cell density (stationary phase), growth slows as nutrients become limiting, but:
- Product-consuming biosynthetic enzymes are still active
- The cell is under stress (nutrient limitation, product toxicity)
- Resources are available for production if properly redirected

**Ideal strategy**: allow cells to grow freely (maximizing biomass), then switch at the right moment to redirect resources from growth to production. This requires the switch to happen automatically — sensing cell density or a metabolic signal — without requiring manual intervention.

## Quorum Sensing-Based Switches

**Quorum sensing** (QS) is the natural bacterial mechanism for population-density detection. The most widely used system for metabolic engineering is the LuxI/LuxR system from *Vibrio fischeri*:

**Mechanism**:
- LuxI: autoinducer synthase; produces N-(3-oxohexanoyl)-L-homoserine lactone (OHHL, a type of **AHL** — acyl-homoserine lactone)
- AHL accumulates in culture proportional to cell density
- At high AHL concentrations, AHL binds LuxR protein
- LuxR-AHL complex activates target promoters (PLux)

**Engineering application**:
1. Express LuxI constitutively in the production strain → AHL accumulates proportionally to OD
2. Place production pathway enzymes under PLux control → low expression at low OD (growth phase), high expression at high OD (production phase)
3. No manual induction required; the system is autonomous

**Worked example**: Williams et al. (2015) used this system for fatty acid ethyl ester (FAEE) production. During exponential growth, FAEE pathway genes were off. At OD 10–12, AHL reached the PLux activation threshold, switching on expression of WS/DGAT (wax ester synthase). FAEEs accumulated only in the production phase, avoiding early product toxicity that reduced growth.

## Malonyl-CoA Biosensor for Fatty Acid Production Control

A more sophisticated approach uses intracellular metabolite levels rather than cell density as the control signal. **FapR**, a fatty acid synthesis regulator from *Bacillus subtilis*, binds malonyl-CoA and represses its target genes:

- Without malonyl-CoA: FapR binds fapO operator → represses target genes
- With malonyl-CoA: FapR is inactivated → target genes expressed

**Application**: to maximize polyketide or fatty acid production while preventing malonyl-CoA depletion:
1. Express pathway enzymes consuming malonyl-CoA (polyketide synthases) under FapR-repressed promoter
2. When malonyl-CoA is high (fatty acid synthesis not keeping up with supply), FapR is inactivated → polyketide synthase expressed → malonyl-CoA consumed
3. When malonyl-CoA drops too low, FapR reactivated → polyketide synthase repressed → malonyl-CoA recovers

This **negative feedback** prevents malonyl-CoA depletion, which would stall fatty acid synthesis (essential for membrane production) and kill the cell.

**Quantitative design**: the FapR system has a sigmoidal response to malonyl-CoA. The threshold concentration $K_d$ (FapR-malonyl-CoA) is ~10 µM. By choosing which genes to put under FapR control, the engineer sets the malonyl-CoA setpoint at which production begins.

## Two-Stage Bioprocesses

Not all dynamic control requires genetic switches. **Two-stage bioprocesses** physically separate growth and production phases by modifying culture conditions:

**Stage 1 (growth phase)**:
- Nutrient-rich medium
- Optimal temperature (37°C for E. coli)
- Inducible genes off
- Cell density builds to target OD (typically OD 20–80)

**Stage 2 (production phase)** triggered by:
- Adding inducer (IPTG, arabinose) to switch on production pathway
- Temperature shift (e.g., 37°C → 30°C to improve protein folding or slow growth)
- Nitrogen limitation (cells shift metabolism when N is limiting)
- pH or dissolved oxygen shift

**Example**: lycopene production at 37°C during growth phase; temperature shift to 28°C at high OD activates CrtI (lycopene synthase is more active at lower temperature, and growth slows, reducing resource competition).

## CRISPR-Based Dynamic Control

**dCas9 + metabolite-responsive sgRNA**: some sgRNA scaffolds can be engineered to respond to small molecules that alter sgRNA structure, modulating dCas9-mediated gene repression. This enables programmable transcriptional control of multiple genes simultaneously in response to intracellular metabolite levels.

**CRISPRi knockdown of growth genes at high density**: design a dCas9-KRAB circuit that represses ribosomal protein genes (essential for growth) when cell density is high (detected by QS signal). This directly reduces growth rate while maintaining production gene expression — a more direct way to redirect resources.

## Design Principles for Dynamic Metabolic Engineering

**Principle 1: Switch at the right time**. The optimal switch point balances biomass accumulation (needed for high volumetric productivity) against early product accumulation. Too early: insufficient biomass; too late: cells are stressed and production is inefficient. Optimal switch point must be empirically determined.

**Principle 2: Match sensor dynamics to pathway kinetics**. A sensor with a response time of 30 minutes controlling a pathway with a 5-minute metabolite turnover creates oscillatory behavior. Match sensor response time to pathway timescale.

**Principle 3: Avoid escape mutants**. Cells under genetic circuits that limit their growth have strong evolutionary pressure to escape by mutating the circuit. Use essential gene integration (not plasmid-based circuits); use multiple redundant switches; and minimize fermentation duration.

**Principle 4: Computational model before building**. ODE models of the feedback circuit (including QS dynamics, promoter response, enzyme expression) predict whether the designed circuit achieves stable switch behavior or oscillates. Model first; build second.

$$\frac{d[\text{AHL}]}{dt} = \alpha_{LuxI} \cdot [cell] - \delta_{AHL} \cdot [\text{AHL}]$$
$$\frac{d[P_{prod}]}{dt} = \frac{[\text{AHL}]^n}{K^n + [\text{AHL}]^n} \cdot \alpha_{max} - \delta_P \cdot [P_{prod}]$$

Where $[P_{prod}]$ is the production enzyme concentration, $\alpha_{LuxI}$ is LuxI expression rate per cell, and $n$ is the Hill coefficient for LuxR-AHL activation.

## Why This Matters

Dynamic metabolic engineering addresses the most fundamental limitation of static strain engineering: the growth-production conflict. By programming the cell to switch modes autonomously — whether using quorum sensing, metabolite sensors, or physical process control — the engineer captures the benefits of both rapid growth (high biomass for high volumetric productivity) and dedicated production (maximum resource allocation to the desired compound). The increase in titer and productivity achievable by dynamic control is often 2–5-fold compared to the best static strain, without any additional pathway engineering. As genetic circuit design matures and biological sensors become more diverse and reliable, fully autonomous cellular factories that self-regulate their entire metabolic state will become the standard for industrial biotechnology.

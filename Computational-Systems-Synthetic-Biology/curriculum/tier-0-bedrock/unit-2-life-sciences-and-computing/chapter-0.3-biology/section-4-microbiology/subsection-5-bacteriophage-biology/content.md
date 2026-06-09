# Bacteriophage Biology

The ocean contains roughly $10^{31}$ bacteriophage particles — more phages than there are stars in the observable universe by many orders of magnitude. Every second, approximately $10^{23}$ bacterial infections occur across the world's oceans. Phages are the most numerically dominant biological entities on Earth, and they are the primary agents driving bacterial diversity, evolution, and population dynamics. Yet until recently they were barely taught in standard biology courses. For computational biologists, phages matter not only as ecological forces but as exceptional tools — serving as vectors, as model systems for genetic circuits, and as templates for phage display and phage therapy.

Bacteriophages (phages) are viruses that infect bacteria. They are the most abundant biological entities on Earth (~$10^{31}$ total), constitute >90% of the virus particles in the ocean, and are major drivers of bacterial evolution, diversity, and population dynamics. Beyond their ecological role, phages are exceptional tools for molecular biology.

## Phage Life Cycle Decisions: Lytic vs. Lysogenic

When a phage infects a bacterium, it faces a fork in the road:

**Lytic cycle**: The phage immediately takes over the cell's molecular machinery, produces hundreds of phage particles, then lyses (bursts) the host cell, releasing progeny. The cycle completes in ~30–60 min. The key parameter is the **burst size** (β): typical values for T4 phage are ~100–200 phage per lysed cell.

**Lysogenic cycle**: The phage DNA integrates into the host chromosome (or circularizes as an episome) and becomes a **prophage**. The prophage is replicated passively along with the host chromosome at every cell division. Prophage genes (except the **lysogeny maintenance functions**) are repressed. The prophage can be induced by SOS signals (DNA damage, etc.) to excise and enter the lytic cycle.

The lytic/lysogenic decision is made early in infection and depends on environmental conditions (multiplicity of infection, host growth rate, nutrient availability) and stochastic fluctuations in regulatory protein concentrations.

## Lambda Phage: The Canonical Genetic Switch

Lambda (λ) phage is the best-studied model of a bistable genetic switch. Its lytic/lysogenic decision is regulated by two key transcription factors:

**CI (the λ repressor)**: A site-specific DNA-binding protein that represses all lytic-cycle genes by binding **OR** (the right operator) and **OL** (the left operator). CI also cooperatively activates its own transcription from P_RM_, creating a positive feedback loop that maintains the lysogenic state. At high concentrations, CI dimers form octamers via long-range interactions between OR and OL, further stabilizing repression.

**Cro**: A small repressor protein that binds the same operator sites as CI but with opposite affinity ranking. While CI binds OR1 > OR2 > OR3, Cro binds OR3 > OR2 > OR1. High Cro levels repress P_RM_ (the CI promoter), preventing CI synthesis. Cro drives commitment to the lytic cycle.

The CI/Cro circuit is a **bistable switch**:
- **Lysogenic state**: high CI, low Cro; CI represses lytic genes and activates its own promoter
- **Lytic state**: high Cro, low CI; Cro represses CI synthesis; lytic cycle proceeds

**Induction**: DNA damage activates RecA, which cleaves CI through an allosteric mechanism (CI autocleavage). Dropping CI levels allows derepression of lytic genes, prophage excision (via integrase + excisionase), and the lytic cycle to proceed.

This circuit was one of the first genetic networks modeled mathematically (Ptashne lab, 1980s) and is a canonical example in every systems biology course. It is a physical realization of a bistable switch with two stable steady states and a separatrix (unstable steady state) between them. The key insight that carries over to synthetic biology is that bistability does not require complex machinery — it requires mutual repression or a positive feedback loop with sufficient nonlinearity, and the λ switch achieves this with just two proteins and three operator sites.

**Multiplicity of infection (MOI)** affects the lysogenic decision: at high MOI (multiple phage per cell), the probability of lysogeny increases. Mechanistically, the CII protein (a transcriptional activator of CI from P_I_) is more stable in cells infected by multiple phage (because CII degradation is diluted relative to its production), biasing the decision toward lysogeny.

## Key Phage Biology Parameters

For quantitative modeling, phage biology requires knowing key parameters:

| Parameter | Typical value (λ, T4) | Definition |
|---|---|---|
| Latent period | 40–60 min | Time from infection to first lysis |
| Burst size (β) | 100–200 phage/cell | Progeny phage per lysed cell |
| Adsorption rate (k) | $10^{-9}$ to $10^{-8}$ mL/min | Rate of phage binding to host |
| Infection probability | ~1 at high phage:cell ratio | Fraction of cells infected |

The **one-step growth experiment** measures latent period and burst size directly: infect a culture synchronously, dilute to prevent secondary infections, and measure phage titers over time.

## Phage Families and Diversity

Phages display enormous structural diversity:

**Doudna-class tailed phages (Caudoviridae)**: ~96% of all characterized phages; dsDNA genome; three tail morphologies:
- *Myoviridae*: long contractile tails (T4, Mu)
- *Siphoviridae*: long non-contractile tails (λ, M13's far relative T5)
- *Podoviridae*: short non-contractile tails (T7)

**T4 phage**: 170 kb dsDNA, 289 genes, complex icosahedral head (~100 nm) + contractile tail + 6 long tail fibers. The tail fibers recognize the LPS O-antigen of *E. coli*. T7 phage encodes its own RNA polymerase (T7 RNAP) — a single-subunit RNAP that transcribes only from T7 promoters (PT7) at ~230 nt/s; widely used in synthetic biology as an orthogonal transcription system.

**M13 phage (Inoviridae)**: filamentous ssDNA phage (6407 nt), ~7 nm × 900 nm rod. Non-lytic — it is continuously secreted through the cell membrane without killing the host. Used extensively in **phage display**: foreign peptides are fused to the pIII coat protein and displayed on the phage surface; selections against target proteins identify tight-binding sequences (**directed evolution** of binding proteins).

## Phage-Host Co-evolution in Population Dynamics

The classic **Lotka-Volterra predator-prey model** describes phage-host dynamics:

$$\frac{dB}{dt} = rB - \phi VB$$
$$\frac{dV}{dt} = \beta \phi VB - \delta V$$

where $B$ = bacteria, $V$ = phage, $r$ = bacterial growth rate, $\phi$ = adsorption rate, $\beta$ = burst size, $\delta$ = phage decay rate. This leads to oscillating populations — bacteria and phage cycle out of phase, a pattern observed in chemostat experiments.

Real populations introduce CRISPR immunity, receptor mutation escape, and lysogeny, creating much richer dynamics. These dynamics are actively modeled with multi-strain models incorporating phage-bacteria coevolution.

## Why This Matters for Computational Biology

Lambda phage was the original proving ground for bistable switch theory and gene circuit analysis — understanding the CI/Cro system gives deep intuition for how bistability arises from positive feedback. T7 RNAP is used in cell-free transcription-translation (TX-TL) systems and in synthetic circuits that require orthogonal transcription. Phage display is a high-throughput experimental directed evolution tool — each selection round is analogous to a generation of natural selection, and the optimization of binding affinity over rounds is a direct example of navigating a fitness landscape. Phage therapy (using phages to kill antibiotic-resistant bacteria) requires models of phage-host dynamics that must account for bacterial resistance evolution (including CRISPR spacer acquisition) and phage counter-evolution. Prophage induction is a stress response that can confound transcriptomic experiments in bacteria — knowledge of which genes are phage-derived (using PhiSpy, PHASTER) allows exclusion of prophage artifacts from analysis.

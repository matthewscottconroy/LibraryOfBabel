# When to Use Agent-Based Models

## Individual Variation and Collective Behavior

Imagine treating a bacterial infection with an antibiotic. You give a dose that should — according to the average MIC — kill all the bacteria. And yet, a few cells survive. Not because they carry a resistance gene, but because a handful of them happened to be in a slow-growing, low-metabolism state at the time of treatment. Those survivors repopulate the colony. The same dose, the same genetic background, a very different outcome — because what mattered was not the average cell but the rare individual.

In many biological systems, the collective behavior of a population cannot be predicted from the average behavior of its members. When individual variation, local interactions, and spatial heterogeneity are important, **agent-based models (ABMs)** — also called individual-based models — provide the appropriate framework.

An ABM represents each individual (cell, bacterium, molecule, organism) as an autonomous agent with its own internal state and behavioral rules. Agents interact with each other and with their environment according to specified local rules. Collective patterns — tissue architecture, microbial biofilm structure, tumor morphology, immune response dynamics — emerge from these local interactions without being explicitly programmed.

## The Core Question: When Does Individual Variation Matter?

The essential criterion for choosing ABMs over population-level models (ODEs, PDEs) is whether **individual variation generates qualitatively different population behavior**.

### When Individual Variation Matters

**Rare events**: If 1 in 10,000 cells is a drug-resistant mutant, population ODE models cannot represent this minority subpopulation accurately — they predict only average behavior. An ABM can track the resistant cell and predict the probability of treatment failure from that single resistant cell.

**Spatial structure**: A population of bacteria growing in a biofilm experiences local nutrient gradients; cells at the biofilm surface may be growing rapidly while cells in the interior are dormant. A well-mixed ODE model predicts average behavior that describes neither sub-population accurately. The spatial structure itself — which requires spatial agent tracking — determines the population dynamics.

**Cell-cell heterogeneity in signaling**: In a population of T cells responding to antigen, some cells are activated and proliferate, while others undergo anergy or death. The outcome of the immune response depends on the distribution of single-cell activation states, not the average.

**Emergence of spatial patterns**: tissue morphogenesis, tumor invasion at tissue interfaces, bacterial colony pattern formation — these depend on local cell-cell and cell-matrix interactions that cannot be captured by homogeneous ODEs.

### When Population Models Are Sufficient

If the population is large and well-mixed, all individuals are essentially identical, and there is no spatial structure, ODE/PDE models are simpler, faster, and equivalent to an ABM.

**Rule of thumb**: if you need to predict single-cell outcomes (not just population averages) or if spatial patterning is qualitatively important, use an ABM.

## Biological Domains Demanding ABMs

| Biological system | Why ABM is needed | Example ABM application |
|---|---|---|
| Tumor growth | Spatial invasion, subclonal heterogeneity | Tumor spheroid growth models |
| Biofilm formation | Nutrient gradient, structural heterogeneity | iDynoMiCS bacterial biofilm |
| Tissue morphogenesis | Cell movement, contact, signaling | CompuCell3D organoid models |
| Immune cell dynamics | Stochastic encounter, individual activation | ABM of CTL-tumor interactions |
| Stem cell niche | Spatial positioning, symmetric/asymmetric division | Intestinal crypt models |
| Microbial ecology | Competition, spatial exclusion | Mesa/NetLogo community models |

## The Tradeoffs

**Advantages of ABMs**:
- Natural representation of individual variation
- Captures emergent spatial patterns
- Can incorporate arbitrary behavioral complexity per agent
- Mechanistically explicit (each rule has a biological interpretation)
- Naturally handles discrete events (cell division, death, migration)

**Disadvantages of ABMs**:
- Computationally expensive: $N$ agents each running internal models → cost scales with $N$
- Analysis is difficult: no analytical steady-state; requires many simulation runs for statistical analysis
- Parameter identification is hard: many behavioral rules, each with parameters
- Stochastic: results vary between runs; need ensemble statistics

## Choosing the Level of Agent Complexity

The appropriate internal complexity for each agent depends on the question:

**Cell as a rule-following particle**: simplest. Each cell has position, type, and simple division/death rules (e.g., "divide if local nutrient > threshold"). Appropriate for tissue morphogenesis studies where internal signaling detail is unnecessary.

**Cell with ODE internal model**: intermediate. Each agent runs its own ODE model (e.g., cell cycle model) that determines when it divides, what signals it produces, and how it responds to extracellular factors. Appropriate for questions about how intracellular signaling heterogeneity produces population-level diversity.

**Cell with FBA/ME internal model**: most detailed. Each agent's metabolism is modeled with FBA; metabolic state determines growth rate, metabolite secretion, and survival. CompuCell3D supports this through Vivarium-like integration. Appropriate for studying how metabolic heterogeneity drives ecological interactions in microbial communities.

## A Decision Framework

Before building an ABM, answer these questions:

1. **Is the number of individuals small enough to simulate individually?** For 10⁶ bacteria in a biofilm, individual-based simulation is feasible. For 10¹⁰ bacteria in a gut microbiome, it is not (without extreme computational resources).

2. **Does individual heterogeneity change qualitative predictions?** Run a simpler model first; if it fails to reproduce key observed behaviors (e.g., spatial patterning, rare event statistics), the ABM is justified.

3. **Is the ABM's added complexity testable?** ABMs have more parameters than equivalent population models. If these parameters cannot be measured or constrained by data, the additional model complexity may not improve predictive power.

4. **What computational resources are available?** A 10,000-agent ABM with ODE internal models may require ~10-100 CPU-hours per simulation; 1,000 replicate runs for statistics may require 10,000–100,000 CPU-hours.

## Why This Matters

ABMs have revealed important biological principles that population models cannot capture: the spatial structure of biofilms determines antibiotic tolerance (not just resistance) by creating protected niches; tumor growth dynamics are driven by rare cancer stem cells whose stochastic self-renewal decisions are not captured by average-population models; intestinal crypt homeostasis requires spatially regulated stem cell competition in a specific niche geometry. These insights — which directly inform drug development and regenerative medicine — were accessible only through the individual-based perspective that ABMs provide.

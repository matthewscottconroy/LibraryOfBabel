# Cathedral II: A Genetic Circuit with Predictive Design

---

## The Question

Can mathematical models of genetic parts predict the behavior of a novel genetic circuit before it is built?

---

## What You're Building

A genetic circuit (biosensor, memory device, or oscillator) designed by mathematical modeling first, built second, with systematic comparison between prediction and experiment.

---

## Prerequisites

- [Tier 0.1](../curriculum/tier-0-bedrock/0.1-mathematics.md): ODEs, bifurcation theory, Hill functions
- [Tier 0.2](../curriculum/tier-0-bedrock/0.2-chemistry.md): Enzyme kinetics
- [Tier 0.3](../curriculum/tier-0-bedrock/0.3-biology.md): Transcription, translation, promoters
- [Tier 2.1](../curriculum/tier-2-systems-biology/2.1-mathematical-modeling.md): ODE modeling, stochastic simulation, parameter estimation
- [Tier 3.1](../curriculum/tier-3-synthetic-biology/3.1-genetic-parts-devices.md): Genetic parts
- [Tier 3.2](../curriculum/tier-3-synthetic-biology/3.2-genetic-circuit-design.md): Circuit design

---

## The Project

### Phase 1: Circuit Specification

Choose a target function. Recommended entry-level projects:
- **Biosensor with analog output**: input molecule concentration → graded fluorescence output; characterize the dose-response
- **Biosensor with threshold (digital) output**: above/below a concentration threshold → HIGH/LOW output
- **Coherent FFL pulse generator**: input → transient output; characterize pulse width and delay
- **Toggle switch**: bistable memory device; switch between states with two different inducers

More advanced:
- **Oscillator**: protein concentration oscillates with measurable period
- **Band-pass filter**: output HIGH only in a defined input range
- **Multi-input logic gate**: two or three inputs define output

### Phase 2: Parts Selection and Characterization

1. Select candidate parts (promoters, RBSes, TFs, terminators) from the literature or iGEM Registry

2. Find or measure transfer function for each part:
   - Promoter: [inducer] → transcription rate (proxy: GFP fluorescence per cell per hour)
   - Repressor: [repressor] → transcription reduction (fluorescence ratio)
   - Fit Hill function: F = α·x^n/(K^n + x^n); extract α, K, n

3. Determine dynamic parameters:
   - mRNA and protein degradation rates (literature or pulse-chase experiments)
   - Protein synthesis rate per mRNA

4. Check signal matching: output concentration range of upstream part must span input sensitivity range of downstream part

### Phase 3: Mathematical Modeling

5. Write ODEs for your circuit using characterized part parameters

6. Simulate:
   - Time courses: does circuit reach expected steady states?
   - Dose-response: sweep inducer concentration; predict transfer curve of assembled circuit
   - If bistable: identify bistability region in parameter space; ensure operating point is within it
   - If oscillatory: verify Hopf bifurcation conditions; predict period

7. Sensitivity analysis:
   - Vary each parameter by ±50%; how much does output change?
   - Identify which parameters the circuit is most sensitive to
   - These are the parameters to measure most carefully in experiment

8. Stochastic simulation (Gillespie):
   - Predict cell-to-cell variability (coefficient of variation)
   - Are steady states robust to noise?

9. Write quantitative predictions:
   - Expected fold-change in output between LOW and HIGH states
   - EC50 of inducer for half-maximal response
   - Expected time constant for switching
   - Expected CV (cell-to-cell variability)

### Phase 4: Circuit Construction

10. DNA assembly:
    - Design construct in silico: promoter + RBS + TF gene + terminator
    - Verify reading frames, no internal restriction sites (if using traditional cloning)
    - Gibson Assembly or Golden Gate for multi-part assembly
    - Verify sequence by Sanger sequencing

11. Transformation and colony selection

12. Measure part behaviors in the assembled construct context:
    - Parts may behave differently in new genetic context (context effects)
    - Document: does each individual component match characterized parameters?

### Phase 5: Circuit Testing

13. Measure circuit behavior:
    - Dose-response: vary inducer concentration → measure output (flow cytometry for distributions)
    - Time course: add inducer at t=0; measure output every 30 min for 8 hours
    - If bistable: induce switching with each stimulus; verify bistability by checking memory

14. Compare systematically to model predictions:
    - Overlay experimental data on model predictions in same plots
    - Calculate: discrepancy at each inducer concentration
    - Statistics: is discrepancy within predicted noise range or beyond?

### Phase 6: Model Refinement

15. Identify sources of discrepancy:
    - Context effects: RBS or promoter changed by neighboring sequence?
    - Retroactivity: downstream circuit loading upstream behavior?
    - Metabolic burden: circuit expression slowing growth → changing part parameters?
    - Missing biology: mRNA secondary structure? TF dimerization not included?

16. Update model to include identified mechanisms; re-simulate

17. Report: what did the model predict correctly? What did it miss? Why?

---

## Expected Output

- GitHub repository: all ODE models, stochastic simulations, experimental data, analysis code
- Figures: model predictions vs. experimental data, aligned and overlaid
- A clear account of prediction accuracy and identified failure modes
- Updated model that explains discrepancies

---

## Why This Cathedral Matters

Most synthetic biology papers report a circuit that works without documenting:
- What was predicted before building
- How accurate the prediction was
- Why prediction failed (when it did)

Systematic comparison of prediction to experiment is what advances the field from demonstration to engineering.

---

## Key Tools

- SciPy solve_ivp or tellurium: ODE simulation
- StochPy or BioSimulator.jl: Gillespie algorithm
- COPASI: parameter estimation from dose-response data
- scipy.optimize: Hill function fitting
- CELLO (optional): automated circuit design comparison
- FlowJo, FCSalyzer, or Python fcsparser: flow cytometry analysis

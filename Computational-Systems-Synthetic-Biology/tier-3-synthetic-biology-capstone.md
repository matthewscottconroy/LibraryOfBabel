# Tier 3 Capstone: Synthetic Biology Integration Project

## "Design, Characterise, and Optimise a Synthetic Genetic Circuit for a Defined Biological Application"

---

## Overview

The Tier 3 capstone is the most open-ended project in the curriculum: you will design a synthetic genetic circuit for a defined biological application, characterise it computationally, predict its performance, and develop a plan for experimental implementation and iterative improvement. The project integrates all of Tier 3's tools — genetic part design, circuit logic, CRISPR-based regulation, metabolic engineering considerations, and biosafety analysis — into a coherent engineering workflow.

This capstone is intentionally challenging and intentionally realistic: the problems encountered here (circuit orthogonality, metabolic burden, evolutionary stability, biosafety) are the problems that define the frontier of synthetic biology.

---

## Biological Motivation

Synthetic biology has moved from proof-of-concept circuits (the toggle switch, the repressilator) to circuits designed for real applications: biosensors for environmental monitoring, genetic circuits controlling CAR-T cell activation in cancer therapy, metabolic engineering for biofuel production, and diagnostic circuits for pathogen detection. The field's central challenge is predictability: how do we design a circuit that performs as intended in a complex cellular context?

This capstone provides a structured framework for confronting that challenge computationally, before committing to wet-lab work.

---

## Application Options

Choose **one** application from the following, or propose your own (approval required):

**Option A: Cell-density-dependent gene expression circuit (biosensor application).** Design a synthetic quorum sensing circuit that activates a reporter gene only when cell density exceeds a threshold, and turns off a "burden" gene (a metabolically expensive protein) below the threshold. This implements density-dependent resource allocation.

**Option B: Synthetic kill switch for biosafety.** Design a two-input kill switch that causes cell death when (condition 1) the cell escapes a defined containment zone (detected by the absence of a chemical inducer added to growth medium) AND (condition 2) the circuit has been active for more than a defined time period. This requires a logic gate AND a timer circuit.

**Option C: Metabolic pathway controller.** Design a genetic circuit that senses the accumulation of a toxic metabolic intermediate and upregulates the next enzyme in the pathway (push) while downregulating competing branch pathways (pull), maintaining the intermediate at a low, defined concentration.

**Option D: CRISPR-based temporal control circuit.** Design a circuit in which CRISPR-dCas9 (dead Cas9, no nuclease activity) acts as a programmable transcription factor. The circuit should activate two genes in sequence (A then B) rather than simultaneously, using a timer based on transcriptional delay.

---

## Project Components

### Component 1: Circuit Design and Part Characterisation (Weeks 1–3)

**Tasks:**
- Specify the biological function your circuit must perform. Define success criteria quantitatively (e.g., "reporter expression ≥ 10-fold above background at cell density ≥ $10^8$ cells/mL, with ≤ 2-fold variation across the ON state").
- Draw a complete circuit diagram showing all components: promoters, regulatory proteins, output genes, feedback elements.
- Select specific biological parts for each component using the iGEM Registry or Addgene:
  - Promoters: specify strength (in RPU relative to a standard promoter), inducibility, and regulatory logic.
  - Ribosome binding sites (RBS): specify translation initiation rate (Salis lab RBS Calculator).
  - Coding sequences: specify the proteins, their $K_d$ values for relevant interactions, and any required post-translational modifications.
  - Terminators: specify termination efficiency.
- Calculate the predicted expression level of each component from the part characteristics.
- Assess orthogonality: does each part interact with other components of your circuit, or with the host's regulatory network, in unexpected ways? Consult published orthogonality data.

**Deliverable:** Circuit diagram, parts list with quantitative specifications, orthogonality assessment.

### Component 2: Mathematical Modelling and Performance Prediction (Weeks 4–6)

**Tasks:**
- Write a system of ODEs for all species in your circuit (mRNAs, proteins, inducer molecules, CRISPR complexes if applicable).
- Parameterise the ODEs using the part characteristics from Component 1.
- Simulate the steady-state response of the circuit to its inputs. Plot dose-response curves, time-courses, and (for bistable or oscillatory circuits) phase portraits.
- Perform a bifurcation analysis: identify the parameter values at which the circuit changes qualitative behaviour (e.g., the threshold density for quorum sensing activation).
- Perform a robustness analysis: how sensitive is the circuit performance to parameter variation? Which parts are the "critical" parameters that must be tightly controlled?
- If your circuit involves stochastic switching (e.g., a bistable switch), implement a Gillespie simulation and characterise the switching kinetics.
- Estimate the metabolic burden of your circuit: total amino acid consumption rate for all expressed proteins. Compare with typical burdens reported in the literature (≤ 10% of total protein production budget is generally considered acceptable).

**Deliverable:** ODE system (code), dose-response/time-course figures, bifurcation/robustness analysis, metabolic burden estimate.

### Component 3: CRISPR Component Design (if applicable) (Week 7)

*Complete this component only if your circuit uses CRISPR-dCas9, CRISPR-Cas12a, or base editing.*

**Tasks:**
- Design your guide RNAs (gRNAs) using an online gRNA design tool (CRISPick, CHOPCHOP, or equivalent).
- Predict off-target binding sites in the host genome using CRISPR-specific off-target prediction tools.
- Assess whether any off-target sites overlap with essential genes.
- Design the dCas9 fusion protein's regulatory domain (activation domain, repression domain, or epigenetic modifier) and specify the expected fold-change in target gene expression.

**Deliverable:** gRNA sequences, off-target analysis, expected regulatory effect.

### Component 4: Evolutionary Stability Analysis (Week 8)

Synthetic circuits impose metabolic burden, and cells that lose circuit function (through mutation or gene loss) grow faster than circuit-expressing cells. This creates an evolutionary pressure to "break" the circuit. This component requires you to assess the evolutionary stability of your design.

**Tasks:**
- Identify the most likely escape mutations: which mutations in your circuit would eliminate its function while reducing metabolic burden? (These are typically mutations that create premature stop codons in key regulatory proteins, or that inactivate the promoter driving the circuit.)
- Estimate the fitness cost of your circuit (using your metabolic burden estimate and typical cost-of-expression data from Kafri et al. 2016).
- Model the evolutionary dynamics of circuit loss: using a simple two-population model ($N_{circuit}$ and $N_{no-circuit}$, with growth rates $r$ and $r + \Delta r$ respectively), compute the time for the circuit-bearing population to drop below 50% in a non-selective environment.
- Propose at least two design strategies to improve evolutionary stability (e.g., coupling circuit function to an essential gene, using auxotrophic containment, reducing burden through protein degradation tags).

**Deliverable:** Escape mutation analysis, evolutionary dynamics model, stability improvement proposals.

### Component 5: Biosafety and Regulatory Analysis (Week 9)

**Tasks:**
- Classify your organism and your genetic modifications under relevant biosafety frameworks (NIH Guidelines for Research Involving Recombinant DNA Molecules; EU Directive 2009/41/EC if applicable).
- Identify any biosafety concerns: does your circuit confer an environmental fitness advantage? Does it encode proteins of concern (antibiotic resistance, toxins)?
- If your application involves release (e.g., a biosensor for environmental deployment), specify the containment strategy: kill switch, auxotrophic dependence, orthogonal codon code, or combination thereof.
- Write a biosafety summary in the format required by an institutional biosafety committee (IBC), including: organism, modifications, hazard assessment, containment strategy, emergency response plan.

**Deliverable:** IBC-format biosafety summary (1–2 pages).

### Component 6: Experimental Implementation Plan (Week 10)

**Tasks:**
- Specify the complete experimental plan for building and testing your circuit:
  - **Cloning strategy**: list the primers, PCR amplifications, restriction digests, ligations or Golden Gate/Gibson assembly steps required.
  - **Testing and characterisation order**: which individual parts do you characterise first (characterise each promoter and RBS independently before combining into a circuit)?
  - **Controls**: list all positive and negative controls.
  - **Measurement plan**: flow cytometry for population-level fluorescence, Western blot for protein quantification, qRT-PCR for mRNA quantification, or plate-reader fluorescence?
  - **Timeline**: estimate the number of weeks required to build, assemble, and characterise the circuit.

**Deliverable:** Complete experimental plan (2 pages), including a Gantt chart.

---

## Assessment Rubric

| Criterion | Weight | Excellent | Proficient | Developing |
|-----------|--------|-----------|------------|------------|
| Circuit design logic | 20% | Circuit correctly implements the specified function; part choices justified quantitatively | Circuit mostly correct; some part choices unjustified | Circuit design flaws that would prevent function |
| Mathematical modelling | 25% | ODE system complete and parameterised; bifurcation and robustness analysis rigorous | ODE correct but analysis shallow | ODE errors or incomplete |
| Evolutionary stability | 15% | Escape mutations identified, evolutionary dynamics modelled, stability strategies proposed | Basic evolutionary analysis | No evolutionary stability analysis |
| Biosafety analysis | 15% | Thorough IBC-format analysis; containment strategy appropriate | Basic biosafety assessment | No biosafety analysis |
| Experimental plan | 15% | Realistic, complete plan with timeline and controls | Plan present but incomplete or unrealistic | No experimental plan |
| Scientific writing | 10% | Clear, precise, figures well-captioned | Adequate | Unclear |

---

## Extension Challenges

**Extension A: Directed evolution.** Propose a directed evolution strategy to improve one property of your circuit (e.g., increase the dynamic range of your sensor, reduce the OFF-state expression). Specify the selection scheme, the library construction method, and the expected number of rounds required.

**Extension B: Cell-free implementation.** Adapt your circuit for a cell-free expression system (TX-TL, from Noireaux lab). What changes to the design are required? What are the advantages and limitations of cell-free vs. in-cell implementation for your application?

**Extension C: Multi-organism system.** Extend your circuit to a two-organism system: split the circuit between two organisms that communicate via a defined chemical signal. Design the communication channel (signalling molecule, biosynthesis pathway, receptor) and model the coupled system.

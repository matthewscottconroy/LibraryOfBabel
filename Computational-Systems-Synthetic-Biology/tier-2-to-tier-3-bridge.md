# Bridge: Tier 2 (Systems Biology) → Tier 3 (Synthetic Biology)

## What You Carry Forward from Tier 2

Tier 2 taught you to understand biological systems quantitatively: to model their dynamics, analyse their network topology, and predict their responses to perturbation. Tier 3 takes everything you learned and puts it in the service of *design*: creating new biological systems that don't exist in nature, or re-engineering existing ones to perform specific functions.

The transition from analysis to design is not merely a change in activity — it is a change in intellectual stance. In Tier 2, the biological system is given; you are trying to understand it. In Tier 3, the biological system is chosen or created; you are responsible for its behaviour.

### From Mathematical Modelling (ODE and Boolean)
- In Tier 3, every circuit you design should be modelled before it is built. The ODE modelling skills from Tier 2 are your primary design tool: you will use them to predict switch points, oscillation periods, response times, and dynamic ranges before committing to a cloning strategy.
- Boolean network models from Tier 2 are useful for the early, logical stages of circuit design: does the circuit implement the desired logic? Before worrying about quantitative parameters, verify the logical architecture.
- **What is new in Tier 3**: you are not given the parameters — you choose them. Characterising the quantitative properties of specific genetic parts (promoter strengths, binding affinities, Hill coefficients) and connecting them to your model is the key challenge.

### From Flux Balance Analysis
- FBA from Tier 2 is directly operational in Tier 3 for metabolic engineering: you will use FBA to predict whether your target metabolite can be produced at high flux, which competing pathways to knock out, and what the theoretical yield ceiling is. CoBARApy is your tool.
- FBA also reveals potential burdens: if your synthetic circuit diverts significant carbon or nitrogen flux, FBA can estimate the growth cost.

### From Network Analysis
- The network motif analysis of Tier 2 is the vocabulary of synthetic circuit design in Tier 3. Feedforward loops (as pulse generators), toggle switches (as memory elements), and negative feedback (as noise reducers) are the building blocks. You know their mathematical properties from Tier 2; Tier 3 teaches you to implement them with specific genetic parts.
- The concept of "modularity" from Tier 2 (the idea that networks can be decomposed into semi-autonomous functional modules) is the central design principle of synthetic biology. Tier 3 makes this concrete: how do you design genetic modules that behave predictably in isolation AND in combination?

---

## The Conceptual Leap Being Made

Tier 2 answered the question: "Given this biological system and these interactions, what behaviour does it produce?" Tier 3 inverts the question: "Given the behaviour I want, what interactions and parts do I need?"

This inversion is harder than it sounds. In analysis, you start from something real and work toward understanding. In design, you start from a specification (what the system should do) and work toward something real (a DNA sequence, a set of protein interactions, a metabolic pathway). The space of possible designs is vast; the space of designs that actually work is small.

**The abstraction-implementation gap.** Every synthetic biology design involves a gap between the abstraction level (a circuit diagram with boxes and arrows) and the implementation level (specific DNA sequences, protein-protein interactions, cellular context). Tier 2 works mostly at the abstraction level; Tier 3 confronts the implementation level. The behaviours predicted by circuit diagrams often fail when implemented, because:
- Parts don't have the exactly properties assumed
- Parts interact with the host cell (load effects, resource competition, off-target regulation)
- Evolutionary pressure selects against circuit function
- Stochastic noise is larger than models predicted

**Failure is information.** In Tier 3, most circuits fail on the first attempt. The skill is not to avoid failure but to learn from it: to identify which assumption in the model was wrong, to redesign accordingly, and to iterate. The Design-Build-Test-Learn (DBTL) cycle is the fundamental workflow of synthetic biology.

**Context dependence.** A genetic part characterised in one organism and condition may behave very differently in another. Tier 3 teaches context-sensitive design: the importance of using parts characterised in your chassis, accounting for growth phase, medium composition, and genetic background.

---

## Self-Assessment Questions

**From Tier 2 — ensuring readiness:**
1. You have an ODE model of a toggle switch and you have identified the bistable parameter region by finding the saddle-node bifurcations. How would you use this model to *design* a toggle switch with a specific switch point? What parameters would you tune, and how would you tune them experimentally?
2. You have performed FBA on *E. coli* iJO1366 and identified that acetyl-CoA is a key metabolic hub. You want to increase flux toward your target metabolite (which branches off from acetyl-CoA). Which reactions would you knock out? Which enzymes would you overexpress? How would FBA guide these choices?
3. Describe the incoherent type 1 feedforward loop (IFFL-1) and explain what computational function it performs. How would you implement it with specific transcription factors?

**Synthetic biology readiness:**
4. What is a BioBrick? What is the Golden Gate assembly method? When would you use each?
5. What is orthogonality in the context of genetic circuits? Give two examples of orthogonal components.
6. What is metabolic burden, and how does it affect circuit performance? Describe two strategies to reduce burden.

**Practical readiness:**
7. You have designed a circuit using two transcription factors: TetR and LacI. Describe the complete molecular biology experiment to: (a) clone your circuit into a plasmid, (b) verify the sequence, (c) transform into *E. coli*, and (d) measure GFP expression under induction conditions.
8. What is the difference between constitutive and inducible promoters? When would you use each in circuit design?
9. What is the role of degradation tags (e.g., ssrA tags) in synthetic circuit design?

---

## Recommended Review if You Feel Shaky

| Topic | Review resource | Time estimate |
|-------|-----------------|---------------|
| Synthetic biology principles | Brophy & Voigt (2014), *Nature Methods* "Principles of genetic circuit design" | 2 days |
| Genetic parts characterisation | Salis (2011), *Nature Biotechnology*, "The ribosome binding site calculator" | 1 day |
| Cloning methods | NEB Cloning guide + Golden Gate tutorial | 3 days |
| Metabolic engineering | Lee et al. (2011), *Nature Chemical Biology* review | 3 days |
| CRISPR design | Addgene CRISPR Guide (free online) | 1 week |

---

## What Tier 3 Demands That Tier 2 Did Not

**Wet-lab thinking.** Even if you are primarily a computational person, Tier 3 requires you to think about experimental implementation at every stage of design. A circuit that is beautiful on paper but impossible to build (because the required parts don't exist, or because the assembly is infeasible) is not a good circuit design. This demands familiarity with the practical constraints of molecular biology.

**Biological engineering judgement.** Which chassis organism? Which selection marker? Which promoter system? These choices are not determined by equations — they require knowledge of the practical biology of your organism, your application, and your laboratory constraints. This judgement is built by reading case studies (the original toggle switch, repressilator, and subsequent improvements) and by understanding what went wrong in each implementation and why.

**Ethical and biosafety awareness.** Tier 3 is the point at which the work you are doing could, in principle, have environmental consequences. Engineered organisms that escape containment, circuits that confer antibiotic resistance or pathogenic properties, and synthetic pathways that produce toxic chemicals all require thoughtful biosafety analysis. This is not a formality — it is a genuine intellectual responsibility that Tier 3 trains you to take seriously.

**Patience with the design cycle.** The Design-Build-Test-Learn cycle typically requires 3–6 months per design iteration in a well-resourced laboratory. Computational pre-screening (using the models from Tier 2) can dramatically reduce the number of experimental iterations required. The bridge into Tier 3 is therefore also a bridge into professional scientific practice: the patience, rigour, and creativity required to design something that actually works in a living cell.

# Section 2: The Mechanism Problem

## From Description to Explanation

A perennial tension in science is between description and explanation. Description tells us what happens; explanation tells us why and how. The history of science is full of phenomena that were described long before they were explained: the periodic table was a description of chemical regularities before it was an explanation of atomic structure; the correlation between altitude and boiling point was known before the kinetic theory of gases explained it.

Basal cognition research is currently more descriptive than explanatory. We have documented adaptive behavior in non-neural organisms across many taxa and ecological contexts. We have characterized these behaviors using the vocabulary of cognition — sensing, integration, memory, decision-making. We have begun to identify candidate molecular mechanisms — bioelectric signals, chemical gradients, cytoskeletal dynamics — that might underlie these behaviors. But we do not have satisfactory mechanistic explanations for most of what we observe. We do not know, in the detail required for genuine scientific understanding, how the slime mold's cytoplasmic oscillations implement the spatial optimization of its network, or how the plant's electrical signals propagate across the leaf and how those signals translate into behavioral responses, or how the bacterial quorum sensing circuit encodes collective decisions.

The mechanism problem is not one problem but several, organized around a central question: **what physical processes implement basal cognition?**

## Bioelectricity, Chemistry, and Mechanics: Which Substrate Matters?

Three candidate substrates for basal cognition have received the most attention: bioelectric signals, chemical signaling, and mechanical forces. Understanding which is most important — or whether there is a substrate-independent logic that can be implemented in any of them — is a central open question.

**Bioelectricity** is the substrate that has received the most theoretical attention from Michael Levin and colleagues. Bioelectric patterns — the spatial distribution of membrane voltage across a tissue, and the dynamics of that distribution over time — carry information about the state of the tissue relative to its target morphology, guide cell behavior through the regulation of ion channel expression and activity, and can propagate information across distances too large for diffusion-based signaling to cover quickly. The evidence that bioelectric signals are genuinely informational — that their pattern encodes something about the state of the system, not just that a signal is present — is strongest in the developmental biology context (Levin, 2021).

But bioelectricity is not the only player. **Chemical gradients** — of hormones, metabolites, reactive oxygen species, and small signaling molecules — convey information over a range of spatial and temporal scales, from the millisecond signaling of neurotransmitters to the days-long dynamics of plant hormonal gradients. Chemical signaling is the most ancient form of intercellular communication and is present in every organism that has been studied. Its information-carrying capacity is high — the diversity of chemical signals, each with its own specificity and kinetics, allows the encoding of complex spatial and temporal patterns.

**Mechanical forces** have received less attention as cognitive substrates but are increasingly recognized as important. The mechanical properties of the extracellular matrix guide cell migration, the stiffness of the substrate influences cell fate decisions, the tension in the cytoskeleton regulates gene expression, and the flow of cytoplasm through fungal hyphae and plant phloem carries not only metabolites but signals whose timing encodes information about the state of distant parts of the network. Mechanosensing — the detection of physical force — is evolutionarily ancient and present in every domain of life.

The honest answer to "which substrate matters?" is probably "all of them, in ways that depend on the system and the behavior." Different organisms may implement similar cognitive functions in different substrates — as different computing technologies can implement the same algorithm in silicon, optical fiber, or fluid pressure. If so, the important question is not which substrate but what computational principles are substrate-independent — what features of the information processing are preserved across different physical implementations.

## Are There Universal Mechanisms?

The search for universal mechanisms of cognition — mechanisms that are present and functional across the full diversity of cognitively active biological systems — is one of the most ambitious programs in basal cognition research.

Several candidate universal mechanisms have been proposed:

**Feedback control.** Every organism that exhibits homeostasis — maintenance of an internal state against external perturbation — implements feedback control: a sensor detects the deviation of some variable from a set point, and an actuator acts to reduce the deviation. This is mechanistically present from the simplest single-celled organisms (using membrane receptors and signal transduction to maintain osmotic balance) to the most sophisticated neural systems (using complex cortical circuits to maintain body temperature, blood glucose, and arterial pressure). Whether the homeostatic set point constitutes a "goal" in any cognitively interesting sense is debated, but the mechanistic structure is genuinely universal.

**Oscillatory dynamics.** Oscillatory processes — periodic fluctuations in molecular concentrations, membrane voltage, cytoplasmic flow, or population behavior — are present throughout biology at every temporal scale, from microsecond protein conformational changes to decades-long ecological cycles. In the context of cognition, oscillations often serve as timing mechanisms, phase relationships between oscillators encode information, and the entrainment of oscillators to external rhythms implements a form of anticipation. The prevalence of oscillatory dynamics across biological systems suggests that oscillatory computation may be a universal feature of biological information processing.

**Bistability and hysteresis.** Many biological systems exhibit bistability: two stable states between which the system can switch when pushed across a threshold, retaining the new state after the pushing force is removed. This is a form of memory — the system "remembers" which state it was pushed into, even after the perturbation has passed. Bistable systems are found from gene regulatory networks (toggle switches, as in Chapter 35) to cellular decision-making (the differentiation commitment of stem cells) to ecological systems (alternative stable states). Bistability may be a universal mechanism for memory and irreversible decision-making in biological systems.

**Diffusion-reaction dynamics.** The interaction of activating and inhibiting processes with different diffusion rates — Turing's reaction-diffusion framework — generates spatial patterns across biological systems from bacterial populations to developing embryos. This mechanism is not universal in the sense of being present in all organisms, but it may be universally available — realizable wherever there are coupled chemical reactions with differential diffusion — and may therefore represent a computational tool that evolution has repeatedly deployed across very different biological contexts.

## The Gap Between Models and Reality

A persistent frustration in basal cognition research is the gap between the mathematical models used to describe cognitive behaviors and the biological mechanisms that implement them. This gap is not unique to basal cognition — it is present throughout biology — but it is particularly acute in a field where the mechanisms are poorly understood and the models are often developed to describe behavior without specifying implementation.

Consider the slime mold example. The optimization of the *Physarum* transport network is well-described by a mathematical model in which the diameter of a tube increases as the flux through it increases, and decreases otherwise — a positive feedback that reinforces high-flux connections and eliminates low-flux ones (Tero et al., 2010). This model accurately predicts the network topology that *Physarum* builds and explains why that topology has good properties. But the model is not a mechanistic account. It does not specify which molecular players implement the flux-dependent tube diameter regulation, whether the regulation operates through cytoskeletal dynamics, biochemical signaling, or mechanical feedback, or what the quantitative parameter values are in the biological system.

The gap between the descriptive model and the mechanistic implementation is large, and filling it is a major research program. Candidate mechanisms for *Physarum* network optimization include: peristaltic cytoplasmic flow, generated by rhythmic contraction of the actomyosin cytoskeleton, whose flow is modulated by the chemical signals (such as adenosine) released at feeding sites; the mechanical adaptation of tube walls to flow stress; and the chemical signaling mediated by the oscillatory calcium dynamics that propagate through the network. Each of these has some experimental support; none is fully established.

This is the state of the field: rich descriptive models of cognitive behaviors, incomplete and contested mechanistic accounts, and a large body of experimental work needed to connect them.

## Why the Mechanism Problem Matters

One might ask: if the behaviors are real and the mathematical models accurately predict them, why does the mechanistic explanation matter?

It matters for several reasons. First, without mechanistic understanding, we cannot evaluate the generality of the phenomena: are the same mechanisms operating in different organisms, or are superficially similar behaviors implemented in completely different ways? Second, without mechanism, we cannot design interventions: if we want to manipulate, disrupt, or restore basal cognitive functions — for medical, agricultural, or AI applications — we need to know what we are targeting. Third, without mechanism, we cannot connect basal cognition to the broader physics of living systems: we cannot say how much energy the computation requires, how reliable it is, what its limits are, or how it relates to the thermodynamics of non-equilibrium systems.

And fourth — perhaps most important from a scientific standpoint — the mechanism is where the interesting surprises are likely to be. When we understand the physical process underlying a cognitive behavior, we often find that it implements the computation in a way we did not anticipate, exploiting physical regularities in unexpected ways. The discovery that bacterial chemotaxis is implemented by temporal differentiation of chemical concentration — not by direct gradient sensing — was not predicted from the behavioral description alone. The mechanism was the discovery.

---

## References

Levin, M. (2021). Bioelectric signaling: Reprogrammable circuits underlying embryogenesis, regeneration, and cancer. *Cell*, 184(8), 1971–1989.

Tero, A., Takagi, S., Saigusa, T., Ito, K., Bebber, D. P., Fricker, M. D., ... & Nakagaki, T. (2010). Rules for biologically inspired adaptive network design. *Science*, 327(5964), 439–442.

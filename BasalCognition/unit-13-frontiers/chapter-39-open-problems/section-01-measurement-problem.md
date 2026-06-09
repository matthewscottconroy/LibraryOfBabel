# Section 1: The Measurement Problem

## What We're Measuring and Why It's Hard

Every scientific field needs ways to measure what it studies. Astronomy needs ways to measure distance, luminosity, and spectral composition. Genetics needs ways to measure genotype and phenotype. Neuroscience needs ways to measure neural activity — and it has developed extraordinary tools for doing so: patch-clamp electrophysiology, two-photon calcium imaging, fMRI, single-unit recording. These tools, applied to nervous systems that were already recognized as the substrate of cognition, produce measurements that can be interpreted within established theoretical frameworks.

Basal cognition research lacks an equivalent toolkit. We are trying to measure cognitive function in organisms whose cognitive mechanisms are largely unknown, using criteria for cognition that are themselves contested, without established frameworks for interpreting the measurements we make. This is not a complaint; it is a description of what makes the field scientifically exciting and methodologically challenging.

The measurement problem has three distinct dimensions: the problem of criteria (what counts as evidence of cognition?), the problem of baselines (against what should we compare the behavior of non-neural systems?), and the problem of metrics (how do we quantify cognitive function in ways that are comparable across very different biological systems?).

## The Criteria Problem

What counts as evidence of cognition in a non-neural system? The question cannot be answered by simply listing what neurons do and then checking whether the system does those things — that approach would guarantee that only neural systems would ever qualify. But it also cannot be answered by such a broad criterion that everything qualifies — a pH meter "responds to" acidity, but we do not take this as evidence of cognition.

Several criteria have been proposed in the literature, and they capture different aspects of what we intuitively mean by cognition:

**Behavioral flexibility and context-dependence.** A cognitive system responds differently in different contexts — it does not have a fixed stimulus-response mapping but adjusts its response based on the relationship between the current stimulus and previous experience, other concurrent stimuli, or the current internal state. This is sometimes operationalized as the ability to pass "response reversal" tests — where the organism must unlearn a previously reinforced association after the contingency is reversed — or as the demonstration of "context generalization," where a response acquired in one context transfers to structurally similar but perceptually different contexts.

**Memory beyond the stimulus duration.** A cognitive system retains information about past states that influences its future behavior after the stimulus that caused the state has disappeared. This rules out systems that maintain their response only while the stimulus is present, and is a minimal criterion that distinguishes cognition from mere reflex. Demonstrating this requires careful experimental control to rule out local chemical persistence (a residual chemical signal) as an alternative explanation for the observed persistence.

**Anticipation and temporal integration.** A cognitive system uses its current state to predict future states and modulates its behavior accordingly — it acts in the present based on a model of what will happen next. The anticipation of periodic stimuli by *Physarum polycephalum* (Saigusa et al., 2008), and the temporal derivative computation in bacterial chemotaxis (computing whether the chemical environment is improving or worsening rather than just what it is currently), are examples of this criterion being met.

**Goal-directedness.** A cognitive system maintains a behavior that converges toward a particular outcome across different environmental perturbations — it is "trying" to achieve something rather than just producing output. Goal-directedness is both the most intuitive criterion for cognition and the most philosophically contested, because it is not clear how to distinguish genuine teleology (the behavior is organized with respect to a future state) from apparent teleology (the behavior happens to converge on a predictable outcome by mechanical necessity).

**Non-linear integration.** A cognitive system integrates multiple information sources in ways that are not simply additive — it combines inputs in ways that implement logical operations (AND, OR, NOT), nonlinear weighting, or context-dependent gain. This distinguishes "cognition" from simple summation and is the basis for claims that signaling networks in cells implement computation rather than mere transduction.

Each of these criteria captures something real about cognitive function, and none is individually sufficient. A system that meets one criterion but not the others is not obviously cognitive; a system that meets all five is a much stronger candidate. The challenge is that different non-neural systems meet different subsets of these criteria, and there is no consensus on how to weight them.

## The Anthropocentric Baseline Problem

Perhaps the deepest difficulty in measuring non-neural cognition is the problem of anthropocentric baselines. Our intuitive judgments about what counts as cognitive behavior are calibrated against human experience and human cognitive architecture. When we observe behavior in a non-neural system, we are comparing it — implicitly or explicitly — to what a human cognitive system would do. This comparison is almost always misleading.

Consider the anticipation experiments with *Physarum*. Saigusa et al. (2008) showed that slime molds trained to expect a temperature drop every 30 minutes began to reduce their movement speed in anticipation of the expected drop — even when the drop was omitted. This looks like anticipation to us because it resembles what a human would do in the same situation. But the criterion we should apply is not "does this resemble human cognition?" but "is this consistent with an information-processing process that generates representations of future states?"

The anthropocentric baseline problem also manifests in the operationalization of learning. Classical and operant conditioning paradigms were developed for vertebrate nervous systems, with specific assumptions about the timescale of association, the nature of reinforcement, and the behavioral repertoire being modified. Applying these paradigms to plants or slime molds without modification may miss genuine forms of learning that operate on different timescales or through different mechanisms, while falsely detecting "learning" in systems where what appears is actually a simple chemical persistence.

Pamela Lyon (2006) has argued for a "biogenic approach" to cognition: starting from the cognitive capacities of simple organisms and building up to more complex ones, rather than starting from human cognition and asking which organisms approach it. This approach avoids the anthropocentric baseline problem by not assuming that human cognitive capacities are the reference point. Instead, it asks: what is the minimal set of information-processing capacities that are necessary and sufficient for adaptive behavior in a given ecological context? This question has different answers for different organisms in different environments, and it does not presuppose that the answer for *E. coli* should look like the answer for humans.

## Information-Theoretic Approaches

One principled approach to measuring cognitive function that avoids some of the anthropocentric baseline problems is information theory. Information-theoretic measures can quantify the amount of information a system's behavior conveys about its environment, the degree to which a system's response to one stimulus is influenced by its history of previous stimuli, and the complexity of the internal states underlying behavioral responses — all without presupposing any particular cognitive architecture.

Several information-theoretic measures have been applied to biological systems at various levels of complexity:

**Mutual information.** The mutual information between a system's environmental inputs and its behavioral outputs measures how much information about the environment is captured in the behavior. A system with zero mutual information between inputs and outputs is not responding to its environment; a system with high mutual information is extracting and using environmental information effectively.

**Transfer entropy.** Transfer entropy measures the directed statistical dependence between two variables over time — how much knowing the past of variable X reduces uncertainty about the future of variable Y. In the context of biological networks, transfer entropy can detect directed information flow from sensory inputs to behavioral outputs, and can characterize how that flow is modulated by internal state or history (Schreiber, 2000).

**Integrated information (phi).** As discussed in Chapter 36, Tononi's integrated information theory proposes phi — the degree to which a system's causal structure exceeds the sum of its parts' causal structures — as a measure of consciousness. Whether or not phi is the right measure of consciousness, it is a principled measure of integrated information processing that can be applied to any system with a causal structure.

**Predictive information.** The predictive information of a system — the mutual information between its past and its future — measures how well the system's current state compresses information about its history in a way that is relevant for future behavior. Systems with high predictive information maintain internal representations of their past that are informative about their future — a hallmark of cognitive function.

These measures do not replace the need for behavioral criteria; they complement them. A system that meets behavioral criteria for cognition should also, if our understanding is correct, show the appropriate information-theoretic signatures. Divergence between behavioral and information-theoretic measures is itself scientifically informative — it may indicate that the behavioral criterion was being met for non-cognitive reasons, or that the information-theoretic measure is not capturing what the behavioral criterion is tracking.

## Developing New Metrics

The measurement problem in basal cognition will not be solved by adopting existing metrics from neuroscience or cognitive psychology without modification. It will require the development of genuinely new metrics — ones that capture the distinctive features of non-neural cognition and that can be applied comparably across very different biological systems.

What might such metrics look like? Some candidates:

**Adaptive compression.** How efficiently does the system's behavioral repertoire compress information about its environment, relative to the complexity of that environment? A system that maintains a rich behavioral repertoire in a variable environment, while collapsing to simpler responses in a stable environment, is performing adaptive compression — adjusting the complexity of its information processing to match the complexity of the task.

**Counterfactual sensitivity.** How sensitive is the system's behavior to counterfactual environmental conditions — conditions it is not currently experiencing but could experience? A system that responds appropriately to hypothetical scenarios (e.g., prepares for winter before winter arrives) is displaying a form of anticipatory computation that exceeds simple reactive processing.

**Error correction.** How well does the system detect and correct deviations from a target state? The capacity to detect error — to have an internal representation of where one is relative to where one should be — is a fundamental feature of goal-directed cognition, and it can be measured by systematically introducing deviations and observing the corrective response.

The development of these metrics is a genuinely open research problem. It is also a tractable one — the mathematical frameworks exist; what remains is the biological and experimental work of applying them to the right systems. This is the kind of problem that should appeal to students comfortable working across the boundaries of biology, physics, and mathematics.

---

## References

Lyon, P. (2006). The biogenic approach to cognition. *Cognitive Processing*, 7(1), 11–29.

Saigusa, T., Tero, A., Nakagaki, T., & Kuramoto, Y. (2008). Amoebae anticipate periodic events. *Physical Review Letters*, 100(1), 018101.

Schreiber, T. (2000). Measuring information transfer. *Physical Review Letters*, 85(2), 461–464.

Tononi, G. (2004). An information integration theory of consciousness. *BMC Neuroscience*, 5, 42.

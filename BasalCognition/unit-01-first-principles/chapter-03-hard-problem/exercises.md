# Chapter 3 Exercises: The Hard Problem and Its Biological Shadow

---

## Part I: Reflection and Discussion

**1.** Chalmers argues there is an "explanatory gap" between functional descriptions of consciousness and phenomenal experience itself. Daniel Dennett disputes this, arguing that Mary doesn't actually learn anything new when she leaves the room — she just gains a new *ability* (to recognize red) rather than new propositional knowledge. Who do you find more persuasive, and why? Does your answer affect how you think about whether bacterial sensing involves any form of experience?

**2.** Section 3.2 discusses Searle's biological naturalism — the view that consciousness requires specific neurobiological processes, not just functional equivalents. Consider the following: Is Searle's position self-undermining? If consciousness is a biological phenomenon with a neurological basis, and we don't understand why that neurological basis produces consciousness, have we explained consciousness or just pushed the mystery deeper into biology? What would a genuine explanation of consciousness look like on Searle's view?

**3.** Section 3.3 presents the combination problem as the most serious objection to panpsychism: how do micro-experiences combine into unified macro-experiences? But consider the parallel problem for emergentism (the mainstream view): if micro-physical processes have no phenomenal properties at all, how does organized complexity produce phenomenal experience from nothing? Which problem seems harder to you, and does your assessment of their relative difficulty change your prior for or against panpsychism?

**4.** Section 3.4 proposes a methodological stance: use functional vocabulary freely where functional organization is present; use phenomenal vocabulary only with explicit qualification. One challenge for this stance: all the vocabulary we have for describing cognitive processes (sensing, deciding, remembering, anticipating) was developed by humans to describe their own phenomenal experience. Can we really use this vocabulary in a purely functional way, or does the vocabulary itself carry an implicit phenomenological commitment that cannot be stripped out?

**5.** Suppose a fully worked-out theory of consciousness (say, a refined version of Integrated Information Theory) predicts that a system needs a phi-value above a certain threshold to have any experience, and that E. coli's integrated information falls below this threshold. Would this be evidence that E. coli is not conscious? What would it take to convince you that a non-neural organism has no experience — and is this standard consistent or inconsistent with how you evaluate evidence for experience in other humans?

---

## Part II: Thought Experiments

**Thought Experiment 1: The Philosophical Zombie Bacterium**

A philosophical zombie (p-zombie) is a hypothetical creature that is functionally identical to a conscious being but has no inner experience — it is, as Chalmers puts it, "in the dark." P-zombies are conceivable by stipulation and are used to argue for the logical independence of functional cognition from phenomenal experience.

Now consider a specific question: Is a bacterium more like a philosophical zombie, or more like us?

If you say "more like a p-zombie" — that the bacterium's functional cognition is definitely not accompanied by experience — you owe an explanation of why. What is it about the bacterium's organization that makes you confident it lacks experience? And does this explanation generalize: does it also imply that C. elegans (302 neurons) is a p-zombie? That a bee is?

If you say "more like us" — that there is probably something it is like to be a bacterium — you also owe an explanation. On what grounds? The hard problem tells us that functional sophistication doesn't establish experience. What additional evidence or argument supports the claim?

If you say "we can't know" — this is the agnostic position defended in this chapter. But agnosticism is not a resting place; it is a call to develop better tools. What kind of experiment, in principle, could give you evidence one way or the other?

**Thought Experiment 2: Grading Experience**

Many researchers who study animal welfare believe that experience comes in degrees — that a lobster has less rich experience than a dog, which has less rich experience than a human. This view motivates the practice of grading moral consideration by the richness of cognitive and experiential capacity.

Now extend this view to its logical limits. If experience comes in degrees, and if non-neural organisms have some cognitive capacities (even minimal ones), at what point on the continuum of experience do we reach "zero experience"? Is there a sharp threshold, or does experience simply become arbitrarily small?

If there is a sharp threshold: What determines it? Is it the presence of neurons? A certain level of integrated information? The capacity for pain behavior? How do you justify drawing the line where you draw it without begging the question?

If there is no sharp threshold: What are the implications for ethics? For science? For the concept of experience itself?

**Thought Experiment 3: The Inverted Spectrum of Basal Cognition**

The "inverted spectrum" thought experiment asks: could two people both behave identically when seeing red, but have inverted phenomenal experiences — one experiencing what the other experiences as green, and vice versa? If yes, this suggests that phenomenal character is not determined by functional role.

Now transpose this to basal cognition. Suppose two bacteria have functionally identical chemotaxis mechanisms — same molecular architecture, same behavioral outputs. But suppose (if this is coherent) that they have different phenomenal properties: the "experience" (if any) of moving toward aspartate is different in each, even though the function is the same.

- Is this supposition coherent? What would it mean for the phenomenal properties of organisms to vary independently of their functional organization?
- If the supposition is coherent, does it support or undermine the idea that we can use functional measures (behavioral richness, information integration) as proxies for phenomenal richness?
- Does the inverted spectrum thought experiment reveal anything specific about the difficulties of attributing experience to non-neural organisms?

---

## Part III: Laboratory Investigations

**Lab 1: Mapping the Explanatory Gap in Practice**

This is a reflective exercise rather than a wet lab experiment. Its purpose is to make the explanatory gap vivid and concrete.

Step 1: Choose a simple biological process in a non-neural organism — for example, E. coli chemotaxis, Paramecium avoidance behavior, or root gravitropism in a seedling.

Step 2: Write a complete functional description of the process at three levels: (a) molecular mechanism (which molecules, which reactions), (b) cellular behavior (what the cell does, and in response to what stimuli), (c) adaptive function (what the behavior achieves for the organism's continued organization).

Step 3: Now ask: Is there an explanatory gap between your description and any phenomenal experience the organism might have? What would be *left out* of your description, if the organism has experience?

Step 4: Compare your description of the chosen process with a description of a simple human process — say, withdrawing your hand from a hot stove. At what points in the human description do phenomenal claims appear? At what points do functional claims alone seem insufficient to describe what's happening?

*Analysis questions*:
- Is the explanatory gap equally wide for the bacterial and the human process?
- Does the exercise reveal any features of the biological process that seem to resist purely functional description?

**Lab 2: Testing IIT Predictions in Simple Systems**

Integrated Information Theory (IIT) proposes a mathematical measure of consciousness — phi (Φ) — defined as the amount of information generated by a system above and beyond the information generated by its parts independently. Higher phi = more consciousness.

Using the PyPhi package (available at https://github.com/wmayner/pyphi) or equivalent:

1. Model a simple three-node network representing a minimal signal integration circuit (analogous to a chemotaxis pathway)
2. Calculate the phi value of the network
3. Compare to: (a) the same nodes operating independently (no integration), (b) a five-node version of the same network

*Analysis questions*:
- What phi values do simple biological signal networks produce on this model?
- IIT predicts that networks with phi > 0 have some degree of experience. Do the networks you modeled meet this criterion?
- What are the philosophical objections to using phi as a proxy for experience? (Consider Block's "China Brain" concern applied to IIT.)

**Lab 3: Designing a Criterion-Based Study of Behavioral Evidence for Experience**

Design (but do not necessarily conduct) an experiment that would generate the best available behavioral evidence for or against the presence of experience in a non-neural organism.

Choose one: Mimosa pudica (the sensitive plant), E. coli, Physarum polycephalum.

Step 1: Review the behavioral criteria that have been proposed as indicators of experience (see Barron & Klein, 2016, for one proposal focused on insects):
- Spontaneous behavior in the absence of stimuli
- Integrated, flexible responses to aversive stimuli
- Motivational states that persist beyond the stimulus
- Trade-offs between aversive and appetitive stimuli (indicating valenced experience)
- Learning to avoid aversive stimuli

Step 2: Design experiments to test each criterion in your chosen organism.

Step 3: Evaluate the limitations of behavioral evidence: even if your organism passes all tests, does this establish experience? What confounders might produce the behavioral result without experience? Can these be controlled?

*Key reference to consult*: Barron, A.B., & Klein, C. (2016). What insects can tell us about the origins of consciousness. *Proceedings of the National Academy of Sciences*, 113(18), 4900–4908.

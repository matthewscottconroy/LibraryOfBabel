# Cognitive Architecture: Connections to Empirical Research

The debates about cognitive architecture — classical vs. connectionist, modular vs. non-modular, brain-bound vs. extended — are among the most empirically tractable in philosophy of mind. The questions are partly conceptual (what does it mean for cognition to be *symbolic* rather than sub-symbolic?) and partly empirical (which architectural framework best explains the behavioral and neural data?). This document maps the philosophical positions onto the research programs that bear most directly on them.

---

## Classical Computationalism and Fodor's Modularity Thesis

### The Empirical Case for Modularity

Fodor's *The Modularity of Mind* (1983) claimed that input systems (vision, audition, language parsing) are domain-specific, informationally encapsulated, fast, automatic, and have characteristic breakdown patterns associated with specific neural systems. These are empirically testable claims. The evidence is substantial but more complex than Fodor initially suggested.

**Language processing:** The classical evidence for linguistic modularity came from the dissociation between syntactic processing and general cognition. Patients with Broca's aphasia have impaired syntactic comprehension while preserving semantic understanding and general intelligence; patients with Wernicke's aphasia have impaired semantic comprehension while sometimes preserving syntactic processing. These dissociations suggest that syntactic and semantic processing are relatively independent systems. More recent evidence from ERP studies (the ELAN component for early syntactic processing, the N400 for semantic processing, the P600 for syntactic reanalysis) supports the modularity of different aspects of language processing.

**Visual processing:** Prosopagnosia — the loss of face recognition following damage to the fusiform face area (FFA) — while preserving recognition of non-face objects is a striking case of domain-specific processing. Congenital prosopagnosia (difficulty with face recognition in the absence of brain damage) suggests that face processing is a distinct module with a genetic basis. However, the *expertise hypothesis* (Gauthier and Tarr) challenges the face-specificity claim: the FFA may respond not to faces specifically but to any category for which the subject has acquired high-level expertise (birds in ornithologists, cars in car experts). The debate about whether FFA is face-specific or expertise-specific bears directly on the modularity hypothesis.

**Informational encapsulation:** Fodor's most specific modularity claim — that input systems are informationally encapsulated (their processing is not affected by beliefs, expectations, or higher cognitive states) — has been challenged by research on top-down effects in perception. Perceptual learning (the ability of experience to reshape perceptual processing) suggests that low-level perceptual processing is not encapsulated. Pylyshyn's criterion for cognitive impenetrability — early visual processing should not be affected by beliefs and knowledge — seems to be violated by effects of semantic context on early visual processing (e.g., contextual facilitation of object recognition in congruent scenes).

### The Failure of Classical AI and Its Philosophical Implications

The *frame problem* — how a symbolic reasoning system knows what to update and what to leave unchanged when the world changes — was a central challenge for classical AI that has never been satisfactorily solved. Classical planning systems (STRIPS, its descendants) required explicit axioms listing the effects of each action, which quickly became intractable for realistic environments.

Dreyfus drew on this failure in *What Computers Can't Do* (1972) and *What Computers Still Can't Do* (1992) to argue that the classical computational framework is inadequate for human cognition because human cognition is grounded in embodied, contextual engagement with the world — exactly the kind of context-sensitivity that the frame problem shows classical systems cannot handle. Whether the frame problem has been dissolved by connectionist approaches, or whether it remains a genuine challenge for all computational approaches, is an open question.

---

## Connectionism: The Evidence and Its Interpretation

### The Original PDP Results

Rumelhart and McClelland's (1986) past-tense learning model was the founding demonstration of connectionist cognitive modeling. Rather than implementing explicit rules for past-tense formation (add "-ed" to the verb stem), their model learned past-tense forms through exposure to training examples. Crucially, the model exhibited a U-shaped developmental pattern that closely resembled children's acquisition of past-tense forms: correct irregular forms (went, went) at early stages (when these high-frequency items dominate training), then a period of over-regularization errors (goed, wented) as regular patterns are generalized, then a return to correct forms. This matched the behavioral pattern in children without implementing any explicit rules.

Pinker and Prince (1988) challenged the model, arguing that its behavior could be explained by feature co-occurrence statistics rather than any genuine understanding of morphological structure, and that the model failed on systematic cases (inflecting novel verbs with sounds similar to existing irregular verbs) that children handle correctly. This debate crystallized the question of whether connectionist models exhibit genuine systematicity or are merely sophisticated pattern matchers.

### Deep Learning and Modern Neural Networks

The emergence of deep learning since 2010 has dramatically changed the empirical landscape. Large convolutional neural networks (CNNs) trained on ImageNet achieved human-level performance on image classification and exhibit hierarchical feature detectors — simple edge detectors in early layers, complex object detectors in higher layers — that closely resemble the hierarchical organization of visual cortex (Yamins and DiCarlo, 2016). This provides the strongest evidence to date for a connectionist account of visual processing.

Yamins et al. (2014) showed that representations in CNNs predict neural responses in macaque IT cortex better than any previous model, including biologically inspired models. This is striking: the CNN was optimized for image classification, not for predicting neural responses, yet it produced representations that closely match those in the cortical hierarchy. This suggests that the computational demands of object recognition, rather than specific architectural constraints, largely determine the representational structure of ventral visual cortex.

**The systematicity question revisited:** Fodor and Pylyshyn's argument that connectionist systems cannot exhibit genuine systematicity (because any thinker who can think "John loves Mary" must be able to think "Mary loves John") has been challenged by empirical findings from neural networks. Modern transformers (the architecture underlying large language models) do exhibit a form of compositionality — their representations of complex sentences are compositionally structured in ways that allow them to generalize to novel combinations. Whether this is *genuine* systematicity (with the right compositional structure) or a simulation of systematicity that breaks down in the right test cases is an active research question.

**Representational similarity analysis:** A methodological advance that has been valuable for the architecture debates is *representational similarity analysis* (RSA, Kriegeskorte et al., 2008). RSA compares the geometry of representational spaces in different systems — how similar or different are the representations of different stimuli in model A compared to model B and to neural recordings from area X? RSA provides a tool for testing whether the representational structure of neural network models matches that of biological neural systems, going beyond behavioral agreement to representational agreement.

---

## Predictive Processing: The Empirical Evidence

### The Free Energy Framework

Karl Friston's *free energy principle* (Friston, 2010) proposes that all biological systems — from single cells to brains — minimize a quantity called free energy, which is an upper bound on surprise (the improbability of sensory states given the organism's model of the world). For brains, minimizing free energy means maintaining accurate models of the environment and acting to confirm predictions. This framework unifies perception (updating the model to minimize prediction error), action (changing the world to confirm predictions), and learning (updating the model parameters to improve long-term prediction accuracy).

Predictive coding — the neural implementation of PP — holds that the brain represents the world through a hierarchical generative model in which higher levels send *predictions* to lower levels and lower levels send *prediction errors* back upward. The activity of each neural area is best understood not as encoding stimuli but as encoding prediction errors — the difference between what was expected and what was received.

### Empirical Evidence for Predictive Coding

**Mismatch negativity (MMN):** The MMN is an ERP component generated by the auditory system in response to deviants in a repeated sequence of sounds, even when subjects are not attending to the sounds. PP interprets the MMN as a prediction error signal: the repeated standard sets up a prediction, and the deviant violates it. The MMN is generated in auditory cortex and propagates to frontal areas, consistent with the PP architecture of upward-propagating prediction errors.

**Surprise modulation of neural responses:** Neurons in higher cortical areas often show suppression of responses to repeated stimuli (repetition suppression). PP interprets this as a consequence of accurate prediction: when a stimulus is expected, its prediction error is low, so the prediction error signal that drives neural responses is small. Novel stimuli generate large prediction errors and large neural responses. This interpretation has been supported by studies showing that repetition suppression is context-dependent: the degree of suppression depends on whether the context generates the prediction, not merely on whether the stimulus has been seen before (Summerfield and Trittschuh, 2008).

**Perceptual filling-in and top-down effects:** PP predicts that what we experience is not the raw input but the prediction, corrected by prediction errors. This predicts extensive top-down effects on perception. The Müller-Lyer illusion, the Necker cube, and other bistable percepts can be understood as cases where the brain has two equally good predictive hypotheses and switches between them. Agoraphobia and other anxiety disorders might be understood as pathological priors — overly strong expectations of threat that generate threat-confirming prediction errors even in safe environments.

### The Active Inference Framework for Action

PP accounts of action — *active inference* — hold that actions are selected not by computing the action that maximizes expected reward but by generating motor commands that fulfill proprioceptive predictions. The agent has a model of the desired state of the body (the intended action) and generates motor commands designed to reduce the prediction error between the actual proprioceptive input and the predicted input. This unifies perception and action under the same free energy minimization framework.

The active inference account makes specific predictions about the structure of voluntary action. It predicts that the distinction between voluntary and involuntary movement lies in whether the motor commands are generated by descending predictions (voluntary) or ascending proprioceptive errors (reflexive). It also predicts specific relationships between the motor system and the predictive hierarchy that are testable with neuroimaging.

The philosophical significance of active inference is considerable: it suggests that the traditional distinction between perception (responding to the world) and action (acting on the world) is not fundamental. Both are instances of the same process of minimizing prediction error, with the difference being whether the error is reduced by updating the model (perception) or by acting on the world (action). This has implications for theories of agency (Unit 08) and for the relationship between perception and action in embodied cognition.

---

## The Extended Mind: Empirical Grounding

### Distributed Cognition Research

Edwin Hutchins' *Cognition in the Wild* (1995) is the empirical anchor for the distributed cognition tradition. Hutchins studied naval navigation — specifically, the process by which a ship's team determines position and plots course — through detailed ethnographic observation. His central finding was that the cognitive process of navigation is not located in any individual brain but is distributed across persons, instruments (charts, alidades, compasses), and their interactions. The relevant representational states — the ship's position, its heading, the bearing to landmarks — are encoded in external representations (plots on charts, angles set in instruments) as well as in individual minds.

Hutchins showed that this distributed system exhibits properties — robustness to individual error, graceful handling of novel situations, efficient division of cognitive labor — that cannot be explained by the cognitive processes of any individual. This provides empirical support for the extended mind thesis: the cognitive processes involved in navigation literally extend beyond individual skulls into the sociotechnical system.

### External Representations and Cognitive Offloading

Empirical work on *cognitive offloading* — the use of external resources to reduce internal cognitive demands — has grown substantially (Risko and Gilbert, 2016). People routinely use external tools (notes, calendars, smartphones) to extend their cognitive capacities beyond what would be possible with biological memory and computation alone. Studies show that subjects given external tools perform better on working memory and reasoning tasks, not because the tools supplement limited capacity but because the integrated system (person + tools) forms a more capable cognitive unit.

The philosophical question is whether this is merely *using* cognitive tools (instrumentalism) or whether it constitutes genuine cognitive extension (Clark-Chalmers). The empirical evidence does not settle this question — both interpretations are compatible with subjects performing better when they use external tools — but it does establish that external representations are causally integrated into cognitive processing in ways that go beyond mere consultation.

**Predictive processing and the extended mind:** A synthesis of PP and the extended mind has been proposed (Clark, 2016): if the brain is a prediction machine that minimizes prediction error, and if external structures can be incorporated into the predictive hierarchy (by generating reliable predictions and absorbing prediction errors), then those structures count as part of the extended cognitive system. This gives PP a principled reason to endorse the extended mind thesis: the boundary of the cognitive system is the boundary of the predictive hierarchy, which may extend beyond the skull.

---

## Embodied Cognition: The Empirical Literature

### Simulation Theory and Motor Resonance

The most influential research program in embodied cognition is *simulation theory* (Barsalou, 1999; Gallese, 2007): understanding concepts involves simulating the perceptual and motor states associated with them, using the same neural systems that implement those states. Understanding the concept GRASP involves motor system activation; understanding the concept RED involves visual cortex activation.

**Action verb and motor cortex:** Pulvermüller et al. (2005) showed that reading action verbs associated with different body parts (kick, pick, lick) produced differential activation in motor cortex topographically organized by body part, consistent with simulation theory. Reading "kick" activates leg motor areas; reading "pick" activates hand motor areas. This finding has been widely replicated, though its interpretation is contested: some argue the motor cortex activation is constitutive of understanding (strong embodied simulation); others argue it is an epiphenomenal correlate of a process that could in principle occur without it.

**Conceptual metaphors and motor simulation:** Glenberg and Kaschak (2002) showed that sentence comprehension was facilitated when the motor response required by the experimental task was compatible with the motor direction implied by the sentence ("Open the drawer" facilitated responses toward the subject; "Close the drawer" facilitated responses away). This *action compatibility effect* suggests that comprehending sentences about actions involves preparing the relevant actions.

**TMS disruption studies:** Transcranial magnetic stimulation (TMS) applied to motor cortex during action verb processing disrupts performance in some conditions, providing causal evidence (beyond mere correlation) that motor cortex is involved in conceptual processing. However, TMS effects on conceptual processing are typically small and context-dependent, suggesting that motor cortex is not necessary for language understanding in all contexts.

### The Rubber Hand Illusion and Bodily Self-Models

The rubber hand illusion (Botvinick and Cohen, 1998) demonstrates that the sense of body ownership is malleable and depends on multisensory predictive integration. When a rubber hand is stroked synchronously with the subject's hidden real hand while the subject watches the rubber hand, subjects report the illusion that the rubber hand is their own, and show physiological responses (skin conductance, skin temperature drop in the real hand) consistent with experiencing the rubber hand as part of the body.

The RHI has been interpreted within PP as a case of predictive integration: the brain has a prior that its hand is where it typically is, but synchronous touch on the visible rubber hand generates a prediction that the visible hand is the felt hand. When the evidence (synchronous touch + vision of the rubber hand) is strong enough, the predictive model updates to incorporate the rubber hand as part of the body. This interpretation is supported by the finding that asynchronous touch (which should not generate the same prediction) produces a much weaker illusion, and by the correlation between illusion strength and the shift of perceived hand position toward the rubber hand.

The RHI has implications for the philosophy of bodily self-awareness (Metzinger's phenomenal self-model) and for understanding disorders of body ownership (somatoparaphrenia — the denial of ownership of a limb) as pathologies of predictive integration.

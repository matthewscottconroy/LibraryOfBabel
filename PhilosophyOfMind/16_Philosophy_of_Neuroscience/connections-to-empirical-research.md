# Philosophy of Neuroscience: Connections to Empirical Research

Philosophy of neuroscience is distinctive in that its primary object of study — neuroscience itself — is an ongoing scientific enterprise rather than a completed body of knowledge. This means that philosophical analysis must engage with the current methodology, findings, and debates of neuroscience as a live science, not merely with what neuroscience has established. This document maps the central philosophical frameworks (levels of explanation, reduction, NCC, mechanistic explanation) onto the research programs that most directly bear on them.

---

## Marr's Three Levels: Empirical Applications and Critiques

### The Vision Example

Marr developed his three-levels framework in the context of visual neuroscience. At the *computational level*, the problem of early vision is: given the two-dimensional array of intensity values on the retina, recover the three-dimensional structure of the world. This is mathematically specified as a reconstruction problem, and the relevant algorithms must solve this problem.

At the *algorithmic level*, Marr developed specific algorithms — for the detection of zero-crossings in the convolution of the image with a Laplacian-of-Gaussian filter, for the matching of binocular disparities, for the computation of shape from shading — that could solve the computational problem. These algorithms were specified in terms of explicit computational steps.

At the *implementational level*, Marr noted that the same algorithms could in principle be implemented in many physical substrates. The question of which neural circuits implement Marr's algorithms is an empirical question that requires the tools of neurophysiology and neuroanatomy.

The framework's success in visual neuroscience has been substantial: the three-levels analysis has guided the identification of neural structures for specific visual computations. Orientation selectivity in V1 (the detection of edges at specific angles) can be understood as implementing an early step in the recovery of image contours; disparity tuning in V1 and V2 can be understood as implementing binocular matching for depth estimation; face-selective neurons in IT cortex can be understood as implementing the recognition of face identities.

### Where the Framework Is Strained

The three-levels framework faces several challenges when applied beyond early visual processing:

**Underdetermination of the computational level:** For early vision, the computational problem is mathematically well-specified (inverse optics). For higher cognition — reasoning, planning, social cognition — the computational problem is much harder to specify. What is the *function* that prefrontal cortex computes? The computational-level specification of social cognition or executive function requires assumptions about the goals of the computation that are often not independently motivated.

**Interactions between levels:** Marr assumed the three levels were relatively independent: you could specify the computational problem without knowing the algorithm, and the algorithm without knowing the implementation. But neural constraints often influence algorithmic choices: the massive recurrency of the cortex, its energy constraints, and its temporal dynamics constrain which algorithms can be implemented. The algorithmic choice of predictive coding (generating predictions and propagating prediction errors) may be partly determined by the implementational advantages of recurrent circuits. The levels are not as independent as Marr suggested.

**The level of psychological kinds:** The three levels leave ambiguous where folk psychological kinds (beliefs, desires, emotions) fit. These are not computational-level descriptions (they are not mathematically specified functions), not algorithmic-level descriptions (they are not procedures), and not implementational-level descriptions (they are not neural circuits). They seem to require a fourth level — sometimes called the *personal level* (in contrast to the sub-personal levels of Marr's framework) — that specifies the psychological agent and their states.

---

## The Mechanistic Explanation Framework

### Machamer, Darden, and Craver

Machamer, Darden, and Craver (2000) proposed that scientific explanation — at least in biology and neuroscience — is typically mechanistic: we explain a phenomenon by identifying the mechanism that produces it. A mechanism is an organized system of *entities* (neurons, synapses, receptors, ion channels) and *activities* (firing, releasing neurotransmitter, binding, depolarizing) whose organization produces the phenomenon.

Crucially, mechanistic explanation is *multilevel*: the mechanism involves entities at one level that are themselves composed of entities at lower levels. Synaptic transmission is the mechanism for certain aspects of neural signaling; the molecular events at the synapse (vesicle release, receptor binding, ion flow) are the mechanism for synaptic transmission; and so on. Explanation in neuroscience works by filling in the mechanistic sketch — identifying the entities, their activities, and their organization at the relevant level of detail.

Craver (*Explaining the Brain*, 2007) developed the most complete version of this framework. He introduces the notion of a *mechanistic model*: a representation of the entities, activities, and organizational features of a mechanism that is used to explain and predict phenomena. A mechanistic model is neither a law nor a description of regularities; it is an idealized representation of the causal structure of the mechanism.

### The Constitutive Relevance Criterion

Craver's most philosophically important contribution is the *constitutive relevance criterion*: a component of a mechanism is constitutively relevant to a phenomenon if and only if it is part of the mechanism that produces the phenomenon. This criterion distinguishes genuine mechanistic explanation (identifying mechanism components) from mere correlation (identifying neural activity that co-varies with the phenomenon without being part of the mechanism).

The constitutive relevance criterion has methodological implications for neuroscience. A simple correlation between area X's activity and behavior Y does not show that X is constitutively relevant to Y: X might be an effect of the processes that produce Y rather than a component of those processes. Establishing constitutive relevance requires *manipulationist* evidence — evidence that manipulating X (by lesion, TMS, or optogenetic stimulation) affects Y in the predicted way.

The distinction between mere correlation and constitutive relevance bears on the NCC research program. Identifying the neural correlate of a conscious state (by comparing neural activity in conscious vs. unconscious conditions) establishes correlation but not necessarily constitutive relevance. The neural activity might be an effect of the processes that generate consciousness rather than a component of the consciousness-generating mechanism. Establishing that the identified correlates are constitutively relevant requires the manipulationist methodology.

---

## The NCC Research Program: Methods and Limitations

### Contrastive Methods

NCC research relies on *contrastive methodology*: identifying the neural activity that differs between conscious and unconscious conditions (e.g., perceiving vs. not perceiving a stimulus) while holding everything else constant. The contrast isolates the neural activity associated with consciousness.

The major experimental paradigms (binocular rivalry, masking, attentional blink, change blindness) all implement this contrastive logic. In binocular rivalry, the two conditions are left-eye-dominant vs. right-eye-dominant perception, with the physical stimulation constant. The NCC is the activity that tracks perceptual dominance rather than physical stimulation. In masking paradigms, the two conditions are masked (invisible) and unmasked (visible) presentations of the same stimulus; the NCC is the additional activity in the unmasked condition.

### The Backward Masking Paradigm and Early vs. Late Correlates

A productive debate in consciousness neuroscience concerns whether the NCC involves *early* activity (in sensory areas, shortly after stimulus presentation) or *late* activity (in prefrontal-parietal areas, 200-400ms after stimulus). This debate tracks the theoretical conflict between *local recurrence* views (consciousness depends on recurrent processing within sensory areas) and *global workspace* views (consciousness depends on global broadcasting to prefrontal-parietal areas).

Lamme's (2006) local recurrence view holds that recurrent processing within V1 and other sensory areas is sufficient for phenomenal consciousness, independent of feedback from frontoparietal areas. Evidence: masking that interrupts recurrent processing in V1 (by short stimulus-onset asynchrony) eliminates conscious perception and the late P3 wave; masking that does not interrupt V1 recurrence (backward masking at longer SOAs) leaves early components intact while eliminating the P3. Lamme interprets this as showing that local recurrence generates phenomenal consciousness even without frontoparietal involvement.

The global workspace interpretation disputes this: the P3 wave (generated by frontoparietal areas) is the NCC; local recurrence generates unconscious "phenomenal" processing that does not become conscious without frontoparietal ignition. The interpretation turns partly on how one individuates the NCC from its enabling conditions (the local sensory processing that feeds into the global workspace) and partly on philosophical commitments about whether phenomenal consciousness can occur without access.

### The No-Report Paradigm

A methodological innovation designed to address the phenomenal/access confound is the *no-report paradigm* (Tsuchiya et al., 2015). Standard NCC paradigms use report as the indicator of consciousness. But report is an access process — it requires the information to be available for verbal output. If phenomenal consciousness can occur without access (Block's overflow hypothesis), then report-based paradigms confound the NCC of phenomenal consciousness with the NCC of access consciousness.

The no-report paradigm attempts to identify NCCs independently of report, using eye movements, pupil dilation, or neural signatures as implicit markers of perceptual state. When NCCs are identified with no-report paradigms and compared to report paradigms, the results sometimes diverge: activity in frontoparietal areas (the GWT signature) is more strongly associated with report-based than no-report-based consciousness, while activity in sensory areas is more similarly associated with both. This is consistent with a local recurrence view: phenomenal consciousness is generated by sensory area recurrence; access is generated by frontoparietal recruitment.

---

## Reduction and Emergence in Neuroscience

### The Protein Folding Analogy

Reduction in science typically takes the form of *mechanistic reduction*: explaining a higher-level phenomenon by identifying the lower-level mechanism that produces it. Biologists reduced genetics to molecular biology by identifying DNA as the physical substrate of the gene and identifying DNA replication as the mechanism of heredity. This is not *eliminative* reduction (genes don't disappear) but *constitutive* reduction (the nature of genes is identified with their molecular constituents).

The question is whether psychological kinds (beliefs, desires, emotions) can be constitutively reduced to neural kinds in the same way. The multiple realizability argument (Unit 02) suggests that the reduction is blocked: the same psychological kind (pain) can be realized by many different neural states, so no specific neural state is the *nature* of pain.

Craver's mechanistic approach provides a more nuanced account. Even if pain cannot be identified with C-fiber firing (because of multiple realizability), the *mechanism* for pain production in a human can be identified with specific neural components. The reduction is *species-specific* or *context-specific*: human pain is constitutively realized by this mechanism; octopus pain by a different mechanism. The psychological kind is unified across species; the neural mechanism is not.

### The Emergentist Challenge

Strong emergence — the view that higher-level properties have causal powers that are not reducible to the causal powers of their lower-level constituents — has been invoked to account for consciousness. If consciousness is strongly emergent, then the neural correlates of consciousness are not sufficient to explain it; there are additional properties of the conscious state that are not entailed by the neural properties.

The standard physicalist response is to insist on weak emergence: higher-level properties supervene on lower-level properties (no mental difference without a neural difference) but are not logically derivable from them without idealization. The explanatory gap between neural descriptions and phenomenal descriptions is a feature of our concepts, not the world — consistent with Type-B physicalism. Whether weak emergence is sufficient to account for the apparent independence of phenomenal consciousness from physical properties is the central question on which hard problem debates turn.

The *causal exclusion* argument (Kim) generates a related problem: if conscious states are weakly emergent (they supervene on but are distinct from neural states), then they seem to be causally inert — their causal work is done by their neural realizers. This connects to the broader problem of mental causation (Unit 02), and it has implications for neuroscience: if psychological states have no autonomous causal powers, then neuroscientific explanation (in terms of neural mechanisms) is complete and psychological explanation is at best a convenient shorthand.

---

## Philosophy of Psychiatry: Classification and Explanation

### The Research Domain Criteria (RDoC)

The National Institute of Mental Health's Research Domain Criteria (RDoC) framework represents a philosophical intervention in psychiatry: it proposes to classify mental disorders not by symptom clusters (as the DSM does) but by dimensions of observable behavior and neurobiological measures. RDoC identifies five major domains (negative valence systems, positive valence systems, cognitive systems, systems for social processes, and arousal/regulatory systems) and attempts to characterize disorders in terms of dysfunctions in these domains.

The philosophical motivation for RDoC is *theoretical validity*: DSM categories like "major depressive disorder" may not be natural kinds — they may be heterogeneous collections of conditions that share surface symptoms but have different causes and mechanisms. RDoC attempts to identify natural kinds by reference to underlying biological mechanisms rather than surface symptom clusters.

The philosophical challenge for RDoC is that the choice of domains and the mapping of behaviors onto neural systems presupposes a theory of what the relevant neural mechanisms are. If that theory is wrong — if, for instance, the dopamine system is not univocally the neural basis of positive affect — then the RDoC categories will not be natural kinds either. The classification scheme is only as good as the theory that underlies it.

### The Biopsychosocial Model and Its Tensions

George Engel's biopsychosocial model (Engel, 1977) proposed that mental illness cannot be adequately explained by biological factors alone; social and psychological factors are irreducibly involved. This is a claim about the *level of explanation* appropriate for mental illness, and it connects directly to the philosophy of neuroscience debate about reduction.

The tension in contemporary psychiatry is between a reductionist tendency (identify the neural mechanisms of mental disorders and explain psychiatric conditions at the neural level) and an anti-reductionist tendency (mental disorders are constituted by social relationships, psychological meanings, and personal histories that are not captured by neural descriptions). The DSM and RDoC represent different positions on this spectrum; the biopsychosocial model attempts a synthesis that neither program fully achieves.

The philosophical insight that mechanistic explanation is multilevel supports a version of the biopsychosocial model: explaining psychiatric disorders requires describing mechanisms at multiple levels — molecular, cellular, circuit, psychological, social — and no single level is privileged. The relevant mechanism for depression might include serotonergic dysfunction (molecular), disrupted hippocampal neurogenesis (cellular), default mode network hyperactivity (circuit), ruminative thought patterns (psychological), and social isolation or trauma (social).

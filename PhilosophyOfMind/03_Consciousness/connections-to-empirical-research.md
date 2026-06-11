# Consciousness: Connections to Empirical Research

Philosophy of consciousness has been unusual among philosophical subfields in its systematic engagement with empirical science. This is partly because the hard problem is precisely a claim about the limits of scientific explanation — so scientific progress is directly relevant to whether the hard problem can be dissolved or only deepened — and partly because the phenomenology of consciousness is rich enough to generate specific, testable predictions. This document maps the major philosophical positions onto active empirical research programs.

---

## The Neural Correlates of Consciousness (NCC) Research Program

### The Paradigm Cases

The NCC research program, initiated by Crick and Koch in 1990, seeks the minimal neural conditions sufficient for conscious experience. The paradigm experimental design exploits conditions in which subjective experience and physical stimulation come apart. The three most productive paradigms are:

**Binocular rivalry:** When different images are presented to each eye simultaneously (a face to one eye, a house to the other), subjects experience alternating dominance rather than superposition — they see either the face or the house, switching every few seconds. The physical stimulation is constant; the perceptual experience alternates. Comparing neural activity during face-dominant periods to house-dominant periods identifies activity correlated with conscious perception rather than with stimulation per se. Tong et al. (1998) showed that activation in face-selective areas (FFA) and scene-selective areas (PPA) tracks perceptual dominance, and more recent work (e.g., Brascamp et al., 2015) implicates frontal and parietal regions in determining which percept dominates.

**Change blindness and inattentional blindness:** When attention is diverted, large visual changes can go completely unnoticed (change blindness) and unexpected objects can be entirely missed (inattentional blindness — Simons and Chabris's "invisible gorilla" study, 1999). These phenomena suggest that visual awareness requires more than retinal stimulation and even more than processing in early visual areas; it requires attentional engagement. The neural substrates of attention and their relationship to conscious awareness are a major research area (Dehaene et al.'s work on the attentional blink).

**Masking and subliminal perception:** Brief presentation of a stimulus, followed immediately by a patterned mask, can prevent conscious perception while the stimulus still produces neural responses and behavioral priming effects. Comparing neural activity to masked (subliminally processed) vs. unmasked (consciously perceived) stimuli isolates the additional neural activity associated with consciousness.

### Philosophical Implications of NCC Research

The central philosophical difficulty for NCC research is what Ned Block calls the *phenomenal/access* confusion (Block, 2007): experiments designed to identify the NCC typically use *report* as the criterion for consciousness — subjects are asked whether they saw something, and the NCC is the neural activity that correlates with affirmative reports. But reportability is a criterion for *access* consciousness (availability for reasoning and verbal report), not necessarily for *phenomenal* consciousness. If there can be phenomenally conscious states that are not access-conscious — if, as Block argues, phenomenal consciousness can "overflow" access — then NCC research as standardly practiced may be identifying the NCC of *access* consciousness while leaving phenomenal consciousness unaddressed.

The *overflow hypothesis* is testable: if phenomenal consciousness is broader than access, subjects should have perceptual experience of items in a briefly presented array that they cannot subsequently report (because the items decay from visual short-term memory before they can be reported). The capacity limitation of working memory would then give the appearance of no consciousness where there was actually consciousness. Block and Lamme support this interpretation; Cohen, Dennett, and others deny it, arguing that without access there is no consciousness. The debate hinges on whether the richness of phenomenal experience subjects report (seeing the whole array as rich) reflects genuine consciousness or is a post-hoc confabulation based on partial access.

---

## Global Workspace Theory: The Leading Empirical Theory

### The Theory

Bernard Baars' Global Workspace Theory (GWT) holds that consciousness consists in the global broadcasting of information across a *global workspace* — a large-scale neural workspace accessible to multiple cognitive processes (attention, working memory, language, executive function, long-term memory). When information enters the global workspace, it becomes available to all these processes and thereby becomes conscious. When it is processed locally — within modality-specific systems — it remains unconscious.

Stanislas Dehaene and Jean-Pierre Changeux developed *Global Neuronal Workspace Theory* (GNWT), the neurobiologically specific version of GWT. GNWT identifies the physical correlates of Baars' workspace: it is implemented by long-range cortico-cortical connections linking prefrontal and parietal cortices, which provide the broadcasting mechanism. These areas project to and receive from local cortical areas, enabling global integration of locally processed information.

### The Experimental Evidence

**Masking experiments:** Dehaene et al. (2001) showed that when words are masked and not consciously perceived, they produce local activation in occipital areas but fail to produce activation in prefrontal-parietal network. When the same words are consciously perceived, they produce a dramatic ignition of prefrontal-parietal areas. This ignition is the putative neural signature of global workspace broadcasting.

**The P3 wave:** Consciously perceived stimuli produce a large, late positive ERP component (P3b, occurring 300-500ms after stimulus) that unmasked stimuli do not. The P3b is generated by prefrontal-parietal sources and is interpreted by GWT researchers as the electrophysiological signature of global broadcasting. Its amplitude correlates with subjective confidence in perception. Critically, the P3b is reduced or absent in vegetative state patients and in patients under anesthesia, making it a candidate marker for disorders of consciousness.

**The Perturbational Complexity Index (PCI):** Casali et al. (2013) developed a measure of consciousness based on the complexity of EEG responses to transcranial magnetic stimulation (TMS) — the PCI. Stimulating an area of the brain and recording the complexity of the subsequent EEG response measures how much information is being integrated and broadcast across the cortex. The PCI reliably distinguishes conscious from unconscious states (wakefulness vs. various depths of anesthesia, NREM sleep vs. REM sleep) and discriminates between disorders of consciousness (vegetative state vs. minimally conscious state). Whether PCI measures GWT's global broadcasting or IIT's integrated information (or both) is debated.

### Philosophical Assessment

GWT is a theory of *access* consciousness: it explains why certain information is available for reasoning and report. Whether it explains *phenomenal* consciousness depends on whether one accepts that phenomenal consciousness just is (or is exhausted by) access consciousness. Block argues it does not; Dehaene and Changeux argue that phenomenal properties reduce to certain features of the global neuronal workspace. The debate is ultimately about whether there is a residual explanatory gap even after explaining access — which is precisely the hard problem.

A useful diagnostic: GWT predicts that unconscious stimuli (masked words, unattended stimuli during inattentional blindness) produce *only* local processing, while conscious stimuli produce *global* processing. This is a substantive empirical prediction, confirmed by masking studies. But this does not show that global processing *is* consciousness; it shows that global processing is necessary for consciousness-as-measured-by-reports. Whether it is sufficient, and whether it captures phenomenal consciousness, are separate questions.

---

## Integrated Information Theory: The Most Radical Proposal

### The Theory

Giulio Tononi's Integrated Information Theory (IIT) begins with phenomenology rather than neuroscience: it asks what intrinsic properties experience has, then asks what physical properties could instantiate them. Tononi identifies five axioms of phenomenal experience: consciousness *exists* (the existence axiom); it is composed of distinguishable features (the *composition* axiom); it is informationally specific (the *information* axiom); it is integrated, not decomposable into independent parts (the *integration* axiom); and it is the one and only way the information is present (the *exclusion* axiom). From these axioms, IIT derives that consciousness is identical to the intrinsic causal power of a system — specifically, the maximum irreducible cause-effect structure, measured by Φ (phi).

Systems with high Φ are highly conscious; systems with low Φ are less conscious; systems with Φ = 0 are not conscious at all. Crucially, IIT implies that *feedforward* networks — including most standard deep neural networks — have Φ = 0, since their causal structure is not irreducible (each element's causal contribution is not different from what it would be if the element were independent). Recurrent networks can have high Φ.

### Empirical Predictions and Tests

IIT makes predictions that differ from GWT in important ways:

**Feedforward vs. recurrent:** IIT predicts that feedforward processing should generate less consciousness than recurrent processing, holding computational complexity constant. GWT predicts that any processing that enters the global workspace is conscious. These predictions are potentially discriminable: unconscious but computationally powerful feedforward processing (predicted by IIT) vs. conscious global workspace entry even for feedforward processes (predicted by GWT).

**Cerebellum vs. cortex:** The cerebellum has roughly 70% of the brain's neurons but appears to contribute little to consciousness: cerebellar lesions can produce profound motor deficits without disrupting consciousness. IIT explains this as follows: the cerebellum is organized as a largely feedforward system with many parallel, modular pathways and low integrated information. The cortex, which is richly recurrent, has high Φ. GWT explanation: the cerebellum's local processing never enters the global workspace.

**The PCI measure:** Massimini and Tononi developed PCI partly as a proxy for Φ. The connection is not exact (computing exact Φ is computationally intractable for real neural systems), but the two measures converge in their predictions about states of consciousness. Whether PCI genuinely approximates Φ, or whether it measures something more like global broadcasting (relevant to GWT), is contested.

### Philosophical Assessment

IIT is the most philosophically ambitious theory of consciousness because it claims to explain phenomenal consciousness, not just access consciousness. The explanation has the form of an identity: consciousness is identical to integrated information in the relevant sense. If this identity holds, then there is no hard problem for IIT to dissolve — the explanatory gap is bridged by the identity claim.

But IIT faces a serious philosophical objection: the *China Brain* objection applied to IIT. Simple systems — large grids of logic gates — could in principle have high Φ while having no behavior that we would associate with consciousness. IIT implies they are highly conscious. This is counterintuitive and strikes many as a *reductio* of the theory. Tononi's response is that counterintuitive implications can be borne if the theory is otherwise successful; but whether the success of IIT is sufficient to warrant this tolerance for counterintuitive implications is genuinely contested.

---

## Higher-Order Theories and Their Empirical Implications

### The Theory

Higher-order theories (HOT, Rosenthal; DHOT, Carruthers) hold that a mental state is conscious when it is accompanied by a higher-order representation of it. First-order visual states (representations of the visual world) become conscious when there are thoughts *about* those states (higher-order representations). This generates a specific prediction: brain areas involved in higher-order representation should correlate with consciousness of first-order states.

### Empirical Evidence

The *prefrontal cortex* is the main candidate for the neural implementation of higher-order representations. Patients with bilateral prefrontal damage (as in severe frontal lobe injuries) show impaired introspective reporting while often retaining basic perceptual and cognitive function. HOT theories predict this dissociation: first-order representations survive, but the higher-order machinery for making them conscious is damaged.

*Recurrent processing* from higher to lower cortical areas — the kind of top-down feedback that might implement higher-order representation — has been shown to be necessary for consciousness in masking studies (Lamme, 2006). When feedback is blocked (by very early masking), subjects do not report conscious experience even though feedforward processing is intact. HOT theories interpret this as showing that first-order processing without higher-order feedback is insufficient for consciousness.

However, the relationship between recurrent processing and higher-order representation is not straightforward. Recurrent processing within early visual areas (V1 recurrence) might generate consciousness without engaging prefrontal higher-order systems. This *local recurrence* view — consciousness can be generated by recurrent processing in sensory cortex, without prefrontal involvement — challenges HOT theories.

---

## The Libet Experiments and the Neural Timing of Conscious Decisions

### The Research

Benjamin Libet's experiments (Libet et al., 1983) asked subjects to flex their wrists whenever they felt like it and to note the position of a clock hand at the moment they became aware of the urge to move. EEG recordings showed that a *readiness potential* — a slow negative ERP buildup — began approximately 550ms before the movement, while subjects reported becoming aware of their intention to move only approximately 200ms before movement. This implies that the neural process initiating voluntary action precedes conscious awareness of the decision by several hundred milliseconds.

Libet himself interpreted his results as compatible with free will: he suggested that there may be a 200ms window in which the conscious mind can veto the action that the brain has already begun to initiate. This "free won't" hypothesis has been widely discussed.

More recent experiments (Soon et al., 2008, using fMRI rather than EEG) found that activity in frontoparietal areas could predict which button subjects would press up to ten seconds before they reported making the decision. This extends Libet's result and has been interpreted as showing that "decisions" are made unconsciously before they become conscious.

### Philosophical Assessment

The Libet results bear on philosophy of action (Unit 08) and free will, but also on the relationship between consciousness and neural processing in consciousness research. They suggest that the temporal relationship between conscious awareness and neural events is more complex than a simple model would predict.

The philosophical interpretation is contested. Critics of the free will interpretation argue: (1) the readiness potential may reflect the motor preparation for an action that hasn't been decided upon yet — a state of readiness, not a decision; (2) subjects' reports of when they became aware of their intention may be subject to systematic timing errors; (3) the prediction from fMRI activity is probabilistic, not certain, suggesting that the neural activity constrains but does not determine the choice.

The deeper question for consciousness research is what Libet's results tell us about the *causal* role of consciousness. If conscious awareness of an intention follows the neural initiation of the action by several hundred milliseconds, does this show that conscious states are *causally inert* epiphenomena? Or does it show that the onset of conscious awareness precedes its peak, and that the temporally displaced awareness still plays a causal role? The relationship between the timing of neural events and the causal efficacy of consciousness is genuinely unsettled.

---

## Disorders of Consciousness and the Clinical Sciences

### The Research Program

The study of disorders of consciousness — vegetative state (VS), minimally conscious state (MCS), and locked-in syndrome — provides a natural experiment in what happens when consciousness is graded or absent. Patients in VS show sleep-wake cycles and reflexive responses but no signs of awareness; patients in MCS show inconsistent but reproducible signs of awareness. The distinction between VS and MCS has been revised multiple times as new detection methods have emerged.

The most striking finding came from Owen et al. (2006): a patient clinically diagnosed as in VS showed brain activation in areas associated with mental imagery (supplementary motor area, parahippocampal gyrus) when asked to imagine playing tennis, indistinguishable from the activation patterns of healthy volunteers. The patient appeared to be able to follow instructions and form mental imagery despite being behaviorally unresponsive. Subsequent work by Owen's group established that such patients can communicate yes/no answers by modulating activity in different brain areas.

### Philosophical Implications

The VS/MCS literature bears directly on the question of *behavioral criteria for consciousness*: a patient in VS who has no behavioral output may nonetheless be conscious, as Owen's results suggest. This vindicates the philosophical point that behavioral evidence is insufficient to determine consciousness — that behavioral and phenomenal consciousness can come apart.

It also raises urgent practical questions about moral status: if VS patients may sometimes be conscious despite having no behavioral output, then the decisions made about their care (withdrawal of life support, decisions about pain management) may be made without adequate information about their phenomenal state. The philosophical analysis of what consciousness requires — and what evidence is relevant — has direct clinical and ethical implications.

The identification of neural markers that correlate with consciousness independently of behavior (the PCI, the P3 wave, neural imagery responses) represents exactly the kind of progress that consciousness research needs: evidence for consciousness that does not depend solely on reportability.

---

## The Relationship Between Attention and Consciousness

### The Research

Attention and consciousness are closely associated: attended stimuli are typically conscious; unattended stimuli are typically not. But the relationship is not one of identity. There is evidence for conscious processing of unattended stimuli (Koivisto and Revonsuo, 2007) and for unconscious processing of attended stimuli (in some masking paradigms, attention is deployed to a masked stimulus that is not consciously perceived).

The most important dissociation for philosophical purposes is *inattentional blindness* vs. *change blindness*. In inattentional blindness, subjects fail to notice unexpected stimuli (the gorilla) when their attention is occupied. In change blindness, subjects fail to notice large changes in scenes when there is a momentary visual disruption (a flicker, a cut). Both suggest that without attention, perceptual processing is insufficient to produce conscious awareness — but they may reflect different mechanisms.

Block's *overflow hypothesis* claims that phenomenal consciousness exceeds access consciousness in some conditions: subjects may phenomenally experience an entire visual array (including items that are not accessed and reported) even though they can only report a small subset. This would show that attention is not required for *phenomenal* consciousness, though it may be required for *access* consciousness. The empirical debate turns on how to interpret subjects' claims that they experienced more than they reported.

### Philosophical Assessment

The attention-consciousness relationship connects to the broader question of whether consciousness requires a global broadcasting mechanism (GWT) or can occur locally (IIT, local recurrence view). If attention is required for global broadcasting, and global broadcasting is required for consciousness, then consciousness requires attention. If IIT is correct and consciousness is integrated information, then highly integrated but unattended processing might still be conscious.

The philosophical lesson is that empirical findings about attention and consciousness underdetermine the theoretical question: the same data — that attention influences what subjects report as conscious — is compatible with multiple theories that make different predictions about cases where attention and consciousness come apart.

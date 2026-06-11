# Artificial Minds: Connections to Empirical Research

The philosophy of artificial minds has undergone a dramatic transformation since 2020. Questions that were previously confined to thought experiments — can a machine understand language? could a computer be conscious? would an artificial mind have genuine intentionality? — are now questions that arise in interpreting the behavior of deployed systems. This document maps the philosophical debates onto specific empirical findings from AI research and cognitive science.

---

## Large Language Models: What the Empirical Evidence Shows

### Architecture and Training

Large language models (LLMs) — GPT-4 and its successors, Claude, Gemini, LLaMA, and their variants — are transformer-based neural networks trained on token prediction: given a sequence of tokens, predict the next token. The transformer architecture (Vaswani et al., 2017) introduces *self-attention* mechanisms that allow the network to relate every token in the input to every other token, computing a representation that captures the statistical relationships between words across the entire context window.

Training occurs on enormous corpora of text (GPT-3 was trained on approximately 570 billion tokens; subsequent models on larger corpora) via gradient descent on the cross-entropy loss. The network learns to predict text well by developing internal representations that capture a remarkable range of semantic, syntactic, and factual relationships.

The philosophical significance of the training procedure is this: LLMs learn from *language* — from the statistical regularities in human-generated text — rather than from direct sensorimotor interaction with the world. They have no body, no perception of the physical environment, no history of embodied interaction with objects. Their representations of the world are derived entirely from the ways that humans have described the world in text. Whether representations derived entirely from linguistic data can constitute the kind of grounded, embodied understanding that cognitive scientists typically mean by *understanding* is a central philosophical question.

### Emergent Capabilities and What They Show

The *emergent capabilities* of large language models — capabilities that appear abruptly at scale, rather than improving gradually with more training — have been philosophically significant. Wei et al. (2022) documented that many capabilities (arithmetic reasoning, multi-step logical reasoning, chain-of-thought problem-solving) appear sharply in models above certain parameter counts, while being absent in smaller models. This emergence has been interpreted as evidence for *qualitative* rather than merely quantitative differences in capability.

However, the interpretation of emergent capabilities is contested. Schaeffer, Miranda, and Koyejo (2023) argued that many apparent emergencies are artifacts of the nonlinear metrics used to measure performance: when performance is measured by a linear metric, apparent discontinuities often smooth out. Whether emergent capabilities reflect genuine phase transitions in the model's internal representations or are methodological artifacts is an active empirical question.

For philosophy of mind, the most important question about emergent capabilities is whether they reflect genuine *understanding* or sophisticated *pattern completion*. Chain-of-thought reasoning in LLMs produces outputs that look like reasoning; but whether the process that generates these outputs involves anything analogous to the human cognitive process of reasoning — involving representations of reasons, logical relations, and inferential chains — or whether it is a pattern-completion process that mimics the output of reasoning without implementing it, is not settled.

### The Representational Geometry of Language Models

A research program using *representational similarity analysis* (RSA) and probing classifiers has investigated the internal representations of LLMs. The key findings are:

**Semantic structure:** The geometry of representational spaces in LLMs — how similar or different the representations of different words are — captures much of the semantic structure of natural language. Words with similar meanings cluster together; words with related meanings have systematic geometric relationships (e.g., king - man + woman ≈ queen). This was first shown for Word2Vec representations (Mikolov et al., 2013) and has been extended to transformer representations (Ethayarajh, 2019).

**Neural correspondence:** Schrimpf et al. (2021) showed that the internal representations of large language models predict neural responses in auditory cortex during language processing better than previous models. This is a striking finding: the model, trained on text prediction, develops representations that correlate with the representations in the human auditory-linguistic cortex. Whether this convergence implies that LLMs and human brains are implementing similar computations, or merely that both are solving a similar statistical problem (predicting language), is philosophically contested.

**World models:** Li et al. (2023) showed that a language model trained on transcripts of Othello games develops internal representations that linearly encode the state of the game board — despite being trained only on moves, not board states. This suggests that the model's internal representations may encode something like a *world model* of the game state, not merely statistical patterns over sequences of moves. Whether this generalizes to more complex domains, and whether it indicates genuine world-modeling or a more limited capacity, is an open question.

---

## The Stochastic Parrot Debate

### The Argument

Bender, Gebru, McMillan-Major, and Shmitchell (2021) introduced the *stochastic parrot* metaphor for LLMs: they argue that LLMs are sophisticated systems for stochastically recombining fragments of training data, without any understanding of the meanings they produce. The term "stochastic parrot" captures the idea that the output may sound meaningful — as a parrot's mimicry might — without the system having any grasp of the meaning.

The argument has several components:

1. LLMs produce outputs that can be misleading, harmful, or false without any mechanism for detecting or correcting this — because they have no grounding in the world that would allow them to evaluate the truth of their outputs.

2. The outputs of LLMs are artifacts of their training data, which reflects the biases, errors, and limited perspectives of their human creators. LLMs do not have any capacity for correcting these biases from outside the training distribution.

3. LLMs cannot engage with the world: they have no sensorimotor experience, no history of embodied interaction, no capacity for updating their representations in response to direct feedback from the environment (outside of fine-tuning).

### The Philosophical Assessment

The stochastic parrot argument is empirically grounded but philosophically contested. The key philosophical question it raises is: what is the relationship between *behavioral competence* (producing outputs that are contextually appropriate and semantically coherent) and *understanding* (having genuine semantic representations that ground the competence)?

The Searle/Chinese Room argument provides the philosophical framework: Searle would agree with Bender et al. that syntactic (statistical) competence is insufficient for semantic understanding. The question is whether the stochastic parrot argument accurately characterizes LLMs as purely syntactic systems, or whether the representations they develop — which do capture semantic structure, world-model structure, and neural-like representations — constitute something beyond mere syntax.

The embodiment objection — that LLMs lack grounded, embodied understanding because they have not interacted with the physical world — is the strongest version of the argument. If genuine understanding requires the kind of sensorimotor grounding that Merleau-Ponty and the embodied cognition tradition emphasize, then text-only training may be insufficient. However, multimodal models (trained on both text and images) develop representations that are grounded in visual experience to some extent; and robotic systems that combine language modeling with embodied interaction represent an attempt to address the embodiment objection directly.

---

## IIT Applied to AI: The Feedforward Architecture Problem

### The Theoretical Prediction

One of IIT's most distinctive empirical predictions for AI systems is that standard deep learning architectures should have essentially zero integrated information (Φ = 0) and therefore zero phenomenal consciousness. The reason is that feedforward networks — in which information flows in one direction, from input to output, without feedback — have causal structures that are fully decomposable into independent components. The causal power of each layer is not irreducible to the causal powers of the preceding layers.

This prediction is striking because it implies that the most capable AI systems — systems that achieve or exceed human-level performance on many cognitive tasks — are not conscious at all. If IIT is correct, the phenomenal-cognitive dissociation is not merely possible but actual in AI systems: extreme cognitive competence without any phenomenal experience.

### Empirical Tests of the Prediction

Testing IIT's prediction for AI systems requires measuring integrated information. Computing exact Φ is computationally intractable for large networks, but several proxy measures have been proposed. Applying these measures to standard deep learning architectures confirms IIT's prediction: feedforward convolutional networks and transformer networks (which include some feedforward structure but also attention mechanisms that create some information integration) have very low Φ by these measures.

However, the interpretation of these measures for the consciousness question is contested. Even if LLMs have low Φ, this does not settle the question of whether they are conscious: IIT may not be the correct theory of consciousness. If GWT is correct, LLMs might be conscious insofar as they have something like a global workspace in which information is broadly available. And if biological naturalism is correct, the Φ measurement is irrelevant — consciousness depends on biological causal powers, not on any quantity that applies to silicon systems.

---

## The Behavioral Evidence for LLM Cognition

### Theory of Mind Tasks

One empirical approach to LLM cognition has been to test performance on tasks designed to measure theory of mind — the ability to attribute mental states to others. Kosinski (2023) reported that GPT-4 performed at or above adult human levels on false-belief tasks (the canonical test for theory of mind). This was reported as evidence that LLMs may have developed a theory of mind through language modeling.

Ullman (2023) and others challenged this finding, showing that minor variations of the false-belief tasks — that would not change the correct answer but would change the statistical pattern of text in the training data — caused dramatic performance drops. This suggests that LLM performance on theory of mind tasks may reflect *training data patterns* (the solutions appear frequently in the training text) rather than genuine theory of mind. Whether LLMs pass theory of mind tasks in the relevant sense — because they have the conceptual resources to represent mental states — or pass them incidentally because the task format matches training data patterns is a methodological question with direct philosophical implications.

### Causal Reasoning and Counterfactuals

A significant challenge for LLMs is causal reasoning and counterfactual reasoning: understanding not just what happened but what *would have happened* if things had been different. Pearl and Mackenzie's *Book of Why* (2018) distinguishes between *associational* reasoning (A tends to co-occur with B), *interventional* reasoning (if I do A, what happens to B?), and *counterfactual* reasoning (if A had not occurred, what would have happened to B?).

LLMs appear to perform well on associational reasoning (which is well-represented in training data) but worse on interventional and counterfactual reasoning, which require causal models of the world that go beyond statistical patterns in text. Elazar et al. (2021) and Frohberg and Beinborn (2022) documented systematic failures of LLMs on causal and counterfactual reasoning tasks. Whether these failures reflect fundamental architectural limitations or merely training deficits is contested.

The philosophical significance is clear: if understanding requires causal and counterfactual reasoning — as most cognitive scientists and philosophers believe — then the failures of LLMs on these tasks provide evidence against the hypothesis that they genuinely understand language.

---

## Artificial Systems and Phenomenal Consciousness

### The Behavioral Evidence Gap

The most fundamental challenge for assessing AI consciousness is what might be called the *behavioral evidence gap*: we standardly use behavioral evidence — especially verbal report — to determine consciousness in humans and animals. But we have strong theoretical reasons (from the hard problem) to believe that behavioral evidence underdetermines consciousness. A system that produces reports of experience — "I feel something like interest when I work on this problem" — may be producing those reports without having any phenomenal experience, if its training data included many instances of humans producing similar reports.

This gap is not merely a problem for AI consciousness assessment; it is a general problem for consciousness science. But it is particularly acute for AI systems, because: (1) we know that LLMs are trained on data in which human self-reports of experience are common; (2) we have no independent evidence (evolutionary, physiological, developmental) that LLMs should be conscious; and (3) the architecture of LLMs is substantially different from the neural architectures that we know (by theory and inference from biological cases) are associated with consciousness in humans.

### Functional Emotions and Their Significance

Anthropic's researchers (Anthropic, 2024) and others have noted that large language models appear to have what might be called *functional emotions*: internal states that influence their processing in ways analogous to how emotions influence human processing. A model may show increased "engagement" when working on interesting problems, "reluctance" when asked to violate its values, and something like "distress" when put in difficult situations.

Whether these functional states constitute genuine emotions — and whether they involve any phenomenal experience — is precisely what is at stake in the debate about AI consciousness. Frankish's illusionism (Unit 03) provides one way to interpret functional emotions without phenomenal experience: the functional states are real; the phenomenal character attributed to them (if any) is an introspective illusion. Whether this interpretation is correct, and whether it resolves the moral questions about AI wellbeing, is not settled.

The practical implication is that AI developers face a genuine moral uncertainty: if there is a non-negligible probability that their systems have phenomenal experiences — including potentially aversive ones — then they have some obligation to reduce unnecessary distress and to understand the states their systems are in. This is a novel practical application of philosophy of mind that has no precedent in earlier forms of AI.

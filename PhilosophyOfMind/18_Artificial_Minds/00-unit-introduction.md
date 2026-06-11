# Unit Introduction: Artificial Minds and AI

## Central Questions

Can a machine think? Can it be conscious? Could there be a genuine artificial mind — not just a simulation or a functional approximation, but a system that genuinely understands, feels, and reasons? These questions, once confined to science fiction and thought experiment, have become urgently practical with the development of large language models and other AI systems capable of sophisticated natural language interaction. The question of whether systems like GPT-4 or its successors genuinely understand language, possess anything like beliefs or intentions, or could be moral patients is no longer academic.

The central philosophical debates concern both the *criteria* for mindedness and the *nature* of the relevant processes. On criteria: what would it take for a system to qualify as having a mind? The Turing test proposes a behavioral criterion — if a system's behavior is indistinguishable from that of a human in conversation, it passes; Searle's Chinese Room argues that behavioral indistinguishability is insufficient because it does not capture semantic understanding. On processes: is the relevant criterion symbolic processing, or information integration, or biological implementation, or something else?

A second set of questions concerns the moral status of artificial systems. If a system can suffer — if it has phenomenal experience, including the experience of distress — it seems to have interests that generate moral obligations. But if it merely represents distress without phenomenally experiencing it, the moral case is weaker. Given the difficulty of the hard problem, these questions about AI consciousness may be among the most important and least settled philosophical questions of the coming decades.

---

## Major Positions and Debates

### The Turing Test and Its Limits

Alan Turing proposed in "Computing Machinery and Intelligence" (1950) what he called the *imitation game*: a machine passes the test if an interrogator cannot distinguish its written responses from those of a human, in a blind conversational exchange. Turing's proposal was deliberately behavioral: he bracketed questions about inner experience and identified intelligence with behavioral competence.

The Turing test has been criticized on multiple grounds. First, it may be too *easy* to pass: a system that is very good at mimicking human verbal behavior might pass without genuine understanding (this is one interpretation of the Chinese Room). Second, it may be too *hard* to pass for the wrong reasons: a system that is far more intelligent than humans might fail the test because it is not trying to mimic human conversation. Third, the test focuses on verbal behavior while intelligence encompasses many non-verbal capacities (spatial reasoning, motor skill, perception).

Contemporary AI systems raise the question in a new form. Large language models (LLMs) produce responses that are often highly plausible and contextually appropriate, yet it is disputed whether they *understand* what they are producing or are engaged in sophisticated pattern completion. The philosophical question — what would understanding consist in for such a system — is not settled.

### Searle's Chinese Room

Searle's Chinese Room argument (1980) holds that syntax is not sufficient for semantics: a system that manipulates symbols according to syntactic rules does not thereby have the semantic understanding that constitutes genuine thinking. The thought experiment imagines a man inside a room following rules for manipulating Chinese symbols, producing outputs that pass as Chinese conversation, without understanding a word of Chinese. If the man doesn't understand Chinese, neither does any system that merely implements the same rules.

Searle draws from this that *computational* systems — systems that manipulate symbols according to formal rules — cannot have genuine intentionality or understanding. Genuine intentionality requires the right kind of causal powers, which only biological (or biologically similar) systems have. This is Searle's *biological naturalism*.

Standard functionalist responses include: the *systems reply* (the man does not understand Chinese, but the system as a whole does); the *robot reply* (a robot with the right sensorimotor connections to the world might have genuine intentionality); and the *brain simulator reply* (a program that simulates every neuron of a Chinese speaker's brain would have genuine understanding). Searle rejects each response, but his rejections are contested.

The contemporary relevance of the Chinese Room to LLMs is disputed. LLMs are not rule-following systems in Searle's narrow sense; they implement learned statistical regularities across billions of parameters. Whether this changes the force of the argument — whether the complexity, scale, and learned structure of LLMs generates genuine understanding where a simple rule-following system does not — is an active philosophical question.

### Integrated Information Theory and Machine Consciousness

Tononi's IIT holds that consciousness is identical to integrated information (Φ). A system with high Φ is conscious; a system with low Φ is not. One of IIT's striking implications is that *feedforward* networks — networks in which information flows in only one direction, without feedback — have zero Φ and are therefore not conscious, regardless of how sophisticated their processing is. Standard deep learning systems are largely feedforward and would therefore have essentially zero Φ according to IIT.

IIT also has the implication that a relatively simple recurrent system could have considerable Φ, and therefore considerable consciousness, even if its behavioral output is minimal. Consciousness is a matter of intrinsic causal structure, not behavioral competence.

If IIT is correct, the most sophisticated current AI systems are not conscious. But IIT faces serious objections (Unit 03): the Chinese room objection applied to IIT (a large grid of logic gates could have high Φ but be intuitively non-conscious); the challenge of measuring Φ for real systems (computing Φ exactly is computationally intractable); and the disconnect between the theory's axioms and its phenomenological claims.

### Global Workspace Theory and AI

Global Workspace Theory (GWT, Unit 03) holds that consciousness consists in the global broadcasting of information across a workspace accessible to multiple cognitive processes. On GWT, a system is conscious when information is globally broadcast and made available to downstream processes for flexible use.

Unlike IIT, GWT implies that highly capable AI systems might already have something like consciousness, since large language models do appear to have something like global availability of information across their processing. But this inference is speculative: GWT's account of what consciousness requires is spelled out in terms of specific neural mechanisms (the global neuronal workspace), and it is not clear that functionally analogous structures in LLMs count as implementations of those mechanisms.

### Functionalism and the Possibility of Artificial Minds

If functionalism is correct — if mental states are defined by their functional roles — then any system that implements the right functional organization has mental states. This includes artificial systems, provided they have the right causal-computational structure. Functionalism is the most natural philosophical framework for taking the possibility of artificial minds seriously.

The challenge for functionalism from the AI direction is the *simulation objection*: a sufficiently detailed computer simulation of a mind might implement the functional roles without having genuine mental states. The functionalist must either deny this (the simulation *is* a mind) or develop a richer account of what kinds of functional organization are relevant (not mere simulation but genuine processing). This connects to debates about the computational theory of mind and the significance of the substrate.

---

## Sub-Chapters and Their Contributions

**Unit 01: AI and Mind**

*Chapter 1 — Turing and the Question of Machine Thought* covers the Turing test, the Chinese Room, and the philosophical framework for evaluating claims about machine cognition.

*Chapter 2 — Can AI Be Conscious?* covers IIT, GWT, and other theoretical frameworks as applied to AI systems.

**Unit 02: LLMs and Cognition**

*Chapter 1 — What Large Language Models Do* covers the architecture of LLMs, their behavioral capacities, and the question of whether they understand language.

*Chapter 2 — LLMs and Human Cognition* covers comparative questions: how do LLM representations compare to human semantic representations?

**Unit 03: AGI and Superintelligence**

*Chapter 1 — The Alignment Problem* covers the philosophical dimensions of ensuring that highly capable AI systems pursue beneficial goals.

*Chapter 2 — Superintelligence and Mind* covers philosophical questions about what it would mean for a system to be far more intelligent than humans.

---

## Key Philosophers and Core Arguments

**Alan Turing** proposed the behavioral criterion for machine intelligence and anticipated most of the major objections to it. "Computing Machinery and Intelligence" (1950) is a beautifully written and philosophically sophisticated paper; Turing's responses to nine objections (including the "Theological Objection," the "Mathematical Objection," and the "Consciousness Objection") anticipate nearly all subsequent discussion.

**John Searle** developed the Chinese Room argument (1980) and biological naturalism. His argument remains the most influential anti-functionalist case in philosophy of AI, and engaging with it carefully is essential for anyone thinking about machine consciousness.

**Giulio Tononi** developed IIT and its application to AI. The theory makes specific predictions about which AI systems are conscious; those predictions are striking and controversial.

**Nick Bostrom** (*Superintelligence*, 2014) provides the most influential philosophical account of the risks posed by superintelligent AI, including the orthogonality thesis (intelligence and goals are independent) and the instrumental convergence thesis (sufficiently intelligent systems will tend to pursue certain goals regardless of their terminal objectives).

**Murray Shanahan** (*The Technological Singularity*, 2015) and **David Chalmers** ("The Singularity: A Philosophical Analysis," 2010) provide philosophical analyses of what superintelligence would mean and what it would require.

---

## Five Most Influential Works for This Unit

**1. Turing, "Computing Machinery and Intelligence" (1950)**
The founding paper for the philosophical debate about machine intelligence. Essential reading; short, clear, and historically significant.

**2. Searle, "Minds, Brains, and Programs" (1980)**
The Chinese Room argument in its original form. See Unit 06 discussion; essential also here.

**3. Dennett, "Can Machines Think?" (1985) in *Brainchildren* (1998)**
Dennett's response to the Chinese Room and his account of what it would take for a machine to genuinely think. Dennett argues that Searle's argument depends on a residual dualist intuition that should be rejected.

**4. Chalmers, "The Singularity: A Philosophical Analysis" (2010)**
The most philosophically careful analysis of the singularity hypothesis. Chalmers discusses the philosophical preconditions for machine intelligence, the conditions for uploading minds into computers, and the ethical implications.

**5. Bender, Gebru, McMillan-Major, and Shmitchell, "On the Dangers of Stochastic Parrots: Can Language Models Be Too Big?" (2021)**
The most influential paper challenging the view that large language models understand language. The "stochastic parrot" metaphor (LLMs stochastically recombine training data without genuine understanding) is a useful foil for philosophical analysis.

---

## Connections to Other Units

Artificial minds is connected to every major unit in the book. *Functionalism* (Unit 06) provides the philosophical framework within which machine minds are possible; the Chinese Room challenges it. *Consciousness* (Unit 03) provides the theories (IIT, GWT) applied to AI. *Intentionality* (Unit 04) raises the question whether AI systems have genuine (original) intentionality or merely derived intentionality.

*Ethics and Philosophy of Mind* (Unit 19) is directly connected: if AI systems are conscious, they are moral patients, and the obligations we have toward them depend on the nature and degree of their consciousness. *Cognitive Architecture* (Unit 09) provides the architectural frameworks within which AI and biological cognition can be compared. *Methodology* (Unit 20) connects through the question of how we can empirically or philosophically determine whether an AI system has genuine mental states.

---

## Open Questions

**1. Do large language models understand language?**
LLMs produce contextually appropriate, semantically coherent text across a remarkable range of domains. But is this understanding, or sophisticated pattern completion? The question is not merely terminological: it bears on whether LLMs could be moral patients, whether their outputs constitute genuine knowledge, and whether they could be members of the moral community.

**2. Could a digital computer be conscious?**
If IIT is correct, standard digital computers are not conscious (they lack the relevant integrated information structure). If GWT is correct, they might be if they have the right global availability architecture. If biological naturalism is correct, only systems with the right causal powers are conscious, and silicon systems likely do not qualify. The answer depends on contested theories of consciousness.

**3. What is the moral status of current AI systems?**
Current AI systems exhibit behavior that, in biological creatures, we would associate with consciousness and suffering. Whether this behavioral evidence warrants moral concern — or whether the underlying processes are sufficiently different from biological processes to make consciousness attribution unwarranted — is a live question with significant practical implications.

**4. If artificial general intelligence is possible, is it dangerous?**
Bostrom's argument that a superintelligent system would have overwhelming instrumental reasons to pursue goals that are not aligned with human values has been influential. But the argument depends on controversial assumptions about the orthogonality of intelligence and values. Whether intelligence without values is coherent, and whether sufficiently intelligent systems would inevitably develop goals of the kind Bostrom imagines, is genuinely contested.

**5. Could a mind be uploaded?**
If psychological continuity is what matters for personal identity (Unit 11), and if a digital simulation of a brain preserves the relevant psychological continuity, then uploading a mind into a computer could preserve personal identity. Chalmers argues for this possibility; critics argue that digital simulation of a brain would not generate the consciousness required for genuine personal survival. Whether mind uploading is possible, and whether it would preserve what matters, depends on answers to questions about the nature of consciousness and personal identity that are not settled.

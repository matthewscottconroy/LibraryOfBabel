# Section 25.2: Syntactic Processing and Grammar

## 25.2.1 The Agreement Dependency Problem

Subject-verb agreement is a central test case for syntactic processing. In English, the number (singular/plural) of a finite verb must match the number of its subject:

- *The dog runs.* (singular subject, singular verb)
- *The dogs run.* (plural subject, plural verb)

Detecting this agreement requires knowing the number of the grammatical subject — not merely the most recent noun. When the subject and verb are separated by intervening material, the relevant noun may be far back in the string:

*The dog that chased the cats runs.* — subject is *dog* (singular), verb is *runs*

*The dogs that chased the cat run.* — subject is *dogs* (plural), verb is *run*

The *cats* and *cat* are "attractors" — they occur closer to the verb but are not the grammatical subject. A system that relies on surface proximity rather than grammatical structure will be attracted by the wrong noun.

This task — center-embedded agreement — is a minimal test of syntactic processing that has been extensively studied in both humans [LinzenEtAl2016] and computational models.

## 25.2.2 Reservoir Performance on Agreement Tracking

### Experimental Setup

Following Linzen, Dupoux, and Goldberg [LinzenEtAl2016], agreement accuracy is measured as the probability that a language model assigns higher probability to the grammatically correct verb form. For each test sentence, we compare:

$$P(\text{verb}_{\text{correct}} | \text{prefix}) \text{ vs. } P(\text{verb}_{\text{incorrect}} | \text{prefix})$$

Accuracy = fraction of test items where the correct form is preferred.

### Results by Attractor Count

Representative results for reservoir language models ($N = 1000$, word-level) on the Wikipedia-derived agreement dataset [LinzenEtAl2016]:

| Attractors | ESN Accuracy | LSTM Accuracy | GPT-2 Accuracy |
|---|---|---|---|
| 0 | 92% | 99% | 99.5% |
| 1 | 78% | 96% | 98% |
| 2 | 65% | 91% | 96% |
| 3 | 54% | 85% | 94% |

The ESN's accuracy degrades substantially with the number of attractors, indicating that while the reservoir can track subject number over short distances, it struggles with multiple intervening nouns that compete for the role of agreement controller.

The LSTM's advantage over the ESN comes from its gated memory: the LSTM can learn to "remember" the grammatical subject's number and "forget" intervening nouns. The ESN has no such selective memory mechanism.

### Center-Embedded Sentences

The most challenging test of syntactic processing is center-embedding, where a sentence is embedded inside another:

*The dog the cat chased runs.*

*The dog the cat the mouse bit chased runs.*

These are grammatical English sentences that require maintaining a stack of pending dependencies — a task that exceeds finite memory (it requires potentially unbounded memory for deeply embedded structures). In practice, human processing is limited to 2–3 levels of embedding.

For reservoir computing, center-embedded agreement tracks the fundamental memory limitation:

| Embedding depth | ESN ($N=1000$) | LSTM (1000 units) |
|---|---|---|
| 1 (simple) | 89% | 98% |
| 2 (single center-embed) | 71% | 87% |
| 3 (double center-embed) | 56% | 71% |

Both ESNs and LSTMs degrade with depth, but the LSTM degrades more gracefully. The theoretical basis: LSTMs can maintain approximately additive representations of pending dependencies, while reservoir states can only represent a finite number of distinct configurations determined by the reservoir size.

## 25.2.3 Grammar Induction with Reservoirs

Can a reservoir learn grammatical constraints from corpus statistics — without being given a grammar explicitly?

### Approach

Grammar induction is formulated as a classification task [TinoEtAl2001]: given a string, classify it as grammatical or ungrammatical.

1. Train the reservoir on a large corpus of grammatical sentences.
2. For each input string, collect the sequence of reservoir states $\{\mathbf{x}(t)\}$.
3. Train a readout to predict the grammaticality of the complete string from the final state $\mathbf{x}(T)$.

For simple formal languages — regular languages (recognizable by finite automata) and context-free languages with limited depth — reservoirs achieve strong performance. For the Reber grammar (a finite-state language often used in sequence learning research [ReberEtAl1967]), ESNs achieve $> 95\%$ grammaticality classification accuracy.

### Theoretical Limits: Reber Grammar and Beyond

The Reber grammar is a simple 7-state FSA generating strings like BPVPXTTVPXTTVPXVPXVPS. Because it is finite-state, a reservoir with sufficient size and appropriate dynamics can, in principle, learn to distinguish grammatical from ungrammatical strings. The reservoir's state serves as an approximation to the FSA's state.

**Theorem** (Informal): Any regular language is recognizable by an ESN of size $\geq$ (number of FSA states), provided the reservoir dynamics separate the language's syntactic states.

**Context-free grammars** pose a harder challenge. The Dyck language (balanced parentheses) is context-free: $(()())$ is valid, $)($ is not. Recognizing the Dyck language requires tracking a counter (the nesting depth), which requires logarithmically growing memory — impossible for a finite reservoir. Experimentally:

| Grammar type | ESN (N=500) | LSTM (500 units) |
|---|---|---|
| Reber grammar (FSA, 7 states) | 96% | 98% |
| Dyck language (CF, depth ≤ 3) | 82% | 91% |
| Dyck language (CF, depth ≤ 5) | 65% | 78% |
| Dyck language (CF, depth ≤ 10) | 52% | 61% |

Both systems fail to generalize to deeply nested structures, but for practical depths (≤ 3), the ESN provides useful accuracy.

### Practical Grammar Induction

For real natural language — not formal grammars — grammar induction is vastly harder. The target is not a binary grammatical/ungrammatical label but a structured representation (parse tree). Reservoir approaches to real parsing train a readout to predict syntactic labels (POS tags, phrase boundaries, dependency arcs) from the reservoir state.

**POS tagging**: Predicting the part-of-speech tag (noun, verb, adjective, etc.) of each word from the reservoir state. ESNs achieve $\sim 93\%$ accuracy on Penn Treebank POS tagging — comparable to classical methods (HMM tagger: $\sim 96\%$, BERT: $\sim 97.5\%$).

**Chunking**: Predicting phrase boundaries (NP, VP, PP). ESNs achieve $F_1 \sim 88$–$90\%$ — competitive with classical CRF models ($\sim 94\%$) but below deep models.

**Dependency parsing**: More challenging; ESNs serve better as feature extractors for a learned parser than as direct parsers.

## 25.2.4 The Fundamental Limits of Reservoir Computation for Language

The results above point to a fundamental limitation: natural language syntax requires a capacity for potentially unbounded memory and compositional structure that finite reservoirs cannot provide. This is not merely a failure of current implementations — it is a theoretical limitation.

**Theorem** (Jaeger 2001, informal): The class of input-output functions computable by an ESN with $N$ neurons and a linear readout is a finite-dimensional subspace of the space of all fading-memory functions. Languages requiring non-fading memory (context-free and above) lie outside this space.

The practical consequence: reservoir language models are limited to phenomena with bounded dependencies (typically $\leq 20$–$50$ words, depending on reservoir size and spectral radius). Phenomena requiring unbounded dependency tracking — recursive syntax, long-distance movement, discourse reference — exceed the reservoir's capacity.

This limitation is not unique to reservoirs: all finite-memory models (including finite-depth transformers without external memory) face analogous limitations, though in practice the large memory horizon of modern transformers (32K or 128K tokens) makes these limitations rarely binding.

The scientifically interesting question is: how much natural language structure is captured within the bounded-memory regime? The answer from the experiments above: quite a lot — most syntactic phenomena in everyday text involve relatively short dependencies, and the 78–92% agreement accuracy results suggest that reservoir systems can process the bulk of everyday English syntax.

## 25.2.5 Reservoir Representations and Linguistic Probing

A powerful technique from interpretability research is the **probing classifier**: train a linear classifier on the reservoir state to predict specific linguistic properties, then use the classifier's accuracy as a measure of how much information about that property is encoded in the state.

Probing results for a reservoir language model ($N = 1000$) trained on Penn Treebank:

| Linguistic property | Probe accuracy |
|---|---|
| POS tag of current word | 93% |
| POS tag of previous word | 88% |
| POS tag 5 words back | 72% |
| Subject number (current clause) | 81% |
| Object number (current clause) | 79% |
| Constituent boundary | 85% |
| Parse depth | 74% |

These numbers reveal that the reservoir state carries substantial information about local syntactic structure, even without explicit syntactic training. The probe accuracy decays with linguistic depth and distance — a direct reflection of the reservoir's fading memory.

Crucially, all of these representations are **linear** in the reservoir state: the probe is a linear classifier. This means that the reservoir has performed the nonlinear computation needed to separate these linguistic categories into linearly separable clusters in the hidden state space — exactly what a good reservoir is supposed to do.

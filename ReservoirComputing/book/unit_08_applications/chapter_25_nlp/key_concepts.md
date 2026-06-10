# Chapter 25: Key Concepts

## Language Model

A probability distribution over sequences of words (or characters): $P(w_1, \ldots, w_T) = \prod_t P(w_t | w_1, \ldots, w_{t-1})$. The conditional distributions $P(w_t | \mathbf{w}_{<t})$ are estimated from corpus statistics. A reservoir language model uses the reservoir state $\mathbf{x}(t)$ as a continuous representation of the prefix history, with a linear readout + softmax mapping to the next-word probability distribution.

## Perplexity

The standard language model evaluation metric: $\text{PPL} = \exp(-\frac{1}{N}\sum_t \log P(w_t | \mathbf{w}_{<t}))$. Measures the geometric mean surprise per word. Lower is better. Intuition: a model with perplexity $k$ is as uncertain as if it were choosing uniformly among $k$ equally likely options at each word. Human-level perplexity on Penn Treebank: ~70–90; reservoir LMs: ~105–140; state-of-the-art LLMs: ~20–35.

## Penn Treebank (PTB)

A standard benchmark corpus for language modeling and syntactic analysis. Wall Street Journal text with gold parse trees and POS tags, divided into standard train/validation/test splits. Vocabulary size 10,000 after preprocessing. Test set: 82,430 words. The most widely used benchmark for comparing language model perplexities.

## Agreement Tracking

The task of predicting whether a verb's number (singular/plural) agrees with its grammatical subject, across intervening nouns that might attract incorrect agreement (attractor nouns). A benchmark for syntactic processing in language models [LinzenEtAl2016]. ESNs achieve ~78% accuracy with 1 attractor, degrading to ~54% with 3 attractors, compared to LSTM's ~96% and ~85% respectively.

## Center-Embedded Sentence

A sentence where a clause is syntactically embedded within another: "The dog the cat chased runs." Requires tracking multiple pending dependencies simultaneously, stress-testing the working memory of any sequential model. Reservoir performance degrades sharply with embedding depth because the number of simultaneously active dependencies exceeds the reservoir's effective state capacity.

## Probing Classifier

A linear classifier trained to predict a specific linguistic property (POS tag, parse depth, syntactic role) from a model's hidden state. Probing accuracy measures how much information about the target property is linearly accessible in the representation. A useful interpretability tool: if a reservoir language model's hidden state allows probing to predict the grammatical subject's number with 81% accuracy, then the reservoir has implicitly computed a representation that separates singular from plural subjects.

## Reber Grammar

A finite-state grammar (7 states) over the alphabet $\{B, P, S, T, V, X\}$ that generates strings like BPVPXTTVPS. Widely used as a benchmark for sequence learning in recurrent neural networks. ESNs achieve >95% grammaticality classification accuracy on Reber grammar, because it is a regular language and reservoir dynamics can approximate finite automata.

## Attractor Noun

In the context of subject-verb agreement tracking, a noun that appears between the grammatical subject and the verb and has opposite number from the subject. Example: "The dogs that chased the cat run" — "cat" (singular) is an attractor for the verb "run" that should agree with "dogs" (plural). Models that rely on surface proximity rather than grammatical structure are incorrectly "attracted" to the wrong noun.

## Grammar Induction

Learning grammatical constraints from a corpus without explicit specification of the grammar. For reservoir computing, this is formulated as: train a language model on grammatical text, then use the model's probability assignments to distinguish grammatical from ungrammatical strings. Reservoirs can successfully induce regular language grammars and shallow context-free constraints (depth ≤ 2–3), but fail on deeply recursive context-free languages.

## Dyck Language

The set of strings of balanced parentheses: $D_k$ uses $k$ types of brackets and requires each opening bracket to be matched by the same type of closing bracket. $D_1$ (single bracket type) is a prototypical context-free language requiring an unbounded counter (nesting depth). Reservoir models trained on bounded-depth Dyck languages fail to generalize to deeper nesting, indicating that they learn statistical regularities of the training distribution rather than the underlying recursive rule.

## Bits-per-Character (BPC)

An alternative to perplexity for character-level language models: $\text{BPC} = -\frac{1}{N}\sum_t \log_2 P(c_t | c_1, \ldots, c_{t-1})$. Equivalent to per-character cross-entropy in bits. Lower is better. Human-level BPC for English is approximately 1.0–1.3 (from compression studies). Reservoir character LMs typically achieve BPC ~1.4–1.6 on Wikipedia-derived benchmarks.

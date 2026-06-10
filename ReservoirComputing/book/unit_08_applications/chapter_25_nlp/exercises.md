# Chapter 25 Exercises

## Language Modeling

**25.1** (Perplexity Computation). A character-level reservoir language model assigns the following log-probabilities to a test sequence "hello" (5 characters): $\log P(h) = -2.1$, $\log P(e|h) = -1.8$, $\log P(l|he) = -1.2$, $\log P(l|hel) = -1.0$, $\log P(o|hell) = -1.9$.

(a) Compute the perplexity of this sequence.

(b) If the vocabulary has $V = 27$ characters (26 letters + space) and each character is equally probable under a uniform model, what is the uniform model's perplexity?

(c) The entropy of English text is approximately 1.1 bits per character. What is the corresponding perplexity? How does the reservoir model compare?

**25.2** (Reservoir LM Implementation). Train the provided `ReservoirCharLM` on a corpus of your choice (e.g., Gutenberg Project novels, available at gutenberg.org).

(a) Report test perplexity for reservoir sizes $N \in \{100, 500, 1000, 2000\}$. Plot perplexity vs. $N$.

(b) Compare to a 2-gram character model (Laplace smoothing) and a 5-gram model. Which $N$ does the reservoir beat the 5-gram model?

(c) Generate 300 characters from the reservoir model (temperature = 1.0) and 300 characters from a 5-gram model. Qualitatively compare the outputs. Which looks more like natural text?

**25.3** (Word Embeddings vs. One-Hot). Modify the `ReservoirCharLM` to work at the word level (word-level language model).

(a) Using one-hot input encoding for a vocabulary of $V = 5000$ words: what is the size of $W_{\text{in}}$? What fraction of a 1 GB memory limit does this consume?

(b) Replace one-hot encoding with pre-trained 100-dimensional GloVe embeddings. What is the new size of $W_{\text{in}}$? How does this affect model training and test perplexity?

(c) Explain conceptually why word embeddings should help for rare words. Does the perplexity improvement concentrate on rare words (check by computing perplexity separately for high-frequency and low-frequency test words)?

## Syntactic Processing

**25.4** (Agreement Tracking — Theory). Consider a finite-state language in which every sentence has the structure NP_subject VP_verb, where the subject NP may contain an intervening relative clause with a local NP:

Structure 1 (0 attractors): $[\text{NP}_1]\ V_1$ (e.g., "dogs run")

Structure 2 (1 attractor): $[\text{NP}_1\ [\text{NP}_2\ V_2]]\ V_1$ (e.g., "dogs that cats bite run")

(a) Show that structure 1 requires only a 2-state memory (singular/plural) to track agreement.

(b) Show that structure 2 requires only a 4-state memory (combinations of singular/plural for $\text{NP}_1$ and $\text{NP}_2$). 

(c) A reservoir of size $N$ can, in principle, represent $2^N$ distinct states. How small can $N$ be while still supporting agreement tracking for up to $k$ attractors? (Assume the reservoir perfectly classifies states.)

(d) In practice, reservoirs achieve < 100% accuracy on agreement tracking. Identify two properties of the reservoir dynamics that limit accuracy beyond pure state capacity.

**25.5** (Center-Embedding Experiment). Using the reservoir character LM from the provided code (adapted to word level):

(a) Train on Penn Treebank and evaluate agreement accuracy following the Linzen et al. [LinzenEtAl2016] protocol (available at https://github.com/TalLinzen/rnn_agreement).

(b) Compare to a baseline that always predicts the verb agrees with the nearest preceding noun.

(c) Compute accuracy separately for sentences with 0, 1, 2, and 3 attractors. Does the decay match the pattern in Table 25.2.2?

(d) Propose a reservoir modification that would improve performance specifically on high-attractor sentences. (Hint: think about what information the reservoir needs to maintain and how the architecture could support it.)

**25.6** (Probing Classifiers). Train a reservoir language model on a small corpus with syntactic annotations (Penn Treebank, if available, or a synthetic grammar).

(a) For each position $t$ in each sentence, collect the reservoir state $\mathbf{x}(t)$.

(b) Train a logistic regression probe to predict: (i) POS tag of word $t$, (ii) POS tag of word $t-2$, (iii) whether position $t$ is inside an NP.

(c) Report accuracy for each probe. Does POS prediction accuracy at lag $t-2$ correlate with the reservoir's spectral radius (try $\rho \in \{0.7, 0.9, 0.99\}$)?

(d) Visualize the reservoir states using 2D PCA, coloring points by POS tag. Are noun states linearly separable from verb states?

## Formal Language Theory

**25.7** (Reber Grammar). The Reber grammar generates strings over the alphabet $\{B, P, S, T, V, X\}$ using the automaton with states $\{q_0, \ldots, q_6\}$ (look up or draw the standard Reber automaton).

(a) Generate 1000 grammatical strings and 1000 ungrammatical strings (random permutations of the same character frequencies). What is the theoretical minimum string length needed to distinguish them?

(b) Train a reservoir classifier ($N = 200$) to classify grammatical vs. ungrammatical strings. Report accuracy.

(c) Train a linear (no reservoir) classifier using only character frequency features. Compare accuracy. What does this reveal about the temporal structure needed for Reber classification?

(d) Use probing classifiers to determine whether the reservoir state at each position encodes the FSA state (7 possible values). Plot probe accuracy as a function of position within the string.

**25.8** (Limits of Reservoir Computation). The Dyck language $D_k$ consists of balanced strings over $k$ types of brackets. $D_1$ uses only $\{(, )\}$; $D_2$ uses $\{(,), [, ]\}$, etc.

(a) Show that $D_1$ is not regular (use the pumping lemma for regular languages).

(b) For $D_1$ limited to depth $\leq d$ (maximum nesting depth), show that this restricted language IS regular (construct the FSA).

(c) Train reservoirs of sizes $N \in \{50, 100, 200, 500\}$ to classify $D_1$ strings of depth $\leq 3$ vs. ungrammatical strings. How large must $N$ be to achieve 90% accuracy? Is there a threshold behavior?

(d) Now test each trained model on $D_1$ strings of depth $\leq 5$ (not seen during training). Does accuracy generalize? What does failure to generalize tell you about what the reservoir has learned?

## Advanced Exercises

**25.9** (Hierarchical ESN for Long-Range Dependencies). The standard ESN struggles with dependencies longer than $\tau_{\text{mem}} \approx 1/\alpha$ steps. A hierarchical ESN [GallicchioMicheli2017] with multiple leaking rates can capture multiple timescales simultaneously.

(a) Design a 3-layer hierarchical ESN with $\alpha_1 = 0.7$, $\alpha_2 = 0.3$, $\alpha_3 = 0.05$ and layers of size $N_1 = N_2 = N_3 = 100$. What are the effective memory horizons of each layer?

(b) Train this hierarchical ESN as a language model. Does it achieve lower perplexity than a flat ESN with $N = 300$ and $\alpha = 0.3$?

(c) Run agreement tracking experiments on the hierarchical model. Does the presence of a slow layer (small $\alpha_3$) improve performance on high-attractor sentences?

**25.10** (Reservoir vs. LSTM: A Theoretical Analysis). The LSTM has forget gate $f_t = \sigma(W_f[h_{t-1}, x_t] + b_f)$ and cell state update $c_t = f_t \odot c_{t-1} + i_t \odot \tilde{c}_t$.

(a) For the agreement tracking task, describe qualitatively how a trained LSTM would use its forget gate to "ignore" attractor nouns and "remember" the subject noun.

(b) What is the analog of the forget gate in an ESN? (Hint: the leaking rate $\alpha$ provides a fixed forgetting schedule.) How does the fixed nature of this forgetting limit the ESN's ability to perform selective memory?

(c) Propose a "gated reservoir" in which a learned gate $g_t = \sigma(W_g \mathbf{x}(t) + W_g^{\text{in}} u(t))$ modulates the leaking rate: $\mathbf{x}(t) = (1 - g_t \alpha)\mathbf{x}(t-1) + g_t \alpha \tanh(\cdots)$. Note that training $W_g$ requires gradient propagation through time, departing from the standard RC paradigm. Would this gated reservoir be theoretically superior to a standard ESN for agreement tracking? What would be the cost?

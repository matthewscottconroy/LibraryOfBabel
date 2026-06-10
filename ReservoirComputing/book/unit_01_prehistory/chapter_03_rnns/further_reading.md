# Chapter 3 Further Reading

## Essential Papers

---

**Hochreiter, S., & Schmidhuber, J. (1997). Long short-term memory. *Neural Computation*, 9(8), 1735–1780.**

This is the paper that changed the practical trajectory of recurrent neural networks. Hochreiter and Schmidhuber begin with a rigorous mathematical analysis of the vanishing gradient problem — building directly on Hochreiter's 1991 diploma thesis — and then present the LSTM architecture as a principled solution. The analysis is sharp: they prove that standard gradient descent in deep networks causes either vanishing or exploding gradients, and they show that the LSTM's gating mechanism (specifically the "constant error carousel" through the cell state) provides a remedy. The paper is long, detailed, and mathematically careful in a way that was unusual for the field. Read the first half carefully, working through the gradient analysis; the second half (experiments) is also worth reading for the benchmark results on tasks that specifically probe long-range memory. This paper is not easy, but it rewards effort: the mathematical argument is essentially complete, and it foreshadows nearly everything that has been written about gradient flow in deep networks since.

---

**Bengio, Y., Simard, P., & Frasconi, P. (1994). Learning long-term dependencies with gradient descent is difficult. *IEEE Transactions on Neural Networks*, 5(2), 157–166.**

The definitive treatment of the vanishing gradient problem as a mathematical phenomenon, independent of the specific RNN architecture. Bengio, Simard, and Frasconi characterize the conditions under which gradient-based learning fails for long-range dependencies, proving the exponential decay result rigorously and providing a clear analysis of why the problem cannot be resolved within the standard gradient descent framework without architectural changes. The paper also discusses the relationship between the spectral conditions for gradient propagation and the spectral conditions for dynamical stability — identifying the fundamental dilemma (stable dynamics = vanishing gradients) that LSTM partially resolves through gating. This should be read alongside or before the LSTM paper; it provides the theoretical foundation that makes the LSTM's design choices legible.

---

**Werbos, P. J. (1990). Backpropagation through time: What it does and how to do it. *Proceedings of the IEEE*, 78(10), 1550–1560.**

The canonical reference for BPTT. Werbos explains the algorithm with unusual clarity, connects it to optimal control theory and dynamic programming, and derives the gradient equations in the general form that remains standard. Of particular value is Werbos's discussion of the relationship between BPTT and the adjoint method in control theory, which makes clear that BPTT is not a trick invented for neural networks but an application of a much older idea (the adjoint or costate equations for optimal control, going back to Pontryagin). This contextualization is important for understanding why BPTT is correct — it follows from basic calculus, not neural network heuristics. Werbos is also characteristically generous in his attribution, explicitly noting that the same method was independently discovered by others.

---

## Books

**Goodfellow, I., Bengio, Y., & Courville, A. (2016). *Deep Learning*. MIT Press. (Chapter 10: Sequence Modeling — Recurrent and Recursive Nets)**

The standard graduate textbook for deep learning. Chapter 10 covers RNNs, BPTT, the vanishing gradient problem, LSTM, and GRU with characteristic thoroughness and clarity. The treatment of BPTT is especially well-organized, with clear notation and step-by-step derivations that complement those in this chapter. The discussion of "echo state networks" in Section 10.9 provides a first view of the reservoir computing paradigm from a deep learning perspective, though it is brief. Available free online at deeplearningbook.org.

---

**Kolen, J. F., & Kremer, S. C. (Eds.). (2001). *A Field Guide to Dynamical Recurrent Networks*. IEEE Press.**

A collection of review articles and original contributions that covers the state of RNN research around 2001 — exactly at the transition point between the "classical" era of RNN research and the emergence of reservoir computing. Includes chapters on BPTT, RTRL, gradient problems, LSTM, and several less-well-known training algorithms. Kolen's chapter on the gradient flow in RNNs is particularly recommended: it provides a geometric, intuitive account of the problem that complements the algebraic derivations in this chapter. The book as a whole is a useful historical document, capturing what was known (and what was frustrating) just as the reservoir computing paradigm emerged.

---

**Siegelmann, H. T. (1999). *Neural Networks and Analog Computation: Beyond the Turing Limit*. Birkhäuser.**

A mathematically sophisticated treatment of the computational power of recurrent neural networks. Siegelmann proves that RNNs with rational weights are Turing-complete and that RNNs with real-valued weights (or even simply with irrational weights) are super-Turing. This is both theoretically interesting and somewhat sobering: the full computational power of RNNs is so great that training them well is unsurprisingly difficult. The book provides the formal foundation for understanding why RNNs are powerful in principle, which motivates the entire discussion of why they are hard to train in practice.

---

## Historical Papers

**Williams, R. J., & Zipser, D. (1989). A learning algorithm for continually running fully recurrent neural networks. *Neural Computation*, 1(2), 270–280.**

The RTRL paper. Williams and Zipser develop an online alternative to BPTT that computes exact gradients causally — without needing to see the future. The algorithm maintains the sensitivity matrix $\partial \mathbf{x}_t / \partial W^{rec}$ at every step, updating it recursively as new inputs arrive. The paper is mathematically clean and careful, and the derivation of RTRL is worth working through alongside BPTT to understand the relationship between the two: RTRL and BPTT compute the same gradient by accumulating it in different orders (forward vs. backward in time). The $O(N^4)$ cost per step is prohibitive at scale, but RTRL remains important as a conceptual reference point and as a biologically plausible candidate for temporal credit assignment.

---

**Elman, J. L. (1990). Finding structure in time. *Cognitive Science*, 14(2), 179–211.**

The paper that introduced the Simple Recurrent Network (SRN) and demonstrated — through experiments on synthetic language data — that recurrent networks could learn grammatical structure implicitly from sequence statistics. Elman's approach was empirical and cognitive-scientific: he was interested in what the network learned, not just whether it performed well. The analyses of internal state representations (PCA on hidden units, hierarchical clustering of activation patterns) are early examples of what is now called "representation analysis" and anticipate the modern field of mechanistic interpretability. This paper is important reading for anyone interested in the cognitive science and linguistic applications of recurrent networks.

---

**Hochreiter, S. (1991). Untersuchungen zu dynamischen neuronalen Netzen. Diploma thesis, Technische Universität München.**

Hochreiter's diploma thesis — unpublished and in German — contains the first complete, rigorous analysis of the vanishing gradient problem in recurrent networks. Schmidhuber's group has made a translation and scan available; the relevant sections have been partially translated and are summarized in the 1997 LSTM paper's mathematical background. For readers who want the original derivation in all its detail, this is the source. It is also historically important as a document of what could be derived and understood by a master's student in 1991 — six years before publication — which speaks to how long the problem was known before the solution was made public.

---

**Cho, K., van Merrienboer, B., Gulcehre, C., Bahdanau, D., Bougares, F., Schwenk, H., & Bengio, Y. (2014). Learning phrase representations using RNN encoder–decoder for statistical machine translation. *EMNLP*, 1724–1734.**

The paper that introduced the Gated Recurrent Unit (GRU) as a simplified alternative to LSTM. The GRU appeared as a side contribution in a paper whose main focus was the encoder-decoder architecture for machine translation (itself one of the foundational architectures of modern NLP). The GRU is presented as a "less complex" alternative to LSTM that achieves comparable performance with fewer parameters. The motivation for the simplification is discussed clearly, and the comparison between LSTM and GRU performance is instructive. This paper also contains, in the encoder-decoder architecture, one of the first clear formulations of using a recurrent network to map a variable-length input sequence to a fixed-length representation (the encoder state) and then back to a variable-length output sequence — a structure that underlies much of modern sequence-to-sequence learning, including attention mechanisms and transformers.

---

**Pascanu, R., Mikolov, T., & Bengio, Y. (2013). On the difficulty of training recurrent neural networks. *Proceedings of ICML*, 1310–1318.**

A modern, comprehensive treatment of the vanishing and exploding gradient problems in RNNs, with clear derivations, practical analysis, and the proposal of gradient clipping as a remedy for exploding gradients. The paper is particularly valuable for its empirical investigations: the authors visualize loss landscapes, measure gradient norms across depths, and show explicitly what "gradient explosion" looks like in practice. The proposal to clip gradients by their global norm (rather than element-wise) is both simple and effective, and remains standard practice in RNN training. The paper also provides a clear discussion of why the vanishing gradient problem is fundamentally harder to address than the exploding gradient problem: clipping addresses the latter, but no simple intervention addresses the former — which requires architectural changes (LSTM, GRU) or a fundamentally different training approach.

---

**Rumelhart, D. E., Hinton, G. E., & Williams, R. J. (1986). Learning representations by back-propagating errors. *Nature*, 323, 533–536.**

The popularization paper for backpropagation. While not primarily about recurrent networks, this paper's impact on the field cannot be overstated: it introduced the backpropagation algorithm to a wide scientific audience with compelling demonstrations of what it could learn (XOR, encoding/decoding tasks, analogical reasoning). The paper is short (about 4 pages), elegant, and still readable. For historical completeness, it should be read alongside Werbos [1990] to understand the full context of backpropagation's "invention" — Rumelhart, Hinton, and Williams independently rediscovered what Werbos had derived in 1974, and their paper's contribution was primarily one of framing, demonstration, and access.

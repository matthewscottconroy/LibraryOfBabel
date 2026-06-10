# Chapter 3 Key Researchers

---

## Paul Werbos

**Born:** 1947, United States  
**Affiliation:** National Science Foundation (Program Director, 1988–2013); Harvard University (PhD, 1974)  
**Key Contribution:** Invention of backpropagation through time

Paul Werbos is the inventor of backpropagation, including its application to recurrent networks through time. His 1974 Harvard PhD thesis, *Beyond Regression: New Tools for Prediction and Analysis in the Behavioral Sciences*, contained the first rigorous derivation of the backpropagation algorithm as a method for training multilayer networks — predating its more famous popularization by Rumelhart, Hinton, and Williams by over a decade. Werbos's work was largely ignored for years, in part because it was embedded in a social science dissertation rather than a computer science or engineering paper.

In 1990, Werbos published a clear, accessible account of **Backpropagation Through Time** [Werbos1990] in the *Proceedings of the IEEE* — a paper that became the standard reference for BPTT and that explicitly connected RNN training to the calculus of dynamic optimization (which Werbos had developed independently of the neural network community, building on work in optimal control theory). This paper laid out exactly the recursive gradient equations that now appear in every deep learning textbook, and it gave the algorithm the name by which it is still known.

Werbos's broader research agenda centered on what he called "approximate dynamic programming" and "adaptive critics" — systems that learn to optimize over time, connecting gradient-based learning to Bellman's dynamic programming and thereby anticipating much of modern reinforcement learning. He spent much of his career at the NSF, where he funded research in neural networks and intelligent systems during a period when such funding was unfashionable.

He received the IEEE Neural Networks Pioneer Award in 1994.

---

## David Rumelhart

**Born:** 1942, Audubon, Iowa  
**Died:** 2011  
**Affiliation:** Stanford University; University of California, San Diego (UCSD); Institute for Psychological Research  
**Key Contribution:** Popularization of backpropagation; PDP framework

David Rumelhart was a cognitive scientist whose influence on neural network research was primarily through the intellectual and social infrastructure he helped create. Together with James McClelland, he co-edited the two-volume *Parallel Distributed Processing* (PDP) [Rumelhart1986pdb], which in 1986 brought neural network models to the attention of a generation of cognitive scientists, linguists, and psychologists. The books combined mathematical rigor with cognitive science framing in a way that no prior work had achieved, and they triggered a massive expansion of neural network research.

The 1986 *Nature* paper "Learning representations by back-propagating errors" [Rumelhart1986], co-authored with Geoffrey Hinton and Ronald Williams, became one of the most cited papers in the history of neural network research — not because it invented backpropagation (Werbos had done that earlier, and Rumelhart's group independently rediscovered it) but because it demonstrated backpropagation's power convincingly and presented it in a way that was accessible and reproducible.

Rumelhart also made early contributions to sequence processing with recurrent networks, and the "Simple Recurrent Network" explored by his collaborator Jeff Elman [Elman1990] grew directly from the PDP framework.

Rumelhart received the American Psychological Association's Distinguished Scientific Contribution Award in 1996. Late in his career, he was diagnosed with a progressive neurological disease that rendered him unable to communicate; he died in 2011, unable to deliver remarks when he was awarded the David E. Rumelhart Prize in 2001 — the highest honor in cognitive science and neural network theory.

---

## Sepp Hochreiter

**Born:** 1967, Germany  
**Affiliation:** Johannes Kepler University Linz (Professor, 2006–present); Technical University of Munich (student, then researcher)  
**Key Contribution:** Identification of the vanishing gradient problem; invention of the LSTM

Sepp Hochreiter's influence on recurrent neural network research can be traced to two documents separated by six years. The first, his 1991 diploma thesis *Untersuchungen zu dynamischen neuronalen Netzen* [Hochreiter1991] (Investigations in Dynamical Neural Networks), contained — in what was then a private, German-language document — the first rigorous mathematical analysis of why gradient descent fails in deep and recurrent networks. Hochreiter identified what he called the "vanishing gradient" and showed precisely that it was caused by products of Jacobians with spectral radius less than 1. This analysis was prescient and essentially complete, but it circulated only within a small research group for years.

In 1997, in collaboration with his doctoral advisor Jürgen Schmidhuber, Hochreiter published the full solution in *Neural Computation*: the Long Short-Term Memory network [Hochreiter1997]. The LSTM introduced the concept of a memory cell protected by trainable gates — the forget gate, input gate, and output gate — creating what Hochreiter called the "constant error carousel": a pathway through the cell state along which gradients could flow without attenuation. The paper is now one of the most cited papers in the history of machine learning.

The impact of LSTM was gradual rather than immediate. The natural language processing and speech recognition communities adopted it slowly through the 2000s, but by the early 2010s, LSTM-based architectures were setting records on speech recognition benchmarks (Google's DeployedSpeech, 2012), machine translation, and handwriting recognition. The technology underlying many voice assistants from 2014 to 2017 was fundamentally LSTM.

Hochreiter has continued to work on deep learning theory, including work on flat minima [Hochreiter1997flat] (which anticipated modern interest in the loss landscape), model explainability, and bioinformatics.

---

## Jürgen Schmidhuber

**Born:** 1963, Munich, Germany  
**Affiliation:** The Swiss AI Lab IDSIA, Lugano (co-director, 1995–present); Technical University of Munich  
**Key Contribution:** Co-invention of LSTM; extensive work on self-referential and meta-learning systems; contributions to sequence-to-sequence learning

Jürgen Schmidhuber is one of the most prolific and controversial figures in modern machine learning. His laboratory at IDSIA in Lugano has produced a remarkable concentration of influential work: LSTM (with Hochreiter), bidirectional RNNs [Schuster1997], Connectionist Temporal Classification (CTC) [Graves2006], early work on deep learning, and much of the technical infrastructure for modern sequence modeling.

Schmidhuber's intellectual interests are unusually broad, spanning from practical deep learning to theoretical questions about the limits of inductive inference, the nature of creativity, and the possibility of superintelligent systems. His 1991 paper on "Curious Model-Building Control Systems" [Schmidhuber1991] anticipated reinforcement learning from curiosity; his 2013 overview paper "My First Deep Learning System of 1991 + Deep Learning Timeline 1962–2013" [Schmidhuber2015] is as much historical assertion as review, reflecting his characteristic tendency to establish priority.

His most concrete and lasting contribution is the LSTM. While his role was primarily as supervisor and intellectual architect (Hochreiter derived most of the mathematics), Schmidhuber's insistence on solving the long-term dependency problem, his deep knowledge of the RNN training literature, and his ability to place the work in the broader context of the RTRL versus BPTT debate were essential to the paper's production and framing.

Schmidhuber has been an outspoken advocate for the recognition of independent discovery and priority in machine learning, a stance that has made him both admired (for historical rigor) and criticized (for perceived self-promotion). Whatever one thinks of the approach, his historical accounts of neural network development are generally accurate, meticulously sourced, and important correctives to a field prone to institutional memory loss.

---

## Ronald Williams

**Born:** 1952  
**Affiliation:** Northeastern University (Professor); formerly MIT, Harvard  
**Key Contribution:** Co-invention of BPTT (with Rumelhart); development of RTRL; REINFORCE algorithm

Ronald Williams occupies a quiet but foundational position in the history of recurrent network training. His co-authorship (with Rumelhart and Hinton) of the 1986 *Nature* backpropagation paper gave him early prominence, but his deeper contributions are to recurrent network training specifically.

In 1989, Williams co-authored (with David Zipser) "A Learning Algorithm for Continually Running Fully Recurrent Neural Networks" [Williams1989], which introduced Real-Time Recurrent Learning (RTRL). RTRL is, in some ways, the algorithmically cleaner of the two main training approaches: it is causal (no backward pass through time), it works on sequences of arbitrary and unknown length, and it computes exact gradients. Its practical limitation — the $O(N^4)$ per-step cost — prevented widespread adoption in the era before GPUs.

In the same year, Williams published the REINFORCE algorithm [Williams1992], which provided a gradient estimator for non-differentiable or stochastic decision processes. REINFORCE is the conceptual ancestor of the policy gradient methods at the heart of modern deep reinforcement learning, and it remains the standard example of a score-function gradient estimator.

Williams's work on RTRL and BPTT was important not only for the algorithms themselves but for the careful mathematical analysis he brought to the comparison. His analysis of the gradient equations in both algorithms clarified the relationship between online and batch learning in recurrent networks in ways that remain relevant.

---

## Jeffrey Elman

**Born:** 1948  
**Died:** 2011  
**Affiliation:** University of California, San Diego; Cognitive Science / Linguistics  
**Key Contribution:** Simple Recurrent Networks; connectionist models of language and grammar

Jeffrey Elman was a cognitive scientist and linguist whose contributions to recurrent network research were primarily empirical and conceptual rather than mathematical. His 1990 paper "Finding Structure in Time" [Elman1990] introduced what became known as the **Elman network** — a simple recurrent network in which the hidden units at time $t$ are fed back (with a one-step delay) as additional inputs at time $t + 1$. This is architecturally equivalent to the standard RNN but with the distinction that the "recurrent" connections go through a separate "context layer" that stores the previous hidden state.

Elman's key contribution was to demonstrate that this simple architecture could, when trained on sequences of language-like tokens, learn syntactic structure implicitly — not through rule-following but through the internal dynamics of the recurrent state. His experiments showed that the network developed internal representations corresponding to grammatical categories, that it could learn dependencies spanning several tokens, and that its internal states reflected something like semantic similarity. These results were highly influential in cognitive science and linguistics, where they were taken as evidence for "emergent" grammar from statistical learning.

Elman's work was empirically grounded in a way that was unusual for the time: rather than proving theorems, he ran experiments and analyzed the representations that emerged. This approach — studying the internal states of trained networks as objects of interest in their own right — anticipated much of modern "mechanistic interpretability" research.

He also wrote (with several collaborators) an influential book, *Rethinking Innateness* [Elman1996], which used connectionist modeling to argue against strong nativist theories of language acquisition. The book had a significant impact on the cognitive science and developmental psychology communities.

---

## References

- [Werbos1990] Werbos, P. J. (1990). Backpropagation through time: What it does and how to do it. *Proceedings of the IEEE*, 78(10), 1550–1560.
- [Rumelhart1986] Rumelhart, D. E., Hinton, G. E., & Williams, R. J. (1986). Learning representations by back-propagating errors. *Nature*, 323, 533–536.
- [Rumelhart1986pdb] Rumelhart, D. E., & McClelland, J. L. (Eds.). (1986). *Parallel Distributed Processing*, Vols. 1–2. MIT Press.
- [Hochreiter1991] Hochreiter, S. (1991). Untersuchungen zu dynamischen neuronalen Netzen. Diploma thesis, Technische Universität München.
- [Hochreiter1997] Hochreiter, S., & Schmidhuber, J. (1997). Long short-term memory. *Neural Computation*, 9(8), 1735–1780.
- [Hochreiter1997flat] Hochreiter, S., & Schmidhuber, J. (1997). Flat minima. *Neural Computation*, 9(1), 1–42.
- [Schmidhuber1991] Schmidhuber, J. (1991). A possibility for implementing curiosity and boredom in model-building neural controllers. *Proc. SAB*, 222–227.
- [Schmidhuber2015] Schmidhuber, J. (2015). Deep learning in neural networks: An overview. *Neural Networks*, 61, 85–117.
- [Schuster1997] Schuster, M., & Paliwal, K. K. (1997). Bidirectional recurrent neural networks. *IEEE Trans. Signal Processing*, 45(11), 2673–2681.
- [Graves2006] Graves, A., & Schmidhuber, J. (2005). Framewise phoneme classification with bidirectional LSTM. *Neural Networks*, 18(5–6), 602–610.
- [Williams1989] Williams, R. J., & Zipser, D. (1989). A learning algorithm for continually running fully recurrent neural networks. *Neural Computation*, 1(2), 270–280.
- [Williams1992] Williams, R. J. (1992). Simple statistical gradient-following algorithms for connectionist reinforcement learning. *Machine Learning*, 8, 229–256.
- [Elman1990] Elman, J. L. (1990). Finding structure in time. *Cognitive Science*, 14(2), 179–211.
- [Elman1996] Elman, J. L., et al. (1996). *Rethinking Innateness: A Connectionist Perspective on Development*. MIT Press.

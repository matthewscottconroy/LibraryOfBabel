# Chapter 5: Further Reading

## Annotated Bibliography

---

### [Jaeger2001] Jaeger, H. (2001). The "echo state" approach to analysing and training recurrent neural networks. *GMD Report 148*, German National Research Center for Information Technology.

This is the founding document of echo state networks. Read it first. Jaeger introduces the ESN architecture, defines the echo state property with full mathematical precision, gives sufficient conditions for the ESP (in terms of both spectral norm and spectral radius), and provides a clean proof that contractivity implies the ESP. The empirical section demonstrates dramatically improved performance on chaotic time series prediction compared to gradient-based RNN methods of the era.

What makes this report remarkable is not just its content but its style: it is mathematically honest (distinguishing what is proved from what is conjectured), practically oriented (the section on how to actually build an ESN is as important as the theory), and historically aware (it situates the ESN within the then-current debates about RNN training). Even though it was never published in a journal (it was superseded by the 2004 *Science* paper and the 2002 tutorial), it remains the most complete and careful account of the foundational theory.

**What to read:** The entire report (89 pages) is worth reading at least once. The most important sections are: Section 2 (echo state property definition and theorem), Section 3 (reservoir construction), Section 4 (training procedure), and Section 5 (experiments). The appendix on sufficient conditions is the technical core.

---

### [Jaeger2002tutorial] Jaeger, H. (2002). A tutorial on training recurrent neural networks, covering BPPT, RTRL, EKF and the "echo state network" approach. *GMD Report 159*, German National Research Center for Information Technology.

This tutorial serves a different purpose from the foundational report. It places the ESN in the context of all methods for training recurrent neural networks — backpropagation through time (BPTT), real-time recurrent learning (RTRL), and extended Kalman filter (EKF) approaches — and explains why each succeeds or fails. The ESN is introduced late in the tutorial, after the reader has been thoroughly motivated by the difficulties of gradient-based RNN training.

The tutorial is pedagogically excellent. It walks through BPTT from first principles (chain rule, vanishing gradients), explains the computational complexity of each method, and shows empirically that the ESN achieves comparable or better performance with far less computational effort. This context makes the power of the reservoir computing idea viscerally clear: all of the hard work of recurrent learning is done by the random fixed reservoir, and the easy part (linear regression) is all that remains.

**What to read:** The ESN sections (Chapters 5-7) are essential. The preceding sections on BPTT and gradient vanishing (Chapters 2-4) provide valuable context for understanding why the ESN is a significant advance.

---

### [Jaeger2004] Jaeger, H. and Haas, H. (2004). Harnessing Nonlinearity: Predicting Chaotic Systems and Saving Energy in Wireless Communications. *Science*, 304(5667), 78-80.

The *Science* paper that brought ESNs to the attention of the broader scientific community. It reports two experiments: (1) a 2400-fold improvement over the previous best method for Mackey-Glass time series prediction, using a 400-neuron ESN trained offline; and (2) a demonstration of ESN-based nonlinear channel equalization for wireless communications.

The paper is extremely concise (the body is two pages), making it ideal for a first introduction to the empirical power of the method. The 2400-fold improvement figure is striking and memorable, but the paper is careful to explain what it means: not 2400 times more data, but 2400 times lower normalized root mean squared error — a qualitative improvement, not just quantitative.

**What to read:** The paper is short enough to read in full. Pay attention to the supplement (available online), which contains the experimental details and the reservoir construction procedure.

---

### [Lukoševičius2012] Lukoševičius, M. (2012). A Practical Guide to Applying Echo State Networks. In *Lecture Notes in Computer Science, Neural Networks: Tricks of the Trade* (2nd ed.), G. Montavon, G.B. Orr, K.-R. Müller (eds.), pp. 659-686. Springer.

The definitive practitioner's guide. This paper condenses years of accumulated experience working with ESNs into a systematic set of guidelines. It is organized around the key hyperparameters (spectral radius, leaking rate, input scaling, regularization) and for each one explains: what it controls mathematically, what its effect on performance is empirically, and how to tune it in practice.

The most important contributions are:
- The systematic analysis of input scaling and its interaction with $\rho$.
- The discussion of output feedback and "teacher forcing" (feeding the target output back as input during training, then switching to the actual output during testing).
- The analysis of task properties (what kind of task benefits from high $\rho$? from small $\alpha$?) and their implications for reservoir design.
- Practical advice on debugging ESN failures: what to check first when performance is poor.

**What to read:** All of it. This is the paper you should have open when implementing your first ESN. Return to it whenever you encounter unexpected behavior.

---

### [LukoAndJaeger2009] Lukoševičius, M. and Jaeger, H. (2009). Reservoir Computing Approaches to Recurrent Neural Network Training. *Computer Science Review*, 3(3), 127-149.

The comprehensive 2009 review of the reservoir computing field by its founders. It covers: the historical development of both ESNs and LSMs; the theoretical foundations (echo state property, liquid state machine computation theorem, fading memory); training methods (offline and online); performance comparisons; and a discussion of open problems as of 2009.

This review is the best single reference for understanding where reservoir computing stood at the end of its first decade, and it remains an excellent entry point for graduate students. The theoretical sections are careful and complete; the historical sections are candid about what was known versus conjectured.

**What to read:** Sections 2-4 (theoretical foundations) and Section 5 (performance comparisons) are most relevant to this chapter. Section 6 (extensions and open problems) points toward the research frontier and complements the later chapters of this book.

---

### [Jaeger2002memory] Jaeger, H. (2002). Short-term memory in echo state networks. *GMD Report 152*, German National Research Center for Information Technology.

Introduces the formal concept of memory capacity for ESNs and establishes the fundamental bound $MC \leq N$. This is a theoretically important paper that connects the ESN to information theory and to the broader theory of system memory.

The main result: for a linear reservoir driven by i.i.d. scalar inputs, the total linear memory capacity (the sum of the squared correlations between the reservoir state and past inputs at all lags) is bounded by the reservoir size $N$, with equality achieved for specific reservoir configurations. This bound does not generalize trivially to nonlinear reservoirs, but it establishes the right conceptual framework.

**What to read:** The first three sections (definition of memory capacity, the upper bound theorem, and the numerical experiments) are directly relevant to Section 3.4 of this chapter. The appendix contains the proof.

---

### [BoydChua1985] Boyd, S. and Chua, L.O. (1985). Fading Memory and the Problem of Approximating Nonlinear Operators with Volterra Series. *IEEE Transactions on Circuits and Systems*, 32(11), 1150-1161.

The foundational paper on fading memory for nonlinear dynamical systems, predating reservoir computing by fifteen years. Boyd and Chua introduced the rigorous definition of the fading memory property and established the Stone-Weierstrass-type result that systems with fading memory can be approximated arbitrarily well by Volterra series. This is the mathematical precursor to the LSM computation theorem of Maass et al. and to the theoretical justification of reservoir computing as a universal approximation framework.

**What to read:** Sections 1-4 (the definition of fading memory and the approximation theorem) are directly relevant. The paper is technical but well-written; the key theorems are clearly stated and the proofs are instructive.

---

### [Rota1960] Rota, G.-C. and Strang, G. (1960). A Note on the Joint Spectral Radius. *Indagationes Mathematicae*, 22, 379-381.

A brief but foundational paper that introduces the concept of the joint spectral radius of a set of matrices — the relevant object when studying the long-run growth of products of matrices drawn from a finite set. This is the theoretical underpinning of the proof that $\rho(W^{rec}) < 1$ implies ESP for the ESN with nonlinear activation (where the effective "matrix" at each step is $D_t W^{rec}$ with $D_t$ varying). The paper is short (two pages) and mathematically elegant.

**What to read:** The full paper, as background for the proof in Section 2.2.

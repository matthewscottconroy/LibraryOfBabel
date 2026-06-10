# Chapter 4 Further Reading

## Essential Papers

---

**Jaeger, H. (2001). The "echo state" approach to analysing and training recurrent neural networks. *GMD Report 148*, German National Research Center for Information Technology, Bremen.**

The founding document of the echo state network. This 48-page technical report is more thorough and mathematically detailed than the summary presentations that followed it, and it repays careful study. The report introduces the echo state property with a formal definition, provides sufficient conditions (spectral norm $\|W^{rec}\|_2 < 1$) and necessary conditions ($\rho(W^{rec}) < 1$), and discusses their relationship. It then describes the training procedure, the role of the washout period, and the relationship between spectral radius and memory. The empirical sections demonstrate ESNs on signal generation, nonlinear system identification, and channel equalization — not headline-making results, but clear and reproducible. Of particular value is Jaeger's appendix on the mathematical analysis of the echo state property: it is a model of careful mathematical exposition, clearly separating what is proved from what is conjectured and what is empirically observed. Freely available from Jaeger's website.

---

**Maass, W., Natschläger, T., & Markram, H. (2002). Real-time computing without stable states: A new framework for neural computation based on perturbations. *Neural Computation*, 14(11), 2531–2560.**

The Liquid State Machine paper. This is the theoretical companion to Jaeger's engineering-focused report: where Jaeger emphasized practical construction and training, Maass et al. emphasized mathematical generality and biological relevance. The paper's central contribution is a universal approximation theorem: a liquid with the separation property and the fading memory/approximation property can approximate any causal, time-invariant functional with fading memory. The proof builds on earlier work by Boyd and Chua [Boyd1985] on Volterra series approximation and extends it to the online, continuous-time setting. The paper also contains detailed simulations of biologically realistic cortical circuits, demonstrating that the abstract mathematical conditions are satisfied by circuits parameterized with experimental data. Read the mathematical sections (2–4) carefully before reading the simulations; the theorem and its conditions are the lasting contribution.

---

**Verstraeten, D., Schrauwen, B., d'Haene, M., & Stroobandt, D. (2007). An experimental unification of reservoir computing methods. *Neural Networks*, 20(3), 391–403.**

The paper that gave the field its name and demonstrated that ESN and LSM are mathematically equivalent. Verstraeten et al. compare ESN and LSM on a common set of benchmarks, showing similar performance and identifying the common design principles. The paper also introduces several practical improvements and discusses the relationship between reservoir parameters and task performance. This paper is worth reading not just for the experimental results but for the conceptual work it does: by naming the unified paradigm "reservoir computing" and demonstrating the equivalence of the two approaches, it created the conditions for a coherent research community to develop.

---

**Lukoševičius, M. (2012). A practical guide to applying echo state networks. In *Neural Networks: Tricks of the Trade* (2nd ed., pp. 659–686). Springer, Berlin.**

The best single reference for practitioners. Lukoševičius (who did his PhD with Jaeger) provides a comprehensive, opinionated, and practically grounded guide to building and using reservoir computers. The guide covers reservoir construction (including spectral radius scaling, sparsity, and input scaling), the training procedure (including the washout period, ridge regression, and cross-validation), common failure modes, and advice on parameter tuning. The tone is that of a practitioner who has made every possible mistake and wants you to avoid them. Read this alongside Jaeger's original report to get both the theory and the practice. Also available as a standalone technical report from the author's website.

---

## Books

**Maass, W. (2011). Liquid state machines: Motivation, theory, and applications. In S. B. Cooper & A. Sorbi (Eds.), *Computability in Context: Computation and Logic in the Real World* (pp. 275–296). World Scientific.**

A chapter-length review of the Liquid State Machine framework by its creator, written for a computer science audience. This is a good secondary introduction to the LSM that emphasizes the theoretical computer science perspective — computability, Turing completeness, and the relationship between liquid state machines and other models of computation. The biological motivation is present but not dominant; the focus is on the mathematical framework and its implications for the theory of real-time computation.

---

**Schrauwen, B., Verstraeten, D., & Van Campenhout, J. (2007). An overview of reservoir computing: Theory, applications and implementations. In *Proceedings of the 15th European Symposium on Artificial Neural Networks*, pp. 471–482. d-side publications.**

An early overview article that surveys the reservoir computing landscape as of 2007, including theory, applications, and hardware implementations. Written for a general neural network audience, it is less mathematically demanding than the original papers and provides a useful map of the field's state at the time. The discussion of hardware implementations anticipates the physical reservoir computing work of Units VII–VIII.

---

**Goodfellow, I., Bengio, Y., & Courville, A. (2016). *Deep Learning*. MIT Press. Section 10.9: Echo State Networks.**

A brief but accessible treatment of reservoir computing from the deep learning perspective. The section situates reservoir computing within the broader landscape of recurrent network training approaches, discusses the echo state property, and explains why the fixed-weight approach is justified. The treatment is concise (about 3 pages) and suitable as a first introduction before reading the primary literature. The book is available free online at deeplearningbook.org.

---

## Historical Papers

**Jaeger, H., & Haas, H. (2004). Harnessing nonlinearity: Predicting chaotic systems and saving energy in wireless communication. *Science*, 304(5667), 78–80.**

The paper that brought reservoir computing to a broader scientific audience. By demonstrating that an ESN could predict the Mackey-Glass chaotic time series with an error "more than an order of magnitude smaller than any previous result" (at the time), Jaeger and Haas established that reservoir computing was not just a theoretical curiosity but a practically competitive method. The wireless communication application — using a reservoir to equalize a nonlinear communication channel, thereby saving transmission energy — was chosen to demonstrate the approach's practical relevance to engineers. This paper is short (3 pages, *Science* format) and worth reading as a historical document and for its concise presentation of the ESN training procedure.

---

**Jaeger, H. (2002). Short term memory in echo state networks. *GMD Report 152*, German National Research Center for Information Technology.**

The paper that introduced the concept of **memory capacity** for echo state networks — a rigorous quantification of how much information about the past input a reservoir retains in its current state. Jaeger defines the memory capacity as:

$$MC = \sum_{k=1}^{\infty} \frac{\text{Cov}^2(y_t, u_{t-k})}{\text{Var}(y_t) \cdot \text{Var}(u_t)}$$

where $y_t$ is the optimal linear readout that estimates $u_{t-k}$ from the reservoir state. He proves that $MC \leq N$ for any $N$-unit reservoir, and that linear reservoirs achieve $MC = N$ exactly. This paper established memory capacity as the primary measure of a reservoir's temporal properties and connected it to the spectral radius: larger $\rho$ means memory distributed over longer timescales. The paper's main result — that the total memory capacity is bounded by the number of reservoir units — is one of the cleanest and most cited results in reservoir computing theory.

---

**Boyd, S., & Chua, L. O. (1985). Fading memory and the problem of approximating nonlinear operators with Volterra series. *IEEE Transactions on Circuits and Systems*, 32(11), 1150–1161.**

An earlier paper that provides much of the mathematical foundation for the LSM's universal approximation theorem. Boyd and Chua prove that any causal, time-invariant, continuous functional with fading memory can be approximated by a Volterra series of finite order and memory. This result — that the Volterra series is universal for "nice" temporal functions — is what Maass et al. exploit: they show that a liquid state machine can approximate any Volterra series, and hence (by Boyd-Chua) any continuous temporal functional with fading memory. This paper is technically demanding (it uses functional analysis and the Stone-Weierstrass theorem) but is worth consulting for the precise statement of the approximation theorem and its conditions.

---

**Rahimi, A., & Recht, B. (2007). Random features for large-scale kernel machines. *Advances in Neural Information Processing Systems*, 20.**

The random kitchen sink paper, which provides the theoretical framework for understanding why random weights work in static machine learning. Rahimi and Recht prove that random projections followed by nonlinear transformations provide an explicit finite-dimensional approximation to shift-invariant kernels, and that a linear classifier in this projected space approximates the kernel SVM. The connection to reservoir computing is direct: the reservoir is the temporal generalization of random kitchen sinks, with the recurrent weights providing the temporal projection and the linear readout providing the linear classifier. This paper is worth reading in parallel with the reservoir computing literature to understand the static analogy; the mathematics is clean and the result is striking.

---

**Lukoševičius, M., & Jaeger, H. (2009). Reservoir computing approaches to recurrent neural network training. *Computer Science Review*, 3(3), 127–149.**

A comprehensive review article surveying the reservoir computing literature through 2009. Lukoševičius and Jaeger cover ESN, LSM, the theoretical foundations (echo state property, memory capacity, Volterra series), practical training considerations, and a broad survey of applications. The paper also discusses non-standard reservoir implementations (including analog hardware, optical systems, and mechanical devices) and the relationship between reservoir computing and other machine learning paradigms. This is the standard reference for a broad overview of the field as it stood in 2009 and is worth reading before diving into the more specialized literature of subsequent chapters.

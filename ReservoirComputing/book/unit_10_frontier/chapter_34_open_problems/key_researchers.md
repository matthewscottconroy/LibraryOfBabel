# Chapter 34: Key Researchers

The researchers listed here are primarily the contributors to the open problems — those who have defined the frontier and whose work most directly bears on the unresolved questions.

**Lukas Gonon.** Mathematical statistician who has proved the most rigorous approximation and generalization bounds for reservoir computing to date [Gonon2020, Gonon2021], establishing both upper bounds and beginning the program of tight rate theory. Gonon's work on reservoir computing under stochastic inputs [GononOrtega2021] is the closest thing to a resolution of Problem 34.1.1, though significant gaps remain.

**Juan-Pablo Ortega.** Co-author with Gonon, working on the mathematical foundations of reservoir computing. Ortega's contributions include the development of reservoir universality results under minimal assumptions and the connection between reservoir dynamics and ergodic theory.

**Dean Sussillo and Larry Abbott.** Authors of the FORCE learning algorithm [SussilloMaass2009]. (Note: the paper is Sussillo and Abbott, not Sussillo and Maass as sometimes cited.) Their empirical demonstrations of fast and stable convergence of FORCE on oscillatory tasks defined Problem 34.1.5 and remain the strongest evidence that online reservoir learning can work. The theoretical understanding of why FORCE converges remains open.

**Naftali Tishby (1952–2021).** Information theorist who developed the information bottleneck framework for understanding representation learning, with implications for the optimal reservoir design problem. Tishby's work on the information bottleneck [TishbyZaslavsky2015] provides tools for characterizing the tradeoff between compressing input history and preserving relevant information for the target — directly relevant to Problem 34.1.2.

**Bernhard Schölkopf (1968–).** Director at the Max Planck Institute for Intelligent Systems. Schölkopf's work on kernel methods [ScholkopfSmola2002], support vector machines, and the theoretical foundations of machine learning provides many of the tools (RKHS, $n$-widths, Rademacher complexity) used in the open problems. His more recent work on causal representation learning is relevant to understanding when reservoir structure encodes causal temporal information.

**Thomas Marcucci and colleagues.** Researchers working on optimal reservoir design from a control-theoretic perspective, connecting the reservoir design problem to system identification and optimal experiment design. This line of work is directly relevant to Problem 34.1.2.

**Emmanuel Candès (1970–).** Mathematician at Stanford who developed compressed sensing theory — the theory of sparse recovery from random measurements. Compressed sensing provides both tools and intuition for the random reservoir approximation problem: a random reservoir is a compressed representation of the infinite-dimensional input history, and the question of approximation rates is related to the compressed sensing question of how well a sparse signal can be recovered from random measurements.

**Martin Hairer (1975–).** Mathematician (Fields Medal 2014) who developed the theory of regularity structures for stochastic partial differential equations. While not directly working on RC, Hairer's tools for analyzing the long-run behavior of non-autonomous dynamical systems (which he developed to handle SPDEs driven by rough noise) are potentially applicable to the pullback attractor theory of Problem 34.1.4.

**Surya Ganguli (1978–).** Neuroscientist and computational theorist at Stanford who has worked on the mathematical theory of learning in recurrent neural networks, including connections to random matrix theory, information transmission, and the dynamics of gradient descent in RNNs. Several of his results are directly relevant to the open problems here [GanguliHuh2010, Ganguli2019].

**Ingvar Ziemann and Henrik Sandberg.** Researchers who have developed convergence theory for online system identification with applications to reservoir computing, addressing aspects of Problem 34.1.3. Their work on least-squares estimation for mixing processes [ZiemannSandberg2022] is the most recent technical progress on this problem.

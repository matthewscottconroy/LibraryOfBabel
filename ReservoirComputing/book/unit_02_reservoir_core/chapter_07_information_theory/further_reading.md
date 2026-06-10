# Chapter 7: Further Reading

## Primary Sources

**Dambre, J., Verstraeten, D., Schrauwen, B., & Massar, S. (2012). Information processing capacity of dynamical systems. *Scientific Reports*, 2, 514.**
[Dambre2012]

This is the foundational paper for Section 7.1. It defines the information-processing capacity framework, proves the $C_{total} \leq N$ bound, and demonstrates the capacity decomposition over an orthonormal basis. The paper's key experimental contribution is showing that the bound is tight for linear orthogonal reservoirs and measuring the actual capacity profile of echo state networks with different hyperparameters. Reading the supplementary material is recommended: it contains the detailed proofs of the Parseval identity argument and the connection to the state covariance matrix. The paper is notable for its clarity — the definitions are precise and the proofs are complete, which is not always the case in the reservoir computing literature.

**Jaeger, H. (2002). Short-term memory in echo state networks. GMD Technical Report 152. German National Research Center for Information Technology, Sankt Augustin, Germany.**
[Jaeger2002memory]

This 2002 technical report is where memory capacity was first defined and the bound $MC \leq N$ was first proved. The report is longer and more exploratory than a modern journal paper — Jaeger was clearly working out the ideas in real time — but this makes it an excellent read for understanding the conceptual development. The report contains the geometric decay formula for linear reservoirs, the connection to orthogonal weight matrices, and the first numerical experiments showing $MC$ as a function of spectral radius. It is freely available from Jaeger's website and is essential reading for anyone who wants to understand memory capacity at a deep level.

## Background and Extensions

**Verstraeten, D., Schrauwen, B., D'Haene, M., & Stroobandt, D. (2007). An experimental unification of reservoir computing methods. *Neural Networks*, 20(3), 391–403.**
[Verstraeten2007]

Provides context for the capacity framework by surveying the landscape of reservoir computing approaches. Useful for understanding where capacity theory fits within the broader field.

**Ganguli, S., Huh, D., & Sompolinsky, H. (2008). Memory traces in dynamical systems. *Proceedings of the National Academy of Sciences*, 105(48), 18970–18975.**
[Ganguli2008]

Analyzes memory capacity in recurrent networks from a neuroscience perspective, with emphasis on the role of network structure and the connection to the Fisher information of the stationary distribution. Provides a complementary mathematical treatment to the Dambre framework.

**Tino, P., & Rodan, A. (2013). Short-term memory in input-driven linear dynamical systems. *Neurocomputing*, 112, 58–63.**
[Tino2013]

Analyzes memory capacity for linear reservoirs in detail, including the effect of the input weight vector on the capacity profile. Shows that the memory profile is determined by the interplay between the reservoir eigenspectrum and the projection of the input weights onto each eigenvector. Highly recommended as a complement to Section 7.2.

**Schreiber, T. (2000). Measuring information transfer. *Physical Review Letters*, 85(2), 461.**
[Schreiber2000]

The original paper introducing transfer entropy. Essential background for the transfer entropy exercises and for understanding how the memory capacity framework relates to information-theoretic measures of directional information flow.

**Lukoševičius, M., & Jaeger, H. (2009). Reservoir computing approaches to recurrent neural network training. *Computer Science Review*, 3(3), 127–149.**
[Lukosevicius2009]

A comprehensive review of reservoir computing that includes a section on capacity and memory, situating the theoretical results within the practical context of reservoir design and training. Recommended as a broad survey alongside the more focused primary sources.

# Key Researchers: Chapter 28 — Statistical Learning Theory for Reservoir Computing

## Foundations of Statistical Learning Theory

**Vladimir Vapnik** (1936–): Soviet-American computer scientist; co-developed the VC dimension with Chervonenkis in the 1970s. Later created Support Vector Machines (SVMs) and formalized statistical learning theory in his landmark texts *Estimation of Dependences Based on Empirical Data* (1982) and *The Nature of Statistical Learning Theory* (1995). Vapnik's framework provided the mathematical language in which generalization bounds for reservoir computing can be stated.

**Alexey Chervonenkis** (1938–2014): Soviet mathematician; co-developed the VC dimension and the fundamental theorem of statistical learning. The Vapnik-Chervonenkis theory remains the most widely-taught framework for understanding learnability. Key paper: Vapnik & Chervonenkis [1971].

**Leslie Valiant** (1949–): British-American computer scientist at Harvard; Turing Award laureate (2010). Introduced the PAC learning framework [Valiant 1984], which gave the field a concrete definition of what it means for a hypothesis class to be "learnable." The PAC framework underlies every sample complexity bound in this chapter.

**Anselm Blumer, Andrzej Ehrenfeucht, David Haussler, Manfred Warmuth** (1989): Proved the fundamental theorem of statistical learning, establishing the equivalence between PAC learnability and finite VC dimension [Blumer et al. 1989].

## Rademacher Complexity

**Peter Bartlett** (1964–): Australian-American machine learning theorist at UC Berkeley. With Mendelson, introduced Rademacher complexity as a tool for data-dependent generalization bounds [Bartlett & Mendelson 2002]. Has also worked extensively on neural network complexity and more recently on benign overfitting. Among the most cited researchers in statistical learning theory.

**Shahar Mendelson**: Australian mathematician; developed Rademacher complexity tools and related bounds on function class complexity. Work with Bartlett [2002] established the now-standard framework for uniform convergence bounds via Rademacher complexity.

**Vladimir Koltchinskii**: American statistician; contributed fundamental results on empirical processes and Rademacher complexity in the context of statistical learning [Koltchinskii & Panchenko 2002].

## Covering Numbers and Metric Entropy

**Richard Dudley** (1938–): American probabilist and statistician at MIT. Introduced the metric entropy integral [Dudley 1967] that bears his name, establishing the connection between the entropy of a metric space and the supremum of a Gaussian process. The Dudley integral bound is a cornerstone of empirical process theory.

**Michel Talagrand** (1952–): French mathematician; Abel Prize laureate (2024). Developed the generic chaining method [Talagrand 2014], which provides sharp bounds on suprema of stochastic processes and tightens Dudley's integral in many settings. His majorizing measure theorem gives the exact answer to questions that Dudley's bound only approximates.

## Double Descent and Benign Overfitting

**Mikhail Belkin** (1975–): American mathematician and machine learning researcher at UCSD. With collaborators, documented the double-descent phenomenon and coined the term "benign overfitting" [Belkin et al. 2019]. Argued that modern machine learning practice is not in contradiction with classical statistical theory — rather, the theory needed extension to the overparameterized regime.

**Andrea Montanari**: Italian-American statistician and electrical engineer at Stanford. With Bartlett and Rakhlin, proved the benign overfitting theorem for linear regression [Bartlett et al. 2020], giving the first rigorous characterization of when interpolating solutions generalize. Also works on mean-field theory for neural networks.

**Alexander Rakhlin**: Machine learning theorist at MIT; contributed to the benign overfitting analysis and to online learning theory.

## Implicit Regularization

**Nathan Srebro**: Israeli-American machine learning researcher at TTIC. With collaborators, investigated implicit regularization in gradient descent for matrix factorization and neural networks [Neyshabur et al. 2015]. Showed that the choice of optimization algorithm — not just the loss function — determines which solution is selected.

**Arthur Jacot**: Swiss mathematician; with Gabriel and Hongler, introduced the neural tangent kernel [Jacot et al. 2018], providing a precise characterization of what gradient descent does in the infinite-width limit.

**Chiyuan Zhang**: With Bengio, Hardt, Recht, and Vinyals, demonstrated that deep networks can fit random labels yet generalize on real labels [Zhang et al. 2017], forcing a rethinking of classical generalization theory.

## Application to Reservoir Computing

**Lukas Gonon**: Swiss mathematician; with Ortega, derived the first rigorous generalization bounds for reservoir computing [Gonon & Ortega 2020, 2021]. Established PAC-type bounds for ESNs with stochastic inputs, using Rademacher complexity tools. Work represents the state of the art in statistical learning theory for RC as of 2025.

**Juan-Pablo Ortega**: Spanish mathematician; collaborates with Gonon on the theoretical foundations of reservoir computing, including universality, stability, and statistical learning.

## References

- Bartlett, P. L. and Mendelson, S. (2002). Rademacher and Gaussian complexities: Risk bounds and structural results. *JMLR*, 3, 463–482.
- Bartlett, P. L., Montanari, A., and Rakhlin, A. (2020). Benign overfitting in linear regression. *PNAS*, 117(48), 30063–30070.
- Belkin, M., Hsu, D., Ma, S., and Mandal, S. (2019). Reconciling modern machine-learning practice and the classical bias-variance trade-off. *PNAS*, 116(32), 15849–15854.
- Blumer, A., Ehrenfeucht, A., Haussler, D., and Warmuth, M. K. (1989). Learnability and the VC dimension. *JACM*, 36(4), 929–965.
- Dudley, R. M. (1967). The sizes of compact subsets of Hilbert space. *Journal of Functional Analysis*, 1(3), 290–330.
- Gonon, L. and Ortega, J.-P. (2020). Reservoir computing universality with stochastic inputs. *IEEE TNNLS*, 31(1), 100–112.
- Jacot, A., Gabriel, F., and Hongler, C. (2018). Neural tangent kernel. *NeurIPS*, 31.
- Koltchinskii, V. and Panchenko, D. (2002). Empirical margin distributions. *Annals of Statistics*, 30(1), 1–50.
- Neyshabur, B., Tomioka, R., and Srebro, N. (2015). In search of the real inductive bias. *ICLR Workshop*.
- Talagrand, M. (2014). *Upper and Lower Bounds for Stochastic Processes*. Springer.
- Valiant, L. G. (1984). A theory of the learnable. *CACM*, 27(11), 1134–1142.
- Vapnik, V. N. and Chervonenkis, A. Y. (1971). Uniform convergence of relative frequencies. *Theory of Probability*, 16(2), 264–280.
- Zhang, C. et al. (2017). Understanding deep learning requires rethinking generalization. *ICLR*.

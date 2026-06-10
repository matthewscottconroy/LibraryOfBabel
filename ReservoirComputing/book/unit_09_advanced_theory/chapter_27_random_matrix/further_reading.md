# Chapter 27: Further Reading

## Foundational Random Matrix Theory

**Anderson, G.W., Guionnet, A., and Zeitouni, O. (2010).** *An Introduction to Random Matrices*. Cambridge University Press. The standard graduate-level reference for RMT. Chapters 2 and 3 give complete proofs of the semicircle law (via both moments and the Stieltjes transform), and Chapter 3 covers the Marchenko-Pastur law. Mathematically rigorous; excellent for the proofs in Sections 27.1–27.2.

**Tao, T. (2012).** *Topics in Random Matrix Theory*. American Mathematical Society. Based on Tao's graduate course. Very readable, with detailed proofs. Chapter 2 covers the moment method for the semicircle law; Chapter 4 covers the circular law. Tao's lecture notes are also freely available online.

**Mehta, M.L. (2004).** *Random Matrices*, 3rd edition. Elsevier. The classical reference, comprehensive but older. Most useful for the physicist's perspective and for results on eigenvalue spacing statistics (relevant to quantum chaos connections in Chapter 31).

**Bai, Z.D. and Silverstein, J.W. (2009).** *Spectral Analysis of Large Dimensional Random Matrices*, 2nd edition. Springer. Focused on large-dimensional statistics. The definitive reference for the Marchenko-Pastur law and its extensions to spiked covariance models.

## Free Probability

**Nica, A. and Speicher, R. (2006).** *Lectures on the Combinatorics of Free Probability*. Cambridge University Press. An accessible introduction to free probability theory, emphasizing the combinatorial (Catalan numbers, non-crossing partitions) and operator-algebraic aspects. The connection between free probability and random matrices is thoroughly developed.

**Voiculescu, D.V., Dykema, K.J., and Nica, A. (1992).** *Free Random Variables*. American Mathematical Society. Voiculescu's original development of free probability. Dense but foundational.

## Concentration Inequalities

**Tropp, J.A. (2012).** "User-Friendly Tail Bounds for Sums of Random Matrices." *Foundations of Computational Mathematics*, 12(4), 389–434. The primary reference for Section 27.3.4. Readable, comprehensive, and full of worked examples. Essential reading for anyone using matrix concentration inequalities.

**Vershynin, R. (2018).** *High-Dimensional Probability: An Introduction with Applications in Data Science*. Cambridge University Press. An excellent textbook covering sub-Gaussian random variables, Hoeffding and Bernstein inequalities, and matrix concentration, with applications throughout data science. Chapters 3, 4, and 5 cover exactly the material in Section 27.3.

**Wainwright, M.J. (2019).** *High-Dimensional Statistics: A Non-Asymptotic Viewpoint*. Cambridge University Press. A comprehensive treatment of high-dimensional statistics with extensive use of concentration inequalities. Chapter 2 covers scalar and matrix concentration from the perspective of statistical estimation.

**Boucheron, S., Lugosi, G., and Massart, P. (2013).** *Concentration Inequalities: A Nonasymptotic Theory of Independence*. Oxford University Press. The comprehensive reference for concentration inequalities, including the entropy method, Talagrand's inequality, and many specialized results. More advanced than Vershynin.

## Applications to Machine Learning and Reservoir Computing

**Tao, T. and Vu, V. (2010).** "Random Matrices: Universality of ESDs and the Circular Law." *Annals of Probability*, 38(5), 2023–2065. The paper proving the circular law under minimal assumptions. Relevant to Section 27.1.5.

**Ledoit, O. and Wolf, M. (2004).** "A Well-Conditioned Estimator for Large-Dimensional Covariance Matrices." *Journal of Multivariate Analysis*, 88(2), 365–411. Derives the optimal shrinkage estimator for the sample covariance matrix under the Marchenko-Pastur distribution. Directly relevant to ridge regression for reservoir readout.

**Couillet, R. and Debbah, M. (2011).** *Random Matrix Methods for Wireless Communications*. Cambridge University Press. A thorough treatment of RMT applications in communications. The signal processing perspective complements the machine learning view and contains results on random feature maps relevant to reservoir computing.

**Gonon, L. (2022).** "Random Feature Neural Networks Learn Black-Scholes Type PDEs without Curse of Dimensionality." *Journal of Machine Learning Research*, 23(1), 1–51. Uses random feature theory (closely related to reservoir computing) to prove approximation rates for PDEs, illustrating how RMT tools enter approximation theory.

## Tracy-Widom and Edge Statistics

**Tracy, C.A. and Widom, H. (1994).** "Level-Spacing Distributions and the Airy Kernel." *Communications in Mathematical Physics*, 159(1), 151–174. The original paper. Notation is dense, but the derivation is a tour de force of integrable systems theory.

**Baik, J., Deift, P., and Johansson, K. (1999).** "On the Distribution of the Length of the Longest Increasing Subsequence of Random Permutations." *Journal of the American Mathematical Society*, 12(4), 1119–1178. A landmark paper connecting Tracy-Widom to combinatorics and random growth, demonstrating the universality of TW beyond random matrices.

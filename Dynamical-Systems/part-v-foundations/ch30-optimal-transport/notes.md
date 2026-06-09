# Chapter Notes — Chapter 30

## On the Primary Sources

Cédric Villani's two books cover everything in this chapter and much more. *Topics in Optimal Transportation* (AMS, 2003) is the shorter, more accessible introduction; *Optimal Transport: Old and New* (Springer, 2009) is the comprehensive monograph, over 900 pages. Villani received the Fields Medal in 2010, and optimal transport was central to the citation. If you read one thing beyond this chapter, read the first three chapters of *Topics*.

The Jordan-Kinderlehrer-Otto paper, *The Variational Formulation of the Fokker-Planck Equation* (SIAM J. Math. Anal., 1998), is the paper that connected optimal transport to PDEs. It's beautifully written and not as technically demanding as you might expect.

## On the Lott-Sturm-Villani Theory

The original papers are Lott-Villani's *Ricci Curvature for Metric-Measure Spaces via Optimal Transport* (Annals of Mathematics, 2009) and Sturm's *On the geometry of metric measure spaces I* and *II* (Acta Mathematica, 2006). For the general reader, Villani's lecture notes *Synthetic Theory of Ricci Curvature Bounds* (online) provide a gentler entry point.

The connection to discrete curvature — defining Ricci curvature for graphs using the LSV approach — is developed in Lin-Lu-Yau (2011) and Ollivier (2009). This has become an active area with applications to network science and computer graphics.

## On Computational Optimal Transport

For computational aspects — including the Sinkhorn algorithm, entropic regularization, and applications to machine learning — the definitive reference is Peyré and Cuturi's *Computational Optimal Transport* (Foundations and Trends in Machine Learning, 2019), available freely online. This bridges the mathematical theory of optimal transport to its computational implementation.

## On Machine Learning Applications

Wasserstein GANs are in Arjovsky, Chintala, and Bottou (ICML 2017). The paper is short and clearly argued; the mathematical content is essentially the Kantorovich-Rubinstein duality theorem (Theorem 30.1.4), and reading the paper alongside Section 30.1 is illuminating.

The broader program of using optimal transport in statistics and machine learning is surveyed in Kolouri et al. (2017) and Mémoli's work on shape analysis using Gromov-Wasserstein distances (for comparing metric measure spaces without an ambient embedding).

## On Otto's Riemannian Structure

The formal Riemannian structure of Wasserstein space is in Otto (2001), *The geometry of dissipative evolution equations: the porous medium equation*. The rigorous framework, using the theory of gradient flows in metric spaces, is in Ambrosio-Gigli-Savaré's *Gradient Flows in Metric Spaces and in the Space of Probability Measures* (Birkhäuser, 2005). The latter is the authoritative reference but requires significant technical background.

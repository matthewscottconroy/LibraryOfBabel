# Notes — Chapter 20

The primary reference is Amari and Nagaoka's *Methods of Information Geometry* (AMS/Oxford, 2000), translated from the 1993 Japanese original. This is the foundational modern text, covering statistical manifolds, dual connections, the Pythagorean theorem, and exponential families in full mathematical detail. Some parts are dense, but the payoff is a unified framework that connects differential geometry, statistics, and information theory.

Amari's newer book *Information Geometry and Its Applications* (Springer, 2016) is more accessible and covers applications to machine learning, neural networks, and signal processing. It includes the natural gradient, variational inference, and connections to optimal transport. A good starting point for readers primarily interested in applications.

For the mathematically rigorous treatment: Ay, Jost, Lê, and Schwachhöfer's *Information Geometry* (Springer, 2017) provides the most careful and general treatment of the subject, including Chentsov's theorem on the uniqueness of the Fisher metric, the geometry of quantum statistical manifolds, and the extension to infinite-dimensional spaces.

For optimal transport and the JKO scheme: Villani's *Optimal Transport: Old and New* (Springer, 2009) is the comprehensive mathematical reference. The Jordan-Kinderlehrer-Otto paper, *The Variational Formulation of the Fokker-Planck Equation* (SIAM Journal of Mathematical Analysis, 1998), is where the gradient flow interpretation of Fokker-Planck originated. The more accessible survey by Peyré and Cuturi, *Computational Optimal Transport* (Foundations and Trends in Machine Learning, 2019), is available free online and covers the computational side.

Natural gradient in neural networks was introduced by Amari in *Natural Gradient Works Efficiently in Learning* (Neural Computation, 1998). The practical K-FAC approximation is from Martens and Grosse, *Optimizing Neural Networks with Kronecker-Factored Approximate Curvature* (ICML 2015).

Information geometry has become central to variational inference: the evidence lower bound (ELBO) is a KL divergence minimization problem, and the optimal variational distribution is the $m$-projection of the true posterior onto the variational family. This is covered in Blei, Kucukelbir, and McAuliffe's review *Variational Inference: A Review for Statisticians* (Journal of the American Statistical Association, 2017).

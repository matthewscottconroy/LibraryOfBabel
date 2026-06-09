# 20.5 Fisher Information and Optimal Transport

There are two natural Riemannian metrics on the space of probability measures on a manifold: the Fisher-Rao metric (information geometry) and the Wasserstein-2 metric (optimal transport). They are complementary — one measures "statistical distinguishability," the other measures "mass transport cost" — and both appear in the analysis of diffusion processes and neural networks.

**Theorem 20.5.1 (Otto's Riemannian Structure on Wasserstein Space).** The space $\mathcal{P}(\Omega)$ of probability measures on a Riemannian manifold $(\Omega, g)$, equipped with the Wasserstein-2 metric, has the formal structure of an infinite-dimensional Riemannian manifold. The Fisher-Rao metric and the Otto metric are the two most natural Riemannian metrics on the space of probability measures.

The two metrics have very different characters:
- The Fisher-Rao metric is local in parameter space: a small change in $\theta$ that significantly changes $p_\theta$ in some region contributes a lot to Fisher information.
- The Wasserstein-2 metric is local in sample space: moving mass a small distance contributes less than moving it a large distance.

A distribution that is close in Fisher-Rao metric is "statistically indistinguishable" from the reference. A distribution close in Wasserstein-2 metric has its mass in approximately the same location.

**The Gradient Flow Connection:**

One of the most beautiful results in modern probability theory is that the Fokker-Planck equation can be interpreted as a gradient flow:

The Fokker-Planck equation $\partial_t \rho = \nabla \cdot (\rho \nabla V) + \sigma \Delta \rho$ is the gradient flow of the *free energy functional*:
$$F(\rho) = \int V\rho\,dx + \sigma\int \rho\log\rho\,dx$$
in the Wasserstein-2 metric (the JKO scheme, Jordan-Kinderlehrer-Otto 1998).

The free energy $F = \text{potential energy} + \sigma \cdot \text{entropy}$ (note: $-\sigma H(\rho)$ is the entropy term). The Fokker-Planck evolution minimizes $F$ over time, with the dissipation measured in the Wasserstein metric. In equilibrium, $F$ is minimized at the Gibbs distribution $\rho \propto e^{-V/\sigma}$ — exactly the maximum entropy distribution under the energy constraint.

This connects information geometry (entropy is the potential), optimal transport (Wasserstein metric measures the geometric cost of redistributing mass), and statistical mechanics (Fokker-Planck describes diffusion in a potential well). The JKO scheme makes this precise and leads to existence proofs for Fokker-Planck via variational methods.

For dynamical systems, this has a direct implication: the long-time evolution of a Fokker-Planck equation toward its stationary distribution is a gradient descent in the Fisher-Rao or Wasserstein sense (depending on the scaling), and the convergence rate is controlled by functional inequalities (Poincaré, log-Sobolev) that connect directly to the geometry of the potential $V$.

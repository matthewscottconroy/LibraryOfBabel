# 30.4 Gradient Flows in Wasserstein Space

A gradient flow in ordinary calculus goes downhill as steeply as possible: $\dot{x} = -\nabla F(x)$. The path is perpendicular to the level sets of $F$ and descends at the rate dictated by the gradient. In a Riemannian manifold, "gradient" is defined using the metric, and the steepest descent path depends on the metric.

In Wasserstein space, the same definition works. A gradient flow in $(\mathcal{P}_2(\mathbb{R}^n), W_2)$ is a curve of probability measures that descends as steeply as possible in some functional $\mathcal{F}$, using the $W_2$ metric to define "steepness." The surprise is that many of the most important PDEs of mathematical physics — equations that describe diffusion, heat flow, and particle dynamics — turn out to be exactly such Wasserstein gradient flows.

**Definition 30.4.1.** A curve $(\mu_t)$ in $\mathcal{P}_2(\mathbb{R}^n)$ is a *gradient flow* of a functional $\mathcal{F}$ in Wasserstein space if it satisfies the continuity equation:
$$\partial_t \mu_t + \nabla \cdot (\mu_t v_t) = 0, \quad v_t = -\nabla \frac{\delta\mathcal{F}}{\delta\mu}\bigg|_{\mu_t}.$$

The continuity equation says mass is conserved (no mass is created or destroyed). The velocity field $v_t$ is minus the "functional gradient" of $\mathcal{F}$ with respect to the measure — the steepest-descent direction in Wasserstein space.

**Theorem 30.4.2 (Jordan-Kinderlehrer-Otto, 1998).** The Fokker-Planck equation:
$$\partial_t \rho = \nabla \cdot (\rho \nabla V) + \Delta\rho$$
(describing diffusion in a potential $V$) is the gradient flow in $\mathcal{P}_2(\mathbb{R}^n)$ of the *free energy functional*:
$$\mathcal{F}(\rho) = \int V(x)\rho(x)\,dx + \int \rho(x)\log\rho(x)\,dx.$$

The free energy functional has two terms: the potential energy $\int V\rho\, dx$ and the entropy $\int \rho\log\rho\, dx$ (up to sign — this is the negative Gibbs entropy, so minimizing $\mathcal{F}$ is maximizing entropy subject to potential energy constraints). The Fokker-Planck equation is the equation of motion for a particle diffusing in potential $V$, and the Jordan-Kinderlehrer-Otto theorem says this motion is precisely the steepest descent of free energy in Wasserstein space.

The connection to Chapter 29 is direct: the free energy functional here is exactly $F = U - TS$ from Definition 29.1.4, and minimizing it is what systems in contact with a heat bath do. The Fokker-Planck equation is thermodynamics in motion.

**Theorem 30.4.3 (Heat Equation as Wasserstein Gradient Flow).** The heat equation $\partial_t \rho = \Delta\rho$ is the gradient flow of the *Boltzmann entropy* $H(\rho) = \int \rho\log\rho\,dx$ in $\mathcal{P}_2(\mathbb{R}^n)$.

This is the special case $V = 0$ of the Jordan-Kinderlehrer-Otto theorem. The heat equation — spreading heat uniformly — is the steepest ascent of Boltzmann entropy in Wasserstein space. As the solution of the heat equation becomes more diffuse, entropy increases; the Wasserstein gradient flow perspective makes this monotone entropy increase into a geometric statement about the trajectory in $\mathcal{P}_2(\mathbb{R}^n)$.

These results are not just philosophically satisfying — they are technically useful. The JKO scheme (named for Jordan, Kinderlehrer, and Otto) provides a numerical method for solving the Fokker-Planck equation by iterating Wasserstein gradient descent steps. Concretely: given $\rho_n$, define $\rho_{n+1}$ as the minimizer of
$$\frac{1}{2\tau}W_2(\rho_{n+1}, \rho_n)^2 + \mathcal{F}(\rho_{n+1}).$$
As $\tau \to 0$, this implicit discretization converges to the Fokker-Planck equation. The scheme is useful computationally because minimizing over couplings (the Kantorovich problem) is a tractable linear program.

The next section takes the geometry further: the structure of Wasserstein space encodes information about Ricci curvature.

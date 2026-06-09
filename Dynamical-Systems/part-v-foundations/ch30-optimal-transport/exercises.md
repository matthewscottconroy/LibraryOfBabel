# Exercises — Chapter 30

These problems range from direct computation (30.1, 30.2) to conceptual verification (30.3, 30.4). Exercise 30.3 in particular connects the abstract gradient flow picture to an explicit computation you can do by hand.

---

**Exercise 30.1.** (Earth Mover's Distance) Compute $W_1(\mu, \nu)$ for $\mu = \frac{1}{3}(\delta_0 + \delta_1 + \delta_2)$ and $\nu = \frac{1}{2}(\delta_0 + \delta_3)$ on $\mathbb{R}$. Find the optimal transport plan.

**Exercise 30.2.** (Brenier's Theorem) Find the optimal transport map from $\mu = \text{Uniform}([-1,1])$ to $\nu = \text{Uniform}([0,2])$ for cost $c(x,y) = |x-y|^2$.

**Exercise 30.3.** Verify the Jordan-Kinderlehrer-Otto theorem for the Gaussian: if $\mu_t = \mathcal{N}(0, \sigma(t)^2)$ is the solution of the heat equation (so $\sigma(t)^2 = 1 + 2t$), compute $\frac{d}{dt}H(\mu_t)$ and verify it equals minus the Wasserstein gradient of $H$.

**Exercise 30.4.** (Talagrand) Use the Talagrand inequality to prove the Gaussian concentration inequality: $\gamma(\{x: |f(x) - m| > t\}) \leq 2e^{-t^2/2}$ for any 1-Lipschitz $f$ and $\gamma = \mathcal{N}(0,1)$, where $m$ is the median.

# 30.6 Optimal Transport and Information Theory

The last piece of the picture connects optimal transport to information theory — specifically, to KL divergence and concentration of measure. These connections explain why Wasserstein distances are useful not just geometrically but probabilistically, and why they appear in machine learning and statistics.

**Theorem 30.6.1 (Talagrand Inequality).** For a Gaussian measure $\gamma$ on $\mathbb{R}^n$ (with variance $\sigma^2$):
$$W_2(\mu, \gamma)^2 \leq 2\sigma^2 D_{KL}(\mu \| \gamma).$$

More generally, a probability measure $\nu$ satisfies the *Talagrand transport inequality* $T_1(C)$ if:
$$W_1(\mu, \nu)^2 \leq 2C \cdot D_{KL}(\mu \| \nu)$$
for all $\mu$.

The Talagrand inequality controls the Wasserstein distance from equilibrium by the KL divergence. Compare to Theorem 29.1.5 (free energy equals $k_BT$ times KL divergence from equilibrium): the Talagrand inequality is the Wasserstein version of the same relationship. Wasserstein distance is bounded by information distance (KL divergence), at a rate controlled by the "temperature" (variance $\sigma^2$) of the reference measure.

**Theorem 30.6.2 (Pinsker's Inequality).** $W_1(\mu, \nu) \leq \sqrt{\frac{1}{2}D_{KL}(\mu \| \nu)}$ (up to constants).

Pinsker's inequality is the stronger statement: $W_1$ is controlled by the square root of KL divergence. This places $W_1$ strictly below KL divergence in the information-distance hierarchy: two distributions can be close in $W_1$ but far in KL divergence (if $\nu$ puts positive mass where $\mu$ does not, KL divergence is infinite, but $W_1$ can still be finite).

**Application 30.6.3 (Concentration of Measure).** If $\nu$ satisfies $T_1(C)$ and $f$ is 1-Lipschitz, then:
$$\nu(\{x : |f(x) - \mathbb{E}f| > t\}) \leq 2e^{-t^2/(2C)}.$$

This is the *Gaussian concentration inequality* — optimal transport bounds give measure concentration.

The derivation is elegant. By $T_1(C)$, the measure of the set $\{|f - \mathbb{E}f| > t\}$ can be bounded using the Talagrand inequality: this set has $W_1$ distance at least $t$ from the "centered" part of $\nu$, so its relative entropy is at least $t^2/(2C)$, so its probability is at most $2e^{-t^2/(2C)}$.

This is concentration of measure — the observation that in high-dimensional spaces, Lipschitz functions concentrate near their mean. The Talagrand inequality turns this into an exact quantitative bound, and the sharpness of the Gaussian bound explains why the Gaussian is the "optimal" concentration distribution.

In machine learning, these connections are immediately useful. Wasserstein GANs (Arjovsky et al., 2017) use $W_1$ as the loss function for training generative models, motivated by the Kantorovich-Rubinstein duality (Theorem 30.1.4) and the observation that $W_1$ provides a useful gradient even when the distributions have disjoint supports. KL divergence would be infinite in this case; $W_1$ is always finite.

The optimal transport perspective also motivates many regularization methods in statistics: entropic regularization (adding a KL divergence penalty to the Kantorovich problem) gives the Sinkhorn algorithm, one of the most efficient computational methods for Wasserstein distances. The entropic regularization connects back to the thermodynamic formalism of Chapter 29, where the KL divergence appeared as excess free energy.

We close by noting where optimal transport sits in the larger story. Chapter 29 gave us thermodynamics (entropy, free energy, irreversibility). Chapter 30 gave us geometry (Wasserstein distance, Ricci curvature, gradient flows). These two stories meet in the LSV theory: Ricci curvature is entropy convexity in Wasserstein space. The next two chapters move in different directions: Chapter 31 to number theory (where ergodic methods solve combinatorial problems), and Chapter 32 to logic (where the Borel hierarchy determines the complexity of classification questions). But the thread running through all of Part V is the same: the mathematics of dynamical systems reaches into unexpected places, and when it arrives, it brings powerful tools.

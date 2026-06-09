# Chapter 20 — Information Geometry

> *The space of probability distributions is a manifold. Information geometry studies its intrinsic geometry — the Riemannian structure given by the Fisher information metric, and the pair of dual connections that replace the Levi-Civita connection.*

**Prerequisites:** Chapters 3 (differential geometry, Riemannian manifolds), 16 (KL divergence, information inequalities).

**What this chapter builds:** The statistical manifold as a Riemannian manifold with the Fisher metric; exponential families as flat manifolds in the information-geometric sense; the Pythagorean theorem for KL divergence; the Cramér-Rao bound; the em-algorithm as alternating projections; and applications to machine learning and optimal transport.

---

## 20.1 Statistical Manifolds

### 20.1.1 Parametric Families

**Definition 20.1.1.** A *statistical model* (or *statistical manifold*) is a family of probability distributions $\mathcal{S} = \{p_\theta : \theta \in \Theta\}$ where $\Theta \subseteq {\mathbb R}^n$ is open and $\theta \mapsto p_\theta$ is smooth.

Examples:
- Gaussian family: $p_{\mu,\sigma^2} = N(\mu, \sigma^2)$, parametrized by $\theta = (\mu, \sigma^2) \in {\mathbb R} \times {\mathbb R}_{>0}$
- Exponential family: $p_\theta(x) = \exp(\theta \cdot T(x) - \psi(\theta)) h(x)$ for sufficient statistic $T$

### 20.1.2 The Fisher Information Metric

**Definition 20.1.2.** The *Fisher information matrix* at $\theta \in \Theta$ is:
$$g_{ij}(\theta) = E_\theta\left[\frac{\partial \ell}{\partial \theta_i} \cdot \frac{\partial \ell}{\partial \theta_j}\right] = -E_\theta\left[\frac{\partial^2 \ell}{\partial \theta_i \partial \theta_j}\right],$$
where $\ell(\theta; x) = \log p_\theta(x)$ is the log-likelihood.

**Theorem 20.1.3.** The Fisher information matrix $g(\theta) = (g_{ij}(\theta))$ is positive semi-definite and defines a Riemannian metric on $\mathcal{S}$ (the *Fisher-Rao metric*).

**Theorem 20.1.4 (Cramér-Rao Bound).** For any unbiased estimator $\hat\theta(X_1, \ldots, X_n)$ of $\theta$ from $n$ i.i.d. samples:
$$\text{Cov}[\hat\theta] \geq \frac{1}{n} g(\theta)^{-1}$$
(in the matrix sense: $\text{Cov}[\hat\theta] - g^{-1}/n$ is positive semi-definite).

The Fisher metric sets a lower bound on how accurately any estimator can recover $\theta$ from data.

**Achievability:** The maximum likelihood estimator (MLE) $\hat\theta_{\text{MLE}} = \arg\max_\theta \sum_{i=1}^n \log p_\theta(x_i)$ achieves the Cramér-Rao bound asymptotically:
$$\sqrt{n}(\hat\theta_{\text{MLE}} - \theta) \xrightarrow{d} N(0, g(\theta)^{-1}).$$

---

## 20.2 Exponential Families

**Definition 20.2.1.** A family $\{p_\theta\}$ is an *exponential family* if:
$$p_\theta(x) = h(x) \exp(\theta \cdot T(x) - \psi(\theta)),$$
where:
- $\theta \in {\mathbb R}^n$ is the *natural parameter*
- $T(x): \mathcal{X} \to {\mathbb R}^n$ is the *sufficient statistic*
- $\psi(\theta) = \log \int h(x) e^{\theta \cdot T(x)} dx$ is the *log-partition function* (normalization)

**Examples:**
- Gaussian: $\theta = (\mu/\sigma^2, -1/(2\sigma^2))$, $T(x) = (x, x^2)$
- Bernoulli: $\theta = \log(p/(1-p))$, $T(x) = x$
- Exponential: $\theta = -\lambda$, $T(x) = x$
- Dirichlet, Poisson, Multinomial — all exponential families

**Key Properties:**
- $\nabla_\theta \psi(\theta) = E_\theta[T(X)]$ (mean of sufficient statistic)
- $\nabla^2_\theta \psi(\theta) = \text{Cov}_\theta[T(X)] = g(\theta)$ (Fisher information = Hessian of $\psi$)
- $\psi$ is convex

**Dual Parametrization:** The *mean parameter* is $\eta = E_\theta[T(X)] = \nabla\psi(\theta)$. The map $\theta \mapsto \eta$ is a bijection (since $\psi$ is strictly convex). The *Legendre transform* $\varphi(\eta) = \theta \cdot \eta - \psi(\theta)$ gives the *negative entropy* as a function of $\eta$.

---

## 20.3 Dual Connections and Dually Flat Geometry

**Definition 20.3.1.** A *statistical manifold* $(\mathcal{S}, g, \nabla, \nabla^*)$ has a Riemannian metric $g$ and two torsion-free affine connections $\nabla$ and $\nabla^*$ (the *dual pair*) satisfying:
$$Z\langle g(X, Y)\rangle = g(\nabla_Z X, Y) + g(X, \nabla^*_Z Y)$$
for all vector fields $X, Y, Z$.

**Definition 20.3.2.** The *$\alpha$-connection* is $\nabla^{(\alpha)} = \frac{1+\alpha}{2}\nabla^{(e)} + \frac{1-\alpha}{2}\nabla^{(m)}$, where:
- *$e$-connection* ($\alpha = 1$): exponential connection (associated to exponential families)
- *$m$-connection* ($\alpha = -1$): mixture connection

**Theorem 20.3.3.** An exponential family is *$e$-flat*: the $e$-connection is flat (zero curvature). Equivalently, the natural parameters $(\theta_i)$ form an affine coordinate system for $\nabla^{(e)}$.

---

## 20.4 KL Divergence and the Pythagorean Theorem

**Theorem 20.4.1 (Pythagorean Theorem for KL Divergence).** Let $\mathcal{E}$ be an $e$-flat submanifold of $\mathcal{S}$ and $q \in \mathcal{S}$. Let $p^*$ be the $m$-projection of $q$ onto $\mathcal{E}$ (the point minimizing $D_{\text{KL}}(p \| q)$ over $p \in \mathcal{E}$):
$$D_{\text{KL}}(p^* \| q) + D_{\text{KL}}(p \| p^*) = D_{\text{KL}}(p \| q) \quad \text{for all } p \in \mathcal{E}.$$

Similarly for $e$-projections onto $m$-flat submanifolds.

**Interpretation:** This is the exact KL-divergence analogue of the Euclidean Pythagorean theorem $\|p^* - q\|^2 + \|p - p^*\|^2 = \|p - q\|^2$ when $p^*$ is the orthogonal projection of $q$ onto a flat subspace.

**Application — EM Algorithm:** The *Expectation-Maximization (EM) algorithm* minimizes $D_{\text{KL}}(p_{\text{data}} \| p_\theta)$ over $\theta \in \Theta$. Geometrically, it is alternating $m$- and $e$-projections:
1. **E-step**: $m$-project the current estimate onto the manifold of complete-data distributions
2. **M-step**: $e$-project back onto the parametric family

Each step decreases $D_{\text{KL}}$, and the algorithm converges to a local minimum.

---

## 20.5 Fisher Information and Optimal Transport

**Theorem 20.5.1 (Otto's Riemannian Structure on Wasserstein Space).** The space $\mathcal{P}(\Omega)$ of probability measures on a Riemannian manifold $(\Omega, g)$, equipped with the Wasserstein-2 metric, has the formal structure of an infinite-dimensional Riemannian manifold. The Fisher-Rao metric and the Otto metric are the two most natural Riemannian metrics on probability space.

**The Gradient Flow Connection:** The Fokker-Planck equation $\partial_t \rho = \nabla \cdot (\rho \nabla V) + \sigma \Delta \rho$ is the gradient flow of the *free energy functional* $F(\rho) = \int V\rho\,dx + \sigma\int \rho\log\rho\,dx$ in the Wasserstein-2 metric (JKO scheme, Jordan-Kinderlehrer-Otto 1998).

The free energy $F = \text{potential energy} + \sigma \cdot \text{negative entropy}$. In equilibrium, this is minimized at the Gibbs distribution $\rho \propto e^{-V/\sigma}$.

---

## 20.6 Natural Gradient

**Definition 20.6.1.** For a loss function $L(\theta)$ on a statistical manifold, the *natural gradient* is:
$$\tilde\nabla L(\theta) = g(\theta)^{-1} \nabla L(\theta),$$
where $g(\theta)$ is the Fisher information matrix.

**Motivation:** Steepest descent in the KL-divergence geometry rather than Euclidean parameter space. Natural gradient descent achieves Fisher efficiency: it converges as fast as any second-order method, without computing the Hessian explicitly.

**Application — Neural Networks:** Amari showed natural gradient descent for neural networks (using the Fisher information of the network output distribution) can significantly accelerate learning and avoids ill-conditioning from layered parametrization.

---

## 20.7 Hypothesis Testing and Information Geometry

**Theorem 20.7.1 (Stein's Lemma / Sanov's Theorem).** Let $X_1, \ldots, X_n$ be i.i.d. from $P$. The optimal exponent for the type II error probability in testing $H_0: P = Q_0$ vs $H_1: P = Q_1$ (while keeping type I error $\leq \alpha$) is:
$$-\frac{1}{n}\log P_{\text{type II}} \to D_{\text{KL}}(Q_0 \| Q_1) \quad \text{as } n \to \infty.$$

**Theorem 20.7.2 (Chernoff Information).** The Chernoff information $C^* = -\min_{0 \leq \lambda \leq 1} \log\sum_x p(x)^\lambda q(x)^{1-\lambda}$ is the optimal exponent for the minimum error probability (regardless of type I/II split) in testing $H_0: P$ vs $H_1: Q$.

Information geometry gives a unified framework: $D_{\text{KL}}(P\|Q)$ is the geodesic distance from $P$ to $Q$ under the $m$-connection; Chernoff information minimizes a $\lambda$-mixture divergence.

---

## Exercises

**Exercise 20.1.** Compute the Fisher information $g(\theta)$ for: (a) $\text{Bernoulli}(\theta)$; (b) $N(\mu, \sigma^2)$; (c) $\text{Poisson}(\lambda)$. Verify that for the Gaussian, $g$ is diagonal in natural coordinates.

**Exercise 20.2.** Show that the Cramér-Rao bound is tight for the MLE of the Bernoulli parameter: compute the variance of $\hat\theta = \bar{X}_n$ and compare to $1/(n g(\theta))$.

**Exercise 20.3.** For the exponential family, prove that $\nabla_\theta \psi = E_\theta[T(X)]$ and $\nabla^2_\theta \psi = \text{Cov}_\theta[T(X)]$. Conclude that $\psi$ is convex.

**Exercise 20.4.** (Pythagorean Theorem) Prove that the KL divergence between a Gaussian $q = N(\mu, \sigma^2)$ and the MLE projection onto a Gaussian family $\mathcal{E}$ (given i.i.d. data) satisfies the Pythagorean decomposition.

**Exercise 20.5.** (EM) Implement the EM algorithm for Gaussian mixture models on a simple 1D dataset. Interpret each E-step and M-step geometrically as alternating projections.

---

## Chapter Notes

Amari and Nagaoka's *Methods of Information Geometry* is the standard reference, translated from Japanese. Amari's *Information Geometry and Its Applications* (2016) is the updated treatment. Ay, Jost, Lê, Schwachhöfer's *Information Geometry* (2017) provides the most rigorous mathematical treatment.

For the connection to optimal transport: Villani's *Optimal Transport: Old and New* and the Jordan-Kinderlehrer-Otto paper on the JKO scheme. The survey by Peyré and Cuturi (*Computational Optimal Transport*) is available online.

Information geometry has become central to machine learning, particularly in natural gradient methods (Amari 1998) and variational inference (where the ELBO is a KL divergence minimization problem solvable by $e$-projections).

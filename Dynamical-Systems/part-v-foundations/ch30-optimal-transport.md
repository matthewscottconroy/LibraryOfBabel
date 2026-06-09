# Chapter 30 — Optimal Transport and Dynamical Systems

> *Moving a pile of dirt to a hole: this is the Monge problem. Kantorovich relaxed it to couplings. Villani showed the optimal coupling has geometric regularity. The Wasserstein distance is the geometry of probability measures — and it connects directly to the geometry of dynamical systems.*

**Prerequisites:** Chapter 2 (measure theory, weak convergence), Chapter 5 (linear algebra, optimization), Chapter 16 (entropy, KL divergence).

---

## 30.1 The Monge-Kantorovich Problem

### 30.1.1 Formulation

**Definition 30.1.1 (Transport Plan).** Given probability measures $\mu, \nu$ on metric spaces $X, Y$, a *transport plan* (or *coupling*) is a probability measure $\pi$ on $X \times Y$ with marginals $\pi_X = \mu$ and $\pi_Y = \nu$:
$$\int_Y d\pi(x,y) = d\mu(x), \quad \int_X d\pi(x,y) = d\nu(y).$$

**Definition 30.1.2 (Kantorovich Problem).** For a cost function $c: X \times Y \to {\mathbb R}_{\geq 0}$, the *optimal transport cost* is:
$$W_c(\mu, \nu) = \inf_{\pi \in \Pi(\mu,\nu)} \int_{X\times Y} c(x,y)\,d\pi(x,y).$$

**Definition 30.1.3 (Wasserstein Distance).** For $p \geq 1$ and metric space $(X,d)$:
$$W_p(\mu, \nu) = \left(\inf_{\pi \in \Pi(\mu,\nu)} \int_{X\times X} d(x,y)^p\,d\pi(x,y)\right)^{1/p}.$$

$W_1$ is called the *Earth mover's distance*; $W_2$ has the richest geometry.

### 30.1.2 Duality — The Kantorovich-Rubinstein Theorem

**Theorem 30.1.4 (Kantorovich-Rubinstein Duality).** For the $W_1$ distance:
$$W_1(\mu, \nu) = \sup_{\|f\|_{\text{Lip}} \leq 1} \left\{\int f\,d\mu - \int f\,d\nu\right\},$$
where the sup is over all 1-Lipschitz functions $f: X \to {\mathbb R}$.

**Theorem 30.1.5 (General Duality — Kantorovich).** For general cost $c$:
$$W_c(\mu, \nu) = \sup_{\varphi \oplus \psi \leq c} \left\{\int \varphi\,d\mu + \int \psi\,d\nu\right\},$$
where the sup is over all $\varphi \in L^1(\mu)$, $\psi \in L^1(\nu)$ with $\varphi(x) + \psi(y) \leq c(x,y)$.

The dual variables $\varphi, \psi$ are *Kantorovich potentials*.

---

## 30.2 Brenier's Theorem and Geometry

**Theorem 30.2.1 (Brenier, 1991).** For $\mu, \nu$ probability measures on ${\mathbb R}^n$ with $\mu$ absolutely continuous w.r.t. Lebesgue, the optimal transport map for cost $c(x,y) = |x-y|^2$ is unique and equals the gradient of a convex function:
$$T^* = \nabla\phi$$
for some convex $\phi: {\mathbb R}^n \to {\mathbb R}$ (the *Brenier potential*). Moreover, $T^*_\# \mu = \nu$ and $T^*$ is the unique map with this property that is a gradient of a convex function.

**Theorem 30.2.2 (Polar Factorization).** Every diffeomorphism $u: {\mathbb R}^n \to {\mathbb R}^n$ (pushing Lebesgue to Lebesgue) factors as:
$$u = \nabla\phi \circ s$$
where $\nabla\phi$ is an optimal transport map and $s$ is a measure-preserving map. This is the *polar factorization* — the analogue of polar decomposition for maps.

**Remark 30.2.3.** The Brenier polar factorization shows that optimal transport provides a canonical decomposition of any diffeomorphism into a "gradient part" (conservative, carrying mass efficiently) and a "volume-preserving part" (conservative, rearranging mass without transport cost).

---

## 30.3 Wasserstein Space as a Metric Space

**Theorem 30.3.1 (Wasserstein Space).** The space $(\mathcal{P}_p(X), W_p)$ of probability measures on a complete separable metric space $X$ with finite $p$-th moments is a complete separable metric space. Convergence in $W_p$ is equivalent to weak convergence plus convergence of $p$-th moments.

**Theorem 30.3.2 (Geodesics in Wasserstein Space).** For $\mu, \nu \in \mathcal{P}_2({\mathbb R}^n)$ with $\mu$ absolutely continuous, the unique $W_2$-geodesic is:
$$\mu_t = ((1-t)\text{id} + t\nabla\phi)_\# \mu, \quad t \in [0,1],$$
where $\nabla\phi$ is the Brenier potential. The geodesic is "straight line" in the sense that mass moves along straight paths.

**Theorem 30.3.3 (Otto's Riemannian Structure).** The Wasserstein space $(\mathcal{P}_2({\mathbb R}^n), W_2)$ has a (formal) Riemannian structure: the tangent space at $\mu$ is $\{s: \int s\,d\mu = 0\}$ (zero-mean functions), with inner product:
$$\langle s_1, s_2 \rangle_\mu = \int \nabla\phi_1 \cdot \nabla\phi_2\,d\mu$$
where $\nabla\phi_i$ are the "velocity fields" solving continuity equations. The Riemannian metric gives $W_2$.

---

## 30.4 Gradient Flows in Wasserstein Space

**Definition 30.4.1.** A curve $(\mu_t)$ in $\mathcal{P}_2({\mathbb R}^n)$ is a *gradient flow* of a functional $\mathcal{F}$ in Wasserstein space if it satisfies the continuity equation:
$$\partial_t \mu_t + \nabla \cdot (\mu_t v_t) = 0, \quad v_t = -\nabla \frac{\delta\mathcal{F}}{\delta\mu}\bigg|_{\mu_t}.$$

**Theorem 30.4.2 (Jordan-Kinderlehrer-Otto, 1998).** The Fokker-Planck equation:
$$\partial_t \rho = \nabla \cdot (\rho \nabla V) + \Delta\rho$$
(describing diffusion in a potential $V$) is the gradient flow in $\mathcal{P}_2({\mathbb R}^n)$ of the *free energy functional*:
$$\mathcal{F}(\rho) = \int V(x)\rho(x)\,dx + \int \rho(x)\log\rho(x)\,dx.$$

**Theorem 30.4.3 (Heat Equation as Wasserstein Gradient Flow).** The heat equation $\partial_t \rho = \Delta\rho$ is the gradient flow of the *Boltzmann entropy* $H(\rho) = \int \rho\log\rho\,dx$ in $\mathcal{P}_2({\mathbb R}^n)$.

---

## 30.5 Entropy and Curvature — Lott-Sturm-Villani

**Definition 30.5.1 (Displacement Convexity).** A functional $\mathcal{F}: \mathcal{P}_2(X) \to {\mathbb R} \cup \{+\infty\}$ is *$K$-displacement convex* if along any $W_2$-geodesic $(\mu_t)$:
$$\mathcal{F}(\mu_t) \leq (1-t)\mathcal{F}(\mu_0) + t\mathcal{F}(\mu_1) - \frac{K}{2}t(1-t)W_2(\mu_0,\mu_1)^2.$$

**Theorem 30.5.2 (Lott-Sturm-Villani, 2006).** A smooth Riemannian manifold $(M, g)$ has Ricci curvature $\geq K$ iff the Boltzmann entropy $S(\mu) = \int \rho\log\rho\,dx$ is $K$-displacement convex in $(\mathcal{P}_2(M), W_2)$.

**Corollary 30.5.3.** The *synthetic Ricci curvature bound* $\text{Ric} \geq K$ for metric measure spaces is defined via displacement convexity of entropy in Wasserstein space. This allows defining Ricci curvature for non-smooth spaces (graphs, fractals, etc.).

---

## 30.6 Optimal Transport and Information Theory

**Theorem 30.6.1 (Talagrand Inequality).** For a Gaussian measure $\gamma$ on ${\mathbb R}^n$ (with variance $\sigma^2$):
$$W_2(\mu, \gamma)^2 \leq 2\sigma^2 D_{KL}(\mu \| \gamma).$$

More generally, a probability measure $\nu$ satisfies the *Talagrand transport inequality* $T_1(C)$ if:
$$W_1(\mu, \nu)^2 \leq 2C \cdot D_{KL}(\mu \| \nu)$$
for all $\mu$.

**Theorem 30.6.2 (Pinsker's Inequality).** $W_1(\mu, \nu) \leq \sqrt{\frac{1}{2}D_{KL}(\mu \| \nu)}$ (up to constants).

**Application 30.6.3 (Concentration of Measure).** If $\nu$ satisfies $T_1(C)$ and $f$ is 1-Lipschitz, then:
$$\nu(\{x : |f(x) - \mathbb{E}f| > t\}) \leq 2e^{-t^2/(2C)}.$$

This is the *Gaussian concentration inequality* — optimal transport bounds give measure concentration.

---

## Exercises

**Exercise 30.1.** (Earth Mover's Distance) Compute $W_1(\mu, \nu)$ for $\mu = \frac{1}{3}(\delta_0 + \delta_1 + \delta_2)$ and $\nu = \frac{1}{2}(\delta_0 + \delta_3)$ on ${\mathbb R}$. Find the optimal transport plan.

**Exercise 30.2.** (Brenier's Theorem) Find the optimal transport map from $\mu = \text{Uniform}([-1,1])$ to $\nu = \text{Uniform}([0,2])$ for cost $c(x,y) = |x-y|^2$.

**Exercise 30.3.** Verify the Jordan-Kinderlehrer-Otto theorem for the Gaussian: if $\mu_t = \mathcal{N}(0, \sigma(t)^2)$ is the solution of the heat equation (so $\sigma(t)^2 = 1 + 2t$), compute $\frac{d}{dt}H(\mu_t)$ and verify it equals minus the Wasserstein gradient of $H$.

**Exercise 30.4.** (Talagrand) Use the Talagrand inequality to prove the Gaussian concentration inequality: $\gamma(\{x: |f(x) - m| > t\}) \leq 2e^{-t^2/2}$ for any 1-Lipschitz $f$ and $\gamma = \mathcal{N}(0,1)$, where $m$ is the median.

---

## Chapter Notes

The foundational texts are Villani's *Topics in Optimal Transportation* (2003) and the longer *Optimal Transport: Old and New* (2009). The Jordan-Kinderlehrer-Otto paper (1998) is *The Variational Formulation of the Fokker-Planck Equation* (SIAM J. Math. Anal.).

The Lott-Sturm-Villani theory is in Lott-Villani's *Ricci Curvature for Metric-Measure Spaces via Optimal Transport* (Ann. Math. 2009) and Sturm's two papers *On the geometry of metric measure spaces I, II* (Acta Math. 2006).

The connection to machine learning: Wasserstein distances are the basis of Wasserstein GANs (Arjovsky et al., 2017) and optimal transport in ML (Peyré-Cuturi, *Computational Optimal Transport*, 2019).

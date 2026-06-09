# 20.2 Exponential Families

Exponential families are the most important class of parametric distributions in statistics. They include Gaussians, Bernoullis, Poissons, exponentials, Dirichlet, Gamma, and essentially every named distribution used in practice. From the information-geometric perspective, they are special because they are *flat* — in a sense we will make precise in Section 20.3.

**Definition 20.2.1 (Exponential Family).** A family $\{p_\theta\}$ is an *exponential family* if:
$$p_\theta(x) = h(x) \exp(\theta \cdot T(x) - \psi(\theta)),$$
where:
- $\theta \in \mathbb{R}^n$ is the *natural parameter*
- $T(x): \mathcal{X} \to \mathbb{R}^n$ is the *sufficient statistic*
- $\psi(\theta) = \log \int h(x) e^{\theta \cdot T(x)} dx$ is the *log-partition function* (normalization)

**Examples:**
- Gaussian: $\theta = (\mu/\sigma^2, -1/(2\sigma^2))$, $T(x) = (x, x^2)$
- Bernoulli: $\theta = \log(p/(1-p))$ (log-odds), $T(x) = x$
- Exponential: $\theta = -\lambda$ (negative rate), $T(x) = x$
- Dirichlet, Poisson, Multinomial — all exponential families

**Key Properties:**

The log-partition function $\psi$ encodes all the moments through its derivatives:

- $\nabla_\theta \psi(\theta) = E_\theta[T(X)]$ (the gradient of $\psi$ is the mean of the sufficient statistic)
- $\nabla^2_\theta \psi(\theta) = \text{Cov}_\theta[T(X)] = g(\theta)$ (the Hessian of $\psi$ is the Fisher information matrix!)
- $\psi$ is convex

The fact that the Hessian of $\psi$ equals the Fisher information is deep and beautiful: the curvature of the log-partition function as a function of the natural parameter is exactly the Fisher information. This is the key to the dually flat geometry of exponential families.

**Dual Parametrization:**

Every exponential family admits a second natural parametrization, the *mean parametrization*. The *mean parameter* is:
$$\eta = E_\theta[T(X)] = \nabla\psi(\theta).$$

Since $\psi$ is strictly convex, the map $\theta \mapsto \eta$ is a bijection (by the bijectivity of the gradient of a strictly convex function). The two parametrizations, $\theta$ (natural) and $\eta$ (mean), are related by the *Legendre transform*:
$$\varphi(\eta) = \sup_\theta [\theta \cdot \eta - \psi(\theta)].$$

The Legendre transform $\varphi$ turns out to be the *negative entropy*: $\varphi(\eta) = -H(p_\theta)$ as a function of the mean parameter $\eta$. This connects the geometry of the exponential family directly to entropy.

The natural parameters $\theta$ and mean parameters $\eta$ play dual roles: $\theta$ is the coordinate system for the *exponential connection* (flat), while $\eta$ is the coordinate system for the *mixture connection* (also flat). We make this precise in Section 20.3.

For practical data analysis, exponential families are special because sufficient statistics exist: given $T(X_1) + \cdots + T(X_n)$ (the sum of sufficient statistics), you can compute the MLE of $\theta$ without needing the individual data points. The MLE is simply the parameter $\hat\theta$ for which $E_{\hat\theta}[T(X)] = \bar{T}$ (the sample mean of the sufficient statistic equals the model's predicted mean). This is the method of moments, and for exponential families it coincides with maximum likelihood.

# Exercises — Chapter 11

The problems below span the chapter's main themes: verifying chaos axioms directly, computing fractal dimensions, working with the logistic map's conjugacy, and thinking carefully about what information-theoretic bounds on prediction actually say. Several problems will require numerical experimentation alongside the analysis.

---

**Exercise 11.1.** Show that the tent map $T(x) = 1 - |2x-1|$ on $[0,1]$ is Devaney chaotic by verifying all three conditions explicitly.

**Exercise 11.2.** (Hausdorff Dimension) Compute $\dim_H(C_\lambda)$ where $C_\lambda$ is the Cantor set formed by removing the middle $\lambda$-fraction at each stage. (*Hint:* Use the self-similarity: $C_\lambda$ is covered by $2^n$ intervals of length $((1-\lambda)/2)^n$.)

**Exercise 11.3.** For the Hénon map at $a = 1.4$, $b = 0.3$: compute numerically the two Lyapunov exponents $\lambda_1 > 0 > \lambda_2$ and estimate $\lambda_1 + \lambda_2$ (should be $\log |b| = \log 0.3 \approx -1.2$). Estimate the Kaplan-Yorke dimension.

**Exercise 11.4.** Show that the logistic map $f_4$ with the arcsine invariant measure satisfies Pesin's formula: compute $\int \log |f_4'(x)|\,d\mu_{\text{arc}}(x)$ and show it equals $\log 2$.

**Exercise 11.5.** (Multifractal) For a Bernoulli measure $\mu_p = (p, 1-p)$ on $\{0,1\}^{\mathbb N}$ (the doubling map attractor): show the local dimension of $\mu_p$ at a point $x$ with asymptotic frequency $\rho$ of 1s is $\alpha(x) = -\rho \log p - (1-\rho)\log(1-p)$. Compute $f(\alpha) = \dim_H\{\alpha(x) = \alpha\}$ and show it is the entropy function $-\rho\log\rho - (1-\rho)\log(1-\rho)$ (a Legendre transform of $\alpha$).

**Exercise 11.6.** Derive the Lorenz equations from the Navier-Stokes equations via the Galerkin truncation (Fourier modes). What does the truncation ignore?

**Exercise 11.7.** (Predictability) An ODE has maximal Lyapunov exponent $\lambda = 1/\text{day}$. You can measure initial conditions to accuracy $\varepsilon = 10^{-6}$ (in normalized units), and need prediction accuracy $L = 0.1$. What is the predictability horizon? By what factor would you need to improve measurement accuracy to double the predictability horizon?

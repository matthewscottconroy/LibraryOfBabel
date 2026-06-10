# Chapter 28 Exercises: Statistical Learning Theory for Reservoir Computing

## Section 28.1 — PAC Learning and VC Dimension

**Exercise 28.1.** (Warmup) Verify the union-bound derivation of the finite-class PAC sample complexity. Starting from the inequality $\Pr[\exists h : \hat{\mathcal{L}}(h) = 0,\, \mathcal{L}_\mathcal{D}(h) > \varepsilon] \leq |\mathcal{H}|e^{-\varepsilon m}$, show that setting the right-hand side equal to $\delta$ and solving for $m$ yields $m \geq \frac{1}{\varepsilon}(\ln|\mathcal{H}| + \ln\frac{1}{\delta})$.

**Exercise 28.2.** (VC dimension) Show that the VC dimension of the class of halfspaces $\{\mathbf{x} \mapsto \mathrm{sign}(\mathbf{w}^T\mathbf{x} + b) : \mathbf{w} \in \mathbb{R}^N, b \in \mathbb{R}\}$ is $N+1$, not $N$. What does this mean for the PAC sample complexity of a reservoir with bias term in the readout?

**Exercise 28.3.** Consider a reservoir with $N = 200$ neurons used for binary classification. Using the PAC bound $m(\varepsilon, \delta) = O((N/\varepsilon)\ln(N/\varepsilon) + (1/\varepsilon)\ln(1/\delta))$, compute the required number of training examples to guarantee $\varepsilon = 0.05$ with $\delta = 0.01$. Compare this to the practical rule of thumb $T \approx 10N$.

**Exercise 28.4.** (Sauer-Shelah) Apply the Sauer-Shelah lemma to bound $\Pi_{\mathcal{H}_N}(m)$ for the halfspace class with VC dimension $N$. For $m = 2N$, compute the upper bound and compare to $2^{2N}$ (the maximum possible value).

**Exercise 28.5** (Hard). Show that the VC dimension of the class of functions $\{u \mapsto \mathrm{sign}(f(\mathbf{x}(u))) : \mathbf{w} \in \mathbb{R}^N, \|\mathbf{w}\|_2 \leq B\}$ restricted to inputs $u$ with $\|\mathbf{x}(u)\|_2 \leq R$ satisfies $d_\mathrm{VC} \leq (BR)^2$. *Hint: use the fat-shattering dimension and Sauer-Shelah.*

## Section 28.2 — Rademacher Complexity

**Exercise 28.6.** Verify the computation in Theorem 28.4: show that $\sup_{\|\mathbf{w}\|_2 \leq B} \frac{1}{m}\sum_i \sigma_i \mathbf{w}^T\mathbf{x}_i = \frac{B}{m}\|\sum_i \sigma_i \mathbf{x}_i\|_2$. Use the Cauchy-Schwarz inequality.

**Exercise 28.7.** Consider an ESN with $N = 500$ neurons and typical state norm $\|\mathbf{x}(t)\|_2 \approx 5$. The readout weights satisfy $\|\mathbf{w}\|_2 \leq 0.1$ (enforced by ridge regression). Compute the Rademacher complexity bound for $T = 1000$ training steps. How does this compare to the VC bound?

**Exercise 28.8.** Suppose the reservoir states satisfy $\|\mathbf{x}(t)\|_2 \leq C$ almost surely for some constant $C$ independent of $N$ (e.g., because the dynamics contract the state). Show that the Rademacher complexity bound $O(BC/\sqrt{T})$ is independent of the reservoir size $N$.

**Exercise 28.9.** The generalization bound (Theorem 28.3) assumes $f \in [0,1]$. For a regression readout with outputs in $[-M, M]$, state the appropriate modification to the bound. By how much does the bound increase compared to the $[0,1]$ case?

**Exercise 28.10** (Hard). Derive the Rademacher complexity of the class of nonlinear readouts $\mathcal{F} = \{\mathbf{x} \mapsto g(\mathbf{w}^T\mathbf{x}) : \|\mathbf{w}\|_2 \leq B,\, g \text{ is } L\text{-Lipschitz}\}$ using the Talagrand contraction lemma. How does Lipschitz constant $L$ affect the complexity?

## Section 28.3 — Covering Numbers

**Exercise 28.11.** Verify the volumetric bound $\log\mathcal{N}(B\mathbb{B}_2^N, \varepsilon, \|\cdot\|_2) \leq N\log(3B/\varepsilon)$. *Hint: compute the ratio of volumes $\mathrm{vol}((B+\varepsilon)\mathbb{B}_2^N)/\mathrm{vol}(\varepsilon\mathbb{B}_2^N)$.*

**Exercise 28.12.** For a reservoir with state covariance $\Sigma = \mathrm{diag}(1, 1/2, 1/3, \ldots, 1/N)$, compute the stable rank $r(\Sigma) = \mathrm{tr}(\Sigma)/\|\Sigma\|_\mathrm{op}$. How does $r(\Sigma)$ grow with $N$?

**Exercise 28.13.** Apply Dudley's integral bound (Theorem 28.6) to the linear readout class with $N = 100$, $B = 1$, $R = \sqrt{N}$. Evaluate the integral $\int_0^{BR} \sqrt{N\log(3BR/\varepsilon)}\,d\varepsilon$ numerically and compare to the direct Rademacher bound.

**Exercise 28.14** (Research level). The Dudley integral bound is tight for Gaussian processes but can be loose for other function classes. Investigate whether the bound is tight for the reservoir linear readout class by comparing it to the Rademacher bound. Under what conditions on the eigenvalue spectrum is the Dudley bound tight?

## Section 28.4 — Double Descent

**Exercise 28.15.** Consider a reservoir with $N = 50$ neurons and $T = 100$ training examples. The state matrix $\mathbf{X} \in \mathbb{R}^{100 \times 50}$ has singular values $\sigma_i = i^{-1/2}$. Compute the ridge regression solution for $\lambda = 0.01$ and the OLS solution. Plot the test error as a function of $N$ (for varying reservoir sizes).

**Exercise 28.16.** Verify the SVD formula for the minimum-norm interpolating solution: $\hat{\mathbf{w}}_{\min} = \mathbf{X}^T(\mathbf{X}\mathbf{X}^T)^{-1}\mathbf{y}$. Show that this is the solution with minimum $\|\mathbf{w}\|_2$ among all solutions satisfying $\mathbf{X}\mathbf{w} = \mathbf{y}$.

**Exercise 28.17.** The benign overfitting condition requires $k^*(T) \gg T$ where $k^*(T) = \max\{k : \sum_{i>k}\lambda_i \geq k\lambda_k\}$. For eigenvalues $\lambda_i = i^{-\alpha}$, compute $k^*(T)$ as a function of $T$ and $\alpha$. For what values of $\alpha$ is benign overfitting possible?

**Exercise 28.18** (Simulation). Implement a simple ESN with $N \in \{10, 50, 100, 500, 1000\}$ neurons on the NARMA-5 task with fixed $T = 200$ training examples. Plot the test NRMSE as a function of $N$. Verify that the double-descent curve appears near $N = T = 200$.

## Section 28.5 — Implicit Regularization

**Exercise 28.19.** Prove that gradient descent initialized at $\mathbf{0}$ maintains $\mathbf{w}(k) \in \mathrm{rowspace}(\mathbf{X})$ for all $k$. *Hint: show by induction that each GD step adds a vector in $\mathrm{rowspace}(\mathbf{X})$.*

**Exercise 28.20.** Derive the equivalence between early stopping at step $k$ and ridge regression with $\lambda_k \approx 1/(\eta k)$. *Hint: write the GD iterate in the SVD basis and compare to the ridge regression formula in the same basis.*

**Exercise 28.21.** For the reservoir NTK $K_\mathrm{RC}(\mathbf{x}, \mathbf{x}') = \mathbf{x}^T\mathbf{x}'$, write the kernel regression solution for training examples $\{(\mathbf{x}(t), y(t))\}_{t=1}^T$. Show that it equals $\hat{\mathbf{w}}_{\min}^T\mathbf{x}'$ for test input $\mathbf{x}'$.

**Exercise 28.22** (Research). Investigate what implicit regularization is applied by the recursive least squares (RLS) algorithm when used to train a reservoir readout online. Does RLS select the minimum-norm solution? Does it select a different solution? Compare to offline ridge regression on the same data.

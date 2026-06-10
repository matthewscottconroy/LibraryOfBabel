# Chapter 27 Exercises

## Section 27.1: Wigner Semicircle Law

**Exercise 27.1.** *(Catalan numbers)* Prove that the number of non-crossing pair partitions of $\{1, 2, \ldots, 2k\}$ is the Catalan number $C_k = \frac{1}{k+1}\binom{2k}{k}$.
(a) First construct the bijection with Dyck paths (paths in $\mathbb{Z}$ of length $2k$ starting and ending at 0 and staying $\geq 0$).
(b) Show the number of Dyck paths satisfies the recurrence $C_{k+1} = \sum_{j=0}^k C_j C_{k-j}$ (split at the first return to 0).
(c) Solve this recurrence using generating functions to find $C_k = \frac{1}{k+1}\binom{2k}{k}$.

**Exercise 27.2.** *(Method of moments, full proof)* In the proof of the Wigner semicircle law, we argued that only non-crossing pair partitions contribute to the leading term. Make this precise:
(a) Show that if a path $(i_1, \ldots, i_{2k})$ traverses some edge three or more times, it can visit at most $k$ distinct vertices, contributing $O(N^k)$ to the sum (sublinear in $N^{k+1}$).
(b) Show that crossing pair partitions also contribute lower-order terms. (Hint: a crossing partition visits at most $k$ distinct vertices when $N$-dependent constraints are imposed.)

**Exercise 27.3.** *(Circular law)* Simulate an $N \times N$ random matrix $W$ with i.i.d. $\mathcal{N}(0, 1/N)$ entries for $N = 50, 200, 1000$. Plot the empirical distribution of eigenvalues in the complex plane. Compare to the theoretical circular law (uniform distribution on the unit disk). At what $N$ does the simulation closely match the theory?

**Exercise 27.4.** *(Edge eigenvalues and Tracy-Widom)* The largest eigenvalue of a Wigner matrix satisfies $\lambda_{\max} = 2\sigma + \frac{\sigma}{N^{2/3}} \xi_{TW}$ where $\xi_{TW}$ follows the Tracy-Widom distribution [TracyWidom1994]. The Tracy-Widom distribution has mean $\approx -1.77$ and standard deviation $\approx 0.90$.
(a) For a reservoir of size $N = 100$ with target spectral radius $\rho = 0.9$ (so $\sigma = \rho/2 = 0.45$), what is the expected deviation of the actual spectral radius from the target?
(b) What is the probability that the actual spectral radius exceeds 1.0 (leading to instability), assuming the Tracy-Widom approximation?

**Exercise 27.5.** *(Effect of sparsity)* Many practical reservoirs are sparse: most entries of $W$ are zero, with only a fraction $p$ nonzero. For a sparse Wigner matrix with entries $W_{ij} = \xi_{ij} B_{ij}$ where $\xi_{ij} \sim \mathcal{N}(0, \sigma^2/p)$ and $B_{ij} \sim \text{Bernoulli}(p)$ independently, the semicircle law still holds (for $pN \to \infty$). Verify this claim numerically for $N = 500$ and $p \in \{0.01, 0.05, 0.2, 1.0\}$. At what sparsity level does the empirical distribution deviate noticeably from the semicircle?

## Section 27.2: Marchenko-Pastur Law

**Exercise 27.6.** *(Stieltjes transform)* Verify that the Marchenko-Pastur distribution with $c = 1$ has Stieltjes transform $m(z)$ satisfying $m(z) = 1/(-z + 1/(1+m(z)))$.
(a) Solve the resulting quadratic for $m(z)$ and verify the solution has $\text{Im}(m) > 0$ for $\text{Im}(z) > 0$.
(b) Use the Stieltjes inversion formula to recover the Marchenko-Pastur density for $c=1$.

**Exercise 27.7.** *(Effective rank)* Prove Proposition 27.2.1: for a random state matrix $X$ with aspect ratio $c = N/T$, the effective rank $r_{\text{eff}} = [\text{tr}(\hat{\Sigma})]^2 / \text{tr}(\hat{\Sigma}^2) \approx N/(1+c)$.
(a) Show that $r_{\text{eff}}$ depends only on the ratio of the first two spectral moments.
(b) Compute the first two moments of $\rho_{MP}(\cdot; c)$ and evaluate $r_{\text{eff}}$.
(c) What value of $c$ (i.e., what ratio $T/N$) gives effective rank $\geq 0.9N$?

**Exercise 27.8.** *(Optimal regularization)* Under the Marchenko-Pastur law, derive the optimal ridge regularization parameter $\lambda^*$ that minimizes the expected generalization error of the ridge regression readout. Assume the true output is $y = W^* x + \eta$ where $\eta \sim \mathcal{N}(0, \sigma_\eta^2)$ and $W^* \in \mathbb{R}^{1 \times N}$. Express $\lambda^*$ as a function of $c$, $\sigma_\eta^2$, and $\|W^*\|^2$.

**Exercise 27.9.** *(Marchenko-Pastur for correlated states)* The Marchenko-Pastur law assumes i.i.d. columns of $Z$. Reservoir states are not i.i.d. — they are correlated in time. Suppose the state process is an AR(1): $x(t) = \alpha x(t-1) + \sigma_\xi \xi(t)$ where $\xi(t)$ are i.i.d. $\mathcal{N}(0,I)$. How does the Marchenko-Pastur law generalize to correlated samples? State the relevant result (look up [PaulAue2014] or related literature) and describe how the edge values $x_\pm$ change as a function of $\alpha$.

## Section 27.3: Concentration Inequalities

**Exercise 27.10.** *(Sub-Gaussian vs. sub-exponential)* Let $X$ be a bounded random variable, $|X| \leq B$.
(a) Show that $X - \mathbb{E}[X]$ is sub-Gaussian with parameter $\sigma = B/2$.
(b) Show that $X^2 - \mathbb{E}[X^2]$ is sub-exponential with parameters $(B^2, B^2)$.
(c) Explain why products of bounded variables lead to sub-exponential (not sub-Gaussian) tails, and what this implies for bounds on $\text{tr}(\hat{\Sigma}^2)$.

**Exercise 27.11.** *(Matrix Bernstein application)* Let $x_1, \ldots, x_T \in \mathbb{R}^N$ be i.i.d. with $\|x_i\| \leq B$ and $\mathbb{E}[x_i x_i^\top] = \Sigma$. Apply the matrix Bernstein inequality to show:
$$\mathbb{P}\!\left(\left\|\frac{1}{T}\sum_{t=1}^T x_t x_t^\top - \Sigma\right\|_{\text{op}} \geq \varepsilon\right) \leq 2N \exp\!\left(-\frac{cT\varepsilon^2}{B^4 + B^2\varepsilon}\right).$$
State what $T$ is needed (as a function of $N$, $\varepsilon$, $\delta$, and $B$) to have this probability $\leq \delta$.

**Exercise 27.12.** *(Non-independent states)* The matrix Bernstein inequality assumes independent summands, but reservoir states are temporally correlated. Extend the bound to mixing processes: if the state process $\{x(t)\}$ is $\phi$-mixing with mixing coefficients $\phi(k) \leq C e^{-\gamma k}$, show that an analogous concentration inequality holds with $T$ replaced by an effective sample size $T_{\text{eff}} = T / (1 + 2\sum_{k=1}^\infty \phi(k)) \approx T / (1 + 2C/\gamma)$. (This requires the blocking argument for dependent variables; see [BertoniBraun2004].)

**Exercise 27.13.** *(Reservoir design via RMT)* You are designing a reservoir for a task requiring linear memory capacity $C_L = 50$ (the reservoir must accurately recall inputs from up to 50 time steps in the past). Using the RMT results from this chapter:
(a) What is the minimum reservoir size $N$ needed?
(b) What training sequence length $T$ is needed to estimate the capacity reliably (to within $\pm 5$)?
(c) If the input is white noise, what spectral radius should the reservoir have to maximize the memory capacity?

**Exercise 27.14.** *(Research problem)* *[Open-ended]* The concentration bounds in Section 27.3.5 scale as $O(N^2/T)$ in $\varepsilon^2$, meaning large reservoirs need quadratically more data. However, practical experience suggests that reservoirs generalize well even when $T \approx 10N$ rather than $T \approx N^2$. Propose a hypothesis for why the theoretical bounds are pessimistic. What structural property of reservoir states (beyond boundedness) might lead to sharper bounds? Sketch a research program to prove a tighter bound exploiting this structure.

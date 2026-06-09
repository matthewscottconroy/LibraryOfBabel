# Chapter 17 — Entropy and Its Generalizations

> *Shannon entropy is the unique entropy satisfying a natural set of axioms. Relax the axioms and you get a whole family of entropies — Rényi, Tsallis, min-entropy — each optimal for different operational tasks.*

**Prerequisites:** Chapter 16 (Shannon entropy, KL divergence).

**What this chapter builds:** The family of Rényi entropies interpolating between min-entropy and Shannon entropy; min-entropy as the correct quantity for one-shot cryptography; differential entropy for continuous sources; the maximum entropy principle; quantum von Neumann entropy; and the connections to statistical mechanics.

---

## 17.1 Rényi Entropy

**Definition 17.1.1.** The *Rényi entropy of order $\alpha$* ($\alpha > 0$, $\alpha \neq 1$) of a discrete random variable $X$ with pmf $p = (p_1, \ldots, p_n)$ is:
$$H_\alpha(X) = \frac{1}{1-\alpha} \log \sum_i p_i^\alpha.$$

**Limits:**
- As $\alpha \to 1$: $H_\alpha(X) \to H_1(X) = -\sum p_i \log p_i$ (Shannon entropy)
- As $\alpha \to 0$: $H_0(X) = \log |\text{supp}(X)|$ (Hartley entropy — log of support size)
- As $\alpha \to \infty$: $H_\infty(X) = -\log \max_i p_i$ (min-entropy)
- $\alpha = 2$: $H_2(X) = -\log \sum_i p_i^2$ (collision entropy)

**Theorem 17.1.2 (Monotonicity).** $H_\alpha(X)$ is non-increasing in $\alpha$: $\alpha < \beta \Rightarrow H_\alpha \geq H_\beta$.

**Theorem 17.1.3 (Rényi's Characterization).** The family $\{H_\alpha\}$ is the unique family (up to constants) satisfying: (1) symmetry in $(p_i)$, (2) chain rule $H_\alpha(X,Y) = H_\alpha(X) + \langle H_\alpha(Y|X = x)\rangle_\alpha$ for a weighted average, (3) normalization $H_\alpha(\frac{1}{2},\frac{1}{2}) = 1$.

---

## 17.2 Min-Entropy

**Definition 17.2.1.** The *min-entropy* of $X$ is:
$$H_\infty(X) = -\log \max_x p(x) = -\log \|p\|_\infty.$$

Min-entropy is the "worst-case" entropy: it measures the probability of the most likely outcome. A high min-entropy means even the most likely outcome is improbable — the source is "hard to guess."

### 17.2.1 Conditional Min-Entropy

**Definition 17.2.2.** The *conditional min-entropy* of $X$ given $Y$ (the correct "one-shot" version) is:
$$H_\infty(X|Y) = -\log \sum_y p(y) \max_x p(x|y) = -\log \mathbb{E}_Y[\max_x p(X|Y)].$$

*Note: This is NOT $-\log \max_{x,y} p(x|y)$.* The correct definition uses an average over $Y$.

**Smooth Min-Entropy:**

**Definition 17.2.3.** The *$\varepsilon$-smooth min-entropy* is:
$$H_\infty^\varepsilon(X) = \max_{\tilde{p}: \|p - \tilde{p}\|_1 \leq \varepsilon} H_\infty(\tilde{p}).$$

Smooth min-entropy is the right quantity for privacy amplification and one-shot source coding.

**Theorem 17.2.4 (Operational Meaning — Privacy Amplification).** Given $n$ uses of a source with smooth min-entropy $H_\infty^\varepsilon(X^n)$, one can extract $k \leq H_\infty^\varepsilon(X^n) - 2\log(1/\varepsilon)$ bits that are statistically close to uniform (for a 2-universal hash function).

---

## 17.3 Rényi Divergence

**Definition 17.3.1.** The *Rényi divergence of order $\alpha$* between distributions $P$ and $Q$ is:
$$D_\alpha(P \| Q) = \frac{1}{\alpha - 1} \log \sum_x p(x)^\alpha q(x)^{1-\alpha}.$$

Limits: $D_1(P\|Q) = D_{\text{KL}}(P\|Q)$, $D_\infty(P\|Q) = \log \max_x p(x)/q(x)$.

**Theorem 17.3.2 (Data Processing Inequality for Rényi).** For all $\alpha \geq 0$: $D_\alpha(P_{f(X)} \| Q_{f(X)}) \leq D_\alpha(P_X \| Q_X)$ for any measurable function $f$.

---

## 17.4 Differential Entropy

**Definition 17.4.1.** For a continuous random variable $X$ with density $f(x)$, the *differential entropy* is:
$$h(X) = -\int f(x)\log f(x)\,dx.$$

**Warning:** Differential entropy is NOT the limit of discrete entropy as the quantization gets finer: it can be negative (e.g., $X \sim U[0, 1/2]$ has $h(X) = -\log 2 < 0$). It is not invariant under smooth reparametrization.

**Examples:**
- $X \sim N(\mu, \sigma^2)$: $h(X) = \frac{1}{2}\log(2\pi e \sigma^2)$
- $X \sim U[a, b]$: $h(X) = \log(b-a)$
- $X \sim \text{Exp}(\lambda)$: $h(X) = 1 - \log\lambda$

**Theorem 17.4.2 (Gaussian Maximizes Entropy).** Among all distributions with fixed mean $\mu$ and variance $\sigma^2$, the Gaussian $N(\mu, \sigma^2)$ maximizes differential entropy.

---

## 17.5 The Maximum Entropy Principle

**Definition 17.5.1 (MaxEnt — Jaynes).** Given a set of moment constraints $E[g_k(X)] = c_k$ for $k = 1, \ldots, m$, the *maximum entropy distribution* is the one maximizing $H(X)$ (or $h(X)$) subject to the constraints.

**Theorem 17.5.2 (Gibbs / Jaynes).** The maximum entropy distribution subject to moment constraints $E[g_k(X)] = c_k$ has the *exponential family* form:
$$p^*(x) = \frac{1}{Z(\lambda)} \exp\left(-\sum_k \lambda_k g_k(x)\right),$$
where $Z(\lambda) = \int \exp(-\sum \lambda_k g_k)\,dx$ (partition function) and $\lambda_k$ are Lagrange multipliers.

**Examples:**
- Constraint $E[X] = \mu$ only: exponential distribution $p(x) = \lambda e^{-\lambda x}$ for $x \geq 0$.
- Constraints $E[X] = \mu$, $\text{Var}(X) = \sigma^2$: Gaussian distribution.
- No constraints (finite support): uniform distribution.
- Constraint $p(x) = $ given for all $x$: that distribution itself.

**Boltzmann-Gibbs:** In statistical mechanics, the equilibrium distribution of a system at temperature $T$ is the maximum entropy distribution subject to the energy constraint $E[H(x)] = \bar{E}$ — giving the Boltzmann distribution $p(x) \propto e^{-H(x)/kT}$.

---

## 17.6 Tsallis Entropy

**Definition 17.6.1.** The *Tsallis entropy of order $q$* is:
$$S_q(X) = \frac{1}{q-1}\left(1 - \sum_i p_i^q\right) = \frac{1 - \text{Tr}[\rho^q]}{q-1}.$$

(As $q \to 1$: $S_q \to H$ (Shannon). For $q = 2$: $S_2 = 1 - \sum p_i^2$ (linear entropy).)

Tsallis entropy is *non-extensive*: $S_q(X,Y) = S_q(X) + S_q(Y) + (1-q)S_q(X)S_q(Y)$ for independent $X, Y$.

**Application:** Tsallis entropy appears in non-equilibrium statistical mechanics, long-range interacting systems, and multi-fractal distributions. The maximizing distribution for Tsallis entropy with energy constraint is a *q-exponential* (power law) rather than the Boltzmann exponential.

---

## 17.7 Von Neumann Entropy

**Definition 17.7.1.** The *von Neumann entropy* of a quantum state $\rho$ (a density matrix, $\rho \geq 0$, $\text{Tr}[\rho] = 1$) is:
$$S(\rho) = -\text{Tr}[\rho \log \rho] = -\sum_i \lambda_i \log \lambda_i,$$
where $\lambda_i$ are the eigenvalues of $\rho$.

If $\rho = \sum_i \lambda_i |\psi_i\rangle\langle\psi_i|$, then $S(\rho) = H(\lambda_1, \ldots, \lambda_n)$ (Shannon entropy of eigenvalue distribution).

**Properties:**
1. $S(\rho) = 0$ iff $\rho$ is a pure state ($\rho = |\psi\rangle\langle\psi|$, rank 1).
2. $S(\rho) \leq \log d$ ($d$ = dimension), with equality iff $\rho = I/d$ (maximally mixed).
3. *Concavity*: $S(\sum_i p_i \rho_i) \geq \sum_i p_i S(\rho_i)$ (mixing increases entropy).
4. *Unitary invariance*: $S(U\rho U^\dagger) = S(\rho)$.

**Theorem 17.7.2 (Strong Subadditivity — Lieb-Ruskai 1973).** For a tripartite system $ABC$:
$$S(\rho_{AB}) + S(\rho_{BC}) \geq S(\rho_{ABC}) + S(\rho_B).$$

This is the deepest property of quantum entropy. It implies: $S(\rho_{AB}) + S(\rho_{BC}) \geq S(\rho_{AC})$ (strong subadditivity) and $S(\rho_{AB}) \leq S(\rho_A) + S(\rho_B)$ (weak subadditivity).

**Remark 17.7.3.** Strong subadditivity is equivalent to the *monotonicity of relative entropy*: $D(\mathcal{E}(\rho) \| \mathcal{E}(\sigma)) \leq D(\rho \| \sigma)$ for any quantum channel $\mathcal{E}$. The proof (Lieb-Ruskai) uses complex interpolation theory.

---

## 17.8 Connections Between Entropy Measures

**Pinsker's Inequality:**
$$\|P - Q\|_1 \leq \sqrt{2 D_{\text{KL}}(P\|Q)}.$$
KL divergence controls the $L^1$ distance.

**Bretagnolle-Huber-Carol:**
$$\|P - Q\|_1^2 \leq 2(1 - e^{-D_{\text{KL}}(P\|Q)}).$$

**Min-Entropy and Rényi:**
$$H_\infty(X) \leq H(X) \leq H_\alpha(X) \leq H_0(X) \quad \text{for } \alpha \leq 1.$$

**Chain Rule Comparison:**
- Shannon: $H(X|Y) = H(X,Y) - H(Y)$ (exact)
- Min-entropy: $H_\infty(X|Y) \leq H_\infty(X,Y) - H_\infty(Y)$ (only inequality)
- No clean chain rule for Rényi in general

---

## Exercises

**Exercise 17.1.** Compute $H_\alpha(X)$ for $\alpha = 0, 1/2, 1, 2, \infty$ for the distribution $(1/2, 1/4, 1/4)$. Verify monotonicity in $\alpha$.

**Exercise 17.2.** Show that for a fair coin $X \sim \text{Bernoulli}(1/2)$: $H_\alpha(X) = 1$ for all $\alpha > 0$. Interpret: uniform distributions have all Rényi entropies equal.

**Exercise 17.3.** Derive the maximum entropy distribution (Theorem 17.5.2) using Lagrange multipliers. Verify for the case of one constraint $E[X] = \mu$ on $[0, \infty)$.

**Exercise 17.4.** Prove that the Gaussian $N(\mu, \sigma^2)$ maximizes differential entropy among distributions with variance $\sigma^2$, using the fact that $D_{\text{KL}}(f \| g_\sigma) \geq 0$ where $g_\sigma$ is the Gaussian density.

**Exercise 17.5.** (Multifractal Connection) For the Bernoulli measure $\mu_p$ on the doubling map, the $q$-th Rényi entropy of the partition $\xi_n = \{[(k-1)/2^n, k/2^n] : k = 1, \ldots, 2^n\}$ is $H_q(\xi_n) = \frac{1}{1-q}\log \sum_k \mu_p(A_k)^q$. Compute this and connect it to the Rényi dimension $D_q$ of $\mu_p$.

**Exercise 17.6.** (Von Neumann) For a qubit $\rho = \begin{pmatrix}p & 0 \\ 0 & 1-p\end{pmatrix}$: compute $S(\rho)$. For the Bell state $\rho_{AB} = |\Phi^+\rangle\langle\Phi^+|$ where $|\Phi^+\rangle = (|00\rangle + |11\rangle)/\sqrt{2}$: compute $S(\rho_{AB})$, $S(\rho_A)$, $S(\rho_B)$, and verify strong subadditivity.

---

## Chapter Notes

Rényi's original paper *On Measures of Entropy and Information* (1961) introduced the family $H_\alpha$ and the axiomatic characterization. The min-entropy and smooth min-entropy (Section 17.2) are from Renner's PhD thesis *Security of Quantum Key Distribution* (2005) — the foundational work of one-shot information theory.

For von Neumann entropy and strong subadditivity: the original Lieb-Ruskai proof (1973) used a deep theorem of Lieb about convex trace functions. A simpler proof using *monotonicity of quantum relative entropy* was later given by Lindblad and by Uhlmann.

The maximum entropy principle (Section 17.5) is from Jaynes' 1957 papers — an influential but sometimes controversial interpretation of statistical mechanics as inference under constraints.

# Chapter 23 — Ergodic Information Theory

> *The Shannon-McMillan-Breiman theorem is the ergodic-theoretic AEP: for a stationary ergodic process, the information per symbol concentrates at the entropy rate — almost surely. This is Birkhoff's theorem for the information function.*

**Prerequisites:** Chapter 7 (ergodic theory, Birkhoff's theorem), Chapter 16 (Shannon entropy, AEP).

---

## 23.1 Stationary Processes and Their Entropy

**Definition 23.1.1.** A *stationary stochastic process* $(X_n)_{n \in {\mathbb Z}}$ is one where the joint distribution of $(X_{n+k_1}, \ldots, X_{n+k_m})$ is independent of $n$ for any $m$ and $k_1, \ldots, k_m$.

**Key Example:** Any MPT $(X, \mathcal{B}, \mu, f)$ with a measurable function $\varphi: X \to \mathcal{A}$ (an alphabet) defines a stationary process $Y_n = \varphi(f^n(x))$ via the *natural extension*.

**Definition 23.1.2.** The *entropy rate* of a stationary process $(X_n)$ is:
$$h = \lim_{n\to\infty} \frac{1}{n} H(X_1, \ldots, X_n).$$

The limit exists by subadditivity. Also $h = \lim_n H(X_n | X_{n-1}, \ldots, X_1)$ (the conditional entropy stabilizes).

**Theorem 23.1.3.** For an ergodic MPT $(X, \mathcal{B}, \mu, f)$ with generating partition $\xi$, the entropy rate of the process $(Y_n = \xi(f^n(x)))$ equals the KS entropy $h_\mu(f)$.

---

## 23.2 The Shannon-McMillan-Breiman Theorem

**Theorem 23.2.1 (Shannon-McMillan-Breiman).** Let $(X, \mathcal{B}, \mu, f)$ be an ergodic MPT and $\xi = \{A_1, \ldots, A_k\}$ a finite generating partition. Let $\xi_0^{n-1}(x) = A_{i_0} \cap f^{-1}A_{i_1} \cap \cdots \cap f^{-(n-1)}A_{i_{n-1}}$ be the atom of $\bigvee_{k=0}^{n-1}f^{-k}\xi$ containing $x$. Then:
$$-\frac{1}{n}\log \mu(\xi_0^{n-1}(x)) \to h_\mu(f) \quad \mu\text{-a.e.}$$

**Proof Sketch:**
1. The information function of the $n$-step partition is $I_n(x) = -\log \mu(\xi_0^{n-1}(x))$.
2. By the chain rule: $I_n(x) = \sum_{k=0}^{n-1} I(x | f^{-k}\xi \vee \cdots \vee f^{-(k-1)}\xi)$ (sum of conditional informations).
3. The conditional information $i_k(x) = -\log \mu(A_{i_k}(f^k(x)) | \xi_{k+1}^{n-1}(f^k(x)))$ is "almost" a function of $f^k(x)$ for large $k$.
4. Apply Birkhoff's theorem to show $\frac{1}{n}I_n(x) \to E[i_0] = h_\mu(f, \xi) = h_\mu(f)$ a.e.

**The SMB Theorem as Ergodic AEP:**

**Definition 23.2.2.** The *ergodic typical set* at level $n$ and tolerance $\varepsilon$ is:
$$A_\varepsilon^{(n)} = \left\{x \in X : \left|-\frac{1}{n}\log\mu(\xi_0^{n-1}(x)) - h\right| < \varepsilon\right\}.$$

By SMB:
1. $\mu(A_\varepsilon^{(n)}) \to 1$ as $n \to \infty$ for all $\varepsilon > 0$
2. The number of typical atoms: $|A_\varepsilon^{(n)} \cap \mathcal{P}^n| \leq 2^{n(h+\varepsilon)}$ and $\geq (1-\delta)2^{n(h-\varepsilon)}$ for large $n$
3. Each typical atom has measure $\approx 2^{-nh}$

---

## 23.3 Universal Source Coding for Stationary Ergodic Sources

**Theorem 23.3.1 (Lempel-Ziv is Universal).** The Lempel-Ziv algorithm (LZ78 or LZ77) achieves the optimal compression rate $h$ for any stationary ergodic source, without knowing $h$ in advance.

*(Ziv-Lempel 1978, Wyner-Ziv 1994)*

**Proof idea:** LZ parsing divides the sequence into phrases (longest phrases not seen before). By the SMB theorem, the number of phrases in a typical sequence of length $n$ is $\approx n h / \log n$. Each phrase is encoded with $\log$ of the number of phrases (dictionary pointer), giving compression rate $\to h$.

**Theorem 23.3.2 (Optimality of LZ).** No compression algorithm can achieve a rate below $h$ for a stationary ergodic source with entropy rate $h$ (by the SMB theorem — the source cannot be compressed below $h$ bits/symbol).

---

## 23.4 The Ergodic Theorem as a Generalization

The AEP and SMB theorem fit into a hierarchy:

- **LLN:** $(1/n)\sum X_i \to E[X]$ for i.i.d. (convergence in probability)
- **SLLN:** a.s. convergence
- **AEP:** $(1/n)\log p(X_1,\ldots,X_n) \to -h$ for i.i.d. processes (LLN for log-probabilities)
- **SMB:** a.s. convergence for stationary ergodic processes (Birkhoff for log-probabilities)

The SMB theorem is: *Birkhoff's ergodic theorem applied to the information function*.

---

## 23.5 Entropy Rate and the Ziv-Lempel Complexity

**Definition 23.5.1.** The *Lempel-Ziv complexity* $c_n(\omega)$ of a binary sequence $\omega_1 \cdots \omega_n$ is the number of phrases in its LZ parsing.

**Theorem 23.5.2 (Lempel-Ziv 1976).** For any stationary ergodic binary process with entropy rate $h$:
$$\lim_{n\to\infty} \frac{c_n(\omega)}{n/\log n} = h \quad \mu\text{-a.e.}$$

This gives an empirical estimator for entropy that works for ergodic processes without knowledge of the joint distribution.

---

## Exercises

**Exercise 23.1.** For a Markov chain with transition matrix $P = \begin{pmatrix}0.7 & 0.3 \\ 0.4 & 0.6\end{pmatrix}$ and stationary distribution $\pi$, compute the entropy rate $h = -\sum_{ij} \pi_i P_{ij}\log P_{ij}$.

**Exercise 23.2.** Prove the SMB theorem for i.i.d. processes from the classical LLN. Show that for i.i.d. Bernoulli($p$) process, the typical set has $\approx 2^{n h_b(p)}$ elements where $h_b(p) = -p\log p - (1-p)\log(1-p)$.

**Exercise 23.3.** (Collatz) The Collatz map generates a sequence of parities $y_n = T^n(m) \pmod 2$. If the Collatz map has an ergodic invariant measure $\mu$, what does SMB say about the parity sequences?

---

## Chapter Notes

The SMB theorem was proved by Shannon (1948) in the i.i.d. case, McMillan (1953) in $L^1$, and Breiman (1957) for the pointwise a.e. convergence. The proof using the Birkhoff theorem is in Walters' *Introduction to Ergodic Theory* (Chapter 8).

For universal source coding via Lempel-Ziv: the original papers are Ziv-Lempel (1977, 1978). The proof of asymptotic optimality is by Wyner-Ziv (1994).

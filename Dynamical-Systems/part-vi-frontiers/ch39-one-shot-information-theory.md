# Chapter 39 — One-Shot Information Theory

> *Shannon's theory is asymptotic — rates achieved in the limit of infinitely many channel uses. One-shot information theory asks: what can be done with a single channel use? The answer is smooth entropy — a family of entropic quantities that reduce to Shannon entropy in the i.i.d. limit but capture finite-blocklength behavior.*

**Prerequisites:** Chapter 16 (Shannon entropy, channel coding), Chapter 17 (Rényi entropy, min-entropy), Chapter 21 (quantum information, von Neumann entropy).

---

## 39.1 The Need for One-Shot Theory

**The Problem with Asymptotic Theory:** Shannon's capacity theorem says: over $n$ uses of a channel, the maximum rate of reliable communication is $\approx nC$ bits (for large $n$). But:
- Networks have finite latency budgets — you cannot send $n \to \infty$ packets
- Cryptography requires security with *one* application of a protocol
- Physical systems have finite resources — block length matters

**Definition 39.1.1.** A *one-shot* result gives bounds for a *single* use of a resource (channel, source, protocol), without asymptotics.

---

## 39.2 Smooth Min- and Max-Entropy

### 39.2.1 Min-Entropy and Max-Entropy

**Definition 39.2.1.** For a probability distribution $P$ on $\mathcal{X}$:
- *Min-entropy*: $H_\infty(X) = -\log \max_x P(x)$ (measures the "worst-case" randomness)
- *Max-entropy*: $H_0(X) = \log |\text{supp}(P)|$ (measures the "support size")

For quantum states $\rho$:
- $H_{\min}(\rho) = -\log \lambda_{\max}(\rho)$ (negative log of largest eigenvalue)
- $H_{\max}(\rho) = \log \text{rank}(\rho)$ (log of rank)

**Remark 39.2.2.** For i.i.d. sources $X^n = (X_1, \ldots, X_n)$: by LLN, $H_\infty(X^n) \approx nH(X)$ for large $n$. The asymptotic $H_\infty/n \to H(X)$ recovers Shannon entropy.

### 39.2.2 Smooth Entropy

**Definition 39.2.3 (Renner, 2005).** The *$\varepsilon$-smooth min-entropy* of $\rho$ is:
$$H_{\min}^\varepsilon(\rho) = \max_{\tilde\rho: \|\tilde\rho - \rho\|_1 \leq \varepsilon} H_{\min}(\tilde\rho).$$

The *$\varepsilon$-smooth max-entropy* is:
$$H_{\max}^\varepsilon(\rho) = \min_{\tilde\rho: \|\tilde\rho - \rho\|_1 \leq \varepsilon} H_{\max}(\tilde\rho).$$

The smoothing allows us to "cheat" on an $\varepsilon$-fraction of probability mass.

**Theorem 39.2.4 (AEP for Smooth Entropy).** For i.i.d. sources $\rho^{\otimes n}$ as $n \to \infty$:
$$\frac{1}{n}H_{\min}^{\varepsilon}(\rho^{\otimes n}) \to S(\rho), \quad \frac{1}{n}H_{\max}^{\varepsilon}(\rho^{\otimes n}) \to S(\rho)$$
for any fixed $\varepsilon > 0$, where $S(\rho)$ is the von Neumann entropy. The smooth entropies collapse to $S(\rho)$ in the i.i.d. limit.

---

## 39.3 One-Shot Source Coding

**Theorem 39.3.1 (One-Shot Source Coding).** Given a classical source $X$ with distribution $P$, a code of length $\ell$ bits achieves error $\varepsilon$ iff:
$$\ell \geq H_{\min}^\varepsilon(X).$$

The optimal code length (for given error tolerance $\varepsilon$) is exactly the smooth min-entropy.

**Quantum Version (Schumacher in one shot):**

**Theorem 39.3.2 (One-Shot Quantum Source Coding).** For a quantum source $\rho$, the optimal number of qubits $q$ for compression with fidelity $\geq 1 - \varepsilon$ satisfies:
$$q \approx H_{\min}^\varepsilon(\rho).$$

**Second-Order Asymptotics:** For i.i.d. $\rho^{\otimes n}$ with error $\varepsilon$:
$$q^* = nS(\rho) + \sqrt{n \cdot V(\rho)} \cdot \Phi^{-1}(\varepsilon) + O(\log n),$$
where $V(\rho) = \text{Var}(-\log\rho)$ is the *entropy variance* and $\Phi^{-1}$ is the inverse Gaussian CDF. The $\sqrt{n}$ correction is the "dispersion" term.

---

## 39.4 One-Shot Channel Coding

**Theorem 39.4.1 (One-Shot Channel Coding — Polyanskiy-Poor-Verdú, 2010).** For a single use of a channel $W: \mathcal{X} \to \mathcal{Y}$, the maximum number of bits $M$ transmittable with error $\varepsilon$ satisfies:
$$\log M \approx H_{\min}^\varepsilon(Y | X),$$
where the approximation is in terms of the hypothesis testing relative entropy:
$$D_H^\varepsilon(P_{XY} \| P_X \otimes P_Y) = -\log \beta_\varepsilon(P_{XY}, P_X \otimes P_Y).$$

Here $\beta_\varepsilon$ is the minimum type-II error probability when type-I error $\leq \varepsilon$.

**Theorem 39.4.2 (Second-Order Channel Coding).** For $n$ i.i.d. uses of channel $W$ with capacity $C$ and dispersion $V$:
$$\log M^*(n, \varepsilon) = nC - \sqrt{nV}\Phi^{-1}(\varepsilon) + O(\log n).$$

The $-\sqrt{nV}\Phi^{-1}(\varepsilon)$ term is the *backoff from capacity* due to finite blocklength.

---

## 39.5 One-Shot Cryptography

### 39.5.1 Randomness Extraction

**Theorem 39.5.1 (Leftover Hash Lemma — Bennett-Brassard-Crépeau-Maurer, 1995).** If $X$ is a classical random variable with $H_{\min}(X) \geq k$ and $f: \mathcal{X} \to \{0,1\}^\ell$ is a $2$-universal hash function with $\ell \leq k - 2\log(1/\varepsilon)$, then $f(X)$ is $\varepsilon$-close to uniform:
$$\|P_{f(X)} - U_\ell\|_1 \leq \varepsilon.$$

**Quantum Version:**

**Theorem 39.5.2 (Quantum Leftover Hash Lemma — Renner, 2005).** For a quantum side information state $\rho_{XE}$ (where $E$ is the adversary's quantum system), a $2$-universal hash function $f$ extracts $\ell \leq H_{\min}^\varepsilon(X|E)_\rho$ bits that are uniform and independent of $E$.

### 39.5.2 Privacy Amplification

**Definition 39.5.3.** In key agreement: Alice and Bob share a weakly random key $X$ (with partial adversary knowledge $E$). *Privacy amplification* extracts a shorter, fully secret key from $X$.

**Theorem 39.5.4 (Privacy Amplification in One Shot).** The maximum length of a secret key extractable from $\rho_{XE}$ using a public random function is:
$$\ell^* = H_{\min}^\varepsilon(X|E)_\rho - 2\log(1/\delta),$$
with security parameter $\delta$ (probability of failure).

---

## 39.6 The Second Law in Quantum Thermodynamics

**Theorem 39.6.1 (Quantum Second Law — Dahlsten-Renner-Rieper-Vedral, 2011).** A work extraction protocol from a quantum system $\rho$ at temperature $T$ can extract at most:
$$W^* = kT \cdot H_{\min}(\rho)$$
work in a *single shot*, compared to the average $kT \cdot S(\rho)$ from asymptotic i.i.d. processing.

**Corollary 39.6.2.** In the single-shot regime, the work fluctuations are significant. The second law has the form:
$$\langle W \rangle \leq kT \cdot S(\rho) \quad (\text{average})$$
but the one-shot bound $W \leq kT \cdot H_{\min}(\rho)$ holds with high probability (not just on average).

**Connection to Dynamical Systems:** A dynamical system with entropy $h$ generates $h$ bits/time, which can be extracted as work at rate $k_BT \cdot h$. One-shot thermodynamics corrects for finite-time effects: the *actual* work extracted in a finite window $T$ is $\approx k_BT \cdot H_{\min}^\varepsilon(\text{orbit}_T)$.

---

## Exercises

**Exercise 39.1.** Compute $H_{\min}(X)$ and $H_{\max}(X)$ for $X$ with distribution $P(1) = 1/2$, $P(2) = 1/4$, $P(3) = 1/8$, $P(4) = 1/8$. How do they compare to $H(X)$?

**Exercise 39.2.** (Smooth Entropy) For $X$ with $P(1) = 1/2$, $P(2) = 1/2 - \varepsilon$, $P(3) = \varepsilon$: compute $H_{\min}^\varepsilon(X)$ and show it is approximately $1$ (one bit) for small $\varepsilon$.

**Exercise 39.3.** (Second-Order Coding) For the binary symmetric channel with crossover probability $p = 0.1$:
- Compute capacity $C = 1 - h_b(p)$
- Compute dispersion $V = p(1-p)(\log\frac{1-p}{p})^2$
- For blocklength $n = 100$ and error $\varepsilon = 0.01$, find the second-order approximation to the maximum rate

**Exercise 39.4.** Verify the Leftover Hash Lemma for the following scenario: $X$ is uniform on $\{0,1\}^k$ (so $H_{\min}(X) = k$). A $2$-universal hash function maps $X$ to $\{0,1\}^\ell$ with $\ell = k - 2$. Show the output is $1/2$-close to uniform (by direct computation for small $k$).

---

## Chapter Notes

Renner's smooth entropy framework: *Security of Quantum Key Distribution* (PhD thesis, ETH Zürich, 2005). The framework is surveyed in Tomamichel's *Quantum Information Processing with Finite Resources* (Springer, 2016).

Second-order channel coding: Polyanskiy-Poor-Verdú's *Channel coding rate in the finite blocklength regime* (IEEE Trans. Inf. Theory, 2010). The dispersion concept is developed in Hayashi's *Information Spectrum Approach to Second-Order Coding Rate in Channel Coding* (2009).

Quantum thermodynamics: Dahlsten-Renner-Rieper-Vedral's *Inadequacy of von Neumann entropy for characterizing extractable work* (New J. Physics, 2011). The resource theory of thermodynamics is surveyed in Åberg's *Truly work-like work extraction via a single-shot analysis* (Nature Commun., 2013).

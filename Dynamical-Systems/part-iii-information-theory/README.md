# Part III — Information Theory

> *"The fundamental problem of communication is that of reproducing at one point either exactly or approximately a message selected at another point."*
> — Claude Shannon, *A Mathematical Theory of Communication* (1948)

---

## Overview

Information theory is the second great pillar of this curriculum. It provides a mathematical framework for quantifying information, measuring uncertainty, and analyzing the limits of communication, compression, and computation. But to treat information theory as merely a toolkit for communications engineers — a set of formulas about bits and bandwidth — is to miss the point entirely. It is a mathematical theory of *epistemology*: of what can be known, transmitted, and inferred, and at what cost.

Shannon's founding insight (1948) was to separate the *meaning* of a message from its *statistical structure*. A message is modeled as the output of a random source, and information is defined not by what the message means but by how *surprising* it is. Rare events carry more information than probable ones: if you already knew something would happen, learning that it happened tells you nothing. This leads to Shannon entropy $H(X) = -\sum_x p(x) \log p(x)$, which measures the average uncertainty in a random variable $X$ — or equivalently, the minimum average number of bits needed to communicate its value.

The subject has three main branches, corresponding to the three main theorems of Shannon's 1948 paper: **source coding** (compression), **channel coding** (reliable communication), and their generalization **rate-distortion theory** (lossy compression). All three are asymptotic theories: they describe what is achievable in the limit of long messages and many channel uses. The finite-length theory — how quickly you approach the asymptotic limits — is part of the research frontier (Chapter 39).

Part III is structured to develop these branches in order of generality and depth, from classical Shannon theory (Chapter 16) through the zoo of entropy generalizations (Chapter 17), algorithmic information theory (Chapter 18), network information theory (Chapter 19), information geometry (Chapter 20), and quantum information theory (Chapter 21). Each chapter enriches the picture from classical Shannon theory: generalizations replace probabilities with frequencies or program lengths; networks generalize the single-sender/receiver model; geometry equips the space of probability distributions with metric structure; quantum mechanics requires the full apparatus of density matrices and operator algebras.

---

## Prerequisites

The reader should have completed the measure theory chapter (Chapter 2) and should be comfortable with probability theory (random variables, expectations, conditional distributions). For Chapter 18 (Algorithmic Information Theory), some exposure to computability theory is helpful but is reviewed within the chapter. For Chapter 21 (Quantum Information Theory), the linear algebra chapter (Chapter 5) is essential, particularly the spectral theorem and the theory of positive operators.

**What you gain from this part:**
- Complete mastery of Shannon's theory: entropy, mutual information, AEP, source coding, channel capacity, rate-distortion.
- Understanding of the zoo of entropy measures: Rényi, min-entropy, differential entropy, von Neumann.
- Algorithmic information theory: Kolmogorov complexity as program-length information, Martin-Löf randomness.
- Network information theory: MAC, broadcast, relay, Slepian-Wolf, Wyner-Ziv.
- Information geometry: the Fisher metric, Amari's $\alpha$-connections, and the geometry of exponential families.
- Quantum information theory: density matrices, quantum channels, von Neumann entropy, Holevo bound.

---

## Chapter Descriptions

### Chapter 16 — Classical Information Theory

This is the core chapter, covering Shannon's theory completely: from the definition of entropy to the proof of the channel coding theorem. The development is rigorous and complete, with all major theorems proved.

Entropy is defined operationally through the source coding theorem: $H(X)$ bits per symbol is both the minimum average description length and the answer to "how many fair coin flips does $X$ require?" This operational definition — entropy as the minimum description length — connects Shannon's theory to Kolmogorov complexity (Chapter 18) and underlies the asymptotic equipartition property (AEP).

The AEP is the fundamental theorem connecting probability to information: for i.i.d. samples $X_1, X_2, \ldots, X_n$ from distribution $p$, the log-probability $-(1/n) \log p(X_1, \ldots, X_n) \to H(X)$ almost surely. This concentrates the distribution on a "typical set" of roughly $2^{nH(X)}$ sequences, each with approximately equal probability $2^{-nH(X)}$. Source coding uses this: encode only typical sequences, using $nH(X)$ bits, and send atypical sequences with negligible probability of occurrence.

Channel capacity is defined as $C = \max_{p(X)} I(X; Y)$, the maximum mutual information over all input distributions. Shannon's noisy channel coding theorem says that for rates $R < C$, there exist codes with block error probability $\to 0$; for rates $R > C$, error probability is bounded away from 0. The achievability proof uses random coding: draw the codebook at random, and the law of large numbers guarantees the typical set argument works. The converse uses Fano's inequality: the error probability gives a lower bound on the equivocation $H(M | \hat{M})$, and the chain rule of mutual information gives an upper bound on the rate.

**Key formulas and results:** $H(X) = -\sum p \log p$; $I(X;Y) = H(X) - H(X|Y)$; AEP; source coding at entropy rate; channel capacity $C = \max I(X;Y)$; rate-distortion $R(D) = \min_{p(\hat{x}|x): \mathbb{E}[d] \leq D} I(X;\hat{X})$.

### Chapter 17 — Entropy and Its Generalizations

Shannon entropy is not the only reasonable way to quantify uncertainty. Chapter 17 surveys the most important generalizations and explains when each is the appropriate tool.

*Rényi entropy* $H_\alpha(X) = \frac{1}{1-\alpha} \log \sum_x p(x)^\alpha$ is a one-parameter family: $H_0$ is the log-cardinality of the support; $H_1 = \lim_{\alpha \to 1} H_\alpha = H_{\text{Shannon}}$; $H_\infty = -\log \max_x p(x)$ is the min-entropy. Each order $\alpha$ has an operational meaning: $H_2$ governs collision probability (birthday problem); $H_\infty$ governs the extractable randomness in one-shot cryptography. The data-processing inequality holds for all $\alpha \geq 0$.

*Min-entropy* and *smooth min-entropy* are the right quantities for one-shot (non-asymptotic) information theory, developed by Renner in his 2005 thesis. For a single use of a source or channel, the maximum amount of randomness that can be extracted or the minimum description length is given by smooth min-entropy, not Shannon entropy. This connects to the research frontier of one-shot information theory (Chapter 39).

*Von Neumann entropy* $S(\rho) = -\text{Tr}(\rho \log \rho)$ is the quantum analogue of Shannon entropy, defined for density matrices $\rho$ on a Hilbert space. Strong subadditivity — $S(AB) + S(BC) \geq S(B) + S(ABC)$ — is the deepest inequality in quantum information theory (Lieb-Ruskai, 1973) and has connections to operator algebras, modular theory, and quantum thermodynamics.

*Maximum entropy principle*: among all distributions $p$ satisfying constraints $\mathbb{E}[f_i(X)] = a_i$, the maximum-entropy distribution is the *exponential family* $p(x) = Z^{-1} e^{\sum \lambda_i f_i(x)}$. This is Jaynes' formulation: entropy maximization subject to constraints is the principle of least assumptions, choosing the distribution that is consistent with known facts and otherwise maximally uncertain.

### Chapter 18 — Algorithmic Information Theory

Algorithmic information theory (AIT) defines information without probability, using computability theory. The Kolmogorov complexity $K(x)$ of a string $x$ is the length of the shortest program that outputs $x$ on a universal Turing machine. This is an individual definition of information: $K(x)$ measures how much structure $x$ contains, without reference to a distribution.

The invariance theorem ensures $K$ is well-defined up to an additive constant: if $U$ and $U'$ are two universal Turing machines, then $|K_U(x) - K_{U'}(x)| \leq c$ for a constant $c$ depending only on $U$ and $U'$. Most strings of length $n$ have $K(x) \approx n$ — they are incompressible, or *Kolmogorov random*.

The connection to Shannon entropy is given by: for a computable distribution $p$, $K(X) \approx -\log p(X)$ on average, and the entropy $H(X) = \mathbb{E}[-\log p(X)] \approx \mathbb{E}[K(X)]$. But individual strings can be far from this average. The Kolmogorov complexity of an individual sequence $x_1 x_2 \ldots x_n$ captures whether that sequence is random or structured, regardless of any assumed distribution.

*Martin-Löf randomness* is the rigorous definition of a "random infinite sequence": an infinite binary sequence $\omega \in \{0,1\}^{\mathbb{N}}$ is ML-random if it passes all computably enumerable statistical tests. Equivalently (Levin-Schnorr): $\omega$ is ML-random iff $K(\omega \upharpoonright n) \geq n - O(1)$ (the prefixes are incompressible). The halting probability $\Omega = \sum_{p \text{ halts}} 2^{-|p|}$ is the canonical ML-random real: it is computably enumerable but not computable, and its digits encode the halting problem.

### Chapter 19 — Network Information Theory

Network information theory extends Shannon's point-to-point theory to networks of communicating nodes. The capacity regions — the achievable rate tuples — are much more complex than single numbers, and the complete characterization is a major open problem for most networks.

The *multiple access channel (MAC)* has multiple senders and one receiver: the capacity region is a polyhedron characterized by the inequalities $R_i \leq I(X_i; Y | X_{\sim i})$ and the sumrate bound $\sum R_i \leq I(X_1, \ldots, X_k; Y)$. The *broadcast channel* has one sender and multiple receivers: the capacity region is known only for degraded broadcast channels (Bergmans-Cover) and not in general.

*Slepian-Wolf theorem* is the most elegant result in network information theory: two correlated sources $X$ and $Y$ being compressed by separate encoders (who cannot communicate) can be compressed jointly at sum rate $H(X,Y)$, the same as if the encoders cooperated. The individual rates must satisfy $R_X \geq H(X|Y)$ and $R_Y \geq H(Y|X)$, but the sum constraint $R_X + R_Y \geq H(X,Y)$ is achievable. This is surprising: the encoders don't need to know the correlation structure — the decoder can resolve the ambiguity.

*Wyner-Ziv theorem* extends this to lossy coding: if the decoder has side information $Y$ correlated with the source $X$, the rate needed to represent $X$ to distortion $D$ is $R(D) = \min_{p(\hat{x}|x,y): \mathbb{E}[d(X,\hat{X})] \leq D} I(X;\hat{X}|Y)$ — independent of the fact that the encoder does not have access to $Y$. This "coding theorem with side information at the decoder" is foundational for distributed sensing and has connections to the ergodic theory of processes.

### Chapter 20 — Information Geometry

Information geometry treats the space of probability distributions as a Riemannian manifold, with the *Fisher information matrix* as the metric:
$$g_{ij}(\theta) = \mathbb{E}_\theta\left[\frac{\partial \log p(x|\theta)}{\partial \theta_i} \cdot \frac{\partial \log p(x|\theta)}{\partial \theta_j}\right] = I(\theta)_{ij}.$$

The Cramér-Rao bound $\text{Var}[\hat\theta] \geq I(\theta)^{-1}$ gives a lower bound on the variance of any unbiased estimator, in terms of the Fisher metric. This is the fundamental result connecting information geometry to statistics.

Amari's $\alpha$-connections are a one-parameter family of connections on the statistical manifold: the $+1$ connection ($e$-connection) is adapted to exponential families, the $-1$ connection ($m$-connection) is adapted to mixture families, and the $0$ connection is the Levi-Civita connection of the Fisher metric. The duality between $+1$ and $-1$ connections gives the statistical manifold a *dually flat* structure, and the KL divergence is the "divergence function" for this dual structure — the analogue of squared distance for the $m$-connection.

The maximum likelihood estimator achieves the Cramér-Rao bound asymptotically: this is the information-geometric interpretation of efficiency. The EM algorithm, in information-geometric terms, is an alternating projection algorithm: the E-step projects onto the $m$-flat manifold of distributions consistent with the observed sufficient statistics; the M-step projects onto the $e$-flat manifold of model distributions.

### Chapter 21 — Quantum Information Theory

Quantum information theory extends classical information theory to quantum systems. The central object is the *density matrix* $\rho$ — a positive semidefinite trace-1 operator on a Hilbert space $\mathcal{H}$ — which generalizes the probability distribution. Pure states $|\psi\rangle$ correspond to rank-1 projectors $\rho = |\psi\rangle\langle\psi|$; mixed states $\rho = \sum_i p_i |\psi_i\rangle\langle\psi_i|$ are statistical mixtures.

Quantum channels (completely positive trace-preserving maps) are the quantum generalization of classical channels. The Kraus representation $\mathcal{E}(\rho) = \sum_k K_k \rho K_k^\dagger$ (with $\sum_k K_k^\dagger K_k = I$) gives every quantum channel. The *Holevo bound* $\chi(\{p_i, \rho_i\}) = S(\sum_i p_i \rho_i) - \sum_i p_i S(\rho_i)$ is an upper bound on the classical capacity of a quantum channel with ensemble input $\{p_i, \rho_i\}$; the HSW theorem says this bound is achievable.

Entanglement — the distinctly quantum resource with no classical analogue — is quantified by the *entanglement entropy* $S(\rho_A)$ for a pure state $|\psi\rangle_{AB}$ (where $\rho_A = \text{Tr}_B |\psi\rangle\langle\psi|$). Entanglement distillation (converting many imperfect pairs to fewer perfect pairs) and dilution (the reverse) are resource theories whose rates are determined by the entanglement entropy.

Strong subadditivity of von Neumann entropy — $S(AB) + S(BC) \geq S(B) + S(ABC)$ — is the key inequality. It implies the monotonicity of quantum relative entropy under quantum operations (the data processing inequality for $D(\rho\|\sigma) = \text{Tr}[\rho(\log\rho - \log\sigma)]$), which is one of the most useful tools in quantum information theory.

---

## Key Mathematical Concepts

### Entropy and Mutual Information

For a discrete random variable $X$ with distribution $p(x)$:
$$H(X) = -\sum_x p(x) \log p(x) \quad (\text{Shannon entropy})$$
$$H(X|Y) = -\sum_{x,y} p(x,y) \log p(x|y) \quad (\text{conditional entropy})$$
$$I(X;Y) = H(X) - H(X|Y) = H(X) + H(Y) - H(X,Y) \quad (\text{mutual information})$$
$$D(p \| q) = \sum_x p(x) \log \frac{p(x)}{q(x)} \quad (\text{KL divergence / relative entropy})$$

Key properties: $H(X) \geq 0$; $I(X;Y) \geq 0$ (data processing inequality); $H(X|Y) \leq H(X)$ (conditioning reduces entropy); chain rule $H(X_1, \ldots, X_n) = \sum_i H(X_i | X_1, \ldots, X_{i-1})$.

### Typical Sets and AEP

For i.i.d. source $X^n = (X_1, \ldots, X_n)$: the typical set $\mathcal{T}_\epsilon^n = \{x^n : |-\frac{1}{n}\log p(x^n) - H(X)| \leq \epsilon\}$ has the properties:
- $P(X^n \in \mathcal{T}_\epsilon^n) \to 1$
- $|\mathcal{T}_\epsilon^n| \leq 2^{n(H+\epsilon)}$
- $|\mathcal{T}_\epsilon^n| \geq (1-\delta) 2^{n(H-\epsilon)}$ for large enough $n$

This is the AEP, and it is the key to all of Shannon's coding theorems: code typical sequences efficiently, discard atypical ones.

### Channel Capacity

The *capacity* of a discrete memoryless channel $p(y|x)$ is:
$$C = \max_{p(X)} I(X;Y) = \max_{p(X)} \sum_{x,y} p(x) p(y|x) \log \frac{p(y|x)}{p(y)}.$$
For the binary symmetric channel with crossover probability $p$: $C = 1 - h(p)$ where $h(p) = -p\log p - (1-p)\log(1-p)$ is the binary entropy function.

---

## Key Theorems

1. **Shannon's Source Coding Theorem.** For i.i.d. source with entropy $H$: there exist source codes with rate $R = H + \epsilon$ and error probability $\to 0$; for any code with rate $R < H$, the error probability is bounded away from $0$.

2. **Shannon's Channel Coding Theorem.** For a discrete memoryless channel with capacity $C$: for any rate $R < C$, there exist codes with block error probability $\to 0$; for any rate $R > C$, the error probability is bounded away from $0$.

3. **Slepian-Wolf Theorem.** For correlated memoryless sources $(X, Y)$ with joint distribution $p(x,y)$: the achievable rate region for separate lossless compression is $\{(R_X, R_Y) : R_X \geq H(X|Y),\ R_Y \geq H(Y|X),\ R_X + R_Y \geq H(X,Y)\}$.

4. **Holevo's Theorem.** The classical capacity of a quantum channel $\mathcal{E}$ is at most $\max_{\{p_i, \rho_i\}} \chi(\{p_i, \mathcal{E}(\rho_i)\})$ bits per channel use. The bound is achievable by the HSW theorem.

5. **Cramér-Rao Bound.** For any unbiased estimator $\hat\theta$ of $\theta$: $\text{Cov}[\hat\theta] \geq I(\theta)^{-1}$ (in the sense that $\text{Cov}[\hat\theta] - I(\theta)^{-1}$ is positive semidefinite). Equality is achieved by efficient estimators (exponential families with natural parameter estimator).

6. **Stein's Lemma.** For the problem of testing $H_0: X \sim p$ vs. $H_1: X \sim q$ with i.i.d. samples: the type-II error exponent (at fixed type-I error) is the KL divergence $D(p \| q)$.

---

## Connections to Other Parts

Part III connects to the rest of the book in multiple directions:

- **Part II (Dynamical Systems)** provides the dynamical systems that generate the random processes studied in information theory. The shift on a stationary measure-preserving system is the canonical model for a stationary source. The ergodic theorem (Chapter 7) is the AEP for dynamical systems (Shannon-McMillan-Breiman theorem). The KS entropy of a system equals the entropy rate of its symbolic coding.

- **Part IV (Bridges)** makes the connections between dynamical entropy, symbolic dynamics, and information theory explicit and systematic. Chapter 22 (Entropy in Dynamical Systems) is the core bridge; Chapter 23 (Ergodic Information Theory) shows how the Shannon-McMillan-Breiman theorem follows from the ergodic theorem; Chapter 24 provides the precise dictionary between symbolic dynamics and information theory.

- **Part V (Foundations)** connects AIT (Chapter 18) to the computability theory of dynamical systems (Chapter 27). Kolmogorov complexity appears in the context of orbit complexity (Chapter 25).

- **Part VI (Frontiers)** builds on the one-shot information theory of Chapter 17 (min-entropy, smooth entropies) for the frontier of finite-length information theory (Chapter 39). Quantum information complexity (Chapter 38) extends the classical communication complexity of Chapter 26.

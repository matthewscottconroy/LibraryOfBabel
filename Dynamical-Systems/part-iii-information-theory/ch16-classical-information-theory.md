# Chapter 16 — Classical Information Theory

> *Shannon's 1948 paper is the most important engineering paper of the 20th century. But it is also a paper in pure mathematics — about the fundamental limits of communication, compression, and computation with probabilistic sources.*

**Prerequisites:** Chapter 2 (probability theory, measure theory).

**What this chapter builds:** Shannon entropy and its operational meaning; the asymptotic equipartition property (AEP); source coding (data compression to the entropy limit); the noisy channel coding theorem (reliable communication up to channel capacity); rate-distortion theory; and Fano's inequality.

---

## 16.1 Information Measures

### 16.1.1 Shannon Entropy

**Definition 16.1.1.** The *Shannon entropy* of a discrete random variable $X$ with probability mass function $p(x) = P(X = x)$ is:
$$H(X) = -\sum_{x \in \mathcal{X}} p(x) \log p(x) = \mathbb{E}[-\log p(X)].$$

(Convention: $0 \log 0 = 0$. The logarithm base determines units: base 2 gives *bits*, base $e$ gives *nats*, base 10 gives *bans*.)

**Theorem 16.1.2 (Basic Properties of Entropy).**
1. *Nonnegativity*: $H(X) \geq 0$, with equality iff $X$ is deterministic ($p(x_0) = 1$ for some $x_0$).
2. *Maximum*: $H(X) \leq \log |\mathcal{X}|$, with equality iff $X$ is uniform.
3. *Continuity*: $H$ is continuous in $p$.
4. *Expansibility*: $H(X, 0) = H(X)$ (adding an impossible event doesn't change entropy).
5. *Chain rule*: $H(X_1, \ldots, X_n) = \sum_{k=1}^n H(X_k | X_1, \ldots, X_{k-1})$.

*Proof of (2):* By Jensen's inequality applied to $-\log$: $H(X) = E[-\log p(X)] \leq -\log E[p(X)] = -\log(\sum_x p(x)^2)$... Actually: use the log-sum inequality or convexity of $x\log x$.

**Theorem 16.1.3 (Axiomatic Characterization — Shannon/Khinchin).** The only function satisfying continuity, the chain rule, expansibility, and maximum at uniform distribution is $H = -c\sum p_i \log p_i$ for $c > 0$.

### 16.1.2 Joint and Conditional Entropy

**Definition 16.1.4.** For a joint random variable $(X, Y)$:
$$H(X, Y) = -\sum_{x,y} p(x,y)\log p(x,y)$$
$$H(X|Y) = \sum_y p(y) H(X|Y=y) = -\sum_{x,y} p(x,y)\log p(x|y)$$

**Chain Rule:** $H(X, Y) = H(X) + H(Y|X) = H(Y) + H(X|Y)$.

**Theorem 16.1.5 (Conditioning Reduces Entropy).** $H(X|Y) \leq H(X)$, with equality iff $X$ and $Y$ are independent.

### 16.1.3 Mutual Information

**Definition 16.1.6.** The *mutual information* between $X$ and $Y$ is:
$$I(X; Y) = H(X) + H(Y) - H(X, Y) = H(X) - H(X|Y) = H(Y) - H(Y|X).$$

Equivalently: $I(X;Y) = \sum_{x,y} p(x,y) \log \frac{p(x,y)}{p(x)p(y)} = D_{\text{KL}}(p_{XY} \| p_X \otimes p_Y)$.

**Properties:**
1. $I(X;Y) \geq 0$, with equality iff $X \perp Y$.
2. $I(X;Y) = I(Y;X)$ (symmetric).
3. $I(X;Y) = H(X) - H(X|Y)$ (reduction in uncertainty about $X$ after observing $Y$).
4. Chain rule: $I(X_1,\ldots,X_n; Y) = \sum_k I(X_k; Y | X_1,\ldots,X_{k-1})$.

### 16.1.4 KL Divergence

**Definition 16.1.7.** The *Kullback-Leibler divergence* (relative entropy) between distributions $P$ and $Q$ is:
$$D_{\text{KL}}(P \| Q) = \sum_x p(x) \log \frac{p(x)}{q(x)} = E_P\left[\log\frac{p(X)}{q(X)}\right].$$

**Theorem 16.1.8 (Gibbs' Inequality / Nonnegativity).** $D_{\text{KL}}(P \| Q) \geq 0$, with equality iff $P = Q$.

*Proof:* $D_{\text{KL}}(P\|Q) = E_P[-\log(q(X)/p(X))] \geq -\log E_P[q(X)/p(X)] = -\log(\sum q(x)) = 0$ by Jensen's inequality (since $-\log$ is convex).

**Warning:** $D_{\text{KL}}$ is not a metric: it is asymmetric ($D_{\text{KL}}(P\|Q) \neq D_{\text{KL}}(Q\|P)$) and does not satisfy the triangle inequality.

---

## 16.2 The Asymptotic Equipartition Property

### 16.2.1 Typical Sequences

**Theorem 16.2.1 (Weak AEP — Shannon 1948).** Let $X_1, X_2, \ldots$ be i.i.d. with distribution $p(x)$ and entropy $H = H(X_1)$. Then:
$$-\frac{1}{n}\log p(X_1, \ldots, X_n) \to H \quad \text{in probability.}$$

*Proof:* By the Weak Law of Large Numbers: $-\frac{1}{n}\log p(X_1, \ldots, X_n) = \frac{1}{n}\sum_{k=1}^n (-\log p(X_k)) \to E[-\log p(X)] = H$.

**Definition 16.2.2.** The *$\varepsilon$-typical set* is:
$$A_\varepsilon^{(n)} = \left\{(x_1,\ldots,x_n) \in \mathcal{X}^n : \left|-\frac{1}{n}\log p(x_1,\ldots,x_n) - H\right| < \varepsilon\right\}.$$

**Theorem 16.2.3 (Properties of Typical Set).**
1. $P((X_1,\ldots,X_n) \in A_\varepsilon^{(n)}) \geq 1 - \delta$ for $n$ large enough (depending on $\varepsilon, \delta$).
2. $|A_\varepsilon^{(n)}| \leq 2^{n(H+\varepsilon)}$ (at most exponentially many typical sequences).
3. $|A_\varepsilon^{(n)}| \geq (1-\delta)2^{n(H-\varepsilon)}$ for large $n$ (at least).

**Interpretation:** Of the $|\mathcal{X}|^n$ possible sequences, the typical set has $\approx 2^{nH}$ members. The non-typical sequences are collectively unlikely. This is the *data compression principle*: we only need to encode typical sequences, which requires $nH$ bits — exactly the entropy.

---

## 16.3 Source Coding

### 16.3.1 Lossless Data Compression

**Definition 16.3.1.** A *code* for $X$ is a function $C: \mathcal{X} \to \{0,1\}^*$ (mapping outcomes to binary strings). The *expected length* is $L(C) = \sum_x p(x) |C(x)|$ where $|C(x)|$ is the length of the codeword for $x$.

**Definition 16.3.2.** A code is *prefix-free* if no codeword is a prefix of another. Prefix-free codes are uniquely decodable.

**Theorem 16.3.3 (Kraft Inequality).** Codeword lengths $\ell_1, \ldots, \ell_m$ correspond to a prefix-free code iff $\sum_i 2^{-\ell_i} \leq 1$.

**Theorem 16.3.4 (Shannon's Source Coding Theorem).** For a source $X$ with entropy $H(X)$:
$$H(X) \leq L(C^*) < H(X) + 1,$$
where $C^*$ is the optimal prefix-free code (Huffman code).

For $n$-tuples: the optimal code for $(X_1, \ldots, X_n)$ achieves $L/n \to H(X)$ as $n \to \infty$.

*Achievability:* Set $\ell_x = \lceil -\log p(x) \rceil$. Then $\sum 2^{-\ell_x} \leq \sum p(x) = 1$ (Kraft), so a prefix-free code exists. The length is $\ell_x \leq -\log p(x) + 1$, so $L \leq H + 1$.
*Converse:* Kraft + AM-GM gives $\sum p_x \ell_x \geq H(X)$.

### 16.3.2 Huffman Coding

The Huffman code achieves the optimal (minimum expected length) prefix-free code:
1. Build a priority queue of (symbol, probability) pairs.
2. Repeatedly combine the two lowest-probability nodes into a new node with their combined probability.
3. Assign 0/1 labels to the two branches at each combination.
4. Read off codewords by tracing paths from root to leaves.

**Theorem 16.3.5.** Huffman coding achieves the optimal average code length for prefix-free codes.

---

## 16.4 The Noisy Channel

### 16.4.1 Channel Models

**Definition 16.4.1.** A *discrete memoryless channel (DMC)* is specified by:
- Input alphabet $\mathcal{X}$, output alphabet $\mathcal{Y}$
- Transition probabilities $p(y|x) = P(Y = y | X = x)$

"Memoryless" means successive channel uses are independent: $p(y_1,\ldots,y_n | x_1,\ldots,x_n) = \prod_i p(y_i|x_i)$.

**Examples 16.4.2.**
- *Binary Symmetric Channel (BSC)*: $\mathcal{X} = \mathcal{Y} = \{0,1\}$, $p(0|0) = p(1|1) = 1-\epsilon$, $p(1|0) = p(0|1) = \epsilon$.
- *Binary Erasure Channel (BEC)*: outputs $X$ with probability $1-\epsilon$ and $\text{?}$ (erasure) with prob $\epsilon$.
- *Gaussian Channel*: $Y = X + Z$ where $Z \sim N(0, N)$.

### 16.4.2 Channel Capacity

**Definition 16.4.3.** The *capacity* of a DMC is:
$$C = \max_{p(x)} I(X; Y).$$

**Examples:**
- BSC with bit-flip probability $\epsilon$: $C = 1 - H(\epsilon, 1-\epsilon) = 1 - h_b(\epsilon)$ bits/use.
- BEC with erasure probability $\epsilon$: $C = 1 - \epsilon$ bits/use.
- AWGN channel with power $P$ and noise variance $N$: $C = \frac{1}{2}\log(1 + P/N)$ (Shannon's formula).

**Definition 16.4.4.** An $(n, M, \varepsilon)$-code for a channel consists of:
- An *encoder*: a mapping from $\{1, \ldots, M\}$ (messages) to $\mathcal{X}^n$ (channel inputs)
- A *decoder*: a mapping from $\mathcal{Y}^n$ to $\{1, \ldots, M\}$
with maximum probability of error $\max_m P(\text{error}|m\text{ sent}) \leq \varepsilon$.

A rate $R$ is *achievable* if for every $\varepsilon > 0$ and large $n$, there is an $(n, 2^{nR}, \varepsilon)$-code.

### 16.4.3 Shannon's Noisy Channel Coding Theorem

**Theorem 16.4.5 (Shannon 1948).** The capacity $C$ of a DMC is the supremum of achievable rates:
- *(Achievability)* For every $R < C$ and $\varepsilon > 0$, there exists an $(n, 2^{nR}, \varepsilon)$-code for large enough $n$.
- *(Converse)* For every $R > C$ and every $(n, 2^{nR}, \varepsilon)$-code, $\varepsilon \geq 1 - R/C - o(1)$.

*(Achievability proof — Random Coding).* Generate $M = 2^{nR}$ codewords $x^n(1), \ldots, x^n(M)$ i.i.d. from $p^*(x)$ (the capacity-achieving input distribution). To decode $y^n$: find the unique $m$ such that $(x^n(m), y^n)$ are jointly typical. For $R < C$, the probability of error $\to 0$.

*(Converse proof — Fano's Inequality).*

**Theorem 16.4.6 (Fano's Inequality).** Let $M$ be a random variable uniform on $\{1, \ldots, 2^{nR}\}$, transmitted over the channel, and $\hat{M}$ the decoded estimate. If $P_e = P(M \neq \hat{M})$:
$$H(M|\hat{M}) \leq H(P_e) + P_e \cdot nR.$$
This limits how much information can be reliably decoded.

---

## 16.5 Rate-Distortion Theory

### 16.5.1 The Rate-Distortion Problem

**Setup:** Source $X$ with distribution $p(x)$; reconstruction alphabet $\hat{\mathcal{X}}$; distortion measure $d: \mathcal{X} \times \hat{\mathcal{X}} \to [0, \infty)$.

**Definition 16.5.1.** The *rate-distortion function* is:
$$R(D) = \min_{p(\hat{x}|x): E[d(X,\hat{X})] \leq D} I(X; \hat{X}).$$

$R(D)$ gives the minimum bits/sample needed to describe the source with average distortion $\leq D$.

**Theorem 16.5.2 (Rate-Distortion Theorem — Shannon).** The rate $R$ is achievable at distortion $D$ iff $R \geq R(D)$.

**Example 16.5.3 (Gaussian Source, MSE Distortion).** $X \sim N(0, \sigma^2)$, $d(x, \hat{x}) = (x - \hat{x})^2$:
$$R(D) = \frac{1}{2}\log\frac{\sigma^2}{D} \quad \text{for } 0 \leq D \leq \sigma^2.$$

This is the *water-filling formula*. Achieving rate $R$ allows reconstruction with distortion $D = \sigma^2 2^{-2R}$ (each additional bit halves the distortion).

---

## 16.6 Information Inequalities

**Theorem 16.6.1 (Data Processing Inequality).** If $X \to Y \to Z$ form a Markov chain, then $I(X; Z) \leq I(X; Y)$: processing cannot increase mutual information.

*Proof:* Markov chain $\Rightarrow$ $I(X;Y,Z) = I(X;Y) + I(X;Z|Y) = I(X;Y)$ (since $Z\perp X | Y$). Also $I(X;Y,Z) = I(X;Z) + I(X;Y|Z) \geq I(X;Z)$.

**Theorem 16.6.2 (Log-Sum Inequality).** For nonneg $a_1, \ldots, a_n$ and $b_1, \ldots, b_n$:
$$\sum_i a_i \log \frac{a_i}{b_i} \geq \left(\sum_i a_i\right)\log\frac{\sum_i a_i}{\sum_i b_i}.$$

This is the key tool for proving properties of entropy and KL divergence.

**Theorem 16.6.3 (Subadditivity of Entropy).** $H(X_1, \ldots, X_n) \leq \sum_i H(X_i)$, with equality iff $X_1, \ldots, X_n$ are mutually independent.

---

## Exercises

**Exercise 16.1.** Compute $H(X)$ for $X$ with $P(X = 0) = 1/2$, $P(X = 1) = 1/4$, $P(X = 2) = 1/4$. Build the Huffman code and verify $L(C^*) = H(X)$ in this case.

**Exercise 16.2.** Prove that for the BSC with flip probability $\epsilon$: $C = 1 - H(\epsilon)$. (*Hint:* $I(X;Y) = H(Y) - H(Y|X) = H(Y) - H(\epsilon)$; maximize $H(Y)$ over input distributions.)

**Exercise 16.3.** Prove Fano's inequality. (*Hint:* Let $E = \mathbf{1}[M \neq \hat{M}]$. Use the chain rule for entropy: $H(M|\hat{M}) = H(M,E|\hat{M}) = H(E|\hat{M}) + H(M|E,\hat{M})$. Bound each term.)

**Exercise 16.4.** (AEP) Let $X_1, X_2, \ldots$ be i.i.d. $\text{Bernoulli}(1/3)$. Describe the typical set $A_\varepsilon^{(n)}$: what sequences are typical? How many are there (approximately)?

**Exercise 16.5.** (Joint Typicality) State the Joint Typicality Lemma: if $(X^n, Y^n)$ are generated from $p(x,y)$, they are jointly typical; if $\tilde{X}^n$ is independent of $Y^n$ with marginal $p(x)$, the probability that $(\tilde{X}^n, Y^n)$ are jointly typical is $\approx 2^{-nI(X;Y)}$.

**Exercise 16.6.** Prove the rate-distortion lower bound $R(D) \geq \max_{p(\hat{x})} [H(\hat{X}) - h_b(D)] - \log|\hat{\mathcal{X}}|$ for the binary source with Hamming distortion.

**Exercise 16.7.** (Connections to Dynamics) For the doubling map $T: x \mapsto 2x \pmod 1$: interpret the $n$-bit binary expansion of $x$ as the result of a source coding scheme. Show the entropy rate of the source is $\log 2$ bits per symbol. How does this connect to $h_\mu(T) = \log 2$?

---

## Chapter Notes

Shannon's original paper — *A Mathematical Theory of Communication* (1948, *Bell System Technical Journal*) — is readable and beautiful. Every information theorist should read it. Cover and Thomas' *Elements of Information Theory* is the standard modern textbook.

For the historical context: Shannon was working on cryptography during WWII and generalized Nyquist-Hartley's earlier work on channel capacity. The key insight was the operational meaning of entropy as a compression limit — and that entropy is not just a formula but the answer to a precise question about limits.

Fano's inequality (Section 16.4.6) appears constantly: in proving channel capacity converses, in lower bounding the sample complexity of learning algorithms, and in the information-theoretic proofs of lower bounds in communication complexity (Chapter 26).

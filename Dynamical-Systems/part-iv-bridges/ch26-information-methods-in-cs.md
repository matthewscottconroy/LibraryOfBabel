# Chapter 26 — Information-Theoretic Methods in Computer Science

> *Entropy is not just a measure of uncertainty — it is a combinatorial weapon. Shearer's lemma gives submodular bounds; the entropy method proves the Loomis-Whitney inequality; communication complexity uses information to lower-bound computation. Information pervades the foundations of computer science.*

**Prerequisites:** Chapter 16 (Shannon entropy, mutual information), Chapter 18 (algorithmic information theory), Chapter 25 (chaos and computation).

---

## 26.1 The Entropy Method in Combinatorics

### 26.1.1 Shearer's Lemma

**Theorem 26.1.1 (Shearer's Lemma).** Let $X_1, \ldots, X_n$ be random variables and $\mathcal{F} \subseteq 2^{[n]}$ a collection of subsets of $[n]$ such that each $i \in [n]$ belongs to at least $k$ sets in $\mathcal{F}$. Then:
$$H(X_1, \ldots, X_n) \leq \frac{1}{k} \sum_{S \in \mathcal{F}} H(X_S),$$
where $X_S = (X_i)_{i \in S}$.

*(proof)* By submodularity of entropy: $H(X_1, \ldots, X_n) = \sum_i H(X_i | X_1, \ldots, X_{i-1})$. Weighted averaging over the sets in $\mathcal{F}$, each $i$ appears $\geq k$ times, giving the bound.

**Application 26.1.2 (Loomis-Whitney Inequality).** Let $A \subseteq {\mathbb Z}^d$ be a finite set. Let $A_{-i}$ denote the projection of $A$ onto the hyperplane perpendicular to the $i$-th coordinate. Then:
$$|A|^{d-1} \leq \prod_{i=1}^d |A_{-i}|.$$

*(proof)* Choose $X = (X_1, \ldots, X_d)$ uniformly from $A$. Then $H(X) = \log|A|$ and $H(X_{-i}) \leq \log|A_{-i}|$. Shearer's lemma with the collection $\{[d] \setminus \{i\}\}_{i=1}^d$ (each element appears $d-1$ times) gives $(d-1)\log|A| \leq \sum_i \log|A_{-i}|$.

### 26.1.2 The Entropy Method for Counting

**Example 26.1.3 (Counting Graphs).** The number of triangle-free graphs on $n$ vertices is at most $2^{n^2/4}$ (Kruskal-Katona, proved by entropy method).

*(proof sketch)* Let $G$ be a triangle-free graph. The Shannon entropy of the edge set $E(G)$ satisfies:
$$H(E(G)) \leq \frac{1}{3}\sum_{\text{triangles}} H(E(\triangle)) \leq \frac{\binom{n}{3}}{3} \cdot \log 8.$$

**Theorem 26.1.4 (Entropy Method for Graph Coloring).** For a $k$-colorable graph $G$ on $n$ vertices with $m$ edges:
$$m \leq \frac{n^2}{2}\left(1 - \frac{1}{k}\right).$$

This is Turán's theorem, proved via entropy.

---

## 26.2 Communication Complexity

### 26.2.1 The Model

**Definition 26.2.1.** The *communication complexity* of a function $f: \mathcal{X} \times \mathcal{Y} \to \mathcal{Z}$ is the minimum number of bits Alice (holding $x \in \mathcal{X}$) and Bob (holding $y \in \mathcal{Y}$) must exchange to compute $f(x,y)$, in the worst case over all $(x,y)$.

**Definition 26.2.2 (Communication Protocols).** A *deterministic protocol* is a binary tree where:
- Internal nodes are labeled by Alice or Bob (who speaks)
- Edges are labeled by bits
- Leaves are labeled by output values

The cost is the depth of the tree. The *deterministic communication complexity* $D(f)$ is the minimum cost over all protocols.

**Randomized complexity** $R(f)$: protocols that may use shared/private randomness, with error probability $\leq 1/3$.

**Quantum complexity** $Q(f)$: protocols where parties exchange qubits and share entanglement.

### 26.2.2 Lower Bounds via Information Theory

**Definition 26.2.3 (Information Complexity).** The *information complexity* of a protocol $\Pi$ with respect to distribution $\mu$ is:
$$IC_\mu(\Pi) = I(X; \Pi | Y) + I(Y; \Pi | X),$$
where $(X,Y) \sim \mu$ and $\Pi$ is the transcript. The *information complexity* of $f$ at $\mu$ is $IC_\mu(f) = \inf_{\Pi \text{ computes } f} IC_\mu(\Pi)$.

**Theorem 26.2.4 (Information Complexity Lower Bounds Communication).** For any distribution $\mu$:
$$D(f) \geq IC_\mu(f).$$

**Theorem 26.2.5 (Equality Lower Bound).** The communication complexity of the equality function $EQ_n(x,y) = [x = y]$ satisfies:
$$D(EQ_n) = n, \quad R(EQ_n) = O(\log n).$$

The randomized protocol: Alice sends a hash of $x$; this fails with probability $\leq 1/n$.

**Theorem 26.2.6 (Disjointness Lower Bound — Kalyanasundaram-Schnitger, Razborov).** The communication complexity of $DISJ_n(x,y) = [x \cap y = \emptyset]$ for $x,y \subseteq [n]$ satisfies:
$$R(DISJ_n) = \Omega(n).$$

*(proof sketch)* The information cost per coordinate is $\Omega(1)$ for any correct protocol. By a direct sum argument (each coordinate is independent under the hard distribution), the total cost is $\Omega(n)$.

### 26.2.3 Direct Sum Theorems

**Theorem 26.2.7 (Direct Sum for Information Complexity — Bar-Yossef et al.).** Computing $f$ on $k$ independent instances requires $k \cdot IC(f)$ bits of information:
$$IC_{\mu^k}(f^k) = k \cdot IC_\mu(f).$$

**Remark 26.2.8.** Direct sum theorems are the key to proving communication complexity lower bounds for composed functions. The direct sum holds for information complexity but not always for communication complexity (an important distinction).

---

## 26.3 Information Complexity and Circuit Lower Bounds

### 26.3.1 Entropy and Circuit Complexity

**Theorem 26.3.1 (Shannon, 1949).** A random Boolean function $f: \{0,1\}^n \to \{0,1\}$ requires circuit size $\Omega(2^n / n)$. In particular, most functions require exponential circuit size.

*(proof)* Count: there are $2^{2^n}$ Boolean functions on $n$ bits. But circuits of size $s$ have at most $s^{2s}$ distinct functions. For $s = c \cdot 2^n/n$, this count is much smaller than $2^{2^n}$, so most functions cannot be computed by size-$s$ circuits.

**Information-Theoretic Interpretation:** The circuit size needed to compute $f$ is related to the Kolmogorov complexity $K(f)$ — a random function has $K(f) \approx 2^n$ bits, requiring exponential circuits.

### 26.3.2 Communication Complexity and Circuit Lower Bounds

**Theorem 26.3.2 (Karchmer-Wigderson).** The depth of a circuit computing $f: \{0,1\}^n \to \{0,1\}$ equals the communication complexity of the relation $KW_f \subseteq f^{-1}(1) \times f^{-1}(0) \times [n]$:
$$\text{depth}(f) = D^{rel}(KW_f),$$
where Alice holds $x \in f^{-1}(1)$, Bob holds $y \in f^{-1}(0)$, and they must find a coordinate $i$ with $x_i \neq y_i$.

**Theorem 26.3.3 (Monotone Circuit Lower Bounds via Communication).** The Karchmer-Wigderson relation for monotone circuits uses only monotone messages. Lower bounds on $D(KW_f)$ under monotone protocols give depth lower bounds for monotone circuits.

---

## 26.4 Expander Graphs and Information

### 26.4.1 Spectral Expansion and Information Diffusion

**Definition 26.4.1.** A $d$-regular graph $G$ on $n$ vertices is an $\varepsilon$-*expander* if for every set $S$ with $|S| \leq n/2$:
$$|N(S)| \geq (1+\varepsilon)|S|.$$

Equivalently, the spectral gap $\lambda = \lambda_1 - \lambda_2 \geq \varepsilon d$ (where $\lambda_1 = d$ is the largest eigenvalue of the adjacency matrix).

**Theorem 26.4.2 (Expander Mixing Lemma).** For a $d$-regular expander with second eigenvalue $\lambda_2$, and any sets $A, B \subseteq V$:
$$\left|E(A,B) - \frac{d|A||B|}{n}\right| \leq \lambda_2 \sqrt{|A||B|}.$$

**Information-Theoretic Interpretation:** Random walks on expanders mix rapidly (in $O(\log n)$ steps), meaning information propagates quickly. The spectral gap controls the mixing time: $\tau_{\text{mix}} = O(\log n / \lambda)$.

### 26.4.2 Pseudorandom Generators and Extractors

**Definition 26.4.3.** A *pseudorandom generator* $G: \{0,1\}^s \to \{0,1\}^n$ is a function such that $G(U_s)$ is computationally indistinguishable from $U_n$ (where $U_k$ denotes uniform on $\{0,1\}^k$).

**Definition 26.4.4 (Extractor).** An $(k, \varepsilon)$-*extractor* is a function $\text{Ext}: \{0,1\}^n \times \{0,1\}^d \to \{0,1\}^m$ such that for any source $X$ with min-entropy $H_\infty(X) \geq k$:
$$\|\text{Ext}(X, U_d) - U_m\|_1 \leq \varepsilon.$$

**Theorem 26.4.5 (Expanders as Extractors).** Expander graphs yield near-optimal randomness extractors. A random walk of length $t$ on a $d$-regular $\lambda_2$-expander starting from a high-entropy source extracts $\approx t \log d - O(t\lambda_2/d)$ nearly-uniform bits.

---

## 26.5 Coding Theory and Dynamical Systems

**Definition 26.5.1.** A *linear code* $C \subseteq \mathbb{F}_q^n$ of dimension $k$ and minimum distance $d$ encodes $k$ symbols into $n$ symbols and can correct up to $\lfloor(d-1)/2\rfloor$ errors.

**Theorem 26.5.2 (Gilbert-Varshamov Bound).** There exist codes with rate $R = k/n$ and relative distance $\delta = d/n$ satisfying:
$$R \geq 1 - H_q(\delta),$$
where $H_q(\delta) = -\delta\log_q(\delta/(q-1)) - (1-\delta)\log_q(1-\delta)$ is the $q$-ary entropy function.

**Connection to Subshifts:** Good codes correspond to subshifts with high topological entropy but good distance properties. Low-density parity-check (LDPC) codes correspond to sparse constraint graphs — the *Tanner graph* is essentially a factor graph of a Markov random field on a constrained shift.

**Theorem 26.5.3 (Capacity-Achieving Codes — Polar Codes, Arıkan 2009).** Polar codes achieve the Shannon capacity of any binary-input memoryless channel with complexity $O(n\log n)$ encoding/decoding, using a recursive structure based on the butterfly network — a discrete dynamical system.

---

## Exercises

**Exercise 26.1.** (Shearer's Lemma) Use Shearer's lemma to prove the Cauchy-Schwarz inequality for counting: if $A \subseteq [n] \times [n]$ and $R_i$ are row projections, then $|A|^2 \leq n \sum_i |R_i|$.

**Exercise 26.2.** (Communication Complexity) Design a randomized protocol for $EQ_n$ using only $O(\log n)$ bits with error $\leq 1/3$. Use pairwise-independent hashing.

**Exercise 26.3.** Prove the Karchmer-Wigderson theorem for a specific function: show that the depth of the AND function on $n$ bits equals $\log n$, and verify by computing $D(KW_{AND_n})$.

**Exercise 26.4.** (Extractors) Show that pairwise-independent hash functions give a $(k, \varepsilon)$-extractor with $d = n$ seed bits and $m = k - 2\log(1/\varepsilon)$ output bits. Find the optimal trade-off.

---

## Chapter Notes

For the entropy method in combinatorics: Alon and Spencer's *The Probabilistic Method* (Chapter 15) covers Shearer's lemma and its applications. The original entropy proof of Loomis-Whitney is due to Ruzsa and Szemerédi.

Communication complexity: Kushilevitz and Nisan's *Communication Complexity* is the standard reference. The information complexity approach is developed in Bar-Yossef et al. (2004) and Braverman-Rao (2011).

Expanders and extractors: Hoory-Linial-Wigderson's survey *Expander graphs and their applications* (Bulletin AMS, 2006) is comprehensive. Shaltiel's survey *Recent developments in explicit constructions of extractors* covers the state of the art.

Polar codes: Arıkan's original paper *Channel Polarization: A Method for Constructing Capacity-Achieving Codes* (IEEE Trans. Inf. Theory, 2009).

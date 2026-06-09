# Chapter 12 — Symbolic Dynamics

> *Every orbit is a sequence. Symbolic dynamics makes this precise, turning the continuous into the combinatorial and connecting dynamical systems to information theory, automata theory, and combinatorics on words.*

**Prerequisites:** Chapters 6 (topological dynamics, conjugacy), 7 (ergodic theory, entropy), 9 (Markov partitions).

**What this chapter builds:** The full shift and subshifts as dynamical systems; subshifts of finite type (SFTs) and their transition matrices; sofic shifts as factors of SFTs; topological entropy via word growth; the Perron-Frobenius theorem in this setting; the zeta function counting periodic orbits; and the connection to automata theory and information theory.

---

## 12.1 The Full Shift

**Definition 12.1.1.** Let $\mathcal{A}$ be a finite *alphabet* (e.g., $\mathcal{A} = \{0, 1, \ldots, k-1\}$). The *full shift* on $\mathcal{A}$ is:
$$\mathcal{A}^{\mathbb Z} = \{(x_n)_{n \in {\mathbb Z}} : x_n \in \mathcal{A}\} \quad \text{with the shift map } \sigma: \mathcal{A}^{\mathbb Z} \to \mathcal{A}^{\mathbb Z}, \quad \sigma(x)_n = x_{n+1}.$$

The *product topology* on $\mathcal{A}^{\mathbb Z}$ (where $\mathcal{A}$ has the discrete topology) is metrizable by:
$$d(x, y) = 2^{-\min\{|n| : x_n \neq y_n\}}.$$

**Properties of $(\mathcal{A}^{\mathbb Z}, \sigma)$:**
- $\mathcal{A}^{\mathbb Z}$ is compact (Tychonoff), perfect, totally disconnected — hence homeomorphic to the Cantor set (for $|\mathcal{A}| \geq 2$)
- $\sigma$ is a homeomorphism
- $(\mathcal{A}^{\mathbb Z}, \sigma)$ is topologically mixing
- The periodic points $\{x : \sigma^n(x) = x\}$ are dense (there are $k^n$ periodic points of period $n$)
- The topological entropy is $h_{\text{top}}(\sigma) = \log |\mathcal{A}|$

---

## 12.2 Subshifts

**Definition 12.2.1.** A *subshift* is a pair $(X, \sigma|_X)$ where $X \subseteq \mathcal{A}^{\mathbb Z}$ is a nonempty closed shift-invariant set ($\sigma(X) = X$).

The *language* of a subshift $X$ is $\mathcal{L}(X) = \{w \in \mathcal{A}^* : w \text{ appears as a word in some } x \in X\}$.

**Theorem 12.2.2 (Characterization of Subshifts).** A nonempty closed set $X \subseteq \mathcal{A}^{\mathbb Z}$ is a subshift iff it is defined by a collection of *forbidden words*: $X = \{x \in \mathcal{A}^{\mathbb Z} : \text{no word from } \mathcal{F} \text{ appears in } x\}$ for some $\mathcal{F} \subseteq \mathcal{A}^*$.

**Examples 12.2.3.**
- The even shift: $\mathcal{A} = \{0,1\}$, sequences where between any two 1s there is an even number of 0s. Forbidden words: $\{10^{2n+1}1 : n \geq 0\}$.
- The Thue-Morse shift: the orbit closure of the Thue-Morse sequence $0110100110010110\ldots$ (defined by: $t_0 = 0$, $t_{2n} = t_n$, $t_{2n+1} = 1 - t_n$).
- The Fibonacci shift: defined by forbidden word $\{11\}$.

---

## 12.3 Subshifts of Finite Type

**Definition 12.3.1.** A *subshift of finite type (SFT)* is a subshift defined by a finite set of forbidden words $\mathcal{F}$. Equivalently, after taking the "higher block" representation, an SFT can be described by a *transition matrix*.

**Theorem 12.3.2 (Vertex SFT).** Every SFT is conjugate to a *vertex SFT*: given by a finite directed graph $G = (V, E)$ where $\mathcal{A} = V$ (or $\mathcal{A} = E$) and $X_G = \{(x_n) : (x_n, x_{n+1}) \in E \text{ for all } n\}$ (or the edge version).

The *transition matrix* $A \in M_{|V|}(\{0,1\})$ has $A_{ij} = 1$ iff $(i,j) \in E$.

**Example 12.3.3 (Golden Mean Shift).** $\mathcal{A} = \{0,1\}$, forbidden word $\{11\}$. Transition matrix:
$$A = \begin{pmatrix} 1 & 1 \\ 1 & 0 \end{pmatrix}.$$
This is the vertex SFT on 2 vertices where state 1 can be followed by 0 or 1, but state 2 can only be followed by state 1. The Perron-Frobenius eigenvalue is $\lambda_+ = (1+\sqrt{5})/2$ (the golden ratio).

---

## 12.4 Topological Entropy of Subshifts

**Definition 12.4.1.** The *word complexity function* $p_X(n)$ counts the number of distinct words of length $n$ in $\mathcal{L}(X)$: $p_X(n) = |\{w \in \mathcal{A}^n : w \in \mathcal{L}(X)\}|$.

**Theorem 12.4.2.** The topological entropy of a subshift $X$ is:
$$h_{\text{top}}(X) = \lim_{n \to \infty} \frac{1}{n} \log p_X(n).$$

The limit exists by subadditivity: $p_X(m+n) \leq p_X(m) \cdot p_X(n)$ (Fekete's lemma).

**Theorem 12.4.3 (Entropy of SFTs).** For an irreducible vertex SFT with transition matrix $A$:
$$h_{\text{top}}(X_A) = \log \lambda_{\text{PF}}(A).$$

*Proof:* The number of paths of length $n$ in $G$ is $\sum_{i,j} (A^n)_{ij} = \mathbf{1}^T A^n \mathbf{1}$. By Perron-Frobenius, $(A^n)_{ij} \sim c_{ij} \lambda_{\text{PF}}^n$, so the growth rate is $\log \lambda_{\text{PF}}$.

---

## 12.5 Sofic Shifts

**Definition 12.5.1.** A *sofic shift* is the image of an SFT under a sliding block code (a continuous, shift-commuting map that reads a window of symbols and outputs a single symbol).

Equivalently: $X$ is sofic iff $\mathcal{L}(X)$ is a *regular language* (accepted by a finite automaton).

**Theorem 12.5.2.** Every SFT is sofic, but not every sofic shift is an SFT.

**Counter-example 12.5.3 (Even Shift).** The even shift is sofic (its language is regular) but is not an SFT (it requires infinitely many forbidden words to describe).

**Theorem 12.5.4 (Fischer Cover).** Every irreducible sofic shift has a canonical minimal deterministic presentation: the *Fischer cover* (or *left Krieger cover*). The Fischer cover is the unique minimal edge SFT that maps onto $X$ by a 1-block code.

---

## 12.6 The Zeta Function

**Definition 12.6.1.** The *dynamical zeta function* of $(X, \sigma)$ is the formal power series:
$$\zeta_X(t) = \exp\left(\sum_{n=1}^\infty \frac{|\text{Fix}(\sigma^n)|}{n} t^n\right),$$
where $|\text{Fix}(\sigma^n)| = |\{x \in X : \sigma^n(x) = x\}|$ counts periodic points.

**Theorem 12.6.2 (Rationality for SFTs).** For an irreducible SFT with transition matrix $A$:
$$\zeta_{X_A}(t) = \frac{1}{\det(I - tA)}.$$

*Proof:* $|\text{Fix}(\sigma^n)| = \text{tr}(A^n) = \sum_i \lambda_i^n$ (by Perron-Frobenius). Then $\sum_{n \geq 1} \frac{\text{tr}(A^n)}{n} t^n = -\log \det(I - tA)$, so $\zeta(t) = 1/\det(I-tA)$.

**Example 12.6.4.** For the golden mean shift with $A = \begin{pmatrix}1&1\\1&0\end{pmatrix}$: $\det(I - tA) = (1-t)(1) - t \cdot t \cdot (-1) = 1 - t - t^2$. So $\zeta_{GM}(t) = 1/(1-t-t^2)$.

---

## 12.7 Conjugacy and Classification

The isomorphism problem for SFTs: when are two SFTs topologically conjugate?

**Theorem 12.7.1 (Williams, 1973).** Two irreducible SFTs are flow equivalent (there is a homeomorphism of their suspension flows) iff they have the same *dimension group* (a certain group associated to the matrix $A$, invariant under a relation called *strong shift equivalence*).

The isomorphism problem for conjugacy (not flow equivalence) is harder. Strong shift equivalence is computable in principle but has no known polynomial-time algorithm.

**Definition 12.7.2 (Strong Shift Equivalence).** Matrices $A, B$ over ${\mathbb Z}_{\geq 0}$ are *elementary equivalent* (over one step) if there exist matrices $R, S$ with $A = RS$ and $B = SR$. They are *strong shift equivalent* if they are connected by a sequence of elementary equivalences.

**Theorem 12.7.3 (Williams).** SFTs $X_A$ and $X_B$ are topologically conjugate iff $A$ and $B$ are strong shift equivalent.

**Corollary 12.7.4 (Kim-Roush).** The strong shift equivalence relation is decidable (over $\{0,1\}$ matrices). Over ${\mathbb Z}$ it is undecidable (Kim-Roush, 1992).

---

## 12.8 Automorphisms of Shifts

**Definition 12.8.1.** The *automorphism group* $\text{Aut}(\sigma)$ of a full shift $(\mathcal{A}^{\mathbb Z}, \sigma)$ consists of all homeomorphisms $\phi: \mathcal{A}^{\mathbb Z} \to \mathcal{A}^{\mathbb Z}$ that commute with $\sigma$.

**Theorem 12.8.2 (Curtis-Hedlund-Lyndon).** Every automorphism of a shift is a *sliding block code*: there is a window size $N$ and a function $\Phi: \mathcal{A}^{2N+1} \to \mathcal{A}$ such that $\phi(x)_n = \Phi(x_{n-N}, \ldots, x_{n+N})$.

**Theorem 12.8.3.** The automorphism group $\text{Aut}(\sigma)$ of the full 2-shift is a countable group containing:
- All finite-order homeomorphisms (from finite permutations of $\mathcal{A}$)
- Powers of $\sigma$ (so ${\mathbb Z} \hookrightarrow \text{Aut}(\sigma)$)
- The *marker automorphisms* (Ryan's theorem: the center of $\text{Aut}(\sigma)$ is $\langle \sigma \rangle$)
- Free groups and other exotic groups

---

## 12.9 The Connection to Information Theory

The symbolic dynamics / information theory dictionary:

| Symbolic Dynamics | Information Theory |
|---|---|
| Subshift $X$ | Stationary source (stochastic process) |
| Topological entropy $h_{\text{top}}(X)$ | Maximum entropy rate |
| Measure of maximal entropy | Source achieving maximum entropy |
| Parry measure on an SFT | Markov source (maximal entropy) |
| Sofic shift | Hidden Markov source |
| Factor map $\pi: X \to Y$ | Noisy channel |
| Conjugacy | Lossless coding |
| Sliding block code $(N,M)$ | Block code with delay $N$ and anticipation $M$ |
| SFT property | Finite memory channel |
| Entropy $h(f, \xi)$ | Entropy rate of the coded process |

**Theorem 12.9.1 (Parry Measure).** Every irreducible SFT $X_A$ has a unique measure of maximal entropy (the *Parry measure*). It is the Markov measure defined by the transition probabilities:
$$P(i \to j) = \frac{A_{ij} r_j}{\lambda_{\text{PF}} r_i},$$
where $r$ and $l$ are the right and left Perron-Frobenius eigenvectors of $A$, and the initial distribution is $\pi_i = l_i r_i$.

The Parry measure realizes $h_\mu(\sigma) = h_{\text{top}}(X_A) = \log \lambda_{\text{PF}}$.

---

## Exercises

**Exercise 12.1.** Show that the golden mean shift (forbidden word $\{11\}$) is a subshift of finite type. Write the transition matrix and compute $p(n)$ for small $n$ (the number of allowed words of length $n$). Verify $h_{\text{top}} = \log(1+\sqrt{5})/2$.

**Exercise 12.2.** Prove that the even shift (sequences where all runs of 0s between consecutive 1s have even length) has a sofic presentation but is not an SFT. (*Hint:* Show that any set of forbidden words that defines the even shift must be infinite.)

**Exercise 12.3.** Compute the zeta function of the golden mean shift using $\zeta(t) = 1/\det(I - tA)$. Expand as a power series and verify the coefficient of $t^n$ equals $|\text{Fix}(\sigma^n)|$ for the golden mean shift.

**Exercise 12.4.** (Parry Measure) For the golden mean shift, compute the Parry measure: find the Perron-Frobenius eigenvectors of $A = \begin{pmatrix}1&1\\1&0\end{pmatrix}$ and write the Markov transition probabilities.

**Exercise 12.5.** Let $X$ be a subshift. Show that $p_X(n+1)/p_X(n) \geq 1$ (the complexity function is nondecreasing). Show that $X$ is periodic (all orbits periodic) iff $p_X(n) = $ const for all large $n$.

**Exercise 12.6.** (Collatz Connection) Consider the Collatz map as a coding: to each $n \in {\mathbb N}$, associate the sequence of parities $c_k = n_k \pmod 2$ where $n_{k+1} = T(n_k)$. The sequence $c \in \{0,1\}^{\mathbb N}$ is the *Collatz itinerary* of $n$. What subshift does the set of all Collatz itineraries generate? Is it an SFT? A sofic shift?

**Exercise 12.7.** (Research) The *entropy of the Collatz process*: if we view the parity sequence $c_n = T^n(m) \pmod 2$ as a stationary process (under some invariant measure), what is its entropy rate? What constraints does the Collatz conjecture place on this entropy?

---

## Chapter Notes

The standard text is Lind and Marcus, *An Introduction to Symbolic Dynamics and Coding* — the best reference for the subject, written at the right level. The original paper by Morse-Hedlund (1938) is worth reading for historical perspective.

For the connection to information theory and automata: sofic shifts are exactly the systems whose generating process is a hidden Markov model. The paper *Hidden Markov Processes* by Ephraim and Merhav (*IEEE Transactions on Information Theory*, 2002) surveys the information-theoretic perspective.

The automorphism group of the full shift (Section 12.8) is a rich algebraic object studied by Boyle, Lind, Rudolph, and others. The undecidability of strong shift equivalence over ${\mathbb Z}$ (Kim-Roush 1992) is related to the undecidability of the word problem for groups — a connection explored further in Chapter 27.

Chapter 24 makes the symbolic dynamics/information theory dictionary precise via the ergodic AEP.

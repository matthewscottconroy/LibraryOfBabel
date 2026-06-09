# Chapter 40 — Circuit Complexity and Information Theory

> *Can every NP problem be solved in polynomial time? The P vs NP question. Information-theoretic lower bounds for circuits — entropy arguments — give the only unconditional complexity lower bounds we know. The frontier is small-depth circuits, monotone circuits, and the connection between Fourier analysis and computational complexity.*

**Prerequisites:** Chapter 26 (entropy method, communication complexity, expanders), Chapter 18 (Kolmogorov complexity), Chapter 16 (Shannon entropy, mutual information).

---

## 40.1 Boolean Circuits and Complexity

**Definition 40.1.1.** A *Boolean circuit* $C$ on $n$ inputs is a directed acyclic graph where:
- Input nodes: labeled $x_1, \ldots, x_n$
- Internal gates: labeled AND ($\wedge$), OR ($\vee$), NOT ($\neg$) with fan-in 2 (or unlimited for *unbounded fan-in*)
- One output node

The *size* $|C|$ is the number of gates; the *depth* is the length of the longest path from input to output.

**Definition 40.1.2 (Circuit Complexity Classes).** 
- $\mathbf{P/poly}$: functions computable by polynomial-size circuits
- $\mathbf{NC}^1$: functions computable by $O(\log n)$-depth, polynomial-size circuits  
- $\mathbf{AC}^0$: functions computable by constant-depth, polynomial-size circuits with unbounded fan-in AND/OR gates
- $\mathbf{ACC}^0$: $\mathbf{AC}^0$ augmented with $\text{MOD}_m$ gates (modular counting)

Hierarchy: $\mathbf{NC}^1 \subseteq \mathbf{AC}^0 \subseteq \mathbf{ACC}^0 \subseteq \mathbf{NC} \subseteq \mathbf{P/poly}$.

---

## 40.2 Lower Bounds via Entropy and Counting

### 40.2.1 Shannon's Counting Argument

**Theorem 40.2.1 (Shannon 1949 — Counting Lower Bound).** For random $f: \{0,1\}^n \to \{0,1\}$, the minimum circuit size is $\Omega(2^n/n)$.

*(proof)* There are $2^{2^n}$ Boolean functions but only $(2s)^{2s}$ distinct functions of size-$s$ circuits. For $s = c2^n/n$: $(2s)^{2s} < 2^{2^n}$, so most functions require size $\Omega(2^n/n)$.

**Information-Theoretic Version:** A circuit of size $s$ can be described in $O(s\log s)$ bits. A random function requires $\Omega(2^n)$ bits to describe (its truth table). So circuit size $\Omega(2^n/n)$ follows from Kolmogorov complexity considerations.

### 40.2.2 Worst-Case vs. Average-Case

**Definition 40.2.2.** A function $f$ is *$(s, \varepsilon)$-hard on average* if for every circuit $C$ of size $s$:
$$\Pr_x[C(x) = f(x)] \leq 1/2 + \varepsilon.$$

**Theorem 40.2.3 (Yao's XOR Lemma, 1982).** If $f$ is $(s, 1/3)$-hard, then $f \oplus f$ (XOR of two independent copies) is $(s^{1/2}, 2^{-\Omega(n)})$-hard. Iterating: $f^{\oplus k}$ is exponentially hard on average.

**Remark:** Yao's XOR lemma connects worst-case and average-case hardness — a key tool for pseudorandom generators.

---

## 40.3 AC⁰ Lower Bounds: Switching Lemma

**Theorem 40.3.1 (Håstad's Switching Lemma, 1987).** Let $f: \{0,1\}^n \to \{0,1\}$ be computed by a size-$m$ DNF (disjunctive normal form with clauses of width $w$). For a random restriction $\rho$ that fixes each variable independently to $0$ or $1$ with probability $1-p$ each and leaves it free with probability $p$:
$$\Pr_\rho[f|_\rho \text{ requires decision tree of depth} > t] \leq (5pw)^t.$$

**Corollary 40.3.2 (Parity Not in AC⁰).** The parity function $\oplus_n(x) = x_1 \oplus \cdots \oplus x_n$ requires exponential size in $\mathbf{AC}^0$. More precisely, any depth-$d$ circuit for parity requires size $\exp(\Omega(n^{1/(d-1)}))$.

**Information-Theoretic Intuition:** Parity is information-theoretically "maximally sensitive" — flipping any one bit flips the output. Constant-depth circuits cannot track this sensitivity without exponential size.

### 40.3.2 Fourier Analysis of Boolean Functions

**Definition 40.3.3.** Every $f: \{0,1\}^n \to \{-1,1\}$ has a *Fourier expansion* $f(x) = \sum_{S \subseteq [n]} \hat{f}(S) \prod_{i \in S} (-1)^{x_i}$, where $\hat{f}(S) = \mathbb{E}_x[f(x)\prod_{i \in S}(-1)^{x_i}]$.

**Parseval's Identity:** $\sum_S \hat{f}(S)^2 = 1$.

**Theorem 40.3.4 (Total Influence = Noise Sensitivity).** The *total influence* $I(f) = \sum_i \Pr_x[f(x) \neq f(x \oplus e_i)]$ satisfies $I(f) = \sum_S |S| \hat{f}(S)^2$. Functions in $\mathbf{AC}^0$ have $I(f) = O((\log n)^{d-1})$ (polynomial of log), while parity has $I(\oplus_n) = n$.

---

## 40.4 Monotone Complexity

**Definition 40.4.1.** A circuit is *monotone* if it uses no NOT gates. The monotone circuit complexity $C^+(f)$ is the minimum size over all monotone circuits computing $f$.

**Theorem 40.4.2 (Razborov, 1985 — Clique Requires Exponential Monotone Circuits).** The clique function $\text{CLIQUE}_{k,n}$ (does the $n$-vertex graph have a $k$-clique?) requires monotone circuit size $\exp(\Omega(k))$ for $k = n^{1/4}$.

*(proof outline)* Razborov's *approximation method*: any monotone circuit for CLIQUE must approximate two distributions — random cliques and random sets without large cliques. Entropy bounds show no small monotone circuit can separate them.

**Theorem 40.4.3 (Alon-Boppana, 1987).** For the bipartite matching problem on $n \times n$ bipartite graphs, monotone circuits require size $\exp(\Omega(n^{1/5}))$.

---

## 40.5 Natural Proofs and the Limits of Lower Bounds

**Definition 40.5.1 (Razborov-Rudich, 1994).** A complexity lower bound proof is a *natural proof* if the property $P: \{f: \{0,1\}^n \to \{0,1\}\} \to \{0,1\}$ used in the proof satisfies:
1. *Constructivity*: $P$ is computable in $2^{O(n)}$ time from the truth table
2. *Largeness*: $\Pr_f[P(f) = 1] \geq 1/\text{poly}(n)$ (many functions have property $P$)
3. *Usefulness*: $f \in \mathbf{P/poly}$ implies $P(f) = 0$ (hard functions fail $P$)

**Theorem 40.5.2 (Razborov-Rudich Natural Proof Barrier).** If pseudorandom generators exist in $\mathbf{P/poly}$ with exponential hardness, then no natural proof can prove superpolynomial lower bounds against $\mathbf{P/poly}$.

**Interpretation:** The switching lemma, Razborov's approximations, and other known lower bound methods are all "natural proofs." This theorem says that to prove $\mathbf{P} \neq \mathbf{NP}$, we need fundamentally non-natural techniques.

**The Three Barriers to P≠NP:**
1. *Relativization* (Baker-Gill-Solovay 1975): diagonalization fails
2. *Natural proofs* (Razborov-Rudich 1994): constructive arguments fail
3. *Algebrization* (Aaronson-Wigderson 2009): algebraic relativization fails

---

## 40.6 Information Complexity and Communication

**Theorem 40.6.1 (Information Complexity vs. Circuit Complexity).** For a Boolean function $f$ computed by a communication protocol with information complexity $IC(f)$:
$$C^{\text{cc}}(f) \geq IC(f) \geq \log C^{\text{circuit}}(f),$$
where $C^{\text{cc}}$ is communication complexity and $C^{\text{circuit}}$ is circuit complexity (via Karchmer-Wigderson).

**Theorem 40.6.2 (Data Structure Lower Bounds via Entropy).** For a data structure that stores $n$ elements and answers queries:
- Any static data structure for predecessor queries requires $\Omega(\log\log n)$ probe time (van Emde Boas lower bound via entropy)
- Any dynamic data structure for union-find requires $\Omega(\alpha(n))$ amortized time (inverse Ackermann, by an entropy argument on the operation sequence)

---

## Exercises

**Exercise 40.1.** (Counting Lower Bound) Show that the function $f: \{0,1\}^{20} \to \{0,1\}$ with the highest circuit complexity requires at least $c \cdot 2^{20}/20$ gates for some constant $c$. Compute the bound explicitly.

**Exercise 40.2.** (Fourier Analysis) Compute the Fourier coefficients of the majority function $\text{MAJ}_3(x_1, x_2, x_3) = 1$ iff $x_1 + x_2 + x_3 \geq 2$. What is the total influence $I(\text{MAJ}_3)$?

**Exercise 40.3.** Verify the switching lemma for a width-2 DNF with 4 clauses. Apply a random restriction with $p = 1/4$ and compute the probability that the restricted function requires a depth-2 decision tree.

**Exercise 40.4.** (Research) Identify one complexity lower bound proof that is *not* a natural proof (i.e., violates at least one of the constructivity, largeness, or usefulness conditions). Explain why it avoids the Razborov-Rudich barrier.

---

## Chapter Notes

Håstad's switching lemma: *Computational Limitations of Small-Depth Circuits* (MIT Press, 1987). Fourier analysis of Boolean functions: O'Donnell's *Analysis of Boolean Functions* (Cambridge, 2014) is the comprehensive reference.

Razborov's monotone lower bounds: *Lower bounds on monotone complexity of the logical permanent* (Math. Notes USSR, 1985). The natural proof barrier: Razborov-Rudich *Natural proofs* (J. CSS, 1997).

The three barriers to P≠NP are surveyed in Aaronson's *Algebrization: A New Barrier in Complexity Theory* (2009) and his book *Quantum Computing Since Democritus* (Cambridge, 2013). Arora-Barak's *Computational Complexity: A Modern Approach* (Cambridge, 2009) covers circuit complexity comprehensively.

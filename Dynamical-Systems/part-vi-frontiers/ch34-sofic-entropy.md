# Chapter 34 — Sofic Groups and Sofic Entropy

> *Sofic groups are groups that can be approximated by finite symmetric groups. For actions of sofic groups, Lewis Bowen defined a new notion of entropy in 2010 that extends Kolmogorov-Sinai entropy beyond the amenable setting. This is one of the most significant developments in ergodic theory in decades.*

**Prerequisites:** Chapter 7 (KS entropy, Ornstein's theorem), Chapter 33 (orbit equivalence, amenable groups), Chapter 12 (symbolic dynamics, sofic shifts).

---

## 34.1 Sofic Groups

**Definition 34.1.1 (Gromov-Weiss, 1999).** A countable group $\Gamma$ is *sofic* if for every $\varepsilon > 0$ and every finite set $F \subseteq \Gamma$, there exists $n \in {\mathbb N}$ and a map $\sigma: \Gamma \to \text{Sym}(n)$ (the symmetric group on $\{1,\ldots,n\}$) such that:
1. $|\{i \in [n] : \sigma(\gamma\gamma')(i) = \sigma(\gamma)\sigma(\gamma')(i)\}| \geq (1-\varepsilon)n$ for all $\gamma, \gamma' \in F$ (almost a homomorphism)
2. $|\{i \in [n] : \sigma(\gamma)(i) \neq i\}| \geq (1-\varepsilon)n$ for all $1 \neq \gamma \in F$ (almost free)

**Examples 34.1.2.**
- All amenable groups are sofic (Weiss, 2000)
- All residually finite groups are sofic (e.g., free groups, $SL(n, {\mathbb Z})$)
- All sofic groups from residually amenable groups
- No non-sofic group is known to exist (this is a major open problem)

**Open Problem 34.1.3.** Does there exist a non-sofic group? This is one of the biggest open problems in group theory. A non-sofic group would be a group with no finite approximations whatsoever.

---

## 34.2 Bowen's Sofic Entropy

### 34.2.1 Definition via Microstates

**Setup:** Let $\Gamma \curvearrowright (X, \mu)$ be a free measure-preserving action of a sofic group $\Gamma$. Let $\Sigma = (\sigma_n: \Gamma \to \text{Sym}(d_n))$ be a sofic approximation.

**Definition 34.2.1 (Bowen, 2010).** Let $\xi = \{A_1, \ldots, A_k\}$ be a measurable partition of $X$. For a sofic approximation $\Sigma$ and $n$ large:
- A *microstate* is a map $\phi: [d_n] \to \{1,\ldots,k\}$ (a coloring of $[d_n]$) that "looks like" the partition $\xi$ of $X$ with respect to the $\Gamma$-action.
- The *microstate space* $\text{Map}(\xi, F, \delta, \sigma_n)$ is the set of colorings $\phi$ such that for $s \in F$ and $i$ in a $(1-\delta)d_n$-fraction of $[d_n]$: $\phi(\sigma_n(s)(i)) = j$ iff $s\cdot x \in A_j$ (approximately equivariant)

**Definition 34.2.2.** The *sofic entropy* of the partition $\xi$ with respect to $\Sigma$ is:
$$h_\Sigma(\xi, \Gamma \curvearrowright X) = \inf_{F, \delta} \limsup_{n\to\infty} \frac{1}{d_n} \log |\text{Map}(\xi, F, \delta, \sigma_n)|.$$

The *sofic entropy* of the action is $h_\Sigma(\Gamma \curvearrowright X) = \sup_\xi h_\Sigma(\xi, \Gamma \curvearrowright X)$.

### 34.2.2 Properties of Sofic Entropy

**Theorem 34.2.3 (Bowen, 2010).** For a free ergodic action of a sofic group $\Gamma$:
1. $h_\Sigma(\Gamma \curvearrowright X) \in [-\infty, \infty]$ (can be $-\infty$)
2. $h_\Sigma$ is an invariant of the action (independent of the choice of sofic approximation $\Sigma$, for Bernoulli actions)
3. For Bernoulli actions $\Gamma \curvearrowright (X_0, \mu_0)^\Gamma$: $h_\Sigma = H(\mu_0)$ (the base entropy)
4. If $\Gamma$ is amenable: $h_\Sigma$ equals the classical KS entropy

**Theorem 34.2.4 (Kerr-Li, 2011).** Sofic entropy is independent of the sofic approximation for all actions (not just Bernoulli), confirming it is a genuine invariant.

---

## 34.3 The Non-Bernoulli Problem and Sofic Entropy

**The Classical Ornstein Problem:** For amenable groups, Bernoulli shifts with the same base entropy are isomorphic (Ornstein's theorem). What happens for non-amenable groups?

**Theorem 34.3.1 (Bowen, 2010).** Two Bernoulli shifts $\Gamma \curvearrowright (X_0)^\Gamma$ and $\Gamma \curvearrowright (Y_0)^\Gamma$ of a free group $\Gamma = F_r$ are isomorphic iff $H(X_0) = H(Y_0)$. The sofic entropy distinguishes them.

**Remark 34.3.2.** This resolved a long-open problem: Bernoulli shifts of free groups are classified by their entropy, generalizing Ornstein's theorem to the non-amenable setting.

**Theorem 34.3.3 (Ornstein and Weiss, Bowen).** For any sofic group $\Gamma$: if two free ergodic actions are orbit equivalent, their sofic entropies satisfy:
$$h_\Sigma(\Gamma \curvearrowright X) \leq h_\Sigma(\Lambda \curvearrowright Y) \quad \text{if } \Gamma \leq \Lambda \text{ as orbit-equivalence relations.}$$

Sofic entropy is monotone under "orbit sub-relations."

---

## 34.4 Entropy Beyond Sofic Groups

**Definition 34.4.1 (Rokhlin Entropy — Seward, 2014).** For any free ergodic action $\Gamma \curvearrowright (X, \mu)$ (not assuming soficity):
$$h_{\text{Rok}}(\Gamma \curvearrowright X) = \inf_\xi H(\xi | \xi^-),$$
where the infimum is over all generating partitions $\xi$ and $\xi^- = \bigvee_{e \neq \gamma \in \Gamma} \gamma\xi$ is the "past" partition.

**Theorem 34.4.2 (Seward, 2020).** Rokhlin entropy agrees with sofic entropy for sofic groups: if $\Gamma$ is sofic and the action is free and ergodic:
$$h_{\text{Rok}}(\Gamma \curvearrowright X) = h_\Sigma(\Gamma \curvearrowright X).$$

**Theorem 34.4.3.** Rokhlin entropy is defined for all groups, not just sofic ones. It satisfies:
- $h_{\text{Rok}} \leq H(\xi)$ for any generating partition $\xi$
- For Bernoulli shifts: $h_{\text{Rok}}(\Gamma \curvearrowright (X_0)^\Gamma) = H(X_0)$
- For amenable groups: $h_{\text{Rok}}$ equals KS entropy

---

## 34.5 Topological Sofic Entropy

**Definition 34.5.1 (Kerr-Li, 2011).** For a continuous action $\Gamma \curvearrowright X$ of a sofic group on a compact metric space:
$$h_\Sigma^{\text{top}}(\Gamma \curvearrowright X) = \sup_{\mu \in M_\Gamma(X)} h_\Sigma(\Gamma \curvearrowright (X, \mu)),$$
where the sup is over $\Gamma$-invariant probability measures.

**Theorem 34.5.2 (Variational Principle for Sofic Entropy).** For sofic group actions on compact spaces:
$$h_\Sigma^{\text{top}}(\Gamma \curvearrowright X) \geq \sup_\mu h_\Sigma(\Gamma \curvearrowright (X, \mu)),$$
with equality when the action is expansive.

**Remark 34.5.3.** The variational principle may fail in general for sofic entropy — this is a fundamental difference from the classical (amenable) case, where the variational principle always holds.

---

## Exercises

**Exercise 34.1.** Verify that all finite groups are sofic (with the trivial sofic approximation given by the Cayley graph). Show that every subgroup of a sofic group is sofic.

**Exercise 34.2.** (Microstate computation) For $\Gamma = {\mathbb Z}$ acting on $X = \{0,1\}^{\mathbb Z}$ by the shift (Bernoulli shift with fair coin): write down explicitly what a "microstate" $\phi: [n] \to \{0,1\}$ looks like for the sofic approximation $\sigma_n(1) = $ cyclic shift on $[n]$. Count the number of microstates and verify $h_\Sigma = \log 2$.

**Exercise 34.3.** Show that the sofic entropy of the trivial action $\Gamma \curvearrowright (\{*\}, \delta_*)$ (single point) is 0. What is the sofic entropy of the action $\Gamma \curvearrowright (X, \mu)$ where $\mu$ is an atomic measure with atoms of size $1/k$?

**Exercise 34.4.** (Research) The question of whether sofic entropy depends on the sofic approximation is open for general actions. Find a reference for Kerr-Li's proof that it is independent, and outline the key idea.

---

## Chapter Notes

Gromov introduced sofic groups in *Endomorphisms of Symbolic Algebraic Varieties* (J. European Math. Soc., 1999); the term "sofic" was coined by Weiss in *Sofic groups and dynamical systems* (Sankhyā, 2000).

Bowen's sofic entropy: *Measure conjugacy invariants for actions of countable sofic groups* (J. AMS, 2010). Kerr-Li's topological sofic entropy and independence of sofic approximation: *Entropy and the variational principle for actions of sofic groups* (Inventiones, 2011).

Seward's Rokhlin entropy: *Krieger's finite generator theorem for actions of countable groups* (Invent. Math., 2019, 2020). The connection between sofic groups, entropy, and von Neumann algebras is surveyed in Hayes's *A random matrix approach to the Peterson-Thom conjecture* (Indiana Math. J., 2021).

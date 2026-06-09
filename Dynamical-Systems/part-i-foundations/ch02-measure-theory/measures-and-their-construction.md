# 2.2 Measures and Their Construction

## 2.2.1 Measures

A measure is a function that assigns a non-negative size to each measurable set, consistently with the idea that disjoint pieces can be combined: the size of a union of disjoint sets is the sum of their sizes.

**Definition 2.2.1.** A *measure* on $(\Omega, \mathcal{F})$ is a function $\mu: \mathcal{F} \to [0, \infty]$ satisfying:
1. $\mu(\emptyset) = 0$
2. If $A_1, A_2, \ldots \in \mathcal{F}$ are pairwise disjoint, then $\mu\left(\bigcup_{n=1}^\infty A_n\right) = \sum_{n=1}^\infty \mu(A_n)$ (*countable additivity*)

The triple $(\Omega, \mathcal{F}, \mu)$ is a *measure space*. If $\mu(\Omega) < \infty$, $\mu$ is a *finite measure*. If $\mu(\Omega) = 1$, $\mu$ is a *probability measure*.

Countable additivity is the key axiom. Finite additivity alone (the sum rule for finitely many disjoint sets) would not be enough to support a useful theory of integration. Countable additivity is what makes the convergence theorems in Section 2.3 work.

Let's see the range of measures you'll encounter:

**Examples 2.2.2.**
- *Lebesgue measure* $\lambda$ on $(\mathbb{R}, \mathcal{B}(\mathbb{R}))$: $\lambda([a,b]) = b - a$. This is the formal version of "length." More generally, $\lambda^n$ on $\mathbb{R}^n$ assigns to each box $[a_1,b_1] \times \cdots \times [a_n,b_n]$ the measure $(b_1-a_1)\cdots(b_n-a_n)$.
- *Counting measure* on $(\mathbb{N}, 2^{\mathbb{N}})$: $\mu(A) = |A|$ (the cardinality of $A$). Every point has measure 1.
- *Dirac measure* $\delta_x$ concentrated at $x$: $\delta_x(A) = 1$ if $x \in A$, else $0$. A point mass.
- *Discrete probability measure*: $\mu = \sum_i p_i \delta_{x_i}$ where $p_i \geq 0$ and $\sum p_i = 1$. A probability distribution on a finite or countable set.

These examples span a wide range: Lebesgue measure is "continuous" (every point has measure zero), counting measure assigns weight to individual points, and Dirac masses concentrate all weight on a single point. The theory handles all of them uniformly.

Basic properties follow almost immediately from the definition:

**Proposition 2.2.3 (Properties of Measures).**
1. *Monotonicity:* $A \subseteq B$ implies $\mu(A) \leq \mu(B)$.
2. *Subadditivity:* $\mu(\bigcup_n A_n) \leq \sum_n \mu(A_n)$.
3. *Continuity from below:* If $A_1 \subseteq A_2 \subseteq \cdots$, then $\mu(\bigcup_n A_n) = \lim_n \mu(A_n)$.
4. *Continuity from above:* If $A_1 \supseteq A_2 \supseteq \cdots$ and $\mu(A_1) < \infty$, then $\mu(\bigcap_n A_n) = \lim_n \mu(A_n)$.

Properties (3) and (4) are the measure-theoretic versions of the statement that measures behave well under limits. They're used constantly when computing measures of complicated sets by approximating with simpler ones. Note that (4) requires $\mu(A_1) < \infty$ — without this, the decreasing limit could involve $\infty - \infty$, which is undefined.

## 2.2.2 Carathéodory's Extension Theorem

The key theorem for constructing measures. Rather than defining a measure on all measurable sets at once — which would be circular — you define it on a simple class (like intervals), and then extend.

**Theorem 2.2.4 (Carathéodory).** Let $\mathcal{A}$ be an algebra on $\Omega$ (closed under finite unions and complements) and $\mu_0: \mathcal{A} \to [0, \infty]$ a countably additive function on $\mathcal{A}$. Then $\mu_0$ extends to a measure on $\sigma(\mathcal{A})$. If $\mu_0(\Omega) < \infty$, the extension is unique.

This is how Lebesgue measure is actually constructed. You start with the algebra of finite unions of intervals, define length on it (additively), and Carathéodory's theorem extends this to the entire Borel $\sigma$-algebra. The proof is nontrivial — you have to define an outer measure on all subsets, then restrict to the "measurable" ones — but the upshot is that the extension is canonical and unique.

**Definition 2.2.5.** A measure space $(\Omega, \mathcal{F}, \mu)$ is *complete* if: whenever $A \in \mathcal{F}$, $\mu(A) = 0$, and $B \subseteq A$, we have $B \in \mathcal{F}$. The *completion* of any measure space is obtained by adjoining all subsets of null sets.

Completeness here is different from completeness of a metric space — but the spirit is the same: closing under a relevant limit operation. The Lebesgue measure space is the completion of the Borel measure space. The extra "Lebesgue measurable" sets that are not Borel sets are all subsets of Borel null sets.

With measures in hand, we can build the Lebesgue integral — and that's where the real power of the theory emerges.

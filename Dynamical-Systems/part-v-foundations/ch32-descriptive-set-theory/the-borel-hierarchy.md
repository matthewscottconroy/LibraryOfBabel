# 32.1 The Borel Hierarchy

When Borel and Lebesgue developed measure theory at the turn of the 20th century, they needed to understand which sets of real numbers can be assigned a measure. The Borel sets — the smallest $\sigma$-algebra containing the open sets — were the first answer. But are all Borel sets the same? Can we distinguish "simple" Borel sets from "complicated" ones?

Yes — via the Borel hierarchy. The hierarchy classifies Borel sets by how many times you need to take complements and countable unions to build them. At the bottom are the open and closed sets. Above them are the $F_\sigma$ sets (countable unions of closed sets) and $G_\delta$ sets (countable intersections of open sets). Above them are $F_{\sigma\delta}$, $G_{\delta\sigma}$, and so on, through all countable ordinals.

This hierarchy turns out to be exactly the right language for classifying dynamical properties — which we already glimpsed in Chapter 27 (the arithmetic hierarchy of computability theory). Here we see the same structure from the topological side.

## 32.1.1 Borel Sets and Their Complexity

**Definition 32.1.1.** In a Polish space (completely metrizable separable topological space) $X$:
- $\Sigma^0_1$: open sets
- $\Pi^0_1$: closed sets (complements of open sets)
- $\Sigma^0_{\alpha+1}$: countable unions of $\Pi^0_\alpha$ sets
- $\Pi^0_{\alpha+1}$: countable intersections of $\Sigma^0_\alpha$ sets
- $\Delta^0_\alpha = \Sigma^0_\alpha \cap \Pi^0_\alpha$: the ambiguous class

The *Borel sets* are $\bigcup_{\alpha < \omega_1} \Sigma^0_\alpha$ (the union over all countable ordinals).

The $\Sigma/\Pi$ notation mirrors the logical notation: $\Sigma$ means "existential" (a set is $\Sigma^0_{\alpha+1}$ if it's a countable *union* — there *exists* an index such that the point is in the corresponding piece), and $\Pi$ means "universal" (a set is $\Pi^0_{\alpha+1}$ if it's a countable *intersection* — *for all* indices, the point is in the corresponding piece). The superscript 0 indicates "first-order" (dealing with points, not sets of points).

**Example 32.1.2.**
- $F_\sigma$ sets: countable unions of closed sets = $\Sigma^0_2$
- $G_\delta$ sets: countable intersections of open sets = $\Pi^0_2$
- $F_{\sigma\delta}$ = $\Pi^0_3$, $G_{\delta\sigma}$ = $\Sigma^0_3$, etc.

**Theorem 32.1.3 (Hierarchy is Strict).** Each class in the Borel hierarchy is strictly larger than the previous. No Borel set is missing from the hierarchy; every Borel set has a well-defined Borel rank.

The strictness of the hierarchy means that the classification is nontrivial — you genuinely can't simplify an $F_{\sigma\delta}$ set to an $F_\sigma$ set in general. The proofs use Baire category arguments and the theory of universal sets.

## 32.1.2 Analytic and Coanalytic Sets

Beyond the Borel sets lie the projective sets. The first level beyond Borel consists of the analytic and coanalytic sets.

**Definition 32.1.4.** A set $A \subseteq X$ is *analytic* ($\Sigma^1_1$) if it is the continuous image of a Borel set: $A = f(B)$ for $f$ continuous and $B$ Borel.

Equivalently, $A$ is analytic iff it is the projection of a Borel set in $X \times Y$.

**Definition 32.1.5.** A set is *coanalytic* ($\Pi^1_1$) if its complement is analytic.

The superscript 1 indicates "second-order" — we're taking projections of Borel sets, which involves a quantifier over real numbers (not just natural numbers). Analytic sets arise naturally in analysis: the set of continuous functions with a particular property is often analytic, even when it's not Borel.

**Theorem 32.1.6 (Luzin Separation Theorem).** Two disjoint analytic sets can be separated by a Borel set.

This is a deep structural result. You might expect that analytic sets, being more general than Borel sets, could be "too complicated" for Borel separation. But disjoint analytic sets can always be separated — which in particular shows that the complement of an analytic set, if it's also analytic, must be Borel.

**Theorem 32.1.7 (Luzin-Suslin).** Every Borel set is analytic. Not every analytic set is Borel (there exist $\Sigma^1_1$ sets that are not Borel).

The first statement is easy: Borel sets are "built" from open sets by countable operations, and projections preserve this complexity. The second statement — the existence of non-Borel analytic sets — is the key fact. It shows the hierarchy is genuinely longer than the Borel hierarchy. And it connects to the classification problems of Section 32.3: the isomorphism relation for ergodic systems is $\Sigma^1_1$-complete, meaning it's as complex as any analytic set.

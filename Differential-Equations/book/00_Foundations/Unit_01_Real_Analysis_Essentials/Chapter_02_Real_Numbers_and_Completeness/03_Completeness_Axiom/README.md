# The Completeness Axiom

The field axioms and order axioms together define an ordered field, a structure shared by both $\mathbb{Q}$ and $\mathbb{R}$. The single axiom that distinguishes the real numbers from the rationals — the axiom responsible for the "no holes" property of the real line — is the Completeness Axiom. It is the cornerstone of real analysis, and virtually every existence theorem in the subject depends on it, directly or indirectly.

## The Least Upper Bound Property

**Completeness Axiom (Least Upper Bound Property).** Every nonempty subset of $\mathbb{R}$ that is bounded above has a least upper bound in $\mathbb{R}$.

To unpack this: a **least upper bound** (or **supremum**) of a set $S$ is a number $\alpha$ satisfying:

1. $\alpha$ is an upper bound of $S$: $s \leq \alpha$ for all $s \in S$.
2. $\alpha$ is the smallest upper bound: if $M$ is any upper bound of $S$, then $\alpha \leq M$.

We write $\alpha = \sup S$.

**Example.** Let $S = \{q \in \mathbb{Q} : q^2 < 2\}$. This set is nonempty (it contains $0$) and bounded above (all elements are less than $2$). In $\mathbb{Q}$, there is no rational number that serves as a least upper bound: any candidate $r$ with $r^2 = 2$ is irrational. In $\mathbb{R}$, the Completeness Axiom guarantees the existence of $\sup S = \sqrt{2} \in \mathbb{R}$.

This is the precise sense in which $\mathbb{R}$ has "no holes": every set that should have a supremum does have one.

## Infimum

By an analogous construction, every nonempty subset bounded below has a **greatest lower bound** or **infimum**, written $\inf S$. This follows from the Completeness Axiom by considering the set $\{-s : s \in S\}$: its supremum negated gives $\inf S$.

**Definition.** $\beta = \inf S$ if:
1. $\beta \leq s$ for all $s \in S$ ($\beta$ is a lower bound).
2. If $m$ is any lower bound, then $m \leq \beta$ ($\beta$ is the greatest lower bound).

## The Characterization of Supremum

A number $\alpha$ is the supremum of $S$ if and only if:
1. For all $s \in S$, $s \leq \alpha$.
2. For every $\varepsilon > 0$, there exists $s \in S$ with $s > \alpha - \varepsilon$.

The second condition says that $\alpha$ cannot be "backed away from" — every number less than $\alpha$ is exceeded by some element of $S$. This is the key tool for using the supremum in proofs: to show that something is at least $\alpha$, find elements of $S$ arbitrarily close to $\alpha$.

**Example.** Let $S = (0, 1)$. Then $\sup S = 1$. Note that $1 \notin S$; the supremum need not belong to the set. The condition $s > 1 - \varepsilon$ is satisfied for any $\varepsilon \in (0,1)$ by taking, e.g., $s = 1 - \varepsilon/2 \in (0,1)$.

## Consequences of Completeness

**Theorem (Nested Interval Property).** Let $[a_n, b_n]$ be a sequence of closed bounded intervals with $[a_1, b_1] \supseteq [a_2, b_2] \supseteq \cdots$ (nested). Then $\bigcap_{n=1}^\infty [a_n, b_n] \neq \emptyset$. If additionally $b_n - a_n \to 0$, the intersection contains exactly one point.

*Proof sketch.* The set $\{a_n\}$ is increasing and bounded above by $b_1$; let $\alpha = \sup\{a_n\}$. Then $\alpha \in [a_n, b_n]$ for each $n$. If $b_n - a_n \to 0$, uniqueness follows. $\square$

This result is used to prove the Bolzano-Weierstrass theorem (every bounded sequence has a convergent subsequence) and hence many existence results.

**Theorem.** $\sqrt{2} \in \mathbb{R}$.

*Proof.* Let $S = \{x \in \mathbb{R} : x \geq 0,\ x^2 < 2\}$. Then $S$ is nonempty ($1 \in S$) and bounded above ($x \in S \Rightarrow x < 2$, since $x \geq 2$ would give $x^2 \geq 4 > 2$). Let $\alpha = \sup S$. We claim $\alpha^2 = 2$.

If $\alpha^2 < 2$: set $\varepsilon = (2 - \alpha^2)/(2\alpha + 1)$. Then $(\alpha + \varepsilon)^2 = \alpha^2 + 2\alpha\varepsilon + \varepsilon^2 \leq \alpha^2 + (2\alpha + 1)\varepsilon = 2$. So $\alpha + \varepsilon \in S$, contradicting $\alpha = \sup S$.

If $\alpha^2 > 2$: set $\varepsilon = (\alpha^2 - 2)/(2\alpha)$. Then $(\alpha - \varepsilon)^2 = \alpha^2 - 2\alpha\varepsilon + \varepsilon^2 > \alpha^2 - 2\alpha \cdot \frac{\alpha^2-2}{2\alpha} = 2$. So $\alpha - \varepsilon$ is an upper bound of $S$ smaller than $\alpha$, contradicting the "least" part of $\alpha = \sup S$.

Therefore $\alpha^2 = 2$. $\square$

This proof illustrates the standard two-step technique for using the supremum: show that the supremum cannot be too small (by exhibiting an element of $S$ larger than any presumed bound) and cannot be too large (by exhibiting a smaller upper bound).

## Failure of Completeness in $\mathbb{Q}$

The same argument in $\mathbb{Q}$ would construct $\alpha = \sup\{q \in \mathbb{Q} : q^2 < 2\}$, but this supremum does not exist in $\mathbb{Q}$: no rational satisfies $\alpha^2 = 2$. The Completeness Axiom explicitly rules out this failure for $\mathbb{R}$.

## Connection to Convergence

The Completeness Axiom is what makes limits well-defined. A bounded, monotone increasing sequence $a_1 \leq a_2 \leq a_3 \leq \cdots$ bounded above by $M$ has a natural candidate for its limit: $\alpha = \sup\{a_n : n \in \mathbb{N}\}$. The axiom guarantees $\alpha$ exists, and the characterization of the supremum allows one to prove $a_n \to \alpha$. This is the Monotone Convergence Theorem, treated in Chapter 3.

In differential equations, the completeness of $\mathbb{R}$ extends (through completeness of $C([a,b])$ in the sup-norm) to guarantee that Cauchy sequences of approximate solutions converge to true solutions. The Completeness Axiom is the reason iterative methods work.

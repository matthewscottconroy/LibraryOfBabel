# 1.3 Continuous Functions

Continuity is the central concept in topology, and it has a particularly clean formulation in metric spaces. We've already met continuous functions informally; now we need three increasingly strong versions: continuity, uniform continuity, and Lipschitz continuity. Each one imposes tighter control on how the function can vary.

## 1.3.1 Continuity and Uniform Continuity

**Definition 1.3.1.** A function $f: (X, d_X) \to (Y, d_Y)$ is *continuous at $x_0$* if for all $\varepsilon > 0$ there exists $\delta > 0$ such that $d_X(x, x_0) < \delta$ implies $d_Y(f(x), f(x_0)) < \varepsilon$.

$f$ is *continuous* if it is continuous at every point, and *uniformly continuous* if $\delta$ can be chosen independently of $x_0$.

The difference between continuity and uniform continuity is a matter of who gets to go first. In ordinary continuity, you first fix a point $x_0$, then choose $\delta$ — and $\delta$ is allowed to depend on both $\varepsilon$ *and* $x_0$. In uniform continuity, you choose $\delta$ based only on $\varepsilon$, and it has to work everywhere simultaneously. Uniform continuity is a global condition; continuity is local.

Compact spaces erase this distinction:

**Theorem 1.3.2.** A continuous function on a compact metric space is uniformly continuous.

This is a genuinely useful theorem. If you're working on a compact domain, you never have to worry about whether continuity is uniform — it automatically is.

Stronger than uniform continuity is the Lipschitz condition:

**Definition 1.3.3.** $f: X \to Y$ is *Lipschitz* with constant $L$ if $d_Y(f(x), f(x')) \leq L \cdot d_X(x, x')$ for all $x, x' \in X$.

Lipschitz continuity implies uniform continuity — just take $\delta = \varepsilon/L$. The Lipschitz constant of a $C^1$ map is bounded by the supremum of $\|Df\|$, so differentiable maps with bounded derivative are automatically Lipschitz. This observation is the bridge between calculus and metric space theory that we'll use constantly.

Lipschitz maps are the "nice" maps of dynamical systems theory. When a map is Lipschitz with constant $L < 1$, we call it a *contraction*, and the theory becomes even richer — see Section 1.4.

## 1.3.2 The Arzelà-Ascoli Theorem

Here's a question that comes up repeatedly in dynamics: we have a sequence of maps $f_n: X \to X$, and we want to extract a convergent subsequence. When can we do this?

In finite dimensions, a bounded sequence in $\mathbb{R}^n$ has a convergent subsequence — that's Bolzano-Weierstrass. But the space of continuous functions is infinite-dimensional, and boundedness alone isn't enough. We need an additional condition: *equicontinuity*.

**Definition 1.3.4.** A family $\mathcal{F}$ of functions $f: K \to \mathbb{R}$ (with $K$ compact) is *equicontinuous* at $x_0$ if for all $\varepsilon > 0$ there exists $\delta > 0$ such that $|x - x_0| < \delta$ implies $|f(x) - f(x_0)| < \varepsilon$ for all $f \in \mathcal{F}$ simultaneously.

$\mathcal{F}$ is *uniformly equicontinuous* if $\delta$ can be chosen independently of $x_0$.

Think of it this way: each individual function in the family is continuous, but equicontinuity says the *whole family* is continuous with the same modulus — no function in $\mathcal{F}$ gets to be "more oscillatory" than what $\delta$ controls.

**Theorem 1.3.5 (Arzelà-Ascoli).** Let $K$ be a compact metric space and $\mathcal{F} \subseteq C(K, \mathbb{R})$. Then $\mathcal{F}$ has compact closure in $(C(K, \mathbb{R}), d_\infty)$ if and only if $\mathcal{F}$ is pointwise bounded and equicontinuous.

*(proof sketch)* Given a sequence $(f_n)$ that is equicontinuous and pointwise bounded, choose a countable dense set $\{x_1, x_2, \ldots\} \subseteq K$ and apply a diagonal argument: first extract a subsequence converging at $x_1$, then a further subsequence converging at $x_2$, and so on. The diagonal subsequence converges pointwise on the dense set, and equicontinuity promotes this to uniform convergence.

This is one of those theorems where you want to sit with the proof strategy. The diagonal argument is a beautiful trick: you're building a subsequence that converges everywhere simultaneously by enforcing convergence at one point at a time. Equicontinuity is what lets you "fill in the gaps" between the countable dense set and get uniform convergence on all of $K$.

**Application in Dynamics.** Given a sequence of maps $f_n: X \to X$ with $\sup_n \text{Lip}(f_n) < \infty$ on a compact space, Arzelà-Ascoli extracts a uniformly convergent subsequence. This is the standard argument for constructing invariant measures and proving fixed-point theorems. We'll invoke it by name when it appears — and it will appear a lot.

This application is why equicontinuity matters for dynamics. Lipschitz bounds propagate under composition in a controlled way, so sequences of iterates of a Lipschitz map form an equicontinuous family. That's the pipeline: boundedness + equicontinuity → convergent subsequence → limit object (invariant measure, fixed function, invariant manifold).

The next section takes the other key property — completeness — and shows what it enables: a fixed-point theorem that is the backbone of existence proofs throughout mathematics.

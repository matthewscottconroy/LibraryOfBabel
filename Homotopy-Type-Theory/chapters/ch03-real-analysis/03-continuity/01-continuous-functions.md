# 3.1 Continuous Functions

## Three Equivalent Definitions

Continuity is the right notion of "structure-preserving map" for metric (and topological) spaces. There are three equivalent ways to define it, each highlighting a different aspect.

**Definition (ε-δ, pointwise).** A function $f : X \to Y$ between metric spaces is *continuous at $x \in X$* if:
$$\forall \varepsilon > 0,\, \exists \delta > 0,\, \forall x' \in X:\, d_X(x', x) < \delta \Rightarrow d_Y(f(x'), f(x)) < \varepsilon$$

$f$ is *continuous* if it is continuous at every point.

The ε-δ definition is the classical one. It says: inputs within distance $\delta$ of $x$ produce outputs within distance $\varepsilon$ of $f(x)$. To show $f$ is continuous at $x$, you must produce a $\delta$ for every $\varepsilon$.

**Definition (sequential).** $f$ is continuous at $x$ if: for every sequence $x_n \to x$ in $X$, we have $f(x_n) \to f(x)$ in $Y$.

This is often the easiest to use in practice: to check continuity, just check that sequences are preserved.

**Definition (topological/open sets).** $f : X \to Y$ is continuous if: for every open set $V \subseteq Y$, the preimage $f^{-1}(V) = \{x \in X \mid f(x) \in V\}$ is open in $X$.

This is the "right" definition from a categorical perspective: it refers only to open sets, not to the metric. It's the one that generalizes to topological spaces.

**Theorem.** All three definitions are equivalent (for maps between metric spaces).

*Proof sketch.* ε-δ $\Leftrightarrow$ sequential: If ε-δ holds and $x_n \to x$, given $\varepsilon > 0$ find $\delta$ for ε-δ, then find $N$ with $d(x_n, x) < \delta$ for $n \geq N$, giving $d(f(x_n), f(x)) < \varepsilon$. Conversely, if ε-δ fails for some $\varepsilon$, then for each $n$ there's $x_n$ with $d(x_n, x) < 1/n$ but $d(f(x_n), f(x)) \geq \varepsilon$. This $x_n \to x$ but $f(x_n) \not\to f(x)$.

ε-δ $\Leftrightarrow$ open sets: If $V \subseteq Y$ is open and $f(x) \in V$, find $\varepsilon$ with $B(f(x), \varepsilon) \subseteq V$. By ε-δ, find $\delta$ with $f(B(x, \delta)) \subseteq B(f(x), \varepsilon) \subseteq V$. So $B(x, \delta) \subseteq f^{-1}(V)$, making $f^{-1}(V)$ open. Conversely, given $\varepsilon > 0$, $B(f(x), \varepsilon)$ is open, so $f^{-1}(B(f(x), \varepsilon))$ is open. It contains $x$, so there's $\delta > 0$ with $B(x, \delta) \subseteq f^{-1}(B(f(x), \varepsilon))$, giving the ε-δ condition. $\square$

## Compositions and Properties

**Theorem.** The composition of continuous functions is continuous.

*Proof (using open sets).* Let $f : X \to Y$ and $g : Y \to Z$ be continuous. For any open $W \subseteq Z$:
$$(g \circ f)^{-1}(W) = f^{-1}(g^{-1}(W))$$
$g^{-1}(W)$ is open in $Y$ (since $g$ is continuous), and $f^{-1}$ of an open set is open in $X$ (since $f$ is continuous). $\square$

This is why the open-sets definition is "right": the composition property becomes completely transparent.

**Theorem.** Sums, products, quotients (when denominator is nonzero) of continuous real-valued functions are continuous.

These follow from the continuity of the arithmetic operations $+, \cdot, /$ on $\mathbb{R}$.

## Homeomorphisms

**Definition.** A *homeomorphism* is a continuous bijection $f : X \to Y$ whose inverse $f^{-1} : Y \to X$ is also continuous.

Two spaces are *homeomorphic* if there is a homeomorphism between them.

Warning: a continuous bijection need not have a continuous inverse. Consider $f : [0, 1) \to S^1$ defined by $f(t) = e^{2\pi i t}$ (winding the half-open interval onto the circle). This is a continuous bijection, but its inverse is not continuous at the point $1 \in S^1$.

This is why we need to separately require continuity of $f^{-1}$.

**Homeomorphism-invariant properties** (topological invariants): If $X \cong Y$ (homeomorphic), then:
- $X$ is compact iff $Y$ is compact
- $X$ is connected iff $Y$ is connected
- $X$ is separable iff $Y$ is separable
- $\pi_1(X) \cong \pi_1(Y)$ (fundamental groups are isomorphic)

These properties can be used to distinguish spaces: if two spaces differ in a topological invariant, they cannot be homeomorphic.

## Uniform Continuity

The ε-δ definition of continuity allows $\delta$ to depend on both $\varepsilon$ and $x$. A stronger notion requires $\delta$ to depend only on $\varepsilon$.

**Definition.** $f : X \to Y$ is *uniformly continuous* if:
$$\forall \varepsilon > 0,\, \exists \delta > 0,\, \forall x, x' \in X:\, d_X(x, x') < \delta \Rightarrow d_Y(f(x), f(x')) < \varepsilon$$

The same $\delta$ works for all points of $X$ simultaneously.

**Examples:**
- $f(x) = x^2$ is continuous on $\mathbb{R}$ but not uniformly continuous: near large $x$, small changes in $x$ produce large changes in $x^2$.
- $f(x) = x^2$ on $[0, 1]$ is uniformly continuous: the interval is bounded, so the function can't grow too fast.
- $f(x) = \sin(x)$ is uniformly continuous on $\mathbb{R}$ (its derivative is bounded by 1).

**Theorem.** A continuous function on a compact metric space is uniformly continuous.

We'll prove this in the compactness section. The key point: compactness prevents the "local behavior near $x$" from varying too wildly across the space.

**Theorem.** A uniformly continuous function maps Cauchy sequences to Cauchy sequences.

*Proof.* If $(x_n)$ is Cauchy and $\varepsilon > 0$, find $\delta$ for uniform continuity with $\varepsilon$, then find $N$ with $d(x_m, x_n) < \delta$ for $m, n \geq N$. Then $d(f(x_m), f(x_n)) < \varepsilon$. $\square$

This is important for the completion: a uniformly continuous function $f : X \to Y$ (where $Y$ is complete) extends uniquely to a continuous function $\hat{f} : \hat{X} \to Y$ on the completion.

## Lipschitz Maps and Contractions

**Definition.** $f : X \to Y$ is *Lipschitz* (with constant $L$) if:
$$d_Y(f(x), f(x')) \leq L \cdot d_X(x, x') \quad \text{for all } x, x' \in X$$

Lipschitz maps multiply distances by at most $L$. A contraction is a Lipschitz map with $L < 1$.

Lipschitz $\Rightarrow$ uniformly continuous $\Rightarrow$ continuous, but not vice versa.

**Example:** $f(x) = \sqrt{x}$ on $[0, 1]$ is uniformly continuous but not Lipschitz (the derivative $1/(2\sqrt{x})$ is unbounded near $0$).

**Example:** $f(x) = x^2$ on $[0, 1]$ is Lipschitz: $|x^2 - y^2| = |x+y||x-y| \leq 2|x-y|$.

## Continuity and the Category Structure

From a categorical perspective, metric spaces with continuous maps form a category $\mathbf{Met}$:
- Objects: metric spaces
- Morphisms: continuous maps
- Identity: $\text{id}_X$
- Composition: function composition

Homeomorphisms are the *isomorphisms* in this category — the maps with inverses in the category.

This categorical viewpoint will be central when we study topology abstractly. The key question of topology is: which properties of spaces are preserved by continuous maps? Which are preserved by homeomorphisms? Which are preserved only by isometries?

## Continuity and Proofs in HoTT

The connection between continuity and identity types in Homotopy Type Theory is one of the deepest motivations for our whole curriculum.

In HoTT, the *identity type* $\text{Id}_A(a, b)$ (the type of proofs that $a = b$) behaves like the space of *paths* from $a$ to $b$ in a topological space. A function $f : A \to B$ automatically takes paths to paths — it's a continuous map in a sense that's built into the type theory.

More precisely: in HoTT, every function $f : A \to B$ is automatically *homotopy-continuous*: if $p : \text{Id}_A(a, b)$ is a proof that $a = b$, then $\text{ap}_f(p) : \text{Id}_B(f(a), f(b))$ is a proof that $f(a) = f(b)$. This is the type-theoretic version of "continuous maps preserve paths."

The topological intuition is that types are spaces, terms are points, and identity proofs are paths. In this picture, all functions in type theory are continuous by construction — there's no way to define a "discontinuous" function. This is a profound difference from set theory, where arbitrary functions can be highly discontinuous.

We'll develop this connection much more carefully when we study identity types in Chapter 16 and Univalence in Chapter 18.

# 1.1 Homotopy Between Maps and Homotopy Equivalences

## Homotopy as Continuous Deformation

Two maps $f, g : X \to Y$ are *homotopic* if you can continuously deform one into the other. The key idea: a "deformation" of $f$ into $g$ is a one-parameter family of maps — a map $H : X \times [0,1] \to Y$ that starts at $f$ (when $t=0$) and ends at $g$ (when $t=1$).

**Definition 1.1 (Homotopy).** A *homotopy* from $f$ to $g$ is a continuous map $H : X \times [0,1] \to Y$ with:
$$H(x, 0) = f(x) \quad \text{and} \quad H(x, 1) = g(x) \quad \text{for all } x \in X$$

We write $f \simeq g$ if such an $H$ exists, and call $f$ and $g$ *homotopic*.

**Interpretation:** Think of $t \in [0,1]$ as time. The map $H_t : X \to Y$ defined by $H_t(x) = H(x, t)$ is a "snapshot" of the deformation at time $t$. At $t=0$, we have $H_0 = f$; at $t=1$, we have $H_1 = g$. In between, $H_t$ is some intermediate map. The continuity of $H$ ensures the deformation is "smooth" (no jumps).

**Homotopy is an equivalence relation:**
- *Reflexivity:* $f \simeq f$ via the constant homotopy $H(x,t) = f(x)$
- *Symmetry:* If $H$ is a homotopy from $f$ to $g$, then $H'(x,t) = H(x, 1-t)$ is a homotopy from $g$ to $f$
- *Transitivity:* If $H$ goes from $f$ to $g$ and $K$ goes from $g$ to $h$, concatenate them:
$$L(x,t) = \begin{cases} H(x, 2t) & 0 \leq t \leq 1/2 \\ K(x, 2t-1) & 1/2 \leq t \leq 1 \end{cases}$$

The equivalence classes $[f] = \{g : X \to Y \mid g \simeq f\}$ are *homotopy classes of maps*. The set of homotopy classes is written $[X, Y]$.

## Examples

**Straight-line homotopy.** For maps $f, g : X \to \mathbb{R}^n$ (or any convex subset), the straight-line homotopy $H(x,t) = (1-t)f(x) + tg(x)$ is a homotopy from $f$ to $g$. So all continuous maps to $\mathbb{R}^n$ are homotopic to each other: $[X, \mathbb{R}^n]$ is a single homotopy class.

**Null-homotopic maps.** A map $f : X \to Y$ is *null-homotopic* if it's homotopic to a constant map $c_{y_0} : X \to Y$ (sending everything to a single point $y_0$). Null-homotopic means: $f$ can be continuously contracted to a point.

**Maps $X \to S^1$.** Not all maps to the circle are homotopic. The homotopy class of $f : S^1 \to S^1$ is determined by the *degree* (winding number) — how many times $f$ wraps the circle around itself. Degree $n$ and degree $m$ maps are homotopic iff $n = m$.

## Homotopy Equivalence

**Definition 1.2 (Homotopy Equivalence).** A continuous map $f : X \to Y$ is a *homotopy equivalence* if there exists a continuous map $g : Y \to X$ (a *homotopy inverse*) with:
$$g \circ f \simeq \mathsf{id}_X \quad \text{and} \quad f \circ g \simeq \mathsf{id}_Y$$

We write $X \simeq Y$ and say $X$ and $Y$ are *homotopy equivalent* or have the *same homotopy type*.

Homotopy equivalence is an equivalence relation on topological spaces (reflexive, symmetric, transitive). The equivalence class of $X$ is its *homotopy type*.

**Note the difference from homeomorphism:** A homeomorphism requires $g \circ f = \mathsf{id}_X$ and $f \circ g = \mathsf{id}_Y$ (on the nose). A homotopy equivalence only requires these to hold up to homotopy. Homotopy equivalence is coarser: every homeomorphism is a homotopy equivalence, but not vice versa.

## Key Examples

**Contractible spaces.** A space $X$ is *contractible* if it's homotopy equivalent to a single point:
$$X \simeq \{*\}$$

This means: there's a continuous map $H : X \times [0,1] \to X$ with $H(x,0) = x$ and $H(x,1) = x_0$ for some fixed $x_0 \in X$. The space can be "contracted" to a single point.

Examples:
- $\mathbb{R}^n$: contract via $H(x,t) = (1-t)x$
- Any convex subset of $\mathbb{R}^n$: same straight-line contraction
- The cone $CX = X \times [0,1] / (X \times \{1\})$: contract to the cone point
- The path space $PX$ (all paths starting at $x_0$): contract each path to the constant path

Non-examples:
- $S^n$ (for $n \geq 1$): $\pi_n(S^n) = \mathbb{Z} \neq 0$, so $S^n$ is not contractible
- Any space with $\pi_1 \neq 0$: a non-trivial fundamental group means loops can't all be contracted

**Deformation retracts.** A *deformation retract* of $X$ onto $A \subseteq X$ is a homotopy $H : X \times [0,1] \to X$ with:
- $H(x, 0) = x$ (starts as identity)
- $H(x, 1) \in A$ (ends in $A$)
- $H(a, t) = a$ for all $a \in A$ (fixes $A$ throughout)

If $A$ is a deformation retract of $X$, then $A \hookrightarrow X$ is a homotopy equivalence.

**Example:** $S^1$ is a deformation retract of $\mathbb{R}^2 \setminus \{0\}$ (punctured plane). The deformation is "radial projection": $H(x, t) = (1-t)x + t \cdot x/|x|$. This shows $\mathbb{R}^2 \setminus \{0\} \simeq S^1$ — the punctured plane has the same homotopy type as the circle.

**Graphs.** A connected graph $\Gamma$ is homotopy equivalent to a wedge of circles $S^1 \vee \cdots \vee S^1$ (a "rose"). The number of circles is $e - v + 1$ where $e$ = edges, $v$ = vertices. This is because you can collapse a spanning tree (which is contractible) and each remaining edge becomes a circle.

## Homotopy Invariants

A property $P$ of topological spaces is a *homotopy invariant* if $X \simeq Y$ implies $P(X) \iff P(Y)$.

Examples of homotopy invariants:
- Connectedness, path-connectedness
- $\pi_n(X)$ for all $n \geq 0$
- Homology groups $H_n(X)$
- Cohomology rings $H^*(X; R)$
- Being contractible

Examples that are NOT homotopy invariants:
- Being a manifold (of a specific dimension)
- Being Hausdorff
- Cardinality of the point set
- Being compact (can deform a non-compact space to compact and vice versa)

## The Homotopy Category

The *homotopy category* $\mathsf{hTop}$ (or $\mathsf{Ho}(\mathbf{Top})$) is the category where:
- Objects: topological spaces
- Morphisms: homotopy classes of continuous maps

This is obtained from **Top** by "inverting" the homotopy equivalences. In homotopy theory, we work in $\mathsf{hTop}$: we identify maps up to homotopy and spaces up to homotopy equivalence.

**Warning:** $\mathsf{hTop}$ is not very well-behaved as a category. It has limits and colimits that don't match those in **Top** (products in **Top** are not products in $\mathsf{hTop}$). This is why modern homotopy theory uses the richer structure of model categories or (∞,1)-categories — they remember the homotopies, not just their existence.

## Homotopy in HoTT

In HoTT, homotopy has a direct type-theoretic formulation:

**Homotopy between terms:** A *homotopy* from $f : A \to B$ to $g : A \to B$ is a term of type $f \sim g := \prod_{a:A} f(a) = g(a)$ — a family of paths witnessing that $f$ and $g$ agree pointwise.

**Homotopy equivalence:** A function $f : A \to B$ is a *homotopy equivalence* (or *equivalence of types*) if there exists $g : B \to A$ with:
- $\epsilon : f \circ g \sim \mathsf{id}_B$
- $\eta : g \circ f \sim \mathsf{id}_A$

The type of equivalences from $A$ to $B$ is $A \simeq B$.

**Contractible types:** A type $A$ is *contractible* if $\mathsf{isContr}(A) := \sum_{a:A} \prod_{b:A} (a = b)$ — there's a center of contraction $a$ and all other points are path-connected to $a$.

All of this mirrors exactly the classical homotopy theory, but expressed purely in type theory. The topological intuition is essential for understanding what's going on.

## Summary

| Classical | HoTT |
|---|---|
| Homotopy $H : f \simeq g$ | Term of $\prod_a f(a) = g(a)$ |
| Homotopy equivalence $X \simeq Y$ | Equivalence $A \simeq B$ |
| Contractible space | Contractible type |
| Deformation retract | Section of a fibration |
| Homotopy class $[f]$ | Propositional truncation $\|f\|$ |

Homotopy equivalence is the "right" notion of sameness for homotopy theory — and for HoTT. The classical theory tells us how to think about it geometrically; the type theory gives us the formal language to reason about it.

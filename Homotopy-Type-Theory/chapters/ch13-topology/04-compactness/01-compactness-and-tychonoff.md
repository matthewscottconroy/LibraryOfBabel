# 4.1 Compactness

## The Intuitive Idea

Compactness is the key "finiteness" condition in topology. A compact space behaves like a finite set in many respects: continuous functions on compact spaces are bounded, attain their maximum, are uniformly continuous; closed subsets of compact spaces are compact; and so on.

The definition is technical, but the intuition is: a compact space has no "escaping to infinity" behavior.

**Definition 4.1 (Compact Space).** A topological space $X$ is *compact* if every open cover of $X$ has a finite subcover.

An *open cover* is a collection $\{U_\alpha\}$ of open sets with $\bigcup_\alpha U_\alpha = X$. A *finite subcover* is a finite subcollection $U_{\alpha_1}, \ldots, U_{\alpha_n}$ that still covers: $X = U_{\alpha_1} \cup \cdots \cup U_{\alpha_n}$.

## The Heine-Borel Theorem

For $\mathbb{R}^n$, compactness has a concrete characterization:

**Theorem 4.2 (Heine-Borel).** A subset $K \subseteq \mathbb{R}^n$ is compact iff it is closed and bounded.

*Proof sketch (for $\mathbb{R}$).* 
- Closed and bounded $\Rightarrow$ compact: Any bounded subset is contained in $[-M, M]$ for some $M$. By the Bolzano-Weierstrass theorem, any sequence in $[-M, M]$ has a convergent subsequence. This is the sequential version of compactness, which for metric spaces is equivalent to the open cover definition.
- Compact $\Rightarrow$ closed: In a Hausdorff space, compact subsets are closed. (Proof: for any $y \notin K$, separate $y$ from $K$ using open sets; their intersection gives an open neighborhood of $y$ missing $K$.)
- Compact $\Rightarrow$ bounded: The open cover $\{B(0, n) : n \in \mathbb{N}\}$ of $\mathbb{R}^n$ has a finite subcover when restricted to $K$, so $K$ is contained in some $B(0, N)$. $\square$

**Examples of compact spaces:**
- $[0,1]$ and any closed bounded interval $[a,b]$
- $S^n$ (the $n$-sphere)
- $[0,1]^n$ (the $n$-cube)
- Any finite topological space
- The Cantor set $C \subseteq [0,1]$

**Examples of non-compact spaces:**
- $\mathbb{R}$ (unbounded, or cover by open intervals of length 1 with no finite subcover)
- $(0,1)$ (open, so not closed in $\mathbb{R}$; the cover $\{(1/n, 1) : n \geq 2\}$ has no finite subcover)
- $\mathbb{Z}$ with discrete topology (each integer is isolated; the cover by singletons has no finite subcover)

## Key Theorems About Compact Spaces

**Theorem 4.3.** The continuous image of a compact space is compact.

*Proof.* Let $f : X \to Y$ be continuous with $X$ compact. Take any open cover $\{V_\alpha\}$ of $f(X)$. Then $\{f^{-1}(V_\alpha)\}$ is an open cover of $X$. Extract a finite subcover $\{f^{-1}(V_{\alpha_1}), \ldots, f^{-1}(V_{\alpha_n})\}$. Then $\{V_{\alpha_1}, \ldots, V_{\alpha_n}\}$ covers $f(X)$. $\square$

**Corollary 4.4.** If $f : X \to \mathbb{R}$ is continuous and $X$ is compact, then $f$ is bounded and attains its maximum and minimum.

This is the extreme value theorem.

**Theorem 4.5.** A closed subspace of a compact space is compact.

**Theorem 4.6.** In a Hausdorff space, every compact subset is closed.

Together: in a Hausdorff space, the compact subsets are exactly the closed subsets of compact subspaces.

**Theorem 4.7.** If $f : X \to Y$ is continuous and bijective, $X$ is compact, and $Y$ is Hausdorff, then $f$ is a homeomorphism.

*Proof.* It suffices to show $f^{-1}$ is continuous, i.e., $f$ is a closed map. Let $C \subseteq X$ be closed. Then $C$ is compact (closed subspace of compact $X$). Then $f(C)$ is compact (image of compact). Then $f(C)$ is closed (compact subset of Hausdorff $Y$). $\square$

This theorem is extremely useful: once you have a continuous bijection from compact to Hausdorff, you're done.

## Tychonoff's Theorem

**Theorem 4.8 (Tychonoff).** The product of any collection of compact spaces is compact (with the product topology).

$$\prod_{\alpha \in I} X_\alpha \text{ is compact if each } X_\alpha \text{ is compact}$$

For finite products, this follows easily from the tube lemma. For infinite products, the proof requires the Axiom of Choice (or equivalently, Zorn's lemma).

*Historical note:* Tychonoff's theorem is actually *equivalent* to the Axiom of Choice — it's one of the most prominent examples of a "non-constructive" theorem in general topology.

**Corollary 4.9.** The Hilbert cube $[0,1]^\omega$ (countably infinite product of copies of $[0,1]$) is compact.

**Corollary 4.10.** The Cantor space $\{0,1\}^\omega$ (product of countably many copies of $\{0,1\}$) is compact.

## Compactness and the Finite Intersection Property

There's an equivalent reformulation of compactness in terms of closed sets:

**Theorem 4.11.** $X$ is compact iff every collection of closed sets with the *finite intersection property* has non-empty intersection.

The *finite intersection property* (FIP): every finite subcollection has non-empty intersection.

This reformulation is useful in certain proofs (like Tychonoff's theorem via ultrafilters).

## Uniform Continuity and Compactness

**Theorem 4.12 (Heine-Cantor).** A continuous function from a compact metric space to any metric space is uniformly continuous.

*Proof sketch.* For each $x$, the preimage of $B(f(x), \varepsilon/2)$ is open and contains $x$, so contains $B(x, \delta_x)$ for some $\delta_x$. The cover $\{B(x, \delta_x/2)\}$ has a finite subcover. Take $\delta$ to be half the minimum of finitely many $\delta_{x_i}/2$. $\square$

This is why $\sin$ is uniformly continuous on $[0, 2\pi]$ but not on all of $\mathbb{R}$.

## Local Compactness

Many natural spaces aren't compact but are "locally compact" — compact near every point.

**Definition 4.13.** A space $X$ is *locally compact* if every point has a neighborhood with compact closure.

Examples:
- $\mathbb{R}^n$ is locally compact (every point has a neighborhood contained in a closed ball)
- All manifolds are locally compact
- $\mathbb{Q}$ is not locally compact (no neighborhood of any rational has compact closure)

**One-point compactification.** Given a locally compact Hausdorff space $X$, the *one-point compactification* $X^+ = X \cup \{\infty\}$ is compact: add one point "$\infty$" with neighborhoods that are complements of compact subsets of $X$.

The one-point compactification of $\mathbb{R}^n$ is $S^n$. This gives the homeomorphism $S^n \cong \mathbb{R}^n \cup \{\infty\}$ (stereographic projection).

## Compactness in HoTT

Compactness doesn't have a direct analog in HoTT in its classical form, because HoTT is constructive and Tychonoff's theorem requires Choice. However:

**Compact types in constructive mathematics.** There are several constructive notions:
- *Bishop-compactness*: every uniform continuous function attains its infimum
- *Overtness*: a dual notion to compactness (related to the open/closed duality)
- *Searchable types*: a predicate on a compact type can be decided by searching

In HoTT, compactness is discussed in terms of *finite types* (types equivalent to Fin$(n)$ for some $n$) and their propositional truncations. The key finite-type analog: every function out of a finite type factors through a finite discretization.

The deeper connection: the classifying space of compact groups (like $\mathbb{T} = S^1$ or $O(n)$) plays a role in HoTT through the theory of bundles and cohomology.

## Summary

| Theorem | Content |
|---|---|
| Heine-Borel | Compact in $\mathbb{R}^n$ ↔ closed and bounded |
| Extreme Value | Continuous functions on compact spaces attain max/min |
| Closed subsets | Closed subset of compact is compact |
| Continuous image | Continuous image of compact is compact |
| Compact Hausdorff | Compact subset of Hausdorff is closed |
| Continuous bijection | Compact → Hausdorff bijection is homeomorphism |
| Tychonoff | Product of compacts is compact |

Compactness is a crucial tool for converting local (pointwise) properties into global ones. It's the key finiteness condition in topology, and it underpins many of the most powerful theorems (IVT, EVT, uniform continuity, Arzelà-Ascoli, and more).

# Compactness

## What Compactness Is For

Compactness is the topological generalization of finiteness. A finite set has the property that any cover of it by open sets has a finite subcover — trivially, because the whole cover is finite. Compactness asks for the same thing from infinite sets: you start with an arbitrary (potentially infinite or uncountable) open cover, and you demand that a finite subcollection still covers.

Why would you want this? Because the most important theorems of analysis — continuity implies uniform continuity on $[0,1]$, a continuous function on $[0,1]$ attains its maximum and minimum, Heine-Cantor theorem, Arzelà-Ascoli theorem — all follow from compactness. Compactness is the condition that makes infinite sets "behave like finite sets" for the purposes of these covering arguments.

## The Definition

**Definition.** A topological space $X$ is *compact* if every open cover of $X$ has a finite subcover: whenever $\{U_\alpha\}_{\alpha \in I}$ is a collection of open sets with $X = \bigcup_{\alpha \in I} U_\alpha$, there exists a finite subset $J \subseteq I$ with $X = \bigcup_{\alpha \in J} U_\alpha$.

A subset $K \subseteq X$ is compact if it is compact as a topological space in the subspace topology, equivalently, if every cover of $K$ by open sets of $X$ has a finite subcover of $K$.

The definition looks technical, but it encodes a profound fact: you can always "compress" an infinite covering argument down to a finite one. This is what makes compact spaces tractable.

## The Heine-Borel Theorem

The most important examples of compact spaces in classical analysis are the closed and bounded subsets of $\mathbb{R}^n$.

**Theorem (Heine-Borel).** A subset $K \subseteq \mathbb{R}^n$ is compact if and only if it is closed and bounded.

*Proof sketch.* The key step is compactness of $[0,1]$ (and by product, of any closed box in $\mathbb{R}^n$). For $[0,1]$: given an open cover, let $s = \sup\{x \in [0,1] : [0,x] \text{ has a finite subcover}\}$. The point $s$ is covered by some $U_\alpha$, so $[0, s + \varepsilon] \subseteq U_\alpha$ for small $\varepsilon > 0$. If $s < 1$, this extends the finite subcover past $s$, contradicting the supremum. So $s = 1$ and $[0,1]$ has a finite subcover. Closed subsets of compact spaces are compact; bounded sets embed in a compact box.

Conversely, non-closed sets are not compact (the sequence $1/n$ in $(0,1]$ has no cluster point in $(0,1]$), and non-bounded sets are not compact (the cover by intervals $(-n,n)$ has no finite subcover of an unbounded set).

## Compactness is Preserved by Continuous Maps

**Theorem.** The continuous image of a compact space is compact.

*Proof.* Let $f : X \to Y$ be continuous with $X$ compact. Let $\{V_\alpha\}$ cover $f(X)$. Then $\{f^{-1}(V_\alpha)\}$ is an open cover of $X$ (since $f$ is continuous). Extract a finite subcover $\{f^{-1}(V_{\alpha_1}), \ldots, f^{-1}(V_{\alpha_n})\}$. Then $\{V_{\alpha_1}, \ldots, V_{\alpha_n}\}$ covers $f(X)$.

**Corollary.** A continuous function on a compact space to $\mathbb{R}$ is bounded and attains its maximum and minimum.

*Proof.* The image is a compact subset of $\mathbb{R}$, hence closed and bounded. A closed bounded subset of $\mathbb{R}$ contains its supremum and infimum.

This corollary is the backbone of optimization in analysis. Every continuous function on a compact domain has a maximum and minimum — a statement that fails dramatically for non-compact domains (e.g., $f(x) = x$ on $\mathbb{R}$ attains no maximum).

## Compactness in Metric Spaces

In metric spaces, compactness has equivalent characterizations:

**Theorem.** For a metric space $(X, d)$, the following are equivalent:
1. $X$ is compact (every open cover has a finite subcover).
2. $X$ is *limit point compact*: every infinite subset has a limit point.
3. $X$ is *sequentially compact*: every sequence has a convergent subsequence.
4. $X$ is complete and *totally bounded*: for every $\varepsilon > 0$, $X$ can be covered by finitely many balls of radius $\varepsilon$.

These equivalences all fail for general topological spaces. Limit point compactness does not imply compactness without the Hausdorff condition; sequential compactness does not imply compactness for spaces that are not first-countable. The metric case is special.

## Tychonoff's Theorem

The most powerful theorem about compactness — and the one with the most significant foundational implications — is Tychonoff's theorem.

**Theorem (Tychonoff, 1930).** The product of any collection of compact spaces is compact.

For finite products this is elementary (a finite product of open covers can be combined by taking all finite subcoverings simultaneously). For infinite products it is a deep theorem, and its proof requires the axiom of choice.

*Proof idea via ultrafilters.* An *ultrafilter* on a set $I$ is a maximal proper filter — a collection of subsets of $I$ satisfying certain closure properties. By Zorn's lemma (equivalent to the axiom of choice), every filter extends to an ultrafilter. Now suppose $X = \prod_{\alpha \in I} X_\alpha$ with each $X_\alpha$ compact. An ultrafilter $\mathcal{U}$ on a set $S \subseteq X$ determines, for each $\alpha$, an ultrafilter on $X_\alpha$ (the image under $\pi_\alpha$). By compactness of each $X_\alpha$, this ultrafilter converges to some point $x_\alpha \in X_\alpha$. The point $(x_\alpha)_{\alpha \in I} \in X$ is a cluster point of $S$ with respect to $\mathcal{U}$. This shows every infinite subset of $X$ has a cluster point, which implies compactness.

Alternatively, Alexander's sub-basis theorem reduces the problem to sub-basic covers; the Tychonoff product topology is defined precisely so that sub-basic open sets are preimages of open sets in a single factor, making the sub-basic covering argument direct.

## Compactness and the Axiom of Choice

The equivalence of the following statements is a theorem of set theory:

- Tychonoff's theorem (for arbitrary index sets).
- The axiom of choice.

That is: Tychonoff's theorem is as strong as the axiom of choice. In set theories without choice, Tychonoff's theorem fails for infinite products of spaces with more than one point.

This matters for HoTT because HoTT is constructive: the axiom of choice is not a theorem, but rather an additional axiom that may or may not be assumed. Tychonoff's theorem therefore requires care in the constructive setting. In predicative HoTT (without excluded middle or choice), compactness must be handled with care: Bishop-style compactness (total boundedness plus completeness) is used for metric spaces, and the infinite Tychonoff theorem requires explicit foundations.

## Compact Hausdorff Spaces

Many of the best theorems about compact spaces require the Hausdorff condition (distinct points have disjoint neighborhoods). Together, compactness and the Hausdorff condition are very powerful:

- A compact Hausdorff space is normal: disjoint closed sets have disjoint open neighborhoods (Urysohn).
- Every closed subset of a compact Hausdorff space is compact.
- A compact subset of a Hausdorff space is closed.
- A continuous bijection from a compact space to a Hausdorff space is a homeomorphism.

These results make the category of compact Hausdorff spaces particularly well-behaved. The Stone-Čech compactification embeds every completely regular space into a compact Hausdorff space universally.

## The One-Point Compactification

Every locally compact Hausdorff space $X$ has a canonical compactification: the *one-point compactification* $X^+ = X \cup \{\infty\}$, where the open sets are the open sets of $X$ together with sets of the form $(X \setminus K) \cup \{\infty\}$ for $K$ compact in $X$.

Examples:
- $\mathbb{R}^+ \cong S^1$ (the one-point compactification of the real line is the circle).
- $(\mathbb{R}^n)^+ \cong S^n$ (one-point compactification of $\mathbb{R}^n$ is the $n$-sphere).

This last example is crucial for homotopy theory: the $n$-sphere arises naturally as the "compactified $n$-space." The suspension $\Sigma X = X * S^0$ is related: $\Sigma X$ is the union of two cones on $X$. For $X = S^{n-1}$, $\Sigma S^{n-1} = S^n$.

## Compactness in HoTT

Compactness in the constructive setting used by HoTT is more nuanced. The open-cover definition requires working with predicative open covers, and the power-set operations needed to state "every cover has a finite subcover" are set-theoretically sensitive.

In the context of HoTT, the relevant notion is typically *compact types* in the sense that certain dependent choice principles hold. The circle $S^1$ is a compact type (as a HIT), and this compactness is used in proofs of $\pi_1(S^1) = \mathbb{Z}$. The compactness argument in that proof is synthetic — it proceeds through the encode-decode method rather than through open covers — but it plays the same role as compactness does in the classical proof.

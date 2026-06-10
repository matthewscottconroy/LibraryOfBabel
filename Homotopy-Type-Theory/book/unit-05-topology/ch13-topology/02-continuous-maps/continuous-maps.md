# Continuous Maps

## The Preimage Definition

A function $f : X \to Y$ between topological spaces is *continuous* if the preimage of every open set is open: for every $V \in \tau_Y$, we have $f^{-1}(V) \in \tau_X$.

This is the right definition. It captures the intuition that $f$ does not "tear" the space: points that are close together in $X$ (in the sense of lying in small open sets) map to points that are close together in $Y$. More precisely: if you want to control where $f(x)$ lands — to say that $f(x)$ lies in an open set $V$ around $f(x)$ — you can do so by restricting $x$ to the open set $f^{-1}(V)$ around $x$.

Let us verify this matches the $\varepsilon$-$\delta$ definition for metric spaces. Say $f : (X, d_X) \to (Y, d_Y)$ is continuous in the $\varepsilon$-$\delta$ sense: for every $x \in X$ and $\varepsilon > 0$, there exists $\delta > 0$ with $d_X(x, x') < \delta$ implying $d_Y(f(x), f(x')) < \varepsilon$. Now let $V \subseteq Y$ be open. We want $f^{-1}(V)$ to be open. Take any $x \in f^{-1}(V)$, so $f(x) \in V$. Since $V$ is open, there exists $\varepsilon > 0$ with $B(f(x), \varepsilon) \subseteq V$. By $\varepsilon$-$\delta$ continuity, there exists $\delta > 0$ with $B(x, \delta) \subseteq f^{-1}(B(f(x), \varepsilon)) \subseteq f^{-1}(V)$. So $f^{-1}(V)$ is open. The converse direction is equally straightforward.

## Homeomorphisms

A *homeomorphism* is a continuous bijection $f : X \to Y$ whose inverse $f^{-1} : Y \to X$ is also continuous. Two spaces are *homeomorphic*, written $X \cong Y$, if there exists a homeomorphism between them.

Homeomorphism is the isomorphism in the category **Top** of topological spaces and continuous maps. Homeomorphic spaces are "the same topological space" — they have the same open sets, the same closed sets, the same convergent sequences, the same continuous functions, and the same topological properties.

Warning: continuity plus bijectivity does not imply homeomorphism. The map $f : [0, 1) \to S^1$ defined by $f(t) = (\cos(2\pi t), \sin(2\pi t))$ is a continuous bijection but not a homeomorphism: the inverse is not continuous at the point $(1, 0)$. You need compactness of the domain or some other condition to force the inverse to be continuous.

**Theorem (Compact-to-Hausdorff).** A continuous bijection $f : X \to Y$ where $X$ is compact and $Y$ is Hausdorff is a homeomorphism.

*Proof.* Closed subsets of a compact space are compact; their images under $f$ are compact in the Hausdorff space $Y$, hence closed. So $f$ maps closed sets to closed sets, which means $f^{-1}$ maps open sets to open sets, which means $f^{-1}$ is continuous.

## Topological Invariants

A *topological invariant* is a property of a topological space that is preserved by homeomorphism: if $X \cong Y$ and $X$ has the property, then so does $Y$. Topological invariants are the tools for distinguishing spaces — to show $X \not\cong Y$, find an invariant that $X$ has and $Y$ does not.

Examples of topological invariants: compactness, connectedness, path-connectedness, the number of connected components, being Hausdorff, being metrizable, being a manifold of dimension $n$. The fundamental group $\pi_1(X, x_0)$ is a topological invariant (in fact a homotopy invariant — preserved by the coarser notion of homotopy equivalence). So are all homology and cohomology groups.

The fundamental question of classical topology is: which topological invariants completely classify spaces up to homeomorphism? For compact surfaces, the answer is known: the classification theorem states that every compact connected surface is homeomorphic to either the sphere $S^2$, a connected sum of tori $T^2 \# \cdots \# T^2$, or a connected sum of projective planes $\mathbb{RP}^2 \# \cdots \# \mathbb{RP}^2$, and these are all distinct. For higher-dimensional manifolds, the classification is vastly more complicated and largely open.

## The Category Top

Topological spaces and continuous maps form a category **Top**:
- Objects: topological spaces $(X, \tau)$.
- Morphisms: continuous functions $f : X \to Y$.
- Composition: function composition (which preserves continuity: if $f : X \to Y$ and $g : Y \to Z$ are continuous, then for open $W \subseteq Z$, we have $(g \circ f)^{-1}(W) = f^{-1}(g^{-1}(W))$, which is open).
- Identity: the identity function, which is continuous.

**Top** has all limits and colimits. The categorical product is the product topology; the categorical coproduct is the disjoint union (topological sum); the categorical equalizer is the subspace topology on $\{x : f(x) = g(x)\}$; the categorical pushout is the quotient of the disjoint union by the gluing relation. These categorical constructions are the building blocks of all topology.

## The Pasting Lemma

When you want to define a continuous function by cases — say $f : X \to Y$ with $f(x) = g(x)$ for $x \in A$ and $f(x) = h(x)$ for $x \in B$ — you need to know that the two pieces glue continuously.

**Pasting Lemma (Closed Case).** If $X = A \cup B$ with $A$ and $B$ closed, and if $g : A \to Y$ and $h : B \to Y$ are continuous and agree on $A \cap B$, then the function $f : X \to Y$ defined by $f(x) = g(x)$ for $x \in A$ and $f(x) = h(x)$ for $x \in B$ is continuous.

*Proof.* Let $C \subseteq Y$ be closed. Then $f^{-1}(C) = g^{-1}(C) \cup h^{-1}(C)$. Since $g$ is continuous and $A$ is closed, $g^{-1}(C)$ is closed in $A$, hence closed in $X$. Similarly $h^{-1}(C)$ is closed in $X$. The union of two closed sets is closed.

**Pasting Lemma (Open Case).** If $X = \bigcup_\alpha U_\alpha$ is an open cover, and $f_\alpha : U_\alpha \to Y$ are continuous functions that agree on overlaps, then the induced $f : X \to Y$ is continuous.

The pasting lemma is fundamental for constructing homotopies, which are defined piecewise: to concatenate two paths, you paste together two continuous functions on $[0, 1/2]$ and $[1/2, 1]$.

## Open and Closed Maps

Beyond continuous maps, two special classes deserve attention:

A map $f : X \to Y$ is *open* if the image of every open set is open. A map is *closed* if the image of every closed set is closed. A continuous bijection is a homeomorphism if and only if it is open (equivalently, closed).

Not every continuous map is open or closed. The projection $\pi : \mathbb{R}^2 \to \mathbb{R}$ is open (images of open sets are open) but not closed: the image of the closed set $\{(x, 1/x) : x > 0\}$ is $(0, \infty)$, which is not closed. On the other hand, the inclusion of a closed subspace is closed but generally not open.

## Quotient Maps

A *quotient map* is a surjection $q : X \to Y$ where a subset $V \subseteq Y$ is open if and only if $q^{-1}(V)$ is open. Every surjective continuous open map and every surjective continuous closed map is a quotient map.

The universal property of the quotient: a function $f : Y \to Z$ is continuous if and only if $f \circ q : X \to Z$ is continuous. This makes the quotient map the coequalizer in **Top** of the equivalence relation on $X$ that $q$ collapses.

Quotient maps are the key construction for building the circle, torus, Klein bottle, and all CW complexes. They are also the precursor to higher inductive types in HoTT, where the quotient relation is replaced by path constructors that declare which elements are identified.

## Continuity Captures "No Tearing"

What does it really mean for a function to be continuous? Intuitively: you cannot "tear" or "jump." If you follow a path in $X$, the image in $Y$ is a path — no sudden discontinuities.

Here is a precise expression of this intuition. A function $f : X \to Y$ is continuous if and only if: for every $x \in X$ and every neighborhood $V$ of $f(x)$ in $Y$, the preimage $f^{-1}(V)$ is a neighborhood of $x$ in $X$. In words: to control where $f(x)$ lands (confine it to a neighborhood in $Y$), you can do so by restricting $x$ to a corresponding neighborhood in $X$. No tearing means: nearby preimages map to nearby images.

A homeomorphism is a "no tearing" bijection whose inverse is also "no tearing." Homeomorphic spaces are geometrically indistinguishable from the topological point of view. They have the same "shape" in the sense that matters for topology.

## The HoTT Connection

In HoTT, the analog of a continuous map $f : X \to Y$ is a function $f : A \to B$ between types. The analog of a homeomorphism is an equivalence $f : A \simeq B$ — a function with a quasi-inverse $g : B \to A$ and homotopies $\eta : g \circ f \sim \mathsf{id}_A$ and $\varepsilon : f \circ g \sim \mathsf{id}_B$.

The critical difference is that in HoTT, the homotopies $\eta$ and $\varepsilon$ are *terms in types* — they are data, not mere existence statements. An equivalence carries with it explicit witnesses of the invertibility. This makes the type $A \simeq B$ computationally meaningful: you can extract the inverse and the homotopies.

The univalence axiom connects equivalences to identity: $A \simeq B$ implies $A = B$ (as types in the universe). This is the type-theoretic analogue of homeomorphism: equivalent types are identical as mathematical objects. But equivalence in HoTT is homotopy equivalence, not point-set homeomorphism — it is the coarser, more flexible notion that homotopy theory privileges.

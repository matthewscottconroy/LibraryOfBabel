# 1.1 Topological Spaces: Definitions and Examples

## Motivating the Definition

Let's think about what it means for a function $f : \mathbb{R} \to \mathbb{R}$ to be continuous at a point $x_0$. The classical $\varepsilon$-$\delta$ definition says:

$$\forall \varepsilon > 0, \exists \delta > 0, |x - x_0| < \delta \Rightarrow |f(x) - f(x_0)| < \varepsilon$$

This is a statement about metric balls. But there's an equivalent reformulation:

$$f \text{ is continuous} \iff \text{for every open } V \subseteq \mathbb{R} \text{ with } f(x_0) \in V, f^{-1}(V) \text{ is open and contains } x_0$$

Or globally: *$f$ is continuous iff the preimage of every open set is open.*

This second formulation only refers to "open sets," not to the specific metric. So: if we abstract away from the metric and just keep track of which sets are "open," we can still define continuity.

That's the core idea of a topological space: a set equipped with a collection of "open" subsets, chosen to axiomatize the properties that open sets have in metric spaces.

## The Definition

**Definition 1.1 (Topological Space).** A *topological space* is a pair $(X, \tau)$ where $X$ is a set and $\tau \subseteq \mathcal{P}(X)$ is a collection of subsets (called *open sets*) satisfying:
1. **Trivial sets are open:** $\emptyset \in \tau$ and $X \in \tau$
2. **Arbitrary unions:** If $\{U_\alpha\}_{\alpha \in I} \subseteq \tau$ (any collection of open sets), then $\bigcup_{\alpha \in I} U_\alpha \in \tau$
3. **Finite intersections:** If $U, V \in \tau$, then $U \cap V \in \tau$

The collection $\tau$ is called the *topology* on $X$. We often write $X$ for $(X, \tau)$ when the topology is clear from context.

A set $C \subseteq X$ is *closed* if its complement $X \setminus C$ is open. Note: sets can be both open and closed (clopen), neither, open but not closed, or closed but not open. The words "open" and "closed" are not mutually exclusive.

**Why these axioms?** 
- The empty set and $X$ should be open because they're trivially "everywhere" and "nowhere." In a metric space, $\emptyset$ and $X$ are always open.
- Arbitrary unions: openness is a *local* condition. If every point of $U_\alpha$ has a neighborhood in $U_\alpha$, then every point of $\bigcup U_\alpha$ has a neighborhood in the union.
- Only *finite* intersections: the intersection of infinitely many open sets can be closed. Example: $\bigcap_{n=1}^\infty (-1/n, 1/n) = \{0\}$ in $\mathbb{R}$.

## Key Examples

### 1. The Metric Topology

Given a metric space $(X, d)$, define the *metric topology* (or *induced topology*): a set $U \subseteq X$ is open if for every $x \in U$, there exists $\varepsilon > 0$ with $B(x,\varepsilon) = \{y : d(x,y) < \varepsilon\} \subseteq U$.

This satisfies all three axioms:
- $\emptyset$ is vacuously open; $X$ is open (any $\varepsilon$ works for any $x \in X$)
- Unions: if each $U_\alpha$ is open and $x \in \bigcup U_\alpha$, then $x \in U_\alpha$ for some $\alpha$, so there's a ball in $U_\alpha$, hence in the union
- Finite intersections: if $x \in U \cap V$, there are $\varepsilon_1, \varepsilon_2 > 0$ with balls in $U, V$; take $\varepsilon = \min(\varepsilon_1, \varepsilon_2)$

The familiar Euclidean topology on $\mathbb{R}^n$ is the metric topology for the standard metric.

### 2. The Discrete Topology

For any set $X$, the *discrete topology* is $\tau = \mathcal{P}(X)$ — every subset is open. This corresponds to the metric topology for the discrete metric $d(x,y) = 1$ if $x \neq y$, $d(x,x) = 0$: every singleton $\{x\}$ is an open ball of radius $1/2$.

In the discrete topology, *every* function $f : X \to Y$ (to any topological space $Y$) is continuous. The discrete topology is the "finest" topology — it has the most open sets.

### 3. The Indiscrete (Trivial) Topology

The *indiscrete topology* on $X$ is $\tau = \{\emptyset, X\}$ — as few open sets as possible. Every continuous function $f : X \to Y$ with $X$ indiscrete is continuous regardless of what $Y$ is (there are only two open sets to check preimages of). But the only continuous functions $f : Y \to X$ (into an indiscrete space) are constant functions (for Hausdorff $Y$).

The indiscrete topology is the "coarsest" topology — it has the fewest open sets.

### 4. The Sierpiński Space

$X = \{0, 1\}$ with topology $\tau = \{\emptyset, \{1\}, \{0,1\}\}$. The open sets are: nothing, just $\{1\}$, and everything.

This tiny space is surprisingly powerful: **a continuous function $f : Y \to \{0,1\}$ corresponds exactly to an open set in $Y$** (namely $f^{-1}(\{1\})$). The Sierpiński space *classifies* open sets.

This is a concrete connection to logic: in classical logic, a proposition (true/false) corresponds to an element of $\{0,1\}$. In topology, a "proposition about $Y$" (a predicate on $Y$) corresponds to a subset of $Y$. The Sierpiński space makes this correspondence precise: predicates correspond to maps to $\{0,1\}$.

### 5. The Zariski Topology

On $\mathbb{R}$ (or more generally on the prime spectrum of a ring), the *Zariski topology* has closed sets = finite sets (plus all of $\mathbb{R}$). Explicitly:
- Open sets: $\emptyset$, $\mathbb{R}$, and all cofinite sets (sets with finite complement)

Check the axioms:
- $\emptyset$ and $\mathbb{R}$ are open ✓
- Arbitrary unions of cofinite sets: cofinite (if each complement is finite, the union has finite complement) ✓
- Finite intersections: finite intersection of cofinite sets is cofinite ✓

This topology is very coarse (not Hausdorff — in fact $T_1$ but not $T_2$). It's the natural topology in algebraic geometry where "closed" means "zero set of some polynomial."

## Bases for Topologies

In practice, you often specify a topology by giving a *basis* — a generating collection from which you get all open sets.

**Definition 1.2 (Basis).** A *basis* for a topology on $X$ is a collection $\mathcal{B} \subseteq \mathcal{P}(X)$ satisfying:
1. For every $x \in X$, some $B \in \mathcal{B}$ contains $x$
2. If $x \in B_1 \cap B_2$ for $B_1, B_2 \in \mathcal{B}$, there exists $B_3 \in \mathcal{B}$ with $x \in B_3 \subseteq B_1 \cap B_2$

The *topology generated by $\mathcal{B}$* consists of all unions of elements of $\mathcal{B}$:
$$\tau_\mathcal{B} = \left\{ \bigcup_{\alpha \in I} B_\alpha \mid I \text{ any index set}, B_\alpha \in \mathcal{B} \right\}$$

**Examples:**
- The collection of open intervals $(a,b)$ is a basis for the standard topology on $\mathbb{R}$
- Open balls $B(x, \varepsilon)$ form a basis for the metric topology on any metric space
- Rectangular open boxes $(a_1, b_1) \times \cdots \times (a_n, b_n)$ form a basis for $\mathbb{R}^n$

## Building New Spaces from Old

Three fundamental constructions:

### Subspace Topology

Given a topological space $(X, \tau)$ and a subset $A \subseteq X$, the *subspace topology* on $A$ is:
$$\tau_A = \{U \cap A \mid U \in \tau\}$$

This is the "inherited" topology: a set $V \subseteq A$ is open in $A$ iff $V = U \cap A$ for some open $U$ in $X$.

**Examples:** The standard topology on $[0,1]$ is the subspace topology from $\mathbb{R}$. The topology on $S^1 = \{(x,y) : x^2 + y^2 = 1\} \subseteq \mathbb{R}^2$ is the subspace topology.

### Product Topology

Given $(X, \tau_X)$ and $(Y, \tau_Y)$, the *product topology* on $X \times Y$ is generated by the basis:
$$\mathcal{B} = \{U \times V \mid U \in \tau_X, V \in \tau_Y\}$$

Open sets are arbitrary unions of "open boxes" $U \times V$.

**Universal property:** The product topology makes both projections $\pi_1 : X \times Y \to X$ and $\pi_2 : X \times Y \to Y$ continuous. Moreover, it's the *coarsest* topology with this property: if $Z$ is any space with continuous maps $f : Z \to X$ and $g : Z \to Y$, then the induced map $(f, g) : Z \to X \times Y$ is continuous.

This universal property characterizes the product topology: it's the categorical product in **Top**.

### Quotient Topology

Given $(X, \tau_X)$ and a surjection $q : X \to Y$, the *quotient topology* on $Y$ is:
$$\tau_Y = \{V \subseteq Y \mid q^{-1}(V) \in \tau_X\}$$

$V$ is open in $Y$ iff its preimage is open in $X$.

**Universal property:** The quotient map $q$ is continuous, and it's the *finest* topology making $q$ continuous. Any function $f : Y \to Z$ is continuous iff $f \circ q : X \to Z$ is continuous.

**Key examples:**
- The circle: $S^1 = [0,1] / (0 \sim 1)$ — identify the endpoints
- The torus: $T^2 = [0,1]^2 / \sim$ — identify opposite edges
- Projective space: $\mathbb{RP}^n = S^n / (x \sim -x)$ — identify antipodal points
- Suspension: $\Sigma X = X \times [-1,1] / \sim$ where $(x, 1) \sim (x', 1)$ and $(x, -1) \sim (x', -1)$ for all $x, x'$

These quotient constructions are exactly what higher inductive types (HITs) in HoTT generalize! A HIT like the circle HIT $S^1$ is defined by generators (a point and a loop) that mimic the quotient structure.

## The Category Top

Topological spaces and continuous maps form a category, denoted **Top**:
- Objects: topological spaces $(X, \tau)$
- Morphisms: continuous functions $f : X \to Y$
- Composition: ordinary function composition (continuous by Theorem 2.1)
- Identity: the identity function (continuous)

**Top** has:
- All limits: products, pullbacks, equalizers (all exist)
- All colimits: coproducts (disjoint unions), pushouts, coequalizers (all exist)
- Exponentials (function spaces) — but only in the subcategory of compactly generated spaces

The existence of all limits and colimits makes **Top** a convenient setting for categorical constructions. The lack of nice exponentials is a technical drawback, motivating the restriction to compactly generated spaces (Section 7 of the source chapter).

## Homeomorphism vs. Homotopy Equivalence

It's worth noting two notions of "same space":
- **Homeomorphism:** A bijection $f : X \to Y$ with $f$ and $f^{-1}$ both continuous. This is the isomorphism in **Top** — topologically identical spaces.
- **Homotopy equivalence:** A map $f : X \to Y$ with a homotopy inverse $g : Y \to X$ (i.e., $f \circ g \simeq \mathsf{id}_Y$ and $g \circ f \simeq \mathsf{id}_X$). This is a weaker notion — homotopy-equivalent spaces may not be homeomorphic.

A *deformation retract* of $X$ onto $A \subseteq X$ is a homotopy from $\mathsf{id}_X$ to a retraction $r : X \to A$ that fixes $A$. If such a retract exists, $X$ and $A$ are homotopy equivalent.

**Example:** $\mathbb{R}^n$ is homotopy equivalent to a point (it deformation retracts onto any point). But $\mathbb{R}^n$ is not homeomorphic to a point (unless $n = 0$).

In HoTT, paths $p : a = b$ in a type $A$ correspond to *homotopies*, not homeomorphisms. The homotopy theory of spaces — equivalences up to homotopy — is what HoTT captures. Point-set topology (homeomorphism) is a strictly finer invariant.

## Summary

| Topology | Open sets | Key property |
|---|---|---|
| Metric topology | Open balls | Inherited from distance |
| Discrete | All subsets | Every function continuous out |
| Indiscrete | $\emptyset$ and $X$ | Every function continuous in |
| Sierpiński | $\emptyset, \{1\}, \{0,1\}$ | Classifies open sets |
| Zariski | Cofinite + $\emptyset$ | Used in algebraic geometry |

The key insight: topology axiomatizes what you need to define continuity. Once you have a topology, you can define continuous maps, homeomorphisms, and eventually homotopy equivalences — the language of HoTT.

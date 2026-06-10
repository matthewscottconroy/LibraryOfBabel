# Metric Spaces

## Distance as the Primitive

Mathematics often works by identifying the right level of generality: not so specific that results apply only to special cases, not so abstract that nothing can be proved. For analysis, the right level is the *metric space* — a set equipped with a notion of distance.

Real analysis on ℝ works because ℝ has a distance: |x - y|. Complex analysis works because ℂ has a distance: |z - w|. Analysis on ℝⁿ works because ℝⁿ has a distance: the Euclidean distance. Analysis on function spaces works because function spaces have a distance: the supremum norm, or the L² norm. These all look different, but they satisfy the same three axioms. Proving results from just those axioms gives theorems that apply to all of them simultaneously.

**Definition.** A *metric space* is a pair (X, d) where X is a set and d: X × X → [0, ∞) is a *metric* (or *distance function*) satisfying:

**M1 (Non-negativity).** d(x, y) ≥ 0 for all x, y ∈ X, with d(x, y) = 0 iff x = y.

**M2 (Symmetry).** d(x, y) = d(y, x) for all x, y ∈ X.

**M3 (Triangle inequality).** d(x, z) ≤ d(x, y) + d(y, z) for all x, y, z ∈ X.

The triangle inequality captures the geometric fact that the direct route is no longer than any two-leg route. It is the most substantive axiom, and the one that enables most proofs.

## Examples

**ℝ with the absolute value metric.** d(x, y) = |x - y|. The archetypal example. Non-negativity: |x-y| ≥ 0, with equality iff x = y. Symmetry: |x-y| = |y-x|. Triangle inequality: |x-z| = |x-y+y-z| ≤ |x-y| + |y-z|.

**ℝⁿ with the Euclidean metric.** d(x, y) = √(Σᵢ(xᵢ-yᵢ)²). Triangle inequality: the Cauchy-Schwarz inequality.

**ℝⁿ with the taxicab metric.** d(x, y) = Σᵢ|xᵢ - yᵢ|. Also called the L¹ or Manhattan metric. Balls in this metric are diamonds (in ℝ²), not discs.

**ℝⁿ with the max metric.** d(x, y) = maxᵢ|xᵢ - yᵢ|. Balls are cubes. All three metrics on ℝⁿ are *equivalent* in the sense that they produce the same open sets (same topology), even though the metrics themselves are different.

**Discrete metric.** d(x, y) = 0 if x = y, 1 if x ≠ y. Every set becomes a metric space. Sequences converge only when eventually constant.

**The function space C([0,1]).** The set of continuous real-valued functions on [0,1], with d(f, g) = sup_{t ∈ [0,1]} |f(t) - g(t)|. This is the *supremum norm* metric, denoted ‖f - g‖_∞. This is a complete metric space (a crucial fact for analysis of functions).

**Graphs as metric spaces.** A finite graph with vertices V and edges E becomes a metric space: d(v, w) is the length of the shortest path from v to w (number of edges). This is the standard "graph distance."

## Open Balls and the Topology of Metric Spaces

**Definition.** The *open ball* of radius r > 0 centered at x ∈ X is B(x, r) = {y ∈ X | d(x, y) < r}.

Open balls are the building blocks of metric space topology.

**Definition.** A subset U ⊆ X is *open* if for every x ∈ U, there exists r > 0 such that B(x, r) ⊆ U.

Equivalently: U is open iff every point in U has a "neighborhood" entirely contained in U. Open sets are the sets where you can always move a little in any direction and stay inside.

**Definition.** A subset F ⊆ X is *closed* if its complement X \ F is open.

**Basic facts:**
- ∅ and X are both open (and both closed).
- Arbitrary unions of open sets are open.
- Finite intersections of open sets are open.
- Arbitrary intersections of closed sets are closed.
- Finite unions of closed sets are closed.

These four properties of the open sets of a metric space are the axioms for a *topological space*. Metric spaces are a special case of topological spaces — those whose topology comes from a metric.

**Interior, closure, boundary.** The *interior* int(A) of a set A is the largest open set contained in A. The *closure* cl(A) is the smallest closed set containing A. The *boundary* ∂A = cl(A) \ int(A).

**Example.** In ℝ: int((0,1]) = (0,1), cl((0,1]) = [0,1], ∂(0,1] = {0, 1}.

## Continuous Functions

**Definition (ε-δ).** A function f: (X, d_X) → (Y, d_Y) between metric spaces is *continuous at x ∈ X* if for every ε > 0, there exists δ > 0 such that d_X(x', x) < δ implies d_Y(f(x'), f(x)) < ε.

f is *continuous* if it is continuous at every x ∈ X.

**Definition (topological).** f: X → Y is continuous if for every open set V ⊆ Y, the preimage f⁻¹(V) = {x ∈ X | f(x) ∈ V} is open in X.

**Theorem.** These two definitions are equivalent for metric spaces.

*Proof.* (ε-δ → topological) Let V ⊆ Y be open and x ∈ f⁻¹(V). Then f(x) ∈ V, and since V is open, there exists ε > 0 with B(f(x), ε) ⊆ V. By ε-δ continuity, there exists δ > 0 with f(B(x, δ)) ⊆ B(f(x), ε) ⊆ V, so B(x, δ) ⊆ f⁻¹(V). Since x was arbitrary, f⁻¹(V) is open.

(topological → ε-δ) Let x ∈ X and ε > 0. The ball B(f(x), ε) is open in Y, so f⁻¹(B(f(x), ε)) is open in X. It contains x. So there exists δ > 0 with B(x, δ) ⊆ f⁻¹(B(f(x), ε)), which says: d_X(x', x) < δ implies f(x') ∈ B(f(x), ε), i.e., d_Y(f(x'), f(x)) < ε. □

The topological definition — preimages of opens are open — is the right one for topology. It does not reference distances; it uses only the open set structure. When we generalize from metric spaces to topological spaces (which may have no metric), the topological definition is the one that makes sense.

## Homeomorphisms

**Definition.** A *homeomorphism* is a bijective continuous function f: X → Y with a continuous inverse. If such a function exists, X and Y are *homeomorphic* (topologically equivalent).

Homeomorphic spaces are, from a topological standpoint, identical: they have the same open sets (up to relabeling). A coffee cup and a donut are homeomorphic (same number of holes). A sphere and a plane are not.

**Examples:**
- (0, 1) and ℝ are homeomorphic: f(x) = tan(π(x - 1/2)) is a homeomorphism.
- [0, 1] and [0, 2] are homeomorphic: f(x) = 2x.
- [0, 1] and (0, 1) are *not* homeomorphic: [0, 1] is compact but (0, 1) is not.
- S¹ (the circle) and ℝ are not homeomorphic: S¹ is compact and ℝ is not.

Homeomorphism is the right notion of "sameness" for topological spaces. Two homeomorphic spaces have all the same topological properties: same fundamental group, same homology groups, same compactness, same connectedness.

In HoTT, homeomorphism corresponds to type *equivalence* at the 0-truncation level (for topological spaces modeled as types). The Univalence Axiom upgrades this to actual equality: homeomorphic types *are* equal (in the appropriate universe). This resolves the annoying fact that two different constructions of the circle — say, as a quotient of [0,1] and as a subspace of ℝ² — are homeomorphic but not literally equal sets in ZFC.

## Completeness Preview

A metric space is *complete* if every Cauchy sequence converges. We will develop this in the next section, but the key examples:
- ℝ: complete.
- ℚ: not complete (the sequence 1, 1.4, 1.41, 1.414, ... is Cauchy in ℚ but converges to √2 ∉ ℚ).
- C([0,1]) with supremum metric: complete.
- The space of rational polynomials: not complete.

Completeness is the metric-space abstraction of having "no gaps." It is the right property for analysis: you need completeness to guarantee that limits exist, that differential equations have solutions, that fixed-point theorems work.

The key theorem connecting metric spaces and topology: all separable complete metric spaces are topologically equivalent (Urysohn metrization theorem). Metric spaces are the "tamest" kind of topological spaces.

The connection to HoTT: the completion of a metric space is a construction that freely adds limits of Cauchy sequences. It is the metric-space analogue of the higher inductive type construction that adds generators and path-constructors. The Cauchy real numbers are the completion of ℚ — a HIT that freely adjoins limits, with the universal property that any ℚ-homomorphism to a complete metric space extends uniquely to the Cauchy reals. Universal properties everywhere.

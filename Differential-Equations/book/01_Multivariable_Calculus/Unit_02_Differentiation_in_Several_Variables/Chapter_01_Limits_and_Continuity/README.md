# Chapter 01 Limits and Continuity

Before differentiating a function of several variables, one must understand what it means for that function to be continuous, and before continuity, one must understand limits. In one variable, the approach to a point is constrained: one can come from the left or from the right. In $\mathbb{R}^n$, there are infinitely many directions of approach, and a function must behave consistently along all of them to have a limit. This additional complexity introduces genuinely new phenomena — functions that have limits along every straight line through a point but fail to have a limit at that point — and demands careful topological foundations.

## What This Chapter Covers

**Section 1 (Open and Closed Sets: Topology)** builds the vocabulary of neighborhoods and topology in $\mathbb{R}^n$. An open ball of radius $r$ centered at $\mathbf{p}$ is the set $B(\mathbf{p}, r) = \{\mathbf{x} : \|\mathbf{x}-\mathbf{p}\| < r\}$. A set is open if every point has an open ball contained entirely within the set; a set is closed if its complement is open, equivalently if it contains all its limit points. The concepts of interior, boundary, closure, and compactness are introduced here and will be used throughout the course.

**Section 2 (Multivariable Limits)** defines the limit $\lim_{\mathbf{x}\to\mathbf{p}} f(\mathbf{x}) = L$ using the $\epsilon$-$\delta$ definition in $\mathbb{R}^n$: for every $\epsilon > 0$, there exists $\delta > 0$ such that $0 < \|\mathbf{x}-\mathbf{p}\| < \delta$ implies $|f(\mathbf{x}) - L| < \epsilon$. The key challenge is the full generality of "approach" in $\mathbb{R}^n$: the limit must hold for all paths from all directions simultaneously.

**Section 3 (Continuity in Several Variables)** defines continuity at a point in terms of limits and develops the standard properties: sums, products, and compositions of continuous functions are continuous. Polynomials in several variables are continuous everywhere; rational functions are continuous away from the zeros of the denominator.

**Section 4 (Path Dependence and Discontinuity)** explores the converse direction: how does one show that a limit does not exist? The path dependence method computes the limit along two different paths to $\mathbf{p}$ and shows they give different values. This demonstrates the crucial point that even if the limit along every line through $\mathbf{p}$ exists and equals $L$, the function can still fail to have a limit at $\mathbf{p}$.

## How the Sections Build on Each Other

The topological vocabulary of Section 1 underlies everything that follows. Open sets appear in the definition of a limit (the function is defined on a punctured open ball around $\mathbf{p}$); closed sets and compact sets appear in existence theorems (continuous functions on compact sets attain their extreme values). The limit definition of Section 2 is used directly in the continuity definition of Section 3. Section 4 provides the practical techniques — the path method — for demonstrating non-existence of limits, which complements the $\epsilon$-$\delta$ proofs of Section 2.

## How This Chapter Fits into the Unit

Limits and continuity are the foundation on which differentiability is built. In one variable, a function can have a derivative at a point only if it is continuous there; the same is true in several variables. Partial derivatives (Chapter 2) are limits of difference quotients along coordinate axes, and the subtlety of their relationship to differentiability (Chapter 3) — the fact that partial derivatives can exist even at a point where the function is not differentiable, or not even continuous — illustrates exactly the kind of path-dependence phenomenon studied in Section 4. Students who internalize the message of this chapter — that behavior along individual paths is not sufficient to determine a limit — will be prepared for the more subtle notions of differentiability in Chapter 3.

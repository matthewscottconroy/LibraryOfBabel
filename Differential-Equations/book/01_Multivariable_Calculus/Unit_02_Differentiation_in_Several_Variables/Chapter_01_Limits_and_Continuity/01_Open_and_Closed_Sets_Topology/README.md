# Open and Closed Sets: Topology

The analysis of functions of several variables rests on a body of concepts from point-set topology: open sets, closed sets, boundary points, compact sets, and connected sets. These are not merely technical preliminaries — they encode the fundamental geometric structure of $\mathbb{R}^n$ that determines whether limiting processes are well-behaved. The goal of this section is to build fluency with this vocabulary so that later definitions (limit, continuity, differentiability) can be stated precisely and efficiently.

## The Euclidean Metric

The starting point is the distance function. For $\mathbf{x} = (x_1, \ldots, x_n)$ and $\mathbf{y} = (y_1, \ldots, y_n)$ in $\mathbb{R}^n$, the **Euclidean distance** is

$$d(\mathbf{x}, \mathbf{y}) = \|\mathbf{x} - \mathbf{y}\| = \sqrt{\sum_{i=1}^n (x_i - y_i)^2}.$$

This satisfies the three axioms of a metric: $d(\mathbf{x},\mathbf{y}) \geq 0$ with equality iff $\mathbf{x} = \mathbf{y}$; symmetry $d(\mathbf{x},\mathbf{y}) = d(\mathbf{y},\mathbf{x})$; and the triangle inequality $d(\mathbf{x},\mathbf{z}) \leq d(\mathbf{x},\mathbf{y}) + d(\mathbf{y},\mathbf{z})$.

## Open Balls and Neighborhoods

The **open ball** of radius $r > 0$ centered at $\mathbf{p} \in \mathbb{R}^n$ is

$$B(\mathbf{p}, r) = \{\mathbf{x} \in \mathbb{R}^n : \|\mathbf{x} - \mathbf{p}\| < r\}.$$

In $\mathbb{R}^1$, this is an open interval $(p-r, p+r)$. In $\mathbb{R}^2$, it is an open disk. In $\mathbb{R}^3$, it is an open ball (no surface included). The **closed ball** $\overline{B}(\mathbf{p}, r) = \{\mathbf{x} : \|\mathbf{x}-\mathbf{p}\| \leq r\}$ includes the boundary.

A **neighborhood** of $\mathbf{p}$ is any set containing an open ball centered at $\mathbf{p}$.

## Open Sets

A set $U \subseteq \mathbb{R}^n$ is **open** if every point of $U$ is an interior point: for every $\mathbf{p} \in U$, there exists $r > 0$ such that $B(\mathbf{p}, r) \subseteq U$.

Intuitively, an open set contains a small "buffer zone" around every one of its points — no point of $U$ is on the "edge."

**Examples:**
- The open ball $B(\mathbf{p}, r)$ itself is open.
- $\mathbb{R}^n$ is open (trivially).
- The empty set $\emptyset$ is open (vacuously: there are no points that fail the condition).
- The set $\{(x,y) : x^2 + y^2 < 1\}$ (open unit disk) is open.
- The set $\{(x,y) : x^2 + y^2 \leq 1\}$ (closed unit disk) is not open: points on the circle $x^2+y^2=1$ are not interior points.

**Properties:** Arbitrary unions of open sets are open; finite intersections of open sets are open. (Infinite intersections need not be open: $\bigcap_{n=1}^\infty (-1/n, 1/n) = \{0\}$, which is not open in $\mathbb{R}$.)

## Closed Sets

A set $F \subseteq \mathbb{R}^n$ is **closed** if its complement $\mathbb{R}^n \setminus F$ is open, equivalently if $F$ contains all its **limit points** (a point $\mathbf{p}$ is a limit point of $F$ if every open ball around $\mathbf{p}$ contains a point of $F$ other than $\mathbf{p}$ itself).

**Examples:**
- The closed ball $\overline{B}(\mathbf{p}, r)$ is closed.
- $\mathbb{R}^n$ and $\emptyset$ are both closed (and open — sets can be both).
- A finite set is closed.
- The set $\{(x,y): y \geq 0\}$ (the closed upper half-plane) is closed.
- The set $\{(x,y): 0 < x < 1, 0 < y < 1\}$ (open unit square) is neither open nor closed: it is open, but its closure (which includes the boundary) is closed.

A set can be neither open nor closed (e.g., $[0, 1)$ in $\mathbb{R}$).

## Interior, Boundary, and Closure

For any set $S \subseteq \mathbb{R}^n$:
- The **interior** $\text{int}(S)$ is the set of all interior points — the largest open set contained in $S$.
- The **closure** $\overline{S}$ is $S$ together with all its limit points — the smallest closed set containing $S$.
- The **boundary** $\partial S = \overline{S} \setminus \text{int}(S)$ consists of the points that are in the closure but not the interior: every ball around a boundary point contains points both in $S$ and outside $S$.

**Example.** For $S = \{(x,y): x^2+y^2 < 1\}$: $\text{int}(S) = S$; $\overline{S} = \{x^2+y^2 \leq 1\}$; $\partial S = \{x^2+y^2=1\}$ (the unit circle).

## Compact Sets

A set $K \subseteq \mathbb{R}^n$ is **compact** if every sequence in $K$ has a subsequence converging to a point in $K$. By the Heine-Borel theorem (specific to $\mathbb{R}^n$):

**Theorem (Heine-Borel).** A subset $K \subseteq \mathbb{R}^n$ is compact if and only if it is closed and bounded.

**Bounded** means contained in some closed ball: $K \subseteq \overline{B}(\mathbf{0}, M)$ for some $M > 0$.

Compact sets are the correct setting for theorems about continuous functions achieving their extreme values. The **Extreme Value Theorem** states: if $f: K \to \mathbb{R}$ is continuous and $K$ is compact, then $f$ attains its maximum and minimum on $K$. This requires compactness — continuous functions on non-compact sets need not be bounded.

## Connected Sets

A set $S$ is **connected** if it cannot be partitioned into two nonempty disjoint open subsets. Intuitively, a connected set is "all in one piece." A set $S \subseteq \mathbb{R}^n$ is **path-connected** if for any two points in $S$, there is a continuous path connecting them within $S$. Path-connectedness implies connectedness; in $\mathbb{R}^n$ the two notions agree for open sets.

## Why This Matters

The precise notions of open and closed sets determine where functions can be analyzed. The domain of a function must often be specified as an open set for differentiability to make sense at every point (otherwise boundary points require one-sided limits). The extreme value theorem requires a compact domain. The intermediate value theorem requires a connected domain. These assumptions will appear in the hypotheses of theorems throughout the unit, and understanding what they mean geometrically is essential for applying those theorems correctly.

## Common Pitfalls

The adjectives "open" and "closed" do not mean "not closed" and "not open" respectively. A set can be both (like $\mathbb{R}^n$ or $\emptyset$) or neither (like $[0,1)$). Treating them as opposites is one of the most common misconceptions in real analysis.

Also, compactness is not the same as closedness alone; one also needs boundedness. The set $\{(x,0): x \in \mathbb{R}\}$ (the $x$-axis) is closed but not bounded, hence not compact, and continuous functions on it need not be bounded.

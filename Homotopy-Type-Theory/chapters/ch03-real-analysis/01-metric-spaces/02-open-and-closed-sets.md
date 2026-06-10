# 1.2 Open and Closed Sets

## From Distance to Topology

We have a metric space $(X, d)$. The metric gives us a precise notion of distance. Now we want to extract a more abstract notion: *neighborhoods*, or "regions around a point." This is the step from *metric geometry* to *topology*.

The key insight is that most of the important structures of analysis — limits, continuity, compactness — can be described purely in terms of which sets are "open," without ever referring to the exact distance values. The metric generates an abstract structure called the *topology*, and it's this topology that carries the relevant information.

## Open Balls

The most basic piece of structure the metric gives us is the open ball.

**Definition.** Let $(X, d)$ be a metric space, $x \in X$, and $r > 0$. The *open ball of radius $r$ centered at $x$* is:
$$B(x, r) = \{y \in X \mid d(x, y) < r\}$$

An open ball is the set of all points strictly within distance $r$ of the center $x$. The word "open" refers to the strict inequality — we exclude the boundary points at exactly distance $r$.

**Examples:**
- In $\mathbb{R}$ with the standard metric, $B(a, r) = (a - r, a + r)$, an open interval.
- In $\mathbb{R}^2$ with the Euclidean metric, $B(p, r)$ is an open disk (a circle without its circumference).
- In $\mathbb{R}^2$ with the taxicab metric, $B(p, r)$ is a diamond (rotated square) without its edges.
- In the discrete metric, $B(x, 1) = \{x\}$ (the ball of radius 1 is just the point itself), while $B(x, 2) = X$ (the ball of radius 2 contains everything).

Open balls are the building blocks of the topology. Every open set will be built from open balls.

## Open Sets

**Definition.** A subset $U \subseteq X$ is *open* if for every $x \in U$, there exists $r > 0$ such that $B(x, r) \subseteq U$.

In other words: $U$ is open if every point in $U$ has some open ball around it that stays inside $U$. No point of $U$ is "on the boundary of $U$" — every point has a little neighborhood around it.

**Intuition.** Think of an open interval $(a, b)$ in $\mathbb{R}$. Every point $x$ in $(a, b)$ has some breathing room: you can go at least $\min(x - a, b - x) > 0$ to either side and stay inside $(a, b)$. The endpoints $a$ and $b$ are *not* in $(a, b)$, which is why you can always find a ball that stays inside.

Now think of a closed interval $[a, b]$. The endpoints $a$ and $b$ are in $[a, b]$, but every open ball around $a$ contains points less than $a$, which are outside $[a, b]$. So $[a, b]$ is *not* open (in $\mathbb{R}$).

**Basic properties of open sets:**

**Theorem.** In any metric space $(X, d)$:
1. $\emptyset$ and $X$ are open.
2. Any union of open sets is open: if $U_\alpha$ is open for all $\alpha$, then $\bigcup_\alpha U_\alpha$ is open.
3. Any finite intersection of open sets is open: if $U_1, \ldots, U_n$ are open, then $U_1 \cap \cdots \cap U_n$ is open.

*Proof of (1):* $\emptyset$ is open vacuously (there are no points to check). $X$ is open because $B(x, 1) \subseteq X$ for any $x$.

*Proof of (2):* Suppose $\{U_\alpha\}$ is a collection of open sets and let $U = \bigcup_\alpha U_\alpha$. Take any $x \in U$. Then $x \in U_\alpha$ for some $\alpha$. Since $U_\alpha$ is open, there exists $r > 0$ with $B(x, r) \subseteq U_\alpha \subseteq U$. So $U$ is open.

*Proof of (3):* Suppose $U_1, \ldots, U_n$ are open and let $U = \bigcap_{i=1}^n U_i$. Take any $x \in U$. Then $x \in U_i$ for each $i$. Since each $U_i$ is open, there exist $r_i > 0$ with $B(x, r_i) \subseteq U_i$. Let $r = \min(r_1, \ldots, r_n) > 0$. Then $B(x, r) \subseteq B(x, r_i) \subseteq U_i$ for each $i$, so $B(x, r) \subseteq U$. $\square$

Notice: the proof of (3) uses the *minimum* of finitely many positive numbers, which is still positive. For an infinite intersection, this argument breaks down. And indeed, infinite intersections of open sets need not be open:
$$\bigcap_{n=1}^\infty \left(-\frac{1}{n}, \frac{1}{n}\right) = \{0\}$$
which is not open in $\mathbb{R}$ (for any $r > 0$, the ball $(-r, r)$ around $0$ extends outside $\{0\}$).

## Open Balls Are Open

Wait — are open balls actually open sets? The name suggests it, but we should verify it.

**Proposition.** Every open ball $B(x, r)$ is an open set.

*Proof.* Take any $y \in B(x, r)$. We need to find $s > 0$ with $B(y, s) \subseteq B(x, r)$.

Let $s = r - d(x, y) > 0$ (positive because $y \in B(x, r)$ means $d(x, y) < r$).

Now take any $z \in B(y, s)$, so $d(y, z) < s$. By the triangle inequality:
$$d(x, z) \leq d(x, y) + d(y, z) < d(x, y) + s = d(x, y) + r - d(x, y) = r$$

So $z \in B(x, r)$. This proves $B(y, s) \subseteq B(x, r)$. $\square$

Good — open balls are open. And every point $x$ lies in the open ball $B(x, 1)$ (or any ball around $x$). So the open sets are exactly those that look like open balls "locally."

## Closed Sets

**Definition.** A subset $F \subseteq X$ is *closed* if its complement $X \setminus F$ is open.

From the properties of open sets, we immediately get dual properties of closed sets:

1. $\emptyset$ and $X$ are closed (their complements are $X$ and $\emptyset$, both open).
2. Any intersection of closed sets is closed.
3. Any finite union of closed sets is closed.

**Examples in $\mathbb{R}$:**
- Open intervals $(a, b)$: open (as shown above).
- Closed intervals $[a, b]$: closed (their complements are $(-\infty, a) \cup (b, \infty)$, a union of open sets).
- Half-open intervals $[a, b)$: neither open nor closed.
- $\mathbb{R}$ itself and $\emptyset$: both open and closed (the term "clopen").
- Single points $\{a\}$: closed (their complement is $(-\infty, a) \cup (a, \infty)$, open).
- $\mathbb{Q} \subseteq \mathbb{R}$: neither open nor closed.

Beware: "not open" does not mean "closed," and "not closed" does not mean "open." Most sets are neither open nor closed.

## Limit Points and the Closure

There's another useful characterization of closed sets, via limit points.

**Definition.** A point $x \in X$ is a *limit point* (or *accumulation point*) of a set $A \subseteq X$ if every open ball $B(x, r)$ contains a point of $A$ different from $x$:
$$\forall r > 0,\, B(x, r) \cap (A \setminus \{x\}) \neq \emptyset$$

Note: $x$ itself need not be in $A$.

**Examples:** In $\mathbb{R}$:
- Every point of $[0, 1]$ is a limit point of $(0, 1)$.
- $0$ is a limit point of $(0, 1)$ (even though $0 \notin (0, 1)$).
- Every point of $\mathbb{R}$ is a limit point of $\mathbb{Q}$ (rationals are dense).
- No point of $\mathbb{R}$ is a limit point of $\mathbb{Z}$ — integers are isolated.

**Definition.** The *closure* of a set $A$, written $\overline{A}$ or $\text{cl}(A)$, is the union of $A$ and all its limit points.

**Theorem.** $A$ is closed if and only if $A$ contains all its limit points (i.e., $A = \overline{A}$).

*Proof sketch.* If $A$ is closed and $x \notin A$, then $x \in X \setminus A$, which is open. So there's an open ball $B(x, r) \subseteq X \setminus A$, meaning $B(x, r)$ contains no points of $A$. So $x$ is not a limit point of $A$.

Conversely, if $A$ contains all its limit points, then for any $x \notin A$, $x$ is not a limit point, so there exists $r > 0$ with $B(x, r) \cap A = \emptyset$, i.e., $B(x, r) \subseteq X \setminus A$. This makes $X \setminus A$ open, so $A$ is closed. $\square$

The closure of a set is the smallest closed set containing it.

## Interior

**Definition.** A point $x \in A$ is an *interior point* of $A$ if there exists $r > 0$ with $B(x, r) \subseteq A$. The set of all interior points of $A$ is the *interior* of $A$, written $\text{int}(A)$ or $A^\circ$.

The interior of $A$ is the largest open set contained in $A$.

**Examples:** In $\mathbb{R}$:
- $\text{int}([0, 1]) = (0, 1)$
- $\text{int}((0, 1)) = (0, 1)$
- $\text{int}(\{0\}) = \emptyset$
- $\text{int}(\mathbb{Q}) = \emptyset$ (no interval is entirely rational)

**Proposition.** $A$ is open iff $A = \text{int}(A)$.

This gives three equivalent characterizations of open sets:
1. $A$ is open iff every point of $A$ has an open ball contained in $A$.
2. $A$ is open iff $A = \text{int}(A)$.
3. $A$ is open iff $A$ is a union of open balls.

Characterization (3) follows because: if $A$ is open, then for each $x \in A$ there's a ball $B(x, r_x) \subseteq A$, and $A = \bigcup_{x \in A} B(x, r_x)$.

## The Topology

The collection of all open subsets of a metric space $(X, d)$ is called the *topology* generated by $d$, often written $\tau_d$.

This is an abstract object: a collection of subsets of $X$ satisfying the three properties (empty set and whole space open; arbitrary unions; finite intersections). In the next chapter of the curriculum, we'll see that one can study topological spaces abstractly, without reference to any underlying metric. But metric spaces are the fundamental source of examples and the setting for analysis.

Two metrics on the same set can generate the same topology (as with $d_1$, $d_2$, $d_\infty$ on $\mathbb{R}^n$), or different topologies (as with the Euclidean metric and the discrete metric on $\mathbb{R}$, which generate different collections of open sets).

The key point: the topological notions — open sets, closed sets, convergence, continuity — depend only on the topology $\tau_d$, not on the specific distance values. This is why, in later chapters, we'll often pass from metric spaces to topological spaces and work at that level of abstraction.

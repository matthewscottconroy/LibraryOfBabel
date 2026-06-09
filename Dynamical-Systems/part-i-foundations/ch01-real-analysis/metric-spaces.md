# 1.1 Metric Spaces

## 1.1.1 Definition and First Examples

Mathematics lives in spaces — sets of objects equipped with enough structure to talk about closeness. The most basic such structure is a *metric*: a way of measuring distance that obeys three simple rules.

You already know metrics intuitively. The Euclidean distance in the plane. The maximum discrepancy between two functions on an interval. The number of positions where two binary strings differ. What do all these have in common? Three properties that, taken together, capture exactly what we mean by "distance":

**Definition 1.1.1 (Metric Space).** A *metric space* is a pair $(X, d)$ where $X$ is a set and $d: X \times X \to [0, \infty)$ satisfies:
1. $d(x, y) = 0$ if and only if $x = y$ (identity of indiscernibles)
2. $d(x, y) = d(y, x)$ for all $x, y \in X$ (symmetry)
3. $d(x, z) \leq d(x, y) + d(y, z)$ for all $x, y, z \in X$ (triangle inequality)

The triangle inequality is the one that does real work. It rules out "distances" like the squared Euclidean norm (which fails it for general triangles), and it encodes the basic geometric truth that a straight line is the shortest path between two points. It's also what makes metric spaces useful: the triangle inequality is what lets us chain estimates together.

Let's see this in action with a few concrete examples — some of which will become permanent residents in what follows:

**Examples 1.1.2.**
- $(\mathbb{R}^n, d_2)$ with $d_2(x, y) = \|x - y\|_2 = \sqrt{\sum_i (x_i - y_i)^2}$ — the Euclidean metric you've known since school.
- $(\mathbb{R}^n, d_\infty)$ with $d_\infty(x, y) = \max_i |x_i - y_i|$ — the sup (or Chebyshev) metric. Same set, different notion of closeness.
- $(C([0,1]), d_\infty)$ with $d_\infty(f, g) = \sup_{t \in [0,1]} |f(t) - g(t)|$ — the space of continuous functions on $[0,1]$ with the *uniform metric*. Two functions are close iff one never strays far from the other, anywhere on $[0,1]$. This is the space where many dynamical systems naturally live.
- The *discrete metric* on any set $X$: $d(x, y) = 1$ if $x \neq y$ and $d(x, x) = 0$. Absurd-seeming but perfectly valid — and a good test case for whether your intuitions are really about metric spaces or secretly about $\mathbb{R}$.

**Remark 1.1.3.** Here's a subtlety worth pausing on: the same underlying set can carry very different metric structures with completely different analytic properties. The continuous function space $(C([0,1]), d_2)$ with $d_2(f,g) = \sqrt{\int_0^1 |f-g|^2}$ is a very different metric space from $(C([0,1]), d_\infty)$ — the first is not complete, the second is. The metric is the real structure. The set is just the raw material.

## 1.1.2 Open and Closed Sets

Once we have a metric, we can talk about "nearness" precisely enough to define the topological notions of open and closed sets. These concepts are going to appear everywhere.

The key idea: a set is open if every point inside it has breathing room — a small ball around it that stays inside the set. Closed sets are defined by exclusion.

**Definition 1.1.4.** Let $(X, d)$ be a metric space and $x \in X$, $r > 0$.
- The *open ball* of radius $r$ centered at $x$: $B(x, r) = \{y \in X : d(x, y) < r\}$
- The *closed ball*: $\bar{B}(x, r) = \{y \in X : d(x, y) \leq r\}$
- A set $U \subseteq X$ is *open* if for every $x \in U$ there exists $r > 0$ with $B(x, r) \subseteq U$
- A set $F \subseteq X$ is *closed* if its complement $X \setminus F$ is open

A few basic stability properties follow almost immediately from the definitions:

**Proposition 1.1.5.**
1. Arbitrary unions of open sets are open.
2. Finite intersections of open sets are open.
3. $\emptyset$ and $X$ are both open and closed.

Note that the second statement says *finite* intersections. Infinite intersections of open sets can collapse: the intersection $\bigcap_{n=1}^\infty (-1/n, 1/n) = \{0\}$ is a single point, which is not open in $\mathbb{R}$.

Every point and every set has associated "derived" sets that encode the local structure:

**Definition 1.1.6.** The *interior* of $A \subseteq X$ is $\text{int}(A) = \{x \in A : \exists r > 0, B(x,r) \subseteq A\}$. The *closure* is $\bar{A} = \{x \in X : B(x, r) \cap A \neq \emptyset \text{ for all } r > 0\}$. The *boundary* is $\partial A = \bar{A} \setminus \text{int}(A)$.

In words: the interior is the "thickened core" of $A$, the closure is $A$ plus all its accumulation points, and the boundary is what's left over. You should check that $A$ is open iff $A = \text{int}(A)$, and closed iff $A = \bar{A}$.

## 1.1.3 Sequences and Convergence

With a metric in hand, convergence of sequences has a clean definition — and this is where the rubber meets the road for analysis.

**Definition 1.1.7.** A sequence $(x_n)$ in $(X, d)$ *converges* to $x \in X$ if $d(x_n, x) \to 0$ as $n \to \infty$. Equivalently: for all $\varepsilon > 0$ there exists $N$ such that $n \geq N$ implies $d(x_n, x) < \varepsilon$.

This is the standard $\varepsilon$-$N$ definition from calculus, now stated for any metric space. The key observation is that the limit, if it exists, is unique (this follows from the triangle inequality).

There's a subtler notion of convergence that doesn't require knowing the limit:

**Definition 1.1.8.** A sequence $(x_n)$ is *Cauchy* if for all $\varepsilon > 0$ there exists $N$ such that $m, n \geq N$ implies $d(x_m, x_n) < \varepsilon$.

Every convergent sequence is Cauchy — if $(x_n) \to x$, use the triangle inequality: $d(x_m, x_n) \leq d(x_m, x) + d(x, x_n)$, and both terms go to zero. The converse is the interesting question.

**Definition 1.1.9.** A metric space $(X, d)$ is *complete* if every Cauchy sequence converges.

Completeness is one of the most important properties a metric space can have. It's what allows us to prove existence theorems: "this sequence is Cauchy, therefore it converges, therefore the limit exists." Without completeness, a Cauchy sequence might be "trying to converge" to something that isn't in the space.

Here's the range of behavior you should keep in mind:

**Examples 1.1.10.**
- $\mathbb{R}^n$ with any norm-induced metric is complete. This is the content of the classical theorem that every Cauchy sequence in $\mathbb{R}$ converges.
- $(C([0,1]), d_\infty)$ is complete — a uniform Cauchy sequence of continuous functions has a continuous limit (you should prove this; it's a fundamental exercise).
- $(\mathbb{Q}, |\cdot|)$ is *not* complete — sequences of rationals that approximate $\sqrt{2}$ are Cauchy in $\mathbb{Q}$ but converge outside $\mathbb{Q}$. The reals are literally constructed as the completion of the rationals.

The difference between $\mathbb{Q}$ and $\mathbb{R}$ is the paradigm for why completeness matters: incomplete spaces have "holes," and those holes are exactly where limits want to live.

With the metric space framework established, we're ready to ask the most important structural question in analysis: what does it mean for a space to be "finite-like" in a useful sense? The answer is compactness, and it's the subject of the next section.

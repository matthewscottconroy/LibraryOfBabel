# Exercises

---

**Exercise 3.1.** Verify that the taxicab metric $d_1(x, y) = \sum_i |x_i - y_i|$ on $\mathbb{R}^n$ satisfies all four metric axioms. Describe the "unit ball" $\{y : d_1(y, 0) < 1\}$ in $\mathbb{R}^2$ geometrically.

---

**Exercise 3.2.** Prove that the sup metric $d_\infty(x, y) = \max_i |x_i - y_i|$ on $\mathbb{R}^n$ satisfies the triangle inequality.

Show that $d_\infty(x, y) \leq d_2(x, y) \leq \sqrt{n}\, d_\infty(x, y)$ for all $x, y \in \mathbb{R}^n$. Conclude that $d_2$ and $d_\infty$ are Lipschitz equivalent, hence generate the same topology.

---

**Exercise 3.3.** Let $X = \{a, b, c\}$ and $d(a, b) = 1$, $d(a, c) = 2$, $d(b, c) = 3$. Is this a metric? If not, which axiom fails?

---

**Exercise 3.4.** Prove: the closure $\overline{A}$ is the smallest closed set containing $A$. That is:
(a) $\overline{A}$ is closed.
(b) $A \subseteq \overline{A}$.
(c) If $F$ is closed and $A \subseteq F$, then $\overline{A} \subseteq F$.

---

**Exercise 3.5.** 
(a) Show that a finite union of closed sets is closed.
(b) Give an example showing a countably infinite union of closed sets need not be closed.
(c) Show that $\mathbb{Q}$ is not open and not closed in $\mathbb{R}$.

---

**Exercise 3.6.** Prove that in any metric space, every convergent sequence is Cauchy. Give an example of a Cauchy sequence that does not converge (in some incomplete space of your choice).

---

**Exercise 3.7 (Baire Category Theorem).** Prove the following weak form: $\mathbb{R}$ is not a countable union of nowhere-dense sets.

*Hint:* Assume $\mathbb{R} = \bigcup_{n=1}^\infty A_n$ where each $A_n$ is nowhere dense. Construct a nested sequence of closed balls $\overline{B}_n$ with $\overline{B}_{n+1} \subseteq \overline{B}_n \setminus A_n$ and radii $\to 0$. Use completeness to derive a contradiction.

---

**Exercise 3.8.** Let $f, g : [0, 1] \to \mathbb{R}$ be continuous. Prove that $h(x) = \max(f(x), g(x))$ is continuous. What about $k(x) = \min(f(x), g(x))$?

*Hint:* $\max(f, g) = \frac{1}{2}(f + g + |f - g|)$.

---

**Exercise 3.9.** 
(a) Prove that $f : (0, 1) \to \mathbb{R}$ defined by $f(x) = 1/x$ is continuous but not uniformly continuous.
(b) Prove that $g : [1, \infty) \to \mathbb{R}$ defined by $g(x) = 1/x$ is uniformly continuous.

---

**Exercise 3.10.** The Banach Fixed Point Theorem requires a contraction ($k < 1$). Show the theorem fails if we only assume $d(f(x), f(y)) < d(x, y)$ (strict but not uniform contraction).

*Hint:* Consider $f : [1, \infty) \to [1, \infty)$ defined by $f(x) = x + 1/x$.

---

**Exercise 3.11.** Prove that $[0, 1]$ is compact using the sequential definition: every sequence in $[0, 1]$ has a convergent subsequence with limit in $[0, 1]$.

*Hint:* Bisection argument — split $[0, 1]$ in half, one half contains infinitely many terms, iterate.

---

**Exercise 3.12.** Prove the Extreme Value Theorem: a continuous function $f : K \to \mathbb{R}$ on a compact metric space $K$ attains its maximum value.

---

**Exercise 3.13.** 
(a) Prove that $\mathbb{R}^n$ is connected.
(b) Prove that $S^n = \{x \in \mathbb{R}^{n+1} : \|x\| = 1\}$ is path-connected for $n \geq 1$.
(c) Conclude that $S^n$ is connected for $n \geq 1$.

---

**Exercise 3.14.** Prove that $(0, 1)$ and $[0, 1]$ are not homeomorphic by showing:
(a) $[0, 1]$ is compact but $(0, 1)$ is not.
(b) Alternatively: removing the endpoints from $[0, 1]$ gives a disconnected space, but removing any two points from $(0, 1)$ can give either 1, 2, or 3 connected components. Find all cases.

---

**Exercise 3.15 (Intermediate Value Theorem).** 
(a) Use the IVT to show that every odd-degree polynomial $p(x) = x^{2n+1} + a_{2n} x^{2n} + \cdots + a_0$ with real coefficients has at least one real root.

(b) Show that for any continuous $f : S^1 \to \mathbb{R}$, there exist diametrically opposite points $x, -x \in S^1$ with $f(x) = f(-x)$. (This is the Borsuk-Ulam theorem in dimension 1.)

---

**Exercise 3.16.** Define the *diameter* of a set $A \subseteq X$ as $\text{diam}(A) = \sup\{d(x, y) : x, y \in A\}$.

(a) Prove: if $K$ is compact and $f : K \to \mathbb{R}$ is continuous, then $f$ is uniformly continuous. Use the Lebesgue number lemma.

(b) Let $(K_n)$ be a decreasing sequence of non-empty compact sets with $\text{diam}(K_n) \to 0$. Prove $\bigcap_{n=1}^\infty K_n$ is a single point.

---

**Exercise 3.17 (Conceptual — Paths in HoTT).** 

In Homotopy Type Theory, a type $A$ is called *contractible* if there exists $a : A$ and a path $p_x : a = x$ for every $x : A$ (all points are connected to $a$ by a path).

(a) In classical topology, what does it mean for a space to be contractible? (Hint: all points are connected to a basepoint by a path, and this can be done continuously.)

(b) Are the following spaces contractible: $\{pt\}$ (one point), $[0, 1]$, $S^1$, $\mathbb{R}^n$?

(c) Explain why contractible types in HoTT play the role that "trivial" or "empty-of-structure" spaces play in topology. (Hint: a proposition in HoTT is a type where all elements are equal — this is like a contractible space where the only structure is the basepoint.)

---

**Exercise 3.18.** Let $(X, d)$ be a complete metric space and $f : X \to X$ a contraction with constant $k$.

(a) Show the unique fixed point $x^*$ satisfies the *a priori* error estimate:
$$d(x_n, x^*) \leq \frac{k^n}{1 - k} d(x_1, x_0)$$

(b) Apply this to $f(x) = \cos(x)$ on $[0, 1]$ (note: is this a contraction? find $k$). Estimate how many iterations are needed to find a fixed point to within $10^{-6}$.

---

**Exercise 3.19 (Research).** The *p-adic numbers* $\mathbb{Q}_p$ are a completion of $\mathbb{Q}$ with respect to a different metric (the $p$-adic metric, based on divisibility by the prime $p$). Look up the $p$-adic metric and answer:

(a) How is the $p$-adic absolute value $|x|_p$ defined?

(b) The $p$-adic metric satisfies a stronger version of the triangle inequality: $|x + y|_p \leq \max(|x|_p, |y|_p)$. Prove this implies the usual triangle inequality.

(c) In $\mathbb{Q}_p$, the sequence $1, p, p^2, p^3, \ldots$ converges to 0. How is this possible?

(d) How does the existence of different completions of $\mathbb{Q}$ ($\mathbb{R}$ and $\mathbb{Q}_p$ for each prime $p$) relate to the identity problem in foundations?

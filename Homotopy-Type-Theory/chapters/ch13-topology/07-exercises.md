# Exercises

---

**Exercise 13.1 (Zariski Topology).** Consider the Zariski topology on $\mathbb{R}$: closed sets are finite sets and all of $\mathbb{R}$.

(a) Verify this is a topology: check the three axioms.

(b) Show it is $T_1$: for any $x \neq y$, exhibit an open set containing $x$ but not $y$.

(c) Show it is NOT Hausdorff: any two non-empty open sets intersect (the open sets are "dense" in each other).

(d) What does a continuous function $f : \mathbb{R}_{\text{Zariski}} \to \mathbb{R}_{\text{Euclidean}}$ look like? (Hint: preimage of any open interval must be a cofinite set or empty.)

---

**Exercise 13.2 (Product Topology as Categorical Product).** Show that $X \times Y$ with the product topology is the categorical product in **Top**.

(a) The projections $\pi_1 : X \times Y \to X$ and $\pi_2 : X \times Y \to Y$ are continuous.

(b) For any space $Z$ and continuous maps $f : Z \to X$, $g : Z \to Y$, the induced map $(f,g) : Z \to X \times Y$ is continuous.

(c) The induced map is unique: it's the only continuous map $h : Z \to X \times Y$ with $\pi_1 \circ h = f$ and $\pi_2 \circ h = g$.

---

**Exercise 13.3 (Quotient Circle).** The circle $S^1 = [0,1]/(0 \sim 1)$:

(a) Describe explicitly which subsets of $S^1$ are open in the quotient topology.

(b) Show that the map $f : [0,1] \to S^1 \subseteq \mathbb{R}^2$ given by $f(t) = (\cos(2\pi t), \sin(2\pi t))$ is a quotient map.

(c) Using the universal property, define a continuous map $S^1 \to \mathbb{R}$ and show it cannot be injective.

---

**Exercise 13.4 (Compact → Hausdorff → Homeomorphism).** Prove Theorem 2.6: if $f : X \to Y$ is a continuous bijection, $X$ is compact, and $Y$ is Hausdorff, then $f$ is a homeomorphism.

(a) Show that every closed subset $C \subseteq X$ is compact.

(b) Show that $f(C)$ is compact in $Y$.

(c) Show that $f(C)$ is closed in $Y$ (using Hausdorff condition).

(d) Conclude that $f^{-1}$ is continuous (hint: it maps opens to opens, equivalently closeds to closeds).

---

**Exercise 13.5 (Compactness of $[0,1]$).** Prove directly that $[0,1]$ is compact.

(a) Start with an open cover $\{U_\alpha\}$ of $[0,1]$. Let $S = \{x \in [0,1] : [0,x] \text{ has a finite subcover}\}$.

(b) Show $S$ is non-empty (it contains small $x$).

(c) Let $s = \sup S$. Show $s \in S$ (the supremum is in $S$).

(d) Show $s = 1$ (if $s < 1$, we can extend the finite subcover slightly past $s$).

---

**Exercise 13.6 (Compactness is Topological).** Show that compactness is preserved by homeomorphism: if $X \cong Y$ and $X$ is compact, then $Y$ is compact.

---

**Exercise 13.7 (Projective Space).** The real projective space $\mathbb{RP}^n = S^n/(x \sim -x)$:

(a) Show $\mathbb{RP}^1 \cong S^1$. (Hint: Both points of $S^1 = \{-1, 1\} \times \mathbb{R}$ identified by $\sim$... think about it via parametrizing $S^1 \subseteq \mathbb{C}$ by angle $\theta \in [0, \pi)$.)

(b) Is $\mathbb{RP}^2$ a manifold? What is its dimension?

(c) The fundamental group of $\mathbb{RP}^n$ for $n \geq 2$ is $\mathbb{Z}/2\mathbb{Z}$. What does this say about loops in $\mathbb{RP}^n$?

---

**Exercise 13.8 (Retracts and Compactness).** A *retract* of $X$ is a subspace $A$ with a continuous $r : X \to A$ with $r|_A = \mathsf{id}_A$.

(a) Show that if $X$ is compact and $r : X \to A$ is a retraction, then $A$ is compact.

(b) Show that a retract of a Hausdorff space is closed.

(c) Is $S^1$ a retract of $D^2$ (the closed disk)? What would follow if it were?

---

**Exercise 13.9 (The Topologist's Sine Curve).** Let $S = \{(x, \sin(1/x)) : 0 < x \leq 1\} \cup \{0\} \times [-1,1]$.

(a) Show $S$ is connected: it's the closure of the graph $\{(x, \sin(1/x)) : 0 < x \leq 1\}$, which is connected (as the continuous image of $(0,1]$), and closures of connected sets are connected.

(b) Show $S$ is not path-connected: assume for contradiction that there is a path $\gamma : [0,1] \to S$ from $(1, \sin 1)$ to $(0, 0)$. Show that $\gamma$ must oscillate between $y = 1$ and $y = -1$ infinitely often near the endpoint, which contradicts continuity.

---

**Exercise 13.10 (Topology to HoTT Dictionary).** For each classical construction, describe the HoTT analog.

(a) The path space $P(X, x, y) = \{\gamma : [0,1] \to X : \gamma(0) = x, \gamma(1) = y\}$. What is the HoTT analog?

(b) The loop space $\Omega(X, x) = \{\gamma : [0,1] \to X : \gamma(0) = \gamma(1) = x\}$. HoTT analog?

(c) The fundamental group $\pi_1(X, x)$ as defined by homotopy classes of loops. HoTT analog?

(d) A *fibration* $p : E \to B$ (a map with the homotopy lifting property). HoTT analog?

(e) The *fiber* $p^{-1}(b)$ for $b \in B$. HoTT analog?

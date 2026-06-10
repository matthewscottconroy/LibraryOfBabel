# Exercises: Point-Set Topology

---

**Exercise 13.1 (Topology Axioms).** For each of the following, determine whether it is a topology on $X = \{a, b, c\}$. If not, state which axiom fails.

(a) $\tau_1 = \{\emptyset, \{a\}, \{b,c\}, \{a,b,c\}\}$

(b) $\tau_2 = \{\emptyset, \{a\}, \{b\}, \{a,b,c\}\}$

(c) $\tau_3 = \{\emptyset, \{a\}, \{a,b\}, \{a,c\}, \{a,b,c\}\}$

(d) $\tau_4 = \mathcal{P}(\{a,b,c\})$ (all subsets)

---

**Exercise 13.2 (Metric Topology Verification).** Let $X = \mathbb{R}$ with the standard absolute-value metric $d(x,y) = |x-y|$.

(a) Show that the open balls $B(x, \varepsilon) = (x-\varepsilon, x+\varepsilon)$ form a basis for a topology.

(b) Show that this topology satisfies all three topology axioms.

(c) Show that $(0,1)$ is open in this topology, and that $[0,1]$ is closed.

(d) Show that $[0,1)$ is neither open nor closed.

---

**Exercise 13.3 (Sierpiński Space).** Let $\Sigma = \{0, 1\}$ with topology $\{\emptyset, \{1\}, \{0,1\}\}$.

(a) Verify this is a topology.

(b) Show that continuous functions $f : X \to \Sigma$ from any space $X$ are in bijection with open subsets of $X$ (the bijection sends $f$ to $f^{-1}(\{1\})$).

(c) Is $\Sigma$ Hausdorff? Justify.

(d) What does the map $f : \mathbb{R} \to \Sigma$ given by $f(x) = 1$ if $x > 0$ and $f(x) = 0$ if $x \leq 0$ correspond to as an open set? Is $f$ continuous?

---

**Exercise 13.4 (Subspace Topology).** Let $X = \mathbb{R}$ with the standard topology and $A = [0,1]$.

(a) Describe the subspace topology on $A$: which subsets of $[0,1]$ are open?

(b) Show that the set $\{1/n : n \geq 1\} \cup \{0\}$ is closed in $[0,1]$ with the subspace topology.

(c) Show that $(0, 1/2)$ is open in $A$ but not open in $\mathbb{R}$.

(d) Find a subset of $A$ that is open in $A$ but not in $\mathbb{R}$.

---

**Exercise 13.5 (Product Topology).** Consider $X = Y = \mathbb{R}$ and $Z = X \times Y = \mathbb{R}^2$.

(a) Show that open "boxes" $U \times V$ (with $U, V$ open in $\mathbb{R}$) form a basis for the product topology on $\mathbb{R}^2$.

(b) Show that every open ball $B((x,y), \varepsilon) \subseteq \mathbb{R}^2$ is open in the product topology.

(c) Show that the product topology on $\mathbb{R}^2$ equals the standard metric topology on $\mathbb{R}^2$.

(d) (Universal property) Show that the projection $\pi_1 : \mathbb{R}^2 \to \mathbb{R}$ is continuous. Show that the product topology is the coarsest topology making both projections continuous.

---

**Exercise 13.6 (Quotient Circle).** Define $S^1 = [0,1]/\{0 \sim 1\}$.

(a) Describe explicitly which subsets of $S^1$ are open in the quotient topology.

(b) Show that the map $q : [0,1] \to S^1$ defined by $q(t) = e^{2\pi i t}$ (viewing $S^1 \subseteq \mathbb{C}$) satisfies the universal property of the quotient.

(c) Define a continuous map $f : S^1 \to \mathbb{R}$ and show it cannot be injective. (Hint: use the intermediate value theorem.)

(d) Show that the map $g : S^1 \to S^1$ defined by $g(e^{2\pi i t}) = e^{4\pi i t}$ (doubling the angle) is well-defined and continuous.

---

**Exercise 13.7 (Continuity and Preimages).** Let $f : X \to Y$ be a function between topological spaces. Show the following are equivalent:

(a) $f$ is continuous (preimage of every open set is open).

(b) Preimage of every closed set is closed.

(c) For every $x \in X$ and neighborhood $V$ of $f(x)$, there is a neighborhood $U$ of $x$ with $f(U) \subseteq V$.

(d) $f(\overline{A}) \subseteq \overline{f(A)}$ for every $A \subseteq X$.

---

**Exercise 13.8 (Homeomorphism).** Prove or disprove each of the following homeomorphisms.

(a) $(0,1) \cong \mathbb{R}$ (via $f(x) = \tan(\pi(x - 1/2))$).

(b) $[0,1] \cong [0,2]$ (find an explicit homeomorphism).

(c) $[0,1) \cong [0,1]$ (they are not homeomorphic — exhibit a topological invariant that distinguishes them).

(d) $S^1 \cong [0,1]$ (they are not — why?).

---

**Exercise 13.9 (The Topologist's Sine Curve).** Let $S = \{(x, \sin(1/x)) : 0 < x \leq 1\} \cup (\{0\} \times [-1,1])$.

(a) Show that $S$ is connected. (The image of $(0,1]$ under $x \mapsto (x, \sin(1/x))$ is connected; use the fact that the closure of a connected set is connected.)

(b) Show that $S$ is not path-connected. (Assume for contradiction there is a path from $(1, \sin 1)$ to $(0, 0)$ and derive a contradiction from the oscillation of $\sin(1/x)$.)

(c) How many path-components does $S$ have? Identify them.

---

**Exercise 13.10 (Compactness via Sequences).** Show that $[0,1]$ is compact using the following approach.

(a) Let $\{U_\alpha\}$ be an open cover of $[0,1]$. Define $S = \{x \in [0,1] : [0,x] \text{ has a finite subcover}\}$.

(b) Show $S$ is non-empty and bounded above, so $s = \sup S$ exists.

(c) Show $s \in S$ (i.e., $[0,s]$ has a finite subcover). (Use the fact that $s$ is covered by some $U_\alpha$.)

(d) Show $s = 1$. (If $s < 1$, extend the finite subcover to cover a slightly larger interval.)

---

**Exercise 13.11 (Heine-Borel).** Prove that a closed bounded subset of $\mathbb{R}^n$ is compact.

(a) Show that $[0,1]^n$ is compact. (Use Tychonoff for finite products: $[0,1]^n \cong [0,1] \times \cdots \times [0,1]$, and the product of compact spaces is compact.)

(b) Show that any closed subset of a compact space is compact.

(c) Conclude that any closed bounded subset of $\mathbb{R}^n$ is compact.

---

**Exercise 13.12 (Compactness is Preserved).** Let $f : X \to Y$ be continuous and $K \subseteq X$ compact. Show $f(K)$ is compact.

---

**Exercise 13.13 (Non-Compact Spaces).** Show that the following spaces are not compact.

(a) $\mathbb{R}$ with the standard topology. (Exhibit an open cover with no finite subcover.)

(b) $(0,1)$ with the standard topology.

(c) $\mathbb{Q} \cap [0,1]$ (rationals in the unit interval, with the subspace topology from $\mathbb{R}$).

---

**Exercise 13.14 (Torus Construction).** Construct the torus $T^2$.

(a) Describe the equivalence relation on $[0,1]^2$ that gives the torus.

(b) Show that the map $q : [0,1]^2 \to S^1 \times S^1$ given by $q(s,t) = (e^{2\pi is}, e^{2\pi it})$ is a quotient map.

(c) Conclude $T^2 \cong S^1 \times S^1$.

(d) What quotient of $[0,1]^2$ gives the Klein bottle? Describe the identification.

---

**Exercise 13.15 (Real Projective Plane).** The real projective plane $\mathbb{RP}^2 = S^2/\{x \sim -x\}$.

(a) Show $\mathbb{RP}^1 \cong S^1$. (Consider $S^1 \subseteq \mathbb{C}$; the antipodal map is $z \mapsto -z$. The quotient is $S^1/\{z \sim -z\}$. Show this is homeomorphic to $S^1$ via $z \mapsto z^2$.)

(b) Is $\mathbb{RP}^2$ orientable? (No — it contains a Möbius band as a subspace.)

(c) The fundamental group $\pi_1(\mathbb{RP}^2) = \mathbb{Z}/2\mathbb{Z}$. What does this say geometrically about loops in $\mathbb{RP}^2$?

---

**Exercise 13.16 (CW Complex for the Torus).** Build a CW complex homeomorphic to $T^2$.

(a) Describe the cell structure: how many cells in each dimension?

(b) Describe the attaching maps explicitly.

(c) Use van Kampen's theorem for the CW complex to compute $\pi_1(T^2)$.

---

**Exercise 13.17 (HoTT Translation).** For each topological construction, give the HoTT analog.

(a) The path space $P(X, x, y) = \{\gamma : [0,1] \to X : \gamma(0) = x, \gamma(1) = y\}$.

(b) The loop space $\Omega(X, x) = \{\gamma : [0,1] \to X : \gamma(0) = \gamma(1) = x\}$.

(c) The fundamental group $\pi_1(X, x)$.

(d) The fibration $p : E \to B$ with the homotopy lifting property.

(e) The circle $S^1$ as a quotient $[0,1]/\{0 \sim 1\}$.

---

**Exercise 13.18 (Basis Verification).** Show that the following collections form a basis for the indicated topology.

(a) Open intervals with rational endpoints form a basis for the standard topology on $\mathbb{R}$.

(b) Half-open intervals $[a, b)$ with $a, b \in \mathbb{Q}$ form a basis for the Sorgenfrey topology on $\mathbb{R}$.

(c) Open boxes $(a_1, b_1) \times \cdots \times (a_n, b_n)$ form a basis for the product topology on $\mathbb{R}^n$.

---

**Exercise 13.19 (Closure and Interior).** For each subset $A$ of $\mathbb{R}$, compute $\overline{A}$ (closure) and $A^\circ$ (interior).

(a) $A = (0, 1)$.

(b) $A = [0, 1)$.

(c) $A = \mathbb{Q}$ (rationals).

(d) $A = \{1/n : n \geq 1\}$.

(e) $A = \mathbb{Z}$ (integers).

---

**Exercise 13.20 (Connected Components).** For each space, find all connected components and all path-components.

(a) $X = (0,1) \cup (2,3) \cup \{4\}$ with the subspace topology from $\mathbb{R}$.

(b) $X = \mathbb{Q}$ with the subspace topology from $\mathbb{R}$.

(c) $X = \mathbb{R} \setminus \{0\}$ with the standard topology.

---

**Exercise 13.21 (Pasting Lemma Application).** Use the pasting lemma to prove the following.

(a) The concatenation of two paths $\alpha : [0,1] \to X$ (with $\alpha(0) = x$, $\alpha(1) = y$) and $\beta : [0,1] \to X$ (with $\beta(0) = y$, $\beta(1) = z$) is a continuous path from $x$ to $z$.

(b) Any map $f : S^1 \to X$ can be described by a map $\tilde{f} : [0,1] \to X$ with $\tilde{f}(0) = \tilde{f}(1)$, and vice versa.

---

**Exercise 13.22 (Open and Closed Maps).** Give examples of:

(a) A continuous map that is neither open nor closed.

(b) A continuous open map that is not a homeomorphism.

(c) A continuous closed map that is not a homeomorphism.

(d) A bijective continuous map that is not a homeomorphism. (Recall: $[0,1) \to S^1$.)

---

**Exercise 13.23 (Separation Axioms).** 

(a) Show that every metric space is Hausdorff ($T_2$).

(b) Show that every Hausdorff space is $T_1$ (distinct points can be separated by open sets: for every $x \neq y$, there is an open set containing $x$ but not $y$).

(c) Find a $T_1$ space that is not $T_2$. (Hint: the Zariski topology on $\mathbb{R}$.)

(d) Prove that in a Hausdorff space, every singleton $\{x\}$ is a closed set.

---

**Exercise 13.24 (Tychonoff for Two Factors).** Prove that the product of two compact spaces is compact.

(a) Let $K$ and $L$ be compact. Let $\mathcal{U}$ be an open cover of $K \times L$.

(b) For each $x \in K$, show that $\{x\} \times L$ has a finite subcover from $\mathcal{U}$.

(c) Show that the union of the sets in this finite subcover contains an open "tube" $U_x \times L$ around $\{x\} \times L$ for some open $U_x \ni x$.

(d) Extract a finite subcover of $K$ by $\{U_x\}$, and conclude that $K \times L$ has a finite subcover.

---

**Exercise 13.25 (Universal Property of Quotient).** Let $q : X \to X/\sim$ be a quotient map. Prove the universal property: a function $f : X/\sim \to Y$ is continuous if and only if $f \circ q : X \to Y$ is continuous.

(a) Show: if $f$ is continuous, then $f \circ q$ is continuous (composition of continuous maps).

(b) Show: if $f \circ q$ is continuous and $V \subseteq Y$ is open, then $q^{-1}(f^{-1}(V)) = (f \circ q)^{-1}(V)$ is open in $X$, hence $f^{-1}(V)$ is open in $X/\sim$ by definition of the quotient topology.

---

**Exercise 13.26 (CW Structure of $\mathbb{RP}^n$).** Real projective space $\mathbb{RP}^n$ has a CW structure with one cell in each dimension $0, 1, \ldots, n$.

(a) For $\mathbb{RP}^1 \cong S^1$: describe the CW structure (one 0-cell, one 1-cell).

(b) For $\mathbb{RP}^2$: describe the CW structure (one 0-cell, one 1-cell, one 2-cell). What is the attaching map of the 2-cell? (It wraps around the 1-cell twice.)

(c) Use van Kampen's theorem to compute $\pi_1(\mathbb{RP}^2)$ from the CW structure.

---

**Exercise 13.27 (Suspensions).** The suspension $\Sigma X$ of a topological space $X$ is $(X \times [-1,1])/\sim$ where $(x, 1) \sim (x', 1)$ and $(x,-1) \sim (x',-1)$ for all $x, x' \in X$.

(a) Show $\Sigma S^0 \cong S^1$.

(b) Show $\Sigma S^1 \cong S^2$.

(c) Describe the CW structure of $S^n = \Sigma^n S^0$.

---

**Exercise 13.28 (The Compact-Open Topology).** Given topological spaces $X$ and $Y$, the *compact-open topology* on $\mathcal{C}(X, Y)$ (the set of continuous functions) is generated by sets $V(K, U) = \{f : f(K) \subseteq U\}$ for $K \subseteq X$ compact and $U \subseteq Y$ open.

(a) Show these sets form a sub-basis.

(b) Show that if $X = \{*\}$ (a point), then $\mathcal{C}(X, Y) \cong Y$.

(c) Show that evaluation $\text{ev} : \mathcal{C}(X,Y) \times X \to Y$ given by $\text{ev}(f, x) = f(x)$ is continuous (assuming $X$ is locally compact Hausdorff).

---

**Exercise 13.29 (Topological Invariants in Practice).** Determine whether the following pairs of spaces are homeomorphic. For each non-homeomorphic pair, identify an invariant that distinguishes them.

(a) $S^1$ and $\mathbb{R}$.

(b) $S^1$ and $S^2$.

(c) $[0,1]$ and $S^1$.

(d) The figure-eight $S^1 \vee S^1$ and the torus $T^2$.

---

**Exercise 13.30 (Research: Topological Data Analysis).** The *Vietoris-Rips complex* of a finite metric space $(X, d)$ at scale $r > 0$ is the simplicial complex $VR(X, r)$ where a simplex $\{x_0, \ldots, x_k\}$ is included if and only if $d(x_i, x_j) \leq r$ for all $i, j$.

(a) For $X = \{0, 1, 2\} \subseteq \mathbb{R}$ with the standard metric and $r = 1.5$, describe $VR(X, r)$.

(b) For $X = \{(\cos(2\pi k/n), \sin(2\pi k/n)) : k = 0, \ldots, n-1\}$ (equally spaced points on $S^1$), at what value of $r$ does $VR(X, r)$ first become homotopy equivalent to $S^1$?

(c) Explain the idea of persistent homology: as $r$ varies from $0$ to $\infty$, topological features (components, loops, voids) appear and disappear. What does it mean for a feature to "persist"?

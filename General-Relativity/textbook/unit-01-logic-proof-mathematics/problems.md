# Unit I Problems: Logic, Proof, and the Language of Mathematics

*Problems covering propositional and predicate logic, proof techniques, set theory, number systems, and — crucially — tensor algebra and the Einstein summation convention that underlies all subsequent mathematics in this book.*

**Difficulty:** ★ Introductory, ★★ Intermediate, ★★★ Advanced

---

## Part 1: Logic and Proof Techniques

**Problem 1.1** ★
Determine which of the following are propositions. For each that is a proposition, determine its truth value.

(a) $\sqrt{2}$ is rational.
(b) Is there a prime number greater than $10^{100}$?
(c) For all integers $n$, $n^2 \geq 0$.
(d) This statement is false.

**Problem 1.2** ★
Construct truth tables for:

(a) $P \Rightarrow Q$ (implication)
(b) $(P \Rightarrow Q) \wedge (Q \Rightarrow P)$ (biconditional — show it equals $P \Leftrightarrow Q$)
(c) $\neg(P \wedge Q)$ and show it equals $(\neg P) \vee (\neg Q)$ (De Morgan's law)

**Problem 1.3** ★
Prove each by the indicated method:

(a) *Direct proof:* If $n$ is even, then $n^2$ is even.
(b) *Contrapositive:* If $n^2$ is even, then $n$ is even.
(c) *Contradiction:* $\sqrt{2}$ is irrational.
(d) *Induction:* $\sum_{k=1}^n k = n(n+1)/2$ for all $n \geq 1$.

**Problem 1.4** ★★
Negate the following statements (push the negation inside all quantifiers):

(a) $\forall x \in \mathbb{R}: x^2 \geq 0$
(b) $\exists n \in \mathbb{N}: \forall m \in \mathbb{N}, m < n$
(c) $\forall \varepsilon > 0, \exists \delta > 0: |x - a| < \delta \Rightarrow |f(x) - L| < \varepsilon$ (the definition of $\lim_{x\to a}f(x) = L$)

*Note: The negation of (c) is the statement "$f(x)$ does not converge to $L$ as $x\to a$." Make sure your negation captures this exactly.*

---

## Part 2: Set Theory and Cardinality

**Problem 2.1** ★
Let $A$, $B$, $C$ be sets. Prove using element-chasing (pick $x$ and track membership):

(a) $A \cap (B \cup C) = (A\cap B)\cup(A\cap C)$ (distributivity)
(b) $(A\cup B)^c = A^c \cap B^c$ (De Morgan's law for sets)

**Problem 2.2** ★★
Prove that the rational numbers $\mathbb{Q}$ are countable by constructing an explicit bijection from $\mathbb{N}$ to $\mathbb{Q}^+$ (positive rationals), using the Cantor diagonal enumeration of pairs of natural numbers.

**Problem 2.3** ★★
Prove that $\mathbb{R}$ is uncountable (Cantor's diagonalization). *Hint:* Assume a bijection $f: \mathbb{N}\to[0,1]$ exists, write each $f(n)$ in decimal, and construct a number not in the list.

**Problem 2.4** ★★★
The Schröder-Bernstein theorem: if there exist injections $f: A\to B$ and $g: B\to A$, then there exists a bijection $h: A\to B$.

(a) State what this theorem implies about the cardinalities of $\mathbb{R}$ and $(0,1)$.
(b) Sketch the proof: construct $h$ by alternating between $f$ and $g^{-1}$ on a carefully defined subset. (Full proof is demanding; the sketch suffices here.)

---

## Part 3: Index Notation and Tensor Algebra

*These problems are the most important in this unit for the GR curriculum. Einstein summation convention and tensor index manipulation are used on every page of the subsequent material. Master them here.*

**Problem 3.1** ★
Einstein summation convention: in expressions with repeated upper and lower indices, sum over the repeated index. Evaluate the following for $n$-dimensional space, where $\delta^i_{\ j}$ is the Kronecker delta.

(a) $\delta^i_{\ i}$ (sum over $i = 1,\ldots,n$) — this is the trace of the identity matrix.
(b) $\delta^i_{\ j}\delta^j_{\ k}$
(c) $\delta^i_{\ j}V^j$ for a vector $V^j$.
(d) For $n = 4$ (spacetime): what is $\delta^\mu_{\ \mu}$?

**Problem 3.2** ★
Raise and lower indices using the metric $g_{\mu\nu}$ and its inverse $g^{\mu\nu}$ (defined by $g^{\mu\lambda}g_{\lambda\nu} = \delta^\mu_{\ \nu}$).

(a) Express $V_\mu$ (covariant components) in terms of $V^\nu$ (contravariant) and $g_{\mu\nu}$.
(b) Express $T^{\mu\nu}$ in terms of $T_{\alpha\beta}$ and the metric.
(c) For the Minkowski metric $\eta_{\mu\nu} = \text{diag}(-1,+1,+1,+1)$ and 4-velocity $u^\mu = (\gamma c, \gamma v, 0, 0)$: compute $u_\mu = \eta_{\mu\nu}u^\nu$ and the norm $u^\mu u_\mu$.

**Problem 3.3** ★★
Symmetry and antisymmetry: any tensor $T_{\mu\nu}$ can be decomposed uniquely as $T_{\mu\nu} = T_{(\mu\nu)} + T_{[\mu\nu]}$ where:

$$T_{(\mu\nu)} = \frac{1}{2}(T_{\mu\nu} + T_{\nu\mu}), \qquad T_{[\mu\nu]} = \frac{1}{2}(T_{\mu\nu} - T_{\nu\mu})$$

(a) Verify that $T_{(\mu\nu)}$ is symmetric and $T_{[\mu\nu]}$ is antisymmetric.
(b) Show that $A^{\mu\nu}S_{\mu\nu} = 0$ whenever $A^{\mu\nu}$ is antisymmetric and $S_{\mu\nu}$ is symmetric.
(c) The electromagnetic field tensor $F_{\mu\nu}$ is antisymmetric. What is $F_{\mu\nu}F^{\mu\nu}$ in terms of the electric and magnetic fields? (Look up or derive: $F^{\mu\nu}F_{\mu\nu} = 2(B^2 - E^2/c^2)$.)

**Problem 3.4** ★★
Index manipulation practice: simplify or evaluate the following.

(a) $g^{\mu\nu}g_{\mu\nu}$ in $n$ dimensions.
(b) $\delta^\mu_{\ \nu}\delta^\nu_{\ \rho}\delta^\rho_{\ \mu}$ in 4 spacetime dimensions.
(c) For a symmetric tensor $S^{\mu\nu} = S^{\nu\mu}$ and antisymmetric tensor $A_{\mu\nu} = -A_{\nu\mu}$: what is $S^{\mu\nu}A_{\mu\nu}$?
(d) The trace of a $(1,1)$ tensor: $T^\mu_{\ \mu}$. For $T^\mu_{\ \nu} = \delta^\mu_{\ \nu}$ in $n=4$: compute.

**Problem 3.5** ★★★
The Levi-Civita symbol $\varepsilon_{\mu\nu\rho\sigma}$ in 4 dimensions: totally antisymmetric, with $\varepsilon_{0123} = +1$.

(a) How many independent components does $\varepsilon_{\mu\nu\rho\sigma}$ have?
(b) Prove $\varepsilon_{\mu\nu\rho\sigma}\varepsilon^{\mu\nu\rho\sigma} = -4! = -24$ for the Minkowski metric with signature $(-,+,+,+)$. *Hint: count the sign factors carefully.*
(c) The contraction $\varepsilon_{\mu\nu\rho\sigma}\varepsilon^{\mu\nu\alpha\beta} = -2(\delta^\alpha_{\ \rho}\delta^\beta_{\ \sigma} - \delta^\beta_{\ \rho}\delta^\alpha_{\ \sigma})$. Verify this for the specific case $(\rho,\sigma,\alpha,\beta) = (1,2,1,2)$.
(d) The determinant of a matrix $M$ can be written $\det M = \frac{1}{4!}\varepsilon_{\mu\nu\rho\sigma}\varepsilon_{\alpha\beta\gamma\delta}M^\mu_{\ \alpha}M^\nu_{\ \beta}M^\rho_{\ \gamma}M^\sigma_{\ \delta}$. For a $2\times2$ matrix, write down the analogous expression and verify it gives $ad - bc$.

**Problem 3.6** ★★★
Tensor densities: under a coordinate change $x^\mu \to \tilde{x}^\mu$ with Jacobian $J^\mu_{\ \nu} = \partial\tilde{x}^\mu/\partial x^\nu$, a tensor transforms as $T^{\mu\nu}_{\ \ \rho} \to J^\mu_{\ \alpha}J^\nu_{\ \beta}(J^{-1})^\gamma_{\ \rho} T^{\alpha\beta}_{\ \ \gamma}$. A tensor density of weight $w$ picks up an extra factor of $(\det J)^{-w}$ (or $(\det J)^w$ depending on convention).

(a) Show that $\sqrt{-g}$ (where $g = \det g_{\mu\nu}$) transforms as a scalar density of weight $+1$ (i.e., $\sqrt{-\tilde{g}} = |\det J|^{-1}\sqrt{-g}$ — this is the wrong sign, recalculate and state correctly).

(b) Hence show that $\sqrt{-g}\,d^4x$ is a coordinate-invariant volume element (a scalar under coordinate changes).

(c) The action integral $S = \int \mathcal{L}\sqrt{-g}\,d^4x$ is required to be a scalar. What does this require of the Lagrangian density $\mathcal{L}$?

---

## Solutions and Hints for Selected Problems

**3.3(b):** Write out $A^{\mu\nu}S_{\mu\nu} = \sum_{\mu,\nu}A^{\mu\nu}S_{\mu\nu}$. Pair each term $(\mu,\nu)$ with $(\nu,\mu)$: $A^{\mu\nu}S_{\mu\nu} + A^{\nu\mu}S_{\nu\mu} = A^{\mu\nu}S_{\mu\nu} - A^{\mu\nu}S_{\mu\nu} = 0$ using symmetry of $S$ and antisymmetry of $A$.

**3.5(b):** $\varepsilon_{\mu\nu\rho\sigma}\varepsilon^{\mu\nu\rho\sigma} = g^{\mu\alpha}g^{\nu\beta}g^{\rho\gamma}g^{\sigma\delta}\varepsilon_{\mu\nu\rho\sigma}\varepsilon_{\alpha\beta\gamma\delta}$. Each diagonal element of $\eta^{\mu\nu}$ contributes a sign: $(-1)$ for the time component, $(+1)$ for spatial. The sum produces $(-1)\times(+1)\times(+1)\times(+1)\times 4! = -24$.

**3.6(a):** Under $x\to\tilde{x}$, $g_{\mu\nu}(x) \to \tilde{g}_{\mu\nu}(\tilde{x}) = (J^{-1})^\alpha_{\ \mu}(J^{-1})^\beta_{\ \nu}g_{\alpha\beta}$, so $\det\tilde{g} = (\det J^{-1})^2\det g = (\det J)^{-2}\det g$. Hence $\sqrt{-\tilde{g}} = |\det J|^{-1}\sqrt{-g}$. Combining with $d^4\tilde{x} = |\det J|\,d^4x$: the product $\sqrt{-g}\,d^4x$ is invariant.

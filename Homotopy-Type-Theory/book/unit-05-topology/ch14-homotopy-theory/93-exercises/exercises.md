# Exercises: Homotopy Theory

---

**Exercise 14.1 (Homotopy is an Equivalence Relation).** Verify the three properties.

(a) Show $f \simeq f$ for any continuous $f : X \to Y$.

(b) If $H : [0,1] \times X \to Y$ is a homotopy from $f$ to $g$, construct a homotopy from $g$ to $f$.

(c) If $H : f \simeq g$ and $K : g \simeq h$, construct $L : f \simeq h$. Verify $L$ is continuous using the pasting lemma.

---

**Exercise 14.2 (Contractible Spaces).** Determine which of the following spaces are contractible. Justify each answer.

(a) $\mathbb{R}^n$ for any $n \geq 0$.

(b) The unit disk $D^n = \{x \in \mathbb{R}^n : |x| \leq 1\}$.

(c) $S^n$ for $n \geq 1$.

(d) $\mathbb{R}^2 \setminus \{0\}$ (the punctured plane).

(e) A convex subset of $\mathbb{R}^n$.

---

**Exercise 14.3 (Deformation Retracts).** Show that the following spaces are homotopy equivalent.

(a) $\mathbb{R}^n \setminus \{0\}$ is homotopy equivalent to $S^{n-1}$. (Exhibit a deformation retract.)

(b) The Möbius band deformation retracts to its central circle. Describe the retraction.

(c) A connected graph $\Gamma$ with $v$ vertices and $e$ edges has the homotopy type of a wedge of $\beta$ circles, where $\beta = e - v + 1$. Explain why (use spanning trees).

---

**Exercise 14.4 (Computing $\pi_1$).** Compute the fundamental group of each space.

(a) $S^1$ (the circle). State the result and outline the proof using covering spaces.

(b) $\mathbb{R}^2 \setminus \{(0,0)\}$ (the punctured plane).

(c) $\mathbb{R}^2 \setminus \{p_1, p_2\}$ (the plane minus two points). (Hint: use van Kampen.)

(d) $S^1 \vee S^1$ (the figure-eight). State the result as a free group.

---

**Exercise 14.5 (Van Kampen's Theorem).** Use van Kampen's theorem to compute $\pi_1$ of:

(a) The torus $T^2$. (Use the CW structure with one 0-cell, two 1-cells $a, b$, and one 2-cell attached via $aba^{-1}b^{-1}$.)

(b) The Klein bottle $K$. (CW structure as $T^2$ but with the 2-cell attached via $abab^{-1}$.)

(c) $\mathbb{RP}^2$. (One 0-cell, one 1-cell, one 2-cell attached via $a^2$.)

(d) The connected sum $T^2 \# T^2$ (the genus-2 surface). (CW structure with one 0-cell, four 1-cells, and one 2-cell attached via $[a_1, b_1][a_2, b_2] = a_1 b_1 a_1^{-1} b_1^{-1} a_2 b_2 a_2^{-1} b_2^{-1}$.)

---

**Exercise 14.6 (Products and $\pi_1$).** Prove that $\pi_1(X \times Y, (x_0, y_0)) \cong \pi_1(X, x_0) \times \pi_1(Y, y_0)$.

(a) Define a homomorphism $\phi : \pi_1(X \times Y) \to \pi_1(X) \times \pi_1(Y)$ using the two projections.

(b) Define its inverse. Show they are mutually inverse group homomorphisms.

(c) Conclude $\pi_1(T^2) = \mathbb{Z} \times \mathbb{Z}$ directly from $T^2 = S^1 \times S^1$.

---

**Exercise 14.7 (Fundamental Group and Homotopy Equivalence).** Show that homotopy equivalent spaces have isomorphic fundamental groups.

(a) If $f : X \to Y$ is a homotopy equivalence with homotopy inverse $g$, show $f_*$ and $g_*$ are inverse group isomorphisms.

(b) Conclude that $\pi_1$ is a homotopy invariant.

(c) Show that $\mathbb{R}^2 \setminus \{0\}$ and $S^1$ have the same fundamental group by exhibiting a homotopy equivalence.

---

**Exercise 14.8 (Covering Spaces: Construction).** Describe the covering space corresponding to each subgroup.

(a) The trivial subgroup $\{0\} \subseteq \pi_1(S^1) = \mathbb{Z}$. Describe the total space and covering map.

(b) The subgroup $n\mathbb{Z} \subseteq \mathbb{Z}$ for $n \geq 2$. Describe the $n$-sheeted covering.

(c) The subgroup $\langle a^2, b \rangle \subseteq F_2 = \pi_1(S^1 \vee S^1)$. Describe the covering as a graph.

---

**Exercise 14.9 (Path Lifting).** Let $p : \mathbb{R} \to S^1$ be the covering map $p(t) = e^{2\pi it}$.

(a) Find the unique lift of the loop $\gamma(t) = e^{2\pi i t}$ (going around once) starting at $0 \in \mathbb{R}$.

(b) Find the unique lift of $\gamma^n(t) = e^{2\pi int}$ (going around $n$ times) starting at $0$.

(c) Compute the winding number of $\gamma \cdot \overline{\gamma}$ (a loop traversed then reversed). What does this say about the fundamental group?

---

**Exercise 14.10 (Deck Transformations).** Let $p : \mathbb{R} \to S^1$ be the standard covering.

(a) Describe all deck transformations of this covering.

(b) Show the deck transformation group $\text{Deck}(\mathbb{R}/S^1) \cong \mathbb{Z}$.

(c) How does the action of $\pi_1(S^1) = \mathbb{Z}$ on the fiber $p^{-1}(1) = \mathbb{Z}$ relate to the deck transformations?

---

**Exercise 14.11 (Higher Homotopy Groups: Basic Properties).** 

(a) Show that $\pi_n(X \times Y, (x_0, y_0)) \cong \pi_n(X, x_0) \times \pi_n(Y, y_0)$ for all $n \geq 1$.

(b) Conclude $\pi_n(T^2) \cong \pi_n(S^1) \times \pi_n(S^1)$ for all $n$. Using $\pi_k(S^1) = 0$ for $k \geq 2$, what is $\pi_n(T^2)$ for $n \geq 2$?

(c) Show that $\pi_n$ is a functor from the category of based spaces to the category of groups.

---

**Exercise 14.12 (Eckmann-Hilton Argument).** Prove the Eckmann-Hilton lemma.

(a) Let $(G, *, e)$ and $(G, \bullet, e)$ be two group structures on the same set $G$ with the same identity $e$, satisfying the interchange law $(a * b) \bullet (c * d) = (a \bullet c) * (b \bullet d)$ for all $a, b, c, d \in G$.

(b) Show $* = \bullet$: compute $a * b = (a \bullet e) * (e \bullet b) = (a * e) \bullet (e * b) = a \bullet b$.

(c) Show the operation is commutative: compute $a * b = (e \bullet a) * (b \bullet e)$.

(d) Explain how this applies to $\pi_2(X, x_0)$.

---

**Exercise 14.13 (Eilenberg-MacLane Spaces).** Verify or compute.

(a) Show $S^1$ is a $K(\mathbb{Z}, 1)$: compute $\pi_1(S^1) = \mathbb{Z}$ and argue $\pi_k(S^1) = 0$ for $k \geq 2$ using the long exact sequence of the covering $\mathbb{R} \to S^1$.

(b) Show $\mathbb{CP}^\infty$ is a $K(\mathbb{Z}, 2)$. (Use the fibration $S^1 \to S^\infty \to \mathbb{CP}^\infty$ and the fact that $S^\infty$ is contractible.)

(c) What is $K(\mathbb{Z}/2\mathbb{Z}, 1)$? (Answer: $\mathbb{RP}^\infty$. Verify $\pi_1(\mathbb{RP}^\infty) = \mathbb{Z}/2\mathbb{Z}$.)

---

**Exercise 14.14 (Long Exact Sequence of a Fibration).** Apply the long exact sequence.

(a) To the path-loop fibration $\Omega X \to PX \to X$ (where $PX$ is contractible): derive the isomorphism $\pi_n(X) \cong \pi_{n-1}(\Omega X)$.

(b) To the covering $\mathbb{R} \to S^1$: derive $\pi_n(S^1) = 0$ for $n \geq 2$.

(c) To the fibration $S^1 \to S^3 \to S^2$ (Hopf): derive $\pi_3(S^2) \cong \pi_3(S^3) = \mathbb{Z}$.

---

**Exercise 14.15 (The Hopf Fibration).** Work through the Hopf fibration.

(a) In complex coordinates, $S^3 = \{(z_1, z_2) \in \mathbb{C}^2 : |z_1|^2 + |z_2|^2 = 1\}$. Define $p(z_1, z_2) = [z_1 : z_2] \in \mathbb{CP}^1 \cong S^2$. Show $p$ is well-defined and surjective.

(b) Compute the fiber $p^{-1}([1:0]) = \{(e^{i\theta}, 0)\} \cong S^1$.

(c) Show that the Hopf fibration is not trivial: $S^3 \not\cong S^1 \times S^2$. (Compare $\pi_3(S^3) = \mathbb{Z}$ with $\pi_3(S^1 \times S^2) = \pi_3(S^2) = \mathbb{Z}$... hmm. Actually compare $H_*(S^3)$ with $H_*(S^1 \times S^2)$: the latter has non-trivial $H_1$, the former does not.)

---

**Exercise 14.16 (Hurewicz Theorem Application).** Use Hurewicz to compute.

(a) Compute $\pi_2(S^2)$ using Hurewicz ($S^2$ is simply connected and $H_2(S^2) = \mathbb{Z}$).

(b) Compute $\pi_3(S^3)$ using Hurewicz ($S^3$ is 2-connected and $H_3(S^3) = \mathbb{Z}$).

(c) Compute $H_2(S^1 \vee S^2)$. (Hint: use van Kampen for $\pi_1$ and Mayer-Vietoris or the long exact sequence for $H_2$. Then check Hurewicz doesn't immediately apply since $S^1 \vee S^2$ is not simply connected.)

---

**Exercise 14.17 (Compute $\pi_1$ from a CW Structure).** For each CW complex, use van Kampen to compute $\pi_1$.

(a) $\mathbb{RP}^3$: CW structure with one cell in dimensions 0, 1, 2, 3. The 2-cell attaches via $a^2$; the 3-cell doesn't affect $\pi_1$. Compute $\pi_1(\mathbb{RP}^3)$.

(b) The lens space $L(p, q)$: obtained from $S^3$ by a certain gluing. For simplicity, $L(p, 1)$: one 0-cell, one 1-cell, one 2-cell (attached via $a^p$), one 3-cell. Compute $\pi_1(L(p,1))$.

(c) The complement of the trefoil knot in $S^3$. The fundamental group is $\langle a, b \mid a^2 = b^3 \rangle$. Identify this group (it is a well-known group — which one?).

---

**Exercise 14.18 (Fiber Bundles).** Verify that the following are fiber bundles and identify the fiber.

(a) The projection $\pi : S^n \to \mathbb{RP}^n$ (the antipodal quotient map). Fiber?

(b) The map $p : S^{2n+1} \to \mathbb{CP}^n$ given by $p(z_0, \ldots, z_n) = [z_0 : \cdots : z_n]$. Fiber?

(c) The tangent bundle $TM \to M$ of a smooth manifold $M$. Fiber?

---

**Exercise 14.19 (Simply Connected Spaces).** Show the following spaces are simply connected.

(a) $S^n$ for $n \geq 2$.

(b) The infinite-dimensional real projective space... wait, $\mathbb{RP}^\infty$ is NOT simply connected. Instead: show $S^\infty$ (infinite-dimensional sphere) is contractible.

(c) $SU(n)$ for $n \geq 2$ (the special unitary group). (Use the fibration $SU(n-1) \to SU(n) \to S^{2n-1}$ and the long exact sequence.)

---

**Exercise 14.20 (Homotopy Groups of Lie Groups).** Lie groups are topological spaces with group structure.

(a) Show $\pi_1(S^1) = \mathbb{Z}$ directly from the fact that $S^1 = U(1)$ is a Lie group (every Lie group has $\pi_1$ abelian; for $U(1)$, the universal cover is $\mathbb{R}$).

(b) Show $\pi_1(SO(3)) = \mathbb{Z}/2\mathbb{Z}$ using the covering $S^3 = SU(2) \to SO(3)$.

(c) State the relationship $\pi_3(G) = \mathbb{Z}$ for any simple compact Lie group $G$. This follows from the long exact sequence and the fact that $G$ contains $SU(2) \cong S^3$ as a subgroup.

---

**Exercise 14.21 (Whitehead's Theorem).** 

(a) State Whitehead's theorem: a weak homotopy equivalence between CW complexes is a homotopy equivalence.

(b) Find a weak homotopy equivalence between spaces that is not a homotopy equivalence (must involve non-CW complexes; the "Warsaw circle" is a standard example).

(c) Explain why Whitehead's theorem is important for the relationship between model categories and classical homotopy theory.

---

**Exercise 14.22 (The Postnikov Tower).** 

(a) Describe the Postnikov tower of $S^2$: the spaces $P_1(S^2)$, $P_2(S^2)$, $P_3(S^2)$, and the fibrations between them.

(b) $P_1(S^2) = *$ (since $S^2$ is simply connected). $P_2(S^2) = K(\mathbb{Z}, 2) = \mathbb{CP}^\infty$ (since $\pi_2(S^2) = \mathbb{Z}$). Describe the fibration $P_3(S^2) \to P_2(S^2)$ and identify its fiber.

(c) What k-invariant characterizes the fibration $P_3(S^2) \to P_2(S^2)$? (It is an element of $H^4(\mathbb{CP}^\infty; \mathbb{Z}) = \mathbb{Z}$.)

---

**Exercise 14.23 (Homotopy Theory in HoTT).** Translate classical constructions into HoTT.

(a) The loop space $\Omega(X, x_0)$ in HoTT: define it as a type and show it has a group structure.

(b) The fundamental group $\pi_1(A, a) = \|\Omega(A, a)\|_0$ (the 0-truncation of the loop space). Explain why the truncation is needed.

(c) The long exact sequence of a fibration: in HoTT, this follows from the fiber sequence $F \to E \to B$ by applying $\pi_n$ repeatedly. State the sequence for the universal cover of $S^1$.

---

**Exercise 14.24 (Research: Homotopy Groups of Spheres).** 

(a) Look up and list $\pi_n(S^2)$ for $n = 2, 3, 4, 5, 6$.

(b) The Brunerie number: in HoTT, Brunerie proved $\pi_4(S^3) = \mathbb{Z}/n\mathbb{Z}$ for some $n$. The proof left open what $n$ was. It was later shown by a computer calculation that $n = 2$. What does this say about the power of computer-verified mathematics?

(c) The EHP sequence relates $\pi_n(S^k)$, $\pi_n(S^{2k-1})$, and $\pi_n(S^{2k})$. Look it up and explain the intuition for what it says.

---

**Exercise 14.25 (Van Kampen for the Hopf Map).** 

(a) The Hopf fibration $p : S^3 \to S^2$ is a map of degree 1 on $H_3$. What does this mean?

(b) The homotopy class $[p] \in \pi_3(S^2) = \mathbb{Z}$ is the generator. The element $n[p]$ corresponds to a map of Hopf invariant $n^2$... wait, the correct statement is: the Hopf invariant of $n[p]$ is $n$, not $n^2$. Look up the definition of the Hopf invariant and explain why the Hopf map has Hopf invariant $1$.

(c) The Hopf invariant 1 problem (Adams, 1960): for which $n$ does there exist a map $S^{2n-1} \to S^n$ of Hopf invariant 1? The answer is $n \in \{1, 2, 4, 8\}$, corresponding to the real numbers, complex numbers, quaternions, and octonions. State the connection to the existence of normed division algebras.

# Exercises

---

**Exercise 14.1 (Homotopy is an Equivalence Relation).** Verify the three properties.

(a) *Reflexivity:* Show $f \simeq f$ for any continuous $f : X \to Y$.

(b) *Symmetry:* If $H$ is a homotopy from $f$ to $g$, construct a homotopy from $g$ to $f$.

(c) *Transitivity:* If $H$ is a homotopy from $f$ to $g$ and $K$ is a homotopy from $g$ to $h$, construct a homotopy from $f$ to $h$. Show the concatenated map is continuous (hint: use the pasting lemma for continuous maps).

---

**Exercise 14.2 (Products and $\pi_1$).** Show that $\pi_1(X \times Y, (x_0, y_0)) \cong \pi_1(X, x_0) \times \pi_1(Y, y_0)$.

(a) Define maps $\phi : \pi_1(X \times Y) \to \pi_1(X) \times \pi_1(Y)$ and $\psi : \pi_1(X) \times \pi_1(Y) \to \pi_1(X \times Y)$.

(b) Show $\phi$ and $\psi$ are mutually inverse group homomorphisms.

(c) Conclude $\pi_1(T^2) = \pi_1(S^1) \times \pi_1(S^1) = \mathbb{Z} \times \mathbb{Z}$ directly from this.

---

**Exercise 14.3 (Fundamental Group of $\mathbb{RP}^2$).** Use the covering space $S^2 \to \mathbb{RP}^2$.

(a) $S^2$ is simply connected: $\pi_1(S^2) = 0$ (the 2-sphere has no non-trivial loops). State why.

(b) The covering $S^2 \to \mathbb{RP}^2$ is a 2-sheeted cover. From the classification theorem, what does this say about $\pi_1(\mathbb{RP}^2)$?

(c) Show that the deck transformation group (swapping the two sheets) is $\mathbb{Z}/2\mathbb{Z}$, and that $\pi_1(\mathbb{RP}^2) \cong \mathbb{Z}/2\mathbb{Z}$.

---

**Exercise 14.4 (Covering Spaces of the Figure Eight).** $S^1 \vee S^1$ has fundamental group $F_2$ (free group on 2 generators $a, b$).

(a) Describe all subgroups of $F_2$ of index 2 (there are 3 of them: $\langle a^2, b^2, ab \rangle$, $\langle a^2, b, ab^{-1} \rangle$, and one more — find it).

(b) For each subgroup of index 2, describe the corresponding 2-sheeted covering space as a graph with labeled edges.

(c) How many connected 3-sheeted covers of $S^1 \vee S^1$ are there (up to isomorphism)? (There are 7 — you don't need to list all, just explain the counting.)

---

**Exercise 14.5 (Path-Loop Fibration).** Apply the long exact sequence to $\Omega X \hookrightarrow PX \to X$.

(a) State why $PX$ (the based path space) is contractible.

(b) The long exact sequence includes $\pi_n(PX) \to \pi_n(X) \to \pi_{n-1}(\Omega X) \to \pi_{n-1}(PX)$. Using that $\pi_k(PX) = 0$ for all $k$, derive $\pi_n(X) \cong \pi_{n-1}(\Omega X)$.

(c) What does this say about the relationship between $\pi_2(S^2)$ and $\pi_1(\Omega S^2) = \pi_1(S^1)$?

---

**Exercise 14.6 (The Hopf Fibration).** The Hopf fibration $p : S^3 \to S^2$.

(a) In complex coordinates, $S^3 = \{(z_1, z_2) \in \mathbb{C}^2 : |z_1|^2 + |z_2|^2 = 1\}$ and $S^2 = \mathbb{CP}^1$. Define $p(z_1, z_2) = [z_1 : z_2]$ (the complex projective coordinate). Show that $p$ is well-defined and surjective.

(b) Compute the fiber $p^{-1}([1:0]) = \{(e^{i\theta}, 0) : \theta \in [0,2\pi]\} \cong S^1$. Verify it's a circle.

(c) Use the long exact sequence to show $\pi_3(S^2) = \mathbb{Z}$ (see the computation in Section 5).

(d) Show $\pi_2(S^3) = 0$ and $\pi_1(S^3) = 0$ (using the fact that $S^3$ is a Lie group, or directly from cellular structure).

---

**Exercise 14.7 (Eilenberg-MacLane Spaces).** 

(a) Verify that $S^1$ is a $K(\mathbb{Z},1)$ space: compute $\pi_1(S^1) = \mathbb{Z}$ and argue (using covering spaces and the universal cover) that $\pi_k(S^1) = 0$ for $k \geq 2$.

(b) For $K(\mathbb{Z},2)$: what is $\pi_2(\mathbb{CP}^\infty)$? (It should be $\mathbb{Z}$.)

(c) The Postnikov tower of $S^2$ has $P_1(S^2) = \{*\}$ (trivially connected) and $P_2(S^2) = K(\mathbb{Z},2) = \mathbb{CP}^\infty$. Describe the fibration $P_3(S^2) \to P_2(S^2) = \mathbb{CP}^\infty$ and identify the fiber.

---

**Exercise 14.8 (Homotopy Invariance of $\pi_1$).** Show that if $f : X \to Y$ is a homotopy equivalence, then $f_* : \pi_1(X, x_0) \to \pi_1(Y, f(x_0))$ is an isomorphism.

(a) Let $g : Y \to X$ be a homotopy inverse, with homotopy $H : g \circ f \simeq \mathsf{id}_X$. Show that $g_* \circ f_*$ is an isomorphism (up to conjugation by the path traced by $H(x_0, -)$).

(b) Similarly show $f_* \circ g_*$ is an isomorphism.

(c) Conclude $f_*$ is an isomorphism.

---

**Exercise 14.9 (Van Kampen for the Klein Bottle).** The Klein bottle $K$ is built from a square with identifications: $(x,0) \sim (x,1)$ (same direction) and $(0,y) \sim (1, 1-y)$ (opposite direction).

(a) Compute $\pi_1(K)$ using Van Kampen's theorem (decompose like the torus, but with different attaching maps).

(b) The result should be $\langle a, b \mid abab^{-1} = e \rangle$. Is this abelian? (The torus gives $\langle a,b \mid aba^{-1}b^{-1} \rangle = \mathbb{Z}^2$, which is abelian. The Klein bottle gives a non-abelian group.)

(c) Find the abelianization of $\pi_1(K)$: what is $H_1(K) = \pi_1(K)^{ab}$?

---

**Exercise 14.10 (Research: Homotopy Groups of Spheres).** Look up the homotopy groups of spheres.

(a) List $\pi_n(S^2)$ for $n = 2, 3, 4, 5, 6, 7$.

(b) What patterns do you notice? Which groups are $\mathbb{Z}$, which are finite?

(c) What is the status of computing $\pi_n(S^2)$ for large $n$? What computational methods are used?

(d) The Brunerie number: Brunerie proved in HoTT that $\pi_4(S^3) = \mathbb{Z}/n$ for some $n$, and the question of whether $n = 2$ required a computer calculation. What was the result?

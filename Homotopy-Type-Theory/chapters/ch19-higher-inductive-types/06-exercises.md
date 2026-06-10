# Chapter 19 Exercises: Higher Inductive Types

---

## Section 1: The Interval and Circle

**Exercise 1.1 (Interval gives funext).** Use the interval type $\mathbb{I}$ to prove function extensionality. Given $f, g : A \to B$ and $H : \prod_{x:A} f(x) = g(x)$:
1. Define a map $k : A \times \mathbb{I} \to B$ using the $\mathbb{I}$-eliminator
2. Extract a path $f = g$ from $k$
3. State why the interval construction works and the standard J-based approach doesn't give funext

**Exercise 1.2 (Circle loops).** Define the "winding number $n$" map $w_n : S^1 \to S^1$ using the circle's non-dependent eliminator. Show:
- $w_0 \sim \mathsf{const}_\mathsf{base}$ (constant map)
- $w_1 \sim \mathsf{id}_{S^1}$ (identity map)
- $w_m \circ w_n \sim w_{m+n}$ (composition adds winding numbers)

**Exercise 1.3 (Circle recursion).** The double cover of $S^1$ is constructed as follows. Define a type family $P : S^1 \to \mathsf{Type}$ by:
- $P(\mathsf{base}) :\equiv \mathsf{Bool}$
- $\mathsf{transport}^P(\mathsf{loop}) :\equiv \mathsf{neg}$ (the negation equivalence $\mathsf{Bool} \simeq \mathsf{Bool}$)

Show that the total space $\sum_{x:S^1} P(x)$ has the right structure for a double cover (two sheets, connected, the covering map $\pi : \sum_{x:S^1}P(x) \to S^1$ is the first projection).

**Exercise 1.4.** Let $f : S^1 \to X$ and $g : S^1 \to X$ be two maps of circles. Show that $f \sim g$ (they're homotopic) iff $f(\mathsf{base}) = g(\mathsf{base})$ and $\mathsf{ap}_f(\mathsf{loop}) \cdot H(g(\mathsf{base})) = H(f(\mathsf{base})) \cdot \mathsf{ap}_g(\mathsf{loop})$ for any path $H$ between the basepoints.

---

## Section 2: Suspensions and Spheres

**Exercise 2.1 (Suspension of Bool is circle).** Show that $\Sigma \mathsf{Bool} \simeq S^1$ by:
1. Constructing $f : \Sigma \mathsf{Bool} \to S^1$ using the suspension eliminator
2. Constructing $g : S^1 \to \Sigma \mathsf{Bool}$ using the circle eliminator
3. Showing $f \circ g \sim \mathsf{id}$ and $g \circ f \sim \mathsf{id}$

**Exercise 2.2.** Show that $\Sigma S^1 \simeq S^2$ by working through the suspension eliminator for $S^2$ and constructing maps both ways.

**Exercise 2.3 (Join formula).** Verify that $A * \mathbf{1} \simeq \Sigma A$ (the join with a point is the suspension). Construct the equivalence explicitly using the universal properties.

**Exercise 2.4.** Show that $S^0 * S^0 \simeq S^1$ (the join of two two-point sets is the circle). Check this by working out all the constructors and meridians.

---

## Section 3: Pushouts

**Exercise 3.1 (Coproduct is pushout).** Verify that $A \sqcup_\mathbf{0} B = A + B$ (the pushout along the empty type is the coproduct).

**Exercise 3.2 (Quotient as pushout).** A quotient $A/R$ by an equivalence relation $R : A \to A \to \mathsf{Prop}$ can be constructed as a pushout. Define:
- The "relation type" $\sum_{x, y : A} R(x, y)$
- The two projections $\pi_1, \pi_2 : \sum_{x,y:A}R(x,y) \to A$
- Show the pushout $A \sqcup_{\sum R} A$ has the right universal property for a quotient

**Exercise 3.3 (Van Kampen computation).** Use the van Kampen theorem to compute $\pi_1$ of:
1. The torus $T^2$: decompose as two cylinders glued along their boundary circles
2. The figure-eight $S^1 \vee S^1$: the wedge of two circles
3. The Klein bottle: identify two circles with one orientation reversed

**Exercise 3.4.** Show that the pushout $A \sqcup_A B$ (along the identity map $f = \mathsf{id}_A$) satisfies $A \sqcup_A B \simeq B$. This is the "trivial pushout."

---

## Section 4: Truncations

**Exercise 4.1 (Propositional truncation properties).** Show:
1. $\|\mathbf{0}\|_{-1} \simeq \mathbf{0}$ (truncation of empty is empty)
2. $\|\mathbf{1}\|_{-1} \simeq \mathbf{1}$ (truncation of unit is unit)
3. $\|A + B\|_{-1} \simeq \|A\|_{-1} \vee \|B\|_{-1}$ (propositional truncation distributes over coproducts up to propositional or)

**Exercise 4.2.** Show that $\|A \times B\|_{-1} \simeq \|A\|_{-1} \times \|B\|_{-1}$ (propositional truncation distributes over products).

**Exercise 4.3 (Non-commutativity of truncation and dependent sums).** Show that in general:
$$\|\sum_{x:A} B(x)\|_{-1} \not\simeq \sum_{x:\|A\|_{-1}} \|B(x)\|_{-1}$$
Give a specific counterexample.

**Exercise 4.4.** Show: if $A$ is already a proposition, then $\|A\|_{-1} \simeq A$.

**Exercise 4.5 (Set truncation of circle).** Show that $\|S^1\|_0 \simeq \mathbf{1}$ (the set truncation of the circle is the unit type). (Hint: the circle is connected — any two elements are merely equal.)

---

## Section 5: Eilenberg-MacLane Spaces

**Exercise 5.1.** Define $K(\mathbb{Z}/2\mathbb{Z}, 1)$ explicitly as a HIT. Identify its constructors, path constructors, and truncation conditions.

**Exercise 5.2.** Show that the loop space of $K(G, n)$ at its basepoint is $K(G, n-1)$:
$$\Omega(K(G, n)) \simeq K(G, n-1)$$
for $n \geq 1$.

**Exercise 5.3 (K(Z,1) = S^1).** Using the result $\pi_1(S^1) = \mathbb{Z}$ (Theorem 19.6) and the fact that $\pi_k(S^1) = 0$ for $k \geq 2$ (given as a fact from synthetic homotopy theory), show that $S^1$ is a $K(\mathbb{Z}, 1)$: a connected 1-type with fundamental group $\mathbb{Z}$.

**Exercise 5.4 (Maps to K(G, 1)).** Show that homotopy classes of maps $[X, K(G, 1)]$ form a group isomorphic to $\mathsf{Hom}(\pi_1(X), G)$ (group homomorphisms from $\pi_1(X)$ to $G$). This is the classification of $G$-bundles over $X$.

---

## Section 6: Research-Level Exercises

**Exercise 6.1 (Torus as HIT).** Define the torus $T^2$ directly as a HIT (one point, two loops, and a 2-cell for the commutativity relation $aba^{-1}b^{-1}$). Compute $\pi_1(T^2) = \mathbb{Z}^2$ using this HIT structure.

**Exercise 6.2 (Real projective plane).** Define $\mathbb{RP}^2$ as a HIT with one point, one loop, and a 2-cell saying $\mathsf{loop}^2 = \mathsf{refl}$. Compute $\pi_1(\mathbb{RP}^2) = \mathbb{Z}/2\mathbb{Z}$.

**Exercise 6.3 (Brunerie's computation setup).** The computation $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ uses:
1. The Hopf fibration $S^1 \to S^3 \to S^2$
2. The long exact sequence of the fibration
3. The computation $\pi_3(S^2) = \mathbb{Z}$ (from the Hopf map)
4. Freudenthal's theorem

Set up the long exact sequence for the Hopf fibration and show how it gives $\pi_3(S^2) = \mathbb{Z}$ (assuming $\pi_k(S^3) = 0$ for $k \leq 2$).

**Exercise 6.4 (HITs and CW complexes).** The classifying theorem: every type in HoTT (with HITs) has the homotopy type of a CW complex (a space built by attaching cells). Describe how:
1. Point constructors correspond to 0-cells
2. Path constructors correspond to 1-cells
3. 2-path constructors correspond to 2-cells
4. Truncation constructors correspond to "filling in all higher cells"

**Exercise 6.5 (Stabilization).** The suspension-loop adjunction:
$$[X, \Omega Y] \simeq [\Sigma X, Y]$$
follows from the HIT definitions. Prove this using the universal properties of $\Sigma$ (pushout universal property) and $\Omega$ (loop space as a pullback). This is the type-theoretic version of the classical suspension-loop adjunction.

# Exercises

---

**Exercise 12.1 (2-Category of Groupoids).** Show that groupoids form a strict 2-category **Grpd**:

(a) What are the objects, 1-cells, and 2-cells of **Grpd**?

(b) Define vertical composition and horizontal composition of 2-cells.

(c) Verify the interchange law: $(\beta' \circ \beta) \star (\alpha' \circ \alpha) = (\beta' \star \alpha') \circ (\beta \star \alpha)$.

(d) Why is this a *strict* 2-category (unlike the bicategory of spans)?

---

**Exercise 12.2 (Fundamental Groupoids).** Compute the fundamental groupoid $\Pi_1(X)$ for:

(a) $X = S^1$ (the circle): describe the morphism sets $\mathsf{Hom}(x, y)$ for points $x, y \in S^1$. How many path-homotopy classes go from $x$ to $y$?

(b) $X = [0,1]$ (the interval): is $\Pi_1([0,1])$ equivalent (as a groupoid) to a simpler groupoid? What is it?

(c) $X = \mathbb{R}^n$: since $\mathbb{R}^n$ is contractible, what is $\Pi_1(\mathbb{R}^n)$?

(d) $X = S^1 \sqcup S^1$ (two disjoint circles): describe $\Pi_1(X)$ and explain how it encodes the connected components.

---

**Exercise 12.3 (Groupoid Laws in MLTT).** In MLTT, derive the groupoid laws for path concatenation using J.

(a) **Left unit law:** Prove $\mathsf{left\_unit}(p) : \mathsf{refl} \cdot p = p$ for $p : a = b$. (Hint: use J on $p$, reducing to $\mathsf{refl} \cdot \mathsf{refl} = \mathsf{refl}$, which follows by computation.)

(b) **Right unit law:** Prove $\mathsf{right\_unit}(p) : p \cdot \mathsf{refl} = p$.

(c) **Associativity:** Prove $\mathsf{assoc}(p,q,r) : (p \cdot q) \cdot r = p \cdot (q \cdot r)$.

(d) Why are these *propositional* equalities (elements of identity types) rather than *definitional* equalities? What would it mean for them to be definitional?

---

**Exercise 12.4 (2-Groupoid of a Space).** The fundamental 2-groupoid $\Pi_2(X)$:

(a) Define $\Pi_2(X)$: what are its objects, 1-cells, and 2-cells? What equivalence relation do you need at each level?

(b) Verify that 2-cells in $\Pi_2(X)$ are invertible (every homotopy between paths has an inverse).

(c) For $X = S^2$ (the 2-sphere): $\pi_1(S^2) = 0$ (trivial). What does this mean for the 1-cells of $\Pi_2(S^2)$? What is $\pi_2(S^2)$?

(d) Explain why $\Pi_2(S^2)$ cannot be a strict 2-groupoid that captures the full homotopy type of $S^2$. (Hint: $\pi_3(S^2) = \mathbb{Z}$.)

---

**Exercise 12.5 (Nerve of a Group).** The nerve of a group $G$ (viewed as a one-object category):

(a) Describe the $n$-simplices of $N(G)$ explicitly. What is $N(G)_0$? $N(G)_1$? $N(G)_2$?

(b) Describe the face maps $d_0, d_1, d_2 : N(G)_2 \to N(G)_1$. Which face is "composition"?

(c) Show that $N(G)$ is a Kan complex: verify that the horn $\Lambda^2_0 \to N(G)$ always extends to $\Delta^2 \to N(G)$. What does this use about $G$?

(d) Compute $\pi_1(|N(G)|)$ (the fundamental group of the geometric realization of $N(G)$). It should be $G$.

---

**Exercise 12.6 (Inner Horn Filling in a Quasi-Category).** Let $X$ be a quasi-category.

(a) Write out explicitly what a map $\Lambda^2_1 \to X$ consists of. Show that an extension to $\Delta^2 \to X$ provides a "composite."

(b) Write out what a map $\Lambda^3_1 \to X$ consists of (it's a "composable triple" with one composite missing). What does filling it give?

(c) Show that in the nerve $N(\mathcal{C})$ of an ordinary category $\mathcal{C}$, all inner horns fill uniquely. (Hint: the filling is determined by composition in $\mathcal{C}$.)

(d) Show that the outer horn $\Lambda^2_0 \to N(\mathcal{C})$ does not always fill unless $\mathcal{C}$ is a groupoid.

---

**Exercise 12.7 (Eckmann-Hilton).** Prove the Eckmann-Hilton argument in type theory.

(a) Let $A$ be a type with $a : A$, and let $\Omega A = (a =_A a)$ and $\Omega^2 A = (\mathsf{refl} =_{\Omega A} \mathsf{refl})$.

(b) Show that $\Omega^2 A$ has two composition operations: vertical $\cdot_v$ (concatenation of 2-paths) and horizontal $\cdot_h$ (from whiskering/horizontal composition).

(c) Show these operations satisfy the interchange law: $(p \cdot_v q) \cdot_h (r \cdot_v s) = (p \cdot_h r) \cdot_v (q \cdot_h s)$.

(d) Using the Eckmann-Hilton argument, conclude that $\cdot_v = \cdot_h$ and both are commutative. This shows $\pi_2(A)$ is abelian.

---

**Exercise 12.8 (h-Levels).** Classify the following types by their h-level.

(a) $\mathbf{1}$ (the unit type): what is its h-level? (It should be $-2$, contractible.)

(b) $\mathbf{2} = \mathbf{1} + \mathbf{1}$ (the booleans): what is its h-level?

(c) $\mathbb{N}$ (the natural numbers): what is its h-level? (Is it a set, a 1-type, or something higher?)

(d) $S^1$ (the circle type, as a HIT): what is its h-level? (It has $\pi_1(S^1) = \mathbb{Z}$, so it's not a set. Is it a 1-type or higher?)

---

**Exercise 12.9 (The Homotopy Hypothesis for 1-Types).** Prove the homotopy hypothesis for 1-truncated types.

(a) Given a 1-type $A$ (a type where all identity types are sets), show that $A$'s path structure forms a groupoid $\mathcal{G}(A)$.

(b) Given a groupoid $\mathcal{G}$, sketch how to construct a 1-type $B\mathcal{G}$ (the "classifying type" of $\mathcal{G}$) such that $\pi_0(B\mathcal{G}) = \pi_0(\mathcal{G})$ and $\pi_1(B\mathcal{G}, b) = \mathsf{Aut}_\mathcal{G}(b)$ for each $b$.

(c) Informally: why are these constructions mutually inverse? What does this say about the homotopy hypothesis for 1-types?

---

**Exercise 12.10 (Research: Current Status of the Homotopy Hypothesis).** Investigate the current state of the homotopy hypothesis:

(a) What did Grothendieck conjecture, precisely? What models of weak ∞-groupoids did he consider?

(b) State Simpson's theorem (1998) that strict ∞-groupoids are insufficient. What homotopy types are missed?

(c) What is the status of the equivalence between various definitions of weak ∞-groupoids (Batanin, Leinster, etc.) and Kan complexes?

(d) In what sense is the homotopy hypothesis "proved" and in what sense is it still open? Identify at least one specific open question.

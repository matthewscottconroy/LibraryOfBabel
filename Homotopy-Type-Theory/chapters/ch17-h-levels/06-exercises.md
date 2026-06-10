# Chapter 17 Exercises: H-Levels and Truncations

---

## Section 1: Contractible Types

**Exercise 1.1.** Prove that the empty type $\mathbf{0}$ is not contractible. What is its h-level?

**Exercise 1.2.** Prove directly that $\mathbf{1}$ is contractible by giving the center of contraction and the contracting homotopy.

**Exercise 1.3.** Let $A$ and $B$ be contractible. Prove that $A \times B$ is contractible by finding the center and contracting homotopy explicitly.

**Exercise 1.4.** Let $P : A \to \mathsf{Type}$ be a type family such that $A$ is contractible (with center $c$) and $P(c)$ is contractible. Prove that $\sum_{x:A} P(x)$ is contractible.

**Exercise 1.5.** Prove that if $f : A \to B$ is an equivalence and $A$ is contractible, then $B$ is contractible. (Hint: transport the contracting homotopy along the equivalence.)

**Exercise 1.6.** Show that the fiber $\mathsf{fib}_f(b) = \sum_{x:A}(f(x) = b)$ is contractible for all $b : B$ iff $f$ is an equivalence. (One direction: if $f$ is an equivalence with inverse $g$, show each fiber has center $(g(b), \varepsilon_b)$.)

---

## Section 2: Mere Propositions

**Exercise 2.1.** Prove that $\mathbf{0}$ is a proposition (the condition holds vacuously since $\mathbf{0}$ has no elements).

**Exercise 2.2.** Prove that $\mathbf{1}$ is a proposition. Then prove it is contractible using Exercise 1.2.

**Exercise 2.3.** Prove: if $A$ is a proposition and $a : A$, then $A$ is contractible with center $a$.

**Exercise 2.4.** Prove: if $A$ is a proposition, then $A \to A$ is contractible (the only function $A \to A$ that a proposition supports is, up to homotopy, the identity).

**Exercise 2.5.** Let $A$ be a proposition. Prove that $A \times \neg A \simeq \mathbf{0}$, where $\neg A = A \to \mathbf{0}$.

**Exercise 2.6.** Prove that $\mathsf{isProp}(A)$ is itself a proposition. (Hint: let $p, q : \mathsf{isProp}(A)$. For any $x, y : A$: $p(x)(y)$ and $q(x)(y)$ are both paths $x = y$. Use $p$ to show they're equal.)

**Exercise 2.7.** Show that $A$ is a proposition iff the diagonal map $\delta : A \to A \times A$ defined by $\delta(a) = (a, a)$ is an equivalence.

---

## Section 3: H-Sets

**Exercise 3.1.** Show that $\mathsf{Bool}$ is a set by exhibiting paths between any two equal elements (there's only one: true = true is $\mathsf{refl}$, false = false is $\mathsf{refl}$) and showing there are no paths between true and false.

**Exercise 3.2 (Hedberg, worked through).** Prove Hedberg's theorem step by step:
1. Define: given decidable equality on $A$, construct a constant map $r : (x = y) \to (x = y)$ for each $x, y : A$.
2. Show that $r$ is "normalized": $r(p) = r(q)$ for all $p, q : x = y$.
3. Use the normalized $r$ to show that all paths $x = y$ are equal: $p = r(p) = r(q) = q$. (Justify each step carefully.)

**Exercise 3.3.** Show that $\mathbb{N}$ has decidable equality (by induction), and conclude it is a set.

**Exercise 3.4.** Show that if $A$ is a set, then $\mathsf{isSet}(A)$ is a proposition.

**Exercise 3.5.** Prove: if $A$ and $B$ are sets, then $A \to B$ is a set (using function extensionality).

**Exercise 3.6.** Prove: if $A$ is a set and $P : A \to \mathsf{Type}$ takes values in propositions, then $\sum_{x:A} P(x)$ is a set.

---

## Section 4: n-Types

**Exercise 4.1.** Prove the cumulativity theorem: if $A$ is an $n$-type, then $A$ is an $(n+1)$-type. (Induction on $n$, starting from the case $n = -2$.)

**Exercise 4.2.** Show that $S^1$ (the circle, defined in Chapter 19 as a HIT) is not a set. (Hint: exhibit a non-trivial loop at the basepoint — the path constructor of $S^1$ gives such a loop, and it's not equal to reflexivity.)

**Exercise 4.3.** Show that the universe $\mathsf{Type}_0$ is not a set. (Hint: by Univalence, $\mathsf{Bool} = \mathsf{Bool}$ in $\mathsf{Type}_0$ has at least two paths — the identity equivalence and the negation equivalence.)

**Exercise 4.4.** Prove: if $P : A \to \mathsf{Type}$ is a family of $n$-types and $A$ is an $n$-type, then $\sum_{x:A} P(x)$ is an $n$-type. (Induction on $n$, using the path characterization of $\Sigma$-types.)

**Exercise 4.5.** Show: $\pi_1(S^1, \mathsf{base}) = \mathbb{Z}$ implies that $S^1$ is a 1-type (h-level 1) but not a 0-type (set). (This requires computing $\pi_1$ using the techniques of Chapter 20, but formulate what needs to be proved.)

---

## Section 5: Truncations

**Exercise 5.1.** Show that $\|\mathbf{0}\| \simeq \mathbf{0}$ (the propositional truncation of the empty type is empty) and $\|\mathbf{1}\| \simeq \mathbf{1}$ (the propositional truncation of the unit type is the unit type).

**Exercise 5.2.** Show that $\|A\|_{-1} \simeq \mathbf{1}$ iff $A$ is nonempty (inhabited). (Use the universal property of propositional truncation.)

**Exercise 5.3.** Show that $\|A\|_0 \simeq A$ iff $A$ is already a set. (The set truncation is idempotent on sets.)

**Exercise 5.4 (Truncation and products).** Show that $\|A \times B\|_{-1} \simeq \|A\|_{-1} \times \|B\|_{-1}$. (Propositional truncation distributes over products.)

**Exercise 5.5.** Show that $\|A + B\|_{-1} \simeq \|A\|_{-1} \vee \|B\|_{-1}$ (where $\vee$ is propositional or). This is the propositional truncation of a sum.

**Exercise 5.6 (Strong vs weak existence).**
1. Give an example of a type $A$ and predicate $P : A \to \mathsf{Type}$ where $\sum_{x:A} P(x)$ is inhabited but computing an element of it is hard.
2. Show that $\|\sum_{x:A} P(x)\|$ is always a proposition.
3. Formulate the distinction between "there constructively exists" and "there merely exists" in the context of sorting: "for any list $l$, there constructively exists a sorted permutation" vs "there merely exists a sorted permutation."

---

## Section 6: Research-Level Exercises

**Exercise 6.1 (Subsingleton elimination).** A *subsingleton* is a type with at most one element (a proposition). Prove the *subsingleton elimination principle*: for any proposition $P$ and any type $A$, a function $A \to P$ factors uniquely through $\|A\|$.

**Exercise 6.2 (Propositional resizing).** Assume Propositional Resizing: every proposition in $\mathsf{Type}_{n+1}$ is equivalent to a proposition in $\mathsf{Type}_n$. Use this to show that the type of propositions $\mathsf{hProp}$ is a set.

**Exercise 6.3 (Unique choice).** Prove the *unique choice principle*: if $\sum_{x:A} P(x)$ is a proposition (i.e., at most one $x$ satisfies $P$), then $\sum_{x:A} P(x) \simeq \|\sum_{x:A} P(x)\|$. (The truncation doesn't lose information when the type is already a proposition.)

**Exercise 6.4 (Homotopy groups as groups).** For a type $A$ with base point $a : A$ and $n \geq 1$, define $\pi_n(A, a) = \|\Omega^n(A, a)\|_0$ as a set. Show that for $n = 1$, $\pi_1(A, a)$ has a group structure induced by path concatenation in $\Omega(A, a)$.

**Exercise 6.5 (Eilenberg-MacLane spaces).** An Eilenberg-MacLane space $K(G, n)$ for a group $G$ and $n \geq 1$ is characterized by:
$$\pi_k(K(G,n)) = \begin{cases} G & k = n \\ 0 & k \neq n \end{cases}$$
Show that any 1-type with $\pi_1 = G$ and $\pi_0 = \mathbf{1}$ (connected) is an Eilenberg-MacLane space $K(G, 1)$.

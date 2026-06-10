# Chapter 16: Identity Types and Paths

## Introduction

We have already encountered the identity type in Chapter 9 (MLTT). Now, with the homotopy-theoretic perspective from Chapters 12–15, we can understand what the identity type *is*: a type of *paths*. An element $p : a =_A b$ is a path from the point $a$ to the point $b$ in the space $A$.

This is not merely a metaphor. In Voevodsky's simplicial set model:
- Types are Kan complexes
- Terms are points of the Kan complex
- The identity type $a =_A b$ is (modeled by) the path space $\{p : [0,1] \to |A| \mid p(0) = a, p(1) = b\}$

In HoTT, we work with this intuition *synthetically*: we never mention the model, but we reason about types as if they were spaces, using only the rules of MLTT plus univalence.

This chapter develops the full structure of identity types: path induction, the groupoid laws, higher path structure, transport, and the crucial difference between HoTT and MLTT+UIP.

---

## 1. Paths in Type Theory

### 1.1 The Type of Paths

Recall from Chapter 9: given a type $A$ and terms $a, b : A$, the *identity type* $a =_A b$ has:
- **Introduction:** $\mathsf{refl}_a : a =_A a$ (the constant path at $a$)
- **Elimination (J):** From a proof involving only $\mathsf{refl}$, derive the general case

Under the homotopy interpretation:
- $A$ is a space
- $a, b : A$ are points
- $a =_A b$ is the *path space* from $a$ to $b$
- $p : a =_A b$ is a specific path

The key insight is that $a =_A b$ is a **type** (not a proposition). It can have many elements (many distinct paths), and those elements can themselves be related by elements of the identity type $p =_{a =_A b} q$ — which are *homotopies between paths*.

### 1.2 Path Spaces Are Spaces

In topology, for a space $X$ and points $x, y \in X$, the path space $P(X; x, y) = \{\gamma : [0,1] \to X \mid \gamma(0) = x, \gamma(1) = y\}$ is itself a topological space. It may be:
- Empty (if $x$ and $y$ are in different path components)
- Contractible (if $X$ is simply connected)
- Homotopy equivalent to $\Omega X$ (the loop space) — if $x = y$, paths from $x$ to $x$ are loops

The identity type $a =_A b$ mirrors all of this:
- If $a$ and $b$ are in different "path components," $a =_A b$ should be empty (the empty type, a contradiction)
- If $A$ is "contractible," every $a =_A b$ is contractible
- If $a = b$, the type $a =_A a$ is the loop space at $a$

### 1.3 Reflexivity as the Constant Path

$\mathsf{refl}_a : a =_A a$ is the *constant path* at $a$: a path that stays at $a$ for all time $t \in [0,1]$.

In the simplicial set model: $\mathsf{refl}_a$ is the *degenerate* 1-simplex on $a$ — the simplex $\sigma_0(a) \in X_1$ obtained by degenerating the vertex $a \in X_0$.

---

## 2. Path Induction: The J Eliminator

### 2.1 Stating J

The J rule says: to prove a property of *all* paths in $A$ starting from a fixed $a$, it suffices to prove it for the trivial path $\mathsf{refl}_a$.

**The J Eliminator:**
$$\frac{\Gamma \vdash a : A \quad \Gamma, b : A, p : a = b \vdash C : \mathsf{Type} \quad \Gamma \vdash d : C[a/b, \mathsf{refl}_a/p]}{\Gamma, b : A, p : a = b \vdash \mathsf{J}(C, d, b, p) : C}$$

with the computation rule $\mathsf{J}(C, d, a, \mathsf{refl}_a) \equiv d$.

### 2.2 The Topological Reading of J

In topology, the path space $P(X; a, -)$ (paths from $a$ to any point) is *contractible*: the map $P(X; a, -) \to X$ (evaluate at the endpoint) is a fibration, and the fiber over $a$ is the loop space $\Omega(X, a)$. If $X$ is "locally path-connected," the total space $P(X; a, -)$ has a contraction to the constant path at $a$.

**J says:** The "total path space" $\Sigma_{b:A}\, (a = b)$ is *contractible*. To prove this, we give a center $(a, \mathsf{refl}_a)$ and observe that every $(b, p)$ is connected to $(a, \mathsf{refl}_a)$ by a path — but the path is exactly $p$ itself! J is the formal version of this contractibility.

### 2.3 Dependence of J on the Endpoint

Why can't we prove things by inducting on the *source* alone? The answer is subtle but important: the J eliminator fixes the *source* $a$ and varies the *target* $b$ and the path $p$. This is because $\mathsf{refl}_a$ has a fixed source — it is the trivial path at $a$. So path induction is *based* at the source.

There is also an *unbased* form of J that varies both endpoints simultaneously; the two forms are equivalent (as shown in Chapter 9).

---

## 3. Concatenation and Inversion

Using J, we derive the basic operations on paths.

### 3.1 Concatenation

**Theorem 16.1.** For any $p : a = b$ and $q : b = c$, there is a *concatenation* $p \cdot q : a = c$.

*Construction:* By J on $q$ (with $a$ and $b$ as basepoints): it suffices to prove $a = b$ from $p : a = b$ and $\mathsf{refl}_b : b = b$. But $p$ itself is a proof of $a = b$. So: $p \cdot \mathsf{refl}_b \equiv p$.

More carefully: apply J to $q$ with motive $C(c', q') = (a = c')$ and proof $d = p : a = b$. This gives a function $\Pi_{c:A} \Pi_{q : b = c} (a = c)$, and we define $p \cdot q$ as this applied to $(c, q)$.

**Computation:**
$$p \cdot \mathsf{refl}_b \equiv p$$
$$\mathsf{refl}_a \cdot q \equiv q \quad \text{(provable propositionally, not definitionally, in MLTT)}$$

### 3.2 Inversion

**Theorem 16.2.** For $p : a = b$, there is an *inverse* $p^{-1} : b = a$.

*Construction:* By J on $p$: it suffices to define $(\mathsf{refl}_a)^{-1}$. The natural choice is $\mathsf{refl}_a$ itself. So $(\mathsf{refl}_a)^{-1} \equiv \mathsf{refl}_a$.

**Computation:** $(\mathsf{refl}_a)^{-1} \equiv \mathsf{refl}_a$.

### 3.3 The Groupoid Laws

All groupoid laws hold *propositionally* (as elements of identity types):

**Theorem 16.3.** For $p : a = b$, $q : b = c$, $r : c = d$:
1. $\mathsf{refl}_a \cdot p = p$ (left unit)
2. $p \cdot \mathsf{refl}_b = p$ (right unit)
3. $(p \cdot q) \cdot r = p \cdot (q \cdot r)$ (associativity)
4. $p \cdot p^{-1} = \mathsf{refl}_a$ (right inverse)
5. $p^{-1} \cdot p = \mathsf{refl}_b$ (left inverse)

Each identity is an element of a type of the form $s = t$ where $s, t$ are path concatenations. Each is proved by J.

*Proof of (1):* By J on $p$: suffices to show $\mathsf{refl}_a \cdot \mathsf{refl}_a = \mathsf{refl}_a$. By the computation rule for concatenation, $\mathsf{refl}_a \cdot \mathsf{refl}_a \equiv \mathsf{refl}_a$, so $\mathsf{refl}$ proves this. $\square$

*Proof of (3):* By J on $p$: reduces to $(\mathsf{refl}_a \cdot q) \cdot r = \mathsf{refl}_a \cdot (q \cdot r)$, i.e., $q \cdot r = q \cdot r$. QED. (For the fully formal version, one must be careful about which J one applies first.) $\square$

---

## 4. Higher Paths: Paths Between Paths

One of the most striking features of HoTT is that the groupoid laws themselves — the proofs in Theorem 16.3 — are *elements of types*. They can themselves be identified (or not) by higher path types.

**Definition 16.4.** For $p, q : a =_A b$, the type $p =_{a=_Ab} q$ (also written $p = q$ in context) is the *type of 2-paths* or *type of homotopies between paths*.

This type has its own J-eliminator (path induction for paths of paths), and its own groupoid laws.

**The tower of identity types:**
$$A, \quad a =_A b, \quad p =_{a=b} q, \quad \alpha =_{p=q} \beta, \quad \ldots$$

This is an infinite tower, and all levels have groupoid structure. Together, this is the structure of an *∞-groupoid* — the central object in HoTT.

---

## 5. Transport

**Definition 16.5 (Transport).** Given:
- A type family $P : A \to \mathsf{Type}$
- A path $p : a =_A b$

There is a function $\mathsf{transport}^P(p) : P(a) \to P(b)$.

*Construction:* By J on $p$: it suffices to define $\mathsf{transport}^P(\mathsf{refl}_a) : P(a) \to P(a)$. Define it to be the identity function.

**Computation:** $\mathsf{transport}^P(\mathsf{refl}_a) \equiv \mathsf{id}_{P(a)}$.

**Theorem 16.6.** Transport along a path is an *equivalence* (has an inverse given by transporting along the inverse path):
$$\mathsf{transport}^P(p^{-1}) \circ \mathsf{transport}^P(p) = \mathsf{id}_{P(a)}$$

**The fibration picture:** In the simplicial set model, a type family $P : A \to \mathsf{Type}$ corresponds to a fibration $p : E \to B$ (the total space fibered over the base). Transport along a path $\gamma : a \to b$ is *parallel transport*: a lift of $\gamma$ to the total space, moving from the fiber $E_a = P(a)$ to the fiber $E_b = P(b)$.

### 5.1 Transport Examples

**Example 16.7 (Substitution).** If $P(x) = (x = c)$ (equality with a fixed term $c$), then $\mathsf{transport}^P(p) : (a = c) \to (b = c)$ sends $q$ to $p^{-1} \cdot q$.

**Example 16.8 (Vectors).** If $P(n) = \mathsf{Vec}\, A\, n$ and $p : m = n$ (a proof of equality of natural numbers), then $\mathsf{transport}^P(p) : \mathsf{Vec}\, A\, m \to \mathsf{Vec}\, A\, n$ "reindexes" the vector.

**Example 16.9 (Lifting equalities to propositions).** If $P : A \to \mathsf{Prop}$ is a proposition-valued family and $p : a = b$, then $\mathsf{transport}^P(p) : P(a) \to P(b)$ is the "substitution of equals" rule. This is the formal version of "if $a = b$ and $P(a)$, then $P(b)$."

---

## 6. The Action on Paths (ap)

**Definition 16.10 (ap).** Given $f : A \to B$ and $p : a =_A b$:
$$\mathsf{ap}_f(p) : f(a) =_B f(b)$$

*Construction:* By J on $p$: it suffices to define $\mathsf{ap}_f(\mathsf{refl}_a)$. The natural choice: $\mathsf{ap}_f(\mathsf{refl}_a) \equiv \mathsf{refl}_{f(a)}$.

**Theorem 16.11 (ap is a functor).** $\mathsf{ap}$ satisfies:
1. $\mathsf{ap}_f(p \cdot q) = \mathsf{ap}_f(p) \cdot \mathsf{ap}_f(q)$
2. $\mathsf{ap}_f(p^{-1}) = (\mathsf{ap}_f(p))^{-1}$
3. $\mathsf{ap}_{g \circ f}(p) = \mathsf{ap}_g(\mathsf{ap}_f(p))$
4. $\mathsf{ap}_{\mathsf{id}}(p) = p$

**The topological picture:** $\mathsf{ap}_f$ is the function-on-paths induced by $f$: a continuous map sends paths to paths. In HoTT, *every function is continuous* because every function respects the path structure.

---

## 7. Homotopies and Function Extensionality

**Definition 16.12 (Homotopy).** A *homotopy* between $f, g : A \to B$ is:
$$H : \Pi_{x:A}\, f(x) =_B g(x)$$

In topological terms: a homotopy is a family of paths, one for each point $x \in A$, from $f(x)$ to $g(x)$.

**Lemma 16.13 (Naturality of homotopy).** If $H : f \sim g$ (a homotopy) and $p : a = b$, then:
$$H(b) \cdot \mathsf{ap}_g(p) = \mathsf{ap}_f(p) \cdot H(a)$$
(the homotopy "commutes" with the paths)

*Proof:* By J on $p$. Reduces to $H(a) \cdot \mathsf{refl} = \mathsf{refl} \cdot H(a)$, which follows from the unit laws. $\square$

**Function Extensionality (funext):** The statement that homotopic functions are *equal*:
$$\mathsf{funext} : (f \sim g) \to (f = g)$$

This is **not** provable from the J-rule alone in MLTT. But it follows from the univalence axiom (Chapter 18) and is provable in cubical type theory (Chapter 23).

**Why funext is important:** Without funext, we cannot prove that functions that agree pointwise are equal. This would make mathematics cumbersome. With funext, the category of types is a genuine model of mathematics.

---

## 8. Paths in Specific Types

### 8.1 Paths in Product Types

**Theorem 16.14.** For $A \times B$ and $(a_1, b_1), (a_2, b_2) : A \times B$:
$$(a_1, b_1) =_{A \times B} (a_2, b_2) \simeq (a_1 =_A a_2) \times (b_1 =_B b_2)$$

A path in $A \times B$ is a pair of paths — one in each component.

*Proof:* Define $f : (a_1, b_1) = (a_2, b_2) \to (a_1 = a_2) \times (b_1 = b_2)$ by $f(p) = (\mathsf{ap}_{\mathsf{fst}}(p), \mathsf{ap}_{\mathsf{snd}}(p))$. Define the inverse by path induction: it suffices to handle $\mathsf{refl}$, for which we use $(\mathsf{refl}, \mathsf{refl})$. Show these are inverse equivalences. $\square$

### 8.2 Paths in Dependent Pair Types

**Theorem 16.15.** For $(a_1, b_1), (a_2, b_2) : \Sigma_{x:A} B(x)$:
$$(a_1, b_1) =_{\Sigma_{x:A} B(x)} (a_2, b_2) \simeq \Sigma_{p : a_1 = a_2}\, \mathsf{transport}^B(p)(b_1) = b_2$$

A path in a Σ type is a path in the first component together with a path in the second component that lies over it (using transport).

### 8.3 Paths in Function Types

**Theorem 16.16 (assuming funext).** For $f, g : A \to B$:
$$f =_{A \to B} g \simeq \Pi_{x:A}\, f(x) = g(x)$$

Two functions are equal iff they are pointwise equal. This is function extensionality.

---

## Exercises

**16.1.** Prove Theorem 16.3 (the groupoid laws) in full detail using J. For each law, give the explicit J-application.

**16.2.** Prove that transport along a concatenation factors:
$$\mathsf{transport}^P(p \cdot q) = \mathsf{transport}^P(q) \circ \mathsf{transport}^P(p)$$

**16.3.** Prove the naturality of homotopy (Lemma 16.13) in full.

**16.4.** Show that $\mathsf{ap}_f$ is a groupoid homomorphism: that is, verify items (1)–(4) of Theorem 16.11.

**16.5.** In Agda (with `--without-K`), formalize:
  - The definition of path concatenation
  - The proof of associativity
  - The proof of the left unit law

**16.6.** Show that $\Sigma_{b:A} (a = b)$ is contractible for any $a : A$. Give the center of contraction and the homotopy to it.

**16.7.** Paths in $\mathbb{N}$: show that $\mathsf{zero} =_\mathbb{N} \mathsf{succ}(n)$ is empty for any $n$ (there is no proof). (*Hint:* Define a function $A : \mathbb{N} \to \mathsf{Type}$ with $A(\mathsf{zero}) = \mathbf{1}$ and $A(\mathsf{succ}(n)) = \mathbf{0}$, then transport.)

**16.8.** Prove that paths in $\mathbb{N}$ are decidable: for any $m, n : \mathbb{N}$, either $m = n$ or $m \neq n$.

**16.9 (Challenge: higher paths).** Show that the type $\mathsf{refl}_a =_{\Sigma_{b:A}(a=b)} \mathsf{refl}_a$ is contractible (the space of paths from the constant path to itself in the contractible total path space is contractible). What does this mean for the higher groupoid structure of $A$?

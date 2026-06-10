# Chapter 16 Exercises: Identity Types and Paths

These exercises develop facility with path induction (J), transport, and the homotopy-theoretic interpretation of identity types.

---

## Section 1: Path Induction and Groupoid Laws

**Exercise 1.1 (Associativity by double J).**
Prove associativity of path concatenation: for $p : a = b$, $q : b = c$, $r : c = d$:
$$(p \cdot q) \cdot r = p \cdot (q \cdot r)$$
by induction on $p$ and then on $q$. Show how both J applications work out and compute the base case explicitly.

**Exercise 1.2 (Inversion laws).**
Prove that $p^{-1} \cdot p = \mathsf{refl}_b$ and $p \cdot p^{-1} = \mathsf{refl}_a$ for $p : a = b$, using J. At each step, state the motive and the base case explicitly.

**Exercise 1.3 (Symmetric J).**
The standard J rule inductes with the left endpoint $a$ fixed. State and prove the "symmetric J" rule where the right endpoint $b$ is fixed. Then derive the standard J rule from symmetric J and vice versa.

**Exercise 1.4 (Path inversion is an involution).**
Prove $(p^{-1})^{-1} = p$ for all $p : a = b$, using J. Identify the motive and base case.

**Exercise 1.5 (Contractibility via J).**
Show directly that the total path space $\sum_{b:A}(a = b)$ is contractible, by constructing:
- The center of contraction $(a, \mathsf{refl}_a)$
- The contraction path from $(a, \mathsf{refl}_a)$ to any $(b, p)$
using the Σ-path characterization (Theorem 5.2) and J.

---

## Section 2: Transport

**Exercise 2.1 (Transport in constant families).**
Show that for a constant type family $P(x) = B$ (no dependence on $x$):
$$\mathsf{transport}^B(p, b) = b$$
for all $p : a = a'$ and $b : B$. That is, transport in a constant family is the identity. (Hint: use J on $p$.)

**Exercise 2.2 (Transport over path composition).**
Prove the composition rule for transport:
$$\mathsf{transport}^P(p \cdot q, x) = \mathsf{transport}^P(q, \mathsf{transport}^P(p, x))$$
for $p : a = b$, $q : b = c$, and $x : P(a)$.

**Exercise 2.3 (Transport and inversion).**
Show that $\mathsf{transport}^P(p^{-1})$ is the inverse of $\mathsf{transport}^P(p)$ by proving:
1. $\mathsf{transport}^P(p^{-1}, \mathsf{transport}^P(p, x)) = x$
2. $\mathsf{transport}^P(p, \mathsf{transport}^P(p^{-1}, y)) = y$

Use Exercise 2.2 and the groupoid laws.

**Exercise 2.4 (Transport in the identity family).**
Let $P(x) = (a = x)$ for a fixed $a : A$. Compute:
$$\mathsf{transport}^{a=(-)}(p, q)$$
for $p : b = c$ and $q : a = b$. Show that the result equals $q \cdot p$. (Hint: use J on $p$.)

**Exercise 2.5 (Transport in the path family).**
Let $P(x) = (x = c)$ for a fixed $c : A$. Compute:
$$\mathsf{transport}^{(-)=c}(p, q)$$
for $p : a = b$ and $q : a = c$. Express the result using $p^{-1}$ and $q$.

---

## Section 3: ap and apd

**Exercise 3.1 (ap preserves composition).**
Prove $\mathsf{ap}_f(p \cdot q) = \mathsf{ap}_f(p) \cdot \mathsf{ap}_f(q)$ for $f : A \to B$, $p : a = b$, $q : b = c$. Proceed by J on $p$, then J on $q$.

**Exercise 3.2 (ap preserves inversion).**
Prove $\mathsf{ap}_f(p^{-1}) = (\mathsf{ap}_f(p))^{-1}$ for $f : A \to B$ and $p : a = b$.

**Exercise 3.3 (ap for composition).**
Prove $\mathsf{ap}_{g \circ f}(p) = \mathsf{ap}_g(\mathsf{ap}_f(p))$ for composable functions $f : A \to B$ and $g : B \to C$.

**Exercise 3.4 (ap for identity).**
Prove $\mathsf{ap}_{\mathsf{id}_A}(p) = p$ for all $p : a = b$ in $A$.

**Exercise 3.5 (apd reduces to ap).**
Show that when $P(x) = B$ is a constant family and $f : A \to B$ is a non-dependent function, the dependent action $\mathsf{apd}_f(p)$ implies $\mathsf{ap}_f(p) : f(a) = f(b)$.

Specifically: $\mathsf{apd}_f(p) : \mathsf{transport}^B(p, f(a)) = f(b)$. Using Exercise 2.1, deduce that $f(a) = f(b)$.

---

## Section 4: Homotopies as Natural Transformations

**Exercise 4.1 (Naturality square).**
Let $H : f \sim g$ be a homotopy between $f, g : A \to B$, and let $p : a_1 = a_2$ in $A$.
Prove the naturality condition:
$$H(a_1) \cdot \mathsf{ap}_g(p) = \mathsf{ap}_f(p) \cdot H(a_2)$$
using J on $p$.

**Exercise 4.2 (Homotopies and constant functions).**
Let $f : A \to B$ be a constant function, $f(x) = b$ for all $x$. Show that for any $H : f \sim f$ (a self-homotopy), $H$ is homotopic to the constant homotopy $\lambda x.\, \mathsf{refl}_b$.

(Hint: Use the naturality condition from Exercise 4.1 with $g = f$.)

**Exercise 4.3 (Homotopies compose).**
If $H : f \sim g$ and $K : g \sim h$ for $f, g, h : A \to B$, define the composition $K \circ H : f \sim h$ by $(K \circ H)(x) = H(x) \cdot K(x)$.
Show that if $H$ and $K$ are natural transformations (satisfy the naturality square), so is $K \circ H$.

**Exercise 4.4 (Eckmann-Hilton in detail).**
Let $A$ be a type and $a : A$. For $\alpha, \beta : \mathsf{refl}_a = \mathsf{refl}_a$ (elements of the second loop space $\Omega^2 A$):
1. Define horizontal composition $\alpha \star \beta$ (whiskering).
2. State the interchange law between vertical ($\cdot$) and horizontal ($\star$) composition.
3. Derive from the interchange law that $\alpha \cdot \beta = \beta \cdot \alpha$.

---

## Section 5: Paths in Specific Types

**Exercise 5.1 (Product paths).**
Show that the equivalence $(a_1, b_1) =_{A \times B} (a_2, b_2) \simeq (a_1 = a_2) \times (b_1 = b_2)$ is natural in $A$ and $B$: for any $f : A \to A'$ and $g : B \to B'$:
$$\mathsf{ap}_{f \times g} \circ \mathsf{pair\text{-}eq} = \mathsf{pair\text{-}eq} \circ (\mathsf{ap}_f \times \mathsf{ap}_g)$$

**Exercise 5.2 (Σ-paths).**
Let $P : A \to \mathsf{Prop}$ be a predicate taking values in propositions (types with at most one element). Show that:
$$(a_1, p_1) =_{\sum_{x:A} P(x)} (a_2, p_2) \simeq (a_1 = a_2)$$

That is, paths in a subtype (defined by a propositional predicate) are just paths in the base type. (Hint: use the Σ-path characterization; the transport condition is automatic since $P(a_2)$ has at most one element.)

**Exercise 5.3 (Function extensionality consequences).**
Assuming function extensionality, prove:
1. The identity function $\mathsf{id}_A : A \to A$ is the unique function homotopic to itself by the identity homotopy.
2. For $f : A \to B$ and $g : B \to C$, if $f \sim f'$ and $g \sim g'$, then $g \circ f \sim g' \circ f'$.
3. Composition is continuous: the map $(g, f) \mapsto g \circ f$ sends homotopies to homotopies.

**Exercise 5.4 (happly computation).**
Show that $\mathsf{happly}(\mathsf{refl}_f)(x) = \mathsf{refl}_{f(x)}$ for any $f : A \to B$ and $x : A$. This verifies that the trivial path gives the trivial homotopy.

**Exercise 5.5 (Paths in ℕ).**
Show that the identity type on $\mathbb{N}$ is "discrete": for any $m, n : \mathbb{N}$:
- $m =_\mathbb{N} n$ is a proposition (at most one element)
- Equality of natural numbers is decidable

This uses the fact that $\mathbb{N}$ is a set (h-level 0), which we'll prove in Chapter 17 using the path characterization of $\mathbb{N}$ via its inductive structure.

---

## Section 6: Research-Level Exercises

**Exercise 6.1 (Pointed types and based paths).**
A *pointed type* is a pair $(A, a)$ with $a : A$. A *based loop* at $a$ is an element of $\Omega(A, a) = (a =_A a)$. Show:
1. $\Omega(A, a)$ has a group structure (using path concatenation and inversion).
2. The group structure is a *groupoid* structure (not just a group), meaning the laws hold propositionally.
3. Describe the based path space $\sum_{b:A}(a = b)$ and its relation to $\Omega(A, a)$.

**Exercise 6.2 (Sections are homotopic to their fiber.).**
Let $f : A \to B$ and $s : B \to A$ with $r : f \circ s \sim \mathsf{id}_B$ (so $s$ is a section of $f$). Show that for any $b : B$, the fiber $\mathsf{fib}_f(b) = \sum_{a:A}(f(a) = b)$ is inhabited (has an element). Find the element explicitly.

**Exercise 6.3 (Path induction at dimension 2).**
State the "J₂ rule" for paths between paths: to prove $C(H)$ for all $H : p = q$ (where $p, q : a = b$), it suffices to prove $C(\mathsf{refl}_p)$. Derive this from the standard J rule applied twice.

**Exercise 6.4 (Loop space functor).**
For based pointed maps $f : (A, a) \to (B, b)$ (i.e., $f : A \to B$ with $f(a) = b$), define the *loop space map* $\Omega f : \Omega(A, a) \to \Omega(B, b)$ using ap. Show:
1. $\Omega(\mathsf{id}) \sim \mathsf{id}$
2. $\Omega(g \circ f) \sim \Omega g \circ \Omega f$
3. $\Omega f$ is a group homomorphism (respects concatenation and inversion)

This shows that $\Omega$ is a functor from pointed types to groups.

**Exercise 6.5 (Whiskering formulas).**
For $\alpha : p = q$ (a 2-path between paths $p, q : a = b$) and $r : b = c$, define *right whiskering* $\alpha \star r : p \cdot r = q \cdot r$. Similarly define *left whiskering* $r' \star \alpha : r' \cdot p = r' \cdot q$ for $r' : a' = a$.

Show:
1. Both operations are defined by J.
2. The interchange law: $(\alpha \cdot_v \beta) \star r = (\alpha \star r) \cdot_v (\beta \star r)$ where $\cdot_v$ is vertical composition.
3. Whiskering by $\mathsf{refl}$ is the identity.

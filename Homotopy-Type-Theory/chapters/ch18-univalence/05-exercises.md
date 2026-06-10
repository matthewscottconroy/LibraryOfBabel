# Chapter 18 Exercises: Equivalences and Univalence

---

## Section 1: Equivalences

**Exercise 1.1.** Show that the identity function $\mathsf{id}_A : A \to A$ is an equivalence in all three senses: bi-invertible, half-adjoint, and contractible fibers.

**Exercise 1.2.** Show that if $f : A \to B$ and $g : B \to C$ are equivalences, then $g \circ f : A \to C$ is an equivalence (using the contractible fibers definition).

**Exercise 1.3 (Two-out-of-three).** In the composable triple $A \xrightarrow{f} B \xrightarrow{g} C$:
- If $f$ and $g$ are equivalences, show $g \circ f$ is an equivalence.
- If $g \circ f$ and $f$ are equivalences, show $g$ is an equivalence.
- If $g \circ f$ and $g$ are equivalences, show $f$ is an equivalence.

**Exercise 1.4 (Quasi-inverse is not a proposition).** Give an example of a type $A$ and function $f : A \to A$ such that $\mathsf{qinv}(f)$ has more than one element. (Hint: consider $f = \mathsf{id}_A$ and look for non-trivial self-homotopies of the identity.)

**Exercise 1.5.** Show that $A \simeq \mathbf{1}$ iff $A$ is contractible.

**Exercise 1.6 (Fiber sequences).** For $f : A \to B$ and $y : B$, define the fiber $\mathsf{fib}_f(y) = \sum_{x:A}(f(x) = y)$. Show that $\mathsf{fib}_f(y)$ is contractible for all $y$ iff $f$ is an equivalence.

**Exercise 1.7.** Show that equivalences are closed under products: if $f : A \simeq A'$ and $g : B \simeq B'$, then $(f, g) : A \times B \simeq A' \times B'$.

---

## Section 2: The Univalence Axiom

**Exercise 2.1.** Using Univalence, show that $\mathsf{ua}(\mathsf{id}_A) = \mathsf{refl}_A$.

**Exercise 2.2.** Show that $\mathsf{ua}(e^{-1}) = (\mathsf{ua}(e))^{-1}$ — the path corresponding to the inverse equivalence is the inverse path.

**Exercise 2.3.** Show that $\mathsf{ua}(e_2 \circ e_1) = \mathsf{ua}(e_1) \cdot \mathsf{ua}(e_2)$ — the path corresponding to a composition of equivalences is the concatenation of paths.

**Exercise 2.4 (Transport via Univalence).** For $e : A \simeq B$ and $a : A$, prove:
$$\mathsf{transport}^{\mathsf{id}_\mathsf{Type}}(\mathsf{ua}(e), a) = e.1(a)$$
(Transport along $\mathsf{ua}(e)$ applies the underlying function of $e$.)

**Exercise 2.5.** Using Univalence, show that the universe $\mathsf{Type}$ is not a set. (Find a type with a non-trivial automorphism and show it gives a non-trivial loop in the universe.)

---

## Section 3: Consequences

**Exercise 3.1 (Propositional extensionality).** Prove: for propositions $P$ and $Q$, $(P \leftrightarrow Q) \to (P = Q)$, using Univalence.

**Exercise 3.2 (Function extensionality).** Assuming Univalence, sketch the proof of function extensionality for functions $f, g : A \to B$.

**Exercise 3.3 (Structure invariance for products).** Let $P(A) = A \times A$. Given $e : A \simeq B$, use transport along $\mathsf{ua}(e)$ to construct an equivalence $P(A) \simeq P(B)$. Verify this matches the expected equivalence $(a_1, a_2) \mapsto (e(a_1), e(a_2))$.

**Exercise 3.4.** Show that for any equivalence $e : A \simeq B$ and predicate $P : A \to \mathsf{Type}$:
$$\left(\sum_{x:A} P(x)\right) \simeq \left(\sum_{y:B} P(e^{-1}(y))\right)$$
(The total space of a predicate over $A$ is equivalent to the total space of the transported predicate over $B$.)

**Exercise 3.5 (Automorphisms).** Show that the type $\mathsf{Aut}(A) = A \simeq A$ has a group structure:
- Multiplication: composition of equivalences
- Identity: $\mathsf{id}_A$
- Inverse: inverse equivalence

For $A = \mathsf{Bool}$, show $\mathsf{Aut}(\mathsf{Bool}) \simeq \mathbb{Z}/2\mathbb{Z}$.

---

## Section 4: Examples

**Exercise 4.1.** Show that there are exactly $n!$ paths $\mathsf{Fin}(n) = \mathsf{Fin}(n)$ in the universe. (Hint: count the bijections $\mathsf{Fin}(n) \to \mathsf{Fin}(n)$ — these are the permutations, and there are $n!$ of them.)

**Exercise 4.2.** Show that the proposition $\mathbf{1} = \mathbf{1}$ has exactly one path (reflexivity), while $\mathbf{1} = \mathbf{0}$ is empty.

**Exercise 4.3 (Group paths).** Formalize the statement "two groups are equal iff they are isomorphic" in the type of groups $\mathsf{Group} = \sum_{G:\mathsf{hSet}} \mathsf{GroupStr}(G)$. State precisely what "isomorphic" means at the type level.

**Exercise 4.4 (Pointed types).** Define the type of *pointed types* as $\mathsf{Type}_* = \sum_{A:\mathsf{Type}} A$. Show that two pointed types $(A, a)$ and $(B, b)$ are equal in $\mathsf{Type}_*$ iff there is an equivalence $e : A \simeq B$ with $e(a) = b$.

**Exercise 4.5 (Non-trivial path).** Construct the specific path $\mathsf{ua}(\mathsf{neg}) : \mathsf{Bool} = \mathsf{Bool}$ corresponding to the negation equivalence. Show directly (without Univalence) that this path must be non-trivial (not equal to $\mathsf{refl}$) — for instance, by transporting $\mathsf{true}$ along it and getting $\mathsf{false}$.

---

## Section 5: Research-Level Exercises

**Exercise 5.1 (Univalence for Σ-types).** Formulate and prove the univalence principle for Σ-types: two elements $(a_1, b_1)$ and $(a_2, b_2)$ of $\sum_{x:A} B(x)$ are equal iff there's an equivalence of the fibers compatible with the first components.

**Exercise 5.2 (J for the universe).** Formulate the "J rule for the universe" that follows from Univalence: to prove $P(A, B, e)$ for all $A, B : \mathsf{Type}$ and $e : A \simeq B$, it suffices to prove $P(A, A, \mathsf{id}_A)$. Show this follows from Univalence + the ordinary J rule.

**Exercise 5.3 (Cubical Univalence).** In cubical type theory (Chapter 23), Univalence is a theorem, not an axiom. The key is the Glue type, which constructs a type $\mathsf{Glue}(A, e)$ from $A$ and an equivalence $e : B \simeq A$. State the intended type of $\mathsf{ua}$ in cubical type theory and explain why it would be a theorem rather than an axiom.

**Exercise 5.4 (The Univalent Foundations program).** Voevodsky proposed that all mathematical structures should be formalized in a way that makes equality equal to isomorphism. Give two examples of mathematical structures:
1. One where equality (in the natural type-theoretic formalization) already coincides with isomorphism
2. One where it doesn't, and explain how to reformulate it so that it does

**Exercise 5.5 (Universe polymorphism).** In HoTT, there's a hierarchy of universes $\mathsf{Type}_0, \mathsf{Type}_1, \ldots$ with $\mathsf{Type}_n : \mathsf{Type}_{n+1}$. Formulate Univalence for each universe level: for each $n$, $\mathsf{idToEquiv}_{n} : (A =_{\mathsf{Type}_n} B) \simeq (A \simeq B)$ for $A, B : \mathsf{Type}_n$. Show that Univalence at all levels is a consistent axiom.

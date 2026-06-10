# Exercises: Chapter 8

Exercises range from direct computation to proof construction. Problems marked (★) are more challenging; problems marked (★★) require significant proof work or creativity.

---

## Section 1: Type Families

**1.** Write out the type judgment expressing that `IsOdd : ℕ → Type` is a valid type family, where IsOdd n holds iff n is odd. Define IsOdd explicitly by recursion on ℕ.

**2.** The family `Fin : ℕ → Type` can be defined by: Fin 0 = 𝟘, Fin (n+1) = Fin n + 𝟙. List all elements of Fin 4 explicitly.

**3.** Define the type family `LessThan : ℕ → ℕ → Type` (so LessThan m n is inhabited iff m < n) by recursion. Verify that LessThan 2 5 is inhabited and LessThan 5 2 is empty.

**4.** Consider the family `Eq_ℕ : ℕ → ℕ → Type` where Eq_ℕ m n = 𝟙 if m = n (as natural numbers) and Eq_ℕ m n = 𝟘 otherwise (defined by double recursion). Show that Eq_ℕ 3 3 is inhabited and Eq_ℕ 2 4 is empty. Why is this different from the identity type 3 =_ℕ 3?

**5.** (★) Define a type family `Sorted : List ℕ → Type` that is inhabited iff the list is sorted in non-decreasing order. Verify that `Sorted [1, 2, 2, 5]` is inhabited and `Sorted [3, 1, 2]` is empty.

---

## Section 2: Π Types

**6.** Write the type and term for the function that takes a natural number n and a vector v : Vec A n and returns the vector with the same elements in reverse order. (Give the type precisely; the full implementation by recursion is optional.)

**7.** Show that the type Π(A:Type).A is uninhabited. (Hint: what would an element of this type have to do when applied to 𝟘?)

**8.** Construct an element of Π(A B : Type).(A → B) → A → B. What is this function called?

**9.** Write out the full type of function composition in dependent type theory, where the codomain of f may depend on the input:

$$({-} \circ {-}) : \prod_{A:\mathsf{Type}} \prod_{B:A\to\mathsf{Type}} \prod_{C:\prod_{a:A} B(a) \to \mathsf{Type}} \left(\prod_{a:A}\prod_{b:B(a)} C(b)\right) \to \prod_{f:\prod_{a:A} B(a)} \prod_{a:A} C(f(a))$$

Verify this type is correct by checking that the term λg. λf. λa. g a (f a) inhabits it.

**10.** Define the constant function `const : Π(A B : Type). A → B → A` and verify it has the stated type.

**11.** (★) Show that Π(A:Type).A → A has at least two distinct elements. (Hint: consider λA. λa. a and λA. λa. a. Are these definitionally equal? What if A has multiple elements?) Discuss.

---

## Section 3: Σ Types

**12.** Write out the type and a specific element of: "the type of a natural number together with a proof that it is greater than 100." Give both the Σ-type expression and a concrete element.

**13.** Show that A × B ≃ Σ(x:A).B (where B does not depend on x). (Define the equivalence explicitly: give the two maps and show they are inverse.)

**14.** The type `Σ(A:Type).A` contains pairs (A, a) where A is a type and a is an element. Show this type is equivalent to 𝟙 (up to the type-theoretic notion of equivalence). (★)

**15.** In dependent type theory, the type of "a prime number" is:
$$\mathsf{Prime} = \sum_{n:\mathbb{N}} \mathsf{IsPrime}(n)$$
where IsPrime n asserts n is prime. Write the type of the function "given a prime p, return the next prime greater than p." You do not need to implement it — just give its type.

**16.** Verify that the following is an element of Σ(n:ℕ).Σ(m:ℕ).(n + m = 10): the pair (3, (7, proof)). What is the type of proof here? Construct it.

**17.** (★) State and prove the following: if P : A → Type and Q : Σ(a:A).P(a) → Type, then:
$$\sum_{x:\sum_{a:A} P(a)} Q(x) \simeq \sum_{a:A}\sum_{p:P(a)} Q((a,p))$$
This is "Σ-associativity" or "the dependent sum telescopes."

---

## Section 4: Universes

**18.** Explain why the following is problematic: "Let U = Σ(A:U).A. Then U is the type of all types that contain themselves." What universe level issues arise?

**19.** The following term is ill-typed in a system with a stratified universe hierarchy. Identify why:
$$f = \lambda A.\, A \to A : \mathsf{Type}_0 \to \mathsf{Type}_0$$
and then say: what is the correct type for f, and at which universe level does Π(A:Type₀).A → A live?

**20.** Universe polymorphism: write the type of the function `map : (A → B) → List A → List B` so that it works for types A, B in any universe level ℓ. (Use Level as a type of universe levels.)

**21.** (★) Show that the statement "Type₀ : Type₀" leads to a non-terminating sequence in the following sense: from Type₀ : Type₀, construct an element of Π(A:Type₀).A → A by an apparent "impredicative" quantification that quantifies over all terms including itself. You need not complete the full paradox, but sketch where the self-reference arises.

---

## Section 5: Inductive Types

**22.** Compute the normal form of the following (ℕ defined inductively with ind_ℕ):
$$\mathsf{ind}_\mathbb{N}(\lambda n.\, \mathbb{N},\ 0,\ \lambda n\, r.\, n + r,\ \mathsf{succ}(\mathsf{succ}(\mathsf{zero})))$$

**23.** Define multiplication of natural numbers using ind_ℕ. Specify C, c_z, and c_s explicitly.

**24.** Define the predecessor function `pred : ℕ → ℕ` (with pred 0 = 0) using ind_ℕ. Give C, c_z, c_s.

**25.** The eliminator for the empty type 𝟘 is:
$$\mathsf{ind}_\mathbf{0} : \prod_{C:\mathbf{0}\to\mathsf{Type}} \prod_{x:\mathbf{0}} C(x)$$
Use this to derive ¬A → A → B (from a proof that A is false and a proof of A, derive anything). Give the explicit term.

**26.** (★) Define W-type encoding of List A:
- What should A be (the "shape" type)?
- What should B : A → Type be (the "position" family)?
- Identify what sup(a, f) looks like for the "nil" and "cons" cases.

**27.** (★) The eliminator for Bool is `ind_Bool : Π(C:Bool→Type). C(true) → C(false) → Π(b:Bool). C(b)`. Using this, define: (a) if-then-else, (b) negation, (c) conjunction. Give explicit terms.

**28.** (★★) Define the type `Tree A` of binary trees with leaves labeled by A. Give constructors, write the eliminator, and state the computation rules. Then define `size : Tree A → ℕ` using the eliminator.

---

## Section 6: Propositions as Types

**29.** Translate the following logical statement into a Π/Σ type in MLTT: "For every natural number n, if n is even, then n² is even." Give the explicit type.

**30.** Show that the following is a theorem in MLTT (proof-relevant AC, as discussed in the text): from a term h : Π(x:A).Σ(b:B(x)).C(x,b), construct a pair (f, g) where f : Π(x:A).B(x) and g : Π(x:A).C(x, f(x)). Give the explicit terms f and g.

**31.** In classical logic, ¬¬P → P (double negation elimination) is valid. In MLTT, it is not provable in general. Show that the type (Π(P:Type).(P → 𝟘) → 𝟘) → P has no general inhabitant. (Hint: instantiate P = 𝟘 and see what happens.)

**32.** (★) Show that De Morgan's law (¬(P ∧ Q) → ¬P ∨ ¬Q) is not provable in MLTT without additional axioms. Specifically, construct a model where the type:
$$\left(\prod_{P\,Q:\mathsf{Type}} (P \times Q \to \mathbf{0}) \to ((P \to \mathbf{0}) + (Q \to \mathbf{0}))\right) \to \mathbf{0}$$
is uninhabited, i.e., where the De Morgan law does not fail obviously. Discuss why this is hard and what axiom would be needed.

**33.** (★) State the principle of *excluded middle* (LEM) as a type in MLTT: Π(P:Type).P + ¬P. Show that assuming LEM allows you to prove the classical De Morgan law from Exercise 32. Give the explicit proof term (under the assumption of LEM).

**34.** (★★) Curry-Howard for predicate logic: formalize and prove (in MLTT) the following: "If Π(x:A).P(x) → Q(x) and Σ(x:A).P(x), then Σ(x:A).Q(x)." Give the explicit Π/Σ types and the explicit proof term. This is the "existential generalization under a universally quantified implication."

**35.** (★★) Show that in MLTT, the following type is inhabited: 
$$\prod_{A:\mathsf{Type}} \prod_{B:A\to\mathsf{Type}} \left(\prod_{x:A} B(x)\right) \to \prod_{x:A} B(x)$$
(This is the identity function on Π types.) Now show that the following type is also inhabited:
$$\prod_{A:\mathsf{Type}} \prod_{B:A\to\mathsf{Type}} \prod_{C:\prod_{x:A} B(x) \to \mathsf{Type}} \left(\prod_{f:\prod_{x:A} B(x)} C(f)\right) \to \prod_{f:\prod_{x:A} B(x)} C(f)$$
What pattern do you observe? State the general principle and prove it.

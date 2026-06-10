# Applied Exercises

The four judgments, the J rule, the W type, and the intensional/extensional divide are not purely theoretical concerns. Every modern proof assistant — Agda, Coq, Lean 4, Idris — is an implementation of some version of MLTT, and every program written in one of those systems is a term in MLTT whose type is its specification. The exercises below ground the abstract machinery in concrete tasks: writing an interpreter for MLTT terms, using Agda/Coq to discharge Curry-Howard obligations, navigating propositional equality in the setoid library, and using W types as a general recursion schema.

---

## Exercise A.1: A Small MLTT Interpreter
*Domain: Programming Language Implementation / Meta-language Engineering*

**Setup:** One of the most effective ways to internalize the four judgment forms of MLTT is to implement a type checker for a small fragment of the theory. Consider a meta-language (say, Haskell, OCaml, or Python) in which you will represent MLTT terms and contexts and implement the judgment-checking algorithms. A minimal fragment is: the empty type, unit type, Π types (function types), Σ types (dependent pairs), and the natural number type, without yet implementing the universe or the identity type.

**Questions:**

1. Define a datatype `Term` in your meta-language that represents raw MLTT terms: variables (de Bruijn indices), lambda abstractions, applications, pairs, projections, natural number literals, and the recursor for naturals. Define a separate datatype `Type` for the type formers Π, Σ, ℕ, ⊤, ⊥. Implement `check : Context -> Term -> Type -> Bool` that decides whether a term has a given type in a context, and `infer : Context -> Term -> Maybe Type` that infers a type.

2. Implement *definitional equality checking* as a subroutine of `check`. Your implementation must handle at minimum $\beta$-reduction for Π types ($(\lambda x. t)\ a \rightsquigarrow t[a/x]$) and the $\iota$-reduction for ℕ (the computation rules for the recursor). Use normalization-by-evaluation (NbE) or a simple call-by-name normalizer. Confirm that `check () ((\x -> succ x) zero) Nat` returns `True`.

3. The conversion rule says: if $a : A$ and $A =_{\mathrm{def}} B$, then $a : B$. Implement this by calling your definitional equality checker inside `check` at the point where you need types to match. Write a test case that would fail without the conversion rule — for instance, a term that applies a function of type $\Pi_{x:\mathbb{N}} P(\mathsf{rec}(x))$ to an argument of type $P(\mathsf{rec}(\mathsf{zero}))$, where the type mismatch is resolved by $\iota$-reduction.

4. (Extension) Extend your implementation with a universe `Type₀ : Type₁` and the basic rules for the universe (formation, introduction, and elimination). Discuss what goes wrong if you add `Type₀ : Type₀` and sketch how Coquand's paradox arises.

*Abstract concept illustrated: The four judgment forms (Section 1.1); the conversion rule; definitional equality as a decidable algorithm via normalization.*

---

## Exercise A.2: Curry-Howard in Practice (Agda)
*Domain: Formal Verification / Proof Engineering*

**Setup:** In Agda, every proof of a proposition is a term, and writing a proof is the same as writing a program. The identity type `_≡_` in Agda is the MLTT identity type with the J rule available via the `J` eliminator or pattern matching on `refl`. This exercise asks you to prove several facts about natural numbers and vectors in Agda, using only the standard library and the structural rules of MLTT — no imported lemmas except those about the specific type formers involved.

**Questions:**

1. Define the natural numbers inductively in Agda (do not import them from `Data.Nat`). Define addition by recursion and prove `n + zero ≡ n` and `zero + n ≡ n`. The second is definitionally true (by the definition of addition); the first requires a proof by induction. What judgment form does each proof correspond to?

2. Prove `(m + n) + k ≡ m + (n + k)` (associativity of addition) and `m + n ≡ n + m` (commutativity). These are propositional equalities, not definitional ones. Notice that Agda will *not* accept `refl` for either. Write the proofs using `cong` (the `ap` function in MLTT terminology) and `trans`. Write out, in the MLTT inference rule notation from Section 1, the type of `cong` as a dependent function type.

3. Define the vector type `Vec : (A : Set) → ℕ → Set` and implement `append : Vec A m → Vec A n → Vec A (m + n)`. Write a proof that appending the empty vector on the right is definitionally equal to the identity: `append xs [] ≡ xs`. Is this definitional or propositional? Explain why in terms of the computation rules for the ℕ recursor.

4. (Extension) Using the J eliminator directly (rather than pattern matching on `refl`), prove the groupoid laws: symmetry (`sym : a ≡ b → b ≡ a`) and transitivity (`trans : a ≡ b → b ≡ c → a ≡ c`). In the MLTT derivation tree, identify exactly which instance of the J rule is being used in each case.

*Abstract concept illustrated: The identity type as an inductive type; propositional vs. definitional equality; the J rule; the groupoid laws for identity types (Section 3).*

---

## Exercise A.3: Working with Setoids
*Domain: Abstract Algebra / Verified Programming with Quotients*

**Setup:** Intensional MLTT does not have quotient types natively: you cannot form a type $A/{\sim}$ from a type $A$ and an equivalence relation $\sim$ and have the quotient be a genuine MLTT type. The standard workaround is the *setoid pattern*: instead of quotienting, you work with the pair $(A, \sim)$ explicitly, requiring all functions to *respect* the equivalence relation (i.e., map $\sim$-equivalent inputs to $\sim$-equivalent outputs). The Coq standard library's `Setoid` module, and Agda's `Relation.Binary` module, implement this pattern.

**Questions:**

1. In Agda (or Coq), define the type of integers as the setoid quotient of $\mathbb{N} \times \mathbb{N}$ by the relation $(a, b) \sim (c, d) \iff a + d = b + c$ (representing the integer $a - b$). That is: define a record `Int` consisting of a pair of naturals, and define a propositional equivalence relation `_≈_` on `Int`. Prove that `_≈_` is an equivalence relation (reflexivity, symmetry, transitivity).

2. Define integer addition and prove it *respects* the equivalence: if `x ≈ x'` and `y ≈ y'` then `x + y ≈ x' + y'`. This is the statement that addition is a *setoid morphism*. In what sense does this replace the universal property of the quotient type that you would use if quotient types were available?

3. Define the canonical injection `ℕ → Int` (sending $n$ to the pair $(n, 0)$) and prove that it respects equality: if $m = n$ in ℕ (definitionally or propositionally), then the injections are `≈`-equivalent. Define subtraction on `Int` and prove that $n - n \approx 0$ for all $n$.

4. (Extension) Define a *setoid category*: objects are setoids, morphisms from $(A, \sim_A)$ to $(B, \sim_B)$ are functions $f : A \to B$ that respect the equivalences, and two morphisms are equal if they agree on all $\sim_A$-equivalent inputs. Verify that this forms a category (identity and composition are well-defined). Discuss: in what sense is the setoid category a workaround for the absence of quotient types, and what does it fail to capture that a genuine quotient type would provide?

*Abstract concept illustrated: Intensional vs. extensional MLTT; the setoid interpretation; propositional equality when definitional equality is unavailable (Section 6).*

---

## Exercise A.4: W Types as a General Recursion Schema
*Domain: Data Structures and Algorithms / Formal Verification of Recursive Programs*

**Setup:** The W type $W_{x:A} B(x)$ (well-founded trees with branching type $A$ and subtree-count type $B$) is MLTT's primitive well-founded recursion principle. Every inductive type definable in Agda or Coq can be encoded as a W type. This exercise develops the encoding for natural numbers, lists, and binary trees, and uses the W type to define functions that would otherwise require a general recursion principle that MLTT does not have.

**Questions:**

1. **Encoding ℕ as a W type.** Define `ℕ_W : Set` as `W[ b : Bool ] if b then ⊥ else ⊤` (or in Agda notation, `W Bool (λ b → if b then ⊥ else ⊤)`). Here `false` is the zero constructor (no recursive arguments, since $B(\mathsf{false}) = \top$ is a one-element type giving one "placeholder" that serves as a dummy — wait, think carefully: $W_{b:\mathbb{B}} (B b)$ where $B(\mathsf{true}) = \mathsf{Fin}(0) = \bot$ and $B(\mathsf{false}) = \mathsf{Fin}(1) = \top$). Write out the `sup` constructor and implement `zero_W` and `succ_W` in terms of it. Implement addition using `W-ind`.

2. **Encoding lists as a W type.** Given a type $A$, define `List A` as a W type by choosing appropriate $A'$ (the "node type," which is $A + \top$ for cons and nil) and $B : A' \to \mathsf{Type}$ (the branching type, which is $\bot$ for nil and $\top$ for cons). Implement `map` and `foldr` using the W-eliminator, without using Agda's native list type.

3. **Ordinals via W types.** The first uncountable ordinal $\omega_1$ can be encoded as a W type: $W_{f:\mathbb{N}\to\mathbb{N}} \mathsf{Fin}(\mathsf{some\_function}(f))$. More concretely, define countable ordinals as `W ℕ (λ n → Fin n)` — a node labeled by $n$ has exactly $n$ children. Define the ordinal $\omega$ as the "first limit" construction and write a comparison function `_<ₒ_` on ordinals using W-elimination.

4. (Extension) Martin-Löf's original paper uses the W type to define the natural numbers, and from there, to build all of MLTT's number-theoretic capabilities. Show how Cantor's theorem — that the powerset of $\mathbb{N}$ is not countable — can be stated and proved in MLTT using only the W type for $\mathbb{N}$ and the identity type, without importing any external axioms.

*Abstract concept illustrated: W types as well-founded recursion (Section 2.4); the encoding of inductive types; W-elimination as a general recursion principle.*

---

## Exercise A.5: Eliminating Axioms with W Types
*Domain: Foundations of Mathematics / Proof Engineering*

**Setup:** A recurring challenge in proof assistants is that it is tempting to postulate axioms when a direct construction seems difficult. But axioms make a theory less trustworthy and can, in principle, introduce inconsistency. This exercise asks you to replace axiom-based arguments with explicit W-type constructions.

**Questions:**

1. **Infinity without an axiom.** In some formulations of Coq, the axiom `Nat.Strong.rec` (or Coq's "strong recursion" for naturals) is added as an axiom because the termination checker cannot verify certain recursive definitions. Show how to implement "strong induction" on ℕ — i.e., prove `(∀ n, (∀ m, m < n → P m) → P n) → ∀ n, P n` — using only the W type encoding of ℕ from Exercise A.4 and the W-eliminator, without any axiom.

2. **Well-founded recursion on trees.** In Agda, the standard library provides `Data.List.Induction` using accessibility predicates. Show that for binary trees (encoded as a W type), the analogous "structural recursion" principle is derivable from W-elimination alone. Implement a size function `size : Tree A → ℕ` and then use strong induction on the size to define a function that does not obviously have a structurally recursive definition.

3. **Function extensionality and the setoid workaround.** MLTT does not prove function extensionality: the statement `(∀ x, f x ≡ g x) → f ≡ g`. In HoTT, this follows from Univalence. Show that in plain MLTT (without Univalence), you can avoid needing function extensionality in many situations by using setoids (from Exercise A.3): represent functions as setoid morphisms and replace pointwise equality with the setoid equality. Work out the case of proving that two implementations of list reverse are "equal" (in the setoid sense) without invoking function extensionality.

4. (Extension) The Axiom of Choice (AC) is a genuine axiom in set theory. In MLTT, a version of AC is *provable* without axioms: if $\Pi_{x:A} \Sigma_{y:B(x)} P(x,y)$ holds, then $\Sigma_{f:\Pi_{x:A}B(x)} \Pi_{x:A} P(x, f(x))$ holds. Write out this proof in MLTT notation (and implement it in Agda). This is sometimes called the "type-theoretic axiom of choice." Explain why this does *not* conflict with the intuitionistic character of MLTT — what makes this AC provable while classical AC is not?

*Abstract concept illustrated: W types as a source of well-founded induction; the relationship between axioms and explicit constructions; function extensionality as independent of MLTT.*

---

## Exercise A.6: The Groupoid Laws for the Identity Type
*Domain: Algebraic Topology / Categorical Algebra*

**Setup:** One of the key theorems of Chapter 9 is that every type in MLTT, equipped with its identity type, forms a *groupoid* (a category in which every morphism is an isomorphism). The objects are elements $a : A$, the morphisms from $a$ to $b$ are elements of $a =_A b$, composition is transitivity, and inverses are symmetry. All groupoid laws hold *propositionally* (as elements of higher identity types), but not necessarily *definitionally*.

**Questions:**

1. **Left and right unit laws.** For $p : a =_A b$, prove `refl · p ≡ p` and `p · refl ≡ p` (where `·` is `trans`). Are these definitional or propositional equalities in Agda? What does this tell you about the status of the unit laws for path concatenation?

2. **Associativity.** For `p : a ≡ b`, `q : b ≡ c`, `r : c ≡ d`, prove `(p · q) · r ≡ p · (q · r)`. Again: definitional or propositional? What is the type of the associativity proof? Does it live in `(p · q) · r ≡ p · (q · r)` (a path between paths)?

3. **The inverse laws.** Prove `p · p⁻¹ ≡ refl` and `p⁻¹ · p ≡ refl`. These require path induction (the J rule). Write out the J application explicitly, without relying on Agda's pattern matching on `refl` — use the `J` eliminator directly.

4. (Extension) The groupoid laws above say that every type forms a *1-groupoid* (the identity proofs form sets). But in intensional MLTT, there may be identity proofs of identity proofs — paths between paths — so the structure is actually that of a *weak ∞-groupoid*. Write out the first few levels: for paths `p q : a ≡ b` and a path `α : p ≡ q`, state the horizontal composition law `α ⋆ β` (the Eckmann-Hilton argument for types in context `x y : A`). Verify in Agda that `Ω²(A, a)` (the second loop space) is commutative.

*Abstract concept illustrated: The groupoid structure of the identity type; path induction (J rule); higher identity proofs as the seed of HoTT (Section 3).*

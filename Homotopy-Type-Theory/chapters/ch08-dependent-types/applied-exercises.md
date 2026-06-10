# Applied Exercises

Dependent types move the guarantees of a type system from "this program doesn't confuse integers and booleans" to "this program computes a sorted permutation of its input," "this matrix multiplication is only attempted on conformably-dimensioned matrices," and "this parser accepts exactly the strings specified by the grammar." The exercises below are drawn from the central examples of this chapter — length-indexed vectors, sorted structures, type-safe evaluators, and certified parsers — and ask you to work through them in a dependently typed proof assistant (Agda or Lean 4). The goal is fluency with Π types, Σ types, inductive types, and the identity type as they appear in real programs and proofs.

---

## Exercise C.1: Length-Indexed Vectors
*Domain: Verified Data Structures / Agda or Lean 4*

**Setup:** The type `Vec A n` — vectors of type `A` with exactly `n` elements — is the canonical example of a dependent type. The length constraint, enforced statically by the type checker, eliminates an entire class of runtime errors. This exercise develops the core operations on vectors, each with a type that captures its full specification.

**Questions:**
1. In Agda (or Lean 4), define:
   ```agda
   data Vec (A : Set) : ℕ → Set where
     []  : Vec A 0
     _∷_ : A → Vec A n → Vec A (suc n)
   ```
   Then implement the following functions, giving each its precise dependent type:
   - `head : Vec A (suc n) → A` — safe head (no case for the empty vector)
   - `tail : Vec A (suc n) → Vec A n` — safe tail
   - `append : Vec A m → Vec A n → Vec A (m + n)` — concatenation with computed length
   - `zip : Vec A n → Vec B n → Vec (A × B) n` — zip two same-length vectors

   For each function, explain why the type checker rejects any attempt to call `head []` or `zip` on vectors of different lengths.

2. Define `lookup : Vec A n → Fin n → A` — safe indexing with a bounded index. The type `Fin n` is the type of natural numbers strictly less than `n`:
   ```agda
   data Fin : ℕ → Set where
     zero : Fin (suc n)
     suc  : Fin n → Fin (suc n)
   ```
   Verify that `Fin 0` has no elements (there are no numbers less than 0), and that `Fin 3` has exactly three elements: `zero`, `suc zero`, `suc (suc zero)`. Show that `lookup` is total — it never fails, because the `Fin n` index is guaranteed to be in bounds.

3. Define `replicate : (n : ℕ) → A → Vec A n` — a vector of `n` copies of a value. Then prove:
   ```agda
   lookup-replicate : ∀ (n : ℕ) (x : A) (i : Fin n) → lookup (replicate n x) i ≡ x
   ```
   This is a proof about a program — the kind of verified specification that dependent types make possible. What does the proof look like? (Hint: induction on `i`.)

*Abstract concept illustrated: The Π-type formation rule; Σ types as specifications; inductive families; the `Fin n` type as a membership proof.*

---

## Exercise C.2: Verified Sorting
*Domain: Algorithm Verification / Agda or Lean 4*

**Setup:** A sorting function's correctness has two components: the output must be sorted, and it must be a permutation of the input (so no elements are added or removed). With dependent types, both conditions can be enforced by the *type* of the sorting function — the type checker verifies correctness as it checks types. This exercise develops selection sort with a machine-checked correctness proof.

**Questions:**
1. Define the following predicates on lists of natural numbers:
   ```agda
   data Sorted : List ℕ → Set where
     sorted-nil  : Sorted []
     sorted-one  : Sorted (x ∷ [])
     sorted-cons : x ≤ y → Sorted (y ∷ ys) → Sorted (x ∷ y ∷ ys)
   
   data _∈_ : ℕ → List ℕ → Set where
     here  : x ∈ (x ∷ xs)
     there : x ∈ xs → x ∈ (y ∷ xs)
   
   Permutation : List ℕ → List ℕ → Set
   Permutation xs ys = (∀ x → x ∈ xs → x ∈ ys) × (∀ x → x ∈ ys → x ∈ xs)
   ```
   Verify that `Permutation [] []` holds, that `Permutation (1 ∷ 2 ∷ []) (2 ∷ 1 ∷ [])` holds, and that `Sorted (1 ∷ 2 ∷ 3 ∷ [])` holds.

2. Implement `insert : (x : ℕ) → (xs : List ℕ) → Sorted xs → Σ (List ℕ) (λ ys → Sorted ys × Permutation (x ∷ xs) ys)`.
   This is insertion into a sorted list — the core operation of insertion sort — returning the sorted result together with proofs that it is sorted and is a permutation. (Hint: the key lemma is that if `x ≤ y` and `xs` is sorted starting with `y`, then `x ∷ xs` is sorted.)

3. Using `insert`, implement:
   ```agda
   sort : (xs : List ℕ) → Σ (List ℕ) (λ ys → Sorted ys × Permutation xs ys)
   ```
   The return type is the full specification of a sorting function. The proof proceeds by induction on `xs`, using `insert` at each step. After you have implemented `sort`, extract just the sorting function `sort-fn : List ℕ → List ℕ` by projecting the first component of the Σ type. What happens if you try to change the definition of `sort-fn` in a way that violates the sorted or permutation property? (The type checker rejects it.)

*Abstract concept illustrated: Sigma types as specifications encoding propositions about programs; proof-relevant data structures; the role of the identity type in establishing permutation properties.*

---

## Exercise C.3: A Type-Safe Evaluator
*Domain: Programming Language Theory / Agda or Lean 4*

**Setup:** A standard exercise in dependent type theory is to implement a *type-safe evaluator* for a simply-typed expression language. The key idea: represent the *typing derivation* as the abstract syntax tree, so that ill-typed expressions cannot even be constructed. The evaluator then has type `Expr Γ τ → Val τ` — given a well-typed expression with type `τ` in context `Γ`, return a value of the corresponding Agda/Lean type. No runtime type errors are possible.

**Questions:**
1. Define a dependently typed representation of types and expressions:
   ```agda
   data Ty : Set where
     Nat  : Ty
     Bool : Ty
     _⇒_  : Ty → Ty → Ty
   
   -- Interpret a Ty as an Agda type
   Val : Ty → Set
   Val Nat       = ℕ
   Val Bool      = 𝔹
   Val (σ ⇒ τ)  = Val σ → Val τ
   
   -- Typed contexts as lists of types
   Ctx = List Ty
   
   -- Typed de Bruijn variables
   data _∈_ : Ty → Ctx → Set where
     here  : τ ∈ (τ ∷ Γ)
     there : τ ∈ Γ → τ ∈ (σ ∷ Γ)
   
   -- Typed expressions
   data Expr (Γ : Ctx) : Ty → Set where
     Var  : τ ∈ Γ → Expr Γ τ
     Lam  : Expr (σ ∷ Γ) τ → Expr Γ (σ ⇒ τ)
     App  : Expr Γ (σ ⇒ τ) → Expr Γ σ → Expr Γ τ
     Lit  : ℕ → Expr Γ Nat
     Add  : Expr Γ Nat → Expr Γ Nat → Expr Γ Nat
     IfZ  : Expr Γ Nat → Expr Γ τ → Expr Γ τ → Expr Γ τ
   ```
   Implement an environment type `Env : Ctx → Set` (a typed heterogeneous list) and a lookup function `lookup : τ ∈ Γ → Env Γ → Val τ`.

2. Implement the evaluator:
   ```agda
   eval : Env Γ → Expr Γ τ → Val τ
   ```
   This function must be total and have no case for type errors — they are impossible. The case for `Add` returns a `ℕ`, the case for `Lam` returns a function, the case for `App` applies a function. Verify that `eval [] (Add (Lit 3) (Lit 4)) ≡ 7`.

3. Add a `Let` construct to the expression language:
   ```agda
   Let : Expr Γ σ → Expr (σ ∷ Γ) τ → Expr Γ τ
   ```
   Extend `eval` to handle `Let`. Then add *pairs* (`Pair : Expr Γ σ → Expr Γ τ → Expr Γ (σ ⊗ τ)`, adding `_⊗_` to `Ty` and `Val (σ ⊗ τ) = Val σ × Val τ`) and `Fst`, `Snd` projections. How does each addition interact with the dependent type of `eval`? What would you need to add to the language to represent recursive functions, and what would that mean for totality of `eval`?

*Abstract concept illustrated: Inductive families as typing derivations; type families as semantic interpretations; de Bruijn indices as Fin-indexed variables; the identity type in equational specifications.*

---

## Exercise C.4: Finite Sets as Types — Fin and Bounded Arithmetic
*Domain: Type-Safe Indexing / Agda or Lean 4*

**Setup:** The type `Fin n` — the type of natural numbers strictly less than `n` — is a fundamental example of a dependent type in practice. It arises whenever you need a safe array index, a bounded register name, or a finite state in an automaton. This exercise develops the arithmetic of `Fin` types and shows how they interact with Vec.

**Questions:**
1. Prove the following facts about `Fin`:
   - `Fin 0` is empty: there are no elements of type `Fin 0`. (State this as `Fin 0 → ⊥` and prove it by pattern matching.)
   - `Fin 1` has exactly one element: give the unique element and prove by pattern matching that it is the only one.
   - `Fin (m + n) ≅ Fin m + Fin n` (as types): define functions `inject : Fin m → Fin (m + n)`, `raise : Fin n → Fin (m + n)`, and show that every element of `Fin (m + n)` is either `inject i` for some `i` or `raise j` for some `j`. (This is the *splitting lemma* for Fin.)

2. Define a function `tabulate : (Fin n → A) → Vec A n` that converts a function on indices to a vector. Prove:
   ```agda
   lookup∘tabulate : ∀ (f : Fin n → A) (i : Fin n) → lookup (tabulate f) i ≡ f i
   ```
   This says that looking up index `i` in the tabulated vector gives you `f i` — the tabulation is correct. Prove the converse too:
   ```agda
   tabulate∘lookup : ∀ (xs : Vec A n) → tabulate (lookup xs) ≡ xs
   ```
   Together, these proofs establish that `Vec A n ≅ (Fin n → A)` — a vector of length `n` is the same as a function from bounded indices. This is the type-theoretic statement that $A^n \cong A^{\{0,\ldots,n-1\}}$.

3. Define a type of $m \times n$ matrices as `Matrix A m n = Vec (Vec A n) m`. Implement:
   - `transpose : Matrix A m n → Matrix A n m`
   - `matMul : Matrix ℕ m n → Matrix ℕ n p → Matrix ℕ m p` (using dot product)
   The types of these functions enforce dimension compatibility: `matMul` requires that the inner dimensions match (both are `n`), and the output has the outer dimensions. What runtime errors does this prevent? Compare to a naive matrix type `[[Int]]` in Haskell.

*Abstract concept illustrated: Fin as a dependent type; the isomorphism `Vec A n ≅ Fin n → A`; dimension-indexed types as a practical application of Sigma types.*

---

## Exercise C.5: The Identity Type in Action — Rewriting and Path Reasoning
*Domain: Proof Theory / Agda or Lean 4*

**Setup:** The identity type `Id_A(a, b)` (written `a ≡ b` in Agda and `a = b` in Lean) is not just a relation — it is an *inductive type* with computational content. Its single constructor `refl : a ≡ a` says "every element is equal to itself." The elimination rule ($J$) says: to prove any property of an equality proof, it suffices to prove it for `refl`. This exercise develops facility with the identity type by proving the standard properties of equality and showing how they arise from the $J$ rule.

**Questions:**
1. Using only `refl` and the $J$ rule (`subst` in Agda, `Eq.mpr` in Lean 4, or pattern matching on `refl`), prove the following without using any library lemmas:
   - Symmetry: `sym : a ≡ b → b ≡ a`
   - Transitivity: `trans : a ≡ b → b ≡ c → a ≡ c`
   - Congruence: `cong : (f : A → B) → a ≡ b → f a ≡ f b`
   - Transport: `transport : (P : A → Set) → a ≡ b → P a → P b`
   For each proof, explain what computation the proof term encodes. For example, `sym p` applied to a value of type `b ≡ a` — what does the computation do?

2. Prove the following properties of the natural numbers using the identity type:
   - `plus-zero : ∀ (n : ℕ) → n + 0 ≡ n`  (note: this requires induction, not just `refl`)
   - `plus-assoc : ∀ (m n p : ℕ) → (m + n) + p ≡ m + (n + p)`
   - `plus-comm : ∀ (m n : ℕ) → m + n ≡ n + m`
   For `plus-comm`, you will need `plus-zero` and the lemma `plus-suc : ∀ (m n : ℕ) → m + suc n ≡ suc (m + n)`. Identify exactly where each identity proof is used.

3. Consider the following use of transport with Vec:
   ```agda
   -- m + n = n + m at the type level, so Vec A (m + n) ≅ Vec A (n + m)
   vec-comm : Vec A (m + n) → Vec A (n + m)
   vec-comm xs = transport (Vec A) plus-comm xs
   ```
   Why is this definition well-typed? What does `transport (Vec A) plus-comm` do? Now implement `vec-comm` without using `transport` — using `append` and reversals instead. Prove that your explicit implementation and the transport-based one compute the same results (i.e., they are propositionally equal). This gap between definitional and propositional equality is one of the central themes of HoTT.

*Abstract concept illustrated: The identity type and the $J$ rule; transport along paths; the distinction between definitional and propositional equality; the preview of path induction in HoTT.*

---

## Exercise C.6: Certified Parsers via Dependent Types
*Domain: Verified Parsing / Agda or Lean 4*

**Setup:** Parsing is a domain where dependent types deliver unusually clear benefits: you can write a parser whose *type* says it accepts exactly the strings in a given language, and the type checker verifies this statically. This exercise develops a simple certified parser for arithmetic expressions and a verified decision procedure for membership in a regular language.

**Questions:**
1. Define a regular expression type and a membership predicate:
   ```agda
   data RegExp : Set where
     ε    : RegExp              -- empty string
     Char : Char → RegExp       -- literal character
     _·_  : RegExp → RegExp → RegExp  -- concatenation
     _∣_  : RegExp → RegExp → RegExp  -- alternation
     _*   : RegExp → RegExp     -- Kleene star
   
   data _∈L_ : List Char → RegExp → Set where
     in-ε    : [] ∈L ε
     in-char : (c ∷ []) ∈L (Char c)
     in-cat  : xs ∈L r → ys ∈L s → (xs ++ ys) ∈L (r · s)
     in-alt-l : xs ∈L r → xs ∈L (r ∣ s)
     in-alt-r : xs ∈L s → xs ∈L (r ∣ s)
     in-star-0 : [] ∈L (r *)
     in-star-s : xs ∈L r → ys ∈L (r *) → (xs ++ ys) ∈L (r *)
   ```
   Verify by example: produce terms of type `('a' ∷ 'b' ∷ []) ∈L (Char 'a' · Char 'b')` and `[] ∈L (Char 'a' *)`.

2. Implement a decision procedure:
   ```agda
   decide : (r : RegExp) (xs : List Char) → Dec (xs ∈L r)
   ```
   where `Dec P = (P ⊎ (P → ⊥))` (a proof of `P` or a refutation). For the base cases `ε` and `Char c`, the decision is straightforward. For concatenation, you need to check all possible splits of `xs` into `(ys, zs)` with `ys ++ zs = xs`. Implement `decide` and test it.

3. Define a type-indexed *parser combinator* library where each parser has type:
   ```agda
   Parser : RegExp → Set
   Parser r = (xs : List Char) → Dec (Σ (List Char) (λ ys → ys ∈L r × ys ++ rest ≡ xs))
   ```
   (i.e., a parser for `r` takes a string and either finds a prefix matching `r` with a proof of membership, or proves no such prefix exists). Implement `pChar : (c : Char) → Parser (Char c)` and `pSeq : Parser r → Parser s → Parser (r · s)`. What does the type of `pSeq` enforce? How does this compare to parsing combinators in Haskell that use `Either String` for error handling?

*Abstract concept illustrated: Inductive families as semantic specifications; the identity type in specifying string equality; the Dec type as a constructive version of decidability; dependent types as certificates of correctness for algorithms.*

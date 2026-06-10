# Applied Exercises

Cubical Agda is not merely a theoretical curiosity — it is the closest thing we have to a *running implementation* of HoTT's mathematics. The computational content of paths, univalence, and higher inductive types has direct applications to verified software: proving that two algorithms are extensionally equal, transporting correctness proofs across equivalent representations, and defining abstract data types that quotient away irrelevant implementation details. The exercises below explore these applications in a working Cubical Agda development. Each begins with a practical motivation before connecting to the chapter's formal content.

---

## Exercise A.1: Paths Between Algorithms — Two Sorting Implementations
*Domain: Software Verification*

**Setup:** In ordinary programming, two functions are "the same" if they produce equal outputs on all inputs. In Cubical Agda, this can be made fully formal: `funExt` (function extensionality) constructs a path between two functions from a pointwise path between their outputs. This exercise formalizes the claim that two implementations of a simple sorting algorithm are equal as functions.

**Questions:**
1. In Cubical Agda, prove `funExt` from first principles (do not import it — derive it yourself):
   ```agda
   funExt : {A : Type} {B : A → Type}
            {f g : (x : A) → B x}
            → ((x : A) → f x ≡ g x)
            → f ≡ g
   funExt h = λ i x → h x i
   ```
   Verify this typechecks. Explain why this works in Cubical Agda but *not* in ordinary Agda with `--without-K` (where `funExt` must be postulated). What is the key difference?

2. Define insertion sort and selection sort on `List ℕ` in Cubical Agda. Both produce sorted output. Define a predicate `Sorted : List ℕ → Type` and a correctness statement `sortCorrect : (sort : List ℕ → List ℕ) → Type` that says the output is a sorted permutation of the input. Now, using `funExt`, prove:
   ```agda
   insertionSort≡selectionSort : insertionSort ≡ selectionSort
   ```
   What do you need to prove about both algorithms first? The key is proving they both produce sorted permutations, and then that the sorted permutation of a list is unique. Where does this uniqueness argument require `isProp` reasoning?

3. Consider the path `insertionSort≡selectionSort : insertionSort ≡ selectionSort`. In Cubical Agda, this is a function `i : I → (List ℕ → List ℕ)`. What is the type of `transport (cong (λ f → f sorts-correctly) insertionSort≡selectionSort) insertionSortCorrect`? Does this actually produce a proof of `selectionSortCorrect`? Trace through the transport.

*Abstract concept illustrated: `funExt` as a derived theorem in Cubical Agda (not an axiom); paths between functions as specifications of equivalence between programs.*

---

## Exercise A.2: Univalence for Abstract Data Types — Transporting Proofs Across Isomorphisms
*Domain: Software Verification / Abstract Data Type Verification*

**Setup:** A stack can be implemented as a `List` or as a persistent array with a pointer. Both implementations satisfy the same abstract interface. In Cubical Agda, an isomorphism between two implementations gives a path between the types (via `ua`), which allows transporting any proof about one implementation to the other for free. This exercise explores this workflow.

**Questions:**
1. Define two representations of a finite multiset of natural numbers:
   ```agda
   -- Implementation 1: sorted list
   SortedList : Type
   
   -- Implementation 2: list with a "this is a bag" equivalence relation
   -- (or just: list quotiented by permutation, if you have truncation)
   ```
   Define an `Iso` (isomorphism) between `SortedList` and some other representation. You may use the `Iso` type from `Cubical.Foundations.Isomorphism`:
   ```agda
   open import Cubical.Foundations.Isomorphism
   record Iso (A B : Type) : Type where
     constructor iso
     field
       fun   : A → B
       inv   : B → A
       rightInv : section fun inv
       leftInv  : retract fun inv
   ```

2. Use `isoToPath` (which applies `ua` to an `Iso`) to construct a path `SortedList ≡ OtherImpl`. Then transport a proof of "membership is decidable" from one implementation to the other:
   ```agda
   membershipDecidable : (n : ℕ) (s : SortedList) → Dec (n ∈ s)
   -- Given the path p : SortedList ≡ OtherImpl, transport this proof:
   membershipDecidable' : (n : ℕ) (s : OtherImpl) → Dec (n ∈ s)
   membershipDecidable' n s = transport (cong (λ T → (n : ℕ) → T → Dec (n ∈ ...)) p) membershipDecidable n s
   ```
   Work out the precise type of the transported proof. Does it typecheck automatically, or do you need to massage the path?

3. In what sense is transport via `ua` "free" versus manually writing a proof for `OtherImpl`? Identify a property of `SortedList` for which transport via `ua` would give you an *incorrect* statement about `OtherImpl` (e.g., a property that refers to the internal structure of the implementation, not just the abstract interface). This is the sense in which `ua`-transport is "parametric" — it only transports properties that respect the isomorphism.

*Abstract concept illustrated: `ua` and `isoToPath` as tools for proof reuse across isomorphic implementations; the limitation that transported properties must be "implementation-independent."*

---

## Exercise A.3: The Integers in Cubical Agda — Path Algebra and Addition
*Domain: Mathematical Formalization*

**Setup:** The integers $\mathbb{Z}$ can be defined in several ways in type theory: as a quotient of $\mathbb{N} \times \mathbb{N}$ (representing $a - b$), or as an inductive type with `pos : ℕ → ℤ`, `negsuc : ℕ → ℤ` (where `negsuc n = -(n+1)`). The cubical library uses the second representation (`Cubical.Data.Int`). This exercise develops addition and commutativity using path algebra directly — without induction on a propositional equality relation.

**Questions:**
1. Open `Cubical.Data.Int` and examine the definition of `_+ℤ_`. Write the cases explicitly:
   ```agda
   -- pos m + pos n = pos (m + n)
   -- negsuc m + negsuc n = negsuc (suc (m + n))
   -- pos 0 + negsuc n = negsuc n
   -- pos (suc m) + negsuc n = ?
   -- negsuc m + pos 0 = negsuc m
   -- negsuc m + pos (suc n) = ?
   ```
   The interesting cases are the "mixed" cases. Write them by performing the correct simplification. (This is the definition — you are not proving anything yet.)

2. Prove commutativity of addition on `ℤ` in Cubical Agda:
   ```agda
   +ℤ-comm : (m n : ℤ) → m +ℤ n ≡ n +ℤ m
   ```
   The proof is by pattern matching on both `m` and `n` (four cases each), reducing to commutativity of `Nat._+_`. In each case, the path is constructed explicitly. Notice that in Cubical Agda, you are constructing a *function* `I → ℤ` in each case — not just a propositional equality certificate.

3. Define `sucℤ : ℤ → ℤ` and `predℤ : ℤ → ℤ` (successor and predecessor), and prove:
   ```agda
   sucℤ-predℤ : (n : ℤ) → sucℤ (predℤ n) ≡ n
   predℤ-sucℤ : (n : ℤ) → predℤ (sucℤ n) ≡ n
   ```
   Use these to construct `Iso ℤ ℤ` via `sucℤ`/`predℤ`, and observe that `isoToPath` gives you a non-trivial path `ℤ ≡ ℤ`. What does `transport (isoToPath sucIso) (pos 3)` reduce to? Verify with the Agda interactive mode.

*Abstract concept illustrated: Path algebra for constructing concrete paths; `Iso` and `isoToPath` in practice; the computation rule for transport along `ua`.*

---

## Exercise A.4: Implementing `funExt` and Verifying Two Function Implementations
*Domain: Software Verification / Functional Programming*

**Setup:** Function extensionality in Cubical Agda is not merely a logical tool — it is a verification strategy. Two functions are equal if they agree on all inputs, and this equality is witnessed by a path that can be *used* in other proofs (transported, composed, and inverted). This exercise uses `funExt` to verify that two implementations of a recursive function are definitionally or propositionally equal.

**Questions:**
1. Define two implementations of `reverse : List A → List A` — one using `reverse-helper` (tail-recursive with accumulator) and one using `_++_` (naive quadratic). Prove they are equal:
   ```agda
   reverse₁ : List A → List A
   reverse₁ []       = []
   reverse₁ (x ∷ xs) = reverse₁ xs ++ [ x ]
   
   reverse₂ : List A → List A  -- tail-recursive
   reverse₂ = go []
     where go acc []       = acc
           go acc (x ∷ xs) = go (x ∷ acc) xs
   
   reverse₁≡reverse₂ : (xs : List A) → reverse₁ xs ≡ reverse₂ xs
   ```
   Prove this by induction on `xs`. Use `funExt` to get `reverse₁ ≡ reverse₂ : List A → List A`.

2. Use `cong` and path composition to prove that if `f ≡ g : A → B`, then for any `h : B → C`, we have `h ∘ f ≡ h ∘ g : A → C`. Write this as a theorem using `funExt` and `cong`:
   ```agda
   cong-∘ : {A B C : Type} {f g : A → B} (h : B → C)
            → f ≡ g → h ∘ f ≡ h ∘ g
   ```
   Apply this to prove: `length ∘ reverse₁ ≡ length ∘ reverse₂` — i.e., the length of a reversed list is the same whether you use the naive or tail-recursive reverse.

3. In Lean 4, to prove `reverse₁ = reverse₂`, you would prove `∀ xs, reverse₁ xs = reverse₂ xs` and then use `funext` (which is an axiom in Lean 4). In Cubical Agda, `funExt` is a *derived* theorem — it follows from the path definition. Exhibit this derivation explicitly: write `funExt h = λ i x → h x i` and verify it typechecks. Then explain why this wouldn't work in `--without-K` Agda (without cubical mode): what would happen when you try to typecheck `λ i x → h x i`?

*Abstract concept illustrated: `funExt` as a derived theorem; `cong` and path composition; the difference between axiomatic and computational function extensionality.*

---

## Exercise A.5: A Higher Inductive Type for the Rationals
*Domain: Mathematical Formalization / Abstract Algebra*

**Setup:** The rational numbers $\mathbb{Q}$ can be defined as the quotient of $\mathbb{Z} \times \mathbb{Z}_{>0}$ by the equivalence relation $(p, q) \sim (p', q')$ iff $p \cdot q' = p' \cdot q$. In Cubical Agda, this is a *higher inductive type* — a type with a point constructor and a path constructor for the equivalence relation, combined with a truncation to make it a set. This exercise builds the rationals as a HIT.

**Questions:**
1. In Cubical Agda, define the rationals as a set quotient:
   ```agda
   open import Cubical.HITs.SetQuotients
   
   -- A pair (numerator, positive denominator)
   ℚ-pair : Type
   ℚ-pair = ℤ × ℕ₊₁  -- ℕ₊₁ is {n : ℕ | 1 ≤ n} or ℕ with a shifted interpretation
   
   -- The equivalence relation
   _∼_ : ℚ-pair → ℚ-pair → Type
   (p , q) ∼ (p' , q') = p *ℤ (ℕ₊₁.val q') ≡ p' *ℤ (ℕ₊₁.val q)
   
   ℚ : Type
   ℚ = ℚ-pair / _∼_
   ```
   Verify this definition loads. Then define the canonical injection `fromInt : ℤ → ℚ` and the fraction constructor `_/_ : ℤ → ℕ₊₁ → ℚ`.

2. Prove that addition on `ℚ` is well-defined (respects the equivalence relation). In a set quotient `A / R`, to define a function `f : A / R → B`, you must show `f` is constant on equivalence classes: if `a ∼ a'` then `f [a] = f [a']`. Write the addition function:
   ```agda
   _+ℚ_ : ℚ → ℚ → ℚ
   ```
   using `SetQuotient.rec` or `SetQuotient.rec2`, supplying the proof that addition respects `_∼_`.

3. The key identity needed for well-definedness of `+ℚ` is the following: if $(p, q) \sim (p', q')$ and $(r, s) \sim (r', s')$, then $(ps + rq, qs) \sim (p's' + r'q', q's')$. This reduces to a ring identity in $\mathbb{Z}$. Prove this identity using `ring` (from `Cubical.Tactics.RingSolver` or similar) or by `zsolve`. What algebraic structure are you using?

*Abstract concept illustrated: Set quotients as HITs; the recursion principle for quotient types and the well-definedness condition; connecting HIT constructors to algebraic verification.*

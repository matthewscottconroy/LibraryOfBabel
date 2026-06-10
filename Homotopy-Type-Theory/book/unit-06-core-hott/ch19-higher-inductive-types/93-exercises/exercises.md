# Exercises: Higher Inductive Types

## Interval Exercises

**Exercise 1.** Prove that the interval I is contractible by constructing the center and contracting homotopy explicitly. Verify the transport condition `apd_h(seg) : transport^{0=(−)}(seg, refl_0) = seg` holds.

**Exercise 2.** Use the interval to prove function extensionality: for f, g : A → B with H : Π(x:A).f(x)=g(x), construct an explicit path f = g using the I-eliminator and currying.

**Exercise 3.** Show that the interval I has exactly two points "up to homotopy" by proving that every element x : I is equal to either 0_I or 1_I. (Hint: use the I-eliminator with C(x) = (x = 0_I) ∨ (x = 1_I)... actually, this is tricky. Show instead that I is contractible, which implies there is only one point up to homotopy.)

**Exercise 4.** Show that the product I × I (the square) is contractible. (It is contractible as a product of contractible types.)

## Circle Exercises

**Exercise 5.** Define the constant map const_b : S^1 → B (for any b : B) using the S^1 eliminator. What data do you need to provide?

**Exercise 6.** Define the "winding n times" map wₙ : S^1 → S^1 for any n : Z, using the S^1 eliminator. Compute w₀ (the constant map) and w₁ (the identity map) explicitly.

**Exercise 7.** Show that w_m ∘ w_n = w_{m+n}: the composition of winding maps winds (m+n) times. (This is a statement about maps S^1 → S^1 and should be proved using funext.)

**Exercise 8.** Prove that `loop ≠ refl_base` in S^1 — the loop is not trivial. (Hint: define a type family code : S^1 → Type with code(base) = Bool and transport along loop = negation. Transport refl_base gives true = true, while transport loop gives true = false. The two must be different.)

## Suspension Exercises

**Exercise 9.** Define the suspension ΣA formally and write out its eliminator explicitly. Show that for any map f : A → B and any b₀, b₁ : B with p(a) : b₀ = b₁ for each a : A, there is an induced map h : ΣA → B.

**Exercise 10.** Show that ΣBool ≃ S^1 by constructing explicit homotopy equivalences in both directions.

**Exercise 11.** Prove that Σ(A+B) ≃ ΣA ∨ ΣB (the suspension of a coproduct is the wedge of the suspensions).

**Exercise 12.** Show that S^2 = ΣS^1 has a contractible 1-path type: for any x, y : S^2, if x = y then the path space x = y is contractible. (This is the statement that S^2 is simply connected.)

## Pushout Exercises

**Exercise 13.** State and prove the universal property of pushouts: maps from the pushout P of A ←^f C →^g B to any type D correspond to pairs (hA : A→D, hB : B→D) with a homotopy `Π(c:C). hA(f(c)) = hB(g(c))`.

**Exercise 14.** Show that the pushout of 1 ← 1 → 1 (the pushout of two maps from 1 to 1) is the circle S^1... wait, this is not quite right (it gives I ≃ 1 or S^1 depending on how the maps are set up). Be careful: describe exactly which pushout gives S^1.

**Exercise 15.** Prove van Kampen for a simple case: show that π₁(S^1 ∨ S^1) = Z * Z (the free product of Z with itself) using the pushout presentation of the figure-eight (S^1 ∨ S^1 = two circles joined at a point).

## Truncation Exercises

**Exercise 16.** Prove the universal property of propositional truncation: for any proposition P, the map (‖A‖ → P) → (A → P) given by precomposition with |−| : A → ‖A‖ is an equivalence.

**Exercise 17.** Show that ‖‖A‖‖ = ‖A‖ — propositional truncation is idempotent.

**Exercise 18.** Show that for a proposition P, ‖P‖ = P. (The propositional truncation of a proposition is itself.)

**Exercise 19 (Proof-Level).** Prove that ‖A + B‖ ≃ ‖A‖ ∨ ‖B‖, where ∨ denotes propositional disjunction (= ‖A + B‖... wait, this is circular). More carefully: ‖A + B‖ is equivalent to ‖A‖ ∨ ‖B‖ where ∨ is defined as ‖‖A‖ + ‖B‖‖. Show both directions.

**Exercise 20.** Show that if A is merely inhabited (‖A‖ is inhabited) and P : A → Prop is any predicate, then `‖Σ(a:A).P(a)‖ → ‖∃a.P(a)‖` but the converse requires the axiom of choice.

## Proof-Level Exercises

**Exercise 21 (Proof-Level).** Prove that S^n has h-level n+1 (is an n-type) for n = 0 and n = 1. That is, show isSet(Bool) (h-level 2 = 0+2... wait, the indexing: S^0 = Bool is a set = h-level 0; S^1 is a 1-type = h-level 1; S^n is an n-type = h-level n). Prove:
(a) Bool is a set (isSet(Bool))
(b) S^1 is a 1-type (all path types in S^1 are sets, i.e., base = base ≃ Z is a set)

**Exercise 22 (Proof-Level).** Prove that the total space of any HIT fibration is a HIT. Specifically: if X is a HIT and P : X → Type is a family, then `Σ(x:X).P(x)` can be given a HIT presentation by adding to X's constructors the corresponding point constructors in Σ(x:X).P(x), and adding path constructors corresponding to the path constructors of X.

**Exercise 23 (Proof-Level).** Prove that the Eilenberg-MacLane space K(Z,1) is equivalent to S^1. (Hint: K(Z,1) = BZ, the delooping of Z. The fundamental group of S^1 is Z, so S^1 has the right fundamental group. Show that S^1 is a K(Z,1) by showing it is a 1-type with π₁ = Z.)

**Exercise 24 (Proof-Level).** Prove that ‖S^n‖_k is contractible for k < n and is S^n for k ≥ n. (The n-sphere has no homotopy in dimensions below n.) This requires knowing the homotopy groups of S^n, which is deep — so instead prove the easy cases: ‖S^1‖ = 1 (the circle is connected) and ‖S^1‖_{-1} = 1 (trivially, since S^1 is non-empty).

**Exercise 25 (Proof-Level).** Prove that the join A * B is equivalent to the suspension ΣA when B = Bool. More precisely: Bool * A ≃ ΣA. Construct the equivalence explicitly.

**Exercise 26 (Proof-Level).** State and prove the following fact: if X is a HIT defined by point constructors c_i and path constructors p_j, then any two HITs with the same generators are equivalent. This is the "initiality" of HITs: the HIT is the initial algebra for its generators. (This is a deep theorem — Lumsdaine-Shulman — so just state it precisely and sketch the idea.)

**Exercise 27 (Challenge).** Prove that π₂(S^2) = Z using the encode-decode method: define a code family code : S^2 → Type with code(base) = Z, and show that the encoding of 2-loops gives integers and decoding integers gives 2-loops. (This requires knowing that S^2 is simply connected, which you should use as a black box.)

**Exercise 28 (Challenge).** The *James construction* JA is the free topological monoid on A (with unit). In HoTT, JA can be defined as the colimit of A^n (n-fold products) with the monoid structure. Show that J(S^1) ≃ ΩΣ(S^1) — the James construction of the circle is equivalent to the loop space of the suspension of the circle. (This is a theorem in algebraic topology and its HoTT proof uses the Σ ⊣ Ω adjunction.)

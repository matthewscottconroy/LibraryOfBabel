# Applied Exercises

Lean 4 and Mathlib are not merely tools for verifying abstract mathematics in isolation — they are increasingly used in connection with real mathematical and computational problems where the cost of error is high. The exercises below place Lean 4's features in applied contexts: algorithm correctness, cryptography, compiler verification, and research-level mathematics. Each exercise is designed to be carried out in a Lean 4 project with Mathlib; the point is to see how the abstract machinery of tactics, typeclasses, and automation maps onto genuine mathematical and engineering problems.

---

## Exercise A.1: Rank-Nullity from Scratch — Linear Algebra in Mathlib
*Domain: Mathematical Formalization*

**Setup:** The rank-nullity theorem is a cornerstone of linear algebra: for a linear map $f : V \to W$ of finite-dimensional vector spaces, $\dim(\ker f) + \dim(\text{im } f) = \dim V$. In Mathlib, the relevant theorem is `Submodule.finrank_quotient_map` or `LinearMap.finrank_range_add_finrank_ker`. Before looking it up, formalize the statement from scratch to understand how Mathlib represents linear maps and finite-dimensional spaces.

**Questions:**
1. In Lean 4, write the type signature of a statement equivalent to rank-nullity. What typeclasses are needed? (You will need `Module`, `FiniteDimensional`, and `Module.finrank`.) Write the statement without looking at Mathlib — use `#check Module.finrank`, `#check LinearMap.ker`, `#check LinearMap.range` to discover the types.
2. Search Mathlib (using `exact?`, Loogle with query `Module.finrank _ + Module.finrank _ = Module.finrank _`, or Moogle) to find the actual Mathlib lemma. Write its full Lean 4 type signature. How does it differ from what you wrote in (1)?
3. Use the Mathlib lemma to prove the following corollary: if $f : \mathbb{R}^n \to \mathbb{R}^m$ is an injective linear map, then $n \leq m$. (This is `LinearMap.finrank_le_finrank_of_injective` or similar.) Formalize this as a `theorem` in Lean 4.

*Abstract concept illustrated: `#check` and `#search` as tools for type-directed proof search; the gap between informal mathematical statements and their formalized type signatures.*

---

## Exercise A.2: Integer Inequalities in Algorithm Analysis via `omega`
*Domain: Software Verification / Algorithm Analysis*

**Setup:** Many algorithm analysis arguments reduce to elementary integer inequalities. For instance: proving that a binary search on an array of size $n$ terminates (because $\lfloor n/2 \rfloor < n$ for $n \geq 1$), or that a sorting algorithm's comparison count satisfies $c \leq n(n-1)/2$. The `omega` tactic in Lean 4 decides linear arithmetic over integers and naturals automatically. This exercise uses `omega` to verify such arguments.

**Questions:**
1. Prove the following directly with `omega` (no manual steps):
   ```lean
   theorem halving_terminates (n : ℕ) (h : 1 ≤ n) : n / 2 < n := by omega
   theorem merge_count (n m : ℕ) : n + m ≤ n * m + 1 → True := by omega
   theorem loop_bound (i n : ℤ) (h₁ : 0 ≤ i) (h₂ : i < n) : 2 * i + 1 ≤ 2 * n - 1 := by omega
   ```
   Which of these does `omega` prove immediately? Which requires a hypothesis?

2. The following is the key lemma for proving that Euclidean algorithm terminates: if $0 < b \leq a$, then $a \mod b < a$. Try to prove `Nat.mod_lt` from scratch using `omega`. Does `omega` handle nonlinear arithmetic (involving `*` or `mod`)? What is the boundary of `omega`'s power?

3. For a merge sort on an array of length $n$: formalize the statement "the number of comparisons in merge sort is at most $n \cdot \lceil \log_2 n \rceil$" as a Lean 4 `def` or `theorem` stub. Even if you cannot prove it automatically, write the type signature precisely. What does this tell you about the limits of `omega` versus `nlinarith` or manual induction?

*Abstract concept illustrated: Decision procedures as proof automation; the `omega` tactic as a complete decision procedure for Presburger arithmetic.*

---

## Exercise A.3: Polynomial Identities in Cryptography via `ring`
*Domain: Type-Safe Programming / Cryptographic Verification*

**Setup:** Many cryptographic primitives — RSA key generation, elliptic curve arithmetic, Shamir's secret sharing, polynomial commitments — depend on algebraic identities in polynomial rings or finite fields. Errors in these identities are security vulnerabilities. The `ring` tactic in Lean 4 proves polynomial identities in any `CommRing` automatically, by normalization to a canonical form. This exercise verifies some algebraic foundations of cryptographic protocols.

**Questions:**
1. The Shamir secret sharing scheme uses the fact that a polynomial of degree $k-1$ is determined by $k$ points. A key algebraic fact is the Lagrange interpolation formula. Prove the following identity in any `CommRing` using `ring`:
   ```lean
   example (R : Type*) [CommRing R] (a b : R) :
       (a + b)^3 = a^3 + 3*a^2*b + 3*a*b^2 + b^3 := by ring
   
   example (R : Type*) [CommRing R] (a b c : R) :
       (a - b) * (a^2 + a*b + b^2) = a^3 - b^3 := by ring
   ```
   Confirm that `ring` works here. Now try removing the `CommRing` hypothesis and replacing it with `Ring`. Does `ring` still work? Why or why not?

2. In RSA, the correctness relies on Euler's theorem: $a^{\phi(n)} \equiv 1 \pmod{n}$ for $\gcd(a, n) = 1$. In Lean 4 / Mathlib, this is `ZMod.units_pow_card_sub_one_eq_one` or similar. Find the relevant lemma and write down its type. Then prove a consequence: if $p$ is a prime and $a \not\equiv 0 \pmod{p}$, then $a^{p-1} \equiv 1 \pmod{p}$ (Fermat's little theorem). Use `Mathlib.NumberTheory.LucasPrimality` or `ZMod.pow_card_sub_one_eq_one` as appropriate.

3. In elliptic curve cryptography, the group law is defined by polynomial formulas on pairs $(x, y) \in k^2$ satisfying $y^2 = x^3 + ax + b$. Define the type of points on an elliptic curve in Lean 4 (including the point at infinity), and prove that the negation formula $-(x, y) = (x, -y)$ satisfies $-(-(P)) = P$ using `ring` and the curve equation. (You don't need to prove the full group law — just this involution.)

*Abstract concept illustrated: The `ring` tactic as a decision procedure for the equational theory of commutative rings; connecting algebraic automation to security-critical verification.*

---

## Exercise A.4: Building Group Theory from Axioms
*Domain: Mathematical Formalization / Understanding Infrastructure*

**Setup:** Mathlib's `Group` typeclass packages many derived lemmas — `mul_comm` for abelian groups, `inv_mul_cancel`, `mul_left_cancel`, etc. But understanding *why* these hold requires building the theory from axioms, without automation. This exercise formalizes core group theory from scratch, using only the group axioms as hypotheses, to develop intuition for what Mathlib automates behind the scenes.

**Questions:**
1. Without using any Mathlib group lemmas (only `exact`, `apply`, `rw`, and `constructor`), prove the following in a `variable [Group G]` context:
   ```lean
   -- (a) Left identity implies right identity
   theorem right_identity (G : Type*) [Group G] (a : G) : a * 1 = a := by
     -- Hint: use mul_left_cancel and mul_assoc and inv_mul_cancel
     sorry
   
   -- (b) Left inverse implies right inverse
   theorem right_inverse (G : Type*) [Group G] (a : G) : a * a⁻¹ = 1 := by
     sorry
   ```
   For each, write the proof yourself before using `exact?`. After solving it, compare your proof to what `group` or `simp [mul_comm]` produces.

2. Prove from the group axioms (no Mathlib group lemmas) that inverses are unique: if $ab = e$ and $ac = e$, then $b = c$. What does this tell you about the structure of the group axioms — are they independent?

3. Define a `Monoid` structure from scratch (as a `structure`, not using Mathlib's typeclass), and prove that the list `[a, b, c]` under concatenation forms a monoid. What properties must you verify? Write the proof that `List.append_assoc` and `List.nil_append` and `List.append_nil` give you the monoid axioms.

*Abstract concept illustrated: The difference between using Mathlib automation and building from axioms; the `group` tactic as a decision procedure for the free group theory.*

---

## Exercise A.5: Finding and Adding a Missing Mathlib Lemma
*Domain: Mathematical Formalization / Open-Source Contribution*

**Setup:** Mathlib is large but not complete. A common experience when formalizing mathematics is to need a lemma that seems standard but is not in Mathlib — or to find a lemma that is present in one form but not in the form you need. This exercise simulates the workflow of identifying such a gap and filling it, following Mathlib's contribution guidelines.

**Questions:**
1. The following lemma should be provable from Mathlib primitives but may not be stated in exactly this form. Try to find it with Loogle or `exact?`. If it exists, record its Mathlib name. If it does not, prove it from available lemmas:
   ```lean
   -- "The sum of the first n odd numbers is n^2"
   theorem sum_odd_eq_sq (n : ℕ) :
       (Finset.range n).sum (fun k => 2 * k + 1) = n ^ 2 := by
     sorry
   ```
   What tactics help here? Try `induction`, `simp`, `ring`, `omega`. Which combination works?

2. Mathlib has a naming convention: lemmas are named by their conclusion and hypotheses, separated by underscores, following a `subject_verb_object` pattern. Given the following informal statements, predict what Mathlib's name might be, then check with Loogle:
   - "In a linear order, $a \leq b$ and $b \leq a$ implies $a = b$"
   - "The image of a compact set under a continuous map is compact"
   - "A finite integral domain is a field"

3. Suppose the following lemma is missing from Mathlib (it may or may not be — check first). Write it in the correct Mathlib style (with docstring, correct namespace, and using existing lemmas as building blocks):
   ```lean
   /-- In a commutative monoid, the product over a Finset is independent
       of the order of enumeration. -/
   theorem Finset.prod_comm' ...
   ```
   What steps would you take to actually submit this to Mathlib? (Describe the `lake update`, branch creation, `leanprover-community/mathlib4` PR workflow.)

*Abstract concept illustrated: Mathlib's naming conventions and contribution workflow; `exact?` and Loogle as search-first tools.*

---

## Exercise A.6: Verified Finite Computation with `decide`
*Domain: Software Verification / Combinatorics*

**Setup:** The `decide` tactic proves propositions by reducing them to a computation that terminates with `true` or `false`. It works for any `Decidable` proposition — which in Lean 4 includes all quantifiers over finite types, decidable equality, and decidable predicates. This is a powerful tool for verifying small finite cases of combinatorial or number-theoretic claims without manual proof.

**Questions:**
1. Use `decide` to prove the following (all should work immediately):
   ```lean
   example : Nat.Prime 97 := by decide
   example : ¬ Nat.Prime 100 := by decide
   example : (Finset.range 10).card = 10 := by decide
   example : ∀ b : Bool, b || true = true := by decide
   -- What is the largest prime you can check with `decide` before it times out?
   ```
   Experiment with `Nat.Prime 997`, `Nat.Prime 9973`, `Nat.Prime 99991`. At what size does `decide` become impractical, and what tactic replaces it (`norm_num`)?

2. The four-color theorem is too large for `decide`, but small graph coloring instances are not. Define the type of 3-colorings of a graph on 4 vertices (the complete graph $K_4$), and use `decide` to verify that $K_4$ is *not* 3-colorable (it requires 4 colors). You'll need to define the graph as a `Finset`-based adjacency relation.

3. Explain the relationship between `decide` and the kernel of Lean 4. When `decide` proves `P`, what exactly has the kernel checked? Is the resulting proof term small or large? (Use `#print` to inspect a proof term generated by `decide` on a small example.) Why is `norm_num` preferred over `decide` for large numerical computations?

*Abstract concept illustrated: `Decidable` propositions and the `decide` tactic; the difference between kernel-checked computation and tactic-generated proof terms.*

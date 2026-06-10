# Exercises: Identity Types and Paths

## Conceptual Exercises

**Exercise 1.** State precisely why `refl_a : a =_A a` is the *only* axiomatically given element of any identity type. Why can't we axiomatically give a path from 0 to 1 in the natural numbers?

**Exercise 2.** The J rule has a more symmetric formulation (sometimes called J' or the "path induction from both ends"): to prove C(a, b, p) for all a, b : A and p : a = b, it suffices to prove C(a, a, refl_a). Show that this symmetric J' can be derived from the standard J rule (which fixes the left endpoint).

**Exercise 3.** Show that the contractibility of the total path space `Σ(b:A). (a =_A b)` follows from the J rule. Specifically, construct the contracting homotopy that connects any `(b, p)` to the center `(a, refl_a)`.

**Exercise 4.** We defined path concatenation by J-induction on the second argument. What would happen if we defined it by induction on the *first* argument? Write out the alternative definition and its computation rule. Show that the two definitions are propositionally equal (related by a 2-path).

**Exercise 5.** Let A be any type and a : A. Show that the path space `a =_A a` (the loop space at a) forms a group under concatenation and inversion. Identify the identity element, the binary operation, and verify all group axioms hold (propositionally).

## Path Operation Exercises

**Exercise 6.** Prove the left unit law: for p : a = b, `refl_a · p = p`. Give an explicit proof term using the J rule.

**Exercise 7.** Prove associativity: for p : a = b, q : b = c, r : c = d, `(p · q) · r = p · (q · r)`. Identify which base case you reduce to and why it holds.

**Exercise 8.** Prove that inversion is an involution: for p : a = b, `(p⁻¹)⁻¹ = p`. Identify the J-induction and base case.

**Exercise 9.** Show that `ap_f(p · q) = ap_f(p) · ap_f(q)` for f : A → B and p : a = b, q : b = c. This establishes that ap_f is a groupoid homomorphism.

**Exercise 10.** Show that `ap_f(p⁻¹) = (ap_f(p))⁻¹`. Prove this by J-induction on p.

**Exercise 11.** Show that `ap_{g∘f}(p) = ap_g(ap_f(p))`. This shows that composition of functions corresponds to composition of their actions on paths.

**Exercise 12.** Prove the transport-concatenation law: for p : a = b and q : b = c:
```
transport^P(q, transport^P(p, x)) = transport^P(p · q, x)
```
for any x : P(a). Prove by J-induction on q.

## Sigma-Type Path Exercises

**Exercise 13.** Let A be a type, B : A → Type. Prove that a path `(a₁, b₁) = (a₂, b₂)` in `Σ(x:A).B(x)` gives a path `p : a₁ = a₂` in A together with a path `transport^B(p, b₁) = b₂`. Construct the function explicitly using ap and the appropriate transport.

**Exercise 14.** Prove the converse of Exercise 13: from `p : a₁ = a₂` and `q : transport^B(p, b₁) = b₂`, construct a path `(a₁, b₁) = (a₂, b₂)` in `Σ(x:A).B(x)`. Use J-induction on p and then on q.

**Exercise 15.** Using the result of Exercises 13 and 14, show that paths in `Σ(x:A).isProp(B(x))` (the subtype of A where B holds) are equivalent to paths in A (that land in the subtype). In other words, the inclusion of the subtype is a path-embedding.

## Higher Path Exercises

**Exercise 16.** Let α : p = q and β : q = r be 2-paths (paths between paths). Show that α and β can be concatenated: α · β : p = r. What type is this concatenation?

**Exercise 17.** Define the vertical and horizontal composition of 2-paths at the loop space. Then verify that these compositions satisfy the interchange law: `(α ·ᵥ β) ★ (γ ·ᵥ δ) = (α ★ γ) ·ᵥ (β ★ δ)`.

**Exercise 18.** Carry out the Eckmann-Hilton argument in detail: show that for α, β : refl_a = refl_a (elements of Ω²A), vertical and horizontal composition coincide: `α ·ᵥ β = α ★ β`. Then use this to conclude commutativity: `α ·ᵥ β = β ·ᵥ α`.

## Proof-Level Exercises

**Exercise 19 (Proof-Level).** Prove that the based path space `Σ(b:A).(a = b)` is contractible without using the characterization of Sigma-type paths (i.e., give a direct proof using J). Your proof should give an explicit center and contracting homotopy.

**Exercise 20 (Proof-Level).** Prove the naturality of homotopies: for H : f ~ g and p : a = b:
```
ap_g(p) · H(b) = H(a) · ap_f(p)
```
Do this by J-induction on p. Identify the base case and why it holds.

**Exercise 21 (Proof-Level).** Prove that transport^B(p · q) = transport^B(q) ∘ transport^B(p) — that is, transport over a concatenated path is composition of the individual transports (in the appropriate order). Careful with the order: you are transporting from P(a) to P(b) to P(c), and the composition should go in the right direction.

**Exercise 22 (Proof-Level).** Prove that for any type A and any a : A, the fundamental groupoid of A (with points as objects and paths as morphisms) satisfies all the groupoid axioms. This means: (i) composition is well-defined, (ii) the identity morphisms exist, (iii) all morphisms are invertible, (iv) composition is associative (propositionally), (v) unit laws hold (propositionally). Collect the proofs from the sections above into a systematic verification.

**Exercise 23 (Proof-Level).** Let f : A → B be a function and H : f ~ f be a self-homotopy of f. Show that for any path p : a = a (a loop at a), the naturality of H gives:
```
ap_f(p) · H(a) = H(a) · ap_f(p)
```
and conclude that H(a) commutes with ap_f(p) in the group of loops at f(a). What does this say about the center of the fundamental group of B?

**Exercise 24 (Proof-Level).** Prove that every function f : A → B preserves concatenation:
```
ap_f(p · q) = ap_f(p) · ap_f(q)
```
by J-induction on p. Show the key base case explicitly: when p = refl_a, both sides reduce to ap_f(q).

**Exercise 25 (Proof-Level).** Prove that `apd_f(p)` and `ap_f(p)` are related as follows: for a dependent function f : Π(x:A).B(x) where B is a constant family B(x) = C, the dependent action apd_f(p) is related to the ordinary action by:
```
apd_f(p) = transport_const(p, f(a)) · ap_f(p)
```
where `transport_const(p, -)` is the path witnessing that transport in a constant family is the identity. (This shows that apd generalizes ap.)

**Exercise 26.** For the identity function id_A : A → A, show that ap_{id_A}(p) = p for all p : a = b. For the composition g∘f : A → C, show that ap_{g∘f}(p) = ap_g(ap_f(p)).

**Exercise 27.** A pointed type is a pair (A, a) where a : A is the basepoint. Define the type of pointed maps from (A, a) to (B, b) as the type of pairs (f : A → B, e : f(a) = b). Show that a pointed map (f, e) gives a map ap_f on loop spaces, specifically a group homomorphism:
```
Ω(f, e) : Ω(A, a) → Ω(B, b)
```
defined by Ω(f, e)(p) = e⁻¹ · ap_f(p) · e. Verify that this is a group homomorphism.

**Exercise 28 (Challenge).** The Whitehead principle for sets states: a function f : A → B between sets (h-level 0 types) is an equivalence iff it is an isomorphism on π₀. Prove a version of this: if A and B are sets and f : A → B satisfies (i) for every b : B, the fiber fib_f(b) is nonempty, and (ii) f is injective (i.e., ap_f is an equivalence on path types), then f is an equivalence. Use the path characterization of product types and the fact that sets have discrete path structure.

**Exercise 29 (Challenge).** Let A be a type with a · b : A for each a, b : A (a "magma" structure), and suppose p : a = a' and q : b = b'. Define the path:
```
ap₂_(-·-)_{p,q} : a · b = a' · b'
```
by ap in the two-variable function (-·-). Show that this is natural in p and q, and that it respects the groupoid structure: the path induced by p · p' and q · q' equals the concatenation of the paths induced by (p, q) and (p', q').

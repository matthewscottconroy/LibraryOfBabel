# Exercises: Synthetic Homotopy Theory

## Section 1: The Encode-Decode Method

**Exercise 1.** (Warm-up) Let X be a type with basepoint x₀ : X. Define code : X → Type and encode : Π(x:X). (x₀ = x) → code(x) for each of the following choices:
(a) X = Bool, x₀ = false, code should distinguish true from false.
(b) X = N (the natural numbers), x₀ = 0, code(n) = Fin(n+1).
For each case, what does encode compute?

**Exercise 2.** In the encode-decode method, we have:
- encode(x, p) := transport^code(p, c₀) where c₀ : code(x₀)
- decode(x, c) : x₀ = x (requires an X-eliminator)

Write out the conditions that decode must satisfy when X = S¹ (the circle). What is the "transport condition" at the loop constructor?

**Exercise 3.** Prove that for any type X and basepoint x₀ : X, if code : X → Type satisfies:
(a) code is a proposition (isProp(code(x)) for all x), and
(b) there is an equivalence (x₀ = x) ≃ code(x) for all x,
then X is a set. [Hint: every loop at x₀ encodes to an element of code(x₀), and if code(x₀) is a proposition, all loops are equal.]

**Exercise 4.** Let f : A → B be an equivalence. Construct a code family code_f : A → Type such that the encode-decode method directly gives the proof that Σ(x:A). code_f(x) is contractible. What is code_f?

**Exercise 5.** The encode-decode method applied to based path spaces gives: for any x₀ : X, the based path space Σ(x:X). (x₀ = x) is contractible. Prove this directly using the encode-decode method with code(x) := (x₀ = x). [This gives an alternative to the direct proof using J.]

**Exercise 6.** (Reflection) Explain the sense in which the code family for S¹ (code(base) = Z, transport^code(loop) = succ) encodes the "winding number" interpretation of loops. What would happen if you chose transport^code(loop) = pred instead of succ? What theorem would you prove?

## Section 2: π₁(S¹) = Z

**Exercise 7.** In the proof of π₁(S¹) = Z, we defined code : S¹ → Type by:
- code(base) := Z
- ap_code(loop) := ua(succ)

(a) Compute transport^code(loop⁻¹ · loop) starting from n : Z.
(b) Compute transport^code(loop² · loop⁻¹ · loop) starting from 0 : Z.
(c) Show transport^code(loop^n) = succ^n (the n-th iterate of successor) for all n : Z by integer induction.

**Exercise 8.** Define encode : (base =_{S¹} base) → Z by encode(p) = transport^code(p, 0) and decode : Z → (base =_{S¹} base) by decode(n) = loop^n. Verify the following round-trip equations:
(a) encode(decode(0)) = 0
(b) encode(decode(1)) = 1
(c) encode(decode(-1)) = -1
(d) For general n, how does the proof proceed?

**Exercise 9.** The decode function is defined by integer induction:
- decode(0) := refl_base
- decode(n+1) := decode(n) · loop
- decode(-1) := loop⁻¹
- decode(-(n+1)) := decode(-n) · loop⁻¹

(a) Prove that decode(n + m) = decode(n) · decode(m) for all n m : Z. [This says decode is a group homomorphism from (Z, +) to (Ω(S¹, base), ·).]
(b) Prove that decode(-n) = (decode(n))⁻¹ for all n : Z.

**Exercise 10.** The proof that encode ∘ decode = id uses path induction (J). Write out the J-application explicitly:
- What is the motive C(x, p) for the path induction?
- What is the base case that needs to be verified?
- Why does J then give the full result?

**Exercise 11.** Let f : S¹ → S¹ be any map with f(base) = base. Define the degree of f as deg(f) := encode(ap_f(loop)). Prove:
(a) deg(id_{S¹}) = 1.
(b) deg(const_{base}) = 0 (the constant map at base).
(c) deg(f ∘ g) = deg(f) + deg(g) for any two based maps f, g : S¹ → S¹.
[Hint for (c): trace what happens to loop under f ∘ g.]

**Exercise 12.** (Proof-level) Prove that π₁(S¹, base) is abelian. [Hint: π₁(S¹) = Z is abelian since Z is abelian. But give a direct proof using the group structure of the loop space and the equivalence with Z.]

**Exercise 13.** (Proof-level) Show that for the code family code : S¹ → Type defined in the proof:
(a) Σ(x:S¹). code(x) is equivalent to R (or rather, to a discrete type equivalent to Z... think carefully).
(b) The projection Σ(x:S¹). code(x) → S¹ is equivalent to the Hopf map S¹ → S¹ → S¹ (the covering map). [This connects the encode-decode proof to the classical proof via covering spaces.]

## Section 3: The Van Kampen Theorem

**Exercise 14.** Compute the fundamental group of each of the following spaces using van Kampen. For each, write the space as a pushout and apply the theorem:
(a) The figure-eight S¹ ∨ S¹ (wedge of two circles)
(b) The torus T² = S¹ × S¹
(c) The Klein bottle K (two squares with opposite edge identifications, one reversed)
(d) A genus-2 surface (hint: this is more complex; what pushout would you use?)

**Exercise 15.** The van Kampen theorem requires the two pieces A and B and their intersection C to all be connected. What goes wrong if C is disconnected? Give an example and explain how the theorem would need to be modified.

**Exercise 16.** Verify that the HoTT van Kampen theorem applies to the following non-classical situation: let A = K(Z/2Z, 1) and B = K(Z/3Z, 1) with C = {*} (a point). What is π₁(A ∪_C B) = π₁(A ∨ B)? [Answer: Z/2Z * Z/3Z, the free product, which is the modular group PSL(2,Z).]

**Exercise 17.** State and prove (sketch) the version of van Kampen for higher fundamental groups: if P is the pushout of A ←^f C →^g B and all three are (n-1)-connected, what can you say about πₙ(P)?

**Exercise 18.** (Proof-level) In the HoTT proof of van Kampen, the key step is showing that maps out of the pushout P correspond to maps out of A and B that agree on C. Make this precise: state the universal property of the pushout as a type-theoretic equation, and show that maps from π₁(P) to a group G correspond to group homomorphisms from π₁(A) and π₁(B) to G that agree on π₁(C).

## Section 4: The Freudenthal Suspension Theorem

**Exercise 19.** The Freudenthal theorem says: if A is (n-1)-connected, then the map σ : A → ΩΣA is (2n-1)-connected.
(a) Apply this to A = S¹ (which is 0-connected). What connectivity do you get for σ : S¹ → ΩS²?
(b) Apply to A = S² (which is 1-connected). What connectivity do you get for σ : S² → ΩS³?
(c) What does this say about the induced maps on homotopy groups πₖ(Sⁿ) → πₖ₊₁(Sⁿ⁺¹)?

**Exercise 20.** The stable homotopy group π₁ˢ = Z/2Z. This means π_{n+1}(Sⁿ) = Z/2Z for all n ≥ 3.
(a) The Freudenthal theorem tells us when stabilization occurs. What is the connectivity bound for Sⁿ (which is (n-1)-connected)?
(b) At what value of n does Freudenthal guarantee that πₙ₊₁(Sⁿ) = πₙ₊₂(Sⁿ⁺¹)?
(c) The values: π₃(S²) = Z, π₄(S³) = Z/2Z, π₅(S⁴) = Z/2Z. At which point does stabilization begin for k=2?

**Exercise 21.** The Blakers-Massey theorem is used to prove Freudenthal. State the Blakers-Massey theorem (for pushouts) and show how the Freudenthal theorem follows as a special case by taking the pushout to be a suspension.

**Exercise 22.** (Proof-level) The suspension-loop adjunction gives a natural bijection:
```
map(ΣA, B) ≃ map(A, ΩB)
```
Use this (and the fact that S² = ΣS¹) to construct a map S² → K(Z, 2) that represents the generator of π₂(S²) = Z. [This connects synthetic and cohomological perspectives.]

**Exercise 23.** The Freudenthal theorem implies that for n ≥ 3, the suspension map Σ : πₙ(Sⁿ) → πₙ₊₁(Sⁿ⁺¹) is an isomorphism. Since πₙ(Sⁿ) = Z for all n ≥ 1, this gives a consistent family of generators. What does Σ do to the identity map id : Sⁿ → Sⁿ?

## Section 5: The Hopf Fibration

**Exercise 24.** The Hopf family is a type family H : S² → Type defined by:
- H(base) := S¹
- ap_H(surf) := ua(rot)

where surf : base =_{S²} base is the 2-cell and rot : S¹ ≃ S¹ is the rotation equivalence.

(a) What is transport^H(surf) : S¹ → S¹?
(b) What is transport^H(surf⁻¹) : S¹ → S¹?
(c) The total space Σ(x:S²).H(x) should be S³. What condition does the Hopf family need to satisfy for this to hold?

**Exercise 25.** The long exact sequence of the Hopf fibration S¹ → S³ → S² gives:
```
... → π₃(S¹) → π₃(S³) → π₃(S²) → π₂(S¹) → π₂(S³) → π₂(S²) → π₁(S¹) → π₁(S³) → π₁(S²) → ...
```

(a) Fill in all the known values of the groups in this sequence.
(b) From the exactness, deduce π₃(S²) = Z.
(c) Also deduce π₂(S²) = Z using the sequence and the known value π₃(S³) = Z.

**Exercise 26.** The Hopf invariant of a map f : S³ → S² is the integer that measures how many times the preimage circles link. The Hopf fibration h : S³ → S² has Hopf invariant 1 (it is the generator of π₃(S²) = Z).

(a) What is the Hopf invariant of h ∘ h (the composition of h with itself... but wait, the codomain of h doesn't match the domain. Fix this: use 2h : S³ → S², the map that "goes around twice")?
(b) In HoTT, the Hopf invariant is related to the cup product structure on S². What is the cup product structure of H*(S²; Z)?

**Exercise 27.** The join construction gives S¹ * S¹ ≃ S³. Use this to:
(a) Describe geometrically what the join of two circles looks like (the join A * B has points: an element of A, an element of B, or a "segment" connecting an element of A to an element of B).
(b) Explain why S¹ * S¹ has the homotopy type of S³.
(c) The Hopf map h : S³ → S² is constructed from the join. What role does each S¹ factor play?

**Exercise 28.** The quaternionic Hopf fibration is S³ → S⁷ → S⁴. By analogy with the complex Hopf fibration:
(a) What is the long exact sequence for S³ → S⁷ → S⁴?
(b) What can you deduce about π₇(S⁴)?
(c) Compare with the complex case: what does the pattern suggest?

## Proof-Level Exercises

**Exercise 29.** (Proof-level, challenging) Prove the encode-decode theorem in full generality:

Let X be a HIT with basepoint x₀ : X. Given:
- code : X → Type with code(x₀) = G for some type G
- encode : Π(x:X). (x₀ = x) → code(x) defined by transport
- decode : Π(x:X). code(x) → (x₀ = x) defined by HIT induction
- decode ∘ encode = id: proved by path induction (J)
- encode ∘ decode = id: proved by induction on code(x)

Then (x₀ = x) ≃ code(x) for all x.

Make this precise: what exactly is the HIT induction argument for decode, and what conditions on the HIT generators does decode require? [This is the general theorem that both π₁(S¹) = Z and the other computations instantiate.]

**Exercise 30.** (Proof-level) Formalize the van Kampen theorem in HoTT as follows:
- Let P = pushout(A ←^f C →^g B).
- Let G be a group (a set with group structure).
- Define Map_*(P, BG) := {maps P → BG sending p₀ to base}.
- Show that Map_*(P, BG) ≃ Map_*(A, BG) ×_{Map_*(C,BG)} Map_*(B, BG).

[Here BG = K(G, 1) is the classifying space/Eilenberg-MacLane space. This formulation makes van Kampen a statement about mapping spaces rather than fundamental groups, and is the "right" HoTT formulation.]

**Exercise 31.** (Proof-level) Prove that the fundamental group of the suspension ΣA is trivial whenever A is connected. That is, prove π₁(ΣA) = 0 for any connected A. [This can be done with van Kampen applied to ΣA = N ∪_A S, where N and S are the "north hemisphere" and "south hemisphere" — each contractible — and A is the "equator."]

**Exercise 32.** (Proof-level, research-level) The Brunerie number β is defined as a term of type Z in Cubical Agda. The statement of Brunerie's theorem is: π₄(S³) = Z/|β|Z, where β is this specific term. Ljungström and Mörtberg (2023) showed β = 2 by normalization.

(a) Explain what "normalization" means in the context of Cubical Agda. Why is β a valid proof that π₄(S³) = Z/2Z, even though the normalization requires computation?
(b) Compare this to a classical proof that computes a group order. What is the analog in classical algebraic topology? (Hint: consider the Adams spectral sequence, which also computes homotopy groups through a mixture of algebra and computation.)
(c) Is Brunerie's proof a "proof" by the standards of classical mathematics? What would a classical mathematician object to, and how would you respond?

**Exercise 33.** (Proof-level) The following is the skeleton of a proof that π₂(S²) = Z. Fill in the steps.

*Claim:* π₂(S²) = Z.

*Proof sketch:*
1. S² is 1-connected (π₀(S²) = 1 and π₁(S²) = 0).
2. By Freudenthal (with n = 2), the map σ : S² → ΩS³ is [?]-connected.
3. This means σ induces an isomorphism on πₖ for k < [?].
4. In particular, π₂(S²) ≅ π₂(ΩS³) = π₃(S³) = Z.
5. The last equality uses the fact that π₃(S³) = Z, which comes from [?].

Fill in the blanks, and explain why step 4 uses the adjunction π₂(ΩS³) = π₃(S³).

**Exercise 34.** (Proof-level) Compute π₂(RP²). Proceed as follows:

(a) RP² has universal cover S². Classically, the covering fibration Z/2Z → S² → RP² gives a long exact sequence. Write out the sequence and compute π₂(RP²).

(b) In HoTT, RP² is a HIT with base, loop, and surf : loop · loop = refl. Use the encode-decode method (or the fibration sequence) to compute π₁(RP²) = Z/2Z and then deduce π₂(RP²).

(c) Compare the two computations. What is the HoTT analog of the covering space Z/2Z → S² → RP²?

**Exercise 35.** (Proof-level, synthesis) The following theorem connects several themes of this chapter:

*Theorem:* For any n ≥ 1, πₙ(Sⁿ) = Z.

Prove this by induction on n:
- Base case n = 1: This is π₁(S¹) = Z, proved in Section 2.
- Inductive step: Assuming πₙ(Sⁿ) = Z, prove πₙ₊₁(Sⁿ⁺¹) = Z.

For the inductive step, use the Freudenthal theorem. The suspension map σ : Sⁿ → ΩSⁿ⁺¹ induces an isomorphism on πₙ for n < 2n - 1, i.e., for n ≥ 2. So for n ≥ 2:
```
πₙ₊₁(Sⁿ⁺¹) = πₙ(ΩSⁿ⁺¹) ≅ πₙ(Sⁿ) = Z
```

Fill in: what does πₙ(ΩSⁿ⁺¹) = πₙ₊₁(Sⁿ⁺¹) follow from? Why doesn't Freudenthal apply to n = 1 (why can't you use it to get the base case from nothing)?

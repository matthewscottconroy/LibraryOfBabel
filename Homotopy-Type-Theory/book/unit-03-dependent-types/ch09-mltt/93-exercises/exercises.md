# Exercises: Chapter 9

Exercises range from direct verification of rules to proof construction to open-ended analysis. Problems marked (★) require significant work; (★★) are research-level or require creativity beyond the text.

---

## Section 1: The Four Judgments

**1.** For each of the following, identify which of the four judgment forms it is, or explain why it is not a valid judgment:
- (a) ⊢ ℕ type
- (b) n : ℕ, v : Vec ℕ n ⊢ Vec ℕ (n + 1) type
- (c) ⊢ 3 ≡ 5 : ℕ
- (d) ⊢ 3 = 5 : ℕ
- (e) ⊢ refl₃ : 3 = 3

**2.** Write out the context in which the term:
$$f : \prod_{A : \mathsf{Type}_0} \prod_{n : \mathbb{N}} \mathsf{Vec}(A, n) \to \mathsf{Vec}(A, n)$$
is well-formed. Extend this context step by step, verifying that each extension is valid.

**3.** Show that the judgment ⊢ 2 + 3 ≡ 5 : ℕ holds by giving a sequence of reduction steps from 2 + 3 to 5 using the computation rules for ℕ.

**4.** Explain why definitional equality is reflexive, symmetric, and transitive, using the structural rules. Are these properties primitive (rules) or derived (theorems)?

**5.** Show that the conversion rule — if a : A and A ≡ B, then a : B — is needed for the following concrete example: if f : ℕ → ℕ is addition by 1 and we have f 4 : ℕ, explain why f 4 : ℕ is also accepted where ℕ is replaced by any type definitionally equal to ℕ.

---

## Section 2: Type Formers

**6.** Give the full FIEC presentation (Formation, Introduction, Elimination, Computation rules) for the Boolean type Bool, which has two constructors true and false. Include the non-dependent eliminator (if-then-else) as a special case.

**7.** The type A + B (coproduct) has two injection constructors inl and inr. Using the elimination rule for +, define:
- (a) The "swap" function swap : A + B → B + A
- (b) The "map" function map_+ : (A → C) → (B → D) → A + B → C + D

**8.** Verify that the W-type W_{x:A}B(x) with A = Bool and B(true) = 𝟙, B(false) = 𝟘 gives a type equivalent to ℕ. Specifically, show how to convert back and forth between W-elements and natural numbers.

**9.** (★) The recursor for ℕ is not the same as the general eliminator. Specifically, the recursor rec_ℕ : Π(C:Type). C → (ℕ → C → C) → ℕ → C does not give the step function access to the current n. The eliminator ind_ℕ does (c_s : Π(n:ℕ).C(n) → C(succ(n))). Show that rec_ℕ can be derived from ind_ℕ, and that ind_ℕ can be derived from rec_ℕ using Σ types (encoding pairs of the current value and result).

**10.** Write out the full FIEC rules for List A. Then define:
- (a) length : List A → ℕ using the eliminator
- (b) map : (A → B) → List A → List B using the eliminator
- (c) append : List A → List A → List A using the eliminator

---

## Section 3: The Identity Type

**11.** Write out the full J rule with all premises, in the style of an inference rule. Then write the computation rule. Identify what each piece means informally.

**12.** Using J, prove that equality is symmetric: derive a term of type Π(a b : A).(a = b) → (b = a). Give the explicit term and the motive C.

**13.** Using J, prove transitivity: derive a term of type Π(a b c : A).(a = b) → (b = c) → (a = c). Give the explicit term.

**14.** Show that the term constructed in Exercise 12 (call it sym) satisfies: sym(refl_a) ≡ refl_a (by the computation rule for J).

**15.** Show that the concatenation p · q (from Exercise 13) satisfies: p · refl_b ≡ p. Does refl_a · p ≡ p hold definitionally or only propositionally? Discuss.

**16.** (★) Prove the left unit law: refl_a · p = p for p : a = b. Give the explicit motive for J and the base case. (Note: this is propositional equality between paths, not definitional.)

**17.** (★) Prove that (p⁻¹)⁻¹ = p for p : a = b. Give the proof by J.

**18.** (★★) Define the Eckmann-Hilton argument: given p, q : refl_a = refl_a (loops at refl_a in the identity type), show that p · q = q · p. This requires working in the identity type of an identity type. Identify the key step.

---

## Section 4: Path Induction

**19.** State the unbased J' rule (with premises and conclusion). Show how to instantiate J' to recover based J by fixing the basepoint to a.

**20.** (★) Derive J from J'. The key step is defining the right motive for J'. Give the explicit motive D and the base case, and verify that applying J' gives the desired conclusion.

**21.** State the contractibility of the based path space as a type in MLTT:
$$\prod_{p : \sum_{b:A}(a = b)} (a, \mathsf{refl}_a) = p$$
Give a proof sketch using J (you may use transport, which we define formally in Section 5).

**22.** Axiom K says: Π(a:A).Π(p:a=a). p = refl_a. Show that K implies UIP. (UIP: Π(a b:A).Π(p q:a=b). p = q.)

**23.** (★) Axiom K is not derivable from J. Give the outline of the groupoid model argument: what is the model, what do types/terms/identity proofs correspond to, and where does K fail in this model?

**24.** In Agda (with `--without-K`), the following fails to typecheck:
```agda
K : {A : Set} {a : A} (p : a ≡ a) → p ≡ refl
K refl = refl
```
Explain *why* this fails by identifying which aspect of the pattern-matching overlaps with K.

---

## Section 5: Transport and ap

**25.** Using J, construct transport^B : (a = b) → B(a) → B(b) for any type family B : A → Type. Give the motive and base case explicitly.

**26.** Prove that transport^B(refl_a, u) ≡ u by the computation rule for J.

**27.** Prove that transport^B(p⁻¹) and transport^B(p) are inverse to each other:
- For any u : B(a) and p : a = b: transport^B(p⁻¹, transport^B(p, u)) = u

**28.** Using J, construct ap_f(p) : f(a) = f(b) for f : A → B and p : a = b. Give the motive and base case.

**29.** Show that ap respects concatenation: ap_f(p · q) = ap_f(p) · ap_f(q). Prove this by J on q.

**30.** (★) Define apd_f(p) : transport^B(p, f(a)) = f(b) for f : Π(x:A).B(x) and p : a = b. Give the J-derivation. Verify the computation rule holds.

---

## Section 6: Intensional vs. Extensional

**31.** Identify which of the following is a definitional equality (≡) and which is a propositional equality (=) that requires proof:
- (a) 0 + n = n
- (b) n + 0 = n
- (c) (λx:ℕ. x + 1) 4 = 5
- (d) map id l = l for map : (A → A) → List A → List A and l : List A
- (e) n + m = m + n

**32.** The reflection rule says: from p : a = b, derive a ≡ b. Show that, given the reflection rule, the type-checking problem for the following is undecidable:

"Is the judgment ⊢ t : T valid, where T involves the type (f n =_ℕ 0) for f : ℕ → ℕ a computable function?"

(Outline the reduction from the halting problem.)

**33.** (★) Function extensionality (funext) is not provable in intensional MLTT without additional axioms. Specifically, state funext as a type:
$$\mathsf{funext} : \prod_{f g : \prod_{x:A} B(x)} \left(\prod_{x:A} f(x) = g(x)\right) \to f = g$$
Show that funext is *consistent* with intensional MLTT by giving an informal argument (no formal model needed — just sketch why funext cannot derive ⊥ in the presence of J and the standard type formers).

**34.** (★) State propositional extensionality for propositions:
$$\mathsf{propext} : \prod_{P Q : \mathsf{hProp}} (P \leftrightarrow Q) \to P = Q$$
where hProp = Σ(P:Type₀).isProp(P) is the type of h-propositions. Show that propext follows from the univalence axiom (you may use the statement of univalence without proof).

**35.** (★★) The Streicher-Barthe-Capretta-Pons result: in extensional MLTT with the reflection rule, show that the following principle holds (proof sketch is acceptable):

"If T is any type former such that the type of T-structured types forms a contractible type, then T-types are all definitionally equal."

Use this to argue that in extensional MLTT, any two types with the same elements (in the propositional sense) are definitionally equal — and explain why this collapses the universe structure in a way that is incompatible with HoTT.

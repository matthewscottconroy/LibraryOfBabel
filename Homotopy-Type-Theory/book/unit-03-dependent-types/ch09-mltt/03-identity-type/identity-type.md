# The Identity Type

## Equality as a Type

In every formal system, you need to say when two things are the same. In predicate logic, equality is a relation: = is a predicate symbol with the axioms of reflexivity, symmetry, transitivity, and substitutivity. In set theory, equality is defined via extensionality: two sets are equal iff they have the same elements. In MLTT, equality is a type.

This is the move that makes everything else possible.

The identity type a =_A b is the type whose elements are proofs that a equals b, where a, b : A. This is not just a relation on A — it is a *type*, subject to all the machinery of type theory. Elements of a =_A b are terms. You can form Π types over them, Σ types over them, functions that take them as arguments. Equality is not a second-class citizen — it is first-class.

And identity types can have multiple elements. This is the fact that opens the door to HoTT.

## Formation

$$\frac{\Gamma \vdash A\ \mathsf{type} \quad \Gamma \vdash a : A \quad \Gamma \vdash b : A}{\Gamma \vdash a =_A b\ \mathsf{type}} \qquad (\mathsf{Id}\text{-Form})$$

For any type A and any two terms a, b : A, the identity type a =_A b is a well-formed type. It may or may not be inhabited — that depends on whether a and b are actually equal (in whatever sense is appropriate).

**Alternative notation:** Id_A(a, b), a ≡ b (some authors), a ~ b (in some HoTT presentations), Path A a b (cubical type theory).

**Observing that this is a type family:** For fixed a : A, the expression b ↦ (a =_A b) is a type family over A. It maps each b : A to the type of paths from a to b. This family is called the *based path space* at a, and its total space Σ(b:A).(a =_A b) will be contractible — a key fact we derive from the elimination rule.

## Introduction: Reflexivity

The only direct constructor for the identity type is reflexivity:

$$\frac{\Gamma \vdash a : A}{\Gamma \vdash \mathsf{refl}_a : a =_A a} \qquad (\mathsf{Id}\text{-Intro})$$

Every element is equal to itself. There is exactly one constructor, and it only produces self-equalities.

This might seem to say: the only elements of a =_A b are when a ≡ b (definitionally equal), in which case refl_a : a =_A a is an element and a =_A b is inhabited by coercion. But this is not true. Through the elimination rule (J), more equalities can be derived — including equalities between terms that are not definitionally equal.

The crucial point: refl is the only *primitive* constructor. All other equality proofs are derived from refl using J.

## Elimination: The J Rule

The J rule is the elimination principle for the identity type. It encodes the principle: to prove a property of all equality proofs, it suffices to prove it when the proof is reflexivity.

**Based J (the HoTT Book version):**

$$\frac{\Gamma \vdash a : A \quad \Gamma \vdash C : \prod_{b:A} (a =_A b) \to \mathsf{Type} \quad \Gamma \vdash d : C(a, \mathsf{refl}_a) \quad \Gamma \vdash b : A \quad \Gamma \vdash p : a =_A b}{\Gamma \vdash J(a, C, d, b, p) : C(b, p)} \qquad (\mathsf{Id}\text{-Elim})$$

Unpacking the rule:
- a : A is the *basepoint* — the fixed starting point
- C : Π(b:A).(a =_A b) → Type is the *motive* — what you want to prove about all paths from a
- d : C(a, refl_a) is the *base case* — the proof when the path is trivial (reflexivity at a)
- b : A and p : a =_A b are the *target point and path* — the path you apply J to
- J(a, C, d, b, p) : C(b, p) — the conclusion: the property holds for any path from a

**The key insight:** You only need to handle the trivial path refl_a. Any other path from a to any b can, for the purpose of proving the property C, be "contracted" to the trivial path. This is path induction.

## The Computation Rule

$$J(a, C, d, a, \mathsf{refl}_a) \equiv d : C(a, \mathsf{refl}_a) \qquad (\mathsf{Id}\text{-Comp})$$

When you apply J to the reflexivity path, it reduces to the base case d. This is the only computation rule — there is only one constructor (refl), so there is only one case to compute on.

This computation rule holds definitionally: the type checker verifies it without a proof from you.

## Deriving Basic Properties of Equality

From refl and J, we can derive all the expected properties of equality.

### Symmetry (Path Inversion)

**Goal:** From p : a =_A b, construct p⁻¹ : b =_A a.

**Construction:** Apply J with:
- Motive: C(b, p) = (b =_A a) — for each b and path p : a = b, the type of return paths
- Base case: d = refl_a : C(a, refl_a) = (a =_A a)

$$\mathsf{inv}(p) = J(a,\, \lambda b\, p.\, b =_A a,\, \mathsf{refl}_a,\, b,\, p) : b =_A a$$

**Computation:** inv(refl_a) ≡ refl_a. (The inverse of the trivial path is itself.)

### Transitivity (Path Concatenation)

**Goal:** From p : a =_A b and q : b =_A c, construct p · q : a =_A c.

**Construction:** Apply J to q with:
- Motive: C(c, q) = (a =_A c) — "we can reach c from a"
- Base case: d = p : a =_A b = a =_A b = C(b, refl_b)

$$p \cdot q = J(b,\, \lambda c\, q.\, a =_A c,\, p,\, c,\, q) : a =_A c$$

**Computation:** p · refl_b ≡ p. (Concatenating with the trivial path does nothing.)

### The Groupoid Laws

With inversion and concatenation defined via J, we can prove the groupoid laws — but only *propositionally*, as elements of higher identity types.

- **Left unit:** refl_a · p = p, proved by J on p
- **Right unit:** p · refl_b = p, by computation (direct from the definition)
- **Associativity:** (p · q) · r = p · (q · r), proved by J on r
- **Left inverse:** inv(p) · p = refl_b, proved by J on p
- **Right inverse:** p · inv(p) = refl_a, proved by J on p

Each law is an element of an *identity type of an identity type* — a path between paths. This is where the higher structure begins.

**The groupoid interpretation:** Every type A, with elements as objects and identity proofs as morphisms, forms a groupoid. This is not just an analogy — it is a theorem, derivable from J.

## UIP is Not Derivable

**Uniqueness of Identity Proofs (UIP):** The statement Π(A:Type).Π(a b:A).Π(p q:a=_A b). p = q.

This says any two proofs of the same equality are themselves equal. In set-theoretic mathematics, this is trivially true — equality is a relation, and either it holds (in one way) or it does not.

UIP is not derivable from the J rule. The proof: Hofmann and Streicher's 1994 groupoid model. Take any groupoid G. Interpret types as groupoids, terms as objects, and identity proofs a =_A b as morphisms a → b. In a non-trivial groupoid (say, the integers ℤ under addition, viewed as a one-object groupoid), there are multiple distinct morphisms from the single object to itself. Under this interpretation, all the MLTT rules hold — but UIP fails, because there are multiple distinct identity proofs.

Therefore, UIP is consistent with MLTT but not provable from the basic rules. You can add UIP as an axiom (Axiom K, in Streicher's formulation) and remain consistent. But if you add it, you destroy the higher-dimensional structure — all path spaces become sets (discrete groupoids), and HoTT becomes impossible.

## Why UIP Must Fail for HoTT

The entire HoTT program depends on UIP being false.

If UIP held, the identity type a =_A b would always have at most one element (up to equality). Types would behave like sets: elements, with equality being a mere relation. No topology, no higher structure.

When UIP fails, the identity type can have multiple elements. Those elements (paths) can themselves have identity types (paths between paths, i.e., homotopies). Those can have identity types (homotopies of homotopies), and so on. The resulting structure is that of an *infinity-groupoid* — the type-theoretic counterpart of a topological space.

The circle S¹, defined as a higher inductive type, has:
- One point: base : S¹
- One non-trivial loop: loop : base =_{S¹} base
- The identity type base =_{S¹} base has elements loop^n for each integer n (winding number)
- So π₁(S¹, base) = ℤ, the integers — a non-trivial homotopy group

This computation is impossible if UIP holds (it would force loop = refl_base). It is possible — and a theorem provable in HoTT — precisely because UIP fails.

## The Homotopy Interpretation: A Preview

The J rule says: to prove a property C(b, p) for all b and all paths p : a = b, it suffices to prove C(a, refl_a).

Geometrically: the based path space Σ(b:A).(a =_A b) is *contractible*. It has a center of contraction (a, refl_a): every other element (b, p) is connected to (a, refl_a) by a path (given by J itself). A contractible space is, homotopically, a single point.

This is the first theorem of synthetic homotopy theory, derivable from J alone: the total based path space is contractible. Every other homotopy-theoretic fact in HoTT — the fundamental group of the circle, the Hopf fibration, the Freudenthal suspension theorem — ultimately traces back to this one fact about J.

The identity type is a path space. J is the contractibility of the based path space. HoTT is the mathematics that follows from taking this seriously.

# Type Families: Types That Depend on Values

## The Central Idea

Everything in dependent type theory rests on one concept: a type can be indexed by a value. Not by another type — we already had that with type constructors in System Fω — but by an actual term, a thing that computes, a thing with a specific value at runtime.

Call this a *type family*. Formally: a type family over A is a function B : A → Type. Given any element a : A, the application B(a) produces a type. Different values of a, in general, produce different types.

This sounds innocent. It is not. The moment you allow B : A → Type — the moment you allow a function from terms to types — you gain the ability to encode arbitrary mathematical predicates in your type system. The difference between a list and a list of a specific length. The difference between a function and a bijection. The difference between a number and a prime. All of these become distinctions that the type system can see and enforce.

## The Canonical Example: Vec

The type family Vec : ℕ → Type maps each natural number n to the type of n-element lists.

We can define it inductively:
- Vec 0 = 𝟙 (the unit type; there is exactly one 0-element list)
- Vec (n+1) = A × Vec n (a head element paired with an n-element tail)

With this definition, Vec 0 and Vec 3 are genuinely different types. An element of Vec 0 is the unit value ⋆. An element of Vec 3 is a triple (a₁, (a₂, (a₃, ⋆))). Asking whether an element of Vec 5 has type Vec 3 is like asking whether an integer has type Bool — the type checker simply says no, before any computation occurs.

The type-safe `append` function has type:

$$\mathsf{append} : \mathsf{Vec}\, A\, m \to \mathsf{Vec}\, A\, n \to \mathsf{Vec}\, A\, (m + n)$$

The output length is computed from the input lengths at the type level. If you try to pass a 5-element vector where a 3-element vector is expected, you get a type error. The compiler catches it. You do not.

## The Second Canonical Example: Fin

The type family Fin : ℕ → Type maps each natural number n to a type with exactly n elements.

$$\mathsf{Fin}\, 0 = \mathbf{0} \quad (\text{empty type, no elements})$$
$$\mathsf{Fin}\, (n+1) = \mathsf{Fin}\, n + \mathbf{1} \quad (\text{one more element than } \mathsf{Fin}\, n)$$

So Fin 3 has exactly three elements: inl(inl(inr⋆)), inl(inr⋆), inr⋆ — which we can think of as 0, 1, 2.

The application: safe array indexing. If your array has type Vec A n, then a valid index has type Fin n. An index out of bounds is a type error, not a runtime exception. The Java programmer who wrote `array[array.length]` and got an ArrayIndexOutOfBoundsException at 3am on a Saturday would have appreciated this.

## The Formal Judgment

In dependent type theory, we express "B is a type family over A" via the typing judgment:

$$x : A \vdash B(x) : \mathsf{Type}$$

This says: in a context that contains a variable x of type A, the expression B(x) is a well-formed type. The variable x is free — it does not have a specific value. When we substitute a particular term a : A for x, we get the specific fiber type B(a).

More generally, in a context Γ:

$$\Gamma,\, x : A \vdash B(x) : \mathsf{Type}$$

This is the context extension: we take whatever is known in Γ, add the assumption x : A, and under those assumptions B(x) is a type.

The substitution operation is then: if a : A in context Γ, we can form B(a) : Type in context Γ. This is written B[a/x] — substitute a for x throughout B.

## Indexed vs. Parameterized Families

There is a distinction that matters in practice, even though it does not always matter formally: the difference between a *parameterized* type and an *indexed* type.

A parameterized type like List : Type → Type takes a type parameter (the element type) and returns a type. The type parameter is a type, not a value. This is System Fω territory — it does not require dependent types.

An indexed type family like Vec : ℕ → Type takes a *value* (a natural number) and returns a type. This is genuinely dependent.

In practice, Vec A n depends on both: a type A (parameter) and a natural number n (index). We often write the full family as Vec : Type → ℕ → Type. When A is fixed, Vec A : ℕ → Type is a type family over ℕ.

The distinction matters because indices can vary in interesting ways. A *parameterized* type has the same structure for every parameter. An *indexed* type can have completely different structure for different indices — Fin 0 is empty, Fin 3 has three elements, and these are fundamentally different types, not just different "instances" of the same structure.

## Type Families as Predicates

The logical reading of a type family is: B : A → Type is a *predicate* on A. The type B(a) is inhabited if the predicate holds at a, and empty if it does not.

| Logic | Type theory |
|---|---|
| Predicate P on A | Type family B : A → Type |
| P(a) holds | B(a) is inhabited (has an element) |
| P(a) is false | B(a) = 𝟎 (empty type) |

This correspondence is not metaphorical. It is the formal content of the Curry-Howard correspondence, extended to first-order logic. When we form Π and Σ types in the next two sections, we will see that universal quantification is a Π type and existential quantification is a Σ type. The predicate/type-family identification makes this precise.

**Example.** Define IsEven : ℕ → Type by:
- IsEven 0 = 𝟙
- IsEven 1 = 𝟎
- IsEven (n+2) = IsEven n

Then IsEven n is inhabited (nonempty) exactly when n is even. The element of IsEven 4 is a witness that 4 is even. The type IsEven 3 is empty — there is no witness, because 3 is odd.

A function f : Π(n:ℕ). IsEven n → SomeProperty n would need, as part of its definition, a proof that n is even before it can assume SomeProperty n. The predicate is enforced by the type.

## Type Families and Equality: The First Glimpse

Here is something that does not exist in non-dependent type theory. If B : A → Type is a type family and p : a = b is a proof that a equals b in A, then we can *transport* an element of B(a) to an element of B(b):

$$\mathsf{transport}^B : (a = b) \to B(a) \to B(b)$$

This is the type-theoretic substitution of equals for equals. If a and b are identified by the path p, then anything in the fiber over a can be moved to the fiber over b along p.

We will construct transport formally in Chapter 9, once we have the identity type and its elimination rule. But the point to register now is that type families interact with equality in a non-trivial way. A path in A induces a function between fibers. This is the beginning of the geometric picture: type families are fibrations, and transport is parallel transport along a path.

## Fibrations: The Geometric Dictionary

The homotopy type theory perspective interprets type families geometrically. If A is a type (interpreted as a space), then B : A → Type is a fibration over A: for each point a in the base, there is a fiber B(a) sitting above it.

The total space of the fibration is Σ(a:A).B(a) — the type of pairs (a, b) where a : A and b : B(a). There is a projection map π₁ : Σ(a:A).B(a) → A sending (a, b) to a.

A section of the fibration is an element of Π(a:A).B(a) — for each base point a, a choice of element f(a) in the fiber B(a). Sections are exactly Π types.

| Differential geometry | Type theory |
|---|---|
| Fibration E → B over base B | Type family B : A → Type |
| Fiber over point a | B(a) |
| Total space E | Σ(a:A).B(a) |
| Section s : B → E | f : Π(a:A).B(a) |
| Parallel transport along γ | transport^B(p) for p : a = b |

This dictionary is not merely metaphor. In HoTT, where the identity type genuinely behaves like a path space, type families genuinely behave like fibrations in the homotopy-theoretic sense. The geometric intuition is mathematically correct, and theorems from topology translate into theorems of type theory.

## Why Not Just Use Subset Types?

One might object: why do we need type families at all? Can we not just use subsets? Write `{n : ℕ | IsEven n}` for the even numbers and be done with it?

The trouble is that set-theoretic subsets do not compose well with functions. If f : A → B and S ⊆ A, the image f(S) is not automatically a subtype of B in any useful sense. Membership in a subset is a proposition (true or false), not a structure — you cannot have multiple distinct proofs that an element belongs to a subset.

Type families solve both problems. The family B : A → Type does not just say "a has property B or it doesn't"; it specifies a *type* of witnesses. There can be multiple distinct witnesses. Witnesses can carry computational content. And the whole machinery of Π and Σ types, transport, and path induction applies to type families in ways that have no counterpart in set-theoretic subset theory.

This is the deeper reason dependent types matter. They do not just add expressiveness at the surface — "now you can track lengths in types." They change the fundamental nature of what a proof is and what equality means. And that change, followed through to its logical conclusion, leads to HoTT.

## Looking Ahead

In the next section, we form Π types: functions into type families. In Section 3, we form Σ types: pairs where the second component lives in a fiber. These two type formers, combined with the universe (Section 4) and inductive types (Section 5), give the full language of MLTT. Everything else — including the identity type — is built within this language.

The type family Vec A : ℕ → Type that started this section will reappear in Section 5, defined as a proper inductive type. At that point we will have all the machinery to write `append`, `lookup`, `head` (with a proof that the vector is nonempty), and other operations — all with types that make incorrect uses compile errors rather than runtime failures.

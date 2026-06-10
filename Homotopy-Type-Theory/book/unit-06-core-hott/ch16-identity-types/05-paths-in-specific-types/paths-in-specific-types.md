# Paths in Specific Types

## The General Question

We now know what paths are in the abstract: elements of identity types. We know how to concatenate them, invert them, transport along them, and apply functions to them.

But for specific types — the types we actually work with in mathematics — we want to know what paths *look like*. If I have two pairs (a₁, b₁) and (a₂, b₂) in A × B, what does it mean for them to be equal? If I have two functions f, g : A → B, what does it mean for them to be equal? If I have two types A, B in the universe, what does it mean for them to be equal?

The answers are both clean and surprising. Equality in a compound type decomposes into equality in the components — but the decomposition requires transport in the dependent case, and the universe requires an entirely new axiom.

## Paths in Product Types

Two pairs (a₁, b₁) and (a₂, b₂) in A × B are equal if and only if their components are separately equal.

**Theorem.** There is an equivalence:

```
((a₁, b₁) =_{A×B} (a₂, b₂))  ≃  (a₁ =_A a₂) × (b₁ =_B b₂)
```

**From path to component paths.** Given r : (a₁, b₁) = (a₂, b₂), apply ap to the projections:

```
ap_{π₁}(r) : a₁ = a₂
ap_{π₂}(r) : b₁ = b₂
```

**From component paths to path.** Given p : a₁ = a₂ and q : b₁ = b₂, apply J to p and then to q. Reduces to the case a₁ = a₂ = a, b₁ = b₂ = b, where the required path is refl_{(a,b)}.

**These are mutual inverses** — proved by J on the relevant paths, reducing to reflexivity. So the two constructions form an equivalence.

**Computation:** pair-eq(refl_a, refl_b) ≡ refl_{(a,b)}.

Topologically, this is the statement that a path in X × Y is a pair of paths: one in X and one in Y, traversed simultaneously. The product space has no interaction between the components — a path must move coherently through both.

## Paths in Sigma Types

The dependent case is more interesting. Two elements (a₁, b₁) and (a₂, b₂) of Σ(x : A). B(x) live in the same type, but their second components live in *different fibers*: b₁ : B(a₁) and b₂ : B(a₂). You cannot directly compare them.

**Theorem.** There is an equivalence:

```
((a₁, b₁) =_{Σ(x:A).B(x)} (a₂, b₂))  ≃  Σ(p : a₁ = a₂). transport^B(p, b₁) = b₂
```

A path between dependent pairs consists of:
1. A path p : a₁ = a₂ in the first component.
2. A path transport^B(p, b₁) = b₂ in the second component — *after transporting b₁ along p to B(a₂)*, then comparing with b₂.

**Why transport appears.** The second components live in different fibers: b₁ : B(a₁) and b₂ : B(a₂). To compare them, we must move b₁ to B(a₂) using the path p. Transport is the canonical way to do this.

**Interpretation.** A path in the total space Σ(x:A).B(x) of a fibration consists of:
- A path in the base A (the first component).
- A path in the fiber above the endpoint that "covers" the base path (via transport).

This is the homotopy-theoretic notion of a path in a total space covering a path in the base. The fibration condition — that paths can always be lifted — is the statement that transport always exists, which we proved in the previous section.

**Special case: propositions.** If B(x) is a proposition for all x (at most one element), then the transport condition `transport^B(p, b₁) = b₂` is automatic — there is at most one element in B(a₂), so once we know b₂ exists, the transport condition is trivially satisfied. Therefore, paths in Σ(x:A). isProp(B(x)) are just paths in A. Subtypes have the same path structure as the ambient type.

## Paths in Natural Numbers

The natural numbers N are a set (h-level 0, Chapter 17). Every path between natural numbers is unique. The path type n =_N m is either contractible (if n = m) or empty (if n ≠ m).

More precisely, the path type for natural numbers is characterized by:

```
0 = 0   ↔   True
S(n) = S(m)  ↔  n = m
0 = S(n)  ↔  False
S(n) = 0  ↔  False
```

This is proved by Hedberg's theorem (Chapter 17): N has decidable equality, and decidable equality implies the type is a set.

## Paths in Bool

Similarly, Bool = {true, false} has discrete equality:

```
true = true  ↔  True
false = false  ↔  True
true = false  ↔  False
false = true  ↔  False
```

Bool is a set. The only interesting fact about `Bool = Bool` as types in the universe (via Univalence) is that there are exactly two self-equivalences of Bool: the identity and the swap.

## Paths in Function Types: Function Extensionality

What does it mean for two functions f, g : A → B to be equal?

Intuitively, f and g should be equal if they agree on all inputs: for all x : A, f(x) = g(x). This is the principle of *function extensionality*.

The map from paths to pointwise equality is easy:

```
happly : (f = g) → Π(x : A). f(x) = g(x)
happly(p, x) = ap_{ev_x}(p)
```

where ev_x : (A → B) → B is the evaluation function h ↦ h(x).

The other direction — from pointwise equality to a path — is not provable from J alone.

**Axiom (Function Extensionality).** The map happly is an equivalence:

```
(f =_{A→B} g)  ≃  Π(x : A). f(x) =_B g(x)
```

In other words: funext : (f ~ g) → (f = g) exists, inverse to happly.

**Why not provable from J alone.** In a model where functions carry computational content (different algorithms that compute the same values), two functions can be extensionally equal without being definitionally the same. J cannot create new identity proofs between things that might be computationally distinct. So funext requires either an axiom or an appeal to a stronger principle.

**funext follows from Univalence.** In Chapter 18, we will prove that the Univalence Axiom implies function extensionality. The key idea: the interval type I (a HIT with two points and a path between them) gives funext, and the interval type can be constructed from Univalence and the type of propositions.

**Computation rules for funext.** Assuming funext:
- `happly(funext(H)) = H` — funext followed by happly recovers the homotopy.
- `funext(happly(p)) = p` — happly followed by funext recovers the path.
- `funext(λx. refl_{f(x)}) = refl_f` — the trivial homotopy gives reflexivity.

## Paths in the Universe: Preview of Univalence

What does it mean for two types A and B in the universe Type to be equal?

A path p : A = B in the universe is an identification of A with B as types. Transport along p gives a function transport(p) : A → B, which should be an equivalence.

The Univalence Axiom (Chapter 18) makes this precise:

```
(A =_{Type} B)  ≃  (A ≃ B)
```

Paths in the universe are equivalences. Two types are equal iff they are equivalent — iff there is a bijection between them that respects all the type-theoretic structure.

This is the deepest path computation of all. It is the formal expression of the principle that isomorphic structures are interchangeable — not just informally, but in the foundations. We preview it here and develop it fully in Chapter 18.

## Transport and ap for Each Type Former

The general pattern of this section:

| Type former | Path characterization |
|---|---|
| A × B | (a₁,b₁) = (a₂,b₂)  ≃  (a₁=a₂) × (b₁=b₂) |
| Σ(x:A).B(x) | (a₁,b₁) = (a₂,b₂)  ≃  Σ(p:a₁=a₂). tr(p,b₁)=b₂ |
| A → B | f = g  ≃  Π(x:A). f(x)=g(x)  (with funext) |
| Π(x:A).B(x) | f = g  ≃  Π(x:A). f(x)=g(x)  (with funext) |
| A + B | inl(a) = inl(a')  ≃  a=a'; inr(b)=inr(b')  ≃  b=b'; inl=inr  ≃  ∅ |
| Type | A = B  ≃  A ≃ B  (Univalence) |
| N | n = m  iff n and m are the same numeral |
| Bool | true=true  and  false=false  are unit; true=false is empty |

Each characterization is proved using J (plus funext or Univalence where needed). Each says: paths in a compound type decompose into paths in the components, with transport connecting the pieces in the dependent case.

## Why This Matters for Mathematics

These path characterizations are not just mathematical curiosities. They are the foundation for doing mathematics in HoTT.

When you work with a type of mathematical structures — say, Σ(G : Type). isGroup(G) — the path characterization tells you what equality of groups means in HoTT: it is a path in the first component (an equivalence of the underlying types, by Univalence) plus a transported equality in the group structure. This recovers the classical notion of group isomorphism as the correct notion of equality for groups.

When you prove that two functions are equal, funext lets you work pointwise. This is how mathematicians always worked — to show that two group homomorphisms are equal, show they agree on all generators — and funext makes it formally correct.

When you show that two propositions are equivalent, propositional extensionality (which follows from funext applied to the universe of propositions) gives you their equality. Equivalent propositions are the same proposition.

The path structure of specific types is the bridge between abstract type theory and concrete mathematical practice.

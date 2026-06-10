# 4.1 Universes: The Type of Types

## The Problem of Quantifying Over All Types

Dependent types allow types to depend on values. So we should be able to quantify over types — form Π types and Σ types where the domain or codomain is "the type of all types."

But what's the type of a type? In the notation we've been using, we write things like $A : \mathsf{Type}$. But if $\mathsf{Type}$ is itself a type, then $\mathsf{Type} : \mathsf{Type}$. This is the infamous *type-in-type* axiom, and it leads immediately to paradox.

## The Type-in-Type Paradox

If $\mathsf{Type} : \mathsf{Type}$, we can construct Girard's paradox: a closed term of type $\mathbf{0}$ (the empty type). The construction is complex (it encodes a variant of Russell's paradox or the Burali-Forti paradox for ordinals), but the key idea is:

Since $\mathsf{Type} : \mathsf{Type}$, we can form the type
$$U = \sum_{X : \mathsf{Type}} (X \to \mathsf{Type})$$
of "setoids" — types with a family over them. Inside $U$, we can define an ordering-like structure and derive a contradiction by the Burali-Forti argument: the type of all such "ordinals" is itself an ordinal greater than all ordinals.

Concretely: if type-in-type holds, then every type is inhabited — including $\mathbf{0}$ — and the system is inconsistent. Martin-Löf's original type theory from 1971 had type-in-type, and Girard showed it was inconsistent in 1972.

## The Universe Hierarchy

The fix: introduce a *hierarchy* of universes.

$$\mathsf{Type}_0 : \mathsf{Type}_1 : \mathsf{Type}_2 : \mathsf{Type}_3 : \cdots$$

Each universe $\mathsf{Type}_i$ is a type — but it lives in the *next* universe $\mathsf{Type}_{i+1}$, not in itself. Small types live in $\mathsf{Type}_0$; $\mathsf{Type}_0$ itself lives in $\mathsf{Type}_1$; and so on.

**Universe membership rules:**
- If $A : \mathsf{Type}_i$, then $A : \mathsf{Type}_{i+1}$ (universes are *cumulative*)
- $\mathsf{Type}_i : \mathsf{Type}_{i+1}$
- $\mathbb{N} : \mathsf{Type}_0$ (ordinary types live at level 0)
- If $A, B : \mathsf{Type}_i$, then $A \to B : \mathsf{Type}_i$ (arrow types stay at the same level)
- If $A : \mathsf{Type}_i$ and $B : A \to \mathsf{Type}_i$, then $\prod_{x:A} B(x) : \mathsf{Type}_i$ (Π types stay at the same level, given their components are at level $i$)

The hierarchy is *cumulative* in that smaller universes are subsumed by larger ones: anything in $\mathsf{Type}_0$ is also in $\mathsf{Type}_1$, and so on. This is convenient — you don't have to worry about universe levels when you're not near a boundary.

## Universes Are Types of Types

The key property: $\mathsf{Type}_i$ is a type whose *elements* are (small) types at level $< i+1$. So:

- Elements of $\mathsf{Type}_0$: $\mathbb{N}$, $\mathbb{B}$, $A \to B$ (for $A, B : \mathsf{Type}_0$), $\prod_{x:A} B(x)$ (for $A : \mathsf{Type}_0$, $B : A \to \mathsf{Type}_0$), etc.
- Elements of $\mathsf{Type}_1$: everything in $\mathsf{Type}_0$, plus $\mathsf{Type}_0$ itself, plus $A \to \mathsf{Type}_0$ (type families that range over all small types), etc.

This stratification prevents self-reference: $\mathsf{Type}_0 \notin \mathsf{Type}_0$ (the universe of small types is not itself a small type).

**Why does this prevent paradox?** The Burali-Forti/Russell argument requires a "set of all sets" — a set that contains itself as a member. With the hierarchy, no universe is an element of itself. The paradoxical constructions need $\mathsf{Type} : \mathsf{Type}$, which we've ruled out.

## Russell vs. Tarski Style

There are two ways to formalize universes, corresponding to two different styles for universe membership.

**Russell style (what we've been using):** Types at level $i$ *are* elements of $\mathsf{Type}_i$. When you write $A : \mathsf{Type}_i$, you're saying $A$ itself is an element of the universe. Type-level and universe-membership are the same.

$$A : \mathsf{Type}_0 \quad \text{means } A \text{ is a small type}$$
$$\mathbb{N} : \mathsf{Type}_0 \quad \text{— the type of naturals is a small type}$$

**Tarski style:** The universe $\mathsf{Type}_i$ is a type with *codes* — special elements $\hat{A}$ that represent types. A separate operation $\mathsf{El}(\hat{A})$ decodes a code to the actual type.

$$\hat{\mathbb{N}} : \mathsf{Type}_0 \quad \text{— this is the code for ℕ}$$
$$\mathsf{El}(\hat{\mathbb{N}}) = \mathbb{N} \quad \text{— decoding gives the actual type}$$

Tarski style is more explicit (you always know when you're working with codes vs. types) but more verbose. Russell style is more ergonomic (you write $\mathbb{N} : \mathsf{Type}_0$ directly) but requires some care when universes overlap.

Most modern proof assistants (Lean 4, Agda) use Russell style with implicit universe level inference.

## Universe Polymorphism

A constant annoyance in early universe hierarchies: to define a function that works at all universe levels, you need to write it separately for each level. For example, the polymorphic identity:

$$\mathsf{id}_0 : \prod_{A : \mathsf{Type}_0} A \to A$$
$$\mathsf{id}_1 : \prod_{A : \mathsf{Type}_1} A \to A$$
$$\vdots$$

These are the same function but at different levels. Universe polymorphism lets you write it once:

$$\mathsf{id} : \prod_{\ell : \mathsf{Level}} \prod_{A : \mathsf{Type}_\ell} A \to A$$

where $\mathsf{Level}$ is a type of universe levels and $\mathsf{Type}_\ell$ is the universe at level $\ell$.

In Lean 4:
```lean
def id.{u} : {α : Type u} → α → α := fun x => x
-- The .{u} quantifies over universe levels
-- u is a universe variable
```

In Agda:
```agda
id : ∀ {l : Level} {A : Set l} → A → A
id x = x
```

Universe polymorphism is a significant practical feature: it lets you write generic theorems and constructions that work at any universe level, without code duplication.

## Universes and the Univalence Axiom

Universes are particularly important in HoTT because of the Univalence Axiom:

$$\mathsf{ua} : (A \simeq B) \simeq (A = B) \quad \text{for } A, B : \mathsf{Type}_i$$

This says: an equivalence between types (an isomorphism) is the same as an equality of types *in the universe* $\mathsf{Type}_i$. The equality $A = B$ here is the identity type of the universe — a path between types in the space $\mathsf{Type}_i$.

This axiom makes types into homotopy types: the universe $\mathsf{Type}$ is a space, types are points, equivalences between types are paths, and more complex homotopy structure (higher paths) corresponds to higher coherences between equivalences.

Univalence is an axiom about universes, not about individual types. It's precisely this that makes HoTT the theory it is.

## Cumulative Universes and Universe Lifting

With cumulative universes, we have:

$$\mathsf{Type}_0 \subseteq \mathsf{Type}_1 \subseteq \mathsf{Type}_2 \subseteq \cdots$$

in the sense that every type at level $i$ is also a type at level $j > i$. In Russell style, this is typically expressed by a lifting operation:

$$\mathsf{Lift} : \mathsf{Type}_i \to \mathsf{Type}_{i+1}$$

If $A : \mathsf{Type}_0$, then $\mathsf{Lift}(A) : \mathsf{Type}_1$ — the "same" type but viewed as living in the larger universe. Cumulativity means this lifting is transparent: $A$ and $\mathsf{Lift}(A)$ behave the same way.

In practice, proof assistants handle this automatically via universe level inference — you rarely write explicit level annotations unless you're doing something that requires crossing universe boundaries.

## When Universes Are Not Enough

The standard universe hierarchy $\mathsf{Type}_0 : \mathsf{Type}_1 : \cdots$ gives you $\omega$ universes. For most mathematics, this is plenty. But some constructions require *large* universes — universes indexed by ordinals beyond $\omega$.

For example, Mahlo cardinals and inaccessible cardinals (from set theory) have analogs in type theory that require corresponding large universe axioms. These are explored in research on proof-theoretically strong systems. For our purposes (HoTT and everyday mathematics), the $\omega$-hierarchy suffices.

## Universe Levels in Practice

When you're working in Lean 4 or Agda, universe levels are mostly inferred automatically. You write:

```lean
-- Lean 4
def id {α : Type*} (x : α) : α := x
-- Type* means "any universe level" -- Lean infers it
```

You encounter universe issues when:
1. **Trying to put $\mathsf{Type}_i$ into itself:** `Type : Type` is rejected
2. **Functions that produce universes:** A function returning a `Type` has type `Type → Type 1` (or higher), not `Type → Type`
3. **Large structures:** Defining a type of all groups requires working in $\mathsf{Type}_1$ since individual groups' carrier sets live in $\mathsf{Type}_0$

The error message "universe inconsistency" in Lean or Agda means you've violated the hierarchy — typically by trying to quantify over all types at a level while living in that same level.

## Universes and Propositions

In HoTT, there's a special universe of *propositions*: types that are "mere propositions" (have at most one element — they're either empty or contractible). This is called $\mathsf{Prop}$ or $\mathsf{hProp}$ (for "h-propositions" or "h-propositional types").

$$\mathsf{Prop} = \sum_{P : \mathsf{Type}} \mathsf{isProp}(P)$$

where $\mathsf{isProp}(P) = \prod_{x\, y : P} x = y$ (all elements are equal).

In Lean 4, `Prop` is a special universe:
```lean
-- Prop is a universe of propositions
-- proof irrelevance holds for Prop
example (p : Prop) (h1 h2 : p) : h1 = h2 := rfl
-- This works because Lean identifies all proofs of Prop-valued types
```

The universe of propositions is important for classical mathematics: in classical logic, propositions are either $\top$ or $\bot$ (true or false). The $\mathsf{Prop}$ universe captures this by requiring propositions to be "proof-irrelevant" — all proofs of the same proposition are equal.

## Universe Summary

| Feature | Description |
|---|---|
| $\mathsf{Type}_0$ | Universe of "small" types (ℕ, Bool, ordinary types) |
| $\mathsf{Type}_i : \mathsf{Type}_{i+1}$ | Each universe lives in the next one |
| Cumulativity | Every type at level $i$ is also at level $i+1$ |
| Russell style | Types are elements of universes directly |
| Tarski style | Universes have codes with a decoding function |
| Universe polymorphism | Functions that work at every universe level |
| Univalence | Equivalences = paths in the universe (HoTT) |

The universe hierarchy is the least glamorous part of type theory — most of the time, you don't think about it. But it's the foundation that prevents the system from being inconsistent, and it's the home of some of HoTT's most powerful axioms.

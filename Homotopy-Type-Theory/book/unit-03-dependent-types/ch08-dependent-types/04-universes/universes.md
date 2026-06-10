# Universes: Types of Types

## The Problem with Type : Type

Consider the expression: "the type of all types." In a type theory where types are terms — where you can form Π(A:?).B(A) — you need to know what type to put in the placeholder. What is the type of a type?

The most natural answer is: Type. So we write Type : Type. Every type is itself a term, and its type is Type.

This turns out to be inconsistent. Per Martin-Löf's original 1971 formulation of type theory included Type : Type. In 1972, Jean-Yves Girard showed that this leads to contradiction via an encoding of Burali-Forti's paradox — the paradox of the "set of all ordinals," which would be larger than any ordinal and thus larger than itself. Girard's paradox shows that any type theory with Type : Type (and sufficient expressive power) can derive ⊥ — a proof of the empty type.

The resolution: a hierarchy of universes. Type₀ : Type₁ : Type₂ : ... Each universe is a type in the next larger universe. No universe contains itself. The hierarchy goes on forever but is stratified so that no universe appears in a universe of lower index.

## The Girard-Hurkens Paradox

To feel why Type : Type fails, here is the essential structure of Hurkens' simplified version of Girard's paradox.

Define:
- P(X) = X → Type (the "power type" of X, analogous to the power set)
- U = Π(X:Type). (P(P(X)) → X) → P(P(X))

If Type : Type, then U : Type. Define an element:
- τ(t) = λX. λf. λp. t(U)(λx. p(f(τ(x)))) : P(P(U)) → U

Then derive contradiction by showing the type of all "small" elements is both bigger and smaller than U. The details are intricate, but the key move is always self-reference: U appears in its own type, which requires Type : Type to type-check.

Without Type : Type, this self-reference is impossible. Each type lives in a universe, and each universe lives in a higher universe, with no cycles.

## The Universe Hierarchy

We introduce a countably infinite sequence of universes:

$$\mathsf{Type}_0 : \mathsf{Type}_1 : \mathsf{Type}_2 : \cdots$$

The rules:

**Formation:** For each natural number i, Type_i is a type (it lives in Type_{i+1}).

$$\frac{}{\Gamma \vdash \mathsf{Type}_i : \mathsf{Type}_{i+1}} \qquad (\mathsf{Universe}\text{-}\mathsf{Form})$$

**Cumulativity:** If A : Type_i, then also A : Type_{i+1}. A type in a lower universe is automatically a type in any higher universe.

$$\frac{\Gamma \vdash A : \mathsf{Type}_i}{\Gamma \vdash A : \mathsf{Type}_{i+1}} \qquad (\mathsf{Cumul})$$

**Closure:** Each universe is closed under the type formers. If A : Type_i and B : A → Type_i, then Π(x:A).B(x) : Type_i and Σ(x:A).B(x) : Type_i. The universe is closed under Π and Σ at the same level.

Most working mathematics lives in Type₀ or Type₁. The higher universes are needed only when you want to quantify over all small types, or prove theorems that refer to Type₀ as a whole.

## Russell-Style vs. Tarski-Style Universes

There are two conventions for how universes work.

**Russell-style universes** (used by the HoTT Book, Lean, Agda): Elements of Type_i *are* types. If A : Type_i, then A is a type — there is no separate "decoding" step. You can write A → B directly, because A is a type.

**Tarski-style universes**: The universe U contains *codes* for types. If a : U is a code, then El(a) is the actual type it encodes. To use a as a type, you must write El(a). Σ(a:U).El(a) is the type of types in U.

Tarski universes are more explicit and easier to reason about metatheoretically (you can always distinguish codes from types). Russell universes are more convenient in practice (you do not have to write El everywhere). Most modern proof assistants use Russell universes.

**In Agda:** `Set` is Type₀, `Set₁` is Type₁, etc. The `universe-polymorphism` flag allows `Set ℓ` for a universe level ℓ.

**In Lean 4:** `Type 0` through `Type n`, and `Sort 0` (which is Prop, the universe of propositions). Universe polymorphism is built in.

**In Coq:** `Set`, `Prop`, `Type`, with universe polymorphism via `Universe` declarations.

## Universe Polymorphism

Consider the polymorphic identity function:

$$\mathsf{id} : \prod_{A:\mathsf{Type}_0} A \to A$$

This works for types in Type₀. But what if you want id to work on types in Type₁ too? You would need:

$$\mathsf{id}_0 : \prod_{A:\mathsf{Type}_0} A \to A \qquad \mathsf{id}_1 : \prod_{A:\mathsf{Type}_1} A \to A \qquad \cdots$$

That is infinitely many definitions. Universe polymorphism solves this: allow the universe level to be a parameter.

$$\mathsf{id} : \prod_{\ell:\mathsf{Level}} \prod_{A:\mathsf{Type}_\ell} A \to A$$

Here Level is the type of universe levels (0, 1, 2, ...) or in some formulations a separate syntactic category. The function id works at every universe level simultaneously.

Agda's `universe-polymorphism` extension (now the default), Lean 4's universe variables, and Coq's universe polymorphism all implement this idea. Without it, you need a separate copy of every definition for each universe level — unworkable in practice.

## What the Universe Allows: Quantifying Over Types

The key use of the universe is in Π types that quantify over types:

$$\prod_{A:\mathsf{Type}_0} \prod_{B:A\to\mathsf{Type}_0} \prod_{a:A} B(a)$$

This is a proposition that ranges over all types in Type₀. Without the universe — without Type₀ being itself a type in Type₁ — this Π type would not be well-formed. The universe is the mechanism by which we can state universal theorems about all types.

In HoTT, the univalence axiom is a statement of the form:

$$\mathsf{ua} : \prod_{A\, B:\mathsf{Type}_0} (A \simeq B) \to (A =_{\mathsf{Type}_0} B)$$

This quantifies over all types in Type₀ and makes a claim about the identity type in Type₁ (since Type₀ : Type₁). The universe hierarchy is essential for even stating univalence.

## Cumulative vs. Non-Cumulative Hierarchies

In a **cumulative** hierarchy (Lean 4, Coq): if A : Type_i, then automatically A : Type_{i+1}. Every small type is also a large type. Cumulativity makes definitions more convenient — you do not need to insert coercions when using a Type₀ type in a context that expects Type₁.

In a **non-cumulative** hierarchy (Agda by default): the levels are strict. A : Type₀ does not automatically give A : Type₁. Instead, you use a lifting operation Lift : Type₀ → Type₁ that "promotes" a type to a higher universe. This is more explicit but also more verbose.

The trade-off: cumulativity makes terms smaller and more natural; non-cumulativity makes the metatheory cleaner and universe inference more predictable.

For practical HoTT, cumulativity is usually convenient. The HoTT Book implicitly uses a cumulative hierarchy.

## Propositional Truncation and the Universe of Props

In some systems (Lean 4, Coq), there is a special universe Prop of *propositions* — types where all proofs are definitionally equal (proof-irrelevant). Lean 4 has `Prop : Sort 0` separate from `Type 0`.

In pure HoTT (univalent foundations), propositions are defined internally: a type P is a *mere proposition* (or h-proposition) if any two elements of P are equal. This is:

$$\mathsf{isProp}(P) = \prod_{x\, y : P} x = y$$

The universe of propositions is then Σ(P:Type₀).isProp(P). This is a large Σ type — an element of Type₁. No separate Prop universe is needed; propositions are defined as the subtype of Type₀ consisting of types with at most one element.

This is the approach taken in the HoTT Book and in HoTT Agda. It keeps the universe structure clean (just Type₀, Type₁, ...) while still allowing the distinction between proof-relevant types (arbitrary types) and proof-irrelevant types (propositions) to be made internally.

## Why This Matters for HoTT

The universe is not just a bureaucratic device for avoiding paradox. It is a *type* — a type whose elements are types. And since in HoTT, types are spaces, the universe is a *space of spaces*. Paths in the universe are not trivial: a path A = B in the universe (where the = is the identity type of the universe) is, by univalence, exactly an equivalence between A and B.

The universe becomes a kind of "moduli space" for types, where paths are equivalences and homotopies are natural isomorphisms. The structure of the universe, as studied in HoTT, is extraordinarily rich. And all of it depends on having the universe hierarchy set up correctly — stratified, consistent, and with enough expressive power to state the theorems we care about.

Getting the universes right is not pedantry. It is the difference between a consistent and inconsistent foundation.

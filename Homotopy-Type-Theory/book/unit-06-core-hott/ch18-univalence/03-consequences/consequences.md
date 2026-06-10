# Consequences of Univalence

## The Propagation Effect

The Univalence Axiom is not an isolated statement about the universe. It propagates through the entire type theory, generating consequences at every level. In this section, we develop the three most important consequences: function extensionality, propositional extensionality, and the general structure invariance principle.

## Function Extensionality

The most immediately useful consequence of Univalence:

**Theorem (Function Extensionality).** Univalence implies funext:
```
funext : (Π(x:A). f(x) = g(x)) → (f = g)
```

for any f, g : A → B.

*Proof via the interval.* The key step: construct the interval type I = {0, 1, seg : 0=1} as a HIT (Chapter 19). With I available:

Given H : Π(x:A). f(x) = g(x), define k : A × I → B by:
```
k(x, 0) = f(x)
k(x, 1) = g(x)
k(x, seg) = H(x)   (using the I-eliminator)
```

By currying, get k̃ : I → (A → B). Then:
- k̃(0) = λx.f(x) = f
- k̃(1) = λx.g(x) = g

So ap_{k̃}(seg) : f = g. ∎

*Alternative proof via Univalence directly.* Consider the total space of the path fibration: `Σ(t:I). B`. The path `seg : 0 = 1` in I lifts via the product to... this approach is more complex; the interval method is cleaner.

**Dependent function extensionality.** The same holds for dependent functions: for f, g : Π(x:A).B(x), a pointwise homotopy Π(x:A). f(x) = g(x) implies f = g.

**The computation rule.** funext satisfies:
- happly(funext(H)) = H (recover the homotopy)
- funext(happly(p)) = p (recover the path)
- funext(λx. refl_{f(x)}) = refl_f (trivial homotopy gives refl)

## Propositional Extensionality

**Theorem (Propositional Extensionality).** For propositions P and Q:
```
propext : isProp(P) → isProp(Q) → (P ↔ Q) → (P = Q)
```

Two logically equivalent propositions are equal.

*Proof.* Suppose P and Q are propositions with P ↔ Q — that is, functions f : P → Q and g : Q → P. Then f and g together give an equivalence P ≃ Q (since P and Q are propositions, any function between them is automatically an equivalence: the fibers are contractible). By Univalence, P ≃ Q implies P = Q via ua(f, isEquiv(f)). ∎

**Why this is remarkable.** In classical logic, "P and Q are logically equivalent" means "P ↔ Q" — they imply each other. This is a weaker statement than "P = Q" in general. For non-propositional types, logical equivalence (mutual implication) is much weaker than equality. But for propositions, the two notions coincide. Propositional extensionality is the type-theoretic version of the classical principle that truth values (Boolean values) are equal iff they are the same truth value.

## The Structure Invariance Principle

The most powerful consequence of Univalence is the general principle that "mathematical structure is invariant under equivalence."

**Theorem (Structure Invariance, informal).** Any statement about types that is formulated using the standard type-theoretic constructions is automatically invariant under equivalence. If A ≃ B and P(A) holds, then P(B) holds.

*Proof (sketch).* By Univalence, A ≃ B gives a path p : A = B. By transport, any type-family P : Type → Type applied to this path gives transport^P(p) : P(A) ≃ P(B). So a proof of P(A) transports along p to a proof of P(B). ∎

**The formal statement.** For any type-theoretically definable predicate P : Type → Type:
- If (A = B), then transport^P(this path) : P(A) ≃ P(B).
- By Univalence, if (A ≃ B), we get a path A = B, hence transport^P : P(A) ≃ P(B).

This means: P cannot distinguish equivalent types. Any property of A that is expressible in type theory automatically holds for any equivalent B.

**Corollary (Invariance of structure).** If we define "a group" as a type G with a binary operation m : G × G → G satisfying group axioms, then any two equivalent groups satisfy the same group-theoretic statements.

More precisely: `Group = Σ(G:Type). isGroup(G)`. An equivalence of groups (a group isomorphism) corresponds, by Univalence and the Sigma-path characterization, to an equality of groups in the type `Group`. So group-theoretic statements about G hold for any isomorphic group H.

This is the formal content of the informal mathematician's practice: "Without loss of generality, let G = Z/nZ" or "Up to isomorphism, there is only one group of order p." In HoTT, these moves are justified by the structure invariance principle — which follows from Univalence.

## Univalence and Type-Theoretic Mathematics

Univalence makes it possible to do mathematics in HoTT in the way mathematicians actually work, with full formal correctness.

**Defining objects up to isomorphism.** A mathematician says "Let G be a group of order p." In HoTT, this means: let G : Group with |G| = p. By structure invariance, any theorem about G holds for all isomorphic groups — and Univalence makes "isomorphic" identical to "equal."

**Transferring theorems.** If we prove theorem T about the group Z/2Z, and we know H is isomorphic to Z/2Z, then H also satisfies T. In HoTT, this is just transport along the path Z/2Z = H (given by Univalence applied to the isomorphism).

**Type-checking invariance.** Any well-typed expression involving A can be transported to an expression involving an equivalent B, and the result is well-typed. This is the formal expression of "all type-theoretic constructions respect equivalence."

## Consequences for the h-Level Hierarchy

Univalence interacts with the h-level hierarchy in a precise way.

**The universe of propositions is a set.** `Prop = Σ(A:Type).isProp(A)`. Paths between propositions are equivalences of propositions, and equivalences between propositions are propositions (since propositions have at most one element in each direction). So the path type P = Q for P, Q : Prop is a proposition. Hence Prop is a set. ∎

**The universe of sets is a groupoid.** `Set = Σ(A:Type).isSet(A)`. Paths between sets are equivalences of sets (bijections), and the type of bijections between two sets is itself a set. So the path type A = B for A, B : Set is a set. Hence Set is a groupoid (h-level 1). ∎

**The universe is not a set.** The path type Bool = Bool has at least two elements (idToEquiv of which there are two: the identity and the swap). So Type is not a set — it has non-trivial path structure. Type is at least a groupoid, and its full h-level is not finite. ∎

## Summary

| Consequence | Statement | Proof via |
|---|---|---|
| funext | (f~g) → (f=g) | Interval + ua |
| propext | isProp(P) → isProp(Q) → (P↔Q) → P=Q | P≃Q → ua |
| Structure invariance | A≃B → P(A)≃P(B) | Transport along ua |
| Prop is a set | isSet(Prop) | Propext |
| Set is a groupoid | is-1-type(Set) | Equivalences between sets form sets |

Univalence makes HoTT a genuine foundation for mathematics — not just a formal system, but a foundation where the formal rules match the informal mathematical practice of working up to isomorphism. The consequences explored in this section are the formal justifications for moves that mathematicians have always made informally.

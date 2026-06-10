# Intensional vs. Extensional MLTT

## Two Versions of the Same Theory

Martin-Löf presented two major versions of his type theory, and they differ on a single rule. In the intensional version, definitional equality (≡) and propositional equality (=) are genuinely distinct: one is checked automatically, the other requires proof. In the extensional version, a *reflection rule* collapses the two: if you have a propositional proof that a = b, the type checker also accepts a ≡ b.

This single rule changes everything.

The intensional version: type-checking is decidable, propositional equality carries computational content, and the identity type can have multiple distinct elements. This is the foundation of HoTT.

The extensional version: type-checking becomes undecidable, propositional equality is proof-irrelevant, and all identity proofs are equal. This is simpler for some mathematical developments but incompatible with the homotopy interpretation.

## Intensional MLTT

In intensional MLTT (the version we have been developing), the four judgments are fully separated. Definitional equality (≡) is an equivalence relation on terms, closed under computation, decidable by normalization. Propositional equality (=) is a type — something you prove.

**The key property:** In intensional MLTT, UIP (Uniqueness of Identity Proofs) is neither provable nor refutable from the basic rules. You can add it as an axiom (Axiom K) or you can add Higher Inductive Types that violate it. Both are consistent extensions.

**Type-checking is decidable:** Given Γ ⊢ a : A, the type checker can verify this by normalizing a and A and checking the result. The normalization process terminates (strong normalization theorem) and produces unique normal forms (confluence theorem). Type-checking reduces to normal form comparison.

**The computational character:** Every closed term of type ℕ reduces to a numeral (by strong normalization). Every closed function on ℕ computes a definite output on any definite input. The type theory has the same computational character as a programming language.

**Why intensional is the correct version for HoTT:** HoTT requires the identity type to potentially have multiple distinct elements. This requires intensional MLTT (where UIP is not forced) plus the addition of Higher Inductive Types (which actively violate UIP by providing non-trivial path constructors).

## Extensional MLTT

Extensional MLTT adds one rule, the *reflection rule*:

$$\frac{\Gamma \vdash p : a =_A b}{\Gamma \vdash a \equiv b : A} \qquad (\text{Reflection})$$

If you have any propositional proof p : a = b, the type checker accepts a ≡ b as a definitional equality.

**The consequences are immediate and severe.**

**UIP becomes provable:** In extensional MLTT, given any two proofs p, q : a = b, we can derive a ≡ b from p (by reflection) and a ≡ b from q (by reflection). Since definitional equality is symmetric and transitive, we get p ≡ q : a = b... wait, this does not immediately give p = q. But with some additional work: define C(b, p) = Π(q : a = b). p = q. The base case C(a, refl_a) requires refl_a = q for all q : a = a. By reflection on q, we get a ≡ a (trivially), so... the argument is more subtle, but the conclusion holds.

Actually, the standard proof of UIP from reflection uses Streicher's *Axiom K* as an intermediate step. With reflection, you can prove K (every loop is equal to refl), and K implies UIP.

**Type-checking becomes undecidable:** This is the decisive practical consequence. Once propositional equality implies definitional equality, the type checker can be asked to verify p : a = b and then treat a ≡ b. But checking whether a propositional equality holds may require arbitrary computation — even solving halting problems. In particular, you can encode any recursive predicate as a propositional equality, so deciding definitional equality (with reflection) is undecidable.

**The proof-theoretic strength:** Extensional MLTT is equiconsistent with intuitionistic set theory (IZF). Intensional MLTT is also strong but has cleaner proof-theoretic properties.

## The Reflection Rule in Practice

To see why reflection makes type-checking undecidable, consider the following.

In extensional MLTT, suppose you have proven h : f = g where f, g : ℕ → ℕ are two different functions. By reflection, f ≡ g definitionally. Now consider a type like Vec A (f n) — a vector whose length is f(n). Since f ≡ g, the type checker must also accept Vec A (g n) as definitionally equal to Vec A (f n). And to verify this, it must verify that the propositional equality h : f = g can be found — which might require searching through all possible proofs.

More dramatically: if h : n = m where n, m : ℕ are closed terms, reflection forces n ≡ m. Whether n = m holds propositionally depends on whether you can prove it, which could encode any mathematical question. Type-checking becomes mathematically undecidable.

## Extensional Type Theory and NuPRL

The NuPRL proof assistant, developed at Cornell by Robert Constable and collaborators, is based on extensional type theory. In NuPRL, propositional equality and definitional equality are collapsed, and the system relies on a type-checking oracle that is more powerful than a simple normalizer.

NuPRL's approach: the user provides explicit computations alongside proofs, and the system verifies them. Type-checking is not automatic but guided — you provide the type-checking evidence. This makes NuPRL usable despite the undecidability.

NuPRL was influential in the development of verified software: the first mechanically verified proof of a non-trivial program (a distributed protocol) was done in NuPRL. But the undecidability of type-checking means that developing in NuPRL requires more user guidance than in Agda or Coq.

## Why HoTT Must Be Intensional

The argument for intensional MLTT as the foundation of HoTT is definitive:

1. **HoTT requires non-trivial path spaces.** The circle S¹, defined as a Higher Inductive Type, has non-trivial paths (the loop constructor). The fundamental group π₁(S¹) = ℤ is a theorem in HoTT.

2. **Non-trivial path spaces require UIP to fail.** If UIP held, every path from base to base in S¹ would equal refl, collapsing the fundamental group to the trivial group.

3. **UIP fails in intensional MLTT (with appropriate HITs).** The groupoid model shows UIP is not forced. Adding S¹ as a HIT provides an explicit type where UIP fails.

4. **Extensional MLTT forces UIP (via reflection).** In extensional MLTT, all identity proofs are equal, making every type behave like a set.

Therefore, HoTT requires intensional MLTT. The foundation must be the version where definitional and propositional equality are distinct, where the type checker is decidable (by normalization), and where the identity type is free to have rich higher structure.

## Recovering Extensionality in HoTT

HoTT is intensional, but it is not impoverished. Several extensionality principles — the kind of "equal things are definitionally equal" reasoning that makes extensional MLTT convenient — can be recovered.

**Function extensionality:** From univalence, we can prove that any two pointwise-equal functions are (propositionally) equal: if H : Π(x:A). f(x) = g(x), then f = g. This requires a proof (a term of the appropriate identity type) rather than being definitional, but it is available.

**Propositional extensionality:** Two propositions (types with at most one element) are equal iff they are logically equivalent: P = Q iff (P ↔ Q). This also follows from univalence.

**Univalence itself:** The axiom that equivalent types are equal — that Equiv(A, B) → (A = B) — is the most general extensionality principle. It says: two types are equal if they have the same elements up to bijection. This is extensionality at the level of the universe.

These extensionality principles require proofs (terms of identity types) rather than being definitional. But they are available, consistent, and sufficient for all mathematical purposes. The distinction from extensional MLTT is that the equivalences are witnessed by proof terms that can be examined and computed with — not just by the type checker accepting them silently.

## Summary

| Feature | Intensional MLTT | Extensional MLTT |
|---|---|---|
| Reflection rule | No | Yes |
| Type-checking | Decidable | Undecidable |
| UIP | Not derivable | Derivable |
| Identity type | Can have non-trivial structure | Always trivial |
| Function extensionality | Independent (follows from univalence in HoTT) | Built-in |
| Foundation for HoTT | Yes | No |
| Proof assistants | Agda, Coq, Lean (primarily) | NuPRL |

Intensional MLTT is the correct foundation for HoTT because it leaves room for the higher-dimensional structure that makes types into spaces and paths into morphisms. The reflection rule, tempting as it is for its convenience, destroys this structure by conflating the two notions of equality that HoTT needs to keep separate.

The discipline of keeping ≡ and = distinct — of always asking "is this a definitional equality or a propositional one?" — pays dividends in HoTT. The propositional equalities carry more information: they are paths with direction, with homotopy structure, with all the richness of a topological space. Collapsing them into definitional equality is mathematically equivalent to collapsing all spaces to discrete sets. HoTT keeps them distinct, and in doing so, keeps mathematics alive.

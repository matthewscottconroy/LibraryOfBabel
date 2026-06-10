# 6.1 Intensional vs. Extensional MLTT

## The Fork in the Road

Martin-Löf developed type theory in multiple versions. One of the most significant choices in the design of a dependent type theory is what to do with identity proofs: should they carry computational content, or should they collapse to something trivial?

This choice divides type theories into two broad families:
- **Intensional MLTT:** Identity proofs are non-trivial. Two proofs of the same equality can be genuinely different. The type checker does not see through the identity type.
- **Extensional MLTT:** An additional *reflection rule* collapses propositional equality to definitional equality. Any two proofs of the same equality are definitionally equal. This trivializes the identity type but makes type checking undecidable.

The name "intensional" comes from the idea that type checking is *intensional* (it looks at the form/structure of terms) rather than *extensional* (it would need to look at the meaning/equality of terms, which requires running the proof checker recursively).

## Extensional MLTT

**The reflection rule:** In extensional MLTT, there is an additional rule:

$$\frac{\Gamma \vdash p : a =_A b}{\Gamma \vdash a \equiv b : A}$$

If you have a *propositional* equality proof $p : a = b$, you can *reflect* it into a *definitional* equality $a \equiv b$. Propositional equality implies definitional equality.

This rule, combined with J, gives:

**Uniqueness of Identity Proofs (UIP).** Any two proofs of the same equality are definitionally equal. If $p, q : a = b$, then $p \equiv q$, and hence $p = q$ (propositionally, by reflexivity of propositional equality, since $p \equiv q$).

**Effect:** The identity type becomes trivial. There's at most one element of $a = b$ (the "canonical" proof of equality). The identity type doesn't carry any information beyond whether $a$ equals $b$.

**Consequences:**
- Function extensionality holds: if $f(x) = g(x)$ for all $x$, then $f = g$. (Proof: by reflection, $f(x) \equiv g(x)$ for all $x$, so $\lambda x. f(x) \equiv \lambda x. g(x) \equiv f \equiv g$... this needs more care but the idea is right.)
- The theory is essentially a type-theoretic version of ZFC-style set theory where equality is just equality.
- The groupoid interpretation (Section 3.1) collapses: all types are "sets" (0-truncated), with trivial identity types.

**The cost:** Type checking becomes undecidable. The reflection rule means that to determine if two terms have the same type, you might need to decide if they're propositionally equal — which requires checking if there's a proof term of the identity type, which is in general undecidable.

**Systems using extensional MLTT:** Nuprl (the Cornell proof assistant) uses an extensional type theory. The NuPRL/PRL system is a powerful proof assistant based on this foundation. Coq's definitional equality is intensional, but its extensionality axioms move it closer to the extensional camp pragmatically.

## Intensional MLTT

In intensional MLTT, the reflection rule is absent. Propositional equality ($a =_A b$, a type) and definitional equality ($a \equiv b$, a judgment) are strictly distinct.

**Effect:** The identity type can have multiple distinct elements. Two proofs of $a = b$ are in general different terms that may not be definitionally equal.

**Consequences:**
- UIP is *not* provable. There exist models of intensional MLTT where UIP fails.
- The type checker remains decidable (since definitional equality is checked mechanically, without needing to search for identity proofs).
- The theory admits a rich homotopy-theoretic interpretation (Hofmann-Streicher 1994, Awodey-Warren 2009).

**Systems using intensional MLTT:** Agda (with `--without-K`), Lean 4 (for `Type` universes), and Coq (though Coq's `Prop` has proof irrelevance, analogous to UIP for propositions).

## Axiom K and Its Role

**Axiom K** (Streicher, named after Streicher's "K" combinator):
$$K : \prod_{A:\mathsf{Type}} \prod_{a:A} \prod_{p : a = a} p = \mathsf{refl}_a$$

Every loop (self-identity proof) is the trivial loop.

Axiom K is equivalent to UIP: K implies UIP (by a path induction argument), and UIP implies K (trivially, since $p : a = a$ and $\mathsf{refl}_a : a = a$ are two proofs of the same equality, hence equal by UIP).

In Agda:
- `--with-K`: Add Axiom K. Enables the `K` pattern in pattern matching (matching on `refl` even when the pattern has a non-trivial path type).
- `--without-K` (default): Axiom K is not assumed. Required for homotopy type theory in Agda.

In Lean 4:
- For `Prop`: proof irrelevance is assumed, so UIP holds for propositions.
- For `Type`: no UIP is assumed. The `K` combinator is not available (by default) for types with non-trivial homotopy structure.
- `propext` and `Classical.em` (excluded middle) are available as axioms but don't force UIP for types.

## What Makes Intensional MLTT the Right Foundation for HoTT

**Reason 1: Models with non-trivial identity.** Hofmann and Streicher (1994) showed that intensional MLTT has models where types are groupoids, with identity proofs as morphisms. This proved that UIP is genuinely independent of intensional MLTT (it can't be proved or disproved).

**Reason 2: The homotopy hypothesis.** Awodey and Warren (2009) showed that intensional MLTT models can be built from Quillen model categories — the standard framework for abstract homotopy theory. This means types can model homotopy types (spaces up to weak equivalence) in a precise sense.

**Reason 3: Univalence is consistent.** Voevodsky's simplicial set model (2006, published 2012) showed that intensional MLTT + Univalence + HITs is consistent. The model uses Kan simplicial sets (the standard homotopy model of $\infty$-groupoids). This would be impossible in extensional MLTT, where the univalence axiom would collapse to something trivial (since all types would be sets).

**Reason 4: HoTT requires non-trivial loops.** The fundamental group of $S^1$ is $\mathbb{Z}$. To state this in HoTT, the type $\mathsf{base} = \mathsf{base}$ (the loop space of $S^1$ at the basepoint) must be non-trivial — it must have multiple elements (one for each integer). This is impossible in extensional MLTT or with Axiom K.

## The Homotopy Levels (Preview)

In HoTT, types are stratified by their homotopy complexity:

- **$h$-level $-2$ (contractible):** Exactly one element, all paths trivial. $\mathbf{1}$ is contractible.
- **$h$-level $-1$ (propositions):** At most one element. If inhabited, unique up to identity. $\mathbf{0}$, $\mathbf{1}$ are propositions.
- **$h$-level $0$ (sets):** Any two identity proofs are equal (UIP holds for the type). $\mathbb{N}$, $\mathbb{B}$ are sets.
- **$h$-level $1$ (groupoids):** Identity types are sets. The fundamental groupoid of a topological space is a groupoid.
- **$h$-level $n$ (higher):** Identity types are of $h$-level $n-1$.

Extensional MLTT forces everything to be at $h$-level $0$ (sets), which is the appropriate setting for ordinary mathematics. Intensional MLTT allows all $h$-levels, which is the appropriate setting for homotopy theory.

HoTT embraces all $h$-levels. Ordinary mathematics (sets, functions, logic) lives at the lower levels; homotopy theory lives at higher levels; $\infty$-groupoids and $\infty$-categories live at the top. The univalent foundations program uses intensional MLTT so that all these levels coexist coherently.

## Why HoTT Avoids Axiom K (In Core Formalism)

If you add Axiom K to intensional MLTT, you collapse to a system where:
- All identity types are propositions (at most one element)
- All types are sets (at $h$-level $0$)
- The loop space of any type is trivial
- HITs like $S^1$ trivialize: $\mathsf{base} = \mathsf{base}$ would only be $\{\mathsf{refl}\}$, making $\pi_1(S^1) = \mathbf{1}$ (trivial group), which is geometrically wrong

This is why Axiom K is incompatible with HoTT's synthetic topology: it makes all types "flat" sets, destroying the higher-dimensional structure that makes HoTT interesting.

## Proof Irrelevance and the Prop Universe

A middle ground: some type theories have *proof irrelevance* only for *propositions* (types with at most one element), while allowing non-trivial higher structure for general types.

In Lean 4:
- `Prop` is a universe where proof irrelevance holds: any two proofs of `p : Prop` are definitionally equal.
- `Type` is a universe where proof irrelevance does not hold in general.

This corresponds to the idea that "logical propositions" should be proof-irrelevant (the specific proof doesn't matter, only whether the proposition is provable), while "computational types" can have rich structure.

Coq's Prop universe works similarly. Agda has a `--prop` flag for a proof-irrelevant universe.

In HoTT, the distinction is between *mere propositions* (h-propositions: $\mathsf{isProp}(P) = \prod_{x\, y : P} x = y$) and *general types*. You can define $\mathsf{Prop}$ as the subtype of $\mathsf{Type}$ consisting of mere propositions, and work with proof-irrelevant logic in this subuniverse while keeping full higher structure elsewhere.

## Comparison Table

| Feature | Extensional MLTT | Intensional MLTT | Intensional + UIP/K | HoTT (Intensional + Univalence) |
|---|---|---|---|---|
| UIP | Yes | Not provable | Yes (axiom) | For sets only |
| Type checking | Undecidable | Decidable | Decidable | Decidable |
| Funext | Yes | Not provable | Not necessarily | Yes (from Univalence) |
| Homotopy interpretation | Trivial (all sets) | Rich (all $h$-levels) | Trivial (all sets) | Full ($\infty$-groupoids) |
| Loop spaces | Trivial | Non-trivial | Trivial | Rich (fundamental group, etc.) |
| Foundation for math | Yes | Yes | Yes | Yes (stronger) |

## Conclusion

Intensional MLTT is the right foundation for HoTT because:
1. It keeps type checking decidable
2. It allows non-trivial identity types, enabling the homotopy interpretation
3. It admits Univalence as a consistent axiom
4. It can express all the usual mathematics at lower $h$-levels while supporting homotopy theory at higher levels

The price is that some things you expect to be automatic (like funext) require extra axioms or are unavailable. But the HoTT perspective is that this price is worth paying — the extra structure we gain (the ability to do homotopy theory synthetically inside type theory) far exceeds the cost.

Extensional MLTT is the right foundation for computational type theory where you want all mathematical structures to be sets. Intensional MLTT + Univalence is the right foundation for univalent mathematics.

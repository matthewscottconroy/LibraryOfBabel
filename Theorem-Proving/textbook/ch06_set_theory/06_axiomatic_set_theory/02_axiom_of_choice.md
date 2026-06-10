# The Axiom of Choice

> "The axiom of choice is obviously true, the well-ordering principle is obviously false, and who can tell about Zorn's lemma?"
> — Jerry Bona (joke highlighting their logical equivalence)

## An Innocent-Looking Claim

**Axiom of Choice (AC)**: For any family $\mathcal{F}$ of non-empty sets, there exists a *choice function* $f$ such that $f(A) \in A$ for each $A \in \mathcal{F}$.

In plain English: given infinitely many non-empty boxes, you can simultaneously pick one item from each box.

For *finite* families of non-empty sets, choice is trivial: just enumerate the sets and pick an element from each. For *infinite* families, there may be no *definable rule* for picking — the axiom asserts a choice function exists even when no rule is known.

## Why AC is Non-Trivial

Consider an uncountable family of sets, each with no preferred element — for example, the collection of all non-empty subsets of the real numbers. AC says you can pick one element from each. But the reals have no canonical well-ordering visible without AC itself. There is genuinely no algorithm or explicit rule that works.

**AC is independent of ZF**: Gödel showed (1938) that AC cannot be disproved from ZF (AC is consistent with ZF). Cohen showed (1963) that AC cannot be proved from ZF (¬AC is also consistent with ZF). So AC is a genuine choice — mathematicians *decide* to accept it.

Almost all mathematicians accept AC (using ZFC) because the mathematics it enables is cleaner and because its consequences are extensively studied and well-understood.

## Equivalent Forms

Remarkably, AC is equivalent (in ZF) to many seemingly different statements:

- **Well-Ordering Theorem** (Zermelo): Every set can be well-ordered
- **Zorn's Lemma**: Every non-empty partial order in which every chain has an upper bound has a maximal element
- **Tychonoff's Theorem**: Any product of compact topological spaces is compact
- **Every vector space has a basis**
- **Every surjection has a right inverse**
- **Every non-empty product of non-empty sets is non-empty**

These are all equivalent — each implies all others — and they are all equivalent to AC. This web of equivalences is one of the striking structural features of set theory.

## Strange Consequences

AC enables some counterintuitive results:

**Banach-Tarski Paradox**: A solid ball in $\mathbb{R}^3$ can be decomposed into finitely many (about 5–6) pieces and reassembled, using rigid motions only, into two balls each the same size as the original. The "paradox" arises because AC allows choosing elements from uncountably many sets in a way that produces non-measurable sets — sets with no well-defined volume.

**Non-measurable sets**: There exist subsets of $[0,1]$ with no well-defined Lebesgue measure. (Vitali sets are the classic example, constructed using AC to pick one element from each equivalence class under "rational shifts.")

These results are theoretically rigorous but cannot be physically realized — they use AC to make arbitrary choices across uncountable families, which is not a physically implementable procedure.

## AC in Proof Assistants

In Lean 4, AC is available as an axiom:
```lean
#check Classical.choice  -- axiom: if ∃ a, True for α, then α is inhabited
#check Classical.axiomOfChoice  -- the full AC statement
```

Many mathematical proofs in Mathlib use AC implicitly (via `Classical.choice` or related constructs). The constructive subset of mathematics — what is provable without AC or excluded middle — is strictly weaker, but for most practical mathematics, working in ZFC (Lean's classical setting) is standard.

## Exercises
See [problems/ch06_set_theory/05_axiomatic_exercises.md](../../../problems/ch06_set_theory/05_axiomatic_exercises.md)

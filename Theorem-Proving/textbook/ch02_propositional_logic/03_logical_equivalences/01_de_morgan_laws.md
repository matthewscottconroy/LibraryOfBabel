# De Morgan's Laws: Logic's Most Useful Rewrite Rules

> *"Contradict the law of contradiction, and that law promptly goes into action."*
> — Augustus De Morgan

---

Augustus De Morgan (1806–1871) was one of the founders of symbolic logic, a contemporary and friend of George Boole, and a mathematician of considerable range. He is best remembered today for two logical equivalences that bear his name — equivalences so useful that most mathematicians internalize them without even thinking, the way a native speaker uses grammatical rules without consciously knowing them.

De Morgan's laws tell us how negation distributes through conjunction and disjunction. They are the equivalences you reach for when you want to "push a negation inward" — when you want to transform "it is not the case that (both A and B)" into a form that is easier to work with.

## The Laws

**De Morgan's First Law**:
$$\neg(P \wedge Q) \equiv (\neg P \vee \neg Q)$$

"Not both P and Q" is the same as "either not P or not Q."

**De Morgan's Second Law**:
$$\neg(P \vee Q) \equiv (\neg P \wedge \neg Q)$$

"Neither P nor Q" is the same as "not P and not Q."

These are not approximations or rules of thumb. They are logical equivalences — the two sides have exactly the same truth value under every possible valuation, as the truth table confirms:

| P | Q | P∧Q | ¬(P∧Q) | ¬P | ¬Q | ¬P∨¬Q |
|---|---|-----|--------|----|----|-------|
| T | T |  T  |   F    |  F |  F |   F   |
| T | F |  F  |   T    |  F |  T |   T   |
| F | T |  F  |   T    |  T |  F |   T   |
| F | F |  F  |   T    |  T |  T |   T   |

The fourth and seventh columns are identical: ¬(P∧Q) ≡ ¬P∨¬Q. The second law is proved similarly.

## Why These Laws Are So Useful

The De Morgan laws are the key to **negation normal form (NNF)**: the process of pushing all negations inward until they appear only in front of atomic propositions. To convert any formula to NNF:

1. Eliminate → and ↔ (using their definitions in terms of ¬, ∧, ∨)
2. Apply De Morgan laws repeatedly to push ¬ inward
3. Eliminate double negations ¬¬φ → φ

This process always terminates and produces a formula where every ¬ appears directly in front of an atom. NNF is a canonical form that simplifies many operations — in particular, it is the first step in converting a formula to CNF (conjunctive normal form) for SAT solving.

## Intuition Through Examples

The first De Morgan law says: to falsify a conjunction, it suffices to falsify one conjunct.

*"It is not the case that Alice is both tall and blonde."*

This is saying: *either* Alice is not tall, *or* she is not blonde (possibly both). The negation of a conjunction opens up into a disjunction. Dually, the negation of a disjunction closes it down:

*"It is not the case that either Alice or Bob is home."*

This means: Alice is not home AND Bob is not home. Both must be absent for neither to be present.

Here is a concrete programming example. Suppose you want to write the negation of the condition `x > 0 && y > 0`. By De Morgan, the negation is `!(x > 0) || !(y > 0)`, which simplifies to `x <= 0 || y <= 0`. Every programmer who transforms `!(a && b)` to `!a || !b` is applying De Morgan's first law. Every time.

## The Laws in Formal Proof Systems

In Lean 4, the De Morgan laws are theorems that require proof. The interesting thing is that the proof of the → direction (`¬(P ∧ Q) → ¬P ∨ ¬Q`) *requires* classical logic (specifically, the law of excluded middle). Here is why:

Given `h : ¬(P ∧ Q)`, we want to prove `¬P ∨ ¬Q`. Intuitively, we do a case split on P: if P is false, we use `Or.inl (fun hp => absurd hp hnp)`; if P is true, then since `h` says P and Q cannot both hold, Q must be false, giving `Or.inr`.

But the case split on P — assuming either P holds or ¬P holds — is an instance of the law of excluded middle. Without LEM, we cannot split on P and proceed. The intuitionistic version of De Morgan's first law is *weaker*:

$$(\neg P \vee \neg Q) \rightarrow \neg(P \wedge Q) \quad \text{(valid intuitionistically)}$$
$$\neg(P \wedge Q) \rightarrow (\neg P \vee \neg Q) \quad \text{(requires classical LEM)}$$

This distinction — between what is provable classically and what is provable constructively — will be a recurring theme in this textbook. For now, the lesson is: even "obvious" logical laws have a hidden complexity when examined carefully.

## De Morgan in Digital Logic

In hardware design, De Morgan's laws have a direct manifestation. A NAND gate computes ¬(P ∧ Q). A NOR gate computes ¬(P ∨ Q). De Morgan's laws say:

$$\text{NAND}(P, Q) = \text{OR}(\text{NOT}(P), \text{NOT}(Q))$$
$$\text{NOR}(P, Q) = \text{AND}(\text{NOT}(P), \text{NOT}(Q))$$

This means NAND gates can simulate OR (with NOT gates), and NOR gates can simulate AND. Since NAND alone is functionally complete, digital circuits can be built entirely from NAND gates — which is extremely useful since NAND is one of the cheapest and most area-efficient gates to manufacture in CMOS technology.

The connection is not accidental. Boolean algebra, the algebra of propositional logic, *is* the mathematics of digital circuits. Claude Shannon's 1937 master's thesis showed that relay circuits implement Boolean functions, and that Boolean algebra could be used to minimize and optimize circuit designs. This observation launched the era of digital computing.

## A Proof in Lean 4

```lean
-- De Morgan's first law in Lean 4 (requires classical logic for → direction)
theorem de_morgan_and_lean (P Q : Prop) : ¬(P ∧ Q) ↔ (¬P ∨ ¬Q) := by
  constructor
  · -- → direction: ¬(P∧Q) → ¬P∨¬Q
    intro h
    by_cases hp : P    -- case split on P (uses LEM)
    · right             -- P holds, so we prove ¬Q
      intro hq
      exact h ⟨hp, hq⟩  -- combining P and Q gives P∧Q, contradicting h
    · left              -- ¬P holds directly
      exact hp
  · -- ← direction: (¬P∨¬Q) → ¬(P∧Q) — constructively valid
    intro h ⟨hp, hq⟩
    cases h with
    | inl hnp => exact hnp hp
    | inr hnq => exact hnq hq
```

Notice the asymmetry: the `·` branch for the → direction uses `by_cases` (which invokes LEM), while the ← branch is purely constructive.

---

*Next: The full family of propositional equivalences — the algebraic laws of logic.*

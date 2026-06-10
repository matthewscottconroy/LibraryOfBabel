# Negation Rules in Natural Deduction

## Negation: Falsum and Explosion

In natural deduction, negation ($\neg\varphi$) is typically treated as an abbreviation: $\neg\varphi \equiv \varphi \to \bot$, where $\bot$ (falsum, bottom, absurdity) is a propositional constant representing a false proposition.

This definition makes negation's rules fall out from implication's:
- To prove $\neg\varphi$: assume $\varphi$ and derive $\bot$
- To use $\neg\varphi$ (with a proof of $\varphi$): derive $\bot$, then anything

**Negation Introduction (¬I):**

$$\frac{\Gamma, \varphi \vdash \bot}{\Gamma \vdash \neg\varphi} \; (\neg I)$$

If assuming φ leads to contradiction (⊥), then ¬φ holds.

**Negation Elimination (¬E) — Explosion:**

$$\frac{\Gamma \vdash \neg\varphi \qquad \Gamma \vdash \varphi}{\Gamma \vdash \psi} \; (\neg E)$$

From $\neg\varphi$ and $\varphi$ (a contradiction), anything follows. This is *ex falso quodlibet* — "from falsehood, anything." Also called **explosion** or the principle of **ex contradictione sequitur quodlibet**.

## Classical vs. Intuitionistic Negation

The rules above (¬I and explosion) are valid in *both* classical and intuitionistic logic. The difference comes with a third rule:

**Double Negation Elimination (classical only):**

$$\frac{\Gamma \vdash \neg\neg\varphi}{\Gamma \vdash \varphi} \; (\text{DNE, classical})$$

This is not valid in intuitionistic logic! Knowing that "it is not the case that φ is false" does not (constructively) produce a proof of φ — it means any proof of ¬φ leads to contradiction, but does not give a direct proof of φ.

Similarly, **Excluded Middle** ($\varphi \vee \neg\varphi$) is not derivable in intuitionistic natural deduction.

## Proof by Contradiction (Classical)

With Double Negation Elimination, we get **proof by contradiction**:

$$\frac{\Gamma, \neg\varphi \vdash \bot}{\Gamma \vdash \varphi} \; (\text{RAA: reductio ad absurdum})$$

Assume ¬φ leads to contradiction, conclude φ. This is classically valid and is the familiar proof technique:
"Assume for contradiction that φ is false. Then... [derive contradiction]. Therefore φ must be true."

## In Lean 4

```lean
-- Intuitionistic (available without Classical):
theorem neg_intro (h : P → False) : ¬P := h
theorem explosion (h : False) : P := h.elim

-- Classical (requires open Classical or import):
open Classical in
theorem double_neg_elim (h : ¬¬P) : P := by_contra (fun hp => h hp)

-- In intuitionistic logic, we can prove:
theorem neg_neg_intro (h : P) : ¬¬P := fun hnp => hnp h
-- But not: ¬¬P → P (without classical axioms)
```

## Constructive Content

The gap between classical and intuitionistic negation is not merely academic. It reflects a deep question about the nature of mathematical existence:

- **Classical**: to prove φ, it suffices to assume ¬φ and derive contradiction. "Proof by contradiction" is always valid.
- **Constructive**: to prove φ, you must exhibit a direct proof. Ruling out ¬φ does not automatically give you φ — you might have eliminated the wrong assumption.

In Coq and Lean (without classical axioms), ¬¬P → P is *not* provable for arbitrary P. This means that some classical theorems cannot be formalized without adding the LEM axiom — and their proofs are genuinely non-constructive.

## Exercises
See [problems/ch04_proof_systems/](../../../problems/ch04_proof_systems/)

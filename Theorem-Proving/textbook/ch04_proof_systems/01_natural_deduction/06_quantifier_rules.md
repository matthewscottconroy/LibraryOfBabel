# Quantifier Rules in Natural Deduction

## Extending to First-Order Logic

The propositional natural deduction system (∧, ∨, →, ¬) extends to first-order logic by adding rules for the quantifiers ∀ and ∃. These rules formalize the informal reasoning we do with "for all" and "there exists."

## Universal Quantifier Rules

**Universal Introduction (∀I)**:
$$\frac{\Gamma \vdash \varphi[x/x]}{\Gamma \vdash \forall x\, \varphi(x)}$$

where $x$ is not free in any assumption in $\Gamma$.

To prove $\forall x\, \varphi(x)$: take an *arbitrary* $x$ (not mentioned in the hypotheses) and prove $\varphi(x)$. Since $x$ is arbitrary, the conclusion holds for all $x$.

**Universal Elimination (∀E, Universal Instantiation)**:
$$\frac{\Gamma \vdash \forall x\, \varphi(x)}{\Gamma \vdash \varphi[t/x]}$$

From $\forall x\, \varphi(x)$, substitute any term $t$ for $x$ to get $\varphi[t/x]$.

## Existential Quantifier Rules

**Existential Introduction (∃I, Existential Generalization)**:
$$\frac{\Gamma \vdash \varphi[t/x]}{\Gamma \vdash \exists x\, \varphi(x)}$$

If $\varphi$ holds for a specific term $t$, then $\exists x\, \varphi(x)$ holds.

**Existential Elimination (∃E)**:
$$\frac{\Gamma \vdash \exists x\, \varphi(x) \qquad \Gamma, \varphi[y/x] \vdash \psi}{\Gamma \vdash \psi}$$

where $y$ is a fresh variable not free in $\Gamma$, $\psi$, or $\exists x\, \varphi(x)$.

From $\exists x\, \varphi(x)$: to conclude $\psi$, assume $\varphi(y)$ for an *arbitrary witness* $y$ (not mentioned elsewhere) and derive $\psi$. Since the choice of $y$ was arbitrary, $\psi$ must follow just from the *existence* of some element satisfying $\varphi$.

## In Lean 4

```lean
-- Universal introduction (intro x):
example (h : ∀ n : ℕ, n + 0 = n) : ∀ m : ℕ, m + 0 = m := h

-- Universal elimination (apply/specialize):
example (h : ∀ n : ℕ, n > 0) : 5 > 0 := h 5

-- Existential introduction (exact ⟨witness, proof⟩):
example : ∃ n : ℕ, n > 10 := ⟨11, by norm_num⟩

-- Existential elimination (obtain):
example (h : ∃ n : ℕ, n > 10) : ∃ m : ℕ, m > 5 := by
  obtain ⟨n, hn⟩ := h
  exact ⟨n, by linarith⟩
```

## Exercises
See [problems/ch04_proof_systems/](../../../problems/ch04_proof_systems/)

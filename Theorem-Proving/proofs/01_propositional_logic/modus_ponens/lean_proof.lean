-- Modus Ponens in Lean 4

-- Term-mode proof (explicit)
theorem mp_term (P Q : Prop) (hp : P) (hpq : P → Q) : Q := hpq hp

-- Tactic-mode proof
theorem mp_tactic (P Q : Prop) (hp : P) (hpq : P → Q) : Q := by
  apply hpq
  exact hp

-- Via `exact`
theorem mp_exact (P Q : Prop) (hp : P) (hpq : P → Q) : Q := by
  exact hpq hp

-- Using `assumption`
theorem mp_assumption (P Q : Prop) (hp : P) (hpq : P → Q) : Q := by
  apply hpq; assumption

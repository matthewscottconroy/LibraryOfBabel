-- Curry-Howard Correspondence in Lean 4
-- Chapter 11, Section 2

-- In Lean 4, propositions ARE types and proofs ARE programs.
-- This file demonstrates both views simultaneously.

-- ── Conjunction = Product ─────────────────────────────────────

-- As a theorem:
theorem and_comm_thm (P Q : Prop) (h : P ∧ Q) : Q ∧ P := ⟨h.2, h.1⟩

-- As a program (on types, not propositions):
def prod_swap {α β : Type} (p : α × β) : β × α := (p.2, p.1)

-- They have the same computational structure!

-- ── Implication = Function ────────────────────────────────────

theorem imp_id (P : Prop) : P → P := fun h => h
def   id_fun {α : Type} : α → α := fun x => x
-- Same thing.

-- ── Disjunction = Sum ─────────────────────────────────────────

theorem or_comm_thm (P Q : Prop) (h : P ∨ Q) : Q ∨ P :=
  h.elim Or.inr Or.inl

def sum_swap {α β : Type} (s : α ⊕ β) : β ⊕ α :=
  s.elim Sum.inr Sum.inl

-- ── Falsum = Empty Type ───────────────────────────────────────

theorem false_elim (P : Prop) (h : False) : P := h.elim
def empty_elim {α : Type} (e : Empty) : α := e.elim

-- ── Universal = Dependent Function ───────────────────────────

-- ∀ x : α, P x  corresponds to  (x : α) → P x
-- A function that, for each element x, produces a proof of P x

theorem forall_imp (P Q : Nat → Prop)
    (h : ∀ n, P n → Q n) (hp : ∀ n, P n) : ∀ n, Q n :=
  fun n => h n (hp n)

-- ── Existential = Dependent Pair ──────────────────────────────

-- ∃ x : α, P x  corresponds to  Σ x : α, P x

theorem exists_example : ∃ n : Nat, n * n = 25 := ⟨5, by norm_num⟩

def sigma_example : Σ n : Nat, n * n = 25 := ⟨5, by norm_num⟩

-- ── Programs are proofs: certified computation ────────────────

-- A function with a postcondition is a proof-carrying program
def div_by_two (n : Nat) (h : 2 ∣ n) : Nat := n / 2

-- Its type encodes the precondition; the caller must provide the proof
#eval div_by_two 8 ⟨4, rfl⟩

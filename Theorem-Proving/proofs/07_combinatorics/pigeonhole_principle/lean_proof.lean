-- Pigeonhole Principle in Lean 4

import Mathlib.Combinatorics.Pigeonhole

-- Mathlib's version:
#check Finset.exists_lt_card_fiber_of_nsmul_lt_card

-- Basic form: any map from a larger to a smaller finite type is not injective
theorem pigeonhole_basic {α β : Type*} [Fintype α] [Fintype β]
    (h : Fintype.card β < Fintype.card α) (f : α → β) :
    ¬ Function.Injective f := by
  intro hinj
  exact absurd (Fintype.card_le_of_injective f hinj) (not_le.mpr h)

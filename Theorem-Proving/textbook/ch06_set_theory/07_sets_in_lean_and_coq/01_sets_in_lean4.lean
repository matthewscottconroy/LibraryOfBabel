-- Sets in Lean 4
-- Chapter 6, Section 7

import Mathlib.Data.Set.Basic
import Mathlib.Data.Set.Function

variable {α β : Type*}
variable (A B C : Set α)

-- ── Basic set operations ───────────────────────────────────────

-- Membership
example (x : α) (h : x ∈ A) : x ∈ A ∪ B := Set.mem_union_left B h

-- Subset
theorem subset_union_left : A ⊆ A ∪ B := Set.subset_union_left

-- Intersection distributes over union
theorem inter_union_distrib : A ∩ (B ∪ C) = (A ∩ B) ∪ (A ∩ C) :=
  Set.inter_union_distrib_left A B C

-- De Morgan for sets
theorem set_de_morgan_compl_union : (A ∪ B)ᶜ = Aᶜ ∩ Bᶜ :=
  Set.compl_union A B

-- ── Functions on sets ──────────────────────────────────────────

variable (f : α → β)

-- Image and preimage
#check Set.image    -- f '' A : Set β
#check Set.preimage -- f ⁻¹' B : Set α

-- Injectivity ↔ image of intersection = intersection of images
theorem inj_iff_image_inter (hf : Function.Injective f) (A B : Set α) :
    f '' (A ∩ B) = f '' A ∩ f '' B := Set.image_inter hf

-- ── Exercises ──────────────────────────────────────────────────

-- Exercise 1: A ∩ B ⊆ A
theorem inter_subset_left_ex : A ∩ B ⊆ A := by
  sorry

-- Exercise 2: A ⊆ B → A ∩ C ⊆ B ∩ C
theorem mono_inter (h : A ⊆ B) : A ∩ C ⊆ B ∩ C := by
  sorry

-- Exercise 3: (A ∪ B)ᶜ = Aᶜ ∩ Bᶜ  (prove from scratch)
theorem compl_union_ex : (A ∪ B)ᶜ = Aᶜ ∩ Bᶜ := by
  sorry

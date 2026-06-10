# 2.1 Formalization Frontiers

## Why Formalization Research Matters

Formalization — writing machine-verified proofs in a proof assistant — is not just an engineering task. It is a form of mathematical research.

When you formalize a theorem, you are forced to:
- Make every implicit assumption explicit
- Handle every degenerate case that classical proofs wave away
- Find the "right" definitions that make the proof work smoothly
- Discover gaps in proofs you thought you understood

The process of formalization regularly produces new mathematical insights. Brunerie's proof of $\pi_4(S^3)$ revealed the "Brunerie number" — a specific integer defined by the proof that needed to be computed to verify the result. The encode-decode proof of $\pi_1(S^1) = \mathbb{Z}$ clarified the "winding number" construction in a way that the classical proof obscures.

Formalization research is mathematics.

## The Cubical Agda Library

The Cubical Agda library (github.com/agda/cubical) is the primary repository for HoTT formalization. It is organized as:

```
Cubical/
  Core/          -- Primitives: interval, path type, hcomp, transp
  Foundations/   -- h-levels, equivalences, univalence, ua
  HITs/          -- Circle, suspension, spheres, pushouts, truncations
  Homotopy/      -- Homotopy groups, fibrations, covering spaces
  Data/          -- ℕ, ℤ, ℚ, finite sets, etc.
  Algebra/       -- Groups, abelian groups, group cohomology
  Cohomology/    -- Eilenberg-MacLane spaces, cohomology operations
```

### Current Gaps

**Synthetic homotopy theory:**
- $\pi_n(S^n) = \mathbb{Z}$ for $n \geq 2$: the base case $n = 1$ is done; the inductive step using Freudenthal is not fully written
- $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$: proved (Brunerie number formalized), but the conceptual proof is lacking
- Blakers-Massey with sharpness: the bound is proved; sharpness absent
- Long exact sequence for general fibrations: partial

**Higher algebra:**
- Eilenberg-MacLane spaces $K(G, n)$ for $n > 1$: partially defined, spectral sequence machinery absent
- Spectra and stable homotopy: sphere spectrum defined, but minimal development
- K-theory: essentially absent

**Category theory:**
- Groupoids and fundamental groupoid: basic definitions present, but not full theory
- Simplicial sets in Cubical Agda: absent (they live in simplicial type theory)

### How to Contribute

**Step 1: Clone and build the library.**
```bash
git clone https://github.com/agda/cubical
cd cubical
agda --cubical Cubical.Everything
```

**Step 2: Browse the Issues.** The GitHub Issues page has "wanted theorem" and "enhancement" labels marking desired contributions.

**Step 3: Identify a theorem you want to formalize.** Start small — a single lemma that fills a gap.

**Step 4: Write the formalization.** The library conventions:
- Files go in the appropriate directory
- Theorems should use the existing library infrastructure
- Follow the naming conventions in `CONVENTIONS.md`

**Step 5: Submit a pull request.** The Cubical Agda maintainers are responsive and helpful.

## The Lean 4 / Mathlib Ecosystem

Lean 4 and Mathlib cover classical mathematics, but HoTT-specific content is largely absent.

### What Mathlib Has

- Group theory: free groups, presentations, fundamental group of a topological space (via covering spaces)
- Topology: general topology, fiber bundles, covering spaces
- Algebraic topology: homology and cohomology for simplicial complexes

### What Mathlib Lacks (HoTT-specific)

- Higher inductive types (not definable in Lean 4 without K)
- Synthetic proofs of $\pi_1(S^1) = \mathbb{Z}$ (would require HITs)
- Seifert-van Kampen at the HIT level (classical topology version exists)
- Synthetic homotopy groups

### Possible Lean 4 Contributions

**Classical proofs with HoTT inspiration:**
- Formalize the Seifert-van Kampen theorem using Mathlib's covering space theory
- Formalize the Hopf fibration as a classical fiber bundle
- Formalize homotopy groups using the classical topology in Mathlib

**A standalone HoTT library for Lean 4:**
Some researchers are working on a separate HoTT library for Lean 4 that postulates the HoTT axioms and works within them. This is not standard (Lean 4 was not designed for this), but it's possible with careful use of axioms.

**Contributing to Mathlib:**
The standard Mathlib contribution process (detailed in Chapter 21) applies. The Mathlib Zulip is very active. Start by fixing a small bug or adding a missing lemma, then work up to larger projects.

## The Rzk Library

The Rzk proof assistant (Chapter 24) is new and its library (sHoTT) is small. This means contributions are high-impact.

### What sHoTT Has

- Basic definitions of Segal types and Rezk types
- The Yoneda lemma (formalized)
- Basic adjunction theory

### What sHoTT Needs

- Limits and colimits in Segal types
- The Grothendieck construction (correspondence between left fibrations and covariant functors)
- Stable ∞-categories
- Presheaf ∞-toposes

### How to Contribute

The Rzk repository is at github.com/rzk-lang/rzk. The syntax is designed to be close to the Riehl-Shulman papers, so reading those papers and translating theorems into Rzk is the main contribution pathway.

## The Archive of Formal Proofs (for Other Systems)

The Archive of Formal Proofs (AFP) is the main repository for Isabelle/HOL formalizations. While not HoTT-specific, it demonstrates the culture of formalization research:
- Each entry is a formalized proof of a specific theorem
- Entries are peer-reviewed
- They become a permanent, citable part of the mathematical record

An analogous archive for Cubical Agda is nascent; contributing to it would be valuable.

## Practical Skills for Formalization Research

**Learn the tools deeply.** Spending 10 hours learning Cubical Agda's interaction mode, understanding how `hcomp` and `transp` work, reading existing proofs in the library — this investment pays off enormously.

**Start with examples.** Before formalizing your target theorem, formalize several smaller related results. This builds your intuition for what's easy and what's hard in the proof assistant.

**Work in small steps.** The biggest pitfall in formalization is trying to prove a complex theorem in one go and getting stuck. Work incrementally: prove each lemma separately, check each definition compiles, verify each computation rule.

**Use holes liberally.** In Agda, `?` creates a hole. Write the structure of the proof with holes, then fill them in. This lets you understand the overall proof shape before diving into details.

**Talk to others.** The HoTT Zulip (hott.zulipchat.com) and the Lean/Agda Zulips are active. Post your questions, describe what you're working on, ask for help when stuck.

**Read existing proofs.** The best way to learn how to formalize is to read how others have formalized. The Cubical library proofs of `ΩS¹≃ℤ`, the Brunerie proof, the Hopf fibration — these are models of how to structure a large formalization.

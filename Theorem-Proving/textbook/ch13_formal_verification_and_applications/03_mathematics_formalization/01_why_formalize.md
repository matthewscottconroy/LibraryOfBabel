# Why Formalize Mathematics?

> "A proof is a proof. What kind of a proof? It's a proof. A proof is a proof, and when you have a good proof, it's because it's proven."
> — Jean Chrétien (satirically cited in the HoTT community to illustrate the need for precision)

## The Standard of "Mathematical Proof"

What counts as a valid mathematical proof? In practice, a proof is a written argument that convinces the mathematical community. It is published in journals, refereed by experts, and if no one finds a flaw after sufficient scrutiny, it is accepted.

This process works remarkably well — the accumulated body of mathematical knowledge is vast, interconnected, and mostly correct. But "mostly" is a revealing word. Mathematical proofs are produced and checked by humans, and humans make mistakes.

**Famous errors and gaps:**

- **Euler's proof of Fermat's Last Theorem for $n = 3$** (1770): had a gap; the argument tacitly assumed unique factorization in $\mathbb{Z}[\sqrt{-3}]$, which fails. The gap was only noticed and repaired decades later.

- **The Kepler conjecture** (1611-1998): the densest packing of spheres in 3D is the FCC/HCP lattice, with density $\pi/\sqrt{18} \approx 74\%$. Thomas Hales submitted a proof in 1998 — 250 pages of mathematics and 3 gigabytes of computer code. The *Annals of Mathematics* reviewers worked for four years and could only say they were "99% certain" it was correct.

- **Hales's response**: If human verification cannot achieve certainty, use a machine. The **Flyspeck project** (2003–2014) formalized the entire proof in HOL Light. After 11 years and contributions from many researchers, the machine-checked proof confirmed Hales's result with certainty.

- **Voevodsky's experience**: Vladimir Voevodsky — Fields Medal winner — discovered in 2000 that a proof he had published in 1989 contained an error (in the Milnor conjecture work). The error was noticed only when someone tried to build on the results. This experience drove him toward formalization and led to his development of Homotopy Type Theory.

## What Formalization Gives You

**Certainty**: A machine-checked proof is either accepted or rejected — there is no 99%. If the type-checker accepts it, the proof is correct relative to the foundations. This is not merely a convenience; for proofs with millions of steps (like Hales's) or thousands of cases (like the four-color theorem), human checking is simply inadequate.

**Re-use**: Formalized proofs build up a verified library. Mathlib — Lean 4's mathematical library — contains tens of thousands of formalized results. Building on Mathlib, you do not just *cite* that the reals are complete — you have a machine-verified proof that you can build on without fear that the cited result contains a hidden error.

**Discovery**: Writing a formal proof forces you to fill in every gap. Steps that "obviously follow" from what went before sometimes do not follow — and the process of filling them in forces deeper understanding. Several mathematical errors have been caught during formalization attempts.

**Communication**: A formal proof is completely explicit. There is no ambiguity about what is being claimed, what lemmas are used, or what foundational assumptions are made. The entire proof is in the file.

## The Four-Color Theorem

The four-color theorem (1976, Appel and Haken): every planar map can be colored with at most four colors so that no two adjacent regions share a color.

The original proof involved checking about 1,500 configurations by computer — the first major mathematical theorem whose proof relied on computer calculation. Many mathematicians were uncomfortable: can we "understand" a proof that no human has read in full?

**Gonthier's formalization (2005)**: Georges Gonthier formalized the four-color theorem in Coq, producing a proof that:
- Can be checked mechanically in minutes
- Requires no trust in the original 1976 computer programs (which were never formalized)
- Uses a different mathematical argument from Appel-Haken

The formalized proof is *shorter* and more elegant than the original, partly because Gonthier found ways to restructure the argument that a machine could more easily verify.

## The Liquid Tensor Experiment

In December 2020, Peter Scholze — a Fields Medalist working on condensed mathematics — posted a challenge to the mathematics community: verify a specific theorem he had proved (a technical result in "liquid mathematics"), which he was not fully confident in, despite months of effort.

Kevin Buzzard organized a formalization effort in Lean 4. By June 2022, the proof was formalized — and correct. Scholze wrote:

> "I am completely blown away by this achievement... I find it very hard to articulate what exactly has been verified. But I am completely confident now that the proof is correct."

The experiment demonstrated that formalization has reached a level of maturity where cutting-edge mathematical research can be verified — not just undergraduate textbook results.

## Why Lean 4 and Mathlib

**Lean 4** is a dependently-typed proof assistant developed by Leonardo de Moura (Microsoft Research) with an eye toward both formal proof and high-performance programming. Key features for mathematics:

- **Mathlib**: A community library with over 100,000 lemmas, covering most of undergraduate mathematics and significant portions of graduate mathematics (measure theory, algebraic topology, number theory, etc.)
- **Tactic mode**: Proofs can be written interactively, with the system showing the remaining goals at each step
- **Type theory foundation**: Every proof is a term in the Calculus of Inductive Constructions — fully checkable, no special trusted oracles
- **Performance**: Lean 4 is fast enough to compile large mathematical libraries in reasonable time

```lean
-- A small taste of Mathlib's reach
import Mathlib

-- The Cauchy-Schwarz inequality:
#check inner_mul_le_norm_mul_norm  -- in a Hilbert space

-- The fundamental theorem of algebra:
#check Complex.exists_root  -- every non-constant polynomial has a root in ℂ

-- Sylow's theorems:
#check Sylow.exists_subgroup_card_pow_prime  -- Sylow p-subgroups exist

-- The prime number theorem (PNT) -- being formalized as of 2024:
-- π(n) ~ n / ln(n)
```

## Why Coq and Isabelle

**Coq** (now also called "Rocq") predates Lean and has an enormous library of verified results:
- The formalized proof of the four-color theorem (Gonthier)
- The CompCert verified C compiler
- The verified seL4 operating system kernel (uses Isabelle)
- The *Mathematical Components* library (algebraic structures up to Feit-Thompson theorem — the full proof that every finite group of odd order is solvable, a 250-page paper formalized in 170,000 lines of Coq)

**Isabelle** is used for:
- The seL4 verified OS kernel
- Extensive classical analysis libraries (HOL-Analysis)
- Archive of Formal Proofs — thousands of contributed formalized results

## What Formalization Does NOT Do

Formalization verifies correctness *relative to foundations*. If the foundations are inconsistent, all bets are off. But the foundations (ZFC for mathematics, CIC for Lean/Coq) are the most carefully scrutinized formal systems in history — we have very high confidence in their consistency.

Formalization also does not make mathematics *easier* to discover. Finding proofs remains a creative activity requiring insight and mathematical intuition. Formalization makes the *verification* of found proofs machine-checkable — a different activity from discovery.

And formalized proofs can be verbose: a five-line paper proof might become 50 lines of Lean code, because every implicit step must be made explicit. This verbosity is a feature (nothing is hidden) and a bug (tedious to read). Tactic automation is constantly improving to reduce this gap.

## Looking Forward: AI and Formalization

AI-assisted formalization is an active frontier. Large language models can suggest proof steps, fill in routine lemmas, and provide first drafts of formal proofs from natural-language descriptions. Systems like Lean Copilot and DeepMind's AlphaProof (2024) are beginning to automate parts of the formalization workflow.

The long-term vision: a world where mathematical claims are not merely "accepted by community consensus" but *verified* — where important theorems exist in machine-checked form, where the correctness of critical software is guaranteed, and where mathematical knowledge is cumulative in the strongest possible sense.

## Exercises
See [problems/ch13_applications/02_lean_proofs.md](../../../problems/ch13_applications/02_lean_proofs.md)

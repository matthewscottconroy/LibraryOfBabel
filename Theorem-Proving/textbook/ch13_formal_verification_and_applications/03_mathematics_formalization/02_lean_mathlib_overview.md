# Lean Mathlib Overview

## Overview
**Mathlib** is the community library for Lean 4 — a massive, collaboratively developed
formalization of undergraduate and graduate mathematics. It contains over 1 million lines
of formally verified mathematics, covering algebra, analysis, topology, number theory,
category theory, and more.

## Learning Objectives
- Navigate the Mathlib documentation
- Find and apply relevant Mathlib lemmas
- Understand the structure of Mathlib's hierarchy

## Key Areas of Mathlib
- **Algebra**: groups, rings, fields, modules, linear algebra (`Mathlib.Algebra.*`)
- **Analysis**: real and complex analysis, measure theory (`Mathlib.Analysis.*`)
- **Topology**: metric spaces, topological spaces, continuity (`Mathlib.Topology.*`)
- **Number theory**: primes, Diophantine equations, algebraic number theory (`Mathlib.NumberTheory.*`)
- **Combinatorics**: graph theory, finsets, counting (`Mathlib.Combinatorics.*`)
- **Category theory**: functors, natural transformations (`Mathlib.CategoryTheory.*`)
- **Logic**: set theory, order theory, computability (`Mathlib.Logic.*`, `Mathlib.Order.*`)

## Using Mathlib in Your Project
In `lakefile.lean`:
```lean
require mathlib from git
  "https://github.com/leanprover-community/mathlib4"
```

Then `import Mathlib` or specific submodules.

## Key Tactics from Mathlib
- `simp`: simplification with lemma library
- `ring`: proves ring equalities
- `linarith`: linear arithmetic over ordered fields
- `omega`: linear arithmetic over integers/naturals
- `norm_num`: numeric computations
- `aesop`: automated proof search
- `exact?` / `apply?`: search for matching lemmas
- `decide`: decides decidable propositions by computation

## Real-World Impact
- **Fermat's Last Theorem (FLT)**: a Mathlib formalization project is ongoing
- **Sphere eversion**: formalized in Lean 4
- **Liquid tensor experiment**: Clausen-Scholze's condensed mathematics formalized in Lean

## Exercises
See `problems/ch13_applications/02_sat_encoding_exercises.md`

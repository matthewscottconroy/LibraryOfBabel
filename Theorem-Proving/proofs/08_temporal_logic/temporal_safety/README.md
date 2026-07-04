# Temporal Safety and Recurrence

**Statement**: In the two-state system with transitions 0→0, 0→1, 1→0 (and $p$
true exactly at state 1), every valid trace satisfies the LTL properties
$G\,\neg(p \land Xp)$ (safety: never two consecutive $p$-states) and
$GF\,\neg p$ (recurrence: state 0 is visited infinitely often).

A minimal worked example of *verifying temporal properties by theorem proving*:
instead of exploring a finite state space (model checking), we quantify over
all infinite traces and prove the properties deductively.

## Files
- `paper_proof.md`: Kripke structure, LTL statements, and full proofs
- `lean_proof.lean`: Lean 4 formalization (traces as `ℕ → Fin 2`)

## Related
- Textbook: Chapter 14 (Temporal Logic), especially §2 (LTL) and §5 (Model Checking)
- Problems: `problems/ch14_temporal_logic/`

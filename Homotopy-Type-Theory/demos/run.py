#!/usr/bin/env python3
"""
Launcher for HoTT interactive demos.
Run from the project root:  python3 demos/run.py
"""
import subprocess, sys, os

DEMOS = [
    # ── Foundations ──────────────────────────────────────────────────────────
    ("demo_proof_basics.py",      "Proof basics         — propositional logic, induction, predicate logic"),
    ("demo_set_theory.py",        "Set theory           — ZFC, ordinals, cardinals, axiom of choice"),
    ("demo_algebra.py",           "Abstract algebra     — groups, free groups, rings, algebra in HoTT"),
    ("demo_real_analysis.py",     "Real analysis        — metric spaces, convergence, compactness, ℝ in HoTT"),
    ("demo_proof_theory.py",      "Proof theory         — judgments, natural deduction, sequent calculus"),
    # ── Core Type Theory ─────────────────────────────────────────────────────
    ("demo_curry_howard.py",      "Curry-Howard         — propositions as types, proofs as programs"),
    ("demo_normal_forms.py",      "Normal forms         — β/η, Church-Rosser, stuck terms"),
    ("demo_dependent_types.py",   "Dependent types      — Vec n, Fin n, Σ-types: safety through precision"),
    ("demo_induction_vs_recursion.py","Induction vs rec — the motive, J eliminator"),
    ("demo_w_types.py",           "W-types              — universal inductive type, ℕ/List/Ord"),
    # ── Category Theory ───────────────────────────────────────────────────────
    ("demo_categories.py",        "Category theory      — functors, Yoneda, limits, adjunctions, monads"),
    ("demo_categorical_logic.py", "Categorical logic    — CCCs, LCCCs, fibrations, toposes"),
    ("demo_higher_cats.py",       "Higher categories    — 2-cats, homotopy hypothesis, (∞,1)-cats"),
    # ── Classical Topology & Homotopy ─────────────────────────────────────────
    ("demo_simplicial_sets.py",   "Simplicial sets      — Δ, Kan complexes, model structure, HoTT model"),
    ("demo_paths.py",             "Path algebra         — build a space, compose/invert paths, compute π₁"),
    ("demo_groupoid_laws.py",     "Groupoid laws        — higher paths, associativity, Eckmann-Hilton"),
    ("demo_transport.py",         "Transport            — moving values along paths in type families"),
    ("demo_covering_spaces.py",   "Covering spaces      — Galois correspondence, π₁ actions"),
    # ── Homotopy Type Theory ──────────────────────────────────────────────────
    ("demo_hlevels.py",           "H-level hierarchy    — explore the ladder from contractible to ∞-groupoid"),
    ("demo_truncations.py",       "Truncations          — ||A||, ∃ vs Σ, set-truncation, n-types"),
    ("demo_equiv.py",             "Univalence           — type equivalences, ua, transport, automorphism groups"),
    ("demo_univalence_deep.py",   "Univalence (deep)    — idtoeqv, ua, Aut(Bool)=ℤ/2ℤ, transport"),
    ("demo_funext.py",            "Function ext.        — happly, funext from univalence, η-law"),
    ("demo_fundamental_theorem.py","Fund. theorem       — (a=b)≃R(a,b), paths in Σ/Π/×, FTID"),
    ("demo_universes.py",         "Universes            — 𝒰₀:𝒰₁:⋯, Girard's paradox, resizing"),
    # ── Higher Inductive Types ────────────────────────────────────────────────
    ("demo_circle.py",            "π₁(S¹) ≅ ℤ          — walk around S¹, watch the winding number encode as ℤ"),
    ("demo_encode_decode.py",     "Encode-decode        — how we compute π₁(S¹), πₙ(Sⁿ), and π₁ of HITs"),
    ("demo_pushouts.py",          "Pushouts             — S¹, T², Klein bottle, RP², van Kampen"),
    ("demo_quotients.py",         "Quotient types       — ℤ=(ℕ×ℕ)/~, ℚ, circle, quotient groups"),
    # ── Synthetic Homotopy ────────────────────────────────────────────────────
    ("demo_suspension.py",        "Suspension & πₙ(Sⁿ) — the suspension functor and Freudenthal's theorem"),
    ("demo_hopf.py",              "Hopf fibration       — S¹→S³→S², long exact sequence, π₃(S²)=ℤ"),
    ("demo_eilenberg_maclane.py", "Eilenberg-MacLane    — K(G,n), cohomology Hⁿ(X;G), delooping"),
    ("demo_james.py",             "James construction   — J(X), ΩΣX≃J(X), stable homotopy"),
    # ── Semantics ─────────────────────────────────────────────────────────────
    ("demo_kripke.py",            "Kripke semantics     — worlds, forcing, LEM countermodels"),
    ("demo_bhk.py",               "BHK interpretation   — proofs as constructions, ∃ vs ∀"),
    # ── Computational HoTT ───────────────────────────────────────────────────
    ("demo_cubical.py",           "Cubical HoTT         — interval 𝕀, hcomp, ua computable"),
    ("demo_cubical_agda.py",      "Cubical Agda         — paths as functions, HITs, univalence computes"),
    ("demo_lean4.py",             "Lean 4               — tactics, Mathlib, HoTT in Lean"),
    # ── Advanced Topics ───────────────────────────────────────────────────────
    ("demo_simplicial_hott.py",   "Simplicial HoTT      — Segal types, Rezk completeness, directed Yoneda"),
    ("demo_modal_hott.py",        "Modal HoTT           — ♭/♯/○ modalities, cohesion, differential geometry"),
    ("demo_research_frontiers.py","Research frontiers   — open problems, Brunerie number, active research"),
]

def _c(code, t): return f"\033[{code}m{t}\033[0m"
bold = lambda t: _c("1", t)
cyan = lambda t: _c("36", t)
dim  = lambda t: _c("2", t)

def main():
    while True:
        print("\033[2J\033[H", end="")
        print(bold("""
  ╔══════════════════════════════════════════════════════════════════╗
  ║         Homotopy Type Theory — Interactive Demos                 ║
  ╚══════════════════════════════════════════════════════════════════╝
"""))
        for i, (_, desc) in enumerate(DEMOS, 1):
            print(f"  {bold(cyan(str(i)))}   {desc}")
        print(f"\n  {dim('q   quit')}\n")
        try:
            choice = input(bold("  > ")).strip()
        except (EOFError, KeyboardInterrupt):
            break
        if choice in ("q", "quit", "exit"):
            break
        try:
            idx = int(choice) - 1
        except ValueError:
            continue
        if 0 <= idx < len(DEMOS):
            script, _ = DEMOS[idx]
            path = os.path.join(os.path.dirname(__file__), script)
            subprocess.run([sys.executable, path])

if __name__ == "__main__":
    main()

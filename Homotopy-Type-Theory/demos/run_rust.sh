#!/usr/bin/env bash
# Launcher for HoTT interactive Rust demos.
# Run from the demos/ directory:  bash run_rust.sh
# Or from the project root:       bash demos/run_rust.sh

set -euo pipefail
cd "$(dirname "$0")"

BOLD=$'\033[1m'
CYAN=$'\033[36m'
DIM=$'\033[2m'
RESET=$'\033[0m'

DEMOS=(
  "proof_basics:Proof basics          — propositional logic, induction, predicate logic"
  "set_theory:Set theory             — ZFC, ordinals, cardinals, axiom of choice"
  "algebra:Abstract algebra        — groups, free groups, rings, algebra in HoTT"
  "real_analysis:Real analysis          — metric spaces, convergence, compactness"
  "proof_theory:Proof theory           — judgments, natural deduction, sequent calculus"
  "curry_howard:Curry-Howard           — propositions as types, proofs as programs"
  "normal_forms:Normal forms           — β/η reduction, Church-Rosser, stuck terms"
  "dependent_types:Dependent types       — Vec n, Fin n, Σ-types"
  "induction_vs_recursion:Induction vs rec     — the motive, J eliminator"
  "w_types:W-types                — universal inductive type, ℕ/List/Ord"
  "categories:Category theory        — functors, Yoneda, limits, adjunctions, monads"
  "categorical_logic:Categorical logic     — CCCs, LCCCs, fibrations, toposes"
  "higher_cats:Higher categories      — 2-cats, homotopy hypothesis, (∞,1)-cats"
  "simplicial_sets:Simplicial sets       — Δ, Kan complexes, model structure"
  "paths:Path algebra            — build a space, compose/invert paths, π₁"
  "groupoid_laws:Groupoid laws          — higher paths, associativity, Eckmann-Hilton"
  "transport:Transport               — moving values along paths in type families"
  "covering_spaces:Covering spaces       — Galois correspondence, π₁ actions"
  "hlevels:H-level hierarchy      — contractible to ∞-groupoid"
  "truncations:Truncations            — ‖A‖, ∃ vs Σ, set-truncation, n-types"
  "equiv:Equivalences            — type equivalences, ua, automorphism groups"
  "univalence_deep:Univalence (deep)     — idtoeqv, ua, Aut(Bool)=ℤ/2ℤ, transport"
  "funext:Function ext.          — happly, funext from univalence, η-law"
  "fundamental_theorem:Fund. theorem        — (a=b)≃R(a,b), paths in Σ/Π/×, FTID"
  "universes:Universes              — 𝒰₀:𝒰₁:⋯, Girard's paradox, resizing"
  "circle:π₁(S¹) ≅ ℤ           — walk around S¹, winding number encodes as ℤ"
  "encode_decode:Encode-decode         — how we compute π₁(S¹), πₙ(Sⁿ), HITs"
  "pushouts:Pushouts               — S¹, T², Klein bottle, RP², van Kampen"
  "quotients:Quotient types         — ℤ=(ℕ×ℕ)/~, ℚ, circle, quotient groups"
  "suspension:Suspension & πₙ(Sⁿ)   — suspension functor, Freudenthal's theorem"
  "hopf:Hopf fibration         — S¹→S³→S², long exact sequence, π₃(S²)=ℤ"
  "eilenberg_maclane:Eilenberg-MacLane    — K(G,n), cohomology Hⁿ(X;G), delooping"
  "james:James construction     — J(X), ΩΣX≃J(X), stable homotopy"
  "kripke:Kripke semantics       — worlds, forcing, LEM countermodels"
  "bhk:BHK interpretation     — proofs as constructions, ∃ vs ∀"
  "cubical:Cubical HoTT           — interval 𝕀, hcomp, ua computable"
  "cubical_agda:Cubical Agda          — paths as functions, HITs, univalence"
  "lean4:Lean 4                 — tactics, Mathlib, HoTT in Lean"
  "simplicial_hott:Simplicial HoTT      — Segal types, Rezk completeness, Yoneda"
  "modal_hott:Modal HoTT            — ♭/♯/○ modalities, cohesion"
  "research_frontiers:Research frontiers   — open problems, Brunerie number"
)

# Check cargo is available
if ! command -v cargo &>/dev/null; then
  echo "cargo not found — install Rust from https://rustup.rs" >&2
  exit 1
fi

# Build all demos once (no-op if already built)
echo "${DIM}Building demos (first run may take a moment)…${RESET}"
cargo build --quiet 2>&1 || { echo "Build failed — run 'cargo build' for details"; exit 1; }
echo ""

while true; do
  printf "\033[2J\033[H"   # clear screen
  printf "%s\n" "${BOLD}╔══════════════════════════════════════════════════════════════════╗${RESET}"
  printf "%s\n" "${BOLD}║     Homotopy Type Theory — Interactive Rust Demos                ║${RESET}"
  printf "%s\n" "${BOLD}╚══════════════════════════════════════════════════════════════════╝${RESET}"
  echo ""

  i=1
  for entry in "${DEMOS[@]}"; do
    desc="${entry#*:}"
    printf "  %s%2d%s  %s\n" "${BOLD}${CYAN}" "$i" "${RESET}" "$desc"
    (( i++ ))
  done

  echo ""
  printf "  %sq   quit%s\n\n" "${DIM}" "${RESET}"
  printf "  %s> %s" "${BOLD}" "${RESET}"
  read -r choice || break

  [[ "$choice" =~ ^[qQ]$ ]] && break
  [[ "$choice" =~ ^[0-9]+$ ]] || continue
  idx=$(( choice - 1 ))
  [[ $idx -ge 0 && $idx -lt ${#DEMOS[@]} ]] || continue

  entry="${DEMOS[$idx]}"
  bin="${entry%%:*}"
  cargo run --quiet --bin "$bin"
done

echo ""
echo "  ${DIM}Goodbye.${RESET}"

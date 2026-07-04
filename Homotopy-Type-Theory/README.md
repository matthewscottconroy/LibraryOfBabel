# Homotopy Type Theory — Curriculum and Study Tools

A complete self-study curriculum for homotopy type theory (HoTT), from mathematical foundations through research-level formalization, accompanied by an adaptive quiz application that connects to Claude to generate questions dynamically.

---

## What This Project Is

This repository contains everything needed to develop genuine research-level mastery of HoTT and use it for automated theorem proving and foundational research:

- **A fully scaffolded book** in `book/` — 9 units, 27 chapters, each with content sections, exercises (25–35 per chapter), important thinkers, annotated references, thought experiments, and real-world applications, written in vivid Putnam-style prose
- **27 textbook-quality chapters** in `chapters/`, each a directory of section files
- **A master curriculum document** with primary texts, proof assistant exercises, and phase milestones
- **A navigation index** and `NAVIGATION.md` linking the `book/` and `chapters/` views
- **The adaptive quiz** — via the shared Rust workspace at the repository root, plus a legacy standalone `quiz.py`
- **41 interactive Rust REPL demos** — one per topic, each a stateful sandbox for hands-on exploration

The curriculum is designed for someone who wants to reach the research frontier: contributing to Cubical Agda, Mathlib4, or Rzk; working on open problems like Brunerie's number; or developing new type-theoretic tools for automated reasoning.

---

## Repository Layout

```
Homotopy-Type-Theory/
├── book/                  # Fully scaffolded curriculum (9 units × chapters × sections)
│   ├── README.md          # Book overview and navigation
│   ├── unit-01-mathematical-foundations/
│   ├── unit-02-logic-and-computation/
│   ├── unit-03-dependent-types/
│   ├── unit-04-category-theory/
│   ├── unit-05-topology/
│   ├── unit-06-core-hott/
│   ├── unit-07-proof-assistants/
│   ├── unit-08-advanced-foundations/
│   └── unit-09-research-frontiers/
├── curriculum.md          # Master 8-phase study plan with texts and milestones
├── index.md               # Navigation index for all 27 chapters
├── NAVIGATION.md          # Dual-navigation guide across book/ and chapters/
├── subject.toml           # Quiz configuration (chapters, phases, prompt, model)
├── book.toml              # Build configuration for tools/build_book.py
├── quiz.py                # Legacy standalone Python quiz (loads questions/ JSON)
├── questions/             # 1,400+ JSON question bank (27 chapters × 3 difficulties)
├── demos/                 # 41 interactive Rust REPL sandboxes (one per HoTT topic)
│   ├── run_rust.sh        # Rust demo launcher (bash demos/run_rust.sh)
│   ├── run.py             # Python demo launcher (python3 demos/run.py)
│   └── <topic>/           # Each crate: cargo run --bin <topic>
└── chapters/              # 27 chapters, each a directory of section files
    ├── ch00-logic-and-proof/
    ├── ch01-set-theory/
    ├── ch02-abstract-algebra/
    ├── ch03-real-analysis/
    ├── ch04-proof-theory/
    ├── ch05-intuitionistic-logic/
    ├── ch06-curry-howard/
    ├── ch07-stlc-system-f/
    ├── ch08-dependent-types/
    ├── ch09-mltt/
    ├── ch10-category-theory/
    ├── ch11-categorical-logic/
    ├── ch12-higher-categories/
    ├── ch13-topology/
    ├── ch14-homotopy-theory/
    ├── ch15-simplicial-sets/
    ├── ch16-identity-types/
    ├── ch17-h-levels/
    ├── ch18-univalence/
    ├── ch19-higher-inductive-types/
    ├── ch20-synthetic-homotopy/
    ├── ch21-lean4/
    ├── ch22-cubical-agda/
    ├── ch23-cubical-type-theory/
    ├── ch24-simplicial-type-theory/
    ├── ch25-modal-hott/
    └── ch26-research-frontiers/
```

The quiz is the shared Rust workspace at the repository root (`quiz/`); this book
has no per-book quiz application of its own. The `chapters/` entries are
directories (one per chapter); `book/` holds the same material in the unit →
chapter → section hierarchy.

---

## The Eight Phases

| Phase | Chapters | Topics |
|-------|----------|--------|
| 0 | 0–3   | Logic, set theory, abstract algebra, real analysis |
| 1 | 4–7   | Proof theory, intuitionistic logic, Curry-Howard, STLC/System F |
| 2 | 8–9   | Dependent types, Martin-Löf type theory |
| 3 | 10–12 | Category theory, categorical logic, higher categories |
| 4 | 13–15 | Point-set topology, algebraic topology, simplicial sets |
| 5 | 16–20 | Identity types, h-levels, univalence, HITs, synthetic homotopy theory |
| 6 | 21–22 | Lean 4 / Mathlib, Cubical Agda |
| 7 | 23–25 | Cubical type theory, simplicial type theory, modal HoTT |
| 8 | 26    | Research frontiers and open problems |

The phases build strictly on each other. Read the chapters in order.

---

## The Chapters

Each chapter is written at textbook level and includes:

- Formal definitions with precise notation
- Theorems with complete or sketch proofs
- Worked examples
- Code in Lean 4 or Agda where applicable
- Exercises ranging from routine verification to genuine research problems

### Phase 0 — Mathematical Foundations

**Chapter 0: Logic and Proof** — Propositional and predicate logic, all standard proof techniques (direct, contradiction, contrapositive, cases), all forms of induction (mathematical, strong, structural, well-founded). Formal derivation trees. The compactness theorem.

**Chapter 1: Set Theory** — ZFC axioms with motivation, Russell's paradox and why naive comprehension fails, ordinals and cardinals, Cantor's theorem, the axiom of choice and its equivalents (Zorn's lemma, well-ordering theorem, Tychonoff's theorem). Why set theory is an insufficient foundation for HoTT.

**Chapter 2: Abstract Algebra** — Groups through the isomorphism theorems, free groups with the universal property (critical later for HITs and van Kampen), group actions, orbit-stabilizer, Cayley's theorem. Preview of the fundamental group.

**Chapter 3: Real Analysis** — Metric spaces, completeness, Cauchy sequences, the completion theorem, continuous maps, compactness (Heine-Borel, Tychonoff), connectedness, the intermediate value theorem. Connections to topology and paths.

### Phase 1 — Logic and Computation

**Chapter 4: Proof Theory** — Natural deduction for all connectives, derivation trees, detours and redexes, the normalization theorem (Prawitz 1965), strong normalization, the subformula property. Sequent calculus, cut elimination (Gentzen's Hauptsatz).

**Chapter 5: Intuitionistic Logic** — The BHK interpretation of the connectives, IPC, why LEM fails constructively, the disjunction and existence properties, Kripke semantics with the forcing relation, the Gödel-Gentzen translation, Markov's principle, Bishop's constructivism.

**Chapter 6: Curry-Howard** — The central dictionary (propositions-as-types, proofs-as-programs), STLC syntax and typing rules, β-reduction as detour elimination, extension to predicate logic via Π/Σ types, strong normalization as consistency. Preview of identity types as paths.

**Chapter 7: STLC and System F** — Why untyped lambda calculus is unsafe, STLC type safety via progress and preservation, strong normalization via logical relations, Church encodings. System F: universal quantification, parametricity (Reynolds), free theorems, System Fω with kinds.

### Phase 2 — Dependent Types

**Chapter 8: Dependent Types** — Vectors as the motivating example, type families, Π and Σ types with full FIEC rules, universe hierarchy and why `Type : Type` leads to paradox, inductive types (ℕ, lists, W-types). The axiom of choice as a theorem.

**Chapter 9: Martin-Löf Type Theory** — The four judgments, contexts, FIEC for all type formers. The identity type: formation, introduction (refl), elimination (J rule), computation. Path induction vs. based path induction. UIP is not derivable. Transport, ap. Intensional vs. extensional MLTT.

### Phase 3 — Category Theory

**Chapter 10: Category Theory** — Categories with examples (Set, Grp, Top, preorders), functors (covariant and contravariant), natural transformations and naturality, functor categories. Yoneda lemma with complete proof. Universal properties: limits, colimits, pullbacks. Adjunctions (unit-counit, triangular identities, examples). Left adjoints preserve colimits. Monads.

**Chapter 11: Categorical Logic** — Cartesian categories and the CCC/STLC correspondence, slice categories, LCCCs and dependent types (Seely-Hofmann-Dybjer), coherence problems. Grothendieck fibrations. Elementary toposes and the subobject classifier. The internal logic of a topos is intuitionistic. The Awodey-Warren theorem. Voevodsky's simplicial set model.

**Chapter 12: Higher Categories** — Strict 2-categories and bicategories, groupoids, the fundamental groupoid Π₁(X). Grothendieck's homotopy hypothesis. Simplicial sets and Kan complexes. Quasi-categories (Joyal), inner horn filling, ∞-categories. HoTT as the internal language of ∞-toposes.

### Phase 4 — Topology

**Chapter 13: Topology** — Topological spaces, metric topology, quotient spaces (S¹ = ℝ/ℤ), continuous maps, homeomorphisms. Separation axioms, Urysohn's lemma. Connectedness vs. path-connectedness. Compactness, Heine-Borel, Tychonoff. CW complexes.

**Chapter 14: Homotopy Theory** — Homotopy between maps, homotopy equivalence. Fundamental group: paths, loops, π₁. Van Kampen's theorem with examples (π₁(S¹)=ℤ, torus, wedge sum). Covering spaces, path lifting, monodromy, universal cover. Higher homotopy groups πₙ (abelian for n≥2). Eilenberg-MacLane spaces. Fibrations, the long exact sequence, the Hopf fibration.

**Chapter 15: Simplicial Sets** — The simplex category Δ, face and degeneracy maps, simplicial identities. Standard simplex Δ[n], boundary ∂Δ[n], horns Λⁿₖ. Geometric realization, the singular complex, the adjunction |·| ⊣ Sing. Kan complexes, weak homotopy equivalences. The Quillen model structure. Voevodsky's simplicial set model of HoTT.

### Phase 5 — Core HoTT

**Chapter 16: Identity Types as Paths** — The homotopy interpretation: proofs of a=b are paths from a to b. Reflexivity, path concatenation, path inversion — all proved via J. All groupoid laws proved by path induction. Higher paths. Transport (parallel transport), ap (functoriality), homotopies. Function extensionality from the interval type.

**Chapter 17: H-Levels and Truncations** — The h-level hierarchy: contractible (-2), h-props (-1), h-sets (0), n-types. Contractible types as "homotopy singletons". Hedberg's theorem (decidable equality implies set). The propositional truncation ‖A‖ as a HIT with its universal property. Set truncation. The distinction between Σ (explicit witness) and ‖Σ‖ (mere existence).

**Chapter 18: Equivalences and Univalence** — The problem with quasi-inverses. Bi-invertible maps and half-adjoint equivalences (both are propositions). Contractible fibers. The type A≃B. The univalence axiom: idToEquiv : (A=B) → (A≃B) is an equivalence. Consequences: funext, propext, structure invariance. Bool=Bool has two paths.

**Chapter 19: Higher Inductive Types** — The interval (contractible, gives funext). The circle S¹ (base, loop). The S¹ eliminator. The π₁(S¹)=ℤ computation via encode-decode. Suspension ΣA (N, S, merid). Pushouts (inl, inr, glue). Seifert-van Kampen from the pushout universal property. Truncations as HITs. Eilenberg-MacLane spaces. Spectra.

**Chapter 20: Synthetic Homotopy Theory** — The encode-decode method (general setup). Full π₁(S¹)=ℤ proof. Van Kampen in HoTT. The Freudenthal suspension theorem and stability of homotopy groups. The Blakers-Massey theorem (Anel-Biedermann-Finster-Joyal 2017). The Hopf fibration via the join construction. π₃(S²)=ℤ. Brunerie's number and π₄(S³)=ℤ/2ℤ.

### Phase 6 — Proof Assistants

**Chapter 21: Lean 4 and Mathlib** — Installation with elan/lake. File structure and core syntax. All major tactics (intro, apply, exact, rw, simp, ring, omega, induction, rcases, exact?, apply?). Mathlib organization and naming conventions. The CategoryTheory library. Formalizing algebra. Custom tactic metaprogramming via macros and elaborators. Contributing to Mathlib.

**Chapter 22: Cubical Agda** — The `--cubical` pragma. The interval I with i0, i1, complement (~), meet (∧), join (∨). Path types as functions I→A with definitional endpoints. sym, funExt, pathConcat via hcomp. Transport via transp. The Glue type and ua. HITs in Cubical Agda (S¹ definition). The helix family for π₁(S¹)=ℤ. The Cubical library structure.

### Phase 7 — Advanced Foundations

**Chapter 23: Cubical Type Theory** — The CCHM interval as a primitive with De Morgan algebra structure. Face formulas, partial elements, extension types. The `hcomp` (composition) and `transp` (transport) operations. The Kan condition for inductive types. The Glue type and the proof of univalence as a theorem. Canonicity (Huber 2018). Cartesian cubical TT vs. CCHM. XTT. Normalization by evaluation.

**Chapter 24: Simplicial Type Theory** — The second (directed) interval 2. Extension types. The hom type hom_A(a,b) for directed paths. Segal types as ∞-categories (inner horn filling is contractible). Rezk types (complete Segal spaces): isomorphic objects are equal. Functors as plain functions. Natural transformations as directed paths in function types. The synthetic Yoneda lemma. The Rzk proof assistant.

**Chapter 25: Modal Homotopy Type Theory** — Modalities and their universal properties. Lex modalities from reflective subcategories. The cohesion axioms: shape ∫, flat ♭, sharp ♯. The adjunction ∫ ⊣ ♭ ⊣ ♯. Real-cohesive HoTT. De Rham cohomology synthetically. Differential cohomology groups. Principal bundles with connection as types. Gauge theory and Chern-Weil theory in cohesive HoTT.

### Phase 8 — Research

**Chapter 26: Research Frontiers** — A map of open problems: Brunerie's problem, canonicity for Book HoTT, coherence for HITs, directed univalence, π₅(S⁴). Formalization gaps in Cubical Agda, Mathlib4, and Rzk. Connections to algebraic K-theory, topological field theories, chromatic homotopy theory. Concrete starter projects. How to engage with the community. The longer view.

---

## Quick Start

### Start with the book

```
book/README.md  →  nine-unit overview, how chapters are organized, prerequisites
```

### Or read the curriculum for a study plan

```
curriculum.md   →  eight-phase plan with primary texts and milestones
index.md        →  chapter-by-chapter navigation with reading paths by background
```

### Build the book

From the repository root:

```bash
# Build the book (also --html, --markdown, --check)
python3 tools/build_book.py Homotopy-Type-Theory --pdf
```

### Adaptive quiz

The quiz is the shared Rust workspace at the repository root:

```bash
# Take the adaptive quiz
cd quiz && cargo run -p quiz-cli -- --subject ../Homotopy-Type-Theory
```

Generating new questions calls the Claude API and needs `ANTHROPIC_API_KEY`;
see [PROCESS.md](../PROCESS.md#question-bank-generation).

### Legacy standalone quiz (no setup)

A self-contained Python quiz that reads the same `questions/` bank still ships
for offline use:

```bash
python3 quiz.py
```

### Validate before a PR

```bash
python3 tools/validate.py
```

### Interactive Rust demos (hands-on exploration)

Each demo is a stateful REPL sandbox — type commands to build examples, modify
them, and see HoTT concepts in action.

```bash
# Browse all 41 demos in a menu:
bash demos/run_rust.sh

# Or run one directly:
cd demos && cargo run --bin circle        # π₁(S¹) ≅ ℤ
cd demos && cargo run --bin encode_decode # encode-decode method
cd demos && cargo run --bin hopf          # Hopf fibration
```

The Python demo variants (same concepts, interactive Python):
```bash
python3 demos/run.py
```

---

## Prerequisites

The curriculum is self-contained. Phase 0 assumes only mathematical maturity at the level of an advanced undergraduate (comfort with writing proofs, basic familiarity with sets and functions). Everything else is developed from scratch.

For the quiz: a Rust toolchain (the shared `quiz/` workspace), or Python 3.10+
for the legacy `quiz.py`. Generating new questions needs an Anthropic API key.

For proof assistant work:
- **Lean 4**: install via [elan](https://github.com/leanprover/elan)
- **Agda**: install via `cabal install Agda` or `nix-env -iA nixpkgs.agda`
- **Rzk**: install via the [Rzk documentation](https://rzk-lang.github.io)

---

## Community and Resources

| Resource | Link |
|----------|------|
| HoTT Book (free PDF) | homotopytypetheory.org/book |
| HoTT Zulip (primary community) | hott.zulipchat.com |
| Cubical Agda library | github.com/agda/cubical |
| Mathlib4 | github.com/leanprover-community/mathlib4 |
| Mathlib docs | leanprover-community.github.io/mathlib4_docs |
| Rzk | rzk-lang.github.io |
| nLab (reference) | ncatlab.org |
| HoTTEST seminars | homotopytype.theory/seminars |

# Formal Logic and Theorem Proving

*From Propositional Logic to Categorical Foundations*

A comprehensive textbook project treating philosophy, logic, and mathematics as a
unified subject, with formal tools throughout: every topic is developed through its
logic — axiomatizations, formal systems, decidability and completeness results — and
connected to theorem proving in Lean 4 and Coq.

## Structure

```
.
├── textbook/     22 chapters, each a directory of sections with a chapter README.md
├── problems/     Exercises and challenges per chapter, with difficulty ratings
└── proofs/       Worked proofs of key theorems in multiple formats
```

Each proof in `proofs/` is given in up to four formats:

| Format | File |
|--------|------|
| Pen-and-paper proof | `paper_proof.md` |
| Lean 4 formalization | `lean_proof.lean` |
| Coq formalization | `coq_proof.v` |
| Python check / demo | `python_*.py` |

## Chapters

| Chapter | Topic |
|---------|-------|
| [01](textbook/ch01_language_and_logic_foundations/README.md) | **Language and Logic Foundations** — ambiguity, formal languages, validity and soundness, what a proof is |
| [02](textbook/ch02_propositional_logic/README.md) | **Propositional Logic** — connectives, truth tables, equivalences, normal forms, resolution |
| [03](textbook/ch03_first_order_logic/README.md) | **First-Order Logic** — predicates, quantifiers, translation, structures and satisfaction |
| [04](textbook/ch04_proof_systems/README.md) | **Proof Systems** — natural deduction, sequent calculus, resolution, semantic tableaux |
| [05](textbook/ch05_proof_strategies/README.md) | **Proof Strategies** — direct proof, contradiction, contrapositive, cases, existence and uniqueness |
| [06](textbook/ch06_set_theory/README.md) | **Set Theory** — sets, relations, functions, cardinality, ZF axioms and Choice |
| [07](textbook/ch07_induction_and_recursion/README.md) | **Induction and Recursion** — weak, strong, structural, and well-founded induction; termination |
| [08](textbook/ch08_number_theory/README.md) | **Number Theory** — divisibility, primes, modular arithmetic, Euclidean algorithm, Bézout |
| [09](textbook/ch09_model_theory/README.md) | **Model Theory** — structures, soundness and completeness, compactness, non-standard models |
| [10](textbook/ch10_computability_and_incompleteness/README.md) | **Computability and Incompleteness** — Turing machines, the halting problem, Gödel's theorems |
| [11](textbook/ch11_type_theory/README.md) | **Type Theory** — lambda calculus, Curry-Howard, dependent types, homotopy type theory |
| [12](textbook/ch12_modal_and_philosophical_logic/README.md) | **Modal and Philosophical Logic** — Kripke semantics, epistemic and deontic logic, philosophy of logic |
| [13](textbook/ch13_formal_verification_and_applications/README.md) | **Formal Verification and Applications** — Hoare logic, model checking, SAT/SMT, Lean and Coq in practice |
| [14](textbook/ch14_temporal_logic/README.md) | **Temporal Logic** — LTL, CTL, CTL*, model checking algorithms, safety and liveness |
| [15](textbook/ch15_plural_logic/README.md) | **Plural Logic** — plural quantification, Boolos's innocence thesis, second-order expressiveness |
| [16](textbook/ch16_mereology/README.md) | **Mereology** — parthood axioms, classical mereology (GEM), mereology vs. set theory |
| [17](textbook/ch17_information_theory/README.md) | **Information Theory and Logic** — entropy, Kolmogorov complexity, incompressibility arguments, Chaitin |
| [18](textbook/ch18_liars_paradox/README.md) | **The Liar's Paradox and Self-Reference** — Tarski's hierarchy, Kripke fixed points, revenge, paraconsistency |
| [19](textbook/ch19_abstract_algebra/README.md) | **Abstract Algebra and Logic** — groups, rings, fields, lattices and Boolean algebras, universal algebra |
| [20](textbook/ch20_geometry_and_logic/README.md) | **Geometry and Logic** — Euclid as a formal system, non-Euclidean geometry, Hilbert, Tarski's decidability |
| [21](textbook/ch21_category_theory/README.md) | **Category Theory** — categories, functors, limits, adjunctions, monads, toposes |
| [22](textbook/ch22_metaphysics/README.md) | **Metaphysics and the Nature of Objects** — formal ontology, identity, abstract objects, realism debates |

Two appendices — [notation](textbook/appendix-notation.md) and a
[history of logic](textbook/appendix-history.md) — close the textbook.

> **Completion note.** Chapters 1–13 are the mature core. Chapters 14, 16, 18,
> 19, and 21 are fully developed; chapters 15, 17, 20, and 22 are being expanded
> to the same depth and currently cover their topics more briefly.

## Building the Book

Build the full PDF with the shared book builder from the repository root:

```bash
python3 tools/build_book.py Theorem-Proving --pdf
```

Requires `pandoc` and XeLaTeX.

## Quiz

An adaptive quiz over the chapters (driven by each chapter's `README.md` via
`subject.toml`) can be run from the repository root:

```bash
cd quiz && cargo run -p quiz-cli -- --subject ../Theorem-Proving
```

## Tools Used

| Tool | Purpose |
|------|---------|
| **Lean 4 + Mathlib** | Proof assistant; dependent type theory |
| **Coq** | Proof assistant; calculus of inductive constructions |
| **Python (Z3, sympy)** | Computational exploration, SAT/SMT solving |
| **Tarski's World / Carnap** | FOL semantics, blocks-world models (carnap.io — free) |

Getting started:

1. **Lean 4**: `curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh`
2. **Coq**: via `opam` or your system package manager
3. **Python tools**: `pip install z3-solver sympy`
4. **Carnap** (Tarski's World alternative): visit carnap.io — no installation needed

## Philosophy

This book treats logic, proof, and language as a *unified subject* rather than
separate disciplines. Every chapter connects:

- The **philosophical** question (what is meaning? what is proof? what is truth?)
- The **mathematical** formalism (logic, type theory, set theory, algebra)
- The **computational** embodiment (proof assistants, SAT solvers, model checkers)
- The **real-world application** (verification, cryptography, AI, databases)

Proofs are not mere exercises — they are the foundational technology of reliable
software, verified hardware, and secure cryptography.

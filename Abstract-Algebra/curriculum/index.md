# The Cathedral and the Ladder
## A Complete Curriculum in Linear Algebra, Abstract Algebra, and Representation Theory
### Toward Foundations of Mathematics Research

---

> *"The cathedral is built one stone at a time. The ladder reaches toward it before it is finished."*

This curriculum is a structured, self-contained roadmap from first principles through research-level mathematics. It is organized as a sequence of chapters that are deliberately dependent: each chapter assumes fluency with everything that precedes it. Read it like a ladder — each rung is a prerequisite for the next — but keep the cathedral in mind: the whole structure is pointing toward a unified view of algebraic structure, symmetry, and mathematical foundation.

Mathematical notation throughout uses standard $\LaTeX$ conventions. Rendered in any Markdown viewer with MathJax or KaTeX support.

---

## How to Use This Curriculum

Each chapter contains:
- **Learning Objectives** — what fluency looks like at the end
- **Core Concepts** — definitions and structures to master
- **Key Theorems** — results to understand deeply (not just quote)
- **Proof Techniques** — the *how* of the arguments
- **Milestone Exercises** — problems that test structural understanding
- **Connections** — how this chapter threads into later ones

**Pace:** Expect 4–12 weeks per Part depending on prior background. A student starting from Part I with solid calculus should budget 3–4 years for full fluency through Part VI.

**Notation Standard:** $\mathbb{N}, \mathbb{Z}, \mathbb{Q}, \mathbb{R}, \mathbb{C}$ for number systems. $\mathbf{v}, \mathbf{w}$ for vectors. $T, \phi, \psi$ for maps. $G, H, K$ for groups. $R, S$ for rings. $k, F, E$ for fields. $\mathfrak{g}, \mathfrak{h}$ for Lie algebras.

---

## Table of Contents

### Part I — Mathematical Foundations

The bedrock. Without these, everything else is syntax without semantics.

| Chapter | Title | Key Payoff |
|---------|-------|------------|
| [01](ch01-logic-sets-proof.md) | Logic, Sets, and Proof | Formal language; induction; the axiomatic method |
| [02](ch02-relations-functions-cardinality.md) | Relations, Functions, and Cardinality | Equivalence; bijection; infinite sets; Zorn's Lemma |

---

### Part II — Linear Algebra

The first great algebraic structure: vector spaces. This part is the computational and geometric engine for everything that follows.

| Chapter | Title | Key Payoff |
|---------|-------|------------|
| [03](ch03-fields-and-vector-spaces.md) | Fields and Vector Spaces | The abstract definition; span, independence, basis |
| [04](ch04-linear-maps-and-matrices.md) | Linear Maps and Matrices | Rank-nullity; matrix representations; change of basis |
| [05](ch05-determinants-and-multilinear-forms.md) | Determinants and Multilinear Forms | Alternating forms; geometric meaning; cofactor expansion |
| [06](ch06-eigentheory-and-canonical-forms.md) | Eigentheory and Canonical Forms | Diagonalization; Jordan form; rational canonical form |
| [07](ch07-inner-product-spaces.md) | Inner Product Spaces and Spectral Theory | Orthogonality; adjoints; spectral theorem; SVD |
| [08](ch08-multilinear-algebra-tensors.md) | Multilinear Algebra and Tensor Products | Tensors; exterior algebra; symmetric algebra; duality |

---

### Part III — Abstract Algebra

The cathedral begins to rise. Groups, rings, modules, and fields are the four pillars.

| Chapter | Title | Key Payoff |
|---------|-------|------------|
| [09](ch09-group-theory-foundations.md) | Group Theory I — Foundations | Axioms; subgroups; cosets; Lagrange; cyclic groups |
| [10](ch10-group-theory-structure.md) | Group Theory II — Structure and Actions | Homomorphisms; isomorphism theorems; group actions; Sylow |
| [11](ch11-ring-theory.md) | Ring Theory | Ideals; quotients; domains; polynomial rings; factorization |
| [12](ch12-module-theory.md) | Module Theory | Modules over rings; free/projective/injective; structure theorem |
| [13](ch13-field-theory-galois.md) | Field Theory and Galois Theory | Extensions; algebraic closure; Galois correspondence |
| [14](ch14-category-theory.md) | Category Theory | Functors; natural transformations; adjoints; limits; Yoneda |

---

### Part IV — Homological Algebra

The machinery that unifies and generalizes. Essential for modern algebra and topology.

| Chapter | Title | Key Payoff |
|---------|-------|------------|
| [15](ch15-homological-algebra-basics.md) | Complexes, Homology, and Exact Sequences | Chain complexes; the snake lemma; long exact sequences |
| [16](ch16-derived-functors.md) | Derived Functors: Ext and Tor | Projective/injective resolutions; $\mathrm{Ext}^n$; $\mathrm{Tor}_n$ |
| [17](ch17-spectral-sequences.md) | Spectral Sequences | Filtered complexes; convergence; applications in algebra |

---

### Part V — Representation Theory

The synthesis: linear algebra meets group/Lie algebra structure. The heart of modern mathematical physics and number theory.

| Chapter | Title | Key Payoff |
|---------|-------|------------|
| [18](ch18-representations-finite-groups.md) | Representations of Finite Groups | Maschke's theorem; irreducibles; complete reducibility |
| [19](ch19-character-theory.md) | Character Theory | Character tables; orthogonality; Burnside; induced representations |
| [20](ch20-lie-groups-algebras.md) | Lie Groups and Lie Algebras | Manifold groups; exponential map; $\mathfrak{g} = T_e G$ |
| [21](ch21-semisimple-lie-algebras.md) | Semisimple Lie Algebras and Root Systems | Killing form; Cartan subalgebra; root systems; Dynkin diagrams |
| [22](ch22-highest-weight-theory.md) | Highest Weight Theory | Universal enveloping algebra; Verma modules; classification |
| [23](ch23-advanced-representation-theory.md) | Advanced Topics | Modular rep theory; algebraic groups; geometric representation theory |

---

### Part VI — Foundations of Mathematics

The ground beneath the ground. Where the cathedral's foundation meets bedrock.

| Chapter | Title | Key Payoff |
|---------|-------|------------|
| [24](ch24-set-theory-logic.md) | Axiomatic Set Theory | ZFC; ordinals; cardinals; independence; forcing (intro) |
| [25](ch25-model-theory.md) | Model Theory | Structures; completeness; compactness; ultraproducts; algebraic apps |
| [26](ch26-category-theory-foundation.md) | Category Theory as Foundation | Toposes; Lawvere theories; categorical logic; sheaves |
| [27](ch27-topos-homotopy-type-theory.md) | Topos Theory and Homotopy Type Theory | $\infty$-categories; HoTT; univalence; cubical type theory |

---

## The Dependency Graph

```
Ch01 ──► Ch02
  │         │
  ▼         ▼
Ch03 ──► Ch04 ──► Ch05 ──► Ch06
  │         │               │
  │         ▼               ▼
  │       Ch07           Ch08
  │         │               │
  └────┬────┘               │
       ▼                    │
      Ch09 ──► Ch10         │
       │         │          │
       ▼         ▼          ▼
      Ch11 ──► Ch12 ◄──── Ch08
       │         │
       ▼         ▼
      Ch13     Ch14 ──────────────────────┐
       │         │                        │
       └────┬────┘                        │
            ▼                             │
           Ch15 ──► Ch16 ──► Ch17         │
            │                             │
            └────────────────────────┐   │
                                     ▼   ▼
                              Ch18 ──► Ch19
                               │
                               ▼
                              Ch20 ──► Ch21 ──► Ch22 ──► Ch23
                               │
                 Ch24 ──► Ch25 │
                  │            │
                  ▼            ▼
                 Ch26 ──► Ch27
```

---

## Research Orientation

The endpoint of this curriculum is the ability to read and contribute to research in:

- **Algebraic combinatorics** — symmetric functions, Young tableaux, Schubert calculus
- **Number theory** — automorphic forms, Langlands program, $p$-adic representation theory
- **Algebraic geometry** — sheaves, derived categories, motives
- **Mathematical physics** — quantum groups, vertex algebras, topological field theory
- **Foundations** — categorical logic, homotopy type theory, higher algebra

Each of these domains is visible from the top of this curriculum. The chapters here build the prerequisite language and fluency to enter any of them.

---

*Begin with [Chapter 1: Logic, Sets, and Proof](ch01-logic-sets-proof.md).*

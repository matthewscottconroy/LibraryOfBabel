# 23.4 Variations of Cubical Type Theory

## A Design Space, Not a Single System

CCHM is one point in a design space. The cubical type theory program is not a single theory but a family of related theories, each making different choices about the interval structure, composition rules, and the trade-off between definitional equality and propositional equality. Understanding this design space reveals what is essential to the cubical approach and what is contingent.

The key design choices:

1. **What algebra does the interval have?** De Morgan (CCHM), Cartesian (CCTT), or something else?
2. **How much do strict equalities collapse?** Standard (CCHM), boundary-separated (XTT), or two-level?
3. **How are fibrant and strict types related?** All types fibrant (CCHM), or two-level (2LTT)?

## Cartesian Cubical Type Theory

*Cartesian cubical type theory* (CCTT) uses a simpler interval than CCHM. The interval $\mathbb{I}$ in CCTT has:
- Endpoints $0, 1 : \mathbb{I}$
- Meet $\wedge$ and join $\vee$
- **No complement $\sim$**

The absence of complement has one immediate consequence: **path reversal is not definitional**.

In CCHM, `sym p = λ i → p (~ i)` is a definition using complement. In CCTT, there is no $\sim$, so `sym` must be constructed via `hcomp`:

$$\mathsf{sym}(p) :\equiv \mathsf{hcomp}^{a =_A b}_{(i=1) \vee (i=0)}\!\left(\lambda j. \;[(i=0) \mapsto \mathsf{refl}_a,\; (i=1) \mapsto p],\; p(i)\right)$$

(This is schematic; the precise construction fills a 2-cube from $p$ to $\text{refl}$.) The point is that `sym` requires a composition argument, not just a syntactic substitution.

Consequences of removing complement:
- `sym (sym p) = p` is a *path*, not a definitional equality
- Path reversal requires a proof, not just evaluation
- Some cubical arguments that work by symmetry require additional steps

**Why would you want this?** Simplicity. The Cartesian cube category (the presheaf category without complement) is better understood categorically. The metatheory — proving canonicity, normalization, model completeness — is simpler without complement. The implementation in `cooltt` (Angiuli, Sterling, Gratzer) targets Cartesian cubical type theory precisely because the metatheoretical work is more tractable.

**cooltt** (formerly `redtt`) implements CCTT with:
- Normalization by evaluation for the full theory
- Rigorous proof of canonicity
- A verified implementation of the type checker
- Connections to parametric type theory (in ongoing work)

## XTT: Boundary Separation

**XTT** (Sterling, Angiuli, Gratzer, 2019) adds a *boundary separation* principle to cubical type theory.

**Boundary separation**: If two terms $a, b : A$ agree on all faces of a cube — i.e., for every face formula $\phi$ and every specialization to $\phi = 1$, the terms are definitionally equal — then $a = b$ definitionally:

$$\frac{\forall \phi : [\phi \vdash a = b]}{a = b}$$

This is a very strong principle. It says the term-equality relation is *determined by its boundary values*. The type theory becomes "extensional on boundaries."

**Consequences:**
- Associativity of path concatenation becomes *definitional*: $((p \cdot q) \cdot r)(i, j, k) = (p \cdot (q \cdot r))(i, j, k)$ on all faces, so by boundary separation, the terms are definitionally equal.
- The path groupoid laws all hold definitionally.
- Working in XTT feels like a strict type theory for much of the path algebra, even though univalence still holds.

**The cost**: Boundary separation makes the equational theory more complex to decide. The type-checking algorithm must check boundary conditions for all faces, which is computationally expensive. And the metatheory — proving properties of XTT itself — becomes harder.

**XTT and observational type theory**: XTT is closely related to *observational type theory* (OTT), which also makes many propositional equalities definitional by looking at the "observable" structure of types. The connection between these programs is an active research area.

## 2-Level Type Theory (2LTT)

*Two-level type theory* (2LTT, Altenkirch, Capriotti, Kraus, Sattler) is a different approach: rather than designing a single type theory that handles both fibrant (homotopy-theoretic) and strict (definitional) content, 2LTT has two separate layers.

**The two levels:**
- **The outer level (exo-type theory)**: a strict, extensional type theory. Types here have decidable equality; everything is h-set. This level is for metatheoretic reasoning.
- **The inner level (HoTT)**: ordinary homotopy type theory (possibly with cubical features). Types here have the usual homotopy structure.

**Why two levels?** Because some things we want to say *about* type theory cannot be said *in* type theory. For example, reasoning about the shape of a simplex, or defining the semantics of the simplicial interval, requires talking about specific combinatorial objects — things that are best handled in a strict setting.

In 2LTT:
- *Fibrant types* are the inner-level types, with full homotopy structure
- *Strict types* (or *exo-types*) are the outer-level types, with strict equality
- Fibrant types embed into strict types: there is a functor $\iota$ from fibrant to strict
- Not every strict type has a fibrant counterpart

**The key rule**: If $A$ is a strict type and $a, b : A$ satisfy $a = b$ strictly, then $\iota(a) = \iota(b)$ in the fibrant sense. But fibrant equalities need not reflect to strict equalities.

**2LTT and simplicial type theory**: The Riehl-Shulman simplicial type theory (Chapter 24) can be understood as a 2LTT where the outer level handles the simplicial shapes (which need strict equality to reason about combinatorially) and the inner level handles the spaces (which need homotopy theory).

## Normalization by Evaluation (NbE) for Cubical TT

Every implementation of cubical type theory needs a *type-checking algorithm*. The core algorithmic problem: given two terms $t$ and $s$ of the same type, are they definitionally equal?

The standard approach is **normalization by evaluation** (NbE):

1. **Evaluate**: map terms into a *semantic domain* — a mathematical structure that represents the meaning of terms, not their syntax.
2. **Read back**: extract a *canonical syntactic representative* (normal form) from each semantic value.
3. **Compare**: check whether the two normal forms are syntactically identical.

For NbE to work in cubical type theory, the semantic domain must include:
- Dimension variables (as "neutral" elements that can't be simplified further)
- Face formulas (as constraints that may or may not hold)
- Partial elements (defined on some faces, neutral on others)
- The results of `hcomp` and `transp` when the arguments contain neutral terms

The challenge: `hcomp` and `transp` have different computation rules for each type former. When the type family is a neutral term (a variable, or a term containing dimension variables), the composition cannot be computed — it must remain neutral. The NbE algorithm must carefully distinguish between computable and neutral compositions.

**The `cooltt` approach**: The cooltt implementation separates the phase of "normalizing the type" from "normalizing the term." Types are evaluated first to determine which `hcomp`/`transp` rule applies. This modular approach makes the implementation more tractable and more verifiably correct.

**Cubical Agda's approach**: Cubical Agda uses a modified NbE that handles the CCHM interval with complement. The additional algebraic structure (complement, the De Morgan laws) means the semantic domain must also carry this structure. The advantage is that more terms normalize (those using complement in their definitions), but the implementation is more complex.

## What Canonicity and Normalization Mean

*Canonicity* and *normalization* are related but distinct:

**Canonicity**: Every *closed* term of type $\mathbb{N}$ is definitionally equal to a numeral. "Closed" means no free variables — neither term variables nor dimension variables. This is the property needed to say "proofs compute."

**Normalization**: Every term (open or closed) has a unique normal form under reduction. This is stronger: it says not just that the answers are right, but that every term simplifies to a unique canonical expression.

CCHM has canonicity (proved by Huber). Full normalization for CCHM is harder to establish because of the De Morgan structure. Cartesian cubical type theory (CCTT) has both canonicity and full normalization, which is part of why cooltt targeted CCTT.

**Why normalization matters for proof assistants**: A proof assistant's type checker relies on definitional equality — comparing the normal forms of two terms to decide if they are equal. If normalization fails (e.g., some terms have no normal form, or two different normal forms), the type checker may be unsound or incomplete. Cubical Agda works hard to ensure that all terms the user writes normalize, but the theoretical guarantee is strongest for CCTT.

## The Semantics: Fibrant Cubical Sets

All variants of cubical type theory have a common semantic foundation: *fibrant cubical sets*.

**The cube category** $\square$ has objects $[n]$ (the combinatorial $n$-cube) and morphisms that are maps of cubes (faces, degeneracies, connections for CCHM, without connections for CCTT). A *cubical set* is a presheaf $X : \square^{op} \to \mathsf{Set}$.

A cubical set $X$ is *fibrant* (satisfies the Kan condition) if all open boxes in $X$ have fillers. Types in cubical type theory are interpreted as fibrant cubical sets. The composition operation `hcomp` is the witness of fibrancy.

The universe $\mathsf{Type}$ is interpreted as a "universe of small fibrant cubical sets" — a specific fibrant cubical set whose elements are other fibrant cubical sets. The Glue type corresponds to a specific construction in this fibrant universe that witnesses the Kan condition.

The connection to classical topology: fibrant cubical sets with the CCHM interval model the same homotopy theory as Kan simplicial sets. There is a Quillen equivalence between the CCHM cubical model and the Kan-Quillen simplicial model. This means that both give the "same" homotopy theory — the same ∞-groupoids — just with different computational presentations.

## Open Problems and Future Directions

**1. Combining cubical and directed**: The most pressing foundational question is whether cubical type theory (with its undirected interval) and simplicial type theory (with its directed interval) can be unified. Can we have a type theory that is both cubical (computationally) and simplicial (categorically)?

**2. Guarded cubical type theory**: Adding *guarded recursion* (for coinductive types and operational semantics) to the cubical setting. This would enable synthetic domain theory in a computationally sound foundation.

**3. Modular cubical type theory**: Integrating modalities (Chapter 25) with the cubical type theory. The cohesion axioms and the flat/sharp modalities in a computationally complete setting — this would give Cohesive HoTT with computational content.

**4. Higher inductive types in cubical TT**: HITs in the CCHM setting are defined using `hcomp` and path constructors. The theory is well-understood for specific HITs (propositional truncation, suspension, pushouts) but a *general* theory of HITs with arbitrary path constructors and their computation rules is still being developed.

**5. Proof search and automation**: Developing tactics for cubical type theory analogous to `simp`, `omega`, and `decide` in Lean 4. The `cubical` library in Agda has some automation, but a general-purpose cubical tactic language remains an open problem.

The cubical type theory program is young — the CCHM paper is from 2016, Cubical Agda from 2019. The metatheory, semantics, and implementation are all still being refined. The reader who masters this chapter is positioned to contribute.

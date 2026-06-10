# 4.1 Variations of Cubical Type Theory

## The Landscape

CCHM (Cohen-Coquand-Huber-Mörtberg) cubical type theory is not the only cubical type theory. Multiple variants have been developed, each with different design choices about the interval, face structure, and composition operations. Understanding these variations illuminates the design space of computational type theories.

## Cartesian Cubical Type Theory

CCHM uses a De Morgan algebra on the interval: complement $\sim$, meet $\wedge$, join $\vee$. A simpler variant — *Cartesian cubical type theory* (CCTT) — removes the complement and some of the algebraic structure.

**The Cartesian interval.** In CCTT, the interval $\mathbb{I}$ has:
- Endpoints $0, 1 : \mathbb{I}$
- Meet $\wedge$ and join $\vee$ (or just the Cartesian product structure)
- **No complement** $\sim$

**Consequence: path reversal is not definitional.**

In CCHM, `sym p = λ i → p (~ i)` is a definition — it uses $\sim$. In CCTT, there's no $\sim$, so path reversal must be defined via a more complex composition. The result is:
- `sym` exists as a derived operation (via `hcomp`)
- But `sym (sym p) = p` is a *path* (a homotopy), not a *definitional equality*

This is a tradeoff: CCTT is simpler in its primitives, but some path operations that are definitional in CCHM become propositional (require a separate homotopy proof).

**Implementation.** Cartesian cubical type theory is implemented in:
- **cooltt** (Angiuli, Sterling, et al.): A research implementation focused on normalization and type-checking algorithms
- **redtt** (predecessor to cooltt)

**Advantages of CCTT:**
- Simpler type theory (fewer primitives, fewer computation rules)
- Potentially easier to prove metatheoretical results (canonicity, normalization)
- Connects more directly to Cartesian cubical sets (a well-studied category)

**Disadvantages:**
- Path reversal not definitional (requires more explicit constructions)
- Some path operations are more complex

## XTT: Boundary Separation

**XTT** (Sterling, Angiuli, Gratzer, 2019) is a cubical type theory extended with a *boundary separation* axiom.

**The boundary separation axiom:** If two terms agree on all faces of a cube, they are definitionally equal:
$$\text{If } a = b \text{ on all faces } \phi, \text{ then } a = b \text{ (definitionally)}$$

**Consequence:** XTT validates more definitional equalities than CCHM:
- Associativity of path concatenation is *definitional* (not just a path)
- Path groupoid laws hold *definitionally*

This makes XTT a "strict" cubical type theory — more equalities hold strictly (definitionally) rather than up to homotopy.

**The cost:** XTT is more complex to implement and reason about. The additional definitional equalities mean the type checker must handle more cases, and proofs of metatheorems (canonicity, normalization) become harder.

**XTT's realizability semantics.** XTT also includes a connection to *realizability semantics* — a model-theoretic interpretation of type theory using partial computable functions. This connects cubical type theory to programming language theory (where realizability is a standard tool).

## A Comparison Table

| Feature | Book HoTT | CCHM | CCTT | XTT |
|---------|-----------|------|------|-----|
| Path reversal | Axiom (sym) | Definitional (~) | Propositional | Definitional |
| Concatenation assoc. | Axiom | Propositional | Propositional | Definitional |
| Univalence | Axiom | Theorem (Glue) | Theorem (Glue) | Theorem |
| Canonicity | Unknown | Theorem | Theorem | Theorem |
| FunExt | Axiom | Theorem | Theorem | Theorem |
| Implementation | n/a | Cubical Agda | cooltt | n/a |

## Connection to Simplicial Type Theory

An important question: what is the relationship between cubical type theory and simplicial type theory (Chapter 24)?

**The unification question.** Simplicial type theory uses the simplicial interval $\mathbf{2}$ (directed, without complement). Cubical type theory uses $\mathbb{I}$ (undirected, with complement). Can they be unified?

**Attempts at unification:**
1. **Two-level type theory (2LTT):** A type theory with two levels — an "exo-level" (strict) and an "inner level" (homotopy). This can accommodate both the cubical and simplicial intervals in a single system.

2. **Directed type theory:** Various proposals for type theories with directed paths (natural transformations) in addition to undirected paths (equivalences). These are active research areas.

3. **Synthetic ∞-categories with cubical content:** Using Rzk-style simplicial type theory but with a cubical implementation underneath.

The unification of synthetic homotopy theory (HoTT) and synthetic ∞-category theory (simplicial TT) remains an open problem.

## The Semantics: Cubical Sets

Cubical type theory has a natural categorical semantics: *cubical sets*.

**The cube category $\square$.** Objects are $[n] = \{0,1\}^n$ (the vertex sets of $n$-cubes). Morphisms are maps of cubes (face maps, degeneracies, etc.). A *cubical set* is a presheaf $X : \square^{op} \to \mathsf{Set}$.

**Fibrant cubical sets.** A cubical set $X$ is *fibrant* (or *Kan*) if it satisfies the horn-filling condition: any open box in $X$ has a filler.

- $X_0 = X([0]) = X(1)$ is the set of "0-cells" (points)
- $X_1 = X(\{0,1\})$ is the set of "1-cells" (edges/paths)
- $X_2 = X(\{0,1\}^2)$ is the set of "2-cells" (squares)

**Types are fibrant cubical sets.** In the CCHM model, each type $A$ in context $\Gamma$ is interpreted as a fibrant presheaf over the cube category. The Kan condition for $A$ corresponds to the composition operation `hcomp`.

**The universe.** The universe $\mathsf{Type}$ is the fibrant cubical set of "small" fibrant cubical sets. The Glue type corresponds to a specific construction in this fibrant universe.

**Univalence in the model.** The path space of the universe between two types $A$ and $B$ is the type of equivalences $A \simeq B$. This is a theorem about the model (not an axiom), and it corresponds to the type-theoretic fact that `ua` constructs a path in the universe.

## Connection to Simplicial Sets

The Kan-Quillen model structure on simplicial sets (Chapter 15) is the classical foundation for HoTT. Cubical sets provide an alternative but related model.

**The comparison functor.** There are functors:
$$\mathsf{SimplSet} \leftrightarrows \mathsf{CubSet}$$

converting between simplicial and cubical sets. These preserve the homotopy theory (the ∞-groupoid structure), but differ in their computational properties.

**Advantage of cubical sets:** The horn-filling in cubical sets has an explicit algorithmic description (using the De Morgan algebra). This gives rise to the computational rules in CCHM.

**Advantage of simplicial sets:** Simplicial sets are more classical and better connected to classical algebraic topology. The Kan-Quillen model structure is the most studied homotopy theory.

For type theory purposes, cubical sets win because of their computational properties. For connecting to classical topology, simplicial sets are more natural.

## Normalisation by Evaluation

The implementation of cubical type theory in a proof assistant requires a type-checking algorithm. The standard approach is *normalisation by evaluation* (NbE).

**NbE in standard MLTT.** In ordinary MLTT, NbE:
1. Evaluates a term into a semantic domain (denotational semantics)
2. "Reads back" the value into a syntactic normal form
3. Compares normal forms for definitional equality

**NbE in cubical type theory.** For cubical TT, the semantic domain must include:
- Dimension variables and face formulas
- Partial elements (defined on sub-cubes)
- The `hcomp` and `transp` operations, evaluated in the semantic domain
- The Glue type and its operations

The key challenge: `hcomp` and `transp` have different computation rules for each type former. The NbE implementation must case-split on the structure of the type family.

**Implementations:** Cubical Agda uses a variant of NbE, as does `cooltt`. The `cooltt` implementation is particularly clean and is intended as a reference implementation for Cartesian cubical type theory.

## Future Directions

Several research directions extend CCHM cubical type theory:

**1. Cubical type theory with synthetic homotopy theory.** Extending CCHM with the simplicial interval $\mathbf{2}$ to get both undirected paths and directed morphisms — a unified setting for HoTT and ∞-category theory.

**2. Guarded cubical type theory.** Adding *guarded recursion* (coinductive types) to cubical type theory, enabling synthetic domain theory and operational semantics.

**3. Modular cubical type theory.** Adding modalities (Chapter 25) to cubical type theory, giving a setting for cohesive HoTT (spatial HoTT) with computational content.

**4. Proof search in cubical type theory.** Developing automated proving tools (analogous to Lean 4's `omega` and `simp`) that work with cubical path operations. The `cubical` tactic development in Cubical Agda is ongoing.

**5. Dependent cubical type theory.** Extending to *parametric polymorphism*, where functions can be parametric in universe levels, and the parametricity conditions give additional theorems for free.

Cubical type theory is a young field. The CCHM paper is from 2015, and the Cubical Agda implementation is from 2019. The theory is still developing rapidly, with new variants, implementations, and applications appearing regularly.

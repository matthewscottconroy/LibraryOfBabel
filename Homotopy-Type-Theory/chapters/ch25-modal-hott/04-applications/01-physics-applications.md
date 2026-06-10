# 4.1 Gauge Theory and Physics in Cohesive HoTT

## Gauge Theory: The Physical Context

Gauge theory is the mathematical framework underlying the Standard Model of particle physics. The fundamental forces — electromagnetism, the weak force, the strong force — are all gauge theories: they are described by *principal bundles with connection* on spacetime.

- **Electromagnetism**: a $U(1)$-bundle (circle bundle) with connection on spacetime; the connection is the electromagnetic potential $A_\mu$, the curvature is the field strength $F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$
- **Yang-Mills theory**: a $G$-bundle (for a Lie group $G$) with connection; the action principle gives the Yang-Mills equations
- **Gravity**: a principal $GL(4,\mathbb{R})$-bundle with connection (the Levi-Civita connection)

In cohesive HoTT, all of this is naturally described using the classifying types $\mathbf{B}G$ and $\mathbf{B}G_\nabla$.

## The Moduli Stack of Connections

For a compact spacetime $M$ and a Lie group $G$, the *moduli stack of $G$-connections* is:

$$\mathsf{Conn}_G(M) :\equiv (M \to \mathbf{B}G_\nabla)$$

**As a type, this is an ∞-groupoid:**
- *Objects*: connections $\nabla : M \to \mathbf{B}G_\nabla$
- *Morphisms*: gauge transformations $g : M \to G$ acting on connections
- *2-morphisms*: gauge-of-gauge transformations
- Higher morphisms: all higher gauge transformations

This is the *gauge groupoid* of the theory — the fundamental object in gauge theory, naturally presented as an ∞-groupoid in cohesive HoTT.

**Physical observables** are functions on this moduli stack: maps $\mathsf{Conn}_G(M) \to \mathbb{R}$ that are gauge-invariant (constant on morphisms). In cohesive HoTT, these are elements of $\flat \mathsf{Conn}_G(M) \to \mathbb{R}$ (the flat points of the moduli stack).

## Chern-Simons Theory

*Chern-Simons theory* is a 3-dimensional gauge theory with action functional:

$$\mathsf{CS}(\nabla) :\equiv \int_M \mathsf{tr}(A \wedge dA + \tfrac{2}{3} A \wedge A \wedge A)$$

for a connection $\nabla$ with local $\mathfrak{g}$-valued 1-form $A$.

In cohesive HoTT, the Chern-Simons form is a class in the differential cohomology:

$$\mathsf{CS} \in \hat{H}^3(M, \mathbb{Z})$$

The Chern-Simons action functional is a map:

$$\mathsf{CS} : \mathsf{Conn}_G(M) \to U(1)$$

valued in the circle, not the real line — the action is only well-defined modulo $2\pi\mathbb{Z}$.

**Cohesive derivation.** The Chern-Simons form is constructed using the cohesive structure:
1. The Chern-Weil map gives $\mathsf{ch} : \mathbf{B}G_\nabla \to K(\mathbb{Z}, 4)$ (the classifying type for 4-dimensional integral classes)
2. Transgression via $\int_M : K(\mathbb{Z}, 4)^M \to K(\mathbb{Z}, 4-\dim M)$ gives the 3-dimensional class when $\dim M = 1$ ... (more precisely, when $M$ is 3-dimensional and the class is 4-dimensional on $M \times [-]$)

## TQFT and the Cobordism Hypothesis

A *topological quantum field theory* (TQFT) is a symmetric monoidal functor from the cobordism category to a category of vector spaces (or ∞-categorical generalization).

In the cohesive setting, TQFTs are naturally described using the shape modality: a TQFT is a natural transformation

$$Z : (M : \mathsf{Mfld}_n) \to \mathsf{Vect}$$

that only depends on the shape $\int M$.

**The cobordism hypothesis (Lurie).** The classification of fully extended TQFTs:

$$\text{Fully extended } n\text{-TFTs valued in } \mathcal{C} \simeq \text{dualizable objects in } \mathcal{C}$$

This is one of the deepest theorems in modern mathematics. In cohesive HoTT, the cobordism hypothesis would be:

$$\mathsf{Fun}^\otimes(\mathsf{Bord}_n, \mathcal{C}) \simeq \mathcal{C}^{\text{fd}}$$

where the left side is the ∞-groupoid of TFTs and the right is the ∞-groupoid of fully dualizable objects.

Formalizing this in simplicial type theory (Chapter 24) + cohesive HoTT is a major open problem.

## String Theory and Higher Gauge Theory

Higher gauge theories extend ordinary gauge theory by replacing 1-form connections with 2-form or 3-form connections. These appear naturally in string theory:

**B-field (string theory).** The Neveu-Schwarz B-field is a 2-form gauge field, described by a *2-bundle* (a principal bundle for a 2-group). In cohesive HoTT:

$$B \in \hat{H}^3(M, \mathbb{Z})$$

A 2-bundle with 2-connection is a map $M \to \mathbf{B}^2 U(1)_\nabla$.

**M5-brane (M-theory).** The C-field in M-theory is a 3-form gauge field, described by a 3-bundle. In cohesive HoTT:

$$C \in \hat{H}^4(M, \mathbb{Z})$$

**The Bianchi identity.** For a 2-bundle with connection, the field strength $H = dB$ satisfies $dH = \text{(correction terms)}$ — the Bianchi identity. In cohesive HoTT, this follows from the de Rham theorem applied to $\mathbf{B}^2 U(1)$.

The elegant feature: higher gauge fields in string theory are naturally classified by higher bundles, and in cohesive HoTT, higher bundles are maps to higher classifying types $\mathbf{B}^n G_\nabla$. No new machinery is needed beyond the basic cohesion axioms.

## Condensed Mathematics

*Condensed mathematics* (Scholze-Clausen 2019) is a new approach to algebraic geometry and functional analysis that replaces topological spaces with *condensed sets*: sheaves on the site of profinite sets.

The motivation: classical topological algebra is technically awkward. The category of topological abelian groups is not abelian. Completions and products don't behave well. Condensed mathematics fixes these issues by working with a different "topological" structure.

**Pyknotic objects.** The ∞-categorical version of condensed mathematics uses *pyknotic objects*: sheaves of ∞-groupoids on the site of compact Hausdorff spaces.

**Connection to cohesive HoTT.** The moduli of pyknotic objects forms an ∞-topos, and this ∞-topos has a cohesive structure. A tentative *pyknotic type theory* (Anel 2022) proposes:

- Modal types are the pyknotic (condensed) types
- The cohesion axioms encode the pyknotic structure
- The shape modality extracts the underlying homotopy type from a pyknotic space

This is an active research direction connecting cohesive HoTT to the most modern developments in algebraic geometry.

## Summary: Why Cohesive HoTT Matters for Physics

Traditional mathematical physics requires:
- Point-set topology (for manifolds)
- Differential geometry (for connections, curvature)
- Homological algebra (for characteristic classes)
- Category theory (for functoriality)
- ∞-Category theory (for extended TFTs)

Cohesive HoTT provides all of this synthetically, with:
- Types as cohesive spaces (replacing manifolds)
- The cohesion modalities capturing continuous/discrete/homotopy structure
- Principal bundles as maps to classifying types
- Connections via $\mathbf{B}G_\nabla$
- The de Rham theorem built in from the cohesion axioms
- Higher gauge theory naturally from higher classifying types

The payoff: physics computations that normally require coordinate patches and tensor indices become type-theoretic arguments using the modalities. The geometry is encoded in the type theory, not in explicit formulas.

Schreiber's program (*Differential cohomology in a cohesive ∞-topos*) aims to formalize all of this systematically. The current state: many results are verified, but full formalization in a proof assistant is ongoing work.

# Part II — Linear Algebra

## Chapters 4–12: Fields, Vector Spaces, Maps, Matrices, Determinants, Eigentheory, Inner Products, Tensors

* * *

No branch of mathematics has penetrated as deeply into as many other subjects — quantum mechanics, machine learning, differential geometry, signal processing, economics, coding theory, number theory — as linear algebra. The reason is not historical accident: linearity is the first-order approximation of almost everything. A curved surface looks flat at small scales. A nonlinear dynamical system behaves linearly near an equilibrium. A group representation linearizes the abstract algebra of symmetry so that the full toolkit of matrices and traces can be brought to bear. And the specific subject of this book — abstract algebra culminating in representation theory — is, at its core, linear algebra applied to the study of symmetry. To understand why representations work, one must understand what linear maps are and what they can do. Part II provides that understanding, developed from first principles in full generality, and it constitutes the permanent computational and conceptual background of everything that follows.

The abstraction begins immediately. The scalars in a vector space need not be the real or complex numbers; they may be any field — the rationals, a finite field $\mathbb{F}_p$, a number field, a function field. This generality is not pedantry: the representation theory of finite groups in characteristic $p$ (Part XI), Galois theory (Part VI), and the algebraic geometry lurking behind commutative algebra (Part IV) all require vector spaces over fields other than $\mathbb{R}$ or $\mathbb{C}$. Chapter 4 establishes the field axioms and then the vector space axioms — the ten conditions that together define what it means to have a "linear space" over a field — and gives the first catalogue of examples: $F^n$, polynomial spaces, function spaces, and spaces of matrices. Chapter 5 isolates the fundamental trio that makes computation possible: linear independence, spanning, and bases, the coordinate systems that allow every abstract vector space to be compared to $F^n$. The existence of a basis in every vector space — including infinite-dimensional ones — is not trivial; it requires Zorn's lemma, appearing here for the first time as an existence principle in a purely algebraic argument.

From bases the theory accelerates. Chapter 6 studies linear maps — the structure-preserving maps between vector spaces — and establishes the rank-nullity theorem, the fundamental constraint on what any linear map can compress or expand. Chapters 7 and 8 develop the computational machinery: matrices as coordinate representations of linear maps, matrix multiplication as composition, row reduction as the algorithmic form of linear algebra, and the determinant, defined axiomatically as the unique alternating multilinear form normalized to send the identity matrix to 1, then shown to detect invertibility and compute signed volume simultaneously. Chapters 9 and 10 attack the classification problem for a single linear operator: when can $T: V \to V$ be diagonalized, and when it cannot, what is the simplest form it can be placed in by a choice of basis? The answer involves eigenvalues, the characteristic polynomial, the minimal polynomial, and the two canonical forms — the Jordan normal form and the rational canonical form — that together give a complete invariant-theoretic account of what an operator "really is," independent of any basis.

The final two chapters extend the reach of the theory outward in different directions. Chapter 11 adds geometry to the algebraic structure: inner products and norms, orthogonality and the Gram–Schmidt algorithm, the spectral theorem asserting that every self-adjoint operator diagonalizes in an orthonormal basis, and the singular value decomposition, the canonical form for an arbitrary linear map between two inner-product spaces. Chapter 12 transcends linearity itself, studying maps that are linear in each of several arguments simultaneously: the multilinear maps that give rise to tensor products, the exterior algebra (in which the determinant and differential forms live), and the symmetric algebra (which is the polynomial ring in the basis vectors). These structures will reappear in every subsequent part — tensor products in module theory (Part V) and homological algebra (Part VIII), exterior algebra in the determinant and in differential geometry, the symmetric algebra in the structure of universal enveloping algebras (Part X). The investment made in Part II pays compound interest throughout the rest of this book.

* * *

### Internal Dependency Map

```
Ch 4 (Fields, Vector Spaces)
    │
    ▼
Ch 5 (Bases, Dimension)
    │
    ▼
Ch 6 (Linear Maps)
    │
    ├──► Ch 7 (Matrices) ──► Ch 8 (Determinants) ──► Ch 9 (Eigentheory)
    │                                                        │
    │                                                        ▼
    │                                                   Ch 10 (Canonical Forms)
    │
    ├──► Ch 11 (Inner Products, Spectral Theorem, SVD)
    │
    └──► Ch 12 (Tensors, Exterior Algebra, Symmetric Algebra)
```

* * *

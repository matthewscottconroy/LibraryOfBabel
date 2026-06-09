# Part II — Linear Algebra
## Chapters 4–12: Fields, Vector Spaces, Maps, Matrices, Determinants, Eigentheory, Inner Products, Tensors

---

### What This Part Establishes

Linear algebra is the geometry of vector spaces and the algebra of linear maps. It is the computational engine for the rest of the book, and its abstract formulation is the model for all algebraic structures that follow. This part establishes:

1. **The field axioms** — what scalars are and why they must be invertible
2. **Vector spaces** — the abstract notion that unifies $\mathbb{R}^n$, function spaces, polynomial rings, and matrix spaces
3. **Bases and dimension** — the coordinate systems that make computation possible
4. **Linear maps and matrices** — the structure-preserving maps and their coordinate representations
5. **Determinants** — the single number that captures invertibility and volume
6. **Eigentheory** — the invariant directions of a linear operator; the first classification problem
7. **Canonical forms** — the complete answer to "what does a matrix look like, up to basis change?"
8. **Inner products** — adding geometry (length and angle) to the algebraic structure
9. **Tensors and exterior algebra** — multilinear algebra, the language of modern geometry and physics

---

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
    └──► Ch 11 (Inner Products, Spectral Theorem, SVD)
    │
    └──► Ch 12 (Tensors, Exterior Algebra, Symmetric Algebra)
```

---

## Chapter 4 — Fields and Vector Spaces

**What it establishes:** The two foundational structures: the field of scalars and the vector space they act on.

### 4.1 Fields

**4.1.1 The Field Axioms: Addition and Multiplication**
A *field* $(F, +, \cdot)$ satisfies:$(F, +)$ is an abelian group with identity$0$;$(F \setminus \{0\}, \cdot)$ is an abelian group with identity$1$; multiplication distributes over addition. The key constraint distinguishing fields from rings: every nonzero element has a multiplicative inverse. This makes division always possible (by nonzero elements), which is what makes fields the right coefficient domain for linear algebra.

**4.1.2 Consequences of the Axioms: Zero Divisors, Uniqueness**
From the axioms alone: $0 \cdot a = 0$,$(-1) \cdot a = -a$, if$ab = 0$ then$a = 0$ or$b = 0$ (no zero divisors), the additive and multiplicative identities are unique, inverses are unique. These are derived, not assumed. The absence of zero divisors is what makes the cancellation law hold in a field.

**4.1.3 Examples: $\mathbb{Q}$,$\mathbb{R}$,$\mathbb{C}$,$\mathbb{F}_p$,$\mathbb{F}_{p^n}$**
The classical fields and the finite fields. $\mathbb{F}_p = \mathbb{Z}/p\mathbb{Z}$ is a field precisely because$p$ is prime (every nonzero element has a multiplicative inverse mod$p$). The existence and uniqueness (up to isomorphism) of$\mathbb{F}_{p^n}$ for every prime power is stated here and proved in Part VI.

### 4.2 Vector Spaces

**4.2.1 The Vector Space Axioms**
A *vector space* over $F$ is an abelian group$(V, +)$ with a scalar multiplication$F \times V \to V$ that is bilinear and compatible (associativity, unit). Eight axioms total, four for addition and four for scalar multiplication. The power of the definition: it captures both geometric vectors in$\mathbb{R}^n$ and abstract function spaces in a single framework.

**4.2.2 Elementary Consequences: $0 \cdot v = 0$,$(-1)v = -v$**
These are theorems, not axioms. The proof that the scalar zero times any vector gives the zero vector uses only the axioms. A student who understands why these must be proved (rather than assumed) understands the axiomatic method.

**4.2.3 Examples: $F^n$,$M_{m \times n}(F)$,$F[x]$, Function Spaces**
The tableau of examples, organized to show variety: tuples, matrices, polynomials of bounded degree, continuous functions, solutions to homogeneous linear ODEs. Crucially, the same theorem (proved once for abstract vector spaces) applies to all of them simultaneously.

**4.2.4 The Zero Space and Trivial Examples**
The zero space $\{0\}$ has dimension 0. Any field is a vector space over itself of dimension 1. These degenerate cases reveal how the definition handles extremes.

### 4.3 Subspaces

**4.3.1 The Subspace Test: Three Conditions**
A non-empty subset $W \subseteq V$ is a subspace iff:$\mathbf{0} \in W$; closed under addition; closed under scalar multiplication. Equivalently:$a\mathbf{u} + b\mathbf{v} \in W$ for all scalars$a, b$ and vectors$\mathbf{u}, \mathbf{v} \in W$. The test replaces verifying all eight axioms.

**4.3.2 Examples of Subspaces; Non-Examples**
Lines and planes through the origin in $\mathbb{R}^3$ are subspaces; lines and planes not through the origin are not. The solution set of a homogeneous linear system is always a subspace; the solution set of an inhomogeneous system is not (it is a coset). Distinguishing subspaces from cosets is important.

**4.3.3 The Intersection and Sum of Subspaces**
The *intersection* $U \cap W$ is always a subspace. The *union*$U \cup W$ is generally not. The *sum*$U + W = \{u + w \mid u \in U, w \in W\}$ is the smallest subspace containing both$U$ and$W$. The dimension formula$\dim(U + W) = \dim U + \dim W - \dim(U \cap W)$ is the vector-space analogue of inclusion-exclusion.

**4.3.4 Direct Sums: Internal and External**
$V = U \oplus W$ (*direct sum*) if$V = U + W$ and$U \cap W = \{0\}$. Equivalently: every$v \in V$ decomposes *uniquely* as$v = u + w$. The external direct sum$U \oplus W$ is$U \times W$ with componentwise operations. Both concepts are used constantly.

**4.3.5 The Modular Law for Subspaces**
For subspaces with $A \subseteq B$:$B \cap (A + C) = A + (B \cap C)$. This is the vector space analogue of the modular law for groups, rings, and lattices. It controls how sums and intersections interact.

---

## Chapter 5 — Bases, Dimension, and Coordinates

**What it establishes:** How to choose a reference frame; the dimension as the unique "size" of a vector space.

### 5.1 Linear Combinations and Span

**5.1.1 Linear Combinations; the Span of a Set**
A *linear combination* of $\{v_1, \ldots, v_k\}$ is$\sum a_i v_i$ with$a_i \in F$. The *span* is the set of all linear combinations:$\mathrm{span}(S)$ is the smallest subspace containing$S$. The span of the empty set is$\{0\}$.

**5.1.2 Span as the Smallest Subspace; Spanning Sets**
$\mathrm{span}(S)$ equals the intersection of all subspaces containing$S$ — the smallest such subspace.$S$ *spans*$V$ if$\mathrm{span}(S) = V$. Finding efficient spanning sets (with few elements) is the first step toward a basis.

**5.1.3 Reducing a Spanning Set**
Any finite spanning set contains a basis as a subset. The algorithm: repeatedly remove any vector that is a linear combination of the others. This terminates in a basis. The argument uses induction on the size of the spanning set.

### 5.2 Linear Independence

**5.2.1 The Definition: Only the Trivial Relation Holds**
$\{v_1, \ldots, v_k\}$ is *linearly independent* if the only solution to$\sum a_i v_i = 0$ is$a_1 = \cdots = a_k = 0$. Equivalently: no$v_i$ is in the span of the others. An infinite set is independent if every finite subset is.

**5.2.2 Detecting Dependence; the Dependence Lemma**
If $\{v_1, \ldots, v_k\}$ is dependent, then some$v_j$ is in the span of the preceding ones. This gives a way to remove one vector from a dependent set and preserve the span — the key lemma for the exchange argument.

**5.2.3 Maximal Independent Sets**
A maximal linearly independent set (one that cannot be extended while remaining independent) is a basis. This characterization, via Zorn's Lemma, proves every vector space has a basis.

### 5.3 Bases

**5.3.1 Four Equivalent Characterizations of a Basis**
$\mathcal{B}$ is a basis iff: (a) linearly independent and spans$V$; (b) maximal linearly independent set; (c) minimal spanning set; (d) every vector in$V$ has a unique expression as a linear combination of$\mathcal{B}$. Each characterization is useful in different contexts.

**5.3.2 The Existence of a Basis (via Zorn's Lemma)**
Every vector space has a basis. For finite-dimensional spaces, the proof is by induction. For infinite-dimensional spaces, Zorn's Lemma is required: the poset of independent sets has a maximal element (= basis) because every chain has an upper bound (take the union). This is the prototypical Zorn argument.

**5.3.3 Extending Independent Sets; Reducing Spanning Sets**
Every independent set extends to a basis. Every spanning set contains a basis. These allow flexible basis construction: start from whatever structure is convenient, then adjust.

**5.3.4 The Replacement (Exchange) Lemma**
If $\{v_1, \ldots, v_m\}$ spans$V$ and$\{u_1, \ldots, u_k\}$ is independent in$V$, then$k \leq m$. This is proved by a careful exchange argument (replace one$v_i$ by$u_1$, then another by$u_2$, etc., maintaining a spanning set at each step). The Replacement Lemma is the engine of the dimension theorem.

### 5.4 Dimension

**5.4.1 The Invariance of Dimension: All Bases Have the Same Size**
Any two bases of a vector space $V$ have the same cardinality (the *dimension*$\dim V$). Proof for finite case: if$|\mathcal{B}| = n$ and$|\mathcal{C}| = m$, the Replacement Lemma gives$m \leq n$ and$n \leq m$. For infinite-dimensional spaces, the argument uses cardinality theory.

**5.4.2 Dimension of Subspaces; the Dimension Formula for Sums**
$\dim W \leq \dim V$ for$W \subseteq V$, with equality iff$W = V$.$\dim(U + W) = \dim U + \dim W - \dim(U \cap W)$. These are the primary tools for computing dimensions.

**5.4.3 The Coordinate Isomorphism $V \cong F^n$**
Once a basis $\mathcal{B} = (b_1, \ldots, b_n)$ is chosen, the map$v \mapsto [v]_{\mathcal{B}}$ (coordinate vector) is an isomorphism$V \cong F^n$. This says: all$n$-dimensional vector spaces over$F$ are isomorphic. The isomorphism is basis-dependent — there is no canonical one.

**5.4.4 Finite vs. Infinite Dimension**
A finite-dimensional vector space has a finite basis. Infinite-dimensional spaces (e.g., $F[x]$,$\mathcal{C}([0,1])$) have infinite bases, but their dimensions are well-defined cardinalities. For analysis, Hilbert spaces use a different notion of "basis" (orthonormal basis in the topological sense); that is distinct from the algebraic (Hamel) basis used here.

---

## Chapter 6 — Linear Maps

**What it establishes:** The morphisms of vector spaces; the fundamental measurement theorems.

### 6.1 Linear Maps

**6.1.1 The Definition of Linearity; Basic Consequences**
$T: V \to W$ is *linear* if$T(au + bv) = aT(u) + bT(v)$ for all scalars$a, b$ and vectors$u, v$. Immediate consequences:$T(0) = 0$,$T(-v) = -T(v)$. A linear map is determined entirely by its values on a basis: once we know$T(b_1), \ldots, T(b_n)$, the map is determined on all of$V$.

**6.1.2 Examples: Differentiation, Integration, Projection, Rotation**
A roster of fundamental examples from calculus and geometry, all verified to satisfy the definition. Differentiation $D: F[x] \to F[x]$ is linear. Integration$I: \mathcal{C}([a,b]) \to \mathcal{C}([a,b])$ is linear. The projection onto a coordinate is linear. Rotation by a fixed angle is linear. Multiplication by a fixed matrix is linear.

**6.1.3 The Space of Linear Maps $\mathcal{L}(V,W)$**
The set of all linear maps $V \to W$ is itself a vector space under pointwise addition and scalar multiplication.$\dim \mathcal{L}(V,W) = (\dim V)(\dim W)$. When$V = W$: the algebra$\mathcal{L}(V,V) = \mathrm{End}(V)$ under composition; composition is not commutative.

### 6.2 Kernel and Image

**6.2.1 The Kernel as a Subspace; Injectivity Criterion**
$\ker T = \{v \in V \mid T(v) = 0\}$ is a subspace of$V$ (closed under addition and scalar multiplication because$T$ is linear).$T$ is injective iff$\ker T = \{0\}$.

**6.2.2 The Image as a Subspace; Surjectivity Criterion**
$\mathrm{im}\, T = \{T(v) \mid v \in V\}$ is a subspace of$W$.$T$ is surjective iff$\mathrm{im}\, T = W$.

**6.2.3 The Rank-Nullity Theorem**
$\dim(\ker T) + \dim(\mathrm{im}\, T) = \dim V$. This is the fundamental accounting identity for linear maps. Proof: extend a basis of$\ker T$ to a basis of$V$; the images of the new basis vectors form a basis of$\mathrm{im}\, T$.

**6.2.4 Consequences: Injectivity $\Leftrightarrow$ Surjectivity for Equal-Dimension Spaces**
If $\dim V = \dim W < \infty$ and$T: V \to W$ is linear:$T$ injective$\Leftrightarrow$ $T$ surjective$\Leftrightarrow$ $T$ bijective. This is the linear algebra analogue of the fact that an injection of a finite set to itself is a bijection. It fails for infinite-dimensional spaces.

### 6.3 Isomorphisms

**6.3.1 Bijective Linear Maps; the Inverse is Linear**
A *linear isomorphism* is a bijective linear map; its inverse is automatically linear. $V \cong W$ (isomorphic) if a linear isomorphism exists.

**6.3.2 Classifying Finite-Dimensional Vector Spaces up to Isomorphism**
Two finite-dimensional vector spaces over $F$ are isomorphic iff they have the same dimension. The classification is complete: dimension is the only invariant. This is the first "classification theorem" in the book — a complete list of all objects up to isomorphism.

**6.3.3 The Functor Perspective: $\mathcal{L}(V,W)$ Depends Contravariantly on$V$**
A brief forward-looking remark: composing on the left with a fixed $g: W \to X$ sends$\mathcal{L}(V,W) \to \mathcal{L}(V,X)$ covariantly in$W$; composing on the right with$f: U \to V$ sends$\mathcal{L}(V,W) \to \mathcal{L}(U,W)$ contravariantly in$V$. This is the algebraic origin of contravariance, previewing category theory.

---

## Chapter 7 — Matrices and Matrix Algebra

**What it establishes:** The coordinate representation of linear maps; the algebra of matrix manipulation.

### 7.1 Matrices as Arrays

**7.1.1 Rows, Columns, and Entries; Transpose**
A matrix $A \in M_{m \times n}(F)$ is a rectangular array with$m$ rows and$n$ columns. Entry$A_{ij}$ is in row$i$, column$j$. The *transpose*$A^T$ swaps rows and columns. Symmetric matrices ($A^T = A$) and skew-symmetric matrices ($A^T = -A$) are important subspaces.

**7.1.2 Matrix Addition and Scalar Multiplication**
Componentwise operations make $M_{m \times n}(F)$ a vector space of dimension$mn$. The standard basis consists of matrices$E_{ij}$ with a single 1 in position$(i,j)$ and 0 elsewhere.

**7.1.3 Symmetric, Skew-Symmetric, Diagonal, and Triangular Matrices**
Named matrix types that appear constantly. The subspace of symmetric matrices has dimension $n(n+1)/2$; skew-symmetric has dimension$n(n-1)/2$. These subspaces are complementary:$A = \frac{A+A^T}{2} + \frac{A-A^T}{2}$ (symmetric + skew-symmetric).

### 7.2 Matrix Multiplication

**7.2.1 The Row–Column Rule and Its Motivation**
$(AB)_{ij} = \sum_k A_{ik} B_{kj}$ — the dot product of row$i$ of$A$ with column$j$ of$B$. This definition is motivated by composition of linear maps in the next section; it is not arbitrary.

**7.2.2 Associativity but Not Commutativity**
$(AB)C = A(BC)$ — matrix multiplication is associative. But$AB \neq BA$ in general; even the sizes may not match. Counter-examples are exhibited. The failure of commutativity is the first algebraic "surprise" and motivates the study of non-commutative structures.

**7.2.3 The Identity Matrix; the Transpose and Multiplication**
$I_n A = A I_m = A$ for$A \in M_{m \times n}$.$(AB)^T = B^T A^T$ — transposing reverses the order. This reversal is a fundamental feature that recurs in group theory ($(ab)^{-1} = b^{-1}a^{-1}$), in ring theory, in dual maps, and in category theory.

### 7.3 The Matrix of a Linear Map

**7.3.1 Representing $T: V \to W$ by Choosing Bases**
Given ordered bases $\mathcal{B}$ of$V$ and$\mathcal{C}$ of$W$, the matrix$[T]_{\mathcal{B}}^{\mathcal{C}}$ has columns$[T(b_j)]_{\mathcal{C}}$. The fundamental equation:$[T(v)]_{\mathcal{C}} = [T]_{\mathcal{B}}^{\mathcal{C}} \cdot [v]_{\mathcal{B}}$.

**7.3.2 Matrix Multiplication = Composition of Maps**
$[T \circ S]_{\mathcal{A}}^{\mathcal{C}} = [T]_{\mathcal{B}}^{\mathcal{C}} \cdot [S]_{\mathcal{A}}^{\mathcal{B}}$. This is the derivation of the matrix multiplication rule: it was defined precisely to make this equation work.

**7.3.3 The Isomorphism $\mathcal{L}(V,W) \cong M_{m \times n}(F)$**
Once bases are fixed, every linear map corresponds to a unique matrix and vice versa. This isomorphism identifies abstract linear maps with concrete arrays, enabling computation.

### 7.4 Change of Basis

**7.4.1 Transition Matrices and Coordinate Transformations**
The *change-of-basis matrix* $P$ from basis$\mathcal{B}'$ to$\mathcal{B}$ has columns$[b'_j]_{\mathcal{B}}$. Coordinate transformation:$[v]_{\mathcal{B}} = P \cdot [v]_{\mathcal{B}'}$.

**7.4.2 Similarity: $A' = P^{-1}AP$**
Two matrices represent the same linear operator $T: V \to V$ in different bases iff they are *similar* ($A' = P^{-1}AP$ for invertible$P$). Similarity is an equivalence relation on$M_{n \times n}(F)$. The central question: find the "simplest" representative in each similarity class.

**7.4.3 Invariants of Similarity: Trace, Determinant, Characteristic Polynomial**
These quantities do not change under similarity: they are properties of the linear operator $T$, not of its matrix representation. Trace = sum of diagonal entries = sum of eigenvalues. Determinant = product of eigenvalues. Characteristic polynomial encodes all eigenvalues with multiplicity.

### 7.5 Row Reduction and Computation

**7.5.1 Elementary Row Operations and Row Echelon Form**
Three operations: swap rows; scale a row by a nonzero scalar; add a multiple of one row to another. These do not change the solution set of the associated linear system. Applying them systematically produces *row echelon form* (REF): zeros below each leading entry.

**7.5.2 The Unique Reduced Row Echelon Form**
*Reduced* REF (RREF) additionally has each leading entry equal to 1 and zero everywhere else in its column. The RREF of a matrix is unique (independent of the sequence of operations). This is not obvious and requires proof.

**7.5.3 Solving Linear Systems; Computing Rank and Null Space**
Row reduce the augmented matrix $[A \mid b]$ to RREF. Free variables correspond to columns without a leading 1. The null space is parametrized by free variables; its dimension equals the number of free variables (= nullity =$n - \mathrm{rank}$).

**7.5.4 Computing Matrix Inverses**
Row reduce $[A \mid I]$ to$[I \mid A^{-1}]$ (if$A$ is invertible). If the left block cannot be reduced to$I$, then$A$ is singular.

---

## Chapter 8 — Determinants

**What it establishes:** A canonical scalar invariant of a square matrix that detects invertibility and measures volume.

### 8.1 The Axiomatic Definition

**8.1.1 Multilinearity and the Alternating Property**
The determinant is *multilinear*: linear in each row (or column) separately. It is *alternating*: swapping two rows negates the value. From these two properties: if two rows are equal, $\det = 0$; adding a multiple of one row to another does not change$\det$; scaling a row scales$\det$ by the same factor.

**8.1.2 Normalization; Uniqueness of the Determinant**
With the normalization $\det(I) = 1$, the three properties uniquely determine the determinant. This axiomatic approach shows *why* the determinant has the properties it does, rather than treating them as lucky coincidences of the formula.

**8.1.3 Consequences: Zero Rows, Equal Rows, Row Operations**
Immediate from the axioms: any matrix with a zero row or two equal rows has determinant 0; the three types of row operations have specific effects on $\det$. These facts give the computational method.

### 8.2 Computation

**8.2.1 The Leibniz Formula: Sum over Permutations**
$\det(A) = \sum_{\sigma \in S_n} \mathrm{sgn}(\sigma) \prod_{i=1}^n A_{i,\sigma(i)}$. A sum of$n!$ terms, each a signed product of one entry from each row and column. Derives from multilinearity and alternation. For$n=2$:$ad - bc$; for$n = 3$: six terms (Sarrus's rule).

**8.2.2 The Sign of a Permutation; Even and Odd Permutations**
The *sign* $\mathrm{sgn}(\sigma) = (-1)^{N(\sigma)}$ where$N(\sigma)$ is the number of inversions. Even permutations (sign$+1$) form the alternating group$A_n$. The sign appears here first and recurs in exterior algebra.

**8.2.3 Cofactor Expansion along Any Row or Column**
$\det(A) = \sum_j A_{ij} (-1)^{i+j} M_{ij}$ for any fixed row$i$ (or column$j$). The signed minor$(-1)^{i+j}M_{ij}$ is the *cofactor*$C_{ij}$. Choose the row/column with the most zeros to minimize work.

**8.2.4 Row Reduction: The Efficient Method**
Track the effect of each row operation on $\det$; row-reduce to upper triangular$U$; then$\det = \pm \prod_i U_{ii}$. This is$O(n^3)$, vastly superior to Leibniz ($O(n!)$) for large$n$.

**8.2.5 Block Triangular Matrices**
$\det \begin{pmatrix} A & B \\ 0 & C \end{pmatrix} = \det(A) \det(C)$. Proof: row-reduce each block separately. Applies to block diagonal and block triangular matrices.

### 8.3 Properties

**8.3.1 Multiplicativity: $\det(AB) = \det(A)\det(B)$**
The most important property of the determinant. Proof: fix $B$; the map$A \mapsto \det(AB)/\det(B)$ satisfies the three axioms, hence equals$\det(A)$. Consequences:$\det(A^{-1}) = \det(A)^{-1}$;$\det(A^T) = \det(A)$; similar matrices have the same determinant.

**8.3.2 The Adjugate Matrix and the Formula for the Inverse**
The *adjugate* $\mathrm{adj}(A)_{ij} = C_{ji}$ (transpose of cofactor matrix).$A \cdot \mathrm{adj}(A) = \det(A) \cdot I$. When$\det(A) \neq 0$:$A^{-1} = \frac{1}{\det A} \mathrm{adj}(A)$. Elegant in theory; inefficient in practice for large matrices.

**8.3.3 Cramer's Rule**
For invertible $A$: the$j$-th component of the solution to$Ax = b$ is$x_j = \det(A_j)/\det(A)$ where$A_j$ has column$j$ replaced by$b$. Theoretically important; computationally superseded by row reduction. Used in proving smoothness of parametrizations and in implicit function arguments.

**8.3.4 Determinant as Signed Volume**
$|\det(A)|$ = the$n$-dimensional volume of the parallelepiped spanned by the columns of$A$.$\mathrm{sgn}(\det(A))$ = orientation. Under a linear map$T$: volumes scale by$|\det(T)|$. This is the geometric meaning behind the computational formula.

### 8.4 The Characteristic Polynomial

**8.4.1 Definition: $p_T(\lambda) = \det(\lambda I - A)$**
The *characteristic polynomial* of a linear operator $T$ (or matrix$A$). A degree-$n$ polynomial in$\lambda$.

**8.4.2 Degree, Leading Term, and Coefficients**
$p_T(\lambda) = \lambda^n - \mathrm{tr}(A)\lambda^{n-1} + \cdots + (-1)^n \det(A)$. The coefficient of$\lambda^{n-1}$ is$-\mathrm{tr}(A)$ (sum of diagonal entries = sum of eigenvalues). The constant term is$(-1)^n \det(A)$ (product of eigenvalues).

**8.4.3 Similarity Invariance; the Characteristic Polynomial as an Invariant of $T$**
$p_{P^{-1}AP}(\lambda) = p_A(\lambda)$. So$p_T$ is an invariant of the operator$T$, not of its matrix representation. This is the first of many "basis-independent" invariants.

---

## Chapter 9 — Eigentheory

**What it establishes:** The invariant directions of a linear operator and their use for diagonalization.

### 9.1 Eigenvalues and Eigenvectors

**9.1.1 Definition: $Tv = \lambda v$ for Nonzero$v$**
$\lambda$ is an *eigenvalue* of$T$ if there is a nonzero$v$ with$Tv = \lambda v$;$v$ is the *eigenvector*. Geometric meaning: eigenvectors are directions that$T$ preserves, merely scaling them by$\lambda$.

**9.1.2 Finding Eigenvalues: Roots of $p_T$**
$\lambda$ is an eigenvalue iff$T - \lambda I$ is not injective, iff$\det(\lambda I - A) = 0$, iff$\lambda$ is a root of$p_T$. Over$\mathbb{C}$:$p_T$ always splits into linear factors (Fundamental Theorem of Algebra), giving$n$ eigenvalues counted with multiplicity. Over$\mathbb{R}$: some eigenvalues may be complex.

**9.1.3 Eigenspaces and Their Dimensions**
The *eigenspace* $E_\lambda = \ker(T - \lambda I)$ is the subspace of all eigenvectors for$\lambda$ (plus 0). Its dimension is the *geometric multiplicity*$m_g(\lambda)$.

**9.1.4 Algebraic and Geometric Multiplicity**
The *algebraic multiplicity* $m_a(\lambda)$ is the multiplicity of$\lambda$ as a root of$p_T$. Always:$1 \leq m_g(\lambda) \leq m_a(\lambda)$. Diagonalizability requires$m_g = m_a$ for all eigenvalues.

### 9.2 Diagonalization

**9.2.1 When Is $T$ Diagonalizable? The Criterion**
$T$ is *diagonalizable* iff$V$ has a basis of eigenvectors of$T$, iff$m_g(\lambda) = m_a(\lambda)$ for every eigenvalue, iff$V = \bigoplus_\lambda E_\lambda$.

**9.2.2 The Diagonalization Algorithm**
(1) Find eigenvalues as roots of $p_T$; (2) For each eigenvalue, find a basis of$E_\lambda$ by row-reducing$(\lambda I - A)$; (3) If the total count equals$\dim V$, form$P$ (eigenvectors as columns) and$D$ (eigenvalues on diagonal); then$A = PDP^{-1}$.

**9.2.3 Sufficient Conditions: Distinct Eigenvalues**
If $p_T$ has$n$ distinct roots (all eigenvalues distinct), then$T$ is automatically diagonalizable. Eigenvectors for distinct eigenvalues are linearly independent.

**9.2.4 Examples of Non-Diagonalizable Operators**
The $2 \times 2$ nilpotent$\begin{pmatrix}0&1\\0&0\end{pmatrix}$ has$p_T(\lambda) = \lambda^2$ (double root at 0) but$\dim E_0 = 1 < 2$. Not diagonalizable. This is the prototypical failure case.

### 9.3 The Minimal Polynomial

**9.3.1 Definition and Existence**
The *minimal polynomial* $m_T$ is the monic polynomial of least degree with$m_T(T) = 0$. It divides any polynomial$f$ with$f(T) = 0$.

**9.3.2 The Cayley–Hamilton Theorem: $p_T(T) = 0$**
Every square matrix satisfies its own characteristic polynomial. So $m_T \mid p_T$. Proof: easily verified for Jordan blocks; extends to all matrices by similarity.

**9.3.3 Minimal Polynomial Divides Characteristic Polynomial**
$m_T \mid p_T$ and they have the same irreducible factors (same roots, possibly with different multiplicities). The minimal polynomial can be computed from the Jordan form.

**9.3.4 Diagonalizability via the Minimal Polynomial**
$T$ is diagonalizable iff$m_T$ has no repeated irreducible factors (equivalently,$m_T$ splits into distinct linear factors over the algebraic closure). This is the cleanest algebraic characterization.

---

## Chapter 10 — Canonical Forms

**What it establishes:** The complete solution to the classification of linear operators up to similarity.

### 10.1 Generalized Eigenvectors

**10.1.1 Failure of Diagonalization and What Goes Wrong**
When $m_g < m_a$, the eigenspace is too small. We need to find additional vectors in$\ker(T - \lambda I)^k$ for$k > 1$ — *generalized eigenvectors*.

**10.1.2 Generalized Eigenspaces: $J_\lambda = \ker(T - \lambda I)^n$**
The *generalized eigenspace* $J_\lambda$ is the eventual kernel of$(T - \lambda I)$. It is$T$-invariant and contains the true eigenspace$E_\lambda$.

**10.1.3 The Generalized Eigenspace Decomposition**
Over an algebraically closed field: $V = \bigoplus_\lambda J_\lambda$. This decomposition is by the primary decomposition theorem for modules over$F[x]$. Each$J_\lambda$ is$(T - \lambda I)$-nilpotent.

### 10.2 Jordan Normal Form

**10.2.1 Jordan Blocks: Definition and Structure**
The *Jordan block* $J_k(\lambda)$ is$k \times k$ with$\lambda$ on the diagonal and$1$'s on the superdiagonal. It represents the nilpotent operator shifted up, then scaled by$\lambda$.

**10.2.2 The Jordan Canonical Form Theorem**
Over an algebraically closed field, every $T$ is similar to a block-diagonal matrix with Jordan blocks. The form is unique up to permutation of blocks.

**10.2.3 Reading the Jordan Form: Block Sizes and Multiplicities**
Number of blocks with eigenvalue $\lambda$ =$m_g(\lambda)$. Sum of block sizes for$\lambda$ =$m_a(\lambda)$. Size of largest block = nilpotency index of$(T - \lambda I)$ on$J_\lambda$.

**10.2.4 Computing the Jordan Form: The Dimension Sequence Method**
Compute $\dim \ker(T - \lambda I)^k$ for$k = 1, 2, \ldots$ The differences in this sequence give the number of Jordan blocks of each size.

**10.2.5 Matrix Exponentials via Jordan Form**
$e^{tA}$ is computed block by block:$e^{t J_k(\lambda)} = e^{\lambda t} \begin{pmatrix} 1 & t & t^2/2! & \cdots \\ 0 & 1 & t & \cdots \\ & & \ddots & \end{pmatrix}$. Essential for solving linear ODEs$x' = Ax$.

### 10.3 Rational Canonical Form

**10.3.1 Companion Matrices; the Characteristic Polynomial of $C(f)$**
For $f(\lambda) = \lambda^k + a_{k-1}\lambda^{k-1} + \cdots + a_0$, the companion matrix$C(f)$ has$f$ as both its characteristic and minimal polynomial.

**10.3.2 The Rational Canonical Form Theorem (Over Any Field)**
Every $T$ is similar to a block-diagonal matrix with companion matrix blocks$C(f_1), C(f_2), \ldots, C(f_r)$ where$f_1 \mid f_2 \mid \cdots \mid f_r$. The$f_i$ are the *invariant factors* of$T$.

**10.3.3 Invariant Factors and Their Computation**
The invariant factors are determined by $T$'s minimal polynomial ($f_r = m_T$) and the structure of$(F[x], T)$ as an$F[x]$-module (formalized in Part V).

**10.3.4 Relation to Jordan Form over Algebraically Closed Fields**
Over $\mathbb{C}$: the two canonical forms are equivalent — each invariant factor$f_i$ factors into linear factors and decomposes into Jordan blocks. The rational form works over any field; the Jordan form requires algebraic closure.

---

## Chapter 11 — Inner Product Spaces

**What it establishes:** Geometry (length, angle, orthogonality) on a vector space; the spectral theorem; the SVD.

### 11.1 Inner Products

**11.1.1 Real Inner Products: Bilinearity, Symmetry, Positive Definiteness**
An *inner product* $\langle \cdot, \cdot \rangle: V \times V \to \mathbb{R}$ is bilinear, symmetric ($\langle u,v\rangle = \langle v,u\rangle$), and positive definite ($\langle v,v\rangle \geq 0$ with equality iff$v = 0$). The positive definiteness axiom is what makes the induced norm a genuine measure of distance.

**11.1.2 Complex (Hermitian) Inner Products: Sesquilinearity**
Over $\mathbb{C}$:$\langle \cdot, \cdot \rangle$ is *sesquilinear* (conjugate-linear in the first argument, linear in the second), *conjugate-symmetric* ($\langle u,v\rangle = \overline{\langle v,u\rangle}$), and positive definite. The conjugate-linearity is required to maintain positive definiteness over$\mathbb{C}$.

**11.1.3 The Norm and Distance Induced by an Inner Product**
$\|v\| = \sqrt{\langle v,v\rangle}$. The norm satisfies the triangle inequality (via Cauchy-Schwarz) and gives a metric$d(u,v) = \|u - v\|$.

**11.1.4 The Cauchy–Schwarz and Triangle Inequalities**
$|\langle u,v\rangle| \leq \|u\|\|v\|$, with equality iff$u,v$ are proportional. Proof: consider$\|u - tv\|^2 \geq 0$ and optimize over$t$. The triangle inequality follows. These are the most-used inequalities in analysis.

### 11.2 Orthogonality

**11.2.1 Orthogonal Vectors and Orthogonal Sets**
$u \perp v$ if$\langle u,v\rangle = 0$. An *orthogonal set* has all pairs perpendicular; *orthonormal* if additionally each vector has norm 1. An orthogonal set of nonzero vectors is linearly independent.

**11.2.2 Orthonormal Bases; Coordinates via Inner Products**
In an orthonormal basis $\{e_1, \ldots, e_n\}$:$v = \sum \langle v, e_i \rangle e_i$. Coordinates are inner products — no matrix inversion required. The coordinate change formula between two orthonormal bases is an orthogonal (or unitary) matrix.

**11.2.3 The Gram–Schmidt Orthogonalization Process**
Given any basis, produce an orthonormal basis by iterative projection and normalization: $u_k = v_k - \sum_{j<k} \langle v_k, e_j\rangle e_j$, then$e_k = u_k/\|u_k\|$. The span at each step is preserved.

**11.2.4 The QR Decomposition**
$A = QR$ where$Q$ has orthonormal columns and$R$ is upper triangular. Gram-Schmidt applied to the columns of$A$ yields this decomposition. Used in numerical linear algebra for solving least-squares problems.

**11.2.5 Orthogonal Complements and the Projection Theorem**
$W^\perp = \{v \mid v \perp w \text{ for all } w \in W\}$ is a subspace.$V = W \oplus W^\perp$: every$v$ decomposes uniquely as$v = w + w^\perp$. The component$w$ is the *orthogonal projection*$\mathrm{proj}_W v$ — the closest point in$W$ to$v$.

### 11.3 Adjoint Maps

**11.3.1 The Adjoint $T^*$: Definition via the Inner Product**
$T^*: W \to V$ is the unique map satisfying$\langle Tv, w\rangle_W = \langle v, T^*w\rangle_V$ for all$v \in V$,$w \in W$. Existence and uniqueness follow from the Riesz representation theorem.

**11.3.2 The Matrix of the Adjoint: Conjugate Transpose**
In orthonormal bases, $[T^*] = [T]^*$ (conjugate transpose over$\mathbb{C}$; ordinary transpose over$\mathbb{R}$). This is not just a formula; it explains why the conjugate transpose is the "correct" generalization of the transpose.

**11.3.3 Self-Adjoint, Skew-Adjoint, Unitary, and Normal Operators**
Self-adjoint ($T^* = T$): eigenvalues real; geometric analogue of symmetric matrices. Skew-adjoint ($T^* = -T$). Unitary/Orthogonal ($T^* = T^{-1}$): preserves inner products and lengths. Normal ($T^*T = TT^*$): the class for which the spectral theorem holds.

### 11.4 The Spectral Theorem

**11.4.1 Normal Operators Are Unitarily Diagonalizable (Complex)**
A complex operator is normal iff it has an orthonormal basis of eigenvectors. The diagonal entries (eigenvalues) are: real for self-adjoint, of modulus 1 for unitary, complex in general for normal.

**11.4.2 Self-Adjoint Operators Are Orthogonally Diagonalizable (Real)**
A real self-adjoint operator has only real eigenvalues and is diagonalized by an orthogonal matrix. This is the spectral theorem for real symmetric matrices.

**11.4.3 Positive Semidefinite Operators and Square Roots**
$T$ is *positive semidefinite* if$\langle Tv, v\rangle \geq 0$ for all$v$ — equivalently, all eigenvalues$\geq 0$. Every PSD operator has a unique PSD square root$T^{1/2}$ (also self-adjoint).

**11.4.4 Applications: PCA, Quantum Mechanics, the Laplacian**
The spectral theorem underlies principal component analysis (eigenvectors of the covariance matrix), quantum mechanics (observables are self-adjoint; eigenvalues are measurement outcomes), and Fourier analysis (the Laplacian is self-adjoint; its eigenfunctions are the trigonometric basis).

### 11.5 The Singular Value Decomposition

**11.5.1 Singular Values: Eigenvalues of $T^*T$**
The *singular values* of $T: V \to W$ are$\sigma_i = \sqrt{\lambda_i(T^*T)}$, the square roots of the eigenvalues of the PSD operator$T^*T$. Always non-negative.

**11.5.2 The SVD Theorem: $A = U\Sigma V^*$**
Every matrix $A \in M_{m \times n}$ decomposes as$A = U\Sigma V^*$ with$U \in M_{m \times m}$ unitary,$V \in M_{n \times n}$ unitary, and$\Sigma$ diagonal with singular values$\sigma_1 \geq \cdots \geq \sigma_r \geq 0$ on the diagonal. The column of$V$ are eigenvectors of$A^*A$; columns of$U$ are eigenvectors of$AA^*$.

**11.5.3 Rank, Pseudoinverse, and Least Squares via SVD**
$\mathrm{rank}(A) =$ number of nonzero singular values. Pseudoinverse:$A^+ = V\Sigma^+ U^*$ where$\Sigma^+$ inverts the nonzero singular values. Least-squares solution:$\hat{x} = A^+ b$.

**11.5.4 The Eckart–Young Theorem: Best Low-Rank Approximation**
The best rank-$k$ approximation to$A$ (in any unitarily-invariant norm) is$A_k = \sum_{i=1}^k \sigma_i u_i v_i^*$ (truncated SVD). This is the basis of lossy matrix compression and dimensionality reduction.

### 11.6 Bilinear and Quadratic Forms

**11.6.1 Symmetric and Skew-Symmetric Bilinear Forms**
A *bilinear form* $B: V \times V \to F$ is linear in each argument. Symmetric:$B(u,v) = B(v,u)$. Skew-symmetric:$B(u,v) = -B(v,u)$. Every bilinear form is the sum of a symmetric and a skew-symmetric part.

**11.6.2 Sylvester's Law of Inertia; Signature of a Form**
Over $\mathbb{R}$: every symmetric bilinear form can be diagonalized to a form with$+1$'s,$-1$'s, and$0$'s. The *signature*$(p, q, r)$ (counts of$+1$,$-1$,$0$ in any diagonalization) is a complete invariant — it doesn't depend on the choice of diagonalizing basis.

**11.6.3 Classification of Real Quadratic Forms**
A *quadratic form* $Q(v) = B(v,v)$ for a symmetric$B$. Classification by signature: positive definite ($r = q = 0$), positive semidefinite ($r > 0$,$q = 0$), indefinite, etc. Geometric meaning: the level sets$\{Q = c\}$ are ellipsoids, hyperboloids, or degenerate conics.

---

## Chapter 12 — Multilinear Algebra and Tensors

**What it establishes:** The machinery for multilinear maps; the exterior and symmetric algebras; the language of modern geometry and physics.

### 12.1 Dual Spaces

**12.1.1 The Dual Space $V^* = \mathcal{L}(V, F)$; Linear Functionals**
A *linear functional* on $V$ is a linear map$V \to F$. The *dual space*$V^*$ is the vector space of all linear functionals.$\dim V^* = \dim V$ when$V$ is finite-dimensional.

**12.1.2 Dual Bases; the Kronecker Delta**
For basis $\{e_i\}$ of$V$, the *dual basis*$\{e^i\}$ of$V^*$ satisfies$e^i(e_j) = \delta^i_j$. Every$f \in V^*$ is$f = \sum_i f(e_i) e^i$.

**12.1.3 The Double Dual and the Canonical Isomorphism $V \cong V^{**}$**
The canonical map $\iota: V \to V^{**}$ given by$\iota(v)(f) = f(v)$ is an isomorphism for finite-dimensional$V$. It does not depend on any basis choice — it is the first example of a *natural* isomorphism.

**12.1.4 The Dual (Transpose) of a Linear Map**
For $T: V \to W$, the *dual map*$T^*: W^* \to V^*$ is$(T^*f)(v) = f(Tv)$. The matrix of$T^*$ in dual bases equals the transpose of the matrix of$T$. This is the abstract version of the matrix transpose.

### 12.2 Tensor Products

**12.2.1 Bilinear Maps and the Universal Property of $V \otimes W$**
The *tensor product* $V \otimes W$ is characterized by: bilinear maps$V \times W \to U$ are in bijection with linear maps$V \otimes W \to U$. Bilinearity in two arguments becomes linearity in one argument. This is the "universal bilinear object."

**12.2.2 Existence: Constructing $V \otimes W$ via Quotients of Free Modules**
Construct $V \otimes W$ as the free vector space on$V \times W$ modulo the subspace generated by bilinearity relations. The image of$(v,w)$ is written$v \otimes w$.

**12.2.3 The Basis of $V \otimes W$; Dimension Formula**
$\{e_i \otimes f_j\}$ is a basis of$V \otimes W$.$\dim(V \otimes W) = (\dim V)(\dim W)$.

**12.2.4 Pure and Non-Pure Tensors**
Elements of the form $v \otimes w$ are *pure* (or *simple*) tensors. Not every element of$V \otimes W$ is pure — a general element is a sum of pure tensors. This is the source of entanglement in quantum mechanics.

**12.2.5 Key Isomorphisms: $V^* \otimes W \cong \mathcal{L}(V,W)$, Distributivity**
$V^* \otimes W \cong \mathcal{L}(V,W)$: linear maps are tensors.$(V \oplus U) \otimes W \cong (V \otimes W) \oplus (U \otimes W)$: tensor distributes over direct sum.

### 12.3 The Tensor Algebra

**12.3.1 The Tensor Algebra $T(V) = \bigoplus_{k \geq 0} V^{\otimes k}$**
The *tensor algebra* is the direct sum of all tensor powers: $T^0 V = F$,$T^1 V = V$,$T^2 V = V \otimes V$, etc. Multiplication is$\otimes$.

**12.3.2 Universal Property: $T(V)$ is the Free Associative Algebra on$V$**
Any linear map $V \to A$ to an associative algebra$A$ extends uniquely to an algebra map$T(V) \to A$. This is the sense in which$T(V)$ is "free": no relations are imposed.

**12.3.3 Graded Algebras; Homogeneous Components**
$T(V)$ is *graded*:$T^k V \cdot T^l V \subseteq T^{k+l} V$. The quotients$\bigwedge V$ and$\mathrm{Sym}(V)$ are also graded.

### 12.4 The Exterior Algebra

**12.4.1 Alternating Tensors and the Exterior Product $\wedge$**
The *exterior algebra* $\bigwedge V$ is$T(V)$ modulo the ideal generated by$v \otimes v$ for all$v \in V$. The image of$v_1 \otimes \cdots \otimes v_k$ is$v_1 \wedge \cdots \wedge v_k$. The key relation:$v \wedge v = 0$, hence$u \wedge v = -v \wedge u$.

**12.4.2 The Exterior Powers $\bigwedge^k V$ and Their Bases**
The $k$-th exterior power$\bigwedge^k V$ is spanned by$e_{i_1} \wedge \cdots \wedge e_{i_k}$ for increasing index sequences$i_1 < \cdots < i_k$.

**12.4.3 Dimension Formula: $\dim \bigwedge^k V = \binom{n}{k}$**
Follows from the basis count. In particular $\bigwedge^n V$ is 1-dimensional (the *determinant line*) and$\bigwedge^k V = 0$ for$k > n$.

**12.4.4 Linear Independence via Exterior Products**
$v_1, \ldots, v_k$ are linearly independent iff$v_1 \wedge \cdots \wedge v_k \neq 0$.

**12.4.5 The Determinant as the Top Exterior Power $\bigwedge^n T$**
A linear map $T: V \to V$ acts on$\bigwedge^n V$ (the 1-dimensional determinant line) by multiplication by a scalar — that scalar is$\det(T)$. This is the coordinate-free definition of the determinant.

**12.4.6 The Graded Skew-Commutative Algebra $\bigwedge V$**
$\alpha \wedge \beta = (-1)^{|\alpha||\beta|} \beta \wedge \alpha$ for homogeneous elements. Sections of$\bigwedge T^*M$ are differential forms on a manifold; the differential$d$ makes this a cochain complex (de Rham complex).

### 12.5 The Symmetric Algebra

**12.5.1 Symmetric Tensors; the Symmetric Product**
The *symmetric algebra* $\mathrm{Sym}(V)$ is$T(V)$ modulo the ideal generated by$u \otimes v - v \otimes u$. The symmetric product is commutative.

**12.5.2 $\mathrm{Sym}^k(V)$ and Its Basis; Dimension Formula**
$\mathrm{Sym}^k(V)$ has basis$e_{i_1} \cdots e_{i_k}$ for non-decreasing sequences.$\dim \mathrm{Sym}^k(V) = \binom{n+k-1}{k}$.

**12.5.3 The Symmetric Algebra $\mathrm{Sym}(V) \cong F[x_1, \ldots, x_n]$**
Choosing a basis $e_1, \ldots, e_n$ of$V$ identifies$\mathrm{Sym}(V)$ with the polynomial ring. This is the algebraic explanation for why "polynomials in$n$ variables" is the right object: it is the symmetric algebra of the$n$-dimensional coordinate space.

**12.5.4 Polarization: Symmetric Functions and Polynomial Functions**
Over a field of characteristic 0 or $> k$: every degree-$k$ polynomial function$V \to F$ arises from a unique element of$\mathrm{Sym}^k(V)$ (via polarization). This identifies symmetric tensors with polynomial functions.

### 12.6 Tensors in Coordinates

**12.6.1 Upper and Lower Indices; Contravariant and Covariant**
A tensor of type $(p,q)$ has$p$ upper (contravariant) indices and$q$ lower (covariant) indices. Components transform via the change-of-basis matrix for upper indices and its inverse-transpose for lower indices.

**12.6.2 The Einstein Summation Convention**
A repeated index (one upper, one lower) indicates summation. $T^i{}_j v^j = \sum_j T^i{}_j v^j$. This notational convention suppresses explicit sums and clarifies which indices are being contracted.

**12.6.3 Contraction and the Trace**
*Contraction* on indices $i, j$: sum$T^{i \ldots j \ldots} \mapsto \sum_i T^{i \ldots i \ldots}$. Contracting the two indices of a$(1,1)$-tensor gives the trace. Contraction reduces the tensor type by$(1,1)$.

**12.6.4 Change of Basis for Tensors**
Under $e_i \mapsto \tilde{e}_i = P^j_i e_j$: upper indices transform by$P^{-1}$, lower indices by$P$. The transformation laws for each type of tensor index follow from this.

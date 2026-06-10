# Section 9.1: Vector Spaces and Linear Maps

---

## Section Introduction

A vector space is not the arrows-in-3D-space of introductory physics. It is any collection of objects that can be added and scaled, subject to eight axioms. The generality is the point: functions, polynomials, differential operators, matrices, and tensors are all vectors in appropriate vector spaces. Understanding this abstraction is what makes linear algebra universally applicable.

---

## 9.1.1 Vector Spaces

**Definition**: A **vector space** over a field F (typically ℝ or ℂ) is a set V with:
- An operation **addition**: V × V → V, written **u** + **v**
- An operation **scalar multiplication**: F × V → V, written λ**v**

satisfying:

*Axioms*:
1. Commutativity of addition: **u** + **v** = **v** + **u**
2. Associativity of addition: (**u** + **v**) + **w** = **u** + (**v** + **w**)
3. Additive identity: ∃**0** ∈ V such that **v** + **0** = **v**
4. Additive inverse: ∀**v** ∈ V ∃(−**v**) such that **v** + (−**v**) = **0**
5. Associativity of scalar multiplication: λ(μ**v**) = (λμ)**v**
6. Distributivity over vectors: λ(**u** + **v**) = λ**u** + λ**v**
7. Distributivity over scalars: (λ + μ)**v** = λ**v** + μ**v**
8. Unit scalar: 1**v** = **v**

**Examples**:
- ℝⁿ with componentwise addition and scalar multiplication. This is the prototype.
- The set C([a,b]) of continuous functions on [a,b] with (f+g)(x) = f(x)+g(x) and (λf)(x) = λf(x). An infinite-dimensional vector space.
- The set P_n of polynomials of degree ≤ n. Dimension n+1.
- The set M_{m×n} of m×n matrices. Dimension mn.
- The set of solutions to a homogeneous linear ODE y'' + p(x)y' + q(x)y = 0. A 2-dimensional vector space.
- The tangent space T_pM at a point p on a manifold M. An n-dimensional vector space.

---

## 9.1.2 Subspaces, Basis, and Dimension

**Subspace**: A non-empty subset W ⊆ V is a **subspace** if it is closed under addition and scalar multiplication. Equivalently: λ**u** + μ**v** ∈ W for all **u**, **v** ∈ W and λ, μ ∈ F.

**Span**: The span of a set S ⊆ V is the set of all finite linear combinations of elements of S. It is the smallest subspace containing S.

**Linear independence**: Vectors **v**₁, ..., **vₖ** are **linearly independent** if λ₁**v**₁ + ⋯ + λₖ**vₖ** = **0** implies λ₁ = ⋯ = λₖ = 0.

**Basis**: A **basis** of V is a linearly independent set that spans V. Every basis has the same number of elements — the **dimension** dim(V).

**Theorem**: Any two bases of a finite-dimensional vector space have the same cardinality. (Proof: if B has m elements and B' has n elements, and each spans V, then m ≤ n and n ≤ m by the Steinitz exchange lemma.)

**Standard basis of ℝⁿ**: The vectors **e**₁ = (1,0,...,0), **e**₂ = (0,1,...,0), ..., **eₙ** = (0,...,0,1).

**Coordinates**: Given a basis {**e**₁, ..., **eₙ**}, every **v** ∈ V has a unique expression **v** = Σ vⁱ **eᵢ**. The numbers vⁱ are the **components** of **v** in this basis. We write them with upper indices for reasons that become clear in Section 9.4.

---

## 9.1.3 Linear Maps

**Definition**: A map T: V → W is **linear** if:
- T(**u** + **v**) = T**u** + T**v** (additivity)
- T(λ**v**) = λ(T**v**) (homogeneity)

Equivalently: T(λ**u** + μ**v**) = λT**u** + μT**v** for all **u**, **v** ∈ V and λ, μ ∈ F.

**Kernel and image**:
- ker(T) = {**v** ∈ V : T**v** = **0**} — a subspace of V.
- im(T) = {T**v** : **v** ∈ V} — a subspace of W.

**Rank-Nullity Theorem**: dim(ker T) + dim(im T) = dim(V). (Proof: choose a basis for ker T; extend to a basis for V; the images of the additional basis vectors form a basis for im T.)

**Theorem** (characterization of injectivity): T is injective iff ker(T) = {**0**}.

**Theorem** (characterization of bijectivity): If dim(V) = dim(W) < ∞, then T is injective iff T is surjective iff T is bijective.

**Isomorphism**: A bijective linear map. Two finite-dimensional vector spaces over F are isomorphic iff they have the same dimension. In particular, every n-dimensional real vector space is isomorphic to ℝⁿ — but non-canonically (the isomorphism requires choosing a basis).

The non-canonicity of this isomorphism is crucial for GR: the tangent space T_pM at a point is not *naturally* identified with ℝ⁴, only after choosing a coordinate system. The metric g_p provides a *canonical* isomorphism (raising/lowering indices) — but this isomorphism depends on the metric.

---

## 9.1.4 The Space of Linear Maps

The set L(V, W) of linear maps from V to W is itself a vector space (with (S+T)**v** = S**v** + T**v** and (λT)**v** = λ(T**v**)). Its dimension is dim(V)·dim(W).

The space L(V, V) of linear maps from V to itself (endomorphisms) forms an **algebra**: we can compose maps (in addition to adding and scaling). The composition T ∘ S is again linear.

**In GR**: The Riemann curvature tensor R: T_pM × T_pM → L(T_pM, T_pM) is a bilinear map from pairs of tangent vectors to endomorphisms of the tangent space. Its structure as a linear map is what allows us to compute how geodesics deviate from each other.

---

## References

- Axler, S. (2015). *Linear Algebra Done Right*, 3rd ed. Springer.  
  [The best abstract treatment; defines everything in terms of linear maps rather than matrices. Highly recommended.]
- Halmos, P.R. (1974). *Finite-Dimensional Vector Spaces*, 2nd ed. Springer.  
  [A classic; concise, abstract, and elegant. The physicist's linear algebra reference of choice for 50 years.]
- Lang, S. (2004). *Linear Algebra*, 3rd ed. Springer.  
  [Comprehensive treatment with full proofs and good coverage of the dual space and multilinear algebra.]

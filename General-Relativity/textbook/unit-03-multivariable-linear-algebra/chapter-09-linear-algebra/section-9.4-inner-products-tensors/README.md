# Section 9.4: Inner Products, Dual Spaces, and Tensors

---

## Section Introduction

This section bridges linear algebra and the tensor analysis of GR. The dual space, the metric, index notation, and the concept of a tensor are all introduced here. By the end, the reader should understand why GR uses both upper and lower indices, what it means to "raise" or "lower" an index, and how tensors generalize vectors and covectors.

This material is dense but foundational. Every calculation in GR relies on it.

---

## 9.4.1 The Dual Space

**Definition**: The **dual space** V* of a vector space V is the space of all linear functionals on V: L(V, ℝ). Elements of V* are called **covectors**, **1-forms**, or **linear functionals**.

If V has basis {**e**₁, ..., **eₙ**}, the **dual basis** {**e**¹, ..., **eⁿ**} ⊂ V* is defined by:

$$\mathbf{e}^i(\mathbf{e}_j) = \delta^i_j = \begin{cases} 1 & i = j \\ 0 & i \neq j \end{cases}$$

(Kronecker delta). Every covector α ∈ V* has a unique expansion α = αᵢ **eⁱ**, where αᵢ = α(**eᵢ**) ∈ ℝ.

**Natural pairing**: The evaluation map V* × V → ℝ given by (α, **v**) ↦ α(**v**) is a canonical bilinear form — it does not require a metric, just the algebraic structure.

**Upper and lower indices**: We write vector components with **upper** indices (vⁱ) and covector components with **lower** indices (αᵢ). The **Einstein summation convention**: a repeated index (once up, once down) implies summation: α(**v**) = αᵢ vⁱ = Σᵢ αᵢ vⁱ. This is the inner product of a covector with a vector.

**Why two kinds of objects?**: Vectors and covectors transform *differently* under a change of basis. If the basis {**eⱼ**} transforms by **e**'ₖ = Aⱼₖ **eⱼ**, then:
- Vector components transform **contravariantly**: v'ᵏ = (A⁻¹)ᵏⱼ vʲ
- Covector components transform **covariantly**: α'ₖ = Aⱼₖ αⱼ

The two transformation laws are inverse-transpose pairs. Their contraction αᵢ vⁱ is invariant (a scalar). This is why tensors have both upper (contravariant) and lower (covariant) indices.

---

## 9.4.2 Inner Products and the Metric

**Definition**: An **inner product** on V is a bilinear form g: V × V → ℝ that is:
- Symmetric: g(**u**, **v**) = g(**v**, **u**)
- Non-degenerate: g(**u**, **v**) = 0 for all **v** ∈ V implies **u** = **0**.

A **positive definite** inner product satisfies g(**v**, **v**) > 0 for **v** ≠ **0**. An indefinite inner product (the kind used in GR) satisfies non-degeneracy but allows g(**v**, **v**) to be positive, negative, or zero.

**Sylvester's Law of Inertia**: Every non-degenerate symmetric bilinear form on ℝⁿ can be diagonalized to the form diag(+1, ..., +1, −1, ..., −1). The number of +1's and −1's is called the **signature** and is an invariant of the form.

**The Minkowski metric**: The spacetime metric of special relativity is η = diag(−1, +1, +1, +1) (signature (−, +, +, +) = "mostly plus") or diag(+1, −1, −1, −1) (signature (+, −, −, −) = "mostly minus"). This book uses the mostly-plus convention. The metric is non-degenerate but indefinite — vectors **v** with η(**v**, **v**) = 0 are **null** (light-like), η(**v**, **v**) < 0 are **timelike**, and η(**v**, **v**) > 0 are **spacelike**.

**Raising and lowering indices**: The metric g provides a canonical isomorphism between V and V*:
- **Lowering**: vᵢ = gᵢⱼ vʲ (turn a vector vʲ into a covector vᵢ by contracting with the metric)
- **Raising**: vⁱ = gⁱʲ vⱼ (turn a covector vⱼ into a vector vⁱ by contracting with the inverse metric g^{ij}, where g^{ik} g_{kj} = δⁱⱼ)

This is the content of "lowering and raising indices" in GR — it is just the metric isomorphism between the tangent space and its dual.

---

## 9.4.3 Tensors

**Definition**: A tensor of type (r, s) on V is a multilinear map:

$$T: \underbrace{V^* \times \cdots \times V^*}_{r} \times \underbrace{V \times \cdots \times V}_{s} \to \mathbb{R}$$

It takes r covectors and s vectors and returns a real number, linearly in each argument.

**Examples**:
- (0, 0) tensor: a scalar (an element of ℝ).
- (1, 0) tensor: a vector (**v** acts on covectors by α ↦ α(**v**)). Equivalently, a vector is an element of V via the canonical inclusion V ↪ V**.
- (0, 1) tensor: a covector.
- (0, 2) tensor: a bilinear form, like the metric gᵢⱼ.
- (1, 1) tensor: a linear map T: V → V (acts on a vector to give a vector, or equivalently on a covector to give a covector).
- (2, 0) tensor: a contravariant bilinear form, like the inverse metric g^{ij}.

**Components**: In a basis, a (r, s) tensor has components T^{i₁...iᵣ}_{j₁...jₛ} = T(**e**^{i₁}, ..., **e**^{iᵣ}, **e**_{j₁}, ..., **e**_{jₛ}).

**Tensor product**: If S is (r, s) and T is (p, q), then S ⊗ T is (r+p, s+q):
$$(S \otimes T)(\alpha_1, \ldots, \beta_1, \ldots) = S(\alpha_1, \ldots) \cdot T(\beta_1, \ldots)$$

**Contraction**: Summing over one upper and one lower index: T^{i}_{i} = Σᵢ T^{i}_{i} produces a tensor of type (r−1, s−1). This is the trace operation.

---

## 9.4.4 The Metric Tensor and Index Gymnastics

The metric in GR is a (0,2) tensor gᵤᵥ. Its components satisfy:
- Symmetry: gᵤᵥ = gᵥᵤ
- Non-degeneracy: det(gᵤᵥ) ≠ 0 everywhere
- Lorentzian signature: (−, +, +, +)

The **inverse metric** g^{μν} satisfies g^{μα} g_{αν} = δ^μ_ν.

**Index gymnastics** (raising and lowering):
- Lower an index: Tᵤ_{νρ} = gᵤα T^α_{νρ}
- Raise an index: T^μ_{νρ} = g^{μα} Tα_{νρ}
- Raise both: T^{μν}_{ρ} = g^{μα} g^{νβ} T_{αβρ}

The operation is just the metric isomorphism applied to each index separately.

**The Riemann tensor**: R^α_{βγδ} is a (1, 3) tensor — it takes one covector and three vectors and returns a scalar. Lowering the first index: R_{αβγδ} = gᵅμ R^μ_{βγδ}. The symmetries of the Riemann tensor (antisymmetry in γδ, antisymmetry in αβ when both are lower, the Bianchi identity) are algebraic constraints on a (0,4) tensor.

---

## 9.4.5 The Levi-Civita Symbol and the Volume Form

The **Levi-Civita symbol** ε_{i₁i₂...iₙ} is defined by:
- ε_{12...n} = +1
- Completely antisymmetric in all indices: ε changes sign under any transposition.

In ℝ³: ε_{123} = ε_{231} = ε_{312} = 1 and ε_{132} = ε_{213} = ε_{321} = −1, all others 0.

The cross product **A** × **B** has components (**A** × **B**)ⁱ = εⁱʲᵏ Aⱼ Bₖ.

**The volume form** in curved spacetime is √(−g) ε_{μνρσ} dx^μ ∧ dx^ν ∧ dx^ρ ∧ dx^σ — a completely antisymmetric (0,4) tensor. This is the invariant measure on spacetime.

The determinant det(**A**) = εⁱ¹ⁱ²...ⁱⁿ A₁ᵢ₁ A₂ᵢ₂ ... Aₙᵢₙ — the determinant is the unique multilinear antisymmetric function of the columns of **A**, normalized to give 1 for the identity.

---

## References

- Halmos, P.R. (1974). *Finite-Dimensional Vector Spaces*, 2nd ed. Springer. [The dual space and bilinear forms; rigorous and concise.]
- Misner, C.W., Thorne, K.S., and Wheeler, J.A. (1973). *Gravitation*. W.H. Freeman. [Chapters 2–3 on vectors, 1-forms, and tensors in spacetime; the definitive physics introduction to tensor calculus.]
- Nakahara, M. (2003). *Geometry, Topology and Physics*, 2nd ed. IOP Publishing. [Chapter 2 on tensor algebra; mathematical precision with physical applications.]
- Schutz, B.F. (1980). *Geometrical Methods of Mathematical Physics*. Cambridge University Press. [Chapter 2 on vectors and 1-forms; an excellent bridge between linear algebra and GR.]

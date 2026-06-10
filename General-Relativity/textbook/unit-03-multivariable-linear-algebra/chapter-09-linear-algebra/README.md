# Chapter 9: Linear Algebra

---

## Chapter Introduction

Linear algebra is the study of vector spaces and linear maps between them. In physics, it is the study of the spaces in which fields and tensors live, and the transformations that relate different observers. In GR specifically, every concept of fundamental importance has a linear-algebraic core:

- The **metric tensor** gᵤᵥ is a bilinear map from the tangent space to ℝ.
- The **Riemann curvature tensor** R^α_{βγδ} is a multilinear map — it takes three vectors and returns a vector.
- A **coordinate transformation** between charts is a linear map between tangent spaces (its Jacobian).
- **Geodesics** are defined by a second-order ODE on the tangent bundle; their solutions are governed by the eigenstructure of the geodesic deviation equation.
- The **Einstein equations** Gᵤᵥ = 8πTᵤᵥ relate two symmetric tensors at each point of spacetime — a system of 10 symmetric linear equations in the trace-reversed metric perturbation (in linearized GR).

This chapter develops linear algebra with two goals in mind: to provide the algebraic tools needed for all subsequent material, and to establish the conceptual foundations for tensor analysis in Chapter 9's companion, Chapter 27 (Differential Geometry).

We proceed abstractly where it clarifies structure, and concretely where computation matters. The abstract setting (axioms for vector spaces, linear maps, dual spaces) reveals what is coordinate-independent; the concrete setting (matrices, determinants, Gaussian elimination) reveals what is computable.

A special emphasis is placed on **inner product spaces** and **the dual space**, because the distinction between a vector and a covector — between contravariant and covariant objects — is what makes tensor analysis work. Many students first encounter tensors with puzzlement about "upper and lower indices." By the end of this chapter, that puzzle should be resolved: upper indices belong to vectors (in the tangent space), lower indices to covectors (in the cotangent space), and contraction is the natural pairing between them.

---

## Sections in This Chapter

- [Section 9.1: Vector Spaces and Linear Maps](section-9.1-vector-spaces/README.md)
- [Section 9.2: Matrices, Determinants, and Systems of Equations](section-9.2-matrices/README.md)
- [Section 9.3: Eigenvalues, Eigenvectors, and Diagonalization](section-9.3-eigenvalues/README.md)
- [Section 9.4: Inner Products, Dual Spaces, and Tensors](section-9.4-inner-products-tensors/README.md)
- [Exercises](exercises.md)
- [Further Reading and References](further-reading.md)
- [Important Researchers](important-researchers.md)
- [Important Concepts](important-concepts.md)

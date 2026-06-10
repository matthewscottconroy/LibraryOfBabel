# Chapter 7: Multivariable Calculus

---

## Chapter Introduction

In single-variable calculus, the derivative of f: ℝ → ℝ at a point is a number: the slope of the tangent line. For a function f: ℝⁿ → ℝᵐ, the "derivative" at a point cannot be a number — the function takes a vector and returns a vector. The derivative must be a *linear map* from ℝⁿ to ℝᵐ: it is the best linear approximation to f near the given point. This linear map, written as a matrix in coordinates, is the **Jacobian**.

The transition from "derivative as number" to "derivative as linear map" is the central conceptual step of this chapter. It is also the seed of what the derivative becomes in differential geometry: the **pushforward** of vectors between tangent spaces on manifolds.

This chapter develops:
- Partial derivatives and directional derivatives
- The Jacobian and the total derivative (the "correct" generalization of f')
- The chain rule for multivariable functions
- The implicit function theorem in full generality
- Multiple integrals and change of variables
- The Hessian and second-order analysis

The material is technically demanding but conceptually natural: everything is either a direct generalization of single-variable calculus or a consequence of linear algebra. The two subjects interlock everywhere.

---

## A Note on Notation

In this chapter, we use **bold** for vectors: **x** = (x₁, ..., xₙ) ∈ ℝⁿ. Partial derivatives are written ∂f/∂xⁱ or f_{,i} (comma notation). The Einstein summation convention — repeated upper-and-lower indices are summed — will be introduced formally in Chapter 9; here we sum explicitly.

For a function f: ℝⁿ → ℝ, the gradient is the vector ∇f = (∂f/∂x₁, ..., ∂f/∂xₙ). For f: ℝⁿ → ℝᵐ, the Jacobian is the m×n matrix with (i,j) entry ∂fᵢ/∂xⱼ.

---

## Sections in This Chapter

- [Section 7.1: Partial Derivatives and Differentiability](section-7.1-partial-derivatives/README.md)
- [Section 7.2: The Chain Rule and Directional Derivatives](section-7.2-chain-rule/README.md)
- [Section 7.3: The Implicit Function Theorem](section-7.3-implicit-function-theorem/README.md)
- [Section 7.4: Multiple Integrals](section-7.4-multiple-integrals/README.md)
- [Exercises](exercises.md)
- [Further Reading and References](further-reading.md)
- [Important Researchers](important-researchers.md)
- [Important Concepts](important-concepts.md)

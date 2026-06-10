# Chapter 4: Differential Calculus

---

## Chapter Introduction

The derivative is the rate of change of a function. This is a familiar idea — velocity is the rate of change of position, acceleration is the rate of change of velocity, the rate of change of temperature with altitude determines atmospheric dynamics. What makes the derivative a mathematical object rather than a physical intuition is the concept of a **limit**: the instantaneous rate of change is defined as the limit of average rates of change over shorter and shorter intervals.

This chapter builds differential calculus from the ground up. We begin with limits — the most fundamental concept in analysis — and prove their basic properties rigorously. We then define the derivative, develop the rules for computing it, and explore its applications: optimization, approximation, the mean value theorem, and implicit differentiation.

The derivative is also a local linear approximation: near any point where f is differentiable, the function f behaves like a straight line. This idea — that smooth functions look linear at sufficiently small scales — is the seed of the concept of a tangent space that we will develop in differential geometry (Chapter 28). The tangent space at a point on a manifold generalizes the tangent line to a curve and the tangent plane to a surface; the covariant derivative in GR generalizes the derivative to curved spaces.

Get the derivative right here. It will never disappear from this book.

---

## A Note on Rigor and Physical Intuition

Physicists sometimes learn calculus in a "do it first, justify it later" style, computing derivatives by applying rules without worrying about whether those rules have been proved. This pragmatic approach works well in practice — the rules are correct, and they give right answers. But it leaves gaps in understanding.

In this book, we prove everything we use. When we compute d/dx (x²) = 2x, we will know *why* it is true — not because someone told us the power rule, but because we can derive it from the definition of the derivative. When we use the chain rule, we will know its proof and its hypotheses. This rigor pays dividends later: when we encounter the chain rule for functions on manifolds (Section 27.2), the generalization is transparent once you understand the original.

---

## Sections in This Chapter

- [Section 4.1: Limits and Continuity](section-4.1-limits-and-continuity/README.md)
- [Section 4.2: The Derivative](section-4.2-the-derivative/README.md)
- [Section 4.3: Applications of the Derivative](section-4.3-applications/README.md)
- [Section 4.4: Implicit Differentiation and Related Rates](section-4.4-implicit-differentiation/README.md)
- [Exercises](exercises.md)
- [Further Reading and References](further-reading.md)
- [Important Researchers](important-researchers.md)
- [Important Concepts](important-concepts.md)

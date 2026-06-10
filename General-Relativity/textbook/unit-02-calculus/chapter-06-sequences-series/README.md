# Chapter 6: Sequences, Series, and Approximation

---

## Chapter Introduction

A sequence is a list of numbers, one for each natural number: a₁, a₂, a₃, .... A series is the attempt to add them all up. Both ideas are fundamental to analysis — and more subtle than they appear.

The key question about a sequence is whether it *converges*: does it settle toward a definite limit, or does it wander forever? We already encountered this in Section 3.3, where Cauchy sequences of rationals were used to construct the real numbers. The Cauchy criterion — that a sequence converges iff its terms become arbitrarily close to each other — is the central tool for establishing convergence without knowing the limit in advance.

The key question about a series $\sum_{n=1}^\infty a_n$ is the same: does the sequence of partial sums $S_N = \sum_{n=1}^N a_n$ converge? This is harder than it looks. The series $\sum 1/n$ (the harmonic series) diverges to infinity, even though its terms approach 0. The series $\sum 1/n^2$ converges to π²/6 — a stunning fact discovered by Euler. And the series $\sum (-1)^n/n$ converges (conditionally) to -ln 2.

Power series — series of the form $\sum a_n x^n$ — are the bridge between series and analysis. Every analytic function (and this includes most functions in physics) can be represented as a power series on some interval, and that representation is computable, differentiable, and integrable term by term. Taylor series, Fourier series, and Laurent series are all power series or their generalizations.

This chapter is the foundation for:
- Chapter 10 (ODEs): power series solutions to differential equations
- Chapter 12 (Complex Analysis): analytic functions as convergent power series
- Chapter 27 (Differential Geometry): tensor fields expanded in local coordinates
- The perturbative expansions that appear throughout GR (post-Newtonian, post-Minkowskian, small-curvature expansions)

Convergence of series is subtle, and the subject rewards careful thought. A theme: interchanging limits (sum and derivative, sum and integral, limit of a sum vs. sum of limits) requires justification. When such interchanges are justified — uniform convergence is the key criterion — they are legitimate. When they are not, pathological results follow. The pathologies are not exotic — they appear regularly in physics, and knowing when to trust termwise operations is essential.

---

## Sections in This Chapter

- [Section 6.1: Sequences and Their Limits](section-6.1-sequences/README.md)
- [Section 6.2: Series and Convergence Tests](section-6.2-series/README.md)
- [Section 6.3: Power Series and Radius of Convergence](section-6.3-power-series/README.md)
- [Section 6.4: Uniform Convergence](section-6.4-uniform-convergence/README.md)
- [Exercises](exercises.md)
- [Further Reading and References](further-reading.md)
- [Important Researchers](important-researchers.md)
- [Important Concepts](important-concepts.md)

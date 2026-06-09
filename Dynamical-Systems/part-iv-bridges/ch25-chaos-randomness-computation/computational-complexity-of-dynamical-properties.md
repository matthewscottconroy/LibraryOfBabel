# 25.6 Computational Complexity of Dynamical Properties

Undecidability is the extreme case: problems that no algorithm can solve. But there is also a rich theory of *how hard* computable problems are. Within the computable problems about dynamical systems, some are polynomial time, some are $\#P$-hard, and some sit at unexpected points in the complexity hierarchy.

**Lyapunov Exponents:** Computing Lyapunov exponents exactly is $\#P$-hard in general (for piecewise linear maps with rational data). But for generic smooth systems, numerical algorithms converge.

The $\#P$-hardness result for Lyapunov exponents is perhaps surprising: you might think that iterating a piecewise linear map and measuring growth rates would be easy. The difficulty lies in "exactly" — computing the exact value of a Lyapunov exponent is equivalent to computing the permanent of a matrix (a $\#P$-complete problem), because the Lyapunov exponent is the limit of $\frac{1}{n}\log\|Df^n(x)\|$, and the norm of a product of matrices can involve exponentially many paths.

In practice, the Lyapunov exponent is approximated numerically, and for generic smooth systems the approximation converges quickly. The $\#P$-hardness applies to the worst-case inputs, not to the typical smooth case.

**Entropy:** Computing the topological entropy of a piecewise affine map on the interval is polynomial-time. For 2D piecewise affine maps, it is $\#P$-complete.

For 1D piecewise monotone maps, the topological entropy is $\log \lambda$ where $\lambda$ is the largest real root of the kneading invariant polynomial — a polynomial computable from the breakpoint structure. This is a polynomial-time computation.

For 2D maps, the topological entropy is related to the spectral radius of certain matrices derived from the Markov partition, but constructing the Markov partition can be exponentially complex. The $\#P$-completeness comes from the combinatorial explosion in the 2D case.

**Periodic Points:** Counting periodic points of period $n$ for polynomial maps is $\#P$-complete (related to counting roots of polynomial equations).

Counting periodic points of period exactly $n$ for a polynomial $f: \mathbb{C} \to \mathbb{C}$ requires counting solutions to $f^n(z) = z$, which is a polynomial equation of degree $d^n$ (where $d = \deg f$). Counting roots (with multiplicity) is $\#P$-complete in general (it subsumes counting paths in a graph, which is $\#P$-complete).

But notice: *counting* is hard, while *locating* periodic orbits numerically is often feasible. This is a general phenomenon in computational complexity — decision and search problems are often tractable while counting problems are hard.

The complexity landscape here has a lesson. The "easy" properties of dynamical systems — 1D entropy, qualitative behavior of generic orbits, topological transitivity in specific cases — are computable in polynomial time. The "hard" properties — 2D entropy, exact Lyapunov exponents, periodic point counts — involve exponentially many orbits and are $\#P$-hard. And the "impossible" properties — general transitivity, positive entropy for all inputs — are undecidable.

This hierarchy mirrors the phenomenology of chaos: easy to describe qualitatively, hard to compute quantitatively, impossible to predict in the long run. The computational complexity is an exact reflection of the intrinsic complexity of chaotic dynamics.

# Chapter 02: Complex Functions

Having established the algebra and geometry of the complex number system, we now ask what it means for a rule $f$ to assign complex numbers to complex numbers. The definition is formally identical to what you know from real analysis: $f$ is a function from a subset $D \subseteq \mathbb{C}$ to $\mathbb{C}$. But the two-dimensional character of $\mathbb{C}$ introduces phenomena with no real-variable analogue, most prominently the fact that certain natural complex functions are genuinely multivalued.

## Section 01: Functions of a Complex Variable

Any complex function $f : D \to \mathbb{C}$ can be decomposed as $f(x + iy) = u(x,y) + iv(x,y)$, where $u = \mathrm{Re}(f)$ and $v = \mathrm{Im}(f)$ are real-valued functions of two real variables. This decomposition allows complex functions to be studied using the tools of multivariable calculus, but it also obscures the specifically complex-analytic structure. A central theme of this chapter and the next is the question: which functions $f$ are more than just arbitrary pairs $(u, v)$ of real functions?

Familiar functions extend to the complex domain: polynomials, rational functions, and expressions built from algebraic operations are defined wherever the denominator is nonzero. More subtle are the transcendental functions — the exponential, logarithm, trigonometric functions — whose extensions to $\mathbb{C}$ require careful definition and raise the multivalued issue.

## Section 02: Limits and Continuity

The $\varepsilon$-$\delta$ definition of a limit carries over word-for-word: $\lim_{z \to z_0} f(z) = L$ means that for every $\varepsilon > 0$ there exists $\delta > 0$ such that $|f(z) - L| < \varepsilon$ whenever $0 < |z - z_0| < \delta$. Continuity is defined in the standard way, and the usual algebra of limits (sum, product, quotient, composition) holds.

The key difference from real analysis is directional. In $\mathbb{R}$, a limit requires agreement from the left and right; in $\mathbb{C}$, $z$ can approach $z_0$ along any path in the plane. This places much stronger constraints on limits and, as the next unit shows, makes complex differentiability far more demanding than real differentiability.

## Section 03: Branch Cuts and Multivalued Functions

The complex logarithm is defined by $\log z = \ln|z| + i\arg z$. Because $\arg z$ is determined only up to multiples of $2\pi$, this expression is multivalued: a given $z$ has infinitely many logarithms, differing by integer multiples of $2\pi i$.

To obtain a genuine single-valued function, we must restrict the domain by specifying a branch: a continuous choice of $\arg z$ on a domain from which a curve (the branch cut) has been removed. The principal branch uses $\mathrm{Arg}\, z \in (-\pi, \pi]$, with the branch cut along the negative real axis.

Branch cuts are not merely a bookkeeping device. They reflect a genuine topological fact: the complex plane minus the origin is not simply connected, so the argument function cannot be made continuous everywhere on it. Understanding branch cuts at this foundational level is essential for the later theory of analytic continuation and Riemann surfaces.

## Learning Objectives

After completing this chapter, a student should be able to:

- Decompose a complex function into its real and imaginary parts, and work with each component as a function of two real variables.
- Apply the $\varepsilon$-$\delta$ definition of a complex limit, and explain why limits in $\mathbb{C}$ are stronger than limits in $\mathbb{R}$.
- Define continuity for complex functions and verify continuity using the real-part decomposition.
- Explain the origin of multivalued behavior in the complex logarithm and square root.
- Define the principal branch of the logarithm, state where it is defined, and identify its branch cut.
- Extend the discussion to general power functions $z^\alpha$ and understand how the choice of branch affects the function's values.

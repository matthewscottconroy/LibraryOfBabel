# Chapter 03: Sequences and Limits

A sequence is the simplest kind of infinite process: a function from the natural numbers into $\mathbb{R}$, producing an ordered list of values $a_1, a_2, a_3, \ldots$ The central question is whether these values eventually cluster near some fixed real number — whether the sequence converges. This question, made precise by the epsilon-N definition of a limit, is the foundation for all of analysis. Derivatives are limits of difference quotients, integrals are limits of Riemann sums, and solutions to differential equations are often limits of approximation sequences. This chapter develops the theory of sequential convergence rigorously.

## The Epsilon-N Definition

The intuitive idea that $a_n \to L$ means "$a_n$ gets closer and closer to $L$" is too vague for mathematics. The precise definition is: for every positive tolerance $\varepsilon$, the terms $a_n$ eventually all lie within $\varepsilon$ of $L$. Formally:

$$\lim_{n \to \infty} a_n = L \quad \Leftrightarrow \quad \forall \varepsilon > 0,\ \exists N \in \mathbb{N},\ \forall n > N,\ |a_n - L| < \varepsilon.$$

The Archimedean Property from the previous chapter is what makes this definition effective: for any $\varepsilon > 0$, we can always find a natural number $N$ that works.

## Convergence and Divergence

Section 1 establishes the epsilon-N definition and works through its applications in detail. The algebraic limit theorems — limits of sums, products, and quotients — are proved from the definition, using the triangle inequality as the key technical tool.

Section 2 distinguishes convergent sequences from divergent ones and proves that convergent sequences are bounded. Sequences can diverge in two qualitatively different ways: by growing without bound (like $a_n = n$) or by oscillating without settling (like $a_n = (-1)^n$). The chapter treats both. The Squeeze Theorem — if $a_n \leq b_n \leq c_n$ and $a_n, c_n \to L$, then $b_n \to L$ — is a powerful tool derived here.

## Cauchy Sequences

A Cauchy sequence is one whose terms become mutually close regardless of whether they approach any particular limit:
$$\forall \varepsilon > 0,\ \exists N \in \mathbb{N},\ \forall m, n > N,\ |a_m - a_n| < \varepsilon.$$

Section 3 proves the Cauchy criterion: a sequence of real numbers converges if and only if it is Cauchy. This is a profound result because the Cauchy condition can be checked without knowing the limit in advance. In practice, one often knows a sequence should converge (physically or heuristically) and uses the Cauchy criterion to establish convergence rigorously. The proof relies on completeness: every Cauchy sequence is bounded and has a convergent subsequence (Bolzano-Weierstrass), and a Cauchy sequence with a convergent subsequence must itself converge to the same limit.

## The Monotone Convergence Theorem

Section 4 handles a particularly clean case: if a sequence is monotone (always increasing or always decreasing) and bounded, it must converge. Bounded monotone sequences are Cauchy (and the proof does not require Bolzano-Weierstrass), and their limits are given explicitly by the supremum or infimum of the range. This theorem is used constantly — the Picard iteration for ODEs produces a monotone sequence of approximations on appropriate function norms.

## Connection to the Rest of the Unit

The epsilon-N definition established in this chapter is the template for all subsequent limit definitions: the epsilon-delta definition of a function limit in Chapter 5 changes only the domain of the variable. The Cauchy criterion will reappear when completeness of function spaces is discussed. The Monotone Convergence Theorem feeds directly into the convergence tests for series in Chapter 4. Together, the four sections of this chapter provide the analytic machinery that the rest of the course takes for granted.

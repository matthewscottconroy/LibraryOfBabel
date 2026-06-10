# Section 13.4: Uniform Convergence and Function Spaces

---

## Section Introduction

A sequence of functions $f_n: [a,b]\to\mathbb{R}$ converges **pointwise** to $f$ if $f_n(x)\to f(x)$ for each $x$. But pointwise convergence preserves few properties: the limit of a sequence of continuous functions can be discontinuous; the limit of a sequence of integrable functions need not equal the integral of the limit; term-by-term differentiation of a pointwise-convergent series can fail. These failures motivate the stricter notion of **uniform convergence**.

$f_n$ converges **uniformly** to $f$ if: for every $\varepsilon > 0$ there exists $N$ such that $|f_n(x) - f(x)| < \varepsilon$ for all $x\in[a,b]$ and all $n > N$. The key word is "for all $x$" — the same $N$ works simultaneously for every point. Uniform convergence preserves continuity (the uniform limit of continuous functions is continuous), and justifies interchanging limits with integrals and (under additional hypotheses) with derivatives.

The Weierstrass $M$-test is the main tool for establishing uniform convergence of series: if $|f_n(x)|\leq M_n$ for all $x$ and $\sum M_n < \infty$, then $\sum f_n(x)$ converges uniformly. This is how one proves that power series converge uniformly on compact subsets of their disk of convergence.

**Function spaces** — infinite-dimensional vector spaces of functions — are the natural setting for analysis. The space $C([a,b])$ of continuous functions with the sup norm $\|f\|_\infty = \sup_x|f(x)|$ is a complete normed space (Banach space). The space $L^2([a,b])$ of square-integrable functions with the norm $\|f\|_2 = \sqrt{\int|f|^2}$ is a Hilbert space — the natural arena for Fourier analysis and quantum mechanics. Understanding convergence in these spaces is essential for all of analysis and for the functional analytic foundations of quantum field theory and perturbation theory in GR.

---

## Subsections

- [13.4.1: Pointwise vs. Uniform Convergence](13.4.1-convergence.md)
- [13.4.2: Properties Preserved by Uniform Convergence](13.4.2-properties.md)
- [13.4.3: The Weierstrass M-Test](13.4.3-mtest.md)
- [13.4.4: Normed Spaces and Banach Spaces](13.4.4-banach.md)
- [13.4.5: Hilbert Spaces and L² Spaces](13.4.5-hilbert.md)

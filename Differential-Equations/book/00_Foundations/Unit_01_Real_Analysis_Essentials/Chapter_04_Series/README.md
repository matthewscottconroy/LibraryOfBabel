# Chapter 04: Series

An infinite series is the attempt to add infinitely many numbers. The concept sounds paradoxical — how can a process that never ends produce a finite result? — but the resolution, given by the theory of partial sums and limits, is one of the great intellectual achievements of analysis. Series arise throughout differential equations: solutions are expressed as power series, forcing functions are decomposed into Fourier series, and the exponential function $e^{At}$ that drives linear systems is defined by a matrix series. This chapter develops the theory rigorously.

## Partial Sums and Convergence

The series $\sum_{k=1}^\infty a_k$ is defined through its **partial sums** $S_n = \sum_{k=1}^n a_k$. The series **converges** if the sequence $(S_n)$ converges, in which case the sum is $S = \lim_{n\to\infty} S_n$. The series **diverges** otherwise.

This definition converts questions about infinite sums into questions about limits of sequences — the territory of Chapter 3. The Cauchy criterion for sequences immediately gives a Cauchy criterion for series: $\sum a_k$ converges if and only if for every $\varepsilon > 0$ there exists $N$ such that $\left|\sum_{k=m}^n a_k\right| < \varepsilon$ for all $n \geq m > N$.

A necessary condition for convergence is that $a_k \to 0$. The **divergence test** (or $k$-th term test) states: if $a_k \not\to 0$, then $\sum a_k$ diverges. But the condition $a_k \to 0$ is not sufficient — the harmonic series $\sum 1/k$ diverges despite $1/k \to 0$.

## Convergence Tests

Section 1 develops the main convergence tests for series of positive terms: the comparison test (bound $a_k$ above by a convergent series or below by a divergent one), the limit comparison test (compare the ratio $a_k/b_k$ to a constant), the ratio test (examine $|a_{k+1}/a_k|$), the root test (examine $|a_k|^{1/k}$), and the integral test (compare $\sum a_k$ to $\int a(x)\,dx$). Each test has a domain of applicability; knowing which to apply and when is a practical skill developed through examples.

## Absolute vs. Conditional Convergence

Section 2 distinguishes two types of convergence. A series $\sum a_k$ converges **absolutely** if $\sum |a_k|$ converges, and **conditionally** if it converges but $\sum |a_k|$ diverges. The alternating series $\sum (-1)^{k+1}/k = 1 - 1/2 + 1/3 - \cdots$ converges conditionally to $\ln 2$ but not absolutely (since $\sum 1/k$ diverges). Absolute convergence is the "safe" kind: absolutely convergent series can be rearranged freely, while conditionally convergent series cannot. Riemann's rearrangement theorem states that a conditionally convergent series can be rearranged to converge to any real number, or to diverge. This has direct implications for when series manipulations in the theory of ODEs are valid.

## Power Series

Section 3 turns to power series $\sum_{k=0}^\infty c_k (x - a)^k$, which converge on an interval centered at $a$ of radius $R$, called the **radius of convergence**. The radius is determined by the Cauchy-Hadamard formula:
$$\frac{1}{R} = \limsup_{k\to\infty} |c_k|^{1/k}.$$
Within the interval of convergence, a power series defines a function that is infinitely differentiable, and its derivative and integral can be computed term by term. This is the basis for the method of series solutions to ordinary differential equations.

## Connection to the Rest of the Unit

The Monotone Convergence Theorem from Chapter 3 appears here in the comparison test: series with nonnegative terms have monotone partial sums. The geometric series formula, proved by direct computation, is the prototype convergent series against which all others are compared. Power series connect forward to Chapter 5 and 6 — power series define analytic functions that are continuous and infinitely differentiable — and to the series solution methods that will appear when the course turns to second-order linear ODEs with variable coefficients.

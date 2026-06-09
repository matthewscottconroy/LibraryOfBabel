# 2.3 Integration

The Lebesgue integral generalizes the Riemann integral in a fundamental way: instead of partitioning the domain (the $x$-axis), it partitions the range (the $y$-axis), and it measures the "size" of level sets using a measure rather than just counting lengths. This makes it far more powerful for taking limits, and limits are everything in analysis.

The construction proceeds in three steps, each one extending the previous:

## 2.3.1 The Lebesgue Integral

**Step 1: Simple Functions.**
A function $f = \sum_{i=1}^n a_i \mathbf{1}_{A_i}$ (with $A_i$ measurable, $a_i \geq 0$) is *simple*. Define $\int f\,d\mu = \sum_i a_i \mu(A_i)$.

This is the obvious definition: a simple function is just a linear combination of indicator functions, and its integral is the corresponding linear combination of the measures of the sets. The key point is that this definition doesn't depend on the particular representation — you need to verify that different ways of writing the same simple function give the same integral.

**Step 2: Nonneg Measurable Functions.**
For $f \geq 0$ measurable, define
$$\int f\,d\mu = \sup\left\{\int \varphi\,d\mu : 0 \leq \varphi \leq f,\; \varphi \text{ simple}\right\}.$$

Here we approximate $f$ from below by simple functions and take the supremum. This can be infinite, which is allowed. The key fact is that you can always find an increasing sequence of simple functions converging pointwise to $f$, and the integral is then the limit of the integrals of those simple functions.

**Step 3: Integrable Functions.**
Write $f = f^+ - f^-$ where $f^+ = \max(f, 0)$ and $f^- = \max(-f, 0)$. If both $\int f^+\,d\mu$ and $\int f^-\,d\mu$ are finite, define $\int f\,d\mu = \int f^+\,d\mu - \int f^-\,d\mu$.

**Definition 2.3.1.** $f$ is *integrable* (or $f \in L^1(\mu)$) if $\int |f|\,d\mu < \infty$.

## 2.3.2 Convergence Theorems

These are the core computational tools of analysis, and what makes the Lebesgue integral superior to the Riemann integral for purposes of analysis. The fundamental question: when does $\lim_n \int f_n = \int \lim_n f_n$?

The answer depends on what kind of convergence you have and what constraints you impose on the sequence:

**Theorem 2.3.2 (Monotone Convergence Theorem).** If $0 \leq f_1 \leq f_2 \leq \cdots$ are measurable and $f_n \to f$ pointwise, then $\int f\,d\mu = \lim_n \int f_n\,d\mu$.

If the sequence is increasing and nonneg, you can always pass the limit through the integral. No integrability hypothesis needed — both sides can be infinite.

**Theorem 2.3.3 (Fatou's Lemma).** If $f_n \geq 0$ measurable, then $\int \liminf_n f_n\,d\mu \leq \liminf_n \int f_n\,d\mu$.

Fatou's lemma doesn't require convergence at all — just nonnegativity. It says the integral of the liminf is at most the liminf of the integrals. The inequality goes the "right way" for lower bounds.

**Theorem 2.3.4 (Dominated Convergence Theorem).** If $f_n \to f$ pointwise $\mu$-a.e., and $|f_n| \leq g$ for some integrable $g$, then $f$ is integrable and $\int f\,d\mu = \lim_n \int f_n\,d\mu$.

This is the workhorse. If your sequence converges pointwise and is dominated by an integrable function, the limit passes through the integral. The "domination" condition is what tames the sequence and prevents mass from escaping to infinity.

What these theorems are really saying: the Lebesgue integral is continuous with respect to pointwise convergence, under mild conditions. The Riemann integral doesn't have this property — you can write down sequences of Riemann-integrable functions whose pointwise limit is not Riemann-integrable (take the indicator functions of increasingly dense rational sets).

**Application in Dynamics.** These theorems are invoked constantly. When proving that a sequence of invariant measures converges, or when passing a limit through an integral in an ergodic average, dominated convergence is the tool. Fatou's lemma gives the key inequality in the proof of the Birkhoff Ergodic Theorem. Get comfortable with all three; you'll use them without thinking about it.

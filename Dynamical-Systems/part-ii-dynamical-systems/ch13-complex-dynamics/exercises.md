# Exercises — Chapter 13

These problems move from the concrete (computing Julia sets for specific $c$ values) to the structural (understanding what Sullivan's theorem does and does not say, and why transcendental maps behave differently).

---

**Exercise 13.1.** Prove that the Julia set of $f_c(z) = z^2 + c$ is the boundary of the basin of attraction of $\infty$. Show that for $|c| > 2$, the Julia set is a Cantor set.

**Exercise 13.2.** For $c = 0$: $f_0(z) = z^2$. Compute the Julia set, filled Julia set, and Fatou set explicitly. Classify each Fatou component.

**Exercise 13.3.** For $c = -2$: $f_{-2}(z) = z^2 - 2$. Show the Julia set is the interval $[-2, 2] \subseteq \mathbb{R}$ and $f_{-2}$ is conjugate to the Chebyshev polynomial $T_2(\cos\theta) = \cos(2\theta)$ on $[-1,1]$. What is the topological entropy?

**Exercise 13.4.** (Mandelbrot) Show that if $|c| > 2$, then $c \notin \mathcal{M}$. (*Hint:* Show $|f_c^n(0)| \to \infty$.) Find the largest $r$ such that $\{|c| \leq r\} \subseteq \mathcal{M}$.

**Exercise 13.5.** Compute the Hausdorff dimension of the Julia set of $f_c$ for $c$ on the boundary of the main cardioid near $c = 0$ and near $c = -2$. (Use the Bowen formula: $\dim_H(\mathcal{J}(f_c)) = 1 + \lambda^2/(4 \log d) + O(\lambda^4)$ where $\lambda$ is the multiplier of the fixed point.)

**Exercise 13.6.** State Sullivan's No Wandering Domains theorem carefully. Why does the proof not apply to transcendental entire functions (like $e^z$)? (Indeed, wandering domains exist for $z \mapsto e^z$.)

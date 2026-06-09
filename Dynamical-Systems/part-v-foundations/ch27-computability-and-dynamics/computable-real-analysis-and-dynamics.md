# 27.4 Computable Real Analysis and Dynamics

Everything we've discussed so far operates over discrete spaces — Turing machine configurations, natural numbers, finite alphabets. But most dynamical systems live on the real line, or in $\mathbb{R}^n$, or on smooth manifolds. To ask computability questions about these systems, we need a notion of computable real number and computable real function. This is the subject of Type-2 computability (or computable analysis), developed by Klaus Weihrauch starting in the 1980s.

## 27.4.1 Type-2 Computability

The key idea is to represent real numbers by sequences of rational approximations, and to define computable functions as those that map computable representations to computable representations, uniformly.

**Definition 27.4.1 (Weihrauch, 2000).** A real number $x \in \mathbb{R}$ is *computable* if there is a Turing machine that, given $n \in \mathbb{N}$, outputs a rational $q_n$ with $|x - q_n| < 2^{-n}$.

A *computable function* $f: \mathbb{R} \to \mathbb{R}$ maps computable inputs to computable outputs uniformly in the approximation index.

This is the right definition because it makes computation on real numbers continuous — by necessity, not by assumption.

**Theorem 27.4.2 (Every Computable Function is Continuous).** If $f: [0,1] \to [0,1]$ is computable, it is uniformly continuous (and the modulus of continuity is computable).

The proof is surprisingly clean: if $f$ is computable, then to produce a $2^{-n}$-approximation to $f(x)$ you need only a $2^{-m}$-approximation to $x$ for some $m = m(n)$. This $m$ is computable (it's the modulus of continuity), and it witnesses uniform continuity.

**Corollary 27.4.3.** Discontinuous functions (step functions, indicator functions of non-open sets) are not computable. The indicator function of a non-computable set is not computable.

This corollary is worth pausing on. It means that the standard objects of real analysis — Heaviside functions, characteristic functions of Cantor sets, etc. — are simply not in the computable category. This isn't a limitation of our technology; it's a logical necessity. Computation over the reals forces continuity.

## 27.4.2 Degrees of Computability in Dynamics

With computable analysis in hand, we can classify dynamical systems — and their key invariants — according to their Turing degrees. The Mandelbrot set is a striking example.

**Definition 27.4.4 (Turing Degree).** The *Turing degree* of a set $A \subseteq \mathbb{N}$ is the equivalence class of $A$ under Turing reducibility ($A \leq_T B$ if $A$ is computable from $B$).

**Theorem 27.4.5.** For the quadratic family $f_c(z) = z^2 + c$:
- The Mandelbrot set boundary $\partial\mathcal{M}$ contains points of every Turing degree (it is "computationally complete")
- The Julia set $\mathcal{J}(f_c)$ for $c \in \partial\mathcal{M}$ can have arbitrarily high Turing degree

These are consequences of the Braverman-Yampolsky noncomputability results (Section 25.5).

Let that sink in: the boundary of the Mandelbrot set is so complex that for any uncomputable set $A \subseteq \mathbb{N}$ — no matter how far up the arithmetic hierarchy — there is a point on $\partial\mathcal{M}$ whose location encodes $A$. The Mandelbrot set is, in a precise sense, computationally universal. Its boundary is not "harder" than some problems and "easier" than others — it contains problems of every difficulty simultaneously.

This doesn't make numerical exploration of the Mandelbrot set pointless; it explains why the boundary looks so intricate. The infinite complexity is not just visual; it is logical.

In the next section, we use the arithmetic hierarchy to give a systematic classification of dynamical properties by their computational complexity.

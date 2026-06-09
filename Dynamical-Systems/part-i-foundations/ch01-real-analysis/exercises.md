# Chapter 1 — Exercises

These exercises build facility with the metric space framework. Several of them establish facts that will be used later without comment; a few are genuine research-adjacent problems that show how far the basic ideas reach.

---

**Exercise 1.1.** Let $(X, d)$ be a metric space. Show that $d': X \times X \to [0,\infty)$ defined by $d'(x,y) = d(x,y)/(1+d(x,y))$ is also a metric on $X$, and that $(X,d)$ and $(X,d')$ have the same open sets (i.e., the same topology).

**Exercise 1.2.** Prove that the intersection of a compact set and a closed set is compact.

**Exercise 1.3.** Let $f: X \to Y$ be a continuous bijection with $X$ compact and $Y$ Hausdorff. Prove that $f^{-1}$ is continuous.

**Exercise 1.4.** Show that $C([0,1])$ with the $L^1$ norm $\|f\|_1 = \int_0^1 |f(t)|\,dt$ is not complete. (*Hint:* Find a Cauchy sequence whose pointwise limit is not continuous.)

**Exercise 1.5.** (Baire) Let $X = \mathbb{R}$ with the usual metric. Show that $\mathbb{Q}$ is meager in $\mathbb{R}$ but that $\mathbb{R} \setminus \mathbb{Q}$ (the irrationals) is residual. Conclude that the irrationals are dense in $\mathbb{R}$.

**Exercise 1.6.** Let $H$ be a Hilbert space with orthonormal basis $\{e_n\}_{n=1}^\infty$. Define the *shift operator* $S: H \to H$ by $S(e_n) = e_{n+1}$. Show that $S$ is an isometry (preserves the norm) but is not unitary (not surjective). The *adjoint* $S^*$ satisfies $S^*(e_1) = 0$ and $S^*(e_n) = e_{n-1}$ for $n \geq 2$. Show that $S^*S = I$ but $SS^* \neq I$.

**Exercise 1.7.** (Contraction Mapping Theorem) Let $\lambda \in (0,1)$ and define $f: \mathbb{R} \to \mathbb{R}$ by $f(x) = \lambda x + (1-\lambda) x_0$ for a fixed $x_0$. Show that $f$ is a contraction with fixed point $x_0$. Generalize: show that if $g: [a,b] \to [a,b]$ is differentiable with $|g'(x)| \leq \lambda < 1$ for all $x \in [a,b]$, then $g$ has a unique fixed point.

**Exercise 1.8.** Prove the Arzelà-Ascoli Theorem for $K = [0,1]$: a sequence $(f_n)$ in $C([0,1])$ with $\sup_n \|f_n\|_\infty \leq M$ and uniform equicontinuity has a uniformly convergent subsequence.

**Exercise 1.9.** (Research Connection) The Collatz map $C: \mathbb{N} \to \mathbb{N}$ defined by $C(n) = 3n+1$ if $n$ odd and $C(n) = n/2$ if $n$ even extends to a map on the *2-adic integers* $\mathbb{Z}_2$ (the completion of $\mathbb{Z}$ under the 2-adic metric $d_2(m,n) = 2^{-v_2(m-n)}$ where $v_2(k)$ is the largest power of 2 dividing $k$). Show that $\mathbb{Z}_2$ is complete under $d_2$. What does the Collatz conjecture become in this 2-adic setting?

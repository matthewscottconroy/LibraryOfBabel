# Exercises — Chapter 9

These exercises develop the core skills of hyperbolic dynamics: symbolic coding, Markov partitions, shadowing, and the connections between geometry and entropy.

---

**Exercise 9.1.** Verify that the invariant Cantor set of the horseshoe map has Hausdorff dimension $< 2$. (Estimate the dimension using the contraction and expansion rates $\lambda, \mu$.)

**Exercise 9.2.** (Shadowing) For the doubling map $f(x) = 2x \pmod 1$, show that any $\delta$-pseudo-orbit is $\delta/(2-1)$-shadowed by a true orbit. (*Hint:* Solve the "shadow" equation $2x_{n+1} - x_{n+2} = 2e_n$ where $e_n$ are errors.)

**Exercise 9.3.** Compute a Markov partition for the Arnold cat map $f_A$ on ${\mathbb T}^2$ with $A = \begin{pmatrix} 2 & 1 \\ 1 & 1\end{pmatrix}$. (*Hint:* The partition consists of two rectangles aligned with the stable/unstable eigendirections of $A$.) Write down the transition matrix and compute the topological entropy.

**Exercise 9.4.** For the baker's map $B: [0,1]^2 \to [0,1]^2$ defined by $B(x,y) = (2x, y/2)$ for $x < 1/2$ and $B(x,y) = (2x-1, (y+1)/2)$ for $x \geq 1/2$: show it is Anosov, find its stable/unstable foliations, construct a Markov partition, and compute its entropy.

**Exercise 9.5.** (Structural Stability) Let $f: {\mathbb T}^2 \to {\mathbb T}^2$ be the Arnold cat map. Suppose $g$ is a small $C^1$-perturbation. Show that the periodic orbit structure of $g$ is the same as that of $f$ (same number of periodic orbits of each period) using structural stability.

**Exercise 9.6.** (Research Connection) The logistic map $f_\mu(x) = \mu x(1-x)$ for $\mu = 4$ is topologically conjugate to the tent map on $[0,1]$. The tent map has a Markov partition into $\{[0,1/2], [1/2,1]\}$. Construct the symbolic coding and compute the entropy. Is $f_4$ an Anosov map on a compact manifold? (What goes wrong?)

# 25.5 Computable Analysis and Dynamical Systems

The question "is the Mandelbrot set computable?" might sound like it's asking whether Mandelbrot pictures can be drawn. It's asking something deeper: is there an algorithm that, given a complex number $c$ and a tolerance $\varepsilon$, can determine whether $c$ is within $\varepsilon$ of the Mandelbrot set? For most of the complex plane, yes. But near the boundary — where the hard cases live — the answer is more subtle.

**Definition 25.5.1.** A real number $x \in [0,1]$ is *computable* if there is a Turing machine that, given $n$, outputs a rational $q$ with $|x - q| < 2^{-n}$.

**Definition 25.5.2.** A function $f: [0,1] \to [0,1]$ is *computable* if there is a Turing machine that, given a computable $x$ and $n$, outputs a rational $q$ with $|f(x) - q| < 2^{-n}$.

These are the definitions of computable analysis — the theory of computation over continuous objects. Every continuous function is a limit of computable approximations, and we track which limits are themselves computable.

For dynamical systems, the relevant question is: which invariant sets are computable? Can we compute the Julia set of a given polynomial? Can we compute the topological entropy? Can we decide whether the system is chaotic?

**Theorem 25.5.3 (Braverman-Yampolsky 2006, 2008).** For the quadratic family $f_c$:
- The Mandelbrot set $\mathcal{M}$ is computable as a subset of ${\mathbb C}$ (given $c$ and $\varepsilon$, one can decide if $c$ is within $\varepsilon$ of $\mathcal{M}$).
- Julia sets $\mathcal{J}(f_c)$ are in general NOT computable on $\partial\mathcal{M}$: there exist $c \in \partial\mathcal{M}$ for which no algorithm can approximate $\mathcal{J}(f_c)$.

The Mandelbrot set is computable because membership in $\mathcal{M}$ is determined by whether the orbit of 0 under $f_c$ stays bounded — a property that can be checked (with error) by finite computation. Given $c$, iterate $f_c$ starting from 0 and check whether the orbit stays within radius 2 (if it escapes radius 2, it escapes to infinity). This is a computable approximation of membership in $\mathcal{M}$.

But Julia sets on $\partial \mathcal{M}$ are different. The Julia set $\mathcal{J}(f_c)$ for $c \in \partial\mathcal{M}$ is the boundary between parameters where $f_c$ is hyperbolic (structurally stable) and where it is not. Near $\partial\mathcal{M}$, the Julia set can be infinitely complicated in a way that is not computably approximable.

Braverman and Yampolsky constructed explicit $c \in \partial \mathcal{M}$ for which the Julia set $\mathcal{J}(f_c)$ is not computable: given $\varepsilon > 0$, no algorithm can output a finite list of points that forms an $\varepsilon$-dense subset of $\mathcal{J}(f_c)$. The proof uses the undecidability of halting: the computability of $\mathcal{J}(f_c)$ is tied to whether $f_c$ has a certain type of critical point behavior, and determining this behavior reduces to the halting problem.

The non-computability of Julia sets on $\partial\mathcal{M}$ has a practical implication: fractal drawing programs that plot Julia sets for $c \in \partial\mathcal{M}$ are drawing something that is not fully determined by the algorithm. The pictures are approximations, but there is no algorithm that tells you how good the approximation is, or whether you've captured the essential features.

This connects to the broader theme: the most complex dynamical behavior — the boundary between order and chaos, the parameter values where the system transitions from regular to chaotic — is exactly where computational decidability breaks down. The undecidability is not a failure of our tools; it is a feature of the mathematical objects themselves.

# 37.2 Polynomial-Like Maps and Renormalization

One of the key structures in the Mandelbrot set is self-similarity: small copies of the Mandelbrot set appear inside itself, at every scale. This self-similarity is not accidental — it's a precise consequence of renormalization.

A *polynomial-like map* is a proper holomorphic map between Jordan domains that looks like a polynomial from a functional standpoint: it has a well-defined "filled Julia set" and degree. Douady and Hubbard introduced this concept in 1985, and it's the right framework for understanding the small copies in the Mandelbrot set.

**Definition 37.2.1 (Douady-Hubbard, 1985).** A *polynomial-like map of degree $d$* is a proper holomorphic map $f: U' \to U$ between Jordan domains with $U' \Subset U$ and degree $d$. Its *filled Julia set* is $K(f) = \bigcap_{n\geq 0} f^{-n}(\overline{U})$.

**Theorem 37.2.2 (Straightening Theorem — Douady-Hubbard).** Every polynomial-like map of degree $d$ is hybrid equivalent (quasiconformally conjugate on a neighborhood of $K(f)$) to a polynomial of degree $d$.

The Straightening Theorem says: every polynomial-like map is "essentially" a polynomial. You can quasiconformally deform it into a genuine polynomial without changing the filled Julia set. This is why the small copies of the Mandelbrot set in the Mandelbrot set actually look like the Mandelbrot set — they are parametrizing genuine polynomial-like maps.

**Definition 37.2.3.** The quadratic map $f_c(z) = z^2 + c$ is *renormalizable* at period $n$ if there exists $c' \in \mathcal{M}$ and polynomial-like maps $g: U' \to U$ with $g = f_c^n$ and $K(g)$ containing the critical point $0$.

The *renormalization operator* $\mathcal{R}$ maps $f_c$ to the straightening $g$ (viewed as a new quadratic map).

Renormalization is the key to understanding the Feigenbaum universality from Chapter 10. The period-doubling route to chaos arises because applying $\mathcal{R}$ at period 2, repeatedly, converges to a fixed point.

**Theorem 37.2.4 (Lyubich, 1997, 1999).** For infinitely renormalizable maps of bounded combinatorial type, the renormalization operator $\mathcal{R}$ has a unique fixed point in the space of polynomial-like maps (at each period). The convergence is exponential, explaining Feigenbaum universality.

The Feigenbaum constant $\delta \approx 4.669...$ is the ratio of the eigenvalue of the linearization of $\mathcal{R}$ at its fixed point. This is an operator-theoretic statement: the largest eigenvalue of $D\mathcal{R}$ at the fixed point is universal, independent of the specific family of maps. Lyubich's theorem makes this precise and proves it rigorously.

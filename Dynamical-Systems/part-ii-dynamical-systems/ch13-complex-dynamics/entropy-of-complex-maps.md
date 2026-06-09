# 13.7 Entropy of Complex Maps

After all the geometric richness of Julia sets and the Mandelbrot set, let us pause and ask the information-theoretic question: how much entropy does a complex map produce?

For polynomials, the answer is clean and connects directly to the degree. The Julia set is the seat of the chaos, and the topological entropy of the restriction to the Julia set is exactly $\log d$.

**Theorem 13.7.1.** For a polynomial of degree $d$:
$$h_{\text{top}}(f|_{\mathcal{J}(f)}) = \log d.$$

For $f_c$ with $c \in \mathcal{M}$: since the Julia set is connected and $f_c$ has degree 2, we get $h_{\text{top}}(f_c|_{\mathcal{K}(f_c)}) = \log 2$.

What this is saying is: the topological entropy of the chaotic part of a degree-$d$ polynomial is universally $\log d$, independent of $c$ — as long as we restrict to the filled Julia set. The parameter $c$ affects the *geometry* of the Julia set (and hence its Hausdorff dimension, which varies with $c$), but not its topological entropy. All connected quadratic Julia sets have the same entropy.

For real parameters $c$ (the restriction $f_\mu(x) = \mu x(1-x)$ from Chapter 11), the entropy does vary:

**Theorem 13.7.2 (Misiurewicz-Szlenk).** For the real quadratic family $f_\mu: x \mapsto \mu x(1-x)$ viewed as a complex map:
$$h_{\text{top}}(f_\mu)$$
is a monotone function of $\mu$ on $[0, 4]$.

The monotonicity of entropy as a function of the parameter is a deep result about the real quadratic family. It is equivalent to the statement that the critical point $0$ is never "more chaotic" for a smaller parameter than a larger one — a nontrivial combinatorial fact. The proof uses the external ray structure of the Mandelbrot set.

For the complex family $f_c$ as $c$ varies over all of $\mathbb{C}$, the entropy (measured for the full map on $\hat{\mathbb{C}}$, including the basin of $\infty$) is always $\log 2$. The interesting variation is in the *metric* entropy with respect to various invariant measures — a story that involves the theory of equilibrium states and is part of the broader thermodynamic formalism of Chapter 9.

The relationship between entropy, Hausdorff dimension, and the geometry of Julia sets is captured by the Bowen formula: for a hyperbolic polynomial, $\dim_H(\mathcal{J}(f)) = t_0$ where $t_0$ is the unique zero of the topological pressure function $t \mapsto P(t \cdot \phi)$ for $\phi = -\log |f'|$. This connects the fractal geometry of the Julia set to the thermodynamic formalism — a connection we will develop further in the notes.

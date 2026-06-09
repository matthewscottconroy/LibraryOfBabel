# 1.6 Banach and Hilbert Spaces

We now specialize metric spaces in a different direction: we add linear structure. Normed spaces are vector spaces equipped with a metric that's compatible with the algebra. Complete normed spaces — Banach spaces — are where the real action in functional analysis happens. And Hilbert spaces, which have the additional structure of an inner product, are the setting for spectral theory and quantum mechanics.

## 1.6.1 Normed Spaces and Banach Spaces

**Definition 1.6.1.** A *normed space* $(X, \|\cdot\|)$ is a vector space $X$ over $\mathbb{R}$ (or $\mathbb{C}$) equipped with a norm satisfying: (i) $\|x\| \geq 0$ with equality iff $x = 0$; (ii) $\|\alpha x\| = |\alpha| \|x\|$; (iii) $\|x + y\| \leq \|x\| + \|y\|$. A *Banach space* is a complete normed space.

The norm turns the vector space into a metric space via $d(x, y) = \|x - y\|$. The three norm axioms correspond to: the metric is positive (and separates points), the metric scales correctly under scalar multiplication, and the triangle inequality. Completeness then says: Cauchy sequences converge.

Here are the spaces you'll need to know:

**Examples 1.6.2.**
- $\ell^p = \{(a_n)_{n \geq 1} : \sum |a_n|^p < \infty\}$ with $\|(a_n)\|_p = (\sum |a_n|^p)^{1/p}$ for $1 \leq p < \infty$. These are the sequence spaces.
- $\ell^\infty = \{(a_n) : \sup_n |a_n| < \infty\}$ with the sup norm. Bounded sequences.
- $C(K)$ for compact $K$, with the uniform norm $\|f\|_\infty = \sup_K |f|$. This is the function space we've been working with.
- $L^p(\mu)$ for a measure space $(\Omega, \mathcal{F}, \mu)$ — functions with finite $p$-th moment. The measure theory for these comes in Chapter 2.

All of these are Banach spaces. They represent different "sizes" of functions: $L^1$ asks that the function be integrable, $L^2$ that its square be integrable, $L^\infty$ that the function be essentially bounded.

## 1.6.2 Hilbert Spaces

Hilbert spaces are Banach spaces with a richer structure: an inner product that generalizes the dot product from $\mathbb{R}^n$ to infinite dimensions. The inner product gives you angles and orthogonality, not just lengths and distances.

**Definition 1.6.3.** A *Hilbert space* $(H, \langle \cdot, \cdot \rangle)$ is a Banach space whose norm is induced by an inner product: $\|x\|^2 = \langle x, x \rangle$, where $\langle \cdot, \cdot \rangle: H \times H \to \mathbb{R}$ is bilinear, symmetric, and positive definite.

The inner product axioms encode the geometry of the space. Bilinearity means the product behaves linearly in each argument; symmetry means $\langle x, y \rangle = \langle y, x \rangle$; positive definiteness means $\langle x, x \rangle > 0$ for $x \neq 0$.

The fundamental inequality connecting inner products and norms is:

**Theorem 1.6.4 (Cauchy-Schwarz Inequality).** For any $x, y \in H$: $|\langle x, y \rangle| \leq \|x\| \cdot \|y\|$.

This is the abstract version of the geometric fact that the absolute value of the dot product is bounded by the product of lengths. It's used everywhere — for bounding integrals, for proving continuity of the inner product, for establishing duality.

One of the most elegant theorems in Hilbert space theory is the projection theorem, which generalizes the geometric idea of projecting a vector onto a subspace:

**Theorem 1.6.5 (Projection Theorem).** Let $H$ be a Hilbert space and $K \subseteq H$ a closed convex subset. For any $x \in H$, there exists a unique $\hat{x} \in K$ minimizing $\|x - k\|$ over $k \in K$. The map $x \mapsto \hat{x}$ is the *orthogonal projection* onto $K$.

In infinite dimensions, this requires proof — you can't just appeal to continuity arguments on bounded closed sets, since those aren't compact. The proof uses the parallelogram law, which is specific to Hilbert spaces.

The notion of a basis extends to infinite dimensions in two ways:

**Definition 1.6.6.** A *Schauder basis* for a Banach space $X$ is a sequence $(e_n)$ such that every $x \in X$ has a unique expansion $x = \sum_{n=1}^\infty a_n e_n$. An *orthonormal basis* for a Hilbert space $H$ is a Schauder basis with $\langle e_m, e_n \rangle = \delta_{mn}$.

The existence of an orthonormal basis (for separable Hilbert spaces) is a foundational fact. Every element decomposes as an infinite series with orthogonal components, and the Pythagorean theorem holds: $\|x\|^2 = \sum_n |\langle x, e_n \rangle|^2$ (Parseval's identity).

Two key examples:

**Examples 1.6.7.**
- $L^2([0,1])$ has the orthonormal basis $\{e^{2\pi i n t}\}_{n \in \mathbb{Z}}$ — the Fourier basis. The expansion $f = \sum_n \hat{f}(n) e^{2\pi int}$ is the Fourier series, and Parseval's identity says $\int |f|^2 = \sum_n |\hat{f}(n)|^2$.
- Every separable Hilbert space is isometrically isomorphic to $\ell^2$. In a precise sense, there is (up to isomorphism) only one separable Hilbert space.

**Application in Dynamics.** The Koopman operator of a measure-preserving transformation acts on $L^2(\mu)$ as a unitary operator. Its spectral theory — eigenvalues, spectral measures, cyclic vectors — governs mixing, ergodicity, and the recurrence structure of the system. We're building that theory in Chapter 5 and applying it in Chapters 6 and 7. The Hilbert space structure of $L^2(\mu)$ is what makes spectral analysis possible.

The next two sections are different in flavor: they're about general theorems that apply to all complete metric spaces or Banach spaces, and they produce some of the most surprising results in analysis.

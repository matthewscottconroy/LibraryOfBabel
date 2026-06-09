# Inner Product and Hilbert Spaces

Banach spaces provide a setting for limits and operator theory, but they lack the geometric richness of Euclidean space: angles, orthogonality, and projections. Hilbert spaces restore this geometry to infinite-dimensional analysis by adding an inner product. The resulting structure—a complete inner product space—enables Fourier series, orthogonal projections, and the Riesz representation theorem, which is the foundation of the variational approach to elliptic PDEs.

## Inner Products

**Definition.** An **inner product** on a real vector space $H$ is a map $\langle \cdot, \cdot \rangle: H \times H \to \mathbb{R}$ satisfying:

1. **Symmetry:** $\langle x, y \rangle = \langle y, x \rangle$.
2. **Linearity in the first argument:** $\langle ax + by, z \rangle = a\langle x, z\rangle + b\langle y, z\rangle$.
3. **Positive definiteness:** $\langle x, x \rangle \geq 0$, with equality iff $x = 0$.

The **induced norm** is $\|x\| = \sqrt{\langle x, x\rangle}$.

**Cauchy-Schwarz inequality:** $|\langle x, y\rangle| \leq \|x\|\|y\|$, with equality iff $x$ and $y$ are proportional. This follows from expanding $\|x - ty\|^2 \geq 0$ as a quadratic in $t$ and requiring the discriminant to be $\leq 0$.

**Polarization identity:** $\langle x, y\rangle = \frac{1}{4}(\|x+y\|^2 - \|x-y\|^2)$. This shows the inner product is determined by the norm.

**Parallelogram law:** $\|x+y\|^2 + \|x-y\|^2 = 2(\|x\|^2 + \|y\|^2)$. A normed space is an inner product space iff the parallelogram law holds.

## Hilbert Spaces

**Definition.** A **Hilbert space** is a real (or complex) inner product space that is complete with respect to the induced norm.

**Examples:**
- $\mathbb{R}^n$ with $\langle x, y\rangle = \sum_i x_i y_i$ (the Euclidean Hilbert space).
- $L^2(\Omega)$ with $\langle f, g\rangle = \int_\Omega fg \, dx$: complete by Riesz-Fischer.
- $H^k(\Omega) = W^{k,2}(\Omega)$ with $\langle u, v\rangle_{H^k} = \sum_{|\alpha|\leq k}\int D^\alpha u \, D^\alpha v$: complete.
- $\ell^2 = \{(a_n)_{n\geq 1} : \sum a_n^2 < \infty\}$ with $\langle a, b\rangle = \sum a_n b_n$: the sequence Hilbert space.

## Orthogonality and Projections

Two elements $x, y \in H$ are **orthogonal** ($x \perp y$) if $\langle x, y\rangle = 0$. A set $S \subset H$ is orthogonal if its elements are pairwise orthogonal; it is orthonormal if additionally $\|x\| = 1$ for all $x \in S$.

**Projection Theorem.** Let $M$ be a closed subspace of a Hilbert space $H$. For every $x \in H$, there exists a unique $m \in M$ minimizing $\|x - m\|$. This minimizer, the **orthogonal projection** $P_M(x)$, satisfies:

$$x - P_M(x) \perp M, \quad \text{i.e., } \langle x - P_M(x), m\rangle = 0 \text{ for all } m \in M.$$

**Proof.** Let $d = \inf_{m \in M}\|x - m\|$ and $(m_n)$ a minimizing sequence. By the parallelogram law applied to $(x - m_n)/2$ and $(x - m_k)/2$:

$$\left\|\frac{m_n - m_k}{2}\right\|^2 = \frac{\|x - m_n\|^2 + \|x-m_k\|^2}{2} - \left\|x - \frac{m_n+m_k}{2}\right\|^2 \leq \frac{\|x-m_n\|^2 + \|x-m_k\|^2}{2} - d^2 \to 0.$$

So $(m_n)$ is Cauchy, hence converges (by completeness) to some $m \in M$ (closed). Uniqueness: if $m, m'$ both minimize, apply parallelogram law to show $m = m'$. Orthogonality: differentiating $\|x - (m + t\eta)\|^2$ at $t = 0$ for any $\eta \in M$ gives $\langle x - m, \eta\rangle = 0$. $\square$

**Corollary (Orthogonal Decomposition).** $H = M \oplus M^\perp$ where $M^\perp = \{y \in H : y \perp M\}$: every $x \in H$ decomposes uniquely as $x = P_M(x) + (x - P_M(x))$ with $P_M(x) \in M$ and $(x - P_M(x)) \in M^\perp$.

## Orthonormal Bases and Fourier Expansions

A **complete orthonormal system** (or **orthonormal basis**) in $H$ is a countable orthonormal set $\{e_n\}_{n=1}^\infty$ whose linear span is dense in $H$.

**Theorem (Fourier series in Hilbert spaces).** If $\{e_n\}$ is a complete orthonormal system in $H$, then for every $x \in H$:

$$x = \sum_{n=1}^\infty \langle x, e_n\rangle e_n \quad \text{(convergence in } H\text{)}, \quad \|x\|^2 = \sum_{n=1}^\infty |\langle x, e_n\rangle|^2 \quad \text{(Parseval's identity)}.$$

Every separable Hilbert space is isometrically isomorphic to $\ell^2$: the map $x \mapsto (\langle x, e_n\rangle)_{n=1}^\infty$ is an isometric isomorphism $H \to \ell^2$.

**Example: $L^2([0,2\pi])$.** The functions $e_n(x) = e^{inx}/\sqrt{2\pi}$ for $n \in \mathbb{Z}$ form a complete orthonormal system (by the Stone-Weierstrass theorem and the density of trigonometric polynomials). The Fourier expansion is the classical Fourier series:

$$f = \sum_{n=-\infty}^\infty \hat{f}(n) e^{inx}/\sqrt{2\pi}, \quad \hat{f}(n) = \frac{1}{\sqrt{2\pi}}\int_0^{2\pi} f(x)e^{-inx} \, dx,$$

with $\sum_n |\hat{f}(n)|^2 = \|f\|_{L^2}^2$ (Parseval's identity).

## The Riesz Representation Theorem

**Theorem (Riesz Representation for Hilbert Spaces).** For every bounded linear functional $\Lambda: H \to \mathbb{R}$ on a Hilbert space $H$, there exists a unique $y \in H$ such that:

$$\Lambda(x) = \langle x, y\rangle \text{ for all } x \in H, \quad \text{and} \quad \|\Lambda\| = \|y\|.$$

**Proof.** Let $M = \ker \Lambda$. If $M = H$, take $y = 0$. Otherwise, $M$ is a closed proper subspace, and $M^\perp \neq \{0\}$. Pick $z \in M^\perp$ with $\Lambda(z) = 1$ (rescale). For any $x \in H$, write $x = (x - \Lambda(x)z) + \Lambda(x)z$; the first term is in $M$ (since $\Lambda(x - \Lambda(x)z) = \Lambda(x) - \Lambda(x)\Lambda(z) = 0$), so it is orthogonal to $z$. Thus $\langle x, z\rangle = \Lambda(x)\|z\|^2$, giving $\Lambda(x) = \langle x, z/\|z\|^2\rangle$. Set $y = z/\|z\|^2$. $\square$

The Riesz theorem identifies $H$ with its dual $H^*$: $H \cong H^*$ as Hilbert spaces. This self-duality of Hilbert spaces is one of their most important properties and is used constantly in the variational theory of PDEs.

## The Lax-Milgram Theorem

**Theorem (Lax-Milgram).** Let $H$ be a Hilbert space and $a: H \times H \to \mathbb{R}$ a bilinear form satisfying:
- **Boundedness:** $|a(u,v)| \leq M\|u\|\|v\|$ for some $M > 0$.
- **Coercivity:** $a(u,u) \geq \alpha\|u\|^2$ for some $\alpha > 0$.

Then for every bounded linear functional $F: H \to \mathbb{R}$, there exists a unique $u \in H$ with $a(u,v) = F(v)$ for all $v \in H$, and $\|u\| \leq \|F\|/\alpha$.

**Application: Poisson equation.** For $-\Delta u = f$ in $\Omega$ with $u = 0$ on $\partial\Omega$, the weak formulation is: find $u \in H^1_0(\Omega)$ such that $\int_\Omega \nabla u \cdot \nabla v = \int_\Omega fv$ for all $v \in H^1_0(\Omega)$. Here $a(u,v) = \int \nabla u \cdot \nabla v$ is bounded and coercive (the Poincaré inequality gives $a(u,u) = \|\nabla u\|_{L^2}^2 \geq c\|u\|_{H^1}^2$), and $F(v) = \int fv$ is bounded for $f \in L^2$. Lax-Milgram guarantees a unique $u \in H^1_0$.

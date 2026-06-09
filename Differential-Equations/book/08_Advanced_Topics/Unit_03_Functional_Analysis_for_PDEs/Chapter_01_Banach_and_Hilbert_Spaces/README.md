# Chapter 1: Banach and Hilbert Spaces

The proper setting for the functional-analytic approach to PDEs is infinite-dimensional linear algebra. The objects of study are function spaces—spaces whose elements are functions—equipped with norms or inner products that capture the appropriate notion of "size" or "distance" for the problem at hand. This chapter builds these spaces from the ground up: from normed spaces, to Banach spaces (normed and complete), to Hilbert spaces (with inner products), to bounded linear operators (the morphisms of the theory).

## Normed Vector Spaces

A **normed vector space** is a vector space $X$ over $\mathbb{R}$ (or $\mathbb{C}$) equipped with a **norm** $\|\cdot\|: X \to [0,\infty)$ satisfying:
1. $\|x\| = 0 \iff x = 0$.
2. $\|\lambda x\| = |\lambda| \|x\|$ for all scalars $\lambda$.
3. $\|x + y\| \leq \|x\| + \|y\|$ (triangle inequality).

Examples: $\mathbb{R}^n$ with the Euclidean norm; $C([a,b])$ with $\|f\|_\infty = \max_{[a,b]}|f|$; $L^p(\Omega)$ with $\|f\|_p = (\int |f|^p)^{1/p}$; $W^{k,p}(\Omega)$ (Sobolev spaces, developed in Chapter 2).

## Banach Spaces

A **Banach space** is a normed vector space that is complete: every Cauchy sequence converges. Completeness is essential for the applicability of fixed point theorems, the open mapping theorem, and many limit arguments in analysis.

**Examples.** $\mathbb{R}^n$ (with any norm), $L^p(\Omega)$ for $1 \leq p \leq \infty$ (by the Riesz-Fischer theorem), $C([a,b])$ with the sup norm, $W^{k,p}(\Omega)$ for $1 \leq p < \infty$ and $k \geq 0$.

**Counterexample.** $C([a,b])$ with the $L^1$ norm $\|f\|_1 = \int_a^b |f|$ is not complete: the sequence of continuous functions converging to the step function is Cauchy in $L^1$ but has no limit in $C([a,b])$.

## Hilbert Spaces

A **Hilbert space** is a Banach space whose norm is derived from an inner product $\langle \cdot, \cdot \rangle$: $\|x\|^2 = \langle x, x\rangle$. Inner products enable orthogonality, projection, and Fourier expansions.

**The parallelogram law** characterizes Hilbert norms among Banach norms: $\|x\|$ arises from an inner product if and only if $\|x+y\|^2 + \|x-y\|^2 = 2(\|x\|^2 + \|y\|^2)$ for all $x, y$.

**Examples.** $L^2(\Omega)$ with $\langle f, g \rangle = \int_\Omega fg$; $H^k(\Omega) = W^{k,2}(\Omega)$ with $\langle u, v\rangle_{H^k} = \sum_{|\alpha|\leq k}\int D^\alpha u \, D^\alpha v$.

## Key Theorems

**Riesz Representation.** For every bounded linear functional $\Lambda: H \to \mathbb{R}$ on a Hilbert space, there exists a unique $y \in H$ with $\Lambda(x) = \langle x, y\rangle$ and $\|\Lambda\| = \|y\|$. This theorem (identifying $H$ with its dual $H^*$) underlies the Lax-Milgram theorem and hence the variational existence theory for elliptic PDEs.

**Lax-Milgram.** If $a: H \times H \to \mathbb{R}$ is bilinear, bounded ($|a(u,v)| \leq C\|u\|\|v\|$), and coercive ($a(u,u) \geq \alpha\|u\|^2$ for some $\alpha > 0$), then for every bounded functional $F$, there exists unique $u \in H$ with $a(u,v) = F(v)$ for all $v$.

## Bounded Linear Operators

A linear map $T: X \to Y$ between normed spaces is **bounded** if $\|T\| = \sup_{\|x\|=1}\|Tx\| < \infty$. The space $B(X,Y)$ of bounded linear operators is itself a Banach space (if $Y$ is complete).

Bounded operators are the morphisms of the Banach space category. The three fundamental theorems (Uniform Boundedness, Open Mapping, Closed Graph) are the cornerstones of operator theory and have direct applications in PDE theory (the closed graph theorem implies that many natural operators in PDE are bounded).

## Chapter Structure

Section 1 (Normed Spaces): norms, completeness, denseness, and the Hahn-Banach theorem. Section 2 (Completeness and Banach Spaces): completeness, series in Banach spaces, and the three fundamental theorems. Section 3 (Inner Product and Hilbert Spaces): orthogonality, projections, orthonormal bases, and the Riesz representation theorem. Section 4 (Bounded Linear Operators): the operator norm, dual spaces, adjoints, and the Lax-Milgram theorem.

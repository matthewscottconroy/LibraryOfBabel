# Chapter 6 — Exercises

## Important Figures

- **Augustin-Louis Cauchy (1789–1857)** — systematized the theory of linear equations and linear transformations in matrix form
- **William Rowan Hamilton (1805–1865)** — introduced linear operators in the context of quaternions; early abstract treatment of maps between algebraic structures
- **Emmy Noether (1882–1935)** — reformulated linear algebra abstractly in terms of module homomorphisms; the rank-nullity theorem is a special case of the first isomorphism theorem for modules
- **Stefan Banach (1892–1945)** — extended linear map theory to infinite-dimensional normed spaces; bounded linear operators

## References and Primary Sources

- **S. Axler, *Linear Algebra Done Right* (4th ed., Springer, 2024)** — linear maps as the primary objects from the start
- **W. Greub, *Linear Algebra* (4th ed., Springer, 1975)** — thorough coordinate-free treatment
- **P. Lax, *Linear Algebra and its Applications* (2nd ed., Wiley, 2007)** — applications-oriented; connects to analysis and geometry

## Examples, Applications, and Thought Experiments

- **Rank-nullity theorem** — for $T: \mathbb{R}^3 \to \mathbb{R}^2$: if $\ker T = \operatorname{span}(e_3)$, then $\operatorname{rank}(T) = 2$; the constraint $\dim(\text{domain}) = \operatorname{rank}(T) + \operatorname{null}(T)$ is inescapable; "information lost" in the kernel equals "information lost" from the codomain
- **Differentiation as a linear map** — $d/dx: \mathbb{R}[x] \to \mathbb{R}[x]$ is linear; $\ker =$ constant polynomials; image $= \mathbb{R}[x]$; not injective but surjective; the derivative of $x^n$ is $n x^{n-1}$, a linear operation on the coefficient
- **Projection maps** — $\pi: \mathbb{R}^3 \to \mathbb{R}^2$, $(x,y,z) \mapsto (x,y)$: linear, surjective, kernel $=$ $z$-axis; the $z$-coordinate is "forgotten"; projection satisfies $\pi^2 = \pi$ — this idempotent property characterizes projections
- **The zero map and identity** — $0: V \to W$ and $\text{id}: V \to V$ are the initial and terminal objects in the category of linear maps from $V$; every linear map $T$ satisfies $T \circ 0 = 0$ and $T \circ \text{id} = T$; these play the role of $0$ and $1$ in the ring $\operatorname{End}(V)$

## Exercises

1. For each of the following, determine whether $T$ is a linear map. If it is, describe its kernel and image. If it is not, identify which condition of linearity it violates.
   - (a) $T: \mathbb{R}^2 \to \mathbb{R}^2$ defined by $T(x, y) = (x + y,\ xy)$
   - (b) $T: \mathbb{R}[x]_{\leq 3} \to \mathbb{R}[x]_{\leq 2}$ defined by $T(p) = p'$ (differentiation)
   - (c) $T: M_{2 \times 2}(\mathbb{R}) \to \mathbb{R}$ defined by $T(A) = \mathrm{tr}(A)$ (trace)

2. Let $T: \mathbb{R}^4 \to \mathbb{R}^3$ be the linear map defined by $T(x_1, x_2, x_3, x_4) = (x_1 + x_2,\ x_2 + x_3,\ x_3 + x_4)$. Find a basis for $\ker T$ and a basis for $\mathrm{im}\, T$. Verify that the rank-nullity theorem holds.

3. Suppose $T: V \to W$ is a linear map between finite-dimensional spaces with $\dim V = \dim W = n$.
   - (a) Prove that $T$ is injective if and only if $T$ is surjective.
   - (b) Give an example showing that (a) fails when $\dim V \neq \dim W$.
   - (c) Give an example showing that (a) fails for linear maps on infinite-dimensional spaces (e.g., on $\mathbb{R}[x]$).

4. Let $T: V \to W$ and $S: W \to X$ be linear maps. Prove that $\ker T \subseteq \ker(S \circ T)$ and $\mathrm{im}(S \circ T) \subseteq \mathrm{im}\, S$. Give an example where each inclusion is strict.

5. A linear map $P: V \to V$ is called a projection if $P^2 = P$. Prove that $V = \ker P \oplus \mathrm{im}\, P$. Then show that the kernel and image completely determine $P$: if $V = U \oplus W$ for subspaces $U$ and $W$, there is a unique projection $P$ with $\ker P = U$ and $\mathrm{im}\, P = W$.

6. Let $V$ and $W$ be vector spaces with $\dim V = m$ and $\dim W = n$. Show that $\dim \mathcal{L}(V, W) = mn$ by constructing an explicit basis for $\mathcal{L}(V, W)$. (Hint: use the matrices $E_{ij}$ that map the $j$-th basis vector of $V$ to the $i$-th basis vector of $W$ and everything else to zero.)

7. Let $T: V \to W$ be a linear map between finite-dimensional spaces. Prove that $\mathrm{rank}(T) \leq \min(\dim V, \dim W)$. Under what conditions on $T$ is each of $\mathrm{rank}(T) = \dim V$ and $\mathrm{rank}(T) = \dim W$ achievable?

8. (Challenge) Let $V$ be an $n$-dimensional vector space over $F$ and let $T: V \to V$ be a linear map satisfying $T^2 = 0$. Prove that $\mathrm{im}\, T \subseteq \ker T$, and therefore $\mathrm{rank}(T) \leq n/2$. Prove that this bound is sharp by constructing, for any $k \leq n/2$, a linear map $T: F^n \to F^n$ with $T^2 = 0$ and $\mathrm{rank}(T) = k$. (Maps satisfying $T^2 = 0$ are called nilpotent of order 2; they play a central role in Jordan theory.)

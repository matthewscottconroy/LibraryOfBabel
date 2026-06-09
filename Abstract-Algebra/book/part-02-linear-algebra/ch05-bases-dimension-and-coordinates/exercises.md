# Chapter 5 — Exercises

## Important Figures

- **Hermann Grassmann (1809–1877)** — developed the abstract notion of linear independence and spanning sets in *Ausdehnungslehre*
- **Ernst Steinitz (1871–1928)** — proved the Steinitz exchange lemma (1913), establishing that any two bases of a vector space have the same cardinality; the foundation for the uniqueness of dimension
- **Georg Hamel (1877–1954)** — proved (using AC) that every vector space, including infinite-dimensional ones, has a basis (a *Hamel basis*)
- **Augustin-Louis Cauchy (1789–1857)** — early work on linear independence in the context of systems of linear equations

## References and Primary Sources

- **E. Steinitz, "Bedingt konvergente Reihen und konvexe Systeme" (1913)** — exchange lemma and basis theory
- **S. Axler, *Linear Algebra Done Right* (4th ed., Springer, 2024)** — bases and dimension without determinants; particularly clean
- **P. Halmos, *Finite-Dimensional Vector Spaces* (1958)** — classic concise treatment
- **N. Jacobson, *Lectures in Abstract Algebra*, Vol. 2 (Van Nostrand, 1953)** — algebraic approach

## Examples, Applications, and Thought Experiments

- **Standard basis for $\mathbb{R}^3$** — $\{e_1, e_2, e_3\}$; any vector $(a,b,c) = a e_1 + b e_2 + c e_3$ uniquely; coordinates are the coefficients; the basis is an "address system" for the space
- **A non-standard basis for $\mathbb{R}^2$** — $\{(1,1),(1,-1)\}$; decompose $(3,1) = 2(1,1) + 1(1,-1)$; the coordinates are $(2,1)$ in this basis vs. $(3,1)$ in the standard basis; coordinates depend on the chosen reference frame
- **The space $\mathbb{R}[x]$** — has countably infinite dimension; the monomials $\{1, x, x^2, x^3, \ldots\}$ form a basis; every polynomial is a finite linear combination, so this is a legitimate (Hamel) basis; contrast with an orthonormal basis in a Hilbert space (which allows infinite series)
- **Dimension over different fields** — $\mathbb{C}$ as a $\mathbb{C}$-vector space has dimension 1; as an $\mathbb{R}$-vector space, dimension 2; as a $\mathbb{Q}$-vector space, dimension $\infty$; changing the scalars changes the dimension

## Exercises

1. Determine whether each of the following sets is linearly independent over $\mathbb{R}$. If it is dependent, find an explicit dependence relation (a nontrivial linear combination equal to zero).
   - (a) $\{(1, 2, 0),\ (0, 1, 3),\ (1, 3, 3)\}$ in $\mathbb{R}^3$
   - (b) $\{1 + x,\ x + x^2,\ 1 + x^2\}$ in $\mathbb{R}[x]_{\leq 2}$
   - (c) $\{e^x,\ e^{2x},\ e^{3x}\}$ viewed as functions in $C(\mathbb{R})$

2. Let $V = \mathbb{R}^4$ and let $W = \mathrm{span}\{(1,2,0,1),\, (0,1,1,0),\, (1,3,1,1),\, (2,5,1,2)\}$. Find a basis for $W$ by identifying which vectors in the spanning set are redundant. State $\dim W$.

3. Let $V$ be a vector space over $F$ with $\dim V = n$. Prove each of the following:
   - (a) Any set of more than $n$ vectors in $V$ is linearly dependent.
   - (b) Any linearly independent set of exactly $n$ vectors is a basis.
   - (c) Any spanning set of exactly $n$ vectors is a basis.
   (Parts (b) and (c) say that for sets of exactly the right size, independence and spanning each imply the other.)

4. Let $W_1 = \mathrm{span}\{(1,0,1,0),\, (0,1,0,1)\}$ and $W_2 = \mathrm{span}\{(1,1,0,0),\, (0,0,1,1)\}$ be subspaces of $\mathbb{R}^4$. Compute $\dim W_1$, $\dim W_2$, $\dim(W_1 \cap W_2)$, and $\dim(W_1 + W_2)$. Verify the dimension formula $\dim(W_1 + W_2) = \dim W_1 + \dim W_2 - \dim(W_1 \cap W_2)$.

5. Let $V = \mathbb{R}[x]_{\leq 3}$ (polynomials of degree at most 3). Show that the set $\mathcal{B} = \{1,\, x-1,\, (x-1)^2,\, (x-1)^3\}$ is a basis for $V$. Then find the coordinate vector of $p(x) = 2x^3 - x + 4$ with respect to $\mathcal{B}$.

6. Suppose $V$ and $W$ are both finite-dimensional vector spaces over the same field $F$, with $\dim V = \dim W$. Is it necessarily true that $V \cong W$? Prove your answer. (Your proof should use the coordinate isomorphism explicitly.)

7. Let $V$ be a finite-dimensional vector space with subspace $U \subseteq V$. Prove that $\dim U \leq \dim V$, and that $\dim U = \dim V$ implies $U = V$. (Hint: extend a basis of $U$ to a basis of $V$.)

8. (Challenge) Let $V = \mathbb{R}^n$ and suppose $W_1, W_2, \ldots, W_k$ are subspaces satisfying $\dim W_i = n - 1$ for each $i$. Prove that $\bigcap_{i=1}^k W_i$ has dimension at least $n - k$. Give an example showing this bound is sharp. Then determine: for which values of $k$ is it possible that $\bigcap_{i=1}^k W_i = \{\mathbf{0}\}$?

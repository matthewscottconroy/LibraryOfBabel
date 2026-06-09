# Chapter 4 — Exercises

## Important Figures

- **Hermann Grassmann (1809–1877)** — *Die lineale Ausdehnungslehre* (1844): first abstract treatment of what we now call a vector space; largely ignored by contemporaries
- **Giuseppe Peano (1858–1932)** — gave the first fully modern axiomatic definition of a vector space (1888) in *Calcolo Geometrico*
- **Évariste Galois (1811–1832)** — introduced finite fields $\mathbb{F}_{p^n}$ as part of his work on polynomial equations; the fields of scalars are not always $\mathbb{R}$ or $\mathbb{C}$
- **Arthur Cayley (1821–1895)** — developed $n$-dimensional geometry; early work connecting coordinate geometry to algebra

## References and Primary Sources

- **H. Grassmann, *Die lineale Ausdehnungslehre* (1844)** — the founding (if unrecognized) text of abstract vector space theory
- **G. Peano, *Calcolo Geometrico* (1888)** — first axiomatic vector space
- **P. Halmos, *Finite-Dimensional Vector Spaces* (2nd ed., Van Nostrand, 1958)** — elegant and concise; coordinates and duality emphasized
- **S. Axler, *Linear Algebra Done Right* (4th ed., Springer, 2024)** — modern presentation; avoids determinants until necessary

## Examples, Applications, and Thought Experiments

- **$\mathbb{R}^n$ vs. polynomial spaces** — $\mathbb{R}[x]_{\leq n}$ (polynomials of degree $\leq n$) and $\mathbb{R}^{n+1}$ are isomorphic as $\mathbb{R}$-vector spaces, even though one "looks like" functions and the other "looks like" tuples; the abstract axioms capture what they share
- **The field of scalars matters** — $\mathbb{C}$ has dimension 1 over $\mathbb{C}$ and dimension 2 over $\mathbb{R}$; $\mathbb{R}$ has dimension 1 over $\mathbb{R}$ but uncountably infinite dimension over $\mathbb{Q}$; the same set, completely different structural properties depending on the choice of scalars
- **Solution spaces of linear ODEs** — the set of solutions to $y'' + y = 0$ is a 2-dimensional $\mathbb{R}$-vector space spanned by $\sin x$ and $\cos x$; the differential equation is a linear constraint, and the solution set is a subspace; this motivates why linear algebra is the language of differential equations
- **$\mathbb{F}_2$-vector spaces** — over the field $\mathbb{F}_2 = \{0,1\}$, a vector space is just an abelian group where every nonzero element has order 2; $\mathbb{F}_2^n$ is the "bit string" space underlying coding theory and cryptography

## Exercises

1. Let $F$ be a field. Using only the field axioms, prove that the additive identity $0$ and the multiplicative identity $1$ are unique, and that additive and multiplicative inverses are unique. (Your proof should identify, for each claim, exactly which axioms are used.)

2. Verify that $\mathbb{F}_5 = \mathbb{Z}/5\mathbb{Z}$ is a field by checking all the field axioms. Then determine whether $\mathbb{Z}/6\mathbb{Z}$ is a field, and if not, identify which axiom fails and exhibit a specific witness.

3. Let $V = \mathbb{R}^3$ with the standard vector space structure. For each of the following subsets, determine whether it is a subspace, and prove your answer using the three-condition subspace test:
   - (a) $W_1 = \{(x, y, z) : 2x - y + 3z = 0\}$
   - (b) $W_2 = \{(x, y, z) : x^2 + y^2 = 0\}$
   - (c) $W_3 = \{(x, y, z) : x + y = 1\}$

4. Let $P_n$ denote the vector space of polynomials of degree at most $n$ over $\mathbb{R}$. Show that the set of polynomials $p \in P_4$ satisfying $p(1) = 0$ is a subspace of $P_4$. Find a basis for this subspace and determine its dimension. (Do not use the word "dimension" without justification at this stage — describe the basis explicitly.)

5. Let $W_1$ and $W_2$ be subspaces of a vector space $V$. Prove that $W_1 \cup W_2$ is a subspace of $V$ if and only if $W_1 \subseteq W_2$ or $W_2 \subseteq W_1$. (The "only if" direction is where the interesting work lies: find a vector in each subspace that is not in the other, and show their sum leads to a contradiction.)

6. Let $V = \mathbb{F}_2^3$, the vector space of 3-tuples over the two-element field $\mathbb{F}_2 = \{0, 1\}$. List all elements of $V$. How many subspaces does $V$ have? For each dimension $d \in \{0, 1, 2, 3\}$, count the number of subspaces of dimension $d$.

7. Suppose $V = W_1 \oplus W_2$ is an internal direct sum of subspaces. Prove that every element $v \in V$ can be written uniquely as $v = w_1 + w_2$ with $w_1 \in W_1$ and $w_2 \in W_2$. Then show that the projection maps $\pi_i : V \to W_i$ defined by $\pi_1(w_1 + w_2) = w_1$ and $\pi_2(w_1 + w_2) = w_2$ are well-defined and satisfy $\pi_1 + \pi_2 = \mathrm{id}_V$ and $\pi_i^2 = \pi_i$.

8. (Challenge) Let $V = \mathbb{R}[x]$, the space of all polynomials over $\mathbb{R}$ (no degree restriction). Consider the subspaces $W_e$ of even polynomials (those satisfying $p(-x) = p(x)$) and $W_o$ of odd polynomials ($p(-x) = -p(x)$). Prove that $V = W_e \oplus W_o$. Then consider the map $\phi : \mathbb{R}[x] \to \mathbb{R}[x]$ defined by $\phi(p)(x) = p(x^2)$. Describe the image of $\phi$, prove it is a subspace, and determine which elements of $W_e$ lie in the image. Is every even polynomial in the image of $\phi$?

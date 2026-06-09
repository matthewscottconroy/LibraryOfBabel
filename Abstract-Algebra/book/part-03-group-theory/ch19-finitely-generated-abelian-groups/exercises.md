# Chapter 19 — Exercises

## Important Figures

- **Leopold Kronecker (1823–1891)** — proved the structure theorem for finite abelian groups (1870); the primary decomposition form
- **Henry John Stephen Smith (1826–1883)** — Smith normal form of integer matrices (1861): the algorithmic heart of the proof; extends to matrices over any Euclidean domain
- **Georg Frobenius (1849–1917) & Ludwig Stickelberger (1850–1936)** — complete invariant factor form of the theorem (1879)

## References and Primary Sources

- **L. Kronecker, "Auseinandersetzung einiger Eigenschaften der Klassenanzahl idealer komplexer Zahlen" (1870)**
- **H.J.S. Smith, "On Systems of Linear Indeterminate Equations and Congruences" (1861)** — *Phil. Trans. Roy. Soc.* — Smith normal form
- **T. Hungerford, *Algebra* (Springer, 1974)**, Ch. II — clean treatment via modules over PIDs
- **D. Dummit & R. Foote, *Abstract Algebra* (3rd ed.)**, Ch. 5

## Examples, Applications, and Thought Experiments

- **$\mathbb{Z}/12\mathbb{Z}$ vs. $\mathbb{Z}/2\mathbb{Z} \times \mathbb{Z}/6\mathbb{Z}$** — both have order 12, but they are not isomorphic; $\mathbb{Z}/12\mathbb{Z}$ is cyclic (has an element of order 12); $\mathbb{Z}/2\mathbb{Z} \times \mathbb{Z}/6\mathbb{Z}$ has maximum element order $\text{lcm}(2,6) = 6$; the invariant factors distinguish them ($12$ vs. $2,6$)
- **Homology groups of a torus** — $H_1(T^2) \cong \mathbb{Z} \oplus \mathbb{Z}$; the two $\mathbb{Z}$ factors correspond to the two independent 1-cycles (the two "holes"); the structure theorem identifies this as a free abelian group of rank 2; $H_1$ of any surface is a finitely generated abelian group
- **Smith normal form computation** — the integer matrix with rows $(2, 4)$ and $(6, 12)$: row and column operations reduce it to the diagonal matrix $\text{diag}(2, 0)$; the cokernel is $\mathbb{Z}/2\mathbb{Z} \oplus \mathbb{Z}$; this is the group presented by the original matrix
- **All abelian groups of order 360** — $360 = 2^3 \cdot 3^2 \cdot 5$; at each prime, the Sylow subgroup is an abelian $p$-group classified by partitions; $p = 2$: partitions of 3 give $\mathbb{Z}/8$, $\mathbb{Z}/4 \times \mathbb{Z}/2$, $\mathbb{Z}/2 \times \mathbb{Z}/2 \times \mathbb{Z}/2$; $p = 3$: $\mathbb{Z}/9$, $\mathbb{Z}/3 \times \mathbb{Z}/3$; $p = 5$: $\mathbb{Z}/5$; total: $3 \times 2 \times 1 = 6$ groups

## Exercises

1. List all abelian groups of order 72 up to isomorphism. Present each in both invariant factor form and primary decomposition form, and verify that the two representations carry the same information via the Chinese Remainder Theorem.

2. Use Smith normal form to classify the abelian group $A$ presented by the matrix $M$ with rows $(2, 2)$ and $(0, 4)$ over $\mathbb{Z}$ (i.e., $A = \mathbb{Z}^2 / \text{im}(M)$ where $M$ acts by left multiplication). Reduce $M$ to diagonal form using integer row and column operations and identify the invariant factors.

3. Show that the maximum order of an element in $\mathbb{Z}/d_1\mathbb{Z} \oplus \cdots \oplus \mathbb{Z}/d_k\mathbb{Z}$ (with $d_1 \mid d_2 \mid \cdots \mid d_k$) equals $d_k$. Use this to distinguish the two abelian groups of order 36 that have the same primary decomposition at $p = 2$ but differ at $p = 3$.

4. Compute $\text{Hom}(\mathbb{Z}/m\mathbb{Z}, \mathbb{Z}/n\mathbb{Z})$ as an abelian group for general $m, n \geq 1$. Show it is cyclic of order $\gcd(m,n)$. Use this and the structure theorem to compute $\text{Hom}(A, B)$ when $A = \mathbb{Z}/6\mathbb{Z} \oplus \mathbb{Z}/10\mathbb{Z}$ and $B = \mathbb{Z}/15\mathbb{Z}$.

5. Prove that a finitely generated abelian group $A$ is free (i.e., isomorphic to $\mathbb{Z}^r$ for some $r \geq 0$) if and only if $A$ has no torsion elements. Deduce that every subgroup of a free abelian group is free.

6. The group $(\mathbb{Z}/p^k\mathbb{Z})^*$ of units modulo a prime power is known to be cyclic for odd primes $p$ and for $p = 2, k \leq 2$. Verify this for $p = 5, k = 2$ by computing $|(\mathbb{Z}/25\mathbb{Z})^*|$ and finding an explicit generator. For $p = 2, k = 3$, show $(\mathbb{Z}/8\mathbb{Z})^* \cong \mathbb{Z}/2\mathbb{Z} \times \mathbb{Z}/2\mathbb{Z}$ and is not cyclic.

7. Let $A$ be a finite abelian group of order $n$ and let $d \mid n$. Prove that $A$ contains a subgroup of order $d$. (Compare with the general case: Lagrange's theorem guarantees nothing here, but the structure theorem gives more.)

8. (Challenge) Prove the uniqueness of invariant factors: if $\mathbb{Z}/d_1\mathbb{Z} \oplus \cdots \oplus \mathbb{Z}/d_k\mathbb{Z} \cong \mathbb{Z}/e_1\mathbb{Z} \oplus \cdots \oplus \mathbb{Z}/e_m\mathbb{Z}$ with $d_1 \mid \cdots \mid d_k$ and $e_1 \mid \cdots \mid e_m$, then $k = m$ and $d_i = e_i$ for all $i$. (Hint: for each prime $p$, compare the $p$-primary parts of both sides using the primary decomposition, and count elements of each order.)

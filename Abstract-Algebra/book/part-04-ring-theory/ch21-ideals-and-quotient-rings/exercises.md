# Chapter 21 — Exercises

## Important Figures

- **Ernst Kummer (1810–1893)** — introduced "ideal numbers" (1847) to restore unique factorization in the ring of cyclotomic integers $\mathbb{Z}[\zeta_p]$; these are the conceptual precursor to ideals
- **Richard Dedekind (1831–1916)** — reinterpreted Kummer's ideal numbers as actual subsets of rings (*ideals*) satisfying absorption properties (1871); the modern definition
- **Emmy Noether (1882–1935)** — abstract ideal theory; primary decomposition of ideals; Noetherian rings; chain conditions

## References and Primary Sources

- **E. Kummer, "Über die Theorie der komplexen Zahlen" (1847)** — origin of the ideal concept
- **R. Dedekind, "Über die Theorie der ganzen algebraischen Zahlen" (1877)** — ideals defined as sets
- **D. Eisenbud, *Commutative Algebra with a View toward Algebraic Geometry* (Springer, 1995)** — prime spectrum; algebraic geometry connections
- **M. Atiyah & I.G. MacDonald, *Introduction to Commutative Algebra* (Addison-Wesley, 1969)** — concise and definitive

## Examples, Applications, and Thought Experiments

- **$\mathbb{Z}/n\mathbb{Z}$ as a quotient ring** — $\mathbb{Z}/(n) \cong \mathbb{Z}/n\mathbb{Z}$; the quotient ring construction recovers ordinary modular arithmetic; when $n = p$ is prime, $\mathbb{Z}/(p)$ is a field because $(p)$ is a maximal ideal in $\mathbb{Z}$
- **Prime vs. maximal in $\mathbb{Z}$** — $(0)$ is a prime ideal (since $\mathbb{Z}/(0) \cong \mathbb{Z}$ is an integral domain) but not maximal; $(p)$ for $p$ prime is both prime and maximal (since $\mathbb{Z}/(p)$ is a field); every maximal ideal is prime but not conversely
- **$\mathbb{R}[x]/(x^2+1) \cong \mathbb{C}$** — the ideal $(x^2+1)$ is maximal because $x^2+1$ is irreducible over $\mathbb{R}$; the quotient is a field extension of $\mathbb{R}$ of degree 2; $i := x + (x^2+1)$ satisfies $i^2 = -1$; this is how $\mathbb{C}$ is constructed from $\mathbb{R}$ by pure algebra
- **Nilradical and Jacobson radical** — the nilradical $\sqrt{(0)}$ is the intersection of all prime ideals; the Jacobson radical is the intersection of all maximal ideals; both measure "how far" a ring is from being a domain or a field; for $\mathbb{Z}/12\mathbb{Z}$, the nilradical is $(6)/(12\mathbb{Z})$

## Exercises

1. Use the ideal test to determine which of the following subsets are ideals in the given ring: (a) the set of all multiples of 6 in $\mathbb{Z}/24\mathbb{Z}$; (b) the set $\{f \in \mathbb{R}[x] : f(2) = 0\}$; (c) the set of all upper-triangular matrices in $M_2(\mathbb{R})$; (d) the set $\{(a, 0) : a \in \mathbb{Z}\}$ in $\mathbb{Z} \times \mathbb{Z}$.

2. Let $I$ and $J$ be ideals in a commutative ring $R$. Prove that $I + J = \{a + b : a \in I, b \in J\}$ and $I \cap J$ are both ideals. Show by example that $I \cup J$ need not be an ideal, but that if $I \subseteq J$ or $J \subseteq I$ then $I \cup J$ is an ideal.

3. Determine all prime ideals and all maximal ideals of $\mathbb{Z}/36\mathbb{Z}$. For each prime ideal $\mathfrak{p}$, verify directly that the quotient $(\mathbb{Z}/36\mathbb{Z})/\mathfrak{p}$ is an integral domain.

4. Prove the Correspondence Theorem for rings: if $I$ is an ideal of $R$, then there is a bijection between the ideals of $R$ that contain $I$ and the ideals of $R/I$, given by $J \mapsto J/I$. Show that this bijection preserves the property of being prime, and the property of being maximal.

5. Compute the quotient ring $\mathbb{Z}[x]/(x^2 - 3, 2)$. Identify it as a familiar ring, and determine whether it is a field, an integral domain, or neither.

6. Let $R$ be a commutative ring and $\mathfrak{p}$ a prime ideal. Prove that $\mathfrak{p}$ is prime if and only if $R/\mathfrak{p}$ is an integral domain. Then prove that $\mathfrak{m}$ is maximal if and only if $R/\mathfrak{m}$ is a field. Conclude that in a commutative ring, every maximal ideal is prime.

7. Find the nilradical and Jacobson radical of $\mathbb{Z}/72\mathbb{Z}$. Verify that the nilradical equals the intersection of all prime ideals, and that the Jacobson radical equals the intersection of all maximal ideals.

8. (Challenge) Let $R = C([0,1], \mathbb{R})$ be the ring of continuous real-valued functions on $[0,1]$. For each $a \in [0,1]$, let $\mathfrak{m}_a = \{f \in R : f(a) = 0\}$. Prove that each $\mathfrak{m}_a$ is a maximal ideal. Then prove that every maximal ideal of $R$ is of this form. (Hint: suppose $\mathfrak{m}$ is a maximal ideal not of this form; then for every $a \in [0,1]$ there exists $f_a \in \mathfrak{m}$ with $f_a(a) \neq 0$; use compactness to derive a contradiction.)

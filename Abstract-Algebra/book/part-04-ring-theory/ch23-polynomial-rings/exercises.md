# Chapter 23 — Exercises

## Important Figures

- **Leopold Kronecker (1823–1891)** — constructed all algebraic extensions of $\mathbb{Q}$ via quotient polynomial rings; the "Kronecker construction" of field extensions
- **David Hilbert (1862–1943)** — Hilbert basis theorem (1890): $R[x]$ is Noetherian if $R$ is; Nullstellensatz (1893); syzygy theorem; foundational results in polynomial ring theory
- **Bruno Buchberger (1942–)** — Gröbner bases algorithm (his PhD thesis, 1965); made polynomial ideal membership decidable and computable

## References and Primary Sources

- **D. Hilbert, "Über die Theorie der algebraischen Formen" (1890)** — Hilbert basis theorem; Nullstellensatz
- **D. Cox, J. Little & D. O'Shea, *Ideals, Varieties, and Algorithms* (4th ed., Springer, 2015)** — accessible introduction to Gröbner bases; algebraic geometry motivation
- **S. Lang, *Algebra* (rev. 3rd ed., Springer, 2002)**, Chs. IV–V — polynomial rings over fields and over UFDs

## Examples, Applications, and Thought Experiments

- **Constructing $\mathbb{C}$ from $\mathbb{R}$** — $\mathbb{R}[x]/(x^2+1) \cong \mathbb{C}$; the ideal $(x^2+1)$ is maximal since $x^2+1$ is irreducible over $\mathbb{R}$; $i := x + (x^2+1)$ satisfies $i^2 = -1$; every algebraic extension is built this way
- **Eisenstein's criterion** — $f(x) = x^5 - 4x + 2 \in \mathbb{Z}[x]$: $p = 2$ divides all non-leading coefficients but $p^2 = 4$ does not divide the constant term; by Eisenstein, $f$ is irreducible over $\mathbb{Q}$; this is the most efficient irreducibility test for specific polynomials
- **$\mathbb{R}[x]/(x^2-1)$ splits** — $x^2-1 = (x-1)(x+1)$ factors; by CRT, $\mathbb{R}[x]/(x^2-1) \cong \mathbb{R}[x]/(x-1) \times \mathbb{R}[x]/(x+1) \cong \mathbb{R} \times \mathbb{R}$; the quotient splits when the polynomial is reducible
- **Gröbner basis computation** — to determine whether $f = x^2y - 1$ lies in the ideal $(xy^2 - x, x^3 - y^2)$: compute a Gröbner basis; the normal form of $f$ with respect to this basis determines membership; Buchberger's algorithm reduces the membership problem to polynomial division

## Exercises

1. Use polynomial long division to divide $f(x) = x^4 - 3x^3 + x - 5$ by $g(x) = x^2 - 2x + 1$ in $\mathbb{Q}[x]$, obtaining quotient and remainder. Then compute $\gcd(f, g)$ using the Euclidean algorithm.

2. Determine which of the following polynomials are irreducible over $\mathbb{Q}$, over $\mathbb{R}$, and over $\mathbb{C}$: (a) $x^4 + 4$; (b) $x^3 - 3x + 1$; (c) $x^4 + x^3 + x^2 + x + 1$; (d) $x^6 - 2$.

3. Apply Eisenstein's criterion (possibly after a substitution $x \mapsto x + a$ for suitable $a$) to prove irreducibility over $\mathbb{Q}$ of: (a) $x^4 - 10x^2 + 1$ (hint: $x \mapsto x+1$ may help; or try $p=2$ directly); (b) the cyclotomic polynomial $\Phi_p(x) = x^{p-1} + x^{p-2} + \cdots + x + 1$ for a prime $p$.

4. Factor $x^4 + 1$ over $\mathbb{F}_2$, $\mathbb{F}_3$, and $\mathbb{F}_5$, completely into irreducibles in each case. Comment on how the factorization changes with the field.

5. Describe the ring $\mathbb{Q}[x]/(x^3 - 2)$: exhibit a $\mathbb{Q}$-basis, write down the multiplication table for basis elements, and determine whether this ring is a field. If it is a field, identify it as a familiar extension of $\mathbb{Q}$.

6. Let $f(x) = x^4 - 5x^2 + 6 \in \mathbb{Q}[x]$. Factor $f$ completely over $\mathbb{Q}$, over $\mathbb{R}$, and over $\mathbb{C}$. Use the Chinese Remainder Theorem to decompose $\mathbb{Q}[x]/(f)$ as a direct product of simpler rings.

7. Let $F$ be a field and $f(x) \in F[x]$ an irreducible polynomial of degree $n$. Prove that $F[x]/(f)$ is a field extension of $F$ of degree $n$, and that $f$ has a root in this extension. Give an explicit example with $F = \mathbb{F}_3$ and $f(x) = x^2 + 1$.

8. (Challenge) Consider the ideal $I = (x^2 + y - 1, x + y^2 - 1)$ in $\mathbb{Q}[x, y]$. Choose the lexicographic monomial order with $x > y$. Compute a Gröbner basis for $I$ by hand using Buchberger's algorithm: form the $S$-polynomial of the two generators, reduce it modulo the generating set, and adjoin any nonzero remainder. Use your Gröbner basis to find all solutions to the system $x^2 + y = 1$, $x + y^2 = 1$ in $\mathbb{Q}^2$.

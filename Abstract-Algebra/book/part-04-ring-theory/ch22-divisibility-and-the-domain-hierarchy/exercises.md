# Chapter 22 — Exercises

## Important Figures

- **Carl Friedrich Gauss (1777–1855)** — proved unique factorization in $\mathbb{Z}[i]$ (Gaussian integers); Gauss's lemma relating factorization in $R[x]$ and $\text{Frac}(R)[x]$
- **Richard Dedekind (1831–1916)** — introduced Dedekind domains (not by that name) to handle rings like $\mathbb{Z}[\sqrt{-5}]$ where ideal factorization replaces element factorization
- **Pierre Samuel (1921–2009)** — "About Euclidean Rings" (1971): showed $\mathbb{Z}[\frac{1+\sqrt{-19}}{2}]$ is a PID that is not Euclidean with any "norm" function; the hierarchy is strict

## References and Primary Sources

- **C.F. Gauss, *Disquisitiones Arithmeticae* (1801)** — unique factorization in $\mathbb{Z}$; quadratic forms; foundational number theory
- **P. Samuel, "About Euclidean Rings" (1971)** — *J. Algebra* 19 — examples where PID $\not\Rightarrow$ Euclidean
- **D. Samuel & O. Zariski, *Commutative Algebra* (2 vols., Van Nostrand, 1958–1960)** — comprehensive reference

## Examples, Applications, and Thought Experiments

- **$\mathbb{Z}[i]$: Gaussian integer factorization** — $5 = (2+i)(2-i)$ in $\mathbb{Z}[i]$; the rational prime 5 splits because $5 \equiv 1 \pmod{4}$; the norm $N(a+bi) = a^2+b^2$ is a Euclidean function; $\mathbb{Z}[i]$ is Euclidean, hence a PID, hence a UFD
- **$\mathbb{Z}[\sqrt{-5}]$ is not a UFD** — $6 = 2 \cdot 3 = (1+\sqrt{-5})(1-\sqrt{-5})$; the norm shows all four factors are irreducible; but $2 \nmid (1\pm\sqrt{-5})$; not a UFD; yet as a Dedekind domain, ideal factorization is unique: $(6) = \mathfrak{p}_2 \mathfrak{p}_3 \bar{\mathfrak{p}}_3$
- **Euclidean algorithm in $\mathbb{Z}$** — $\gcd(48, 18)$: $48 = 2 \cdot 18 + 12$; $18 = 1 \cdot 12 + 6$; $12 = 2 \cdot 6 + 0$; gcd $= 6$; this is the prototype; the same algorithm works in $k[x]$ using degree as the norm
- **The hierarchy is strict** — Euclidean domains $\subsetneq$ PIDs $\subsetneq$ UFDs $\subsetneq$ integral domains; each inclusion is proper; examples: $\mathbb{Z}[\frac{1+\sqrt{-19}}{2}]$ is PID but not Euclidean; $\mathbb{Z}[x]$ is UFD but not PID ($(2, x)$ is not principal); $\mathbb{Z}[\sqrt{-5}]$ is a domain but not a UFD

## Exercises

1. In $\mathbb{Z}[\sqrt{-5}]$, verify that $2$, $3$, $1+\sqrt{-5}$, and $1-\sqrt{-5}$ are all irreducible by computing norms. Show that none divides another in the wrong way, thereby confirming that $6 = 2 \cdot 3 = (1+\sqrt{-5})(1-\sqrt{-5})$ gives two genuinely distinct irreducible factorizations.

2. Carry out the Euclidean algorithm in $\mathbb{Z}[i]$ to compute $\gcd(11 + 3i, 1 + 8i)$, using the norm $N(a+bi) = a^2 + b^2$ at each division step. Express the gcd as a $\mathbb{Z}[i]$-linear combination of the two elements.

3. Let $R$ be an integral domain. Prove that every prime element of $R$ is irreducible. Then give a specific example in $\mathbb{Z}[\sqrt{-5}]$ of an irreducible element that is not prime, carefully verifying both properties.

4. Prove that a PID is a UFD by showing: (a) every nonzero non-unit has an irreducible factorization (use the ascending chain condition on principal ideals), and (b) in a PID every irreducible is prime (use the fact that $(p)$ is a maximal, hence prime, ideal when $p$ is irreducible).

5. Show that $\mathbb{Z}[x]$ is a UFD but not a PID. For the UFD part, use Gauss's theorem that if $R$ is a UFD then $R[x]$ is a UFD. For the non-PID part, show directly that the ideal $(2, x)$ is not principal.

6. Use the Euclidean algorithm in $k[x]$, for $k = \mathbb{Q}$, to compute $\gcd(x^4 - 1, x^3 - x^2 + x - 1)$ and express it as a $k[x]$-linear combination of the two polynomials.

7. Determine which of the following rings are Euclidean domains, PIDs, or UFDs, providing brief justifications: (a) $\mathbb{Z}[i]$; (b) $\mathbb{Z}[\sqrt{-3}]$; (c) $\mathbb{Q}[x, y]$; (d) $\mathbb{Z}[x]$.

8. (Challenge) Let $\mathcal{O} = \mathbb{Z}[\sqrt{-5}]$ and let $\mathfrak{p} = (2, 1+\sqrt{-5})$. Show that $\mathfrak{p}$ is an ideal of $\mathcal{O}$, that $\mathfrak{p}^2 = (2)$, and that $\mathfrak{p}$ is not principal. Conclude that the class group of $\mathcal{O}$ is nontrivial, and determine its order by finding all ideal classes.

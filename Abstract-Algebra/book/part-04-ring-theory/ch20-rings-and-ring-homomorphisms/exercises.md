# Chapter 20 — Exercises

## Important Figures

- **Richard Dedekind (1831–1916)** — rings arose in his study of algebraic integers; introduced ideals to restore unique factorization in rings where it fails; the word "ring" (*Ring*) was introduced by Hilbert in 1897 for Dedekind's number rings
- **Emmy Noether (1882–1935)** — abstract ring theory; *Idealtheorie in Ringbereichen* (1921): the definitive abstract treatment
- **Joseph Wedderburn (1882–1948)** — Wedderburn's theorem (1905): every finite division ring is a field; a profound constraint on finite algebraic structures

## References and Primary Sources

- **E. Noether, "Idealtheorie in Ringbereichen" (1921)** — *Math. Ann.* 83 — founding paper of abstract ring theory
- **B.L. van der Waerden, *Moderne Algebra* (1930)**, Vol. 1 — dissemination of Noether's approach
- **D. Dummit & R. Foote, *Abstract Algebra* (3rd ed., Wiley, 2004)**, Chs. 7–9 — rings, ideals, and ring homomorphisms

## Examples, Applications, and Thought Experiments

- **$\mathbb{Z}[\sqrt{-5}]$** — an integral domain where unique factorization fails: $6 = 2 \cdot 3 = (1+\sqrt{-5})(1-\sqrt{-5})$; both factorizations use irreducible elements; the norm $N(a+b\sqrt{-5}) = a^2 + 5b^2$ shows none of these factors further; this failure of UFD motivated Kummer and Dedekind to introduce ideals
- **The zero ring** — the ring $\{0\}$ where $0 \cdot 0 = 0 + 0 = 0$ and $1 = 0$; it has exactly one element; any ring map to the zero ring exists; this is the terminal object in the category of rings
- **$C[0,1]$: ring of continuous functions** — pointwise addition and multiplication; maximal ideals are $\mathfrak{m}_a = \{f : f(a) = 0\}$ for each $a \in [0,1]$; the ring recovers the topological space from its maximal ideal spectrum; this is the prototype for spectral theory in algebraic geometry
- **Matrix rings** — $M_n(k)$ is a non-commutative ring; its only ideals are $\{0\}$ and $M_n(k)$ (it is simple); yet it is far from a field; this shows why the theory of ideals in non-commutative rings differs fundamentally from the commutative case

## Exercises

1. Verify directly from the axioms that the following are rings, and determine in each case whether the ring is commutative, unital, and whether it is an integral domain: (a) $\mathbb{Z}/6\mathbb{Z}$ with the usual modular arithmetic; (b) the set $2\mathbb{Z}$ of even integers under ordinary addition and multiplication; (c) $\mathbb{Z}[i] = \{a + bi : a, b \in \mathbb{Z}\}$, the Gaussian integers.

2. Show that the set $R = \{a + b\sqrt{2} : a, b \in \mathbb{Z}\}$ is a subring of $\mathbb{R}$, and find all units of $R$. (Hint: use the norm $N(a + b\sqrt{2}) = a^2 - 2b^2$ and the fact that $N(uv) = N(u)N(v)$.)

3. Let $R$ be a ring and let $a \in R$ be nilpotent, meaning $a^n = 0$ for some positive integer $n$. Prove that $1 - a$ is a unit in $R$ by constructing its inverse explicitly as a finite sum. Conclude that in a commutative ring, the sum of a unit and a nilpotent element is again a unit.

4. Determine all ring homomorphisms $\phi \colon \mathbb{Z}/12\mathbb{Z} \to \mathbb{Z}/18\mathbb{Z}$. For each, identify the kernel and the image.

5. Let $\phi \colon R \to S$ be a ring homomorphism. Prove that if $u \in R$ is a unit, then $\phi(u)$ is a unit in $S$. Give an example showing that $\phi$ need not map units surjectively onto the units of $S$.

6. Determine the characteristic of each of the following rings: (a) $\mathbb{Z}/15\mathbb{Z}$; (b) $M_2(\mathbb{F}_3)$, the ring of $2 \times 2$ matrices over the field with three elements; (c) $\mathbb{Q}[x]/(x^2 - 2)$; (d) $\mathbb{F}_2 \times \mathbb{F}_3$.

7. Let $R$ be a commutative ring of characteristic $p$, where $p$ is prime. Prove that the map $\phi \colon R \to R$ defined by $\phi(a) = a^p$ is a ring homomorphism (the Frobenius endomorphism). Where does the proof use the primality of $p$?

8. (Challenge) Let $R$ be a finite ring with no zero divisors and with $|R| > 1$. Prove that $R$ is a division ring. (This is the finite case of Wedderburn's theorem without the commutativity conclusion.) Then, assuming Wedderburn's theorem that every finite division ring is a field, deduce that every finite integral domain is a field, and exhibit an example showing this fails for infinite domains.

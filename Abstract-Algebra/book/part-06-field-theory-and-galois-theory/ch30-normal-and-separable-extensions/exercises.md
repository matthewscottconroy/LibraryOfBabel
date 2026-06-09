# Chapter 30 — Exercises

## Important Figures

- **Évariste Galois (1811–1832)** — normality implicit in his splitting field constructions; a field is "normal" if it is stable under all automorphisms
- **Heinrich Weber (1842–1913)** — first clear formulation of normal extensions (1893); systematic field theory
- **Ernst Artin (1898–1962)** — reformulated Galois theory via fixed fields of automorphism groups (1942); clarified the role of separability in the correspondence

## References and Primary Sources

- **E. Artin, *Galois Theory* (Notre Dame Math. Lectures, 1942; 2nd ed. 1944)** — Artin's modern approach via automorphisms and fixed fields; the definitive 20th-century treatment
- **N. Jacobson, *Basic Algebra I* (2nd ed., Freeman, 1985)** — separability and the formal derivative
- **J. Milne, *Fields and Galois Theory* (v5.10, 2022; freely available)** — comprehensive online notes

## Examples, Applications, and Thought Experiments

- **$\mathbb{Q}(\sqrt[3]{2})$ is not normal over $\mathbb{Q}$** — the minimal polynomial $x^3 - 2$ has roots $\sqrt[3]{2}$, $\omega\sqrt[3]{2}$, $\omega^2\sqrt[3]{2}$ (where $\omega = e^{2\pi i/3}$); the other two roots are not in $\mathbb{Q}(\sqrt[3]{2}) \subset \mathbb{R}$; a field extension is normal iff every irreducible polynomial with one root in it has all roots in it
- **Splitting field of $x^4-5$** — must adjoin $\sqrt[4]{5}$ and $i$; the splitting field is $\mathbb{Q}(\sqrt[4]{5}, i)$; degree 8 over $\mathbb{Q}$; the Galois group is the dihedral group $D_4$ (order 8)
- **Inseparable extensions in characteristic $p$** — $\mathbb{F}_p(t^{1/p})$ over $\mathbb{F}_p(t)$: the polynomial $x^p - t$ is irreducible (Eisenstein with prime $t$) but $(x - t^{1/p})^p = x^p - t$; only one root with multiplicity $p$; no inseparability occurs in characteristic 0 (the formal derivative of a non-constant irreducible polynomial is coprime to it)
- **The Primitive Element Theorem** — every finite separable extension $K/F$ is simple: $K = F(\alpha)$ for some $\alpha$; the element $\alpha = \sqrt{2} + \sqrt{3}$ generates $\mathbb{Q}(\sqrt{2}, \sqrt{3})/\mathbb{Q}$; a single element encodes the entire extension

## Exercises

1. Show that $\mathbb{Q}(\sqrt[3]{2})/\mathbb{Q}$ is not a normal extension by exhibiting an irreducible polynomial in $\mathbb{Q}[x]$ that has a root in $\mathbb{Q}(\sqrt[3]{2})$ but does not split completely over $\mathbb{Q}(\sqrt[3]{2})$.

2. Let $K$ be the splitting field of $x^4 - 2$ over $\mathbb{Q}$. Determine $[K : \mathbb{Q}]$ and prove that $K/\mathbb{Q}$ is a normal extension. Write $K$ explicitly as a subfield of $\mathbb{C}$.

3. Let $f(x) = x^p - a \in F[x]$ where $F$ has characteristic $p$ and $a \notin F^p$. Compute the formal derivative $f'(x)$ and deduce that $f$ is inseparable. Show that $f = (x - \alpha)^p$ in the splitting field of $f$, where $\alpha^p = a$.

4. Prove that every irreducible polynomial over a field of characteristic zero is separable. Where exactly does the argument fail in characteristic $p$?

5. A field $F$ is perfect if every algebraic extension of $F$ is separable. Prove that every finite field $\mathbb{F}_{p^n}$ is perfect. (Hint: show that the Frobenius map $x \mapsto x^p$ is surjective on $\mathbb{F}_{p^n}$, and use this to analyze the formal derivative of any irreducible polynomial over $\mathbb{F}_{p^n}$.)

6. Let $K = \mathbb{Q}(\sqrt{2}, \sqrt{3})$. Find an explicit primitive element $\theta \in K$ such that $K = \mathbb{Q}(\theta)$, and compute the minimal polynomial of $\theta$ over $\mathbb{Q}$.

7. Prove that any extension of degree 2 is normal. Is every extension of degree 3 normal? Provide a proof or counterexample.

8. (Challenge) Let $F = \mathbb{F}_p(t)$ be the field of rational functions over $\mathbb{F}_p$, and let $K = F(t^{1/p})$. Prove that $K/F$ is a degree-$p$ extension that is not separable, and show moreover that $K/F$ has no primitive element — that is, the Primitive Element Theorem fails here. Identify exactly which hypothesis of the Primitive Element Theorem is violated.

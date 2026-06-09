# Chapter 26 — Exercises

## Important Figures

- **Henri Cartan (1904–2008) & Samuel Eilenberg (1913–1998)** — *Homological Algebra* (1956): projective and injective modules formalized; the homological machinery built on resolutions by these classes
- **Reinhold Baer (1902–1979)** — Baer's criterion for injectivity (1940): $M$ is injective iff every map from an ideal of $R$ into $M$ extends to all of $R$
- **Jean-Pierre Serre (1926–)** — Serre's theorem (1955): projective modules over polynomial rings over a field are free (conjectured); proved by Quillen and Suslin (1976)

## References and Primary Sources

- **H. Cartan & S. Eilenberg, *Homological Algebra* (Princeton, 1956)** — projective and injective modules introduced; the first systematic treatment
- **C. Weibel, *An Introduction to Homological Algebra* (Cambridge, 1994)** — standard modern text
- **F.W. Anderson & K.R. Fuller, *Rings and Categories of Modules* (2nd ed., Springer, 1992)** — categorical and module-theoretic treatment

## Examples, Applications, and Thought Experiments

- **Free module $\mathbb{Z}^n$** — the rank-$n$ free $\mathbb{Z}$-module; basis $\{e_1,\ldots,e_n\}$; every $\mathbb{Z}$-module (abelian group) is a quotient of a free $\mathbb{Z}$-module; free modules are the "simplest" modules; over a field, every module is free (every vector space has a basis)
- **Projective but not free** — over $R = \mathbb{Z}/6\mathbb{Z}$: the module $\mathbb{Z}/2\mathbb{Z}$ satisfies $R \cong \mathbb{Z}/2\mathbb{Z} \oplus \mathbb{Z}/3\mathbb{Z}$, making $\mathbb{Z}/2\mathbb{Z}$ a direct summand of $R$ (hence projective) but it has torsion and hence is not free
- **$\mathbb{Q}$ is an injective $\mathbb{Z}$-module** — every $\mathbb{Z}$-module map from a subgroup into $\mathbb{Q}$ extends to the whole group; this follows from Baer's criterion because $\mathbb{Q}$ is divisible; divisible abelian groups are precisely the injective $\mathbb{Z}$-modules
- **Thought experiment: the three "freedoms"** — free means "a basis exists"; projective means "lifts always exist" (universal lift property); injective means "extensions always exist" (universal extension property); flat means "tensoring is exact"; each class removes one obstruction to linearity, and together they classify the four degrees of "niceness" a module can have

## Exercises

1. Let $R$ be a ring and let $S$ be an arbitrary set. Construct the free $R$-module $F(S)$ on $S$ explicitly as the set of formal finite $R$-linear combinations of elements of $S$. State and prove its universal property: every function $f : S \to M$ into an $R$-module $M$ extends uniquely to an $R$-module homomorphism $\tilde{f} : F(S) \to M$. Use this to show that $\text{Hom}_R(R^n, M) \cong M^n$ as abelian groups for any $R$-module $M$.

2. Prove that a module $P$ is projective if and only if every short exact sequence $0 \to A \to B \to P \to 0$ splits. Deduce that every direct summand of a projective module is projective, and that every free module is projective.

3. Let $R = \mathbb{Z}/6\mathbb{Z}$. Show that $R \cong \mathbb{Z}/2\mathbb{Z} \oplus \mathbb{Z}/3\mathbb{Z}$ as $R$-modules. Explain why $\mathbb{Z}/2\mathbb{Z}$ is a projective $R$-module. Then show directly that $\mathbb{Z}/2\mathbb{Z}$ is not a free $R$-module by showing it cannot have a basis. This gives a concrete example of a projective module that is not free.

4. State Baer's criterion for injectivity. Use it to verify that $\mathbb{Q}$ is an injective $\mathbb{Z}$-module: for each ideal $n\mathbb{Z} \subseteq \mathbb{Z}$ and each homomorphism $f : n\mathbb{Z} \to \mathbb{Q}$, construct an explicit extension to a homomorphism $\mathbb{Z} \to \mathbb{Q}$.

5. An abelian group $D$ is called divisible if for every $d \in D$ and every nonzero $n \in \mathbb{Z}$ there exists $d' \in D$ with $n d' = d$. Prove that every divisible abelian group is an injective $\mathbb{Z}$-module, and that every injective $\mathbb{Z}$-module is divisible. Conclude that $\mathbb{Q}/\mathbb{Z}$ is injective and identify all injective $\mathbb{Z}$-modules.

6. A module $M$ over a commutative ring $R$ is flat if $- \otimes_R M$ is exact. Prove that every projective module is flat. Then show that $\mathbb{Q}$ is a flat $\mathbb{Z}$-module that is not projective as a $\mathbb{Z}$-module. (For the latter, use the fact that any projective $\mathbb{Z}$-module is free.)

7. Suppose $0 \to M' \to M \to M'' \to 0$ is a short exact sequence of $R$-modules and $M''$ is free. Prove that the sequence splits and that $M \cong M' \oplus M''$. Use this to show that submodules of free modules over a PID are free, and explain why this fails over general rings.

8. (Challenge) Prove that a finitely generated projective module over a local ring $R$ (a ring with a unique maximal ideal) is free. The key step is a version of Nakayama's lemma: if $M$ is finitely generated and $\mathfrak{m} M = M$ for the maximal ideal $\mathfrak{m}$, then $M = 0$. Use this to show that a minimal generating set of a projective module over a local ring forms a basis.

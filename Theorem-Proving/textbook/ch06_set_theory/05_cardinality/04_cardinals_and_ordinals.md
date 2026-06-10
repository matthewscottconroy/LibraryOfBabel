# Cardinals and Ordinals

## Two Ways to Generalize "Number"

The natural numbers serve two roles:
- **Cardinal numbers**: measuring *how many* elements a set has ($|\{a, b, c\}| = 3$)
- **Ordinal numbers**: measuring *order position* (the 3rd element in a sequence)

For finite sets, these coincide. For infinite sets, they diverge dramatically and become two distinct hierarchies of infinite numbers — both essential to set theory.

## Cardinal Numbers

The **cardinality** $|A|$ of a set $A$ measures its size via bijections: $|A| = |B|$ iff there exists a bijection $A \to B$.

For infinite sets, Cantor defined the **infinite cardinals**:
- $\aleph_0$ (aleph-naught): $|\mathbb{N}|$ — the smallest infinite cardinal
- $\aleph_1$: the smallest uncountable cardinal
- $\aleph_2, \aleph_3, \ldots$: and so on

Operations on cardinals:
- $\kappa + \lambda$: cardinality of a disjoint union
- $\kappa \cdot \lambda$: cardinality of a Cartesian product
- $\kappa^\lambda$: cardinality of the set of all functions from a $\lambda$-element set to a $\kappa$-element set

For infinite cardinals: $\kappa + \lambda = \kappa \cdot \lambda = \max(\kappa, \lambda)$ (addition and multiplication "collapse" — a striking departure from finite arithmetic).

$2^{\aleph_0} = |\mathbb{R}| = \mathfrak{c}$ (the cardinality of the continuum).

## Ordinal Numbers

**Ordinal numbers** generalize the *position* in a well-ordered sequence. Every ordinal is determined by the well-ordered set of all smaller ordinals:

- $0 = \emptyset$ (the empty well-order)
- $1 = \{0\} = \{\emptyset\}$
- $2 = \{0, 1\}$, $3 = \{0, 1, 2\}$, ...
- $\omega = \{0, 1, 2, 3, \ldots\}$ (the first infinite ordinal — the order type of $\mathbb{N}$)
- $\omega + 1 = \{0, 1, 2, \ldots, \omega\}$ (omega plus one step — a new element after all of $\mathbb{N}$)
- $\omega \cdot 2 = \omega + \omega$, $\omega^2$, $\omega^\omega$, $\varepsilon_0 = \omega^{\omega^{\omega^{\cdots}}}$ (a fixed point of $\alpha \mapsto \omega^\alpha$)

Ordinals measure the structure of well-orderings, not just size. Two sets can have the same cardinality but different ordinal structure: the ordinals $\omega$ and $\omega + 1$ are both countably infinite (same cardinal $\aleph_0$) but have different order types.

## The Relationship

Every cardinal is an ordinal (the smallest ordinal of a given cardinality). The alephs are the infinite cardinals in order: $\aleph_0 = \omega$, $\aleph_1 = \omega_1$, etc.

The Continuum Hypothesis: Is $2^{\aleph_0} = \aleph_1$? (Is there no cardinal between $\aleph_0$ and $2^{\aleph_0}$?) As shown by Gödel and Cohen, this question is independent of ZFC.

## Exercises
See [problems/ch06_set_theory/04_cardinality_challenges.md](../../../problems/ch06_set_theory/04_cardinality_challenges.md)

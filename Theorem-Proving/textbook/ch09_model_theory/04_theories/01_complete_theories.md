# Complete Theories

## Completeness of a Theory

A first-order **theory** $T$ (a set of sentences closed under consequence) is **complete** if for every sentence $\varphi$ in the language, either $T \vdash \varphi$ or $T \vdash \neg\varphi$.

A complete theory has no "gaps" — it answers every yes/no question about its subject matter.

## Examples

**Complete theories**:
- **DLO** (Dense Linear Orders without endpoints): the theory of $(\mathbb{Q}, <)$. Any two countable DLO models are isomorphic (Cantor's theorem), so the theory is $\omega$-categorical, hence complete.
- **ACF** (Algebraically Closed Fields) of fixed characteristic: the theory of $\mathbb{C}$ is complete (and decidable). Any two algebraically closed fields of the same characteristic and same uncountable cardinality are isomorphic.
- **RCF** (Real Closed Fields): the theory of $\mathbb{R}$ with $+, \times, <$. Complete by Tarski's quantifier elimination.

**Incomplete theories**:
- **PA** (Peano Arithmetic): Gödel showed PA is incomplete — there are sentences (like the Gödel sentence) that are neither provable nor disprovable from PA.
- **ZFC**: Similarly incomplete — CH is independent.
- **Group theory**: The theory of all groups is incomplete (some group-theoretic statements hold in some groups but not others).

## Categoricity Implies Completeness

**Theorem (Vaught)**: If a countable theory $T$ has no finite models and is $\kappa$-categorical (has a unique model of cardinality $\kappa$ up to isomorphism) for some infinite $\kappa$, then $T$ is complete.

This is why DLO and ACF are complete: they are $\aleph_0$-categorical (DLO) or $\kappa$-categorical for all uncountable $\kappa$ (ACF$_p$).

## Exercises
See [problems/ch09_model_theory/](../../../problems/ch09_model_theory/)

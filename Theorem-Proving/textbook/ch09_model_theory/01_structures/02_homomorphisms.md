# Homomorphisms and Isomorphisms

## Structure-Preserving Maps

A **homomorphism** between two $\Sigma$-structures $\mathcal{M}$ and $\mathcal{N}$ is a function $h : M \to N$ that preserves all the structure:

- For each constant symbol $c$: $h(c^{\mathcal{M}}) = c^{\mathcal{N}}$
- For each function symbol $f$ of arity $n$: $h(f^{\mathcal{M}}(a_1, \ldots, a_n)) = f^{\mathcal{N}}(h(a_1), \ldots, h(a_n))$
- For each relation symbol $R$ of arity $n$: $(a_1, \ldots, a_n) \in R^{\mathcal{M}} \implies (h(a_1), \ldots, h(a_n)) \in R^{\mathcal{N}}$

An **isomorphism** is a bijective homomorphism whose inverse is also a homomorphism.

Isomorphic structures are "the same" up to renaming of elements — they satisfy exactly the same first-order sentences.

## Embeddings and Elementary Embeddings

An **embedding** is an injective homomorphism that also reflects relations: $(h(a_1), \ldots, h(a_n)) \in R^{\mathcal{N}} \implies (a_1, \ldots, a_n) \in R^{\mathcal{M}}$.

An **elementary embedding** preserves and reflects *all first-order sentences*, not just atomic ones. Elementary embeddings are much more restrictive and connect directly to the Löwenheim-Skolem theorems.

## Examples

- Ring homomorphisms: preserve $+$ and $\times$ — standard algebraic homomorphisms
- Graph homomorphisms: map vertices to vertices, preserving edges
- $\mathbb{Q} \hookrightarrow \mathbb{R}$: an embedding of ordered fields (preserves $+, \times, <$)
- The inclusion $\mathbb{N} \hookrightarrow {}^*\mathbb{N}$ (standard naturals into a non-standard model): an elementary embedding if we include all first-order sentences

## Exercises
See [problems/ch09_model_theory/](../../../problems/ch09_model_theory/)

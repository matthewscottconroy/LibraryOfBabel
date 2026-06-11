# Placement: ch15-homological-algebra-basics.md

## Part in book/
This chapter belongs to **Part VIII: Homological Algebra** (`book/part-08-homological-algebra/`), specifically the foundational chapters:
- `ch38-chain-complexes-and-homology/`
- `ch39-resolutions/`

## Sections in book/ that cover this material

| Curriculum section | Book location |
|---|---|
| Chain complexes and their homology (§15.1) | `ch38-chain-complexes-and-homology/38.1-chain-complexes/` |
| Short and long exact sequences (§15.2) | `ch38-chain-complexes-and-homology/38.2-exact-sequences/` |
| The snake lemma (§15.3) | `ch38-chain-complexes-and-homology/38.3-the-snake-lemma/` |
| The five lemma (§15.4) | `ch38-chain-complexes-and-homology/38.4-diagram-lemmas/` |
| Free, projective, and injective resolutions (§15.5) | `ch39-resolutions/` |
| Extensions of modules and $\mathrm{Ext}^1$ (§15.6) | `ch40-derived-functors-ext-and-tor/40.2-ext/` |

## Content in curriculum/ not fully covered in book/

- The curriculum has a **fully worked derivation of the connecting homomorphism** $\delta: H_n(C'') \to H_{n-1}(C')$ via an explicit element-chasing argument (the "diagram chase"). The book states the snake lemma and uses it, but the explicit construction of $\delta$ from a short exact sequence of complexes is somewhat compressed in `ch38`. → Expand `38.2` with the full connecting homomorphism construction.
- The curriculum includes a section on **the horseshoe lemma** (constructing projective resolutions of middle terms from resolutions of outer terms in a short exact sequence). This is not explicitly present in `ch39`.  → Add as a subsection of `ch39`.
- The curriculum discusses the **long exact sequence in homology** as a theorem, stating and proving it from the snake lemma. The book has this, but the curriculum's treatment explicitly identifies the naturality of the connecting homomorphism, which is not stated as a theorem in `ch38`. This naturality is crucial for deriving spectral sequences later.

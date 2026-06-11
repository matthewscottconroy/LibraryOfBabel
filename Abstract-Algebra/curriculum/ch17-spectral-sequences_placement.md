# Placement: ch17-spectral-sequences.md

## Part in book/
This chapter belongs to **Part VIII: Homological Algebra**, specifically:
- `ch41-spectral-sequences/`

## Sections in book/ that cover this material

| Curriculum section | Book location |
|---|---|
| Motivation and filtered complexes (§17.1) | `ch41-spectral-sequences/41.1-the-idea-and-setup/41.1.1` |
| Bigraded pages and differentials (§17.2) | `ch41-spectral-sequences/41.1-the-idea-and-setup/41.1.2` |
| Double complexes (§17.3) | `ch41-spectral-sequences/41.2-double-complexes/` |
| Convergence (§17.4) | `ch41-spectral-sequences/41.1-the-idea-and-setup/41.1.4` |
| LHS spectral sequence (§17.5) | `ch41-spectral-sequences/41.3-the-lyndon-hochschild-serre-spectral-sequence/` |
| Five-term exact sequence (§17.6) | `ch41-spectral-sequences/41.4-reading-and-using-spectral-sequences/` |

## Content in curriculum/ not fully covered in book/

- The curriculum has a **fully worked computation of $H^*(BS^1) = H^*(K(\mathbb{Z},1))$ via the Serre spectral sequence** for the fibration $S^1 \to * \to BS^1$. The book focuses on algebraic (group-theoretic) spectral sequences; the topological Serre spectral sequence is mentioned but not computed in detail. → Consider adding a topology appendix or worked example to `ch41`.
- The curriculum discusses the **Eilenberg-Moore spectral sequence** briefly (in the context of cohomology of loop spaces). This is entirely absent from the book.
- The curriculum explicitly distinguishes the two conventions for bigraded differentials (Serre vs. Adams) and warns about sign conventions. The book uses one convention throughout but does not flag the existence of the other. → Add a remark in `41.1.2` about conventions.
- The curriculum has exercises working through the **Künneth spectral sequence** as a consequence of the double complex for $C_\bullet(X) \otimes C_\bullet(Y)$. This is not in `ch41`.

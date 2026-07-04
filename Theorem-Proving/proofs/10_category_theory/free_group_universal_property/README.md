# The Free Group Satisfies the Universal Property

**Statement**: For a set $S$, the free group $F(S)$ (reduced words over
$S \cup S^{-1}$) satisfies: for every group $G$ and function $f : S \to G$,
there is a *unique* group homomorphism $\bar{f} : F(S) \to G$ with
$\bar{f} \circ \iota = f$.

Categorically: $F$ is left adjoint to the forgetful functor
$U : \mathbf{Grp} \to \mathbf{Set}$, with unit $\iota$. This is the canonical
example of a universal property — an object characterized not by what it *is*
but by how everything else maps through it.

## Files
- `paper_proof.md`: construction of $F(S)$, existence and uniqueness of $\bar{f}$, categorical reformulation

## Related
- Textbook: Chapter 21 (Category Theory), especially adjunctions; Chapter 19 (Abstract Algebra) for groups
- Problems: `problems/ch21_category_theory/`

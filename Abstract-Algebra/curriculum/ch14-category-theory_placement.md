# Placement: ch14-category-theory.md

## Part in book/
This chapter belongs to **Part VII: Category Theory** (`book/part-07-category-theory/`).

## Sections in book/ that cover this material

| Curriculum section | Book location |
|---|---|
| Categories and functors (§14.1) | `ch33-categories-and-functors/` |
| Natural transformations (§14.2) | `ch34-natural-transformations-and-the-yoneda-lemma/34.1-natural-transformations/` |
| Yoneda lemma (§14.3) | `ch34-natural-transformations-and-the-yoneda-lemma/34.2-the-yoneda-lemma/` |
| Adjoint functors (§14.4) | `ch35-adjoint-functors/` |
| Limits and colimits (§14.5) | `ch36-limits-and-colimits/` |
| Abelian categories (§14.6) | `ch37-abelian-categories/` |

## Content in curriculum/ not fully covered in book/

- The curriculum introduces **monads** briefly in §14.4 as a generalisation of adjunctions and discusses their role in algebra (e.g., "algebras for a monad" = algebras over a monad). The book does not have a dedicated section on monads in Part VII. → Add a subsection on monads and Eilenberg-Moore categories to `ch35` or as a new `ch35.5`.
- The curriculum discusses **Kan extensions** in a remark at the end of §14.5, noting that limits/colimits are special cases. The book does not have a section on Kan extensions. Given their importance in modern category theory (e.g., derived algebraic geometry), a brief treatment would strengthen Part VII.
- The curriculum has worked exercises involving **the universal property of the tensor product** reformulated categorically (as a left adjoint to the internal Hom). The book has this in Part V (module theory) but not cross-referenced from Part VII. Forward/backward pointers would help.

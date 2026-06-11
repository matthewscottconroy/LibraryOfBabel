# Placement: ch12-module-theory.md

## Part in book/
This chapter belongs to **Part V: Module Theory** (`book/part-05-module-theory/`).

## Sections in book/ that cover this material

| Curriculum section | Book location |
|---|---|
| Modules over rings (§12.1) | `ch25-modules-over-rings/25.1-definitions-and-examples/` |
| Submodules and quotient modules (§12.2) | `ch25-modules-over-rings/25.2-submodules-homomorphisms/` |
| Free, projective, injective modules (§12.3) | `ch26-free-projective-and-injective-modules/` |
| Structure theorem over PIDs (§12.4) | `ch27-the-structure-theorem-for-modules-over-pids/` |
| Tensor products (§12.5) | `ch28-tensor-products-of-modules/` |

## Content in curriculum/ not fully covered in book/

- The curriculum discusses the **Nakayama lemma** explicitly as a key tool in commutative algebra (finitely generated modules over local rings). The book has Nakayama in the context of local rings in `ch24` but not in Part V where modules are introduced. → Add a section on Nakayama to `ch25` or `ch26`.
- The curriculum treats **flat modules** and the characterisation of flatness via Tor. The book discusses flatness briefly in `ch26` but does not prove the Tor-characterisation (that $M$ is flat iff $\mathrm{Tor}_1^R(M, -) = 0$). → Expand `ch26` with this characterisation.
- The curriculum includes worked classification of all finitely generated modules over $\mathbb{Z}$ via the structure theorem, and over $k[x]$ (giving Jordan normal form). The book has these in `ch27` but the $k[x]$ application to Jordan form could be more prominently developed.

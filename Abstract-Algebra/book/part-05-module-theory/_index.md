# Part V — Module Theory

**Chapters 25–28**

* * *

One of algebra's recurring strategies is to find the right level of generality — the setting in which a theorem becomes clean and inevitable rather than complicated and case-dependent. Module theory exemplifies this strategy. A module over a ring $R$ is to a vector space what a vector space is to Euclidean space: a further abstraction that strips away the assumption that the scalars are invertible. Replacing "field" by "ring" seems like a small change; the consequences are enormous. Modules over $\mathbb{Z}$ are abelian groups. Modules over a field $F$ are vector spaces. Modules over $F[x]$ are vector spaces equipped with a chosen linear operator — the operator corresponding to multiplication by $x$ — which means that the classification of finitely generated $F[x]$-modules is precisely the theory of canonical forms of linear operators. Modules over a group algebra $k[G]$ are representations of $G$ on a vector space over $k$. A single theorem about finitely generated modules over principal ideal domains simultaneously classifies finitely generated abelian groups, establishes the Jordan normal form, and proves the rational canonical form. Module theory is the unification that makes these disparate subjects recognizable as instances of one.

The rewards of this abstraction come quickly, but they are not free. The essential new phenomenon compared to vector spaces is that without invertible scalars, modules can fail to be free — that is, they need not have a basis. An abelian group like $\mathbb{Z}/2\mathbb{Z}$, viewed as a $\mathbb{Z}$-module, has no subset that is simultaneously linearly independent and spanning in the $\mathbb{Z}$-module sense. This failure of freedom is measured by the torsion submodule, and managing it — understanding when and how modules can be decomposed, and what obstructions prevent them from behaving like vector spaces — is the central problem of module theory. The three special classes studied in Chapter 26 — free, projective, and injective modules — identify the most tractable modules for each of the three homological contexts: constructing maps from, lifting maps to, and extending maps along. These three classes are the building blocks for the resolutions of Part VIII.

Part V develops the theory in four chapters. Chapter 25 introduces the module axioms, submodules, quotients, module homomorphisms, and direct sums, establishing the basic vocabulary that parallels the vector space theory of Part II but now over a general ring. Chapter 26 identifies the three classes of modules central to homological algebra: free modules (those with a basis, admitting the most explicit computations), projective modules (direct summands of free modules, over which all short exact sequences split on the right), and injective modules (over which all short exact sequences split on the left), together with flat modules (those for which tensoring preserves exactness). Chapter 27 proves the structure theorem for finitely generated modules over a principal ideal domain — the master theorem that classifies every such module as a direct sum of cyclic modules, uniquely in two dual forms (the invariant factor decomposition and the primary decomposition) — and draws out its corollaries: the classification of finitely generated abelian groups, the rational canonical form, and (over $F[x]$) the Jordan normal form. Chapter 28 develops the tensor product of modules over a ring, constructs the canonical adjunction between tensor and Hom, and establishes the right-exactness of tensoring together with the failure of left-exactness that motivates the Tor functor. The infrastructure assembled in Part V is the algebraic engine that drives Parts VII through XI.

* * *

## Internal Dependency Map

```
Ch 25 (Modules: Axioms, Submodules, Hom, Direct Sums)
                     |
          ___________|____________
          |                      |
          v                      v
    Ch 26                    Ch 27
(Free/Projective/Injective)  (Structure Thm/PIDs)
          |                      
          v                      
    Ch 28
(Tensor Products of Modules)
```

* * *

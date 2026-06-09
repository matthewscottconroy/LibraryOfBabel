# Chapter 33 — Exercises

## Important Figures

- **Samuel Eilenberg (1913–1998) & Saunders Mac Lane (1909–2005)** — invented category theory in "General Theory of Natural Equivalences" (1945) to formalize "natural" constructions in algebraic topology; a tool invented for one purpose that became foundational
- **Alexander Grothendieck (1928–2014)** — reimagined algebraic geometry via categories (schemes, toposes, sheaves); his use of category theory transformed mathematics
- **F. William Lawvere (1937–2023)** — categorical logic and foundations; topos theory as a foundation for mathematics; *Functorial Semantics of Algebraic Theories* (1963)

## References and Primary Sources

- **S. Eilenberg & S. Mac Lane, "General Theory of Natural Equivalences" (1945)** — *Trans. Amer. Math. Soc.* 58 — the founding paper of category theory
- **S. Mac Lane, *Categories for the Working Mathematician* (2nd ed., Springer, 1998)** — the standard reference
- **T. Leinster, *Basic Category Theory* (Cambridge, 2014; freely available)** — concise and clear modern introduction

## Examples, Applications, and Thought Experiments

- **The category $\mathbf{Set}$** — objects are sets, morphisms are functions; the identity morphism on a set $A$ is $\text{id}_A$; composition is function composition; the paradigmatic concrete category; all other categories are modeled on this
- **The forgetful functor $\mathbf{Grp} \to \mathbf{Set}$** — sends a group to its underlying set and a group homomorphism to the corresponding function; "forgets" the group structure; a functor that is not an isomorphism; right adjoint to the free group functor
- **The fundamental group as a functor** — $\pi_1: \mathbf{Top}_* \to \mathbf{Grp}$: a based space goes to its fundamental group; a continuous map of based spaces induces a group homomorphism; functoriality means $\pi_1(f \circ g) = \pi_1(f) \circ \pi_1(g)$; algebraic topology is the study of functors from topology to algebra
- **Contravariant functors** — the dual space functor $V \mapsto V^* = \text{Hom}(V, k)$ sends a linear map $f: V \to W$ to $f^*: W^* \to V^*$ (going the other direction); a functor that reverses arrows; cohomology theories are contravariant functors

## Exercises

1. Verify that the collection of all sets with functions as morphisms satisfies the category axioms: write out explicitly what associativity of composition and the unit law for identity morphisms assert, and confirm them for functions.

2. Show that the collection of all $R$-modules over a fixed commutative ring $R$, with $R$-module homomorphisms as morphisms, forms a category $\mathbf{Mod}_R$. What is the identity morphism on a module $M$?

3. Let $\mathcal{C}$ be any category. Define the opposite category $\mathcal{C}^{\text{op}}$: objects are the same, but $\text{Hom}_{\mathcal{C}^{\text{op}}}(A, B) = \text{Hom}_{\mathcal{C}}(B, A)$, with composition reversed. Verify that $\mathcal{C}^{\text{op}}$ satisfies the category axioms. What does an epimorphism in $\mathcal{C}$ become in $\mathcal{C}^{\text{op}}$?

4. Show that any functor $F: \mathcal{C} \to \mathcal{D}$ preserves isomorphisms: if $f: A \to B$ is an isomorphism in $\mathcal{C}$, then $F(f): F(A) \to F(B)$ is an isomorphism in $\mathcal{D}$.

5. Let $G$ be a group, viewed as a category $\mathbf{B}G$ with a single object $*$ and $\text{Hom}(*,*) = G$. Describe what a functor $F: \mathbf{B}G \to \mathbf{Set}$ amounts to in terms of classical algebra. (Hint: where does $*$ go, and how does $F$ act on the morphisms?)

6. Show that a monomorphism in $\mathbf{Grp}$ is the same as an injective group homomorphism. Is every epimorphism in $\mathbf{Grp}$ surjective? Give a proof or a counterexample.

7. Define the category $\mathbf{Ring}$ of (unital) rings with ring homomorphisms. Show that the forgetful functor $U: \mathbf{Ring} \to \mathbf{Ab}$ sending a ring to its underlying additive abelian group is indeed a functor.

8. (Challenge) A functor $F: \mathcal{C} \to \mathcal{D}$ is called an *equivalence of categories* if there exists a functor $G: \mathcal{D} \to \mathcal{C}$ and natural isomorphisms $G \circ F \cong \text{Id}_{\mathcal{C}}$ and $F \circ G \cong \text{Id}_{\mathcal{D}}$. Prove that $F$ is an equivalence if and only if $F$ is full, faithful, and essentially surjective (every object of $\mathcal{D}$ is isomorphic to some $F(C)$). You may assume the axiom of choice.

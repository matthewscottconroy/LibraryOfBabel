# Categories

Category theory, founded by Samuel Eilenberg and Saunders Mac Lane in 1945, abstracts the common structure of mathematical constructions: objects connected by morphisms that compose associatively. It is the algebra of mathematical structure itself — and, as this chapter will argue, it is also a form of logic. Groups, spaces, types, and propositions all become instances of a single two-sorted theory.

## Definition

**Definition (Category).** A *category* $\mathcal{C}$ consists of:

- a collection of **objects** $\mathrm{ob}(\mathcal{C})$;
- for each pair of objects $A, B$, a collection $\mathrm{Hom}_{\mathcal{C}}(A, B)$ of **morphisms**, written $f : A \to B$;
- for each object $A$, an **identity morphism** $\mathrm{id}_A : A \to A$;
- a **composition** operation assigning to each $f : A \to B$ and $g : B \to C$ a morphism $g \circ f : A \to C$;

subject to two families of equations:

$$\mathrm{id}_B \circ f = f = f \circ \mathrm{id}_A \qquad \text{(unit laws)}$$

$$h \circ (g \circ f) = (h \circ g) \circ f \qquad \text{(associativity)}$$

Nothing else is assumed. Objects need not have elements; morphisms need not be functions. Everything in this chapter is a consequence of these equations alone.

A logical remark worth pausing on: this definition is an essentially algebraic, first-order theory — two sorts (objects, morphisms), operations $\mathrm{dom}$, $\mathrm{cod}$, $\mathrm{id}$, a partially defined binary operation $\circ$, and universally quantified equations. Category theory, the great abstractor, is itself axiomatized in exactly the equational style of Chapter 3. This is why it formalizes so smoothly in a proof assistant (Section 7) and why its proofs are largely mechanical rewriting.

## Examples

| Category | Objects | Morphisms | Composition |
|----------|---------|-----------|-------------|
| $\mathbf{Set}$ | sets | functions | function composition |
| $\mathbf{Grp}$ | groups | group homomorphisms | composition |
| $\mathbf{Top}$ | topological spaces | continuous maps | composition |
| $\mathbf{Vect}_k$ | vector spaces over $k$ | linear maps | composition |
| a poset $(P, \leq)$ | elements of $P$ | one morphism $x \to y$ iff $x \leq y$ | transitivity |
| a monoid $(M, \cdot, e)$ | a single object $\ast$ | elements of $M$ | multiplication |
| $\mathbf{Rel}$ | sets | binary relations $R \subseteq A \times B$ | relational composition |
| $\mathbf{Mat}_k$ | natural numbers | $m \times n$ matrices as $n \to m$ | matrix multiplication |

The last four repay attention. In a poset there is *at most one* morphism between any two objects: reflexivity supplies identities, transitivity supplies composition, and the axioms hold trivially. A monoid is a category with exactly *one* object: associativity and unit of the monoid are literally the category axioms. Categories thus simultaneously generalize order (many objects, thin homs) and algebra (one object, rich homs). $\mathbf{Rel}$ and $\mathbf{Mat}_k$ show morphisms need not be functions at all.

## Special Morphisms

**Definition (Isomorphism, mono, epi).** A morphism $f : A \to B$ is an **isomorphism** if there exists $g : B \to A$ with $g \circ f = \mathrm{id}_A$ and $f \circ g = \mathrm{id}_B$; a **monomorphism** if it is left-cancellable ($f \circ g = f \circ h \Rightarrow g = h$); an **epimorphism** if it is right-cancellable ($g \circ f = h \circ f \Rightarrow g = h$).

These are element-free rephrasings of injectivity and surjectivity — and in $\mathbf{Set}$ they coincide with them.

**Theorem (Monos in Set).** In $\mathbf{Set}$, $f : A \to B$ is a monomorphism if and only if $f$ is injective.

*Proof.* ($\Leftarrow$) Let $f$ be injective and $f \circ g = f \circ h$ for $g, h : X \to A$. For every $x \in X$, $f(g(x)) = f(h(x))$, so $g(x) = h(x)$; hence $g = h$. ($\Rightarrow$) Let $f$ be mono and $f(a) = f(a')$. Let $1 = \{\ast\}$ and define $g, h : 1 \to A$ by $g(\ast) = a$, $h(\ast) = a'$. Then $f \circ g = f \circ h$, so $g = h$, so $a = a'$. $\square$

The one-point set here is the categorical substitute for "pick an element": elements of $A$ are exactly morphisms $1 \to A$. Dually, epimorphisms in $\mathbf{Set}$ are exactly the surjections. But the correspondence is category-specific:

**Worked example (epi $\neq$ surjective).** In $\mathbf{Ring}$, the inclusion $\iota : \mathbb{Z} \hookrightarrow \mathbb{Q}$ is an epimorphism, though wildly non-surjective. Suppose $g, h : \mathbb{Q} \to R$ are ring homomorphisms agreeing on $\mathbb{Z}$. For $q \neq 0$, $g(1/q)$ is an inverse of $g(q)$, since $g(1/q)\,g(q) = g(1) = 1$. But $g(q) = h(q)$, and inverses in a ring are unique, so $g(1/q) = h(1/q)$. Hence $g(p/q) = g(p)\,g(1/q) = h(p)\,h(1/q) = h(p/q)$, and $g = h$. A morphism is epi when its image *determines* everything, not necessarily when it *is* everything.

## Duality

**Definition (Opposite category).** The **opposite** $\mathcal{C}^{op}$ has the same objects as $\mathcal{C}$, with $\mathrm{Hom}_{\mathcal{C}^{op}}(A, B) = \mathrm{Hom}_{\mathcal{C}}(B, A)$ and composition $g \circ^{op} f = f \circ g$. Identities are unchanged, and the axioms for $\mathcal{C}^{op}$ follow from those for $\mathcal{C}$.

**Metatheorem (Duality principle).** If a statement $\sigma$ in the language of category theory is provable from the category axioms, then so is its dual $\sigma^{\ast}$, obtained by reversing all arrows and the order of all composites.

*Proof sketch.* The set of axioms is invariant under dualization, and the interpretation of $\sigma^{\ast}$ in $\mathcal{C}$ equals the interpretation of $\sigma$ in $\mathcal{C}^{op}$. Since $(\mathcal{C}^{op})^{op} = \mathcal{C}$, any proof of $\sigma$ from the axioms also establishes $\sigma^{\ast}$, by running it in the opposite category. $\square$

This is a genuinely *logical* observation — a metatheorem about the self-duality of an axiom system, in the spirit of Chapter 3 — not a mathematical coincidence. Every theorem is two theorems: mono dualizes to epi, products (Section 3) to coproducts, terminal objects to initial ones. Prove once, conclude twice.

## Size: Small and Locally Small

**Definition.** $\mathcal{C}$ is **locally small** if each $\mathrm{Hom}(A, B)$ is a set, and **small** if additionally $\mathrm{ob}(\mathcal{C})$ is a set.

$\mathbf{Set}$ is locally small but not small: its objects form a proper class, for the Russellian reasons of Chapter 6 — a "set of all sets" is contradictory. The same care governs "the category of all categories": $\mathbf{Cat}$ is officially the category of all *small* categories, so it does not contain itself, echoing exactly how ZFC's cumulative hierarchy dodges Russell's paradox. Grothendieck universes — and Lean's universe polymorphism, as we will see in Section 7 — make this stratification systematic rather than ad hoc.

## Commutative Diagrams

Category theory has a distinctive proof notation: the **commutative diagram**, a directed graph of objects and morphisms in which any two paths with the same endpoints are asserted to have equal composites. The square with sides $f : A \to B$, $g : B \to D$, $h : A \to C$, $k : C \to D$ *commutes* precisely when $g \circ f = k \circ h$. Associativity is what makes the notation sound: a path determines a composite unambiguously, without bracketing. A "diagram chase" is thus an equational proof laid out in two dimensions — a point to remember in Section 7, where such chases turn out to be exactly what proof automation handles best.

## Exercises
See [problems/ch21_category_theory/](../../../problems/ch21_category_theory/)

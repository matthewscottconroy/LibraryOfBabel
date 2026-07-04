# Limits and Colimits

A universal property defines an object not by its internal makeup but by how it relates to every other object — and, by Yoneda, that suffices to determine it up to unique isomorphism. Limits and colimits are the systematic theory of such definitions. They are also, read logically, the categorical form of the connectives.

## Terminal and Initial Objects

**Definition (Terminal, initial).** An object $1$ is **terminal** if for every object $A$ there is exactly one morphism $A \to 1$. Dually, $0$ is **initial** if for every $A$ there is exactly one morphism $0 \to A$.

In $\mathbf{Set}$, every singleton is terminal and $\varnothing$ is initial. In $\mathbf{Grp}$ the trivial group is both. In a poset, terminal = greatest element, initial = least.

## Products and Coproducts

**Definition (Product).** A **product** of objects $A$ and $B$ is an object $A \times B$ equipped with **projections** $\pi_1 : A \times B \to A$ and $\pi_2 : A \times B \to B$ such that for every object $X$ with morphisms $f : X \to A$ and $g : X \to B$, there exists a *unique* morphism $\langle f, g \rangle : X \to A \times B$ satisfying

$$\pi_1 \circ \langle f, g \rangle = f, \qquad \pi_2 \circ \langle f, g \rangle = g.$$

**Worked example (the cartesian product is a product in Set).** Take $A \times B = \{(a,b) : a \in A,\, b \in B\}$ with coordinate projections. Given $f : X \to A$ and $g : X \to B$, define $\langle f, g\rangle(x) = (f(x), g(x))$; both equations hold by construction. For uniqueness, suppose $h : X \to A \times B$ also satisfies $\pi_i \circ h = f, g$. Writing $h(x) = (h_1(x), h_2(x))$, the equations force $h_1 = f$ and $h_2 = g$, so $h = \langle f, g \rangle$. The universal property holds. $\square$

The **coproduct** $A + B$ is the dual: injections $\iota_1 : A \to A + B$, $\iota_2 : B \to A + B$, with a unique $[f, g] : A + B \to X$ for every pair $f : A \to X$, $g : B \to X$. In $\mathbf{Set}$ the coproduct is the disjoint union, and $[f,g]$ is definition by cases. Coproducts vary by category far more than products do: in $\mathbf{Grp}$ the coproduct is the free product, not the disjoint union.

In a poset regarded as a category, the product of $x$ and $y$ is their **meet** $x \wedge y$ (greatest lower bound) and the coproduct is their **join** $x \vee y$: the universal property of the product says exactly "$z \leq x \wedge y$ iff $z \leq x$ and $z \leq y$". A lattice (Chapter 19) is precisely a poset-category with binary products and coproducts; a bounded lattice also has a terminal and an initial object.

## Uniqueness up to Unique Isomorphism

**Theorem (Uniqueness of products).** If $(P, p_1, p_2)$ and $(Q, q_1, q_2)$ are both products of $A$ and $B$, there is a *unique* isomorphism $u : Q \to P$ with $p_i \circ u = q_i$ for $i = 1, 2$.

*Proof.* Applying the universal property of $P$ to the cone $(Q, q_1, q_2)$ yields a unique $u : Q \to P$ with $p_i \circ u = q_i$. Symmetrically, the universal property of $Q$ yields a unique $v : P \to Q$ with $q_i \circ v = p_i$. Consider $u \circ v : P \to P$. It satisfies

$$p_i \circ (u \circ v) = q_i \circ v = p_i \qquad (i = 1, 2),$$

so $u \circ v$ is a morphism from the cone $(P, p_1, p_2)$ to the product $(P, p_1, p_2)$. But $\mathrm{id}_P$ is another such morphism, and the universal property of $P$ says there is *exactly one*. Hence $u \circ v = \mathrm{id}_P$, and symmetrically $v \circ u = \mathrm{id}_Q$. So $u$ is an isomorphism, unique among morphisms commuting with the projections. $\square$

Memorize the shape of this argument — probe each object with the other's universal property, then use *uniqueness* against the identity — because it is the exemplar of all universal-property reasoning. The same proof, word for word, shows that terminal objects, equalizers, pullbacks, and general limits (and dually all colimits) are unique up to unique isomorphism. Universal properties are definite descriptions: like "the $x$ such that $\varphi(x)$" in Chapter 3, they license the definite article.

## Equalizers and Pullbacks

**Definition (Equalizer).** Given parallel morphisms $f, g : A \to B$, an **equalizer** is a morphism $e : E \to A$ with $f \circ e = g \circ e$, universal among such: any $h : X \to A$ with $f \circ h = g \circ h$ factors uniquely through $e$. In $\mathbf{Set}$: $E = \{a \in A : f(a) = g(a)\}$ — equalizers are solution sets.

**Definition (Pullback).** Given $f : A \to C$ and $g : B \to C$, a **pullback** is an object $A \times_C B$ with projections to $A$ and $B$ making the evident square commute, universal among such squares. In $\mathbf{Set}$: $A \times_C B = \{(a, b) : f(a) = g(b)\}$.

Pullbacks interpret *substitution* — Section 4 builds quantifiers as their adjoints — and they define the subobject classifier of Section 6.

## General Limits

**Definition (Limit).** Let $D : \mathcal{J} \to \mathcal{C}$ be a functor (a **diagram** of shape $\mathcal{J}$). A **cone** over $D$ is an object $X$ with morphisms $x_j : X \to D(j)$ for each $j$, such that $D(\alpha) \circ x_j = x_k$ for every $\alpha : j \to k$ in $\mathcal{J}$. A **limit** $\lim D$ is a universal (terminal) cone: every cone factors through it uniquely.

Products are limits over discrete diagrams, terminal objects over the empty diagram, equalizers and pullbacks over small finite shapes. **Colimits** are the dual notion (universal cocones), covering coproducts, initial objects, coequalizers, and pushouts.

**Definition (Complete).** A category is **complete** if it has all small limits, **cocomplete** if it has all small colimits.

**Theorem.** $\mathbf{Set}$ is complete and cocomplete. Concretely, $\lim D = \{(x_j)_j \in \prod_j D(j) : D(\alpha)(x_j) = x_k \text{ for all } \alpha : j \to k\}$ — every limit is carved out of a product by equations.

## Limits as Logic

Consider the poset of propositions under provability: objects are propositions, with one morphism $P \to Q$ exactly when $P \vdash Q$. The lattice operations of Chapter 19 become (co)limits:

| Logic | Category theory |
|-------|-----------------|
| $\top$ (truth) | terminal object |
| $\wedge$ (conjunction) | product |
| $\bot$ (absurdity) | initial object |
| $\vee$ (disjunction) | coproduct |
| $P \vdash Q$ | morphism $P \to Q$ |
| logical equivalence | isomorphism |

The dictionary is exact. The universal property of the product *is* the pair of inference rules for $\wedge$: morphisms $X \to A \times B$ correspond to pairs of morphisms $(X \to A,\ X \to B)$, just as proofs of $A \wedge B$ from $X$ correspond to pairs of proofs ($\wedge$-introduction), with the projections as $\wedge$-elimination. Truth is terminal: everything proves $\top$, uniquely and uninformatively. What the table lacks is implication and the quantifiers — for those we need adjunctions.

## Exercises
See [problems/ch21_category_theory/](../../../problems/ch21_category_theory/)

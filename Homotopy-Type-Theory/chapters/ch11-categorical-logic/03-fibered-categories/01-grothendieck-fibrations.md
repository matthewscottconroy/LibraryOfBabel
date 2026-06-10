# 3.1 Fibered Categories and Grothendieck Fibrations

## An Alternative Perspective

Slice categories model dependent types, but they have a presentation issue: the objects of $\mathcal{C}/\Gamma$ are morphisms in $\mathcal{C}$, which can feel roundabout. Grothendieck fibrations provide a cleaner, more modular way to organize the same information.

The key insight: instead of working inside slice categories, we work with a single *fibration* $p : \mathcal{E} \to \mathcal{B}$ that packages all the slice categories at once. The base $\mathcal{B}$ is the category of contexts; the fibers $\mathcal{E}_\Gamma = p^{-1}(\Gamma)$ are the type families over each context.

## Grothendieck Fibrations

**Definition.** A functor $p : \mathcal{E} \to \mathcal{B}$ is a *Grothendieck fibration* if for every morphism $f : I \to J$ in $\mathcal{B}$ and every object $X \in \mathcal{E}$ with $p(X) = J$, there exists a *cartesian morphism over $f$*: a morphism $\bar{f} : \bar{f}^*(X) \to X$ in $\mathcal{E}$ with $p(\bar{f}) = f$, which is *cartesian*:

For any $g : Y \to X$ in $\mathcal{E}$ and factorization $p(g) = f \circ h$ in $\mathcal{B}$, there exists a unique $\bar{h} : Y \to \bar{f}^*(X)$ in $\mathcal{E}$ with $g = \bar{f} \circ \bar{h}$ and $p(\bar{h}) = h$.

The *fiber* $\mathcal{E}_I = p^{-1}(I)$ is the subcategory of objects over $I$ and morphisms over $\mathsf{id}_I$.

**Intuition:** A Grothendieck fibration is a "family of categories" $\{\mathcal{E}_I\}_{I \in \mathcal{B}}$ parameterized by $\mathcal{B}$, with *reindexing functors* $f^* : \mathcal{E}_J \to \mathcal{E}_I$ for each morphism $f : I \to J$. The cartesian morphism $\bar{f}$ is the "lifting" of $f$ to the total space.

## The Codomain Fibration

**Example.** The *codomain fibration* $\mathsf{cod} : \mathcal{C}^\to \to \mathcal{C}$ sends each morphism $f : A \to B$ to its codomain $B$.

- Objects of $\mathcal{C}^\to$: morphisms in $\mathcal{C}$
- Morphisms in $\mathcal{C}^\to$: commuting squares
- Fiber over $B$: the slice category $\mathcal{C}/B$

The cartesian lift of $f : A \to B$ over an object $g : C \to B$ is the pullback square. So cartesian morphisms over $f$ are pullback squares.

This fibration packages all the slice categories $\{\mathcal{C}/B\}_{B \in \mathcal{C}}$ into one structure.

## Types as Fibrations in Type Theory

In the categorical semantics:
- The base category $\mathcal{B}$: contexts (and substitutions between them)
- The total category $\mathcal{E}$: types in contexts (and terms between them)
- The fibration $p : \mathcal{E} \to \mathcal{B}$: sends each type to its context
- Fiber $\mathcal{E}_\Gamma$: types in context $\Gamma$ (with "morphisms" being terms)
- Cartesian morphism over $\sigma : \Gamma \to \Delta$: substitution, modeled as pullback

**Sections as terms.** A *section* of $p$ over $\Gamma$ is a functor $s : \Gamma \to \mathcal{E}$ with $p \circ s = \mathsf{id}_\Gamma$ — a morphism that picks out an element in each fiber, compatibly. In type theory, a term $\Gamma \vdash a : A$ is a section: it assigns to each substitution $\sigma : \Delta \to \Gamma$ a term $a[\sigma] : A[\sigma]$ in the fiber over $\Delta$.

## The Fundamental Fibration

One fibration is particularly important for the semantics of identity types:

**The fundamental fibration of a category $\mathcal{C}$:** Consider the category $\mathcal{C}^\to$ of morphisms, fibered over $\mathcal{C} \times \mathcal{C}$ by the source-target functor $(s, t) : \mathcal{C}^\to \to \mathcal{C} \times \mathcal{C}$.

For a topological space $X$, this becomes: the total space is the path space $\{(x, y, p) \mid p : x \to y \text{ a path}\}$, fibered over $X \times X$ by the endpoint map.

The fiber over $(x, y)$ is exactly the path space from $x$ to $y$. The reflexivity map $r : X \to X^\to$ sends $x$ to the constant path at $x$.

**This is the topological model of the identity type:**
- Identity type $a =_A b$ ← fiber of the path fibration over $(a, b)$
- Reflexivity $\mathsf{refl}_a$ ← constant path at $a$
- J rule ← lifting property of the fibration

The identity type in type theory is directly modeled by path spaces in topology!

## Display Map Categories

A *display map category* is a category $\mathcal{C}$ with a distinguished class $\mathcal{D}$ of *display maps* (morphisms representing "type projections"), satisfying:
- Display maps are closed under pullback (substitution preserves types)
- Every morphism factors as a display map composed with something else (types and terms separate)
- The class $\mathcal{D}$ is stable under composition (type families can be composed)

In $\mathbf{Set}$: every morphism is a display map. In topology: fibrations are the display maps. In algebraic geometry: flat proper morphisms are display maps.

The display map framework unifies many semantics: choosing different classes of display maps gives different models of type theory.

## Categories with Families (CwFs)

Dybjer's *categories with families* is an approach that directly mirrors the type-theoretic judgments:

**Definition.** A *CwF* consists of:
- A category $\mathcal{C}$ (contexts and substitutions)
- A functor $\mathsf{Ty} : \mathcal{C}^{op} \to \mathbf{Set}$ (types in each context — a presheaf)
- For each $\Gamma$ and $A \in \mathsf{Ty}(\Gamma)$, a set $\mathsf{Tm}(\Gamma, A)$ (terms of type $A$ in context $\Gamma$)
- *Context extension:* for each $\Gamma$ and $A \in \mathsf{Ty}(\Gamma)$, a context $\Gamma.A$ with a projection $p : \Gamma.A \to \Gamma$ and a "generic term" $q \in \mathsf{Tm}(\Gamma.A, A[p])$
- *Comprehension:* for each term $a \in \mathsf{Tm}(\Gamma, A)$, a substitution $(a) : \Gamma \to \Gamma.A$

satisfying appropriate coherence conditions.

CwFs directly capture the typing judgments:
- $\mathsf{Ty}(\Gamma)$: types in context $\Gamma$
- $\mathsf{Tm}(\Gamma, A)$: terms of type $A$ in context $\Gamma$
- Context extension $\Gamma.A$: the context "$\Gamma$ plus a variable of type $A$"
- The generic term $q$: the "last variable" in the extended context

**Every CwF gives an LCCC** (after suitable construction). The CwF framework makes the coherence conditions explicit, avoiding the coherence problem of LCCC semantics.

## The Identity Type in Fibered Semantics

In a fibered category, the identity type has a clean semantics:

**Definition.** A fibration $p : \mathcal{E} \to \mathcal{B}$ has *identity types* if for every object $A$ in a fiber $\mathcal{E}_\Gamma$, there's a fibered adjunction:

$$\delta_A^* \dashv \mathsf{Id}_A$$

where $\delta_A : A \to A \times A$ is the diagonal and $\mathsf{Id}_A$ is the "path space" object with a reflexivity map $r : A \to \mathsf{Id}_A$ over $\delta_A$.

The identity type is the object $\mathsf{Id}_A$ with:
- A map to $A \times A$ (giving the endpoints of each path)
- A section $r$ of the map to $A$ via the diagonal (giving reflexivity paths)
- A universal property (the J rule: any property proved for reflexivity paths extends to all paths)

**In Kan simplicial sets:** The identity type of $A$ is the simplicial path space $A^{[0,1]}$ (paths in $A$). The reflexivity map sends each point to its constant path. The universal property is the Kan horn-filling condition.

## From Fibrations to HoTT

The fibration perspective naturally suggests:

1. **Types as spaces:** A type $A$ is a space (Kan complex). Its elements are points.

2. **Type families as fibrations:** A family $B : A \to \mathsf{Type}$ is a fibration over the space $A$. Each fiber $B(a)$ is the type over the point $a$.

3. **Terms as sections:** A term $x : A \vdash b : B(x)$ is a section of the fibration — a continuous map that picks a point in each fiber.

4. **Identity type as path space:** The identity type $a = b$ is the path space from $a$ to $b$. Reflexivity is the constant path. J is path induction.

This is HoTT's central geometric picture, and it's completely natural from the fibration perspective. The type theory's rules are the categorical rules for fibrations; the geometric intuition is not imposed from outside but emerges from the structure itself.

## Summary

| Fibration Framework | Type Theory |
|---|---|
| Fibration $p : \mathcal{E} \to \mathcal{B}$ | Types organized over contexts |
| Fiber $\mathcal{E}_\Gamma$ | Types in context $\Gamma$ |
| Cartesian morphism over $\sigma$ | Substitution = pullback |
| Section of $p$ | Term |
| Fundamental fibration | Path space = identity type |
| CwF | Direct formalization of typing judgments |
| Display map | "Type projection" |

Fibrations provide the cleanest framework for the categorical semantics of type theory. They separate the concerns of "what are types" (fibers) and "what is substitution" (cartesian lifts) clearly, and they model identity types via path spaces in a geometrically transparent way.

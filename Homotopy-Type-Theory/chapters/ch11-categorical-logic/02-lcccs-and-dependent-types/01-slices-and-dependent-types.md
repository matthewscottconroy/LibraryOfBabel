# 2.1 LCCCs and Dependent Types

## The Key Idea: Slice Categories Model Type Families

In STLC, a context $\Gamma$ modeled as a product $\llbracket \Gamma \rrbracket$, and types were objects of the category. In dependent type theory, a type $A$ in context $\Gamma$ becomes a type *over* $\Gamma$ — a morphism whose domain is the "total space" and whose codomain is the context.

The key shift: **types in context $\Gamma$ are objects of the slice category $\mathcal{C}/\Gamma$**.

An object of $\mathcal{C}/\Gamma$ is a morphism $f : A \to \Gamma$ — a "display map" or "projection" of the total space $A$ down to the context $\Gamma$. The fiber $f^{-1}(\gamma)$ over a point $\gamma \in \Gamma$ is the specific type $A(\gamma)$ you get when you substitute $\gamma$ for the context variables.

**Example in $\mathbf{Set}$.** A type family $B : \Gamma \to \mathsf{Type}$ in $\mathbf{Set}$ corresponds to the total space $\sum_{\gamma:\Gamma} B(\gamma) = \{(\gamma, b) \mid \gamma \in \Gamma, b \in B(\gamma)\}$ with the projection $\pi_1 : \sum_\gamma B(\gamma) \to \Gamma$. This is an object of $\mathbf{Set}/\Gamma$.

The fiber over $\gamma$ is $\pi_1^{-1}(\gamma) = \{(\gamma, b) \mid b \in B(\gamma)\} \cong B(\gamma)$.

## Substitution as Pullback

In type theory, substitution $A[\sigma]$ (substituting a term/morphism $\sigma : \Delta \to \Gamma$ into a type $A$ over $\Gamma$) is modeled by *pullback* in the category.

Given $f : A \to \Gamma$ (a type over $\Gamma$) and $\sigma : \Delta \to \Gamma$ (a substitution), the *reindexed type* $\sigma^*(f) : \sigma^*A \to \Delta$ is the pullback:

$$\begin{array}{ccc} \sigma^*A & \to & A \\ \sigma^*(f)\downarrow & & \downarrow f \\ \Delta & \xrightarrow{\sigma} & \Gamma \end{array}$$

The pullback $\sigma^*A$ is exactly the type family over $\Delta$ obtained by substituting $\sigma$ into $A$: the fiber over $\delta \in \Delta$ is $A(\sigma(\delta))$ — the type at the point $\sigma(\delta)$ in $\Gamma$.

In type theory: $A[\sigma](\delta) = A(\sigma(\delta))$. Categorically: the fiber of $\sigma^*A$ over $\delta$ is the fiber of $A$ over $\sigma(\delta)$.

**The functor $\sigma^* : \mathcal{C}/\Gamma \to \mathcal{C}/\Delta$.** Pulling back along $\sigma$ gives a functor between slice categories. This is the categorical model of substitution: it's a functor, not just a function.

## LCCCs: The Full Definition

**Definition.** A category $\mathcal{C}$ is *locally cartesian closed (LCCC)* if every slice category $\mathcal{C}/\Gamma$ is a cartesian closed category.

Being an LCCC means:
1. Every $\mathcal{C}/\Gamma$ has a terminal object $(\Gamma, \mathsf{id}_\Gamma)$
2. Every $\mathcal{C}/\Gamma$ has binary products (pullbacks over $\Gamma$)
3. Every $\mathcal{C}/\Gamma$ has exponentials $[f, g]_\Gamma$ for $f, g : \mathcal{C}/\Gamma$

**$\mathbf{Set}$ is an LCCC.** In $\mathbf{Set}/\Gamma$:
- Terminal object: $(\Gamma, \mathsf{id}_\Gamma)$
- Products: the pullback $A \times_\Gamma B = \{(\gamma, a, b) \mid f(a) = \gamma = g(b)\}$
- Exponentials: $[f, g]_\Gamma$ is the type family $\gamma \mapsto [A(\gamma), B(\gamma)]$ (functions between fibers)

## The Dependent Type Formers in LCCCs

Here's how Σ, Π, and substitution are modeled:

### Σ Types as Products in Slices

In the slice $\mathcal{C}/\Gamma$, the product of $(A \to \Gamma)$ and $(B \to \Gamma)$ is their pullback over $\Gamma$:

$$A \times_\Gamma B = \{(\gamma, a, b) \mid f(a) = \gamma = g(b)\}$$

This is exactly $\sum_{\gamma:\Gamma} A(\gamma) \times B(\gamma)$ — the type family whose fiber over $\gamma$ is $A(\gamma) \times B(\gamma)$.

But for *dependent* Σ types, where $B$ depends on $A$, we need products of:
- $(A \to \Gamma)$: the type family $A$ over $\Gamma$
- $(C \to A)$: the type family $C$ over $A$ (which depends on elements of $A$, hence on $\Gamma$ via $A$)

Their product in $\mathcal{C}/\Gamma$ is the composite $A \to \Gamma$ with the total space of $C$ over $A$. More precisely, $\Sigma_{f}(C \to A)$ is the composition $(C \to A \to \Gamma)$ viewed as an object of $\mathcal{C}/\Gamma$.

**The Σ functor:** $\Sigma_f : \mathcal{C}/A \to \mathcal{C}/\Gamma$ is defined by composition with $f : A \to \Gamma$. It sends a type over $A$ to a type over $\Gamma$ (by composing the projection with $f$).

### Π Types as Exponentials in Slices

In $\mathcal{C}/\Gamma$, the exponential $[f, g]$ (for $f : A \to \Gamma$ and $g : B \to \Gamma$) is the "internal hom" — the type family over $\Gamma$ whose fiber over $\gamma$ is the set of functions $A(\gamma) \to B(\gamma)$.

For the *dependent* Π type, where $B$ depends on $A$ (and hence on $\Gamma$ via $A$), the Π type is the *right adjoint* to the pullback functor $f^* : \mathcal{C}/\Gamma \to \mathcal{C}/A$.

**The Π functor:** $\Pi_f : \mathcal{C}/A \to \mathcal{C}/\Gamma$ is the right adjoint to $f^*$. It sends a type over $A$ (a dependent type depending on $A$) to the "dependent function type" over $\Gamma$.

The adjunction $f^* \dashv \Pi_f$ is the categorical statement of the type-theoretic adjunction: a term $\Gamma, x : A \vdash t : B(x)$ (a dependent function) corresponds to a morphism $A \to B$ over $\Gamma$.

### The Adjoint Triple

The three operations form an adjoint triple:

$$\Sigma_f \dashv f^* \dashv \Pi_f$$

- $\Sigma_f$: left adjoint — pushforward along $f$ (Σ type, left adjoint to reindexing)
- $f^*$: middle — pullback along $f$ (substitution)
- $\Pi_f$: right adjoint — right adjoint to reindexing (Π type)

This triple is the categorical expression of the fundamental operations of dependent type theory. The adjunctions encode the logical facts:
- $\Sigma_f \dashv f^*$: a term of a Σ type is the same as a pair
- $f^* \dashv \Pi_f$: a term of a Π type is the same as a function (currying for dependent types)

## The Substitution Problem

There's a coherence issue in making the categorical semantics precise: in category theory, pullbacks are defined up to isomorphism, but in type theory, substitution is *strict* (it's a syntactic operation that produces definitionally equal, not just isomorphic, terms).

Concretely: $(A[\sigma])[\tau]$ and $A[\sigma \circ \tau]$ are definitionally equal in type theory (substitution is associative). But in a category, pulling back along $\sigma$ and then $\tau$ gives a pullback that's only isomorphic to the pullback along $\sigma \circ \tau$, not equal.

**Solutions to the coherence problem:**

**1. Contextual categories (Cartmell 1978).** A contextual category is a category with a strict version of the slice structure, matching the syntactic structure of type theory exactly. It avoids the coherence issue by construction.

**2. Categories with families (Dybjer 1995).** A CwF is a functor $F : \mathcal{C}^{op} \to \mathbf{Fam}$ (where $\mathbf{Fam}$ is the category of families of sets). This directly captures the typing rules without going through slice categories.

**3. Comprehension categories (Jacobs 1993).** A functor $p : \mathcal{E} \to \mathcal{C}$ with extra structure. Objects of $\mathcal{E}$ are types in contexts; morphisms are terms.

**4. Split fibrations / displayed categories.** A strict version of Grothendieck fibrations that resolves the coherence issue by requiring strict (not just up-to-isomorphism) functoriality.

**In practice:** Proof assistants like Agda and Lean 4 handle this at the implementation level by maintaining strict definitional equality. The categorical semantics is cleaner when you work with LCCCs and allow the isomorphism mismatch, understanding that it can be resolved via one of the above frameworks.

## The Seely Theorem

**Theorem (Seely 1984, Hofmann 1995, Dybjer 1995).** The following are equivalent (up to the appropriate notion of equivalence):
1. Models of dependent type theory (CwFs, contextual categories, etc.)
2. LCCCs (with additional data for inductive types and universes)

This theorem makes precise the informal claim "dependent type theory = LCCC." The equivalence is:
- Every LCCC gives a model of dependent type theory
- Every model of dependent type theory gives an LCCC (the classifying LCCC)

**Caveat:** The correspondence is not perfect without extra conditions (coherence, strictness). The full theorem requires working with *split* LCCCs (strict coherence conditions) or other structurally cleaner presentations.

## What's Missing: Identity Types

The Seely correspondence covers dependent type theory *without* the identity type. Adding the identity type requires more structure.

The identity type $a =_A b$ is a type family over $A \times A$: for each pair $(a, b)$, it gives the type of proofs that $a = b$. Categorically, this should be a morphism $\mathsf{Id}_A \to A \times A$.

In $\mathbf{Set}$, this would be the diagonal $\Delta : A \to A \times A$ (with $\mathsf{Id}_A = A$ and the identity proof being $\mathsf{refl}_a$). But this makes the identity type trivial (only one element in each fiber where $a = b$), giving UIP.

To model the *intensional* identity type (where UIP can fail), we need *path objects* and a model category structure. This is what the next sections (3 and 5) develop.

## Summary

| Type Theory | LCCC |
|---|---|
| Context $\Gamma$ | Object of $\mathcal{C}$ |
| Type $\Gamma \vdash A\ \mathsf{type}$ | Object of $\mathcal{C}/\Gamma$ |
| Term $\Gamma \vdash a : A$ | Section $\Gamma \to A$ (in $\mathcal{C}/\Gamma$) |
| Substitution $A[\sigma]$ | Pullback $\sigma^* A$ |
| Σ type $\sum_{x:\Gamma} A(x)$ | $\Sigma_f$ functor (composition) |
| Π type $\prod_{x:\Gamma} A(x)$ | $\Pi_f$ functor (right adjoint to pullback) |
| Substitution rule $f^* \dashv \Pi_f$ | Adjunction in slice categories |

The LCCC semantics is elegant and mathematically clean. It reveals that the type-theoretic rules are not arbitrary — they're exactly the rules that hold in any LCCC. Understanding this correspondence gives deep insight into why type theory has the rules it has, and it provides the tools to build models and prove metatheorems.

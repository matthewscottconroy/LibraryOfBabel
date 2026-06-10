# 1.1 Cartesian Closed Categories and STLC

## Contexts as Products

The first step in categorical semantics: how do contexts translate to categories?

In STLC, a context is a list of variable-type pairs: $\Gamma = x_1 : A_1, x_2 : A_2, \ldots, x_n : A_n$. A term in context $\Gamma$ is a term $\Gamma \vdash t : B$ — a term that can use the variables of $\Gamma$.

Categorically, the context $\Gamma$ corresponds to the product $\llbracket \Gamma \rrbracket = A_1 \times A_2 \times \cdots \times A_n$ (with $\llbracket () \rrbracket = \mathbf{1}$ for the empty context). A term $\Gamma \vdash t : B$ becomes a morphism $\llbracket t \rrbracket : \llbracket \Gamma \rrbracket \to \llbracket B \rrbracket$.

This works because a term in context $\Gamma$ takes inputs from all the variables in $\Gamma$ and produces an output. A morphism from $\llbracket \Gamma \rrbracket$ to $B$ does exactly the same thing.

## Cartesian Categories: Modeling Contexts

For the "context = product" idea to work, we need binary products and a terminal object. This is a *cartesian category*.

**Definition.** A *cartesian category* is a category with:
- A *terminal object* $\mathbf{1}$ (modeling the empty context)
- *Binary products* $A \times B$ for all $A, B$ (modeling context extension)

The projections $\pi_1 : A \times B \to A$ and $\pi_2 : A \times B \to B$ model the weakening and variable-projection rules.

**Substitution as morphism composition.** A substitution $\sigma : \Gamma \to \Delta$ (mapping variables of $\Delta$ to terms in $\Gamma$) becomes a morphism $\llbracket \sigma \rrbracket : \llbracket \Gamma \rrbracket \to \llbracket \Delta \rrbracket$. If $\Gamma \vdash t : B$ (a term in context $\Gamma$), then $\llbracket t \rrbracket \circ \llbracket \sigma \rrbracket : \llbracket \Delta \rrbracket \to \llbracket B \rrbracket$ is the substituted term $t[\sigma]$.

This is the categorical rendition of the substitution lemma: the interpretation of $t[\sigma]$ is the composition of $\llbracket t \rrbracket$ and $\llbracket \sigma \rrbracket$.

## Adding Function Types: Cartesian Closed Categories

In STLC, we can form function types $A \to B$. For these to have a categorical model, we need *exponential objects*.

**Definition.** A cartesian category $\mathcal{C}$ is *cartesian closed (CCC)* if for every object $A$, the functor $(-) \times A : \mathcal{C} \to \mathcal{C}$ (product with $A$) has a right adjoint, written $[A, -]$ or $A \Rightarrow -$.

The adjunction gives a natural bijection:

$$\mathsf{Hom}(C \times A, B) \cong \mathsf{Hom}(C, [A, B])$$

This is *currying*: a function from $C \times A$ to $B$ is the same as a function from $C$ to the "function object" $[A, B]$.

**Explicitly:** The adjunction comes with:
- *Evaluation:* $\mathsf{ev} : [A, B] \times A \to B$ (apply a function to an argument)
- *Currying:* for each $f : C \times A \to B$, a unique $\lambda(f) : C \to [A, B]$
- satisfying $\mathsf{ev} \circ (\lambda(f) \times \mathsf{id}_A) = f$

The function type $[A, B]$ is the *internal hom* or *exponential object*.

## The STLC-CCC Correspondence

**Theorem (STLC = CCC).** There is a bijection between:
- Models of STLC (up to logical equivalence)
- CCCs (up to equivalence of categories)

More precisely:
- Every CCC $\mathcal{C}$ gives a model of STLC: interpret types as objects, terms as morphisms, $\times$ as products, $\to$ as exponentials.
- Every model of STLC gives a CCC: the *syntactic category* has contexts as objects and (classes of) terms as morphisms.

The interpretation is:

| STLC | CCC |
|---|---|
| Type $A$ | Object $\llbracket A \rrbracket$ |
| Context $\Gamma$ | Object $\llbracket \Gamma \rrbracket = \prod_i A_i$ |
| Term $\Gamma \vdash t : B$ | Morphism $\llbracket t \rrbracket : \llbracket \Gamma \rrbracket \to \llbracket B \rrbracket$ |
| Unit type $\mathbf{1}$ | Terminal object $\mathbf{1}$ |
| Product type $A \times B$ | Categorical product $\llbracket A \rrbracket \times \llbracket B \rrbracket$ |
| Function type $A \to B$ | Exponential $[\llbracket A \rrbracket, \llbracket B \rrbracket]$ |
| $\lambda$-abstraction | Currying $\lambda$ |
| Function application | Evaluation $\mathsf{ev}$ |
| $\beta$-reduction | The defining equation of currying |
| $\eta$-expansion | Uniqueness in the universal property |

**The $\beta$-rule categorically:** If $f : \Gamma \times A \to B$ (representing $\lambda x. t$) and $a : \Gamma \to A$ (an argument), then $f \circ (\mathsf{id} \times a) = \mathsf{ev} \circ (\lambda(f) \times a)$. When you apply the curried function $\lambda(f)$ to $a$ and then evaluate, you get $f$ applied to $(\mathsf{id}, a)$. This is the categorical $\beta$-rule.

**The $\eta$-rule categorically:** For any $g : \Gamma \to [A, B]$, $\lambda(\mathsf{ev} \circ (g \times \mathsf{id}_A)) = g$. The unique morphism characterized by its evaluations is $g$ itself. This is the categorical $\eta$-rule.

## The Syntactic Category

Given a type theory $\mathbf{T}$, its *syntactic category* (or *classifying category* or *term model*) $\mathcal{C}[\mathbf{T}]$ is:

- **Objects:** Contexts $\Gamma$ of $\mathbf{T}$ (up to definitional equality)
- **Morphisms** $\Gamma \to \Delta$: Substitutions $\sigma$ from $\Gamma$ into $\Delta$ (sequences of terms of the types in $\Delta$, using variables from $\Gamma$)
- **Composition:** Substitution composition
- **Identity:** The identity substitution (each variable maps to itself)

For STLC, the syntactic category is a CCC, with:
- Terminal object: the empty context $()$
- Product $\Gamma \times A$: the extended context $\Gamma, x : A$
- Exponential $[A, B]$: the context containing a single variable of type $A \to B$

**The universal property of the syntactic category:** Every model of STLC (every CCC with designated objects for the base types) is determined by a unique (up to isomorphism) CCC functor from $\mathcal{C}[\text{STLC}]$ to the model. The syntactic category is the "initial" model.

## Examples

**$\mathbf{Set}$ as a model of STLC.** Interpret base types as sets; $\times$ as Cartesian product; $\to$ as function sets $B^A$. Every theorem of STLC holds in $\mathbf{Set}$. This gives soundness.

**Presheaf categories.** For any small category $\mathcal{C}$, the presheaf category $[\mathcal{C}^{op}, \mathbf{Set}]$ is a CCC. This gives a family of models of STLC parameterized by $\mathcal{C}$.

**The Scott domain model.** For reasoning about recursive programs, replace $\mathbf{Set}$ with the category of Scott domains (partially ordered sets with certain completeness properties). This model allows for partial functions and infinite computations, modeling general recursion.

**Realizability models.** The *effective topos* is a CCC where the objects are "realizable" — their elements must be computable. This models constructive/computable mathematics.

## Product Types and Context Extension

In STLC, the context extension rule says: if you have a term $\Gamma \vdash t : B$ and add a new variable $x : A$, you get $\Gamma, x : A \vdash t : B$ (weakening). Categorically, this is precomposition with the projection $\pi_1 : \Gamma \times A \to \Gamma$: the morphism $\llbracket t \rrbracket \circ \pi_1 : \Gamma \times A \to B$ represents $t$ in the extended context.

The variable $x$ in context $\Gamma, x : A$ corresponds to the projection $\pi_2 : \Gamma \times A \to A$. The term $\Gamma, x : A \vdash x : A$ is the projection morphism.

This categorical picture makes the "structural rules" (weakening, contraction, exchange) visible as categorical operations (precomposition with projections, diagonals, and swaps respectively).

## From CCCs to LCCCs: The Next Step

CCCs model STLC well. But they don't model dependent types: in a CCC, there's no analog of a type family $B : A \to \mathsf{Type}$ where the type of the second component of a pair depends on the first.

For dependent types, we need:
- Types to depend on terms (not just other types)
- The "context = product" idea to become "context = object in a slice category"
- Substitution to become pullback

This is what locally cartesian closed categories (LCCCs) provide, developed in the next section.

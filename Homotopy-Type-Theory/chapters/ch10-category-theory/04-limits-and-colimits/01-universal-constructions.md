# 4.1 Limits and Colimits

## The Idea: Universal Cones

Limits and colimits are the most important constructions in category theory. They generalize: products and coproducts, equalizers and coequalizers, pullbacks and pushouts, terminal and initial objects. All of these are "limit" or "colimit" constructions.

The key idea is a *universal property*: a limit is an object that receives maps from all "cones" over a diagram in a unique way.

## Terminal and Initial Objects

The simplest limits and colimits.

**Terminal object.** An object $\mathbf{1}$ is *terminal* if for every object $A$, there exists a unique morphism $!_A : A \to \mathbf{1}$.

Examples:
- In $\mathbf{Set}$: any singleton $\{*\}$
- In $\mathbf{Grp}$: the trivial group $\{e\}$
- In $\mathbf{Top}$: any one-point space
- In a preorder: the maximum element (if it exists)
- In type theory: the unit type $\mathbf{1}$, with the unique map $\mathsf{tt}$ from any type

**Theorem.** Terminal objects are unique up to unique isomorphism. If $\mathbf{1}$ and $\mathbf{1}'$ are both terminal, then the unique maps $!: \mathbf{1} \to \mathbf{1}'$ and $!': \mathbf{1}' \to \mathbf{1}$ are mutually inverse isomorphisms.

*Proof.* $!' \circ ! : \mathbf{1} \to \mathbf{1}$ is a morphism from $\mathbf{1}$ to $\mathbf{1}$. Since $\mathbf{1}$ is terminal, the only such morphism is $\mathsf{id}_\mathbf{1}$. So $!' \circ ! = \mathsf{id}_\mathbf{1}$, and similarly $! \circ !' = \mathsf{id}_{\mathbf{1}'}$. $\square$

**Initial object.** An object $\mathbf{0}$ is *initial* if for every $A$, there exists a unique morphism $\mathbf{0} \to A$.

Examples:
- In $\mathbf{Set}$: the empty set $\emptyset$
- In $\mathbf{Grp}$: the trivial group (it's both terminal and initial — a *zero object*)
- In type theory: the empty type $\mathbf{0}$, with the unique map (ex falso) from $\mathbf{0}$ to any type

## Products

**Definition.** The *product* of objects $A$ and $B$ is an object $A \times B$ with morphisms $\pi_1 : A \times B \to A$ (first projection) and $\pi_2 : A \times B \to B$ (second projection) satisfying: for every object $C$ and morphisms $f : C \to A$, $g : C \to B$, there is a unique morphism $\langle f, g \rangle : C \to A \times B$ with $\pi_1 \circ \langle f, g \rangle = f$ and $\pi_2 \circ \langle f, g \rangle = g$.

The universal property says: giving a map into $A \times B$ is the same as giving a map into $A$ and a map into $B$.

In type theory, the product type $A \times B$ satisfies exactly this universal property: $\langle f, g \rangle = \lambda c. (f\, c, g\, c)$, and $\pi_1 \langle f, g \rangle = f$, $\pi_2 \langle f, g \rangle = g$ by computation.

**Coproduct.** The dual: $A + B$ with injections $\iota_1 : A \to A + B$ and $\iota_2 : B \to A + B$, universal among objects receiving maps from both $A$ and $B$.

In type theory, the sum type $A + B$ is the coproduct: $[f, g] = \mathsf{case}(-, f, g)$.

## Equalizers

**Definition.** The *equalizer* of two morphisms $f, g : A \to B$ is an object $E$ with a morphism $e : E \to A$ such that $f \circ e = g \circ e$, universal with this property: any $h : C \to A$ with $f \circ h = g \circ h$ factors uniquely through $E$.

$$\begin{array}{ccccc} E & \xrightarrow{e} & A & \underset{g}{\overset{f}{\rightrightarrows}} & B \end{array}$$

Example in $\mathbf{Set}$: $E = \{a \in A \mid f(a) = g(a)\}$ with inclusion $e : E \hookrightarrow A$.

In type theory, the equalizer of $f, g : A \to B$ is $\sum_{a:A} f(a) = g(a)$ with projection $\pi_1 : \sum_{a:A} f(a) = g(a) \to A$.

**Coequalizer.** The dual: the coequalizer of $f, g : A \to B$ is a $C$ with a morphism $q : B \to C$ such that $q \circ f = q \circ g$, universal among such $C$.

Example in $\mathbf{Set}$: the quotient $B / \sim$ where $f(a) \sim g(a)$ for all $a$.

In HoTT, coequalizers (and quotients in general) are Higher Inductive Types: you add a path constructor that identifies $f(a)$ and $g(a)$.

## Pullbacks

**Definition.** The *pullback* of morphisms $f : A \to C$ and $g : B \to C$ is an object $A \times_C B$ with morphisms $p_1 : A \times_C B \to A$ and $p_2 : A \times_C B \to B$ such that $f \circ p_1 = g \circ p_2$, universal with this property.

$$\begin{array}{ccc} A \times_C B & \xrightarrow{p_2} & B \\ p_1\downarrow & & \downarrow g \\ A & \xrightarrow{f} & C \end{array}$$

Example in $\mathbf{Set}$: $A \times_C B = \{(a, b) \in A \times B \mid f(a) = g(b)\}$.

In type theory: $A \times_C B = \sum_{a:A} \sum_{b:B} f(a) = g(b)$.

Pullbacks are fundamental in algebraic topology (fiber products), algebraic geometry (scheme fiber products), and logic (substitution).

**In dependent type theory:** If $f : A \to C$ and $g = \mathsf{id}_C$, the pullback of $f$ along $\mathsf{id}_C$ is $\sum_{a:A} f(a) = c$ — the fiber of $f$ over $c$. This is the type-theoretic fiber, fundamental in the study of fibrations.

## The General Definition: Limits

All of the above (terminal objects, products, equalizers, pullbacks) are special cases of the general notion of *limit*.

**Definition.** A *diagram* $D : \mathcal{J} \to \mathcal{C}$ is a functor from a small category $\mathcal{J}$ (the *indexing category* or *shape*) to $\mathcal{C}$.

A *cone over $D$* is an object $L \in \mathcal{C}$ together with morphisms $\lambda_j : L \to D(j)$ for each $j \in \mathcal{J}$, such that for every morphism $\phi : j \to k$ in $\mathcal{J}$, $D(\phi) \circ \lambda_j = \lambda_k$.

The *limit* of $D$ is a cone $(L, \lambda)$ that is universal: for every other cone $(C, \gamma)$, there exists a unique morphism $u : C \to L$ such that $\lambda_j \circ u = \gamma_j$ for all $j$.

**Examples as limits:**
- $\mathcal{J} = \emptyset$ (empty category): limit is the terminal object
- $\mathcal{J} = \bullet\quad\bullet$ (discrete two-point category): limit is the product
- $\mathcal{J} = \bullet \rightrightarrows \bullet$ (two parallel arrows): limit is the equalizer
- $\mathcal{J} = \bullet \to \bullet \leftarrow \bullet$ (cospan): limit is the pullback

**Colimits** are dual: cocones instead of cones, initial instead of terminal, universal morphism going *out* rather than in.

## Limits and Colimits in Type Theory

Every limit construction in $\mathbf{Set}$ has a type-theoretic counterpart:

| Limit/Colimit | $\mathbf{Set}$ | Type Theory |
|---|---|---|
| Terminal | $\{*\}$ | $\mathbf{1}$ |
| Initial | $\emptyset$ | $\mathbf{0}$ |
| Product | $A \times B$ | $A \times B$ |
| Coproduct | $A \sqcup B$ | $A + B$ |
| Equalizer | $\{a \mid f(a) = g(a)\}$ | $\sum_{a:A} f(a) = g(a)$ |
| Coequalizer | $A / \sim$ | HIT with path constructor |
| Pullback | $\{(a,b) \mid f(a) = g(b)\}$ | $\sum_{a:A} \sum_{b:B} f(a) = g(b)$ |
| Pushout | Cospan coequalizer | HIT |

The pattern: limits in type theory are Σ types with equality constraints; colimits require HIT path constructors (or at least propositional truncation for set-level colimits).

## Preservation of Limits by Adjoints

**Theorem.** Right adjoints preserve limits. Left adjoints preserve colimits.

More precisely: if $G : \mathcal{D} \to \mathcal{C}$ is a right adjoint (with left adjoint $F$), and $D : \mathcal{J} \to \mathcal{D}$ is a diagram with limit $\varprojlim D$, then $G(\varprojlim D)$ is the limit of $G \circ D$.

*Proof sketch.* By the universal property of limits and the bijection of the adjunction.

**Examples:**
- The forgetful functor $\mathbf{Grp} \to \mathbf{Set}$ (right adjoint to free group) preserves limits: the product of groups has the same underlying set as the product of the underlying sets.
- The free functor $\mathbf{Set} \to \mathbf{Grp}$ (left adjoint) preserves colimits: the free group on a coproduct $F(A \sqcup B) \cong F(A) * F(B)$ (free product).

In type theory: function types $A \to -$ (right adjoint to $- \times A$) preserve limits. Products $- \times A$ (left adjoint to $[A, -]$) preserve colimits.

## Complete and Cocomplete Categories

A category is *complete* if all small limits exist; *cocomplete* if all small colimits exist.

- $\mathbf{Set}$ is complete and cocomplete.
- $\mathbf{Grp}$ is complete and cocomplete.
- $\mathbf{Top}$ is complete and cocomplete.
- Presheaf categories $[\mathcal{C}^{op}, \mathbf{Set}]$ are complete and cocomplete.

For type theory: the *syntactic category* of MLTT (the category whose objects are types and morphisms are terms) has all limits. Whether it has all colimits depends on what type formers are available — coproducts require sum types, coequalizers require quotient types (HITs), etc.

## Kan Extensions: The Universal Construction

The most general limit/colimit construction is the *Kan extension*. Given functors $F : \mathcal{C} \to \mathcal{D}$ and $K : \mathcal{C} \to \mathcal{E}$, the left Kan extension $\mathsf{Lan}_K F : \mathcal{E} \to \mathcal{D}$ is the "best approximation" to extending $F$ along $K$.

Kan extensions generalize all limits, colimits, and adjoints. As Mac Lane famously noted: "all concepts are Kan extensions." This is an exaggeration, but not by much.

In type theory, the analogous construction is *colimit over a diagram of types*, which in HoTT corresponds to homotopy colimits — a central construction in synthetic homotopy theory.

## The Importance for HoTT

Limits and colimits are central to HoTT because:

1. **HITs as homotopy colimits.** Higher Inductive Types generalize coequalizers and pushouts to the homotopy-theoretic setting. The circle $S^1$, the suspension $\Sigma A$, and the pushout of two types are all HITs — homotopy pushouts.

2. **Truncations as limits.** The propositional truncation $\|A\|$ is a certain colimit (reflection into propositions). The $n$-truncation $\|A\|_n$ is a colimit too. These are universal constructions in the sense of limit theory.

3. **$\infty$-categorical semantics.** HoTT is semantically modeled by $\infty$-toposes, which are $\infty$-categorical objects with specific limit and colimit properties. The connection between HoTT and $\infty$-toposes is mediated by their shared limit/colimit theory.

# Fibered Categories

## A Cleaner Framework

The LCCC approach to dependent type theory — slice categories, pullbacks, adjoints — works mathematically, but has a practical disadvantage: it conflates "types in context $\Gamma$" (objects of $\mathcal{C}/\Gamma$) with "morphisms into $\Gamma$" (also objects of $\mathcal{C}/\Gamma$). The type $A$ in context $\Gamma$ and the type family $B : A \to \mathcal{U}$ are both just morphisms with codomain $\Gamma$ (or $A$), making the structural role of each element less clear.

Fibered categories (Grothendieck fibrations) provide a cleaner framework: the "variable types over varying contexts" are modeled by a *fibration* $p : \mathcal{E} \to \mathcal{B}$, where $\mathcal{B}$ is the *base* (the category of contexts) and the *fibers* $\mathcal{E}_\Gamma = p^{-1}(\Gamma)$ are the categories of types in context $\Gamma$.

## Grothendieck Fibrations

**Definition.** Let $p : \mathcal{E} \to \mathcal{B}$ be a functor. A morphism $\phi : X \to Y$ in $\mathcal{E}$ is *cartesian* over $f : p(X) \to p(Y)$ if for every $Z \in \mathcal{E}$ and every $g : p(Z) \to p(X)$ and $\psi : Z \to Y$ with $p(\psi) = f \circ g$, there exists a unique $\chi : Z \to X$ with $p(\chi) = g$ and $\phi \circ \chi = \psi$.

A functor $p : \mathcal{E} \to \mathcal{B}$ is a *Grothendieck fibration* (or *fibered category*) if for every object $Y \in \mathcal{E}$ and every morphism $f : A \to p(Y)$ in $\mathcal{B}$, there exists a cartesian morphism $\phi : X \to Y$ in $\mathcal{E}$ lying over $f$ (i.e., $p(\phi) = f$).

The cartesian morphism $\phi$ is the *cartesian lift* of $f$ at $Y$. It is unique up to unique isomorphism.

**Intuition.** The base $\mathcal{B}$ is the category of contexts: objects are contexts $\Gamma, \Delta, \ldots$ and morphisms are substitutions (or more generally, context morphisms). The total category $\mathcal{E}$ is the category of types-in-context: an object of $\mathcal{E}$ is a type $A$ in context $\Gamma$, and $p(A) = \Gamma$ is the context in which $A$ lives.

A morphism in $\mathcal{E}$ from $(A$ in $\Gamma)$ to $(B$ in $\Delta)$ lying over $f : \Gamma \to \Delta$ is a morphism of types from $A$ to $B[f]$ (the type $B$ pulled back along $f$). The fibration condition says: for any substitution $f : \Gamma \to \Delta$ and any type $B$ in $\Delta$, there is a canonical "pulled back" type $f^*(B)$ in $\Gamma$, and a canonical cartesian morphism $f^*(B) \to B$ in $\mathcal{E}$.

## The Fiber Categories

For each context $\Gamma \in \mathcal{B}$, the *fiber* of $p$ over $\Gamma$ is the category:
$$\mathcal{E}_\Gamma = \{X \in \mathcal{E} : p(X) = \Gamma\} \text{ with morphisms those } \phi \text{ over } \mathsf{id}_\Gamma$$

The fiber $\mathcal{E}_\Gamma$ is the category of types in context $\Gamma$.

Substitution along $f : \Gamma \to \Delta$ gives a functor $f^* : \mathcal{E}_\Delta \to \mathcal{E}_\Gamma$ (the reindexing functor). This is defined by: $f^*(B)$ is the domain of the cartesian lift of $f$ at $B$.

**Functoriality of reindexing.** The composition $g^* \circ f^* \cong (f \circ g)^*$ and $\mathsf{id}^* \cong \mathsf{Id}$. This is the categorical statement of the substitution lemma: $(B[f])[g] = B[f \circ g]$.

## Cloven and Split Fibrations

A fibration is *cloven* if cartesian lifts are chosen (not just asserted to exist). A cloven fibration gives *specific* reindexing functors $f^* : \mathcal{E}_\Delta \to \mathcal{E}_\Gamma$ (not just functors up to isomorphism).

A cloven fibration is *split* if the reindexing functors compose strictly: $(f \circ g)^* = g^* \circ f^*$ on the nose (not just up to isomorphism). Split fibrations correspond to *pseudofunctors* $\mathcal{B}^{op} \to \mathbf{Cat}$ that are strict.

In type theory: strict split fibrations correspond to type theories with *strict substitution* — substitution that is judgmentally equal on the nose, not just propositionally equal. This corresponds to the *definitional equality* of substitution in MLTT.

## Comprehension Categories

A *comprehension category* is a fibration $p : \mathcal{E} \to \mathcal{B}$ together with a functor $\{-\} : \mathcal{E} \to \mathcal{B}$ that is right adjoint to the "forgetful" functor from $\mathcal{B}$ to the "slice over the fiber" — more precisely, a *context extension* operation.

The context extension sends a context $\Gamma$ and a type $A$ in context $\Gamma$ to the extended context $\Gamma, x:A$ — the context with one more variable.

**Formally.** A comprehension category consists of:
- A fibration $p : \mathcal{E} \to \mathcal{B}$ (types over contexts)
- For each type $A$ over $\Gamma$, a new context $\Gamma.A$ (the extended context) with:
  - A morphism $\pi_A : \Gamma.A \to \Gamma$ (the projection, forgetting the last variable)
  - An object $\mathsf{q}(A) \in \mathcal{E}_{\Gamma.A}$ (the "generic element" of $A$ in extended context)
  - Such that $(\pi_A, \mathsf{q}(A))$ is universal: for any substitution $f : \Delta \to \Gamma$ and any term $t \in \mathcal{E}_\Delta$ over $f^*(A)$, there exists a unique substitution $\langle f, t \rangle : \Delta \to \Gamma.A$ making the appropriate triangle commute.

The universal property of $(\pi_A, \mathsf{q}(A))$ is the categorical version of: "a term $t$ of type $A[f]$ in context $\Delta$ corresponds to a substitution $\Delta \to \Gamma.A$ extending $f$."

## Contextual Categories (C-Systems)

The most closely aligned categorical structure to the syntax of type theory is the *contextual category* (or *C-system*, following Voevodsky).

**Definition.** A *contextual category* is a category $\mathcal{C}$ with:
1. A specified *empty context* object $\langle \rangle$ (the empty list of variable declarations)
2. For each object $\Gamma$ and "type" over $\Gamma$ (an element of a specified set), a new object $\Gamma.A$ (context extension)
3. Projection morphisms $\pi_A : \Gamma.A \to \Gamma$ and generic terms $\mathsf{q}(A) : \Gamma.A \to A[\pi_A]$
4. A specific length function $\ell : \mathsf{Ob}(\mathcal{C}) \to \mathbb{N}$ giving the length of each context
5. ... (various coherence conditions)

Contextual categories are the strictest notion of model for dependent type theory: they model contexts-as-lists-of-variable-declarations, with strict composition and substitution. They are equivalent to *algebraic theories* with typed operations, in the sense of universal algebra.

## Categories with Families

A *category with families* (CwF, Dybjer 1996) is a specific formulation of the categorical semantics of dependent type theory:

- A category $\mathcal{C}$ (the "context" category) with a terminal object $1$
- A functor $\mathsf{Ty} : \mathcal{C}^{op} \to \mathbf{Set}$ (assigning to each context the set of types in that context)
- A functor $\mathsf{Tm} : \mathcal{C}^{op} \to \mathbf{Set}$ over $\mathsf{Ty}$ (assigning to each context and type the set of terms of that type in that context)
- For each $\Gamma \in \mathcal{C}$ and $A \in \mathsf{Ty}(\Gamma)$, a context extension $\Gamma.A$ with projection $p : \Gamma.A \to \Gamma$ and a generic term $v_A \in \mathsf{Tm}(\Gamma.A, A[p])$, satisfying the universal property of context extension

CwFs are equivalent to contextual categories (and to split fibrations with comprehension), but the formulation is more directly amenable to computer verification. The Agda formalization of the semantics of type theory (Altenkirch, Kaposi) uses CwFs as the basic notion of model.

## The Fundamental Fibration

The most fundamental fibration in type theory is the *fundamental fibration* of MLTT:

$$p : \mathcal{C}_\mathsf{MLTT} \to \mathcal{B}_\mathsf{MLTT}$$

where $\mathcal{B}_\mathsf{MLTT}$ is the "base" category (contexts and substitutions) and $\mathcal{C}_\mathsf{MLTT}$ is the category of types-in-contexts. The cartesian lifts are substitutions of types; the fiber over $\Gamma$ is the category of types in context $\Gamma$.

This fibration is the *classifying fibration* for MLTT: any categorical model of MLTT (any CwF, contextual category, or comprehensive fibration satisfying the MLTT rules) receives a unique map from this fundamental fibration.

The existence of the fundamental fibration is the categorical proof that MLTT is *complete* as a type theory: any equation that holds in all categorical models is provable in MLTT.

## From Fibrations to HoTT

The fibration framework provides the natural setting for HoTT's identity type.

In an LCCC, the "standard" interpretation of the identity type as an equalizer does not work (the equalizer is trivial for the identity — it models UIP). The correct interpretation requires the fibration of the identity type to satisfy a *path-lifting property*: any path in the base can be lifted to a path in the total space.

This is the *Awodey-Warren* factorization: the identity type $=_A$ is interpreted as the factorization of the diagonal morphism $\Delta : A \to A \times A$ through a path object $\mathsf{Path}(A)$:

$$A \xrightarrow{r} \mathsf{Path}(A) \xrightarrow{(s,t)} A \times A$$

where $r$ is the reflexivity map (the section of the path space), and $(s, t)$ are the source and target maps. The factorization requires $(s, t)$ to be a *fibration* (acyclic fibration in the model structure) and $r$ to be an *acyclic cofibration*.

In Kan simplicial sets (Voevodsky's model), this factorization comes from the simplicial model structure. The path object $\mathsf{Path}(A)$ is the space of paths in $A$ — literally, the simplicial set of maps from the interval $\Delta[1]$ to $A$. This geometric interpretation is the foundation of HoTT.

# 3.1 The Glue Type and Univalence

## The Gap: Composition in the Universe

We now have `hcomp` and `transp` for all type formers except one: the universe $\mathsf{Type}$ itself.

The universe is special because its *elements* are types, and paths between elements of $\mathsf{Type}$ are equivalences (by univalence). So composition in the universe requires connecting types along equivalences.

The naive approach: define composition in $\mathsf{Type}$ by saying "the composition of a partial family of types is their colimit." But this is circular — defining the universe in terms of itself.

The Glue type is the elegant solution. Instead of defining composition in the universe abstractly, we introduce a new *type former* that explicitly encodes the data needed to compose in the universe. Univalence then follows from this type former.

## The Glue Type

Given:
- A type $B : \mathsf{Type}$ (the "base" type)
- A face formula $\phi$
- Partial data: a partial type $T : [\phi \vdash \mathsf{Type}]$ and a partial equivalence $e : [\phi \vdash T \simeq B]$

Define the *Glue type*:

$$\mathsf{Glue}[\phi \vdash (T, e)]\, B : \mathsf{Type}$$

**Intuition.** Imagine you have type $B$, but on part of the boundary (specified by $\phi$), you want to "replace" $B$ with a different type $T$, connected to $B$ by an equivalence $e$. The Glue type is the result of this replacement.

**Introduction rule.** To introduce an element of $\mathsf{Glue}[\phi \vdash (T, e)]\, B$:

$$\mathsf{glue}[\phi \vdash t]\, b : \mathsf{Glue}[\phi \vdash (T, e)]\, B$$

where $t : [\phi \vdash T]$ is a partial element and $b : B$ with $b = e(t)$ when $\phi = 1$.

**Elimination rule.** To extract the underlying $B$-element:

$$\mathsf{unglue}[\phi \vdash e](g) : B$$

for $g : \mathsf{Glue}[\phi \vdash (T, e)]\, B$.

**Computation rules:**
- When $\phi = 1$: $\mathsf{Glue}[1 \vdash (T, e)]\, B = T$ (the Glue type becomes $T$)
- When $\phi = 1$: $\mathsf{glue}[1 \vdash t]\, b = t$ (the glued element is just $t$)
- When $\phi = 1$: $\mathsf{unglue}[1 \vdash e](t) = e(t)$ (ungluing applies the equivalence)

The key: these rules hold *definitionally*. There's no "path" between the sides — they compute to the same thing.

## Composition in the Universe

Using Glue, we can now define composition in the universe: given a partial family of types $u : (i : \mathbb{I}) \to [\phi \vdash \mathsf{Type}]$ and a base type $A_0 : \mathsf{Type}$ with $A_0 = u(0)$ when $\phi = 1$, the composition is:

$$\mathsf{hcomp}_\phi^\mathsf{Type}(u, A_0) :\equiv \mathsf{Glue}[\phi \vdash (u(1), e_\phi)] A_0'$$

where $e_\phi$ is the equivalence from the partial type $u(1)$ to $A_0'$ obtained by transporting along the tube $u$.

This is the technical heart of CCHM: composition in the universe is Glue-formation. The Glue type is not an ad hoc primitive but exactly the operation needed for universe composition.

## Proving Univalence

We can now prove univalence in CCHM cubical type theory.

**Theorem (Univalence).** For any types $A, B : \mathsf{Type}$, there is an equivalence:

$$(A =_\mathsf{Type} B) \simeq (A \simeq B)$$

In particular, the function $\mathsf{idToEquiv} : (A =_\mathsf{Type} B) \to (A \simeq B)$ defined by transport is an equivalence.

**Proof: the function $\mathsf{ua}$.** Define:

$$\mathsf{ua}(e) :\equiv \lambda i. \mathsf{Glue}[(i = 0) \vdash (A, e), (i = 1) \vdash (B, \mathsf{id}_B)]\, B : A =_\mathsf{Type} B$$

- At $i = 0$: $\phi = 1$, so $\mathsf{Glue}[1 \vdash (A, e)]\, B = A$ ✓
- At $i = 1$: $\phi = 1$ (the other face), so $\mathsf{Glue}[1 \vdash (B, \mathsf{id})]\, B = B$ ✓

So $\mathsf{ua}(e) : A =_\mathsf{Type} B$ is a genuine path from $A$ to $B$ in the universe.

**The computation rule.** The crucial property:

$$\mathsf{transport}(\mathsf{ua}(e), a) = e(a)$$

Let's verify. $\mathsf{transport}(\mathsf{ua}(e), a) = \mathsf{transp}^{\lambda i. \mathsf{ua}(e)(i)}(a)$.

Since $\mathsf{ua}(e)(i) = \mathsf{Glue}[\phi_i \vdash ...]\, B$, transport in a Glue type has a specific reduction rule: it applies the unglue and then the equivalence. Working through the rules:

$$\mathsf{transp}^{\mathsf{Glue}[...]}\,(a) = e(a)$$

This holds by the computation rules for `transp` applied to Glue types. The Glue type is precisely designed so that transport through it applies the underlying equivalence.

**The full equivalence.** It remains to show that `ua` and `idToEquiv` are mutual inverses (up to equivalence).

$\mathsf{ua}(\mathsf{idToEquiv}(p)) = p$: By path induction (J), we can assume $p = \mathsf{refl}$, in which case $\mathsf{idToEquiv}(\mathsf{refl}) = \mathsf{id}$ and $\mathsf{ua}(\mathsf{id}) = ?$. We need $\mathsf{ua}(\mathsf{id}_A) = \mathsf{refl}_A$.

This uses the fact that $\mathsf{Glue}[\phi \vdash (A, \mathsf{id})]\, A = A$ (identity equivalence trivializes the Glue type), so $\mathsf{ua}(\mathsf{id}) = \lambda i. A = \mathsf{refl}$. ✓

$\mathsf{idToEquiv}(\mathsf{ua}(e)) = e$: Follows from the computation rule: $\mathsf{transport}(\mathsf{ua}(e), -) = e(-)$ as functions. By function extensionality (itself a theorem in cubical TT), they're equal as equivalences. ✓

So $\mathsf{idToEquiv}$ is an equivalence — this is univalence. $\square$

## What Makes Univalence a Theorem (Not an Axiom)

The key is the Glue type's computation rules. In Book HoTT, we postulate:
- $\mathsf{ua}(e) : A = B$ (assertion with no computation)
- $\mathsf{transport}(\mathsf{ua}(e), a) = e(a)$ (separate axiom, also with no computation)

In CCHM, we have:
- $\mathsf{ua}(e) : A = B$ (constructed from Glue — it's a definition, not a postulate)
- $\mathsf{transport}(\mathsf{ua}(e), a) = e(a)$ (follows from Glue's computation rules — definitional!)

The difference: in Book HoTT, the two are axioms that just happen to be consistent. In CCHM, the second follows *by computation* from the first.

## Function Extensionality from Paths

Function extensionality is also a theorem:

**Theorem (FunExt).** For $f, g : \Pi_{x:A} B(x)$ and $h : \Pi_{x:A} f(x) =_{B(x)} g(x)$:

$$\mathsf{funExt}(h) : f =_{\Pi_{x:A} B(x)} g$$

*Proof.*

$$\mathsf{funExt}(h) :\equiv \lambda i. \lambda x. h(x)(i)$$

Check:
- $(\lambda i. \lambda x. h(x)(i))(0) = \lambda x. h(x)(0) = \lambda x. f(x) = f$ ✓
- $(\lambda i. \lambda x. h(x)(i))(1) = \lambda x. h(x)(1) = \lambda x. g(x) = g$ ✓

The path $\mathsf{funExt}(h)$ is literally "rearrange the arguments of $h$." No axiom needed. $\square$

This is definitional function extensionality: $\mathsf{funExt}(h)(i)(x) = h(x)(i)$ by computation.

## Propositional Extensionality

Propext (logically equivalent propositions are equal) follows from univalence:

$$\mathsf{propext}(h_1, h_2) :\equiv \mathsf{ua}(\mathsf{isoToEquiv}(\mathsf{iso}\, h_1\, h_2\, ...))$$

where $h_1 : A \to B$ and $h_2 : B \to A$ are the two implications. This constructs an equivalence from an isomorphism between propositions (where the coherences are trivial because propositions have at most one proof), and then applies `ua`.

## The Glue Type as a Colimit

The Glue type has a natural interpretation as a *colimit*: it's the pushout of $T \leftarrow T \xrightarrow{e} B$, where one map is the identity. This colimit "replaces" $T$ in $B$ using the equivalence $e$.

More precisely: $\mathsf{Glue}[\phi \vdash (T, e)]\, B$ is the pullback $T \times_B B = T$ when $\phi = 1$ (trivially $T$) and is $B$ when $\phi = 0$. In between, it interpolates.

This is an instance of a *coend* or *Kan extension* in the appropriate categorical sense. The Glue type is the *rectification* of a partially specified type family — filling in the missing part using the given equivalences.

## The Structure of CCHM Cubical Type Theory

Putting it all together, CCHM cubical type theory has:

**Primitives:**
1. Interval $\mathbb{I}$ with $0, 1, \sim, \wedge, \vee$
2. Face formulas $\phi$
3. Partial types $[\phi \vdash A]$ and extensions
4. `hcomp` for uniform composition
5. `transp` for transport
6. `Glue` type with `glue` and `unglue`

**Derived operations:**
- Path types $a =_A b$ (from $\mathbb{I}$ and function types)
- `refl`, `sym`, `_∙_` (from interval operations)
- `funExt` (from path type)
- `ua` (from Glue type)

**Theorems:**
- Univalence
- Function extensionality
- Canonicity
- All Book HoTT theorems

This is the complete theory that Cubical Agda implements.

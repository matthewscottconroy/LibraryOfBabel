# 1.1 Identity Types as Path Spaces

## From Equality to Paths

Classical mathematics has a simple notion of equality: two objects are equal or they're not. Equality is a proposition (a statement with a definite truth value), not a type with elements.

HoTT enriches this. The identity type $a =_A b$ is a genuine type — it may have many elements, and those elements carry information. An element $p : a =_A b$ is not just a certificate that $a$ equals $b$; it's a specific *reason* or *path* witnessing this equality.

This is not just a logical convenience. In the simplicial set model of Chapter 15:
- $A$ is interpreted as a Kan complex (a combinatorial space)
- $a, b : A$ are 0-simplices (vertices)
- $a =_A b$ is the *simplicial path space*: the Kan complex of 1-simplices from $a$ to $b$
- $p : a =_A b$ is a specific 1-simplex connecting $a$ to $b$

So "identity type" and "path type" are the same thing. Not by convention — by the mathematical content of the model.

## The Formation, Introduction, and Elimination Rules

Let's review the identity type's rules with fresh eyes.

**Formation:**
$$\frac{\Gamma \vdash A : \mathsf{Type} \quad \Gamma \vdash a : A \quad \Gamma \vdash b : A}{\Gamma \vdash a =_A b : \mathsf{Type}}$$

For any type $A$ and any two terms $a, b : A$, we can form the identity type. This makes the identity type a *type family* over $A \times A$: the family $\mathsf{Id}_A : A \times A \to \mathsf{Type}$ sending $(a, b)$ to the path type $a =_A b$.

**Introduction:**
$$\frac{\Gamma \vdash a : A}{\Gamma \vdash \mathsf{refl}_a : a =_A a}$$

The *reflexivity path* $\mathsf{refl}_a$ is the only axiomatically given inhabitant of any identity type. It's the constant path at $a$.

Importantly: we're only given reflexivity from *the same point to itself*. There's no direct way to introduce an element of $a =_A b$ for $a \neq b$ — you have to build such paths from the structure of $A$ itself (using the constructors of $A$ or other type formers).

**Elimination (J rule):**
$$\frac{\Gamma \vdash a : A \quad \Gamma, b:A, p : a =_A b \vdash C : \mathsf{Type} \quad \Gamma \vdash d : C[a/b, \mathsf{refl}_a/p]}{\Gamma, b:A, p:a=_A b \vdash \mathsf{J}(C, d, b, p) : C}$$

**Computation rule:** $\mathsf{J}(C, d, a, \mathsf{refl}_a) \equiv d$

This is path induction: to construct something of type $C(b, p)$ for all $b : A$ and $p : a =_A b$, it suffices to handle the case $b = a$ and $p = \mathsf{refl}_a$.

## The Contractibility Interpretation of J

Why does it suffice to handle $\mathsf{refl}_a$? The deep reason: **the total path space is contractible**.

**Definition 1.1.** For a fixed $a : A$, the *total path space* (or *based path space*) from $a$ is:
$$\mathsf{P}_a A = \sum_{b:A} (a =_A b)$$

**Theorem 1.2.** $\mathsf{P}_a A$ is contractible.

*Proof.* The center of contraction is $(a, \mathsf{refl}_a)$. For any $(b, p) : \mathsf{P}_a A$, we need a path $(a, \mathsf{refl}_a) = (b, p)$ in $\mathsf{P}_a A$.

By the characterization of paths in Σ types (Section 5), this is equivalent to: a path $q : a = b$ and a path-over-$q$ from $\mathsf{refl}_a$ to $p$. Take $q = p$ and the path-over is given by $\mathsf{refl}_p : \mathsf{transport}^{a=(-)}(p, \mathsf{refl}_a) = p$. (This uses the fact that transport of $\mathsf{refl}_a$ along $p$ gives $p$ itself — a computation using J.) $\square$

**The J rule follows:** If $\mathsf{P}_a A$ is contractible, then to define something for all $(b, p) : \mathsf{P}_a A$, it suffices to define it at the single contractible point $(a, \mathsf{refl}_a)$. That's exactly J.

In the simplicial set model: the based path space $\mathsf{P}_a A$ is the pullback of the path fibration $A^{\Delta[1]} \to A$ along $\{a\} \to A$. This is a contractible Kan complex (it deformation retracts to the constant path at $a$).

## Reflexivity as the Degenerate Simplex

In the simplicial model, the identity type $a =_A b$ for $a, b \in \llbracket A \rrbracket_0$ is modeled by the space of 1-simplices from $a$ to $b$ in $\llbracket A \rrbracket$.

The reflexivity $\mathsf{refl}_a \in a =_A a$ is modeled by the *degenerate* 1-simplex $\sigma_0(a) \in \llbracket A \rrbracket_1$: the simplex obtained by degenerating the vertex $a$.

Geometrically: the degenerate 1-simplex at $a$ is the constant path — a 1-simplex that starts and ends at $a$ and doesn't "move." This is exactly what $\mathsf{refl}_a$ should be.

The simplicial identities ensure that the degeneracies have the right properties:
- $\partial_0(\sigma_0(a)) = a = \partial_1(\sigma_0(a))$ (the constant path starts and ends at $a$)
- $\sigma_0(\mathsf{refl}_a) =$ a degenerate 2-simplex at $a$ (the "constant homotopy")

## Why Identity Types Can Have Multiple Elements

In classical mathematics (where types are sets), each set-theoretic equality $a = b$ is either trivially true (if $a$ and $b$ are literally the same element) or false. There's at most one element in $a = b$.

In HoTT, types are not sets in general — they're spaces. And in a space, there can be many distinct paths between two points. For example:
- In the simplicial circle $S^1$: $\mathsf{base} =_{S^1} \mathsf{base}$ has infinitely many elements (one for each integer, corresponding to the winding number)
- In the universe $\mathsf{Type}$: $A =_\mathsf{Type} B$ (a path between types $A$ and $B$) corresponds to an equivalence $A \simeq B$ by Univalence, and there can be many non-isomorphic equivalences

This multiplicity is the key feature of HoTT that makes it genuinely homotopy-theoretic. Types are not discrete (where there's at most one path between any two points), they're general spaces with rich path structure.

## The Identity Type is Not a Proposition

A common confusion: is $a = b$ a *proposition* (at most one proof) or a *type* (many elements)?

In HoTT, $a =_A b$ is a type. It may or may not be a proposition depending on the type $A$:
- If $A$ is a *set* (h-level 0, Chapter 17): $a =_A b$ has at most one element — either it's empty (false) or contractible (true). So set-equality is propositional.
- If $A$ is a *groupoid* (h-level 1): $a =_A b$ may have multiple elements.
- If $A$ is a general type: $a =_A b$ can have arbitrarily complex structure.

The distinction matters for doing mathematics: when working with sets (like $\mathbb{N}$, $\mathbb{Z}$, any discrete type), equality behaves classically. When working with groupoids or higher types (like $S^1$, or $\mathsf{Type}$ itself), equality has genuinely homotopy-theoretic content.

## Summary

| Classical equality | HoTT identity type |
|---|---|
| Proposition (true/false) | Type (with elements) |
| At most one proof | Can have many elements |
| Substitution (Leibniz) | Transport along paths |
| Leibniz's rule | J eliminator |
| Reflexive, symmetric, transitive | Groupoid structure |
| Can be decidable | May have non-trivial higher structure |

The identity type is the linchpin of HoTT. It's the type-theoretic incarnation of the path space from topology, the hom-set from groupoid theory, and the equality relation from logic — all in one unified construction. The J rule is the common principle underlying path induction, the homotopy lifting property, and Leibniz's substitution principle.

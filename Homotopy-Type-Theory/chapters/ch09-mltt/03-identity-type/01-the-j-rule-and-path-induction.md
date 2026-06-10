# 3.1 The Identity Type and Path Induction

## The Central Object of HoTT

Everything that makes HoTT distinctive traces back to the identity type. It's not an add-on or a special case — it's the type-theoretic rendering of equality, and the fact that it can have non-trivial structure is the heart of homotopy type theory.

Let's build up carefully.

## Formation

Given a type $A$ and two elements $a, b : A$, there is a type:

$$\frac{\Gamma \vdash A\ \mathsf{type} \quad \Gamma \vdash a : A \quad \Gamma \vdash b : A}{\Gamma \vdash a =_A b\ \mathsf{type}}$$

The *identity type* $a =_A b$ (also written $\mathsf{Id}_A(a, b)$ or just $a = b$ when $A$ is clear) is the type whose elements are proofs that $a$ equals $b$.

**Important:** This is a *type*, not a judgment. The statement "$a = b$" is something you can prove or disprove by constructing or failing to construct an element of the identity type. The judgment $\Gamma \vdash a = b : A$ (definitional equality) is different — it's checked by the type checker mechanically, without proof.

Two things to notice:
1. $a$ and $b$ are *terms* in $A$, not symbols or names. The identity type depends on actual elements of $A$.
2. For fixed $a$, the family $b \mapsto (a =_A b)$ is a type family over $A$. It's often called the *based path space* with basepoint $a$.

## Introduction: Reflexivity

The only constructor for the identity type is reflexivity:

$$\frac{\Gamma \vdash a : A}{\Gamma \vdash \mathsf{refl}_a : a =_A a}$$

Every element is equal to itself. This is the only way to directly construct an element of an identity type.

Wait — does this mean the only element of $a =_A b$ is $\mathsf{refl}$? No! $\mathsf{refl}$ only inhabits $a =_A a$ (equality of an element with itself). For $a \neq b$ (in some appropriate sense), there may be no element of $a =_A b$. And for specific types (like higher inductive types), there can be *multiple* elements of $a =_A a$ — non-trivial paths from $a$ to itself.

The key point: $\mathsf{refl}$ is the *only introduced* equality. All other equalities — symmetry, transitivity, the results of ap and transport — are *derived* from $\mathsf{refl}$ via the elimination rule.

## Elimination: The J Rule

The J rule is the elimination principle for the identity type. It says: to prove a property of all identity proofs, it suffices to prove it for reflexivity.

**Based path induction (J):**

$$\frac{\Gamma \vdash a : A \quad \Gamma \vdash C : \prod_{b:A} (a =_A b) \to \mathsf{Type} \quad \Gamma \vdash d : C(a, \mathsf{refl}_a) \quad \Gamma \vdash b : A \quad \Gamma \vdash p : a =_A b}{\Gamma \vdash \mathsf{J}(C, d, b, p) : C(b, p)}$$

Let's unpack this:
- $C$ is a *motive*: a type family that takes a point $b$ and a path $p : a = b$, and returns a type. It describes what property you want to prove about all paths starting at $a$.
- $d$ is the *base case*: a proof of $C(a, \mathsf{refl}_a)$, i.e., the property holds when the path is the trivial reflexivity path at $a$.
- Given any $b : A$ and any path $p : a = b$, the J rule produces an element of $C(b, p)$.

**Computation rule:**
$$\mathsf{J}(C, d, a, \mathsf{refl}_a) = d : C(a, \mathsf{refl}_a)$$

When you apply J to a reflexivity path, it reduces to the base case. This is the only computation rule for J (there's only one constructor to compute on).

## Reading J: Path Induction

The J rule says: **any path starting at $a$ can be "contracted" to the reflexivity path**, for the purposes of proving a property.

Geometrically: if you want to prove a property of all paths starting at $a$, you only need to prove it for the trivial path (the path that stays at $a$). Any other path can be continuously deformed to the trivial path — that's what "induction" means for paths.

This is the type-theoretic version of a topological fact: in a *contractible* space, all paths are homotopic. The based path space $\sum_{b:A} (a = b)$ is *contractible* (has a unique center of contraction at $(a, \mathsf{refl}_a)$) — a fact derivable from J.

**Comparison to natural number induction:** Induction on $\mathbb{N}$ says: to prove $P(n)$ for all $n$, prove $P(0)$ and (from $P(n)$) prove $P(n+1)$. Induction on identity proofs says: to prove $C(b, p)$ for all $b : A$ and $p : a = b$, prove $C(a, \mathsf{refl}_a)$. There's only one "base case" because $\mathsf{refl}_a$ is the only direct constructor.

## Alternative: Unbased Path Induction (J')

The *unbased* version of J quantifies over both endpoints simultaneously:

Given:
- $C : \prod_{a\, b : A} (a =_A b) \to \mathsf{Type}$
- $d : \prod_{a:A} C(a, a, \mathsf{refl}_a)$

Produces:
$$\mathsf{J}'(C, d) : \prod_{a\, b : A} \prod_{p : a = b} C(a, b, p)$$

Computation: $\mathsf{J}'(C, d, a, a, \mathsf{refl}_a) = d(a)$.

**Theorem.** J and J' are equivalent: each can derive the other.

*J' from J:* Given $C : \prod_{a\, b:A} (a = b) \to \mathsf{Type}$ and $d : \prod_{a:A} C(a, a, \mathsf{refl}_a)$, apply J with motive $C(a_0, \_\,, \_)$ for each $a_0$ separately. This gives $\mathsf{J}'$.

*J from J':* Given $a : A$, $C : \prod_{b:A} (a = b) \to \mathsf{Type}$, and $d : C(a, \mathsf{refl}_a)$, define $C'(x, y, p) = (x = a) \to C(y, p)$. Then $d' = \lambda z. \mathsf{transport}^C(z, d)$ (where the transport is along the path $z : x = a$). Apply J' to get the result, then instantiate with $\mathsf{refl}_a$.

Both formulations are used in the literature. The HoTT Book uses J (based). Agda's library mostly uses J'. They give the same derivable consequences.

## Deriving Symmetry (Path Inversion)

Using J, we derive that equality is symmetric: if $p : a = b$, there is a path $p^{-1} : b = a$.

**Construction:** Apply J with:
- Motive: $C(b, p) = (b =_A a)$ — for each $b$ and path $p : a = b$, the type of paths from $b$ back to $a$
- Base case: $d = \mathsf{refl}_a : C(a, \mathsf{refl}_a) = (a = a)$

The J rule gives $\mathsf{J}(C, \mathsf{refl}_a) : \prod_{b:A} \prod_{p:a=b} (b = a)$.

Define $p^{-1} = \mathsf{J}(C, \mathsf{refl}_a, b, p)$.

**Computation:** $(\mathsf{refl}_a)^{-1} = \mathsf{J}(C, \mathsf{refl}_a, a, \mathsf{refl}_a) = \mathsf{refl}_a$.

So the inverse of the reflexivity path is reflexivity itself. Makes sense geometrically: the inverse of the constant path is itself.

## Deriving Transitivity (Path Concatenation)

If $p : a = b$ and $q : b = c$, there is a path $p \cdot q : a = c$ (concatenation / transitivity).

**Construction:** Apply J to $q$, with basepoint $b$, motive $C(c, q') = (a =_A c)$, and base case $d = p : (a = b) = C(b, \mathsf{refl}_b)$.

The J rule gives $\mathsf{J}(C, p) : \prod_{c:A} \prod_{q:b=c} (a = c)$.

Define $p \cdot q = \mathsf{J}(C, p, c, q)$.

**Computation:** $p \cdot \mathsf{refl}_b = p$ (by the J computation rule, since $d = p$).

Note: We could also apply J to $p$ first. The two approaches give the same concatenation operation (they're propositionally equal), but there's a subtle difference in the computation rules.

## The Groupoid Laws

With symmetry and transitivity defined by J, we can prove the groupoid laws. Each law is a propositional equality between identity proofs — an element of an identity type of an identity type.

**Left unit:** $\mathsf{refl}_a \cdot p = p$ for $p : a = b$.

*Proof:* Apply J to $p$. When $p = \mathsf{refl}_a$, we need $\mathsf{refl}_a \cdot \mathsf{refl}_a = \mathsf{refl}_a$, which holds by computation ($\mathsf{refl}_a \cdot \mathsf{refl}_a = \mathsf{J}(C, \mathsf{refl}_a, a, \mathsf{refl}_a) = \mathsf{refl}_a$).

**Right unit:** $p \cdot \mathsf{refl}_b = p$ for $p : a = b$.

*Proof:* This also holds by computation (since we defined $p \cdot q$ by J on $q$, the base case is $d = p$, and $p \cdot \mathsf{refl}_b$ reduces to $p$ directly).

**Associativity:** $(p \cdot q) \cdot r = p \cdot (q \cdot r)$.

*Proof:* Apply J to $r$ (reduce to $r = \mathsf{refl}$, then use computation). When $r = \mathsf{refl}$: $(p \cdot q) \cdot \mathsf{refl} = p \cdot q = p \cdot (q \cdot \mathsf{refl})$.

**Inverse laws:**
- $p \cdot p^{-1} = \mathsf{refl}_a$
- $p^{-1} \cdot p = \mathsf{refl}_b$

*Proof:* Apply J to $p$. When $p = \mathsf{refl}_a$: $\mathsf{refl}_a \cdot \mathsf{refl}_a^{-1} = \mathsf{refl}_a \cdot \mathsf{refl}_a = \mathsf{refl}_a$.

**The fundamental observation:** These laws say every type $A$ is a *groupoid*:
- Objects: elements of $A$
- Morphisms from $a$ to $b$: elements of $a = b$
- Composition: $p \cdot q$
- Identity morphism: $\mathsf{refl}_a$
- Inverse: $p^{-1}$

This is the groupoid interpretation of identity types, due to Hofmann and Streicher (1994). It was their insight that types should be interpreted not as sets (where equality is trivial) but as groupoids (where morphisms have structure).

## What J Does Not Allow: UIP

*Uniqueness of Identity Proofs* (UIP) is the statement:

$$\mathsf{UIP} : \prod_{A:\mathsf{Type}} \prod_{a\, b : A} \prod_{p\, q : a = b} p = q$$

Every two proofs of the same equality are equal. In classical set-theoretic mathematics, this is trivially true (equality is just a relation, and either it holds or it doesn't).

**Theorem (Streicher 1993).** UIP is not derivable from J.

*Evidence:* The groupoid model. Take the fundamental groupoid of the circle $S^1$. The basepoint is $\mathsf{base}$. The "paths" from $\mathsf{base}$ to $\mathsf{base}$ are elements of $\pi_1(S^1, \mathsf{base}) = \mathbb{Z}$ — one for each winding number. These are not all equal. So two proofs of $\mathsf{base} = \mathsf{base}$ in the model of $S^1$ are genuinely different.

This is the first indication that types in MLTT can behave like spaces with non-trivial topology. In the groupoid model, UIP fails because groups can be non-trivial.

The consequence for HoTT: since UIP fails, the identity type can have multiple distinct elements (multiple paths between the same two points). These paths can themselves have identity proofs (homotopies between paths), which can have homotopies, and so on. This is the *$\infty$-groupoid* structure of types in HoTT.

## Axiom K and UIP

**Axiom K** (Streicher, 1993) is the statement:

$$K : \prod_{a : A} \prod_{p : a = a} p = \mathsf{refl}_a$$

Every self-loop is the trivial loop. This is equivalent to UIP. It's consistent with MLTT (it holds in the set-theoretic model) but is not derivable from J alone.

In Agda, the `--with-K` flag enables Axiom K. For HoTT, you want Agda run with `--without-K` (the default) — otherwise you'd collapse the homotopy structure.

Lean 4 includes Axiom K via `propext` (propositional extensionality) and `proof_irrel` (proof irrelevance for `Prop`). This means Lean 4's `Prop` universe satisfies UIP, but `Type` universes don't. When you do HoTT in Lean 4, you work in `Type` and avoid `Prop`.

## Higher Identity Types

The identity type of an identity type is itself an identity type: if $p, q : a = b$, then $p =_{a=b} q$ is the type of *homotopies* from $p$ to $q$ — paths between paths.

These higher identity types are not trivial in general:
- $\pi_1(S^1) = \mathbb{Z}$ — the fundamental group of the circle (non-trivial self-loops)
- $\pi_2(S^2) = \mathbb{Z}$ — the second homotopy group of the 2-sphere (non-trivial 2-paths)

In HoTT, types are $\infty$-groupoids: they have elements (0-cells), paths between elements (1-cells), paths between paths (2-cells), and so on to all finite levels. The J rule is the primitive induction principle for this entire $\infty$-dimensional structure.

This is the bridge from type theory to homotopy theory. The precise formulation — using univalence to identify equivalences with paths in the universe, and HITs to build spaces with specified homotopy groups — is the subject of the rest of the book.

## Summary

| Property | Formula | Proof |
|---|---|---|
| Reflexivity | $\mathsf{refl}_a : a = a$ | Constructor |
| Symmetry | $p^{-1} : b = a$ from $p : a = b$ | J (motive: $b = a$) |
| Transitivity | $p \cdot q : a = c$ from $p : a = b$, $q : b = c$ | J on $q$ |
| Left unit | $\mathsf{refl} \cdot p = p$ | J on $p$, computation |
| Right unit | $p \cdot \mathsf{refl} = p$ | Computation |
| Associativity | $(p \cdot q) \cdot r = p \cdot (q \cdot r)$ | J on $r$ |
| Inverse laws | $p \cdot p^{-1} = \mathsf{refl}$ | J on $p$ |
| UIP | *Not derivable* | — |

The identity type, with only $\mathsf{refl}$ as a constructor and J as the eliminator, generates a rich structure: the groupoid laws hold propositionally, transport is definable, and the door to HoTT's higher-dimensional world is open.

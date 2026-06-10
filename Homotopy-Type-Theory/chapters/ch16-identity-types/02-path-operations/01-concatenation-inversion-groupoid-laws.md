# 2.1 Path Operations: Concatenation, Inversion, and the Groupoid Laws

## Building Operations from J

The J rule is the fundamental eliminator for the identity type. Everything else — concatenation, inversion, the groupoid laws — must be derived from J. This derivation illuminates the structure of the identity type.

The key principle: **to define a function out of the identity type (or to prove something about all paths), it suffices to handle the reflexivity case.**

## Path Concatenation

Given a path from $a$ to $b$ and a path from $b$ to $c$, we should be able to concatenate them into a path from $a$ to $c$.

**Theorem 2.1 (Path Concatenation).** There is a function:
$$(-) \cdot (-) : (a =_A b) \to (b =_A c) \to (a =_A c)$$
defined for all $a, b, c : A$.

**Construction via J:** Fix $a, b : A$ and $p : a = b$. We want to define, for all $c : A$ and $q : b = c$, a path $p \cdot q : a = c$.

Apply J to $q$ with:
- The second argument fixed to $p$ and first argument varying: base point $b$, path $q : b = c$
- Motive: $C(c, q) = (a = c)$
- Base case: we need to supply something of type $C(b, \mathsf{refl}_b) = (a = b)$, which is $p$ itself

So define: $p \cdot \mathsf{refl}_b \equiv p$.

By J, this extends uniquely (up to judgmental equality) to $p \cdot q$ for all $q$.

**Computation rule:** $p \cdot \mathsf{refl}_b \equiv p$ (definitional equality)

Note: $\mathsf{refl}_a \cdot p$ is *not* definitionally equal to $p$ — it's only *propositionally* equal to $p$ (proved by J). This asymmetry comes from the fact that our J-definition of concatenation inducte on the second argument $q$.

We could alternatively define concatenation by inducting on the first argument, getting the opposite computation rule. Either way, we only get *one* definitional computation rule; the other follows propositionally.

## Path Inversion

Given $p : a = b$, we should have $p^{-1} : b = a$.

**Theorem 2.2 (Path Inversion).** There is a function:
$$(-) ^{-1} : (a =_A b) \to (b =_A a)$$

**Construction via J:** Apply J to $p : a = b$ with:
- Motive: $C(b, p) = (b = a)$
- Base case: $C(a, \mathsf{refl}_a) = (a = a)$, which has element $\mathsf{refl}_a$

So define: $(\mathsf{refl}_a)^{-1} \equiv \mathsf{refl}_a$.

By J, this gives $p^{-1} : b = a$ for all $p : a = b$.

**Computation rule:** $(\mathsf{refl}_a)^{-1} \equiv \mathsf{refl}_a$

## The Groupoid Laws

With concatenation and inversion defined, we can state and prove the five groupoid laws. Each is a propositional equality — an element of an identity type.

**Lemma 2.3 (Left Unit).** For $p : a = b$: $\mathsf{refl}_a \cdot p = p$.

*Proof:* By J on $p$. Base case: $\mathsf{refl}_a \cdot \mathsf{refl}_a =? \mathsf{refl}_a$. By the computation rule, $\mathsf{refl}_a \cdot \mathsf{refl}_a \equiv \mathsf{refl}_a$, so $\mathsf{refl}$ proves this. $\square$

**Lemma 2.4 (Right Unit).** For $p : a = b$: $p \cdot \mathsf{refl}_b = p$.

*Proof:* This is immediate from the computation rule for concatenation: $p \cdot \mathsf{refl}_b \equiv p$ by definition. So $\mathsf{refl}_p : p \cdot \mathsf{refl}_b = p$. $\square$

Actually, this one is *definitional*, not just propositional! The asymmetry mentioned earlier: right unit holds by definition, left unit requires a proof by J.

**Lemma 2.5 (Associativity).** For $p : a = b$, $q : b = c$, $r : c = d$: $(p \cdot q) \cdot r = p \cdot (q \cdot r)$.

*Proof:* By J on $p$ (inducting on the first path). Base case: $(\mathsf{refl}_a \cdot q) \cdot r =? \mathsf{refl}_a \cdot (q \cdot r)$.

By the left unit law (Lemma 2.3): $\mathsf{refl}_a \cdot q = q$ and $\mathsf{refl}_a \cdot (q \cdot r) = q \cdot r$. And $(q) \cdot r = q \cdot r$. So it reduces to $q \cdot r = q \cdot r$, which is $\mathsf{refl}$. $\square$

Wait, this uses the left unit law in the proof, and the left unit law already uses J. So we're using J inside a J proof. This is fine — the rules allow it, and the computation rules ensure everything is definitionally equal to the right thing.

**Lemma 2.6 (Right Inverse).** For $p : a = b$: $p \cdot p^{-1} = \mathsf{refl}_a$.

*Proof:* By J on $p$. Base case: $\mathsf{refl}_a \cdot (\mathsf{refl}_a)^{-1} = \mathsf{refl}_a$.

By computation: $(\mathsf{refl}_a)^{-1} \equiv \mathsf{refl}_a$ and $\mathsf{refl}_a \cdot \mathsf{refl}_a \equiv \mathsf{refl}_a$ (by right unit). Wait — the right unit gives $\mathsf{refl}_a \cdot \mathsf{refl}_a \equiv \mathsf{refl}_a$. So the left side is definitionally equal to $\mathsf{refl}_a$, and $\mathsf{refl}$ proves the equality. $\square$

**Lemma 2.7 (Left Inverse).** For $p : a = b$: $p^{-1} \cdot p = \mathsf{refl}_b$.

*Proof:* By J on $p$. Base case: $(\mathsf{refl}_a)^{-1} \cdot \mathsf{refl}_a = \mathsf{refl}_a$.

$(\mathsf{refl}_a)^{-1} \equiv \mathsf{refl}_a$, so the left side is $\mathsf{refl}_a \cdot \mathsf{refl}_a \equiv \mathsf{refl}_a$. Done. $\square$

## Why Propositional, Not Definitional?

The groupoid laws hold *propositionally* — there exist proof terms witnessing them — but not *definitionally* (the sides of each equation are not definitionally equal).

Why not definitional? Because definitional equality is decided by computation — it would require defining concatenation such that the computation rules give immediate judgmental equality for all five laws. But the five laws are mutually constraining, and no single definition satisfies all five definitionally.

For example: if we define $p \cdot q$ by inducting on $q$ (as we did), we get right unit definitionally but left unit only propositionally. If we induct on $p$, we get the reverse. There's no way to get both definitionally with a single inductive definition.

This is a reflection of a deeper point: the J rule is the fundamental eliminator, and it's designed so that the reflexivity case computes. Higher equalities (like associativity) require proof, not computation.

In **cubical type theory** (Chapter 23), the situation is different. There, associativity and unit laws can be made to hold definitionally using the interval and its computation rules.

## The Total Path Space is Contractible

The groupoid laws have an important consequence:

**Theorem 2.8.** The total path space $\sum_{b:A}(a = b)$ is contractible.

*Proof:* Define the contraction center as $(a, \mathsf{refl}_a)$. For any $(b, p) : \sum_{b:A}(a = b)$, we need $(a, \mathsf{refl}_a) = (b, p)$.

By the characterization of Σ-paths (Section 5), this is: $\exists\, q : a = b, \mathsf{transport}^{x \mapsto (a=x)}(q, \mathsf{refl}_a) = p$.

Take $q = p$. Then $\mathsf{transport}^{x \mapsto (a=x)}(p, \mathsf{refl}_a) = \mathsf{refl}_a \cdot p$ (transport in a path type concatenates). By the left unit law, $\mathsf{refl}_a \cdot p = p$. So the Σ-path is $(p, \mathsf{left\_unit}(p))$. $\square$

This is the precise statement of "J says the total path space is contractible." Every $(b, p) : \sum_{b:A}(a=b)$ is connected to the basepoint $(a, \mathsf{refl}_a)$ by a canonical path.

## The Eckmann-Hilton Argument for 2-Paths

When we look at paths at the 2-dimensional level (loops at the loop space), something remarkable happens: the two operations on 2-paths (horizontal and vertical composition) coincide and are commutative.

**Theorem 2.9 (Eckmann-Hilton).** For a type $A$ with $a : A$, the operation on $\pi_2(A) = ((\mathsf{refl}_a = \mathsf{refl}_a) : a=a)$ is commutative.

More precisely: if $\alpha, \beta : \mathsf{refl}_a = \mathsf{refl}_a$ (2-paths at the trivial loop), then $\alpha \cdot \beta = \beta \cdot \alpha$.

*Proof:* There are two operations on 2-paths:
- Horizontal: $\alpha \star \beta$ (compose paths horizontally)
- Vertical: $\alpha \cdot \beta$ (compose paths vertically)

Both operations have unit $\mathsf{refl}_{\mathsf{refl}_a}$. The interchange law says $(\alpha \cdot \beta) \star (\gamma \cdot \delta) = (\alpha \star \gamma) \cdot (\beta \star \delta)$.

Setting $\alpha = \delta = \mathsf{refl}$ and $\beta = \beta$, $\gamma = \gamma$: $(\mathsf{refl} \cdot \beta) \star (\gamma \cdot \mathsf{refl}) = (\mathsf{refl} \star \gamma) \cdot (\beta \star \mathsf{refl})$, i.e., $\beta \star \gamma = \gamma \cdot \beta$.

Similarly, $\beta \star \gamma = \beta \cdot \gamma$. So $\beta \cdot \gamma = \gamma \cdot \beta$. $\square$

**Consequence:** $\pi_2(A)$ is always abelian (for any type $A$ and basepoint).

## Summary

| Operation | Definition | Computation rule |
|---|---|---|
| $p \cdot q$ (concatenation) | J on $q$ | $p \cdot \mathsf{refl} \equiv p$ |
| $p^{-1}$ (inversion) | J on $p$ | $\mathsf{refl}^{-1} \equiv \mathsf{refl}$ |
| Left unit: $\mathsf{refl} \cdot p = p$ | J on $p$ | Propositional |
| Right unit: $p \cdot \mathsf{refl} = p$ | By computation | Definitional |
| Associativity: $(p \cdot q) \cdot r = p \cdot (q \cdot r)$ | J on $p$ | Propositional |
| Right inverse: $p \cdot p^{-1} = \mathsf{refl}$ | J on $p$ | Propositional |
| Left inverse: $p^{-1} \cdot p = \mathsf{refl}$ | J on $p$ | Propositional |

The identity type, together with these operations, gives every type the structure of an ∞-groupoid. The groupoid laws hold at the path level, and the same structure holds at every higher level by induction.

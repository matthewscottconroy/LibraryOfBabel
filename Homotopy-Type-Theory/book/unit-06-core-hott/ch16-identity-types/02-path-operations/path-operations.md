# Path Operations

## The Question of Structure

We have established that `a =_A b` is a type, and that its elements are paths. Now the obvious question: what can we *do* with paths?

In topology, you can do three things with paths. You can concatenate them: a path from x to y followed by a path from y to z gives a path from x to z. You can reverse them: a path from x to y gives a path from y to x. And you can compare them up to homotopy, which generates the groupoid structure of the space.

All three operations are available in type theory. And they all follow from a single rule — the J eliminator. This is not obvious. It requires thought. But it is exactly right: if J is the eliminator for the identity type, then every operation on paths must be derived from J. And indeed they all are.

## Path Concatenation

Let A be a type with elements a, b, c : A. Given paths p : a =_A b and q : b =_A c, we want to produce a path p · q : a =_A c.

The definition proceeds by J-induction on q. Fix p : a = b, and consider the family:

```
C(c, q) := (a =_A c)
```

The base case requires an element of C(b, refl_b) = (a =_A b), and we have p. So define:

```
p · refl_b := p
```

By J, this extends uniquely to give `p · q : a = c` for all c and q.

**Computation rule:** `p · refl_b ≡ p` (definitional equality — not just propositional).

Notice the asymmetry. We defined concatenation by inducting on the *second* argument q. This gives us right unit definitionally. The left unit — `refl_a · p = p` — is only provable propositionally, by J-induction on p. This asymmetry is not a defect; it is the honest expression of the fact that J can only give us one definitional computation rule.

Alternatively, we could have defined concatenation by inducting on p, getting left unit definitionally and right unit propositionally. Neither choice is privileged. They produce definitionally distinct but propositionally equal operations.

## Path Inversion

Given p : a =_A b, we want p⁻¹ : b =_A a. Apply J to p:

```
C(b, p) := (b =_A a)
```

Base case: `C(a, refl_a) = (a =_A a)`, supplied by `refl_a`. So define:

```
refl_a⁻¹ := refl_a
```

By J, this extends to give `p⁻¹ : b = a` for all p : a = b.

**Computation rule:** `refl_a⁻¹ ≡ refl_a`.

## The Groupoid Laws

With concatenation and inversion defined, we can state and prove the groupoid laws. Each law is a path — an element of an identity type. None of them are definitional (except right unit). All require proof by J.

**Theorem (Right Unit).** For p : a = b: `p · refl_b = p`.

This is *definitional* by the computation rule for concatenation. It holds before any proof is needed.

**Theorem (Left Unit).** For p : a = b: `refl_a · p = p`.

*Proof.* Apply J to p. The base case requires `refl_a · refl_a = refl_a`. By the computation rule, `refl_a · refl_a ≡ refl_a`. So the base case is proved by `refl_{refl_a}`. J extends this to all p. The witness is a path `lu_p : refl_a · p = p`. ∎

**Theorem (Associativity).** For p : a = b, q : b = c, r : c = d:
```
(p · q) · r = p · (q · r)
```

*Proof.* By J on p (the leftmost path). The base case has p = refl_a, which by left unit gives:
```
(refl_a · q) · r = q · r = refl_a · (q · r)
```
Both sides reduce to q · r by left unit. So the base case is `refl_{q·r}`. J extends to all p. ∎

Notice that we used the left unit law inside this proof — and the left unit law itself requires J. We are building a tower of J-proofs. This is completely legitimate: the rules permit using J inside J applications.

**Theorem (Right Inverse).** For p : a = b: `p · p⁻¹ = refl_a`.

*Proof.* By J on p. Base case: `refl_a · refl_a⁻¹ = refl_a`. Since `refl_a⁻¹ ≡ refl_a`, the left side is `refl_a · refl_a ≡ refl_a`. Proved by `refl_{refl_a}`. ∎

**Theorem (Left Inverse).** For p : a = b: `p⁻¹ · p = refl_b`.

*Proof.* By J on p. Base case: `refl_a⁻¹ · refl_a = refl_a`. Since `refl_a⁻¹ ≡ refl_a`, the left side is `refl_a · refl_a ≡ refl_a`. ∎

## Why Propositional, Not Definitional?

The groupoid laws hold propositionally — there are specific proof terms witnessing them — but not definitionally. The sides of each equation are not judgmentally equal. This surprises newcomers to the field.

The reason is fundamental. Definitional equality is *computational* equality. It is checked by reduction — by running the computation rules until both sides reach the same normal form. The five groupoid laws are mutually constraining, and no definition of concatenation satisfies all five by reduction.

Here is the tension. Concatenation defined by J-induction on q satisfies `p · refl ≡ p` by reduction, but `refl · p` requires a proof. If we instead induct on p, we get `refl · p ≡ p` but must prove `p · refl = p`. Associativity, in either case, requires a proof by J. There is no free lunch.

In cubical type theory (which we reach in Chapter 23), the situation changes. The interval type I with its computation rules allows concatenation to be redefined so that more laws hold definitionally. But in standard HoTT — the HoTT Book formulation — propositional equality is what we have and what we must work with.

This is not a limitation to regret. Propositional equality is *substantive* equality. When we prove `(p · q) · r = p · (q · r)`, the proof term is a specific path, a 2-path in the identity type of paths. This 2-path is mathematical content — it witnesses the specific way the associativity holds, and it participates in higher coherence conditions.

If associativity held definitionally, we would lose this content. The witness would be invisible, folded into the reduction rules. By keeping it propositional, we preserve the full homotopy-theoretic information.

## The Groupoid Structure of Every Type

Collecting what we have proved: every type A in HoTT carries a groupoid structure.

- **Objects:** terms a : A
- **Morphisms:** paths p : a = b
- **Identity:** `refl_a : a = a`
- **Composition:** `p · q : a = c`
- **Inverses:** `p⁻¹ : b = a`
- **Laws:** all five groupoid axioms, proved propositionally

This is called the *fundamental groupoid* of A. In topology, the fundamental groupoid of a space X has points of X as objects and homotopy classes of paths as morphisms. In HoTT, we do not take homotopy classes — we keep the full path space, including the path structure between paths. This gives not a groupoid but an infinity-groupoid, as we will see in the next section.

## Why "Proved by J-Induction" Means "From Refl"

A recurring phrase in this chapter: "proved by J-induction on p." What does this mean, concretely?

It means: we apply the J eliminator, which reduces the problem to the case where p is reflexivity. In that case, the left and right sides of the equation we want to prove are computationally equal (by the computation rules for concatenation and inversion), and we can use `refl` as the proof.

The J rule then tells us that this base case proof extends uniquely to cover all paths p. The extension is not trivial — J is doing real work, expanding the base case into a proof for the entire identity type. But the pattern is always the same: reduce to refl, prove the easy case, apply J.

This is path induction in practice. It is the type-theoretic counterpart of induction on the natural numbers, but for paths: to prove something for all paths, prove it for the trivial path.

## Connection to Higher Structure

The groupoid laws themselves are paths. The left unit proof `lu_p : refl_a · p = p` is an element of the identity type of paths. The associativity proof `assoc_{p,q,r} : (p·q)·r = p·(q·r)` is similarly a 2-path.

And these 2-paths themselves satisfy coherence conditions. The Mac Lane pentagon identity — that five applications of associativity around a quintuple product give the same result — is a 3-path. The tower never ends.

This infinite tower of coherence data is the content of the ∞-groupoid structure of every type. It is not extra structure we impose. It falls out of the J rule, applied iteratively, at every dimension. The next section explores this directly.

## Summary

| Operation | Defined by | Computation |
|---|---|---|
| `p · q` | J on q | `p · refl_b ≡ p` |
| `p⁻¹` | J on p | `refl_a⁻¹ ≡ refl_a` |
| Left unit | J on p | Propositional |
| Right unit | computation | Definitional |
| Associativity | J on p | Propositional |
| Right inverse | J on p | Propositional |
| Left inverse | J on p | Propositional |

Every type carries a groupoid structure. The groupoid laws hold propositionally. The proof terms are 2-paths, the next level of the identity tower.

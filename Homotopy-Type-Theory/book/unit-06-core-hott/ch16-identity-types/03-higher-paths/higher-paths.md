# Higher Paths

## The Tower That Never Ends

We have established that `a =_A b` is a type. But we have not stopped to ask: what kind of type is it? Does it have interesting structure of its own?

Yes. It does. And that structure has structure. And so on, without end.

Given two paths p, q : a =_A b, the identity type `p =_{a=_Ab} q` is the type of *2-paths* or *homotopies between paths*. An element H : p = q is a proof that p and q are the same path — not just that they connect the same points, but that they are homotopic as maps from the trivial interval into A.

There is nothing stopping us from iterating. Given two 2-paths H, K : p = q, the type `H =_{p=q} K` of 3-paths exists. And 4-paths. And so on. The tower of identity types is infinite in general.

This is the *infinity-groupoid structure* of every type.

## The Tower of Identity Types

For a type A with a : A, define the iterated loop spaces:

```
Ω⁰A = A
Ω¹A = Ω(A, a) = (a =_A a)            (the loop space at a)
Ω²A = Ω(Ω(A, a), refl_a)             (loops on the loop space)
ΩⁿA = Ω(Ωⁿ⁻¹A, ...)                 (n-fold iterated loop space)
```

The homotopy groups of A at a are:

```
πₙ(A, a) = ‖ΩⁿA‖₀
```

the set of connected components of the n-th loop space. The 0-truncation collapses the higher path structure to get an honest set (or group for n ≥ 1).

At level 2, there are two distinct operations on 2-paths. This duality is the source of the Eckmann-Hilton argument.

## Whiskering: Composing a 2-Path with a 1-Path

Given paths p, q : a = b and r : b = c, and a 2-path H : p = q, we can "whisker" H by r:

```
H ▷ r : p · r = q · r
```

This is called *right whiskering*. We compose a 2-path H (between 1-paths) with a 1-path r on the right.

Similarly, given a path l : d = a and a 2-path H : p = q, left whiskering gives:

```
l ◁ H : l · p = l · q
```

Whiskering is defined by induction: induct on r (or l) to reduce to the reflexivity case, where the whisker is trivially the identity.

These two operations give us the means to "slide" a 2-path along a context of 1-paths. They are the type-theoretic counterpart of horizontal composition in a 2-category.

## The Two Compositions on 2-Loops

Now fix a : A and consider 2-paths α, β : refl_a = refl_a — elements of Ω²A.

There are two binary operations on Ω²A:

**Vertical composition.** Concatenate α and β as paths in the loop space Ω(A, a):

```
α ·ᵥ β : refl_a = refl_a
```

This is ordinary path concatenation applied one level up.

**Horizontal composition.** Use whiskering to compose "side by side": given loops p, q at a and 2-paths α : p = refl_a and β : refl_a = q, form:

```
α ★ β = (α ▷ q) ·ᵥ (p ◁ β) : p = q
```

When both are 2-loops (p = q = refl_a), this gives another operation:

```
α ★ β : refl_a = refl_a
```

Both ·ᵥ and ★ have unit refl_{refl_a}, and they satisfy the interchange law — which says that performing vertical compositions inside a horizontal composition gives the same result as performing horizontal compositions inside a vertical one.

## The Eckmann-Hilton Argument

The interchange law has a remarkable consequence for 2-loops.

**Theorem (Eckmann-Hilton).** For α, β : refl_a = refl_a (elements of Ω²A), we have:

```
α ·ᵥ β = β ·ᵥ α
```

That is, Ω²A is abelian. Vertical composition of 2-loops commutes.

*Proof.* By the interchange law:

```
(α ·ᵥ refl) ★ (refl ·ᵥ β)  =  (α ★ refl) ·ᵥ (refl ★ β)
```

The left side simplifies (using unit laws and the definition of ★):

```
α ★ β
```

The right side simplifies (horizontal composition with refl is vertical composition, up to some units):

```
α ·ᵥ β
```

A symmetric calculation with (refl ·ᵥ α) ★ (β ·ᵥ refl) gives α ★ β = β ·ᵥ α.

Combining: α ·ᵥ β = α ★ β = β ·ᵥ α. ∎

Moreover, the same argument shows that ·ᵥ and ★ *coincide* on Ω²A. There is only one binary operation on Ω²A, and it is commutative.

**Corollary.** For any type A and basepoint a : A, the group π₂(A, a) is abelian.

This is the type-theoretic proof of a classical theorem: all homotopy groups πₙ for n ≥ 2 are abelian. The classical proof requires the suspension-loop adjunction and some diagram chasing. The type-theoretic proof is a direct calculation with the J rule and the interchange law.

## When the Tower Stabilizes

The tower of identity types is infinite in general, but for many types it stabilizes.

For a *proposition* (h-level -1, Chapter 17): every path type a = b is contractible or empty. No interesting structure at any level. The tower is trivially stable.

For a *set* (h-level 0): every path type a = b is a proposition. The paths exist, but between any two paths there is at most one 2-path (trivial). The groupoid structure is discrete.

For a *groupoid* (h-level 1): every path type a = b is a set. The 2-paths exist, but there is at most one 3-path. The 2-category is strict.

For a general *n-type*: the tower stabilizes at level n — the n-th iterated identity type is contractible or empty.

For the circle S^1: the path type base = base is equivalent to Z. The 2-path type is... more complex. The circle is not an n-type for any finite n.

The sphere S^2: the path type is contractible (there is essentially one path between any two points). But the 2-path type at base is nontrivial. S^2 is a 2-type.

The universe Type: the tower never stabilizes. The universe is not an n-type for any n.

## Path Induction at Higher Levels

The J rule applies at every level. To prove something about all 2-paths H : p = q, apply J to H and reduce to the case H = refl_p. To prove something about all 3-paths, apply J one more time.

This is path induction applied iteratively. The structure at each level is exactly the structure of the identity type at the previous level. The same rule generates the full ∞-groupoid.

This is the profound simplicity of HoTT: one rule (J), applied repeatedly, generates all of higher-dimensional algebra. The ∞-groupoid structure of every type is not an additional axiom. It is the iteration of the single axiom we already have.

## Higher Paths in Mathematical Practice

Where do higher paths appear in everyday mathematics?

**Coherence data.** When you have a monoid structure on a type A — a binary operation m : A → A → A and a unit e : A — the associativity law is a path (a 2-path in the groupoid of A). The coherence condition between different uses of associativity is a 3-path. The Mac Lane pentagon, satisfied by any monoidal category, is a 3-path condition. In HoTT, these coherence conditions are mathematical content, not bureaucracy.

**Naturality squares.** For a natural transformation α : F ⇒ G between functors, the naturality condition says that a certain square of paths commutes. Commutativity of a square is a 2-path. So naturality is inherently 2-dimensional.

**The three-fold loop space.** The 3-fold loop space Ω³A consists of 3-paths. By Eckmann-Hilton applied again, Ω³A is abelian and its operation is "commutative in the extra dimension." These higher commutativity conditions are the beginnings of a richer story — the theory of E_n algebras and operads, which classify algebraic structures by the dimension at which they become fully commutative.

## The ∞-Groupoid Structure: A Summary

Every type A in HoTT carries the following structure:

| Level | Data | Operations | Laws (propositional) |
|---|---|---|---|
| 0 | Terms a : A | — | — |
| 1 | Paths p : a = b | ·, ⁻¹, refl | Groupoid laws |
| 2 | 2-paths H : p = q | ·ᵥ, ★, whiskering | Eckmann-Hilton, interchange |
| n | n-paths | n-dimensional composition | All coherence laws |

The operations at each level are compatible with the operations at adjacent levels (functoriality of composition). All laws hold propositionally — as paths at the next level up. The entire structure is derived from the J rule, applied at each level.

This is the ∞-groupoid structure of a type. It is what classical mathematicians meant, intuitively, by a "space" — an object with points, paths between points, homotopies between paths, and so on indefinitely. HoTT makes this intuition precise, type-theoretic, and computational.

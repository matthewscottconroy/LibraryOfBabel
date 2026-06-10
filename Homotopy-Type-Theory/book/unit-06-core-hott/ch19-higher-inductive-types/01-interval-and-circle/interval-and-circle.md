# The Interval and the Circle

## The Idea: Declaring Paths

Ordinary inductive types declare *points*. N has the point 0 and the point-from-point constructor S. Lists have nil and cons. Every element of an ordinary inductive type is built from these constructors, and the only paths between elements are the ones you prove.

Higher inductive types declare *paths too*. The interval I has two points 0 and 1, and a declared path `seg : 0 = 1` between them. The circle S^1 has one point base and a declared path `loop : base = base` back to itself.

The declared paths are real paths — elements of the corresponding identity types. They are not axioms in the sense of "unverifiable claims." They are constructors, exactly like the point constructors, except they construct paths rather than points.

## The Interval Type

**Definition.** The interval I is the higher inductive type with:
- Point constructor: `0_I : I`
- Point constructor: `1_I : I`
- Path constructor: `seg : 0_I = 1_I`

**The eliminator for I.** To define a function f : I → B, specify:
- `f(0_I) = b₀ : B`
- `f(1_I) = b₁ : B`
- `ap_f(seg) = p : b₀ = b₁`

Computation rules: `f(0_I) ≡ b₀`, `f(1_I) ≡ b₁`, `ap_f(seg) = p` (propositional).

**The dependent eliminator.** To define a section s : Π(x:I). P(x), specify:
- `s(0_I) : P(0_I)`
- `s(1_I) : P(1_I)`
- `apd_s(seg) : transport^P(seg, s(0_I)) = s(1_I)`

## The Interval is Contractible

Despite having two points and a path between them, the interval is contractible.

**Theorem.** `isContr(I)`.

*Proof.* Take center `c = 0_I`. Define the contracting homotopy `h : Π(x:I). 0_I = x` using the I-eliminator:
- `h(0_I) = refl_{0_I} : 0_I = 0_I`
- `h(1_I) = seg : 0_I = 1_I`
- Transport condition: `apd_h(seg) : transport^{0_I=(−)}(seg, refl_{0_I}) = seg`

The transport computation gives: `transport^{0_I=(−)}(seg, refl) = refl · seg = seg` by left unit. So the condition holds. ∎

The interval is contractible — equivalent to 1. Why then is it interesting? Because it is *explicitly* a type with two points and a path between them, making that path available as a computational object. We can explicitly transport along `seg`, apply functions to it, and use it as a template for other paths.

## The Interval Gives Function Extensionality

The key application of the interval:

**Theorem.** If I exists (as a HIT), then function extensionality holds.

*Proof.* Let f, g : A → B with H : Π(x:A). f(x) = g(x). We want f = g.

Define k : A → I → B by the I-eliminator:
- `k(x)(0_I) = f(x)`
- `k(x)(1_I) = g(x)`
- `ap_{k(x)}(seg) = H(x)`

This works: for each x, we have points f(x) and g(x) and a path H(x) between them, which is exactly what the I-eliminator requires.

Now curry: `k̃ : I → (A → B)` defined by `k̃(i)(x) = k(x)(i)`.
- `k̃(0_I)(x) = k(x)(0_I) = f(x)`, so `k̃(0_I) = f`.
- `k̃(1_I)(x) = k(x)(1_I) = g(x)`, so `k̃(1_I) = g`.

The path `ap_{k̃}(seg) : f = g`. ∎

This proof is beautifully conceptual: a pointwise homotopy f ~ g gives a function from the interval to the function space, which maps the interval's path to a path f = g. The interval "generates" the function extensionality.

Note: this gives function extensionality *without* Univalence. In the HoTT Book, function extensionality is usually obtained from Univalence. But the interval gives an independent route.

## The Circle S^1

**Definition.** The circle S^1 is the higher inductive type with:
- Point constructor: `base : S^1`
- Path constructor: `loop : base =_{S^1} base`

The circle has one point and one non-trivial loop at that point.

**The non-dependent eliminator.** To define f : S^1 → B:
- `f(base) = b : B`
- `ap_f(loop) = ℓ : b = b` (a loop in B)

Computation: `f(base) ≡ b`, `ap_f(loop) = ℓ` (propositional).

**The dependent eliminator.** To define s : Π(x:S^1). P(x):
- `s(base) = b : P(base)`
- `apd_s(loop) = ℓ : transport^P(loop, b) = b`

The dependent eliminator requires not just a value at base but a "loop-over-b" — a proof that transporting b around the loop returns it to b.

## What Loop Means

The path constructor `loop : base = base` declares a non-trivial loop. But *is* it non-trivial? Is `loop ≠ refl_base`?

Yes. If `loop = refl_base`, then S^1 would be contractible (all loops are trivial ⇒ all points are connected by paths ⇒ contractible). But S^1 is not contractible — we prove this by exhibiting a non-constant function S^1 → Type that must transport non-trivially.

Specifically: define the code family `code : S^1 → Type` with `code(base) = Z` and `ap_code(loop) = ua(succ)` (the path corresponding to the successor equivalence, by Univalence). Then transporting `0 : Z` around the loop gives `succ(0) = 1 ≠ 0`. This shows the loop is non-trivial — it has a non-trivial effect on transport.

## Maps Out of the Circle

The eliminator says: a non-dependent function f : S^1 → B is exactly a point b : B plus a loop ℓ : b = b. Different choices of (b, ℓ) give different maps.

**Example: The winding maps.** For n : Z, the n-winding map wₙ : S^1 → S^1 corresponds to (base, loop^n): it sends base to base and maps the loop to n copies of the loop (concatenated).

**Example: The constant map.** The constant map const_b : S^1 → B corresponds to (b, refl_b): send base to b and map the loop to the trivial loop.

**Example: The identity.** id_{S^1} corresponds to (base, loop): send base to base and preserve the loop.

These examples show that the circle is "generated" by its loop: to map out of S^1 into B, you choose where to send the single generator (base, loop), subject to the constraint that the image of the loop is indeed a loop at the image of the base.

## The Fundamental Group of the Circle

The key theorem about S^1:

**Theorem.** `(base =_{S^1} base) ≃ Z`

The proof is the encode-decode computation of Chapter 20, using:
1. The code family `code : S^1 → Type` with `code(base) = Z` and loop acting as successor.
2. Encode: transport in code gives the winding number.
3. Decode: n ↦ loop^n gives a loop for each integer.
4. These are mutual inverses.

The circle is therefore a 1-type whose fundamental group is Z. This is the type-theoretic realization of the classical topological theorem — now proved synthetically, from the HIT definition alone.

## Summary

| HIT | Point constructors | Path constructors | Key property |
|---|---|---|---|
| I (interval) | 0_I, 1_I | seg : 0_I = 1_I | Contractible; gives funext |
| S^1 (circle) | base | loop : base = base | π₁(S^1) = Z |

Both HITs demonstrate the power of path constructors: by declaring specific paths as part of the type's definition, we force the type to have specific homotopy properties. The interval forces there to be a path between its two endpoints (and nothing more — the type is contractible). The circle forces there to be a non-trivial loop at its basepoint, which becomes the generator of Z.

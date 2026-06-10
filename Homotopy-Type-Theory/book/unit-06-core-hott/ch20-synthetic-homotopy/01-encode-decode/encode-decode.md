# The Encode-Decode Method

## The Key Question

To compute the fundamental group π₁(X, x₀) of a space X at a basepoint x₀, we need to understand the loop space Ω(X, x₀) = (x₀ =_X x₀). We want to show this type is equivalent to some explicit type (usually Z or some other group).

In classical topology, you would:
1. Find the universal cover X̃.
2. Use covering space theory to identify fibers of X̃ → X with π₁(X).
3. Compute the fiber.

In HoTT, the analog of "finding the universal cover" is finding the right code family.

## The General Framework

**Setup.** We want to show `(x₀ = x₀) ≃ G` for some type G.

**Step 1: Choose a code family.** Define `code : X → Type` such that:
- `code(x₀) = G` (the code at the basepoint is the group we want)
- The transport behavior of code is the group action

The code family is the crucial creative step — choosing the wrong code makes everything fail; choosing the right code makes the rest almost automatic.

**Step 2: Define encode.** Set:
```
encode : Π(x:X). (x₀ = x) → code(x)
encode(x, p) = transport^code(p, e₀)
```

where e₀ : code(x₀) is a "basepoint" of the code at x₀ (usually 0 for Z).

At x = x₀: `encode(x₀, p) = transport^code(p, e₀) : code(x₀) = G`.

So encode sends each loop p : x₀ = x₀ to an element of G, by transporting the basepoint element e₀ along p.

**Step 3: Define decode.** Define:
```
decode : Π(x:X). code(x) → (x₀ = x)
```

At the basepoint: `decode(x₀) : G → (x₀ = x₀)` must be defined, and then extended to all of X using the HIT or type structure of X.

**Step 4: Prove they are inverses.** Show:
```
Π(x:X). Π(p: x₀=x). decode(x, encode(x, p)) = p
Π(c: code(x₀)). encode(x₀, decode(x₀, c)) = c
```

If both hold, then encode and decode form a mutual inverse, establishing the equivalence `(x₀ = x₀) ≃ code(x₀) = G`.

## Why Transport is Encode

The encode map is always transport. This is not a coincidence — it is the fundamental insight of the encode-decode method.

In classical covering space theory, the monodromy action of π₁(X) on the fiber of a covering space is "parallel transport" of the fiber along paths. In HoTT, this is literally transport in a type family: transport^code(p) acts on code(x₀) and gives the "monodromy" action of p on G.

The code family `code : X → Type` is the HoTT version of the covering space. The fiber `code(x₀) = G` is the fiber of the covering space over the basepoint. Transport in code is parallel transport along paths — exactly the monodromy.

Choosing the right code family is choosing the right covering space.

## The Decode Map and the HIT Eliminator

The decode map requires defining a dependent function Π(x:X). code(x) → (x₀ = x). For a HIT X, this uses the dependent eliminator:

- At the basepoint x₀: `decode(x₀) : G → (x₀ = x₀)` is defined explicitly.
- At path constructors of X: the transport condition constrains how decode extends.

For the circle S¹:
- `decode(base, n) = loop^n` (the n-fold loop)
- The transport condition along loop: `transport^{code(−) → base=−}(loop, decode(base)) = decode(base)`.

This transport condition reduces to: `Π(n:Z). loop^{pred(n)} · loop = loop^n`, which holds by definition of loop powers.

## The Round Trip Proofs

**decode ∘ encode = id (on paths):**

We need: `Π(x:X). Π(p: x₀=x). decode(x, transport^code(p, e₀)) = p`.

This is proved by J-induction on p. The base case (p = refl_{x₀}):
```
decode(x₀, transport^code(refl, e₀)) = decode(x₀, e₀) = loop^0 = refl_{x₀}
```

J extends this to all p. (Note: this requires that decode(x₀, e₀) = refl_{x₀}, which we ensure by choosing e₀ = 0 in Z and loop^0 = refl.)

**encode ∘ decode = id (on code):**

We need: `Π(c: code(x₀)). transport^code(decode(x₀, c), e₀) = c`.

This is proved by induction on c. For c = n : Z:
```
transport^code(loop^n, 0) = succ^n(0) = n
```

by the computation that transport along loop = succ, applied n times.

## The Key Computation: Transport = Succ

The whole proof of π₁(S¹) = Z reduces to one computation:

```
transport^code(loop) : Z → Z  =  succ
```

Why? Because code is defined by `ap_code(loop) = ua(succ)` (the Univalence axiom applied to the successor equivalence). The computation rule for ua then gives:

```
transport^{id}(ua(succ)) = fun(succ) = succ
```

And transport^code along loop is transport^{id} along ap_code(loop) = ua(succ), giving succ.

This one computation — that transport in the code family along the generating loop is the successor — drives the entire proof. Every loop computation is a consequence of iterating this single fact.

## Generalizations of Encode-Decode

The encode-decode method applies to many types beyond S¹:

**The n-sphere S^n.** Code family `code : S^n → Type` with `code(base) = Z` and path conditions enforcing that n-fold suspension gives Z as the n-dimensional winding number.

**BG (classifying space of G).** Code family with `code(pt) = G` and transport along each g-loop = left-multiplication by g. The decode map is g ↦ g-loop. The result: Ω(BG, pt) ≃ G.

**K(G, n).** Iterated application: the loop space of K(G,n) is K(G, n-1). The encode-decode proof at each level uses the code family for the previous level.

**General quotient types.** For a type A/R (quotient by equivalence relation R), the path type `[a] = [b]` is equivalent to `‖R(a,b)‖` (the propositional truncation of R). The encode-decode method with `code([b]) = ‖R(a,b)‖` gives this computation.

## Why This Replaces Covering Space Theory

In classical topology:
- Covering spaces exist for "nice" spaces (semi-locally simply connected spaces)
- Constructing the universal cover requires choice (choosing a maximal tree in a triangulation)
- The covering space fibration sequence requires several lemmas
- The monodromy representation requires computing fiber transport

In HoTT:
- Code families always exist (just define them using the HIT eliminator)
- No choice is required (the code family is defined deterministically)
- The encode-decode argument replaces the fibration sequence
- Transport is built into the type theory

The encode-decode method is more general, more direct, and more computational than covering space theory. It applies to any HIT-defined space, without any conditions on the space, and it gives a computable proof (the encode function is literally the winding number computation).

## Summary

| Step | Classical | HoTT |
|---|---|---|
| 1 | Find universal cover | Define code family |
| 2 | Construct monodromy action | Transport in code = group action |
| 3 | Compute fiber | Compute code(x₀) |
| 4 | Prove faithfulness | decode ∘ encode = id via J |
| 5 | Prove surjectivity | encode ∘ decode = id via group induction |

The encode-decode method is the synthetic version of covering space theory. It is cleaner, more general, and directly computable.

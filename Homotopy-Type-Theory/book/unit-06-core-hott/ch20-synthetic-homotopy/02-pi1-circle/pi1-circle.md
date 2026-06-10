# π₁(S¹) = Z: The Full Proof

## The Benchmark

The computation π₁(S¹) = Z is the "hello world" of synthetic homotopy theory. Every technique introduced in this unit is used here:

- The circle S¹ as a HIT (Chapter 19)
- The identity type as a path space (Chapter 16)
- Transport and the code family (Chapter 16)
- The Univalence Axiom to define the code (Chapter 18)
- The encode-decode method (Chapter 20, Section 1)

The result: the loop space Ω(S¹, base) = (base =_{S¹} base) is equivalent to Z. Loops in the circle biject with integers. The bijection is the winding number.

## Recall: The Circle S¹

S¹ is the HIT with:
- `base : S¹`
- `loop : base = base`

A dependent function s : Π(x:S¹). P(x) is determined by:
- `s(base) : P(base)`
- `apd_s(loop) : transport^P(loop, s(base)) = s(base)`

## Step 1: The Code Family

Define `code : S¹ → Type` using the non-dependent S¹-eliminator (a map into Type):

- `code(base) := Z`
- `ap_code(loop) := ua(succ)` — the Univalence axiom applied to the successor equivalence `succ : Z ≃ Z`

The computation rule for ua gives:
```
transport^code(loop) : Z → Z  =  succ   (successor: n ↦ n+1)
transport^code(loop⁻¹) : Z → Z  =  pred  (predecessor: n ↦ n-1)
```

**Why this is the right code.** Each traversal of the loop increments the "winding number" by 1. Transport in code along loop = succ captures exactly this: going around the loop once adds 1.

## Step 2: Encode

Define:
```
encode : Π(x:S¹). (base = x) → code(x)
encode(x, p) := transport^code(p, 0)
```

where 0 : Z is the integer zero (the "starting winding number").

**Key computation.** For the loop iterated n times:
```
encode(base, loop^n) = transport^code(loop^n, 0) = succ^n(0) = n
```

*Proof by induction on n:*
- n = 0: `encode(base, refl) = transport^code(refl, 0) = 0` ✓
- n+1: `encode(base, loop^{n+1}) = encode(base, loop^n · loop)`
         `= transport^code(loop^n · loop, 0)`
         `= transport^code(loop, transport^code(loop^n, 0))` (transport over concat)
         `= succ(encode(base, loop^n)) = succ(n) = n+1` ✓
- n = -1: `encode(base, loop⁻¹) = transport^code(loop⁻¹, 0) = pred(0) = -1` ✓
- n-1: Similar, using transport along loop⁻¹ = pred.

## Step 3: Decode

Define:
```
decode : Π(x:S¹). code(x) → (base = x)
```

At the basepoint: `decode(base) : Z → (base = base)` by `decode(base, n) := loop^n`

where:
```
loop^0      := refl_base
loop^{n+1}  := loop^n · loop
loop^{-1}   := loop⁻¹
loop^{-(n+1)} := loop^{-n} · loop⁻¹
```

**Extending decode to all of S¹.** Use the dependent S¹-eliminator with:
- `decode(base) = λn. loop^n` (as defined above)
- Transport condition: need `apd_decode(loop)`, i.e., need:
  ```
  transport^{code(−) → base=−}(loop, decode(base)) = decode(base)
  ```

By the transport formula for function types:
```
transport^{code → base=−}(loop, f) = λc. transport^{base=−}(loop, f(transport^code(loop⁻¹, c)))
```

So the condition becomes: for all n : Z,
```
transport^{base=−}(loop, loop^{pred(n)}) = loop^n
```

Transport in the right path family concatenates: `transport^{base=−}(loop, p) = p · loop`.

So: `loop^{pred(n)} · loop = loop^n`. This holds by definition: `loop^{n-1} · loop = loop^n`. ✓

So decode extends to all of S¹ and is well-defined.

## Step 4: Round Trip — decode ∘ encode = id

Need: `Π(x:S¹). Π(p: base=x). decode(x, encode(x, p)) = p`

*Proof by path induction (J) on p.* The motive is C(x,p) := (decode(x, encode(x,p)) = p). By J, it suffices to check C(base, refl_{base}):

```
decode(base, encode(base, refl)) = decode(base, 0) = loop^0 = refl_{base}
```

J extends this to all p. ✓

## Step 5: Round Trip — encode ∘ decode = id

Need: `Π(n:Z). encode(base, decode(base, n)) = n`

i.e., `Π(n:Z). encode(base, loop^n) = n`

*Proof by integer induction.* We showed this in Step 2:
- `encode(base, loop^0) = 0` ✓
- `encode(base, loop^{n+1}) = succ(encode(base, loop^n))` ✓
- `encode(base, loop^{n-1}) = pred(encode(base, loop^n))` ✓

By induction on Z, `encode(base, loop^n) = n` for all n. ✓

## The Main Theorem

**Theorem.** `(base =_{S¹} base) ≃ Z`

*Proof.* The maps encode(base) and decode(base) are mutual inverses (Steps 4 and 5). They form a homotopy equivalence. ∎

**Corollary.** `π₁(S¹, base) = Z`

*Proof.* π₁(S¹, base) = ‖base = base‖₀ ≃ ‖Z‖₀ = Z (since Z is already a set). ∎

## What the Proof Shows

**1. Univalence is essential.** The code family is defined using ua(succ). Without Univalence, we cannot define a type family that "counts" loop traversals — the key computation `transport^code(loop) = succ` requires the ua computation rule.

**2. The proof is computable.** The encode function is literally a program:
```
windingNumber : (base = base) → Z
windingNumber p = encode(base, p) = transport^code(p, 0)
```

In Cubical Agda (with definitional computation rules for ua), this function can be run: `windingNumber(loop) = 1`, `windingNumber(loop · loop) = 2`, etc.

**3. The proof is constructive.** We don't just know that π₁(S¹) = Z — we have an explicit bijection, with both directions computable.

**4. No topology was used.** The proof uses only:
- The HIT definition of S¹
- Transport and J (from Chapter 16)
- The Univalence computation rule (Chapter 18)
- Integer arithmetic

No covering spaces. No CW complexes. No singular homology. No long exact sequences. Just type theory.

## Comparison with the Classical Proof

The classical proof of π₁(S¹) = Z runs roughly as follows:

1. Define the covering space R → S¹ (the "winding" map e^{2πit} : R → S¹).
2. Show R is simply connected (contractible).
3. Apply the long exact sequence of the fibration R → S¹ with fiber Z.
4. Conclude π₁(S¹) = Z from the exact sequence.

Each step requires substantial background:
- Covering spaces require the theory of fiber bundles.
- The long exact sequence requires singular homology.
- Simply-connectedness of R requires ε-δ analysis.

The total length of a rigorous classical proof, starting from first principles, is 50+ pages.

The HoTT proof is 2-3 pages, starting from the definitions of Chapter 19.

The HoTT proof is not "easier" in the sense of requiring less mathematics. It is more direct, because the language of HoTT — paths, transport, higher inductive types — is *native* to the subject matter. In the classical proof, you are building homotopy theory on top of set theory, and the overhead is the translation. In HoTT, the homotopy theory *is* the type theory.

## Summary

| Step | Content |
|---|---|
| code : S¹ → Type | code(base) = Z, transport along loop = succ |
| encode(base, p) | transport^code(p, 0) — the winding number |
| decode(base, n) | loop^n — the n-fold loop |
| decode ∘ encode = id | By J on the path p |
| encode ∘ decode = id | By integer induction |
| Conclusion | (base = base) ≃ Z |

The proof of π₁(S¹) = Z is the paradigmatic result of synthetic homotopy theory. Every subsequent computation in this chapter uses the same encode-decode pattern with different code families. The circle sets the template; the rest fills it in.

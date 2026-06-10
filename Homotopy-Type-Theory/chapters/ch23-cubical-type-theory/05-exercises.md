# Chapter 23 Exercises: Cubical Type Theory

---

## Section 1: The Interval and Paths

**Exercise 1.1.** In CCHM cubical type theory, verify the following definitional equalities using the De Morgan algebra laws:

1. `sym (sym p) = p` — because `λ i → p (~(~i)) = λ i → p i`
2. `sym refl = refl` — because `λ i → a (~i) = λ i → a`
3. `cong f (sym p) = sym (cong f p)` — trace through the definitions

**Exercise 1.2.** Describe what the following 2-cube represents geometrically:

```
H : (i j : I) → A
H i j = p (i ∧ j)
```

where `p : a ≡ b`. Draw the square with labeled faces, showing what the four faces are. What homotopy does `H` witness?

**Exercise 1.3.** Define path concatenation using `hcomp` and verify the two endpoints:

```
p ∙ q := λ i → hcomp (i=0) (λ j → [i=0 ↦ p j]) (q i)
```

Show that `(p ∙ q) i0 = a` and `(p ∙ q) i1 = c` where `p : a ≡ b` and `q : b ≡ c`.

**Exercise 1.4.** Show that the J eliminator (path induction) is derivable in cubical type theory. Specifically:

- State what J says
- Sketch how transport along `λ i → P (p i) (?)` gives the result
- What is the computation rule for J (at `refl`)?

---

## Section 2: Composition and Transport

**Exercise 2.1.** Compute `transp (λ i → Bool) i0 true`. What is the result? Why?

**Exercise 2.2.** Compute `transp (λ i → ua succEquiv i) i0 (pos 3)`. What should the result be? Trace through the Glue type computation rules.

**Exercise 2.3.** For a $\Sigma$-type $\Sigma_{x:A} B(x)$ and a path of $\Sigma$-types $P : (i : \mathbb{I}) \to \Sigma\text{-type}$, describe the computation rule for `transp P (a₀, b₀)`. What are the two components of the result?

**Exercise 2.4.** Prove the right unit law for path concatenation: `p ∙ refl ≡ p`. 

*Hint:* Define a 2-cube `H : (i j : I) → A` using `i ∨ ~j` or similar, and verify the boundary conditions.

**Exercise 2.5.** Prove that transport along `refl` is the identity:

```
transp (λ i → A) i0 a = a
```

(This should hold definitionally — which computation rule makes it true?)

**Exercise 2.6.** The Kan condition for the path type: show that `hcomp` for the path type `a ≡ b` is defined pointwise. That is:

```
hcomp φ (λ j → [φ ↦ λ i → u j i]) (λ i → a₀ i) = λ i → hcomp φ (λ j → u j i) (a₀ i)
```

What does this say geometrically?

---

## Section 3: The Glue Type and Univalence

**Exercise 3.1.** Verify the computation rule for `ua`:

```
transport (ua e) a = e.fst a
```

Trace through the following steps:
1. `transport (ua e) a = transp (λ i → ua e i) i0 a`
2. `ua e i = Glue [(i=0) ↦ (A, e)] B` (for appropriate faces)
3. Apply the `transp` rule for Glue types
4. Conclude

**Exercise 3.2.** Show that `ua (idEquiv A) = refl`:

1. `ua (idEquiv A) = λ i → Glue [...] A`
2. When the equivalence is `idEquiv`, what does the Glue type simplify to?
3. Conclude the path is the constant path `λ i → A = refl`

**Exercise 3.3.** Prove propositional extensionality from univalence: if `P Q : Prop` and `P ↔ Q`, then `P = Q`.

*Hint:* A proposition `P` has an equivalence `P ≃ Q` from any `P ↔ Q` (the coherences are automatic because any two proofs of a proposition are equal). Apply `ua` to this equivalence.

**Exercise 3.4.** State the following in cubical type theory (not just in Cubical Agda):

1. The type `isEquiv f` for a function `f : A → B`
2. The Glue type former with its introduction and elimination rules
3. The univalence theorem as a statement about `idToEquiv`

**Exercise 3.5.** In CCHM, the composition in the universe `hcomp_φ^Type(u, A₀)` is defined using the Glue type. Describe what this looks like when:
1. `φ = 0` (no constraint): what is the result?
2. `φ = (i = 0) ∨ (i = 1)` (both faces): what is the result?
3. `φ = (i = 0)` (left face only): what is the result in terms of `A₀` and `u(1)`?

---

## Section 4: Variations

**Exercise 4.1.** In Cartesian cubical type theory (CCTT), path reversal is not definitional. Describe how to define `sym p` using `hcomp` instead of the complement `~`. What is the type of the composition you need to fill?

**Exercise 4.2.** Explain why `sym (sym p) = p` might not hold *definitionally* in CCTT, even though it holds up to a homotopy. What does the homotopy look like?

**Exercise 4.3.** XTT satisfies boundary separation: if two terms agree on all faces, they're definitionally equal. Show how this makes associativity of path concatenation definitional:

1. `(p ∙ q) ∙ r` and `p ∙ (q ∙ r)` should agree on all faces of the appropriate cube
2. By boundary separation, they're definitionally equal

**Exercise 4.4.** The presheaf model of CCHM uses the cube category $\square$. Describe:

1. What are the objects and morphisms of $\square$?
2. What does a fibrant cubical set look like in low dimensions (0-, 1-, 2-cells)?
3. What is the Kan condition for a fibrant cubical set, in terms of horn-filling?

**Exercise 4.5 (Research).** Read the CCHM paper: "Cubical Type Theory: A Constructive Interpretation of the Univalence Axiom" (Cohen, Coquand, Huber, Mörtberg, 2015). 

Answer the following questions:
1. How is the interval $\mathbb{I}$ defined formally in the paper?
2. Where exactly does the Glue type appear, and what are its introduction and elimination rules as stated in the paper?
3. What is the main theorem of the paper, and how is canonicity stated?
4. How does the paper define `hcomp` for function types? Trace through one step of the computation.

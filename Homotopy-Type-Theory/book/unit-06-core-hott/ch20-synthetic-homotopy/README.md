# Chapter 20: Synthetic Homotopy Theory

Before homotopy type theory, computing the fundamental group of the circle required 50 pages of algebraic topology: singular homology, van Kampen's theorem, covering space theory, and eventually a calculation that felt like it was fighting the notation all the way. In HoTT, the proof that π₁(S¹) = Z takes about 5 pages. Not because the mathematics is easier — because the language is right.

When the identity type *is* the path space, when HITs *are* the CW complexes, when transport *is* parallel transport, the proofs become what they always were in the topologist's intuition: paths. This chapter does homotopy theory the way it should have been done all along.

There is something profound happening here that goes beyond economy of notation. The classical proof of π₁(S¹) = Z requires:

1. Defining the circle as a quotient of R by the integer action.
2. Constructing the universal cover (the real line R).
3. Proving that the covering map is a fibration.
4. Applying the long exact sequence of a fibration.
5. Computing the end terms.

Each of these steps requires apparatus — the theory of covering spaces, the definition of fibrations, the long exact sequence — that must be built up before it can be applied. The apparatus is not wrong; it is beautiful. But it is also scaffolding. And scaffolding is not the same as the building.

In HoTT, the scaffolding disappears. The circle is defined by one point and one loop. The "covering space" is the code family, defined by declaring what transport does along the loop. The fibration sequence is the encode-decode setup. The calculation is direct: encode and decode are inverses, and each sends loop^n to n.

The scaffolding disappears because the language is native to the mathematics. In the classical setting, you are modeling topology in set theory and you must constantly translate. In HoTT, you are doing topology. There is no translation.

## The Central Technique

The encode-decode method is the technical heart of synthetic homotopy theory. To compute the loop space Ω(X, x₀) of a space X:

1. Define a *code* type family `code : X → Type` where `code(x₀)` is the group you expect π₁(X) to be.
2. Define *encode*: transport in code gives a map from loops to code values.
3. Define *decode*: code values give loops, by specifying what each element of code(x₀) does.
4. Prove encode and decode are mutual inverses.

This method reduces homotopy group computation to:
- Choosing the right code family (the creative step)
- Verifying transport computations (the technical step)
- Applying J or HIT eliminators (the structural step)

## What We Prove

This chapter proves:

- **π₁(S¹) = Z** — The circle's fundamental group is the integers. The benchmark computation of HoTT.

- **van Kampen** — The fundamental group of a pushout is the amalgamated free product. A clean synthetic proof from the pushout universal property.

- **Freudenthal** — For n-connected A, the suspension map A → ΩΣA is (2n+1)-connected. The stability of homotopy groups.

- **Hopf fibration and π₃(S²) = Z** — The Hopf fibration S¹ → S³ → S², and the computation of the third homotopy group of S².

These are classical theorems, with classical proofs that fill textbooks. The synthetic proofs in HoTT are shorter — but more importantly, they are *explanatory*. They show *why* these theorems are true, in a language where the why is transparent.

## What This Chapter Is

This chapter is a demonstration. Everything we have built — identity types, h-levels, univalence, higher inductive types — comes together here to compute things.

Read it as a proof of concept. These are not the last theorems in synthetic homotopy theory — they are among the first. The field is active, with proofs of the Blakers-Massey theorem, the Seifert-van Kampen theorem in full generality, and computations of increasingly many homotopy groups being developed. What we prove in this chapter is enough to understand the power of the synthetic approach and to see what makes it distinctive.

After this chapter, you will have the foundations. What you do with them is up to you.

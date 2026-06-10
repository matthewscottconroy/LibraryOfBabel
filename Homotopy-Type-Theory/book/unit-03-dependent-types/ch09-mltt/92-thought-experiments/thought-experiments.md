# Thought Experiments: Chapter 9

## Experiment 1: Reading a Proof as a Program

Take a concrete theorem in MLTT: the commutativity of addition, n + m = m + n. In HoTT Agda, the proof is:

```agda
+-comm : (n m : ℕ) → n + m ≡ m + n
+-comm zero    m = sym (+-zero m)
+-comm (suc n) m = trans (cong suc (+-comm n m)) (sym (+-suc m n))
```

This is a term of type Π(n m : ℕ). n + m = m + n. It is simultaneously:
1. A mathematical proof (correct and complete)
2. A program (it computes, given n and m, a specific path between n + m and m + n)

**The thought experiment:** What does this program *do* when you run it? On input (3, 5), it produces a path 3 + 5 = 5 + 3 in ℕ. What is that path as a data structure? It is an element of the identity type — but in MLTT, every element of ℕ = ℕ is refl (by a corollary of the fact that ℕ is a set). So the path is refl, and the program outputs refl. But the computation to get there — the induction steps, the congruence applications — actually runs.

Now consider: is there any case where the path would be *different* from refl? In ℕ (which is a set, with at most one path between any two elements), no. But for types that are not sets — types like the universe Type₀ or the circle S¹ — paths between points can be non-trivial, and the "output" of the program would be a genuine non-trivial path. The program would compute a specific *homotopy*, not just a certificate.

This illustrates the distinction between set-level mathematics (where proofs are trivial computationally) and homotopy-level mathematics (where proofs are genuine computational objects).

## Experiment 2: The J Rule Under the Microscope

Consider the J rule as a machine. It takes:
- A motive C : Π(b:A).(a = b) → Type (what you want to prove)
- A base case d : C(a, refl_a) (proof for the trivial path)
- A specific path p : a = b (the path you want to apply the proof to)

And it outputs: an element of C(b, p).

**The question:** How does J know how to get from d : C(a, refl_a) to an element of C(b, p), when b might be different from a and p might be a complex path?

**The answer:** J does not "know." It is an axiom — a primitive rule of the system. The justification comes from the semantics (the contractibility of the based path space), not from a computational reduction.

But there is a computation rule: J(C, d, a, refl_a) ≡ d. J computes on the single constructor (refl). For any other path p : a = b, J does not reduce further — it is stuck. In intensional MLTT, J applied to a non-refl path is a normal form.

**The thought experiment:** What would it mean for J to have a more detailed computation rule — one that also reduced for specific non-refl paths? In cubical type theory, this is exactly what happens. The interval [0,1] is a type, paths are functions from [0,1], and J reduces on all paths by "pulling back" along the interval. The computation rule for J in cubical type theory is geometric: it uses the geometry of the interval to define how J reduces.

This is the solution to the "missing computation rule" problem for univalence: by making the path structure geometric and computational, cubical type theory gives J a complete set of computation rules, not just the one for refl.

## Experiment 3: What Does a Non-Trivial Identity Proof Look Like?

In ordinary mathematics, proofs of equality are transparent — you prove 2 + 2 = 4 by computation, and there is only one way to do it. But in HoTT, there can be non-trivial identity proofs.

Consider the type A = S¹ (the circle, defined as a HIT). The self-identity proofs at the basepoint — elements of base =_{S¹} base — include:
- refl : base = base (the trivial path)
- loop : base = base (going around once)
- loop · loop : base = base (going around twice)
- loop⁻¹ : base = base (going around once in reverse)
- ... in general, loop^n for any integer n

So the "proof" that base = base is a specific path, labeled by an integer (the winding number). Two "proofs" of base = base are equal (as elements of base =_{S¹} base) iff they have the same winding number.

**The thought experiment:** Could this have happened in classical mathematics? Well, yes — in homotopy theory, the path space of the circle from base to base is exactly the integers ℤ. But in classical foundations, this fact is proved about a specific mathematical structure (the circle as a topological space) using exterior methods (algebraic topology).

In HoTT, the same fact is proved *inside* the type theory. The circle is not a structure being studied from outside — it is a type in the type theory itself. The winding number is not an external invariant — it is computable from the path (via the recursion principle of S¹). This is what "synthetic homotopy theory" means: the topology is done in the type language itself, not about external mathematical structures.

## Experiment 4: The Reflection Rule and the Oracle Problem

Suppose you added the reflection rule to intensional MLTT. You now have extensional MLTT. Consider the type-checking problem:

**Input:** A context Γ and two terms a, b and a claimed proof p : a =_A b.
**Question:** Is Γ ⊢ p : a =_A b a valid judgment?

In intensional MLTT, this is decidable: normalize a, b, and p; check types syntactically.

In extensional MLTT: the same question, but now with the side effect that if p : a = b is valid, then a ≡ b definitionally. The type checker must potentially apply the reflection rule recursively — each reflection gives new definitional equalities, which enable new terms to type-check, which may give new propositional equalities, which by reflection give more definitional equalities...

**The oracle problem:** Consider encoding the halting problem in extensional MLTT. Define a term H(n) : ℕ → Bool where H(n) = true if Turing machine n halts and false otherwise. Define p(n) : H(n) = true iff n halts. In extensional MLTT, having p(n) : H(n) = true allows the reflection rule to accept H(n) ≡ true definitionally. The type checker, when asked whether a term type-checks, must determine whether the Turing machine n halts — an undecidable problem.

This is not a paradox — the system is still consistent. It just means the type checker cannot be automated. A human must guide it, providing explicit evidence at each step. NuPRL handles this by requiring the user to provide a proof term for every type-checking obligation.

**The lesson:** Decidability of type-checking is not a minor technical convenience. It is what makes dependent type theory a practical tool for verified software. An undecidable type checker makes automation impossible, limits tooling, and requires mathematical expertise at every step of a proof. Intensional MLTT preserves decidability; extensional MLTT trades it for convenience. HoTT makes the intensional choice.

## Experiment 5: Martin-Löf's Identity Type vs. Leibniz Equality

There are two natural ways to define equality in type theory.

**Martin-Löf's identity type:** a =_A b is an inductive type with one constructor (refl) and eliminator J. Elements of a = b are paths.

**Leibniz equality:** a =_Leibniz b is defined as Π(P : A → Type). P(a) → P(b). An element of a =_Leibniz b is a function that, given any predicate P and a proof that P holds at a, produces a proof that P holds at b.

**The thought experiment:** Are these the same?

**One direction is easy:** From refl_a : a =_ML a, derive an element of a =_Leibniz a: the function λP. λh. h (using the fact that P(a) → P(a) trivially). More generally, from p : a =_ML b, use J to derive an element of a =_Leibniz b.

**The other direction is harder:** From H : a =_Leibniz b, can we derive p : a =_ML b? Apply H to the predicate P(x) = (a =_ML x) with proof refl_a : P(a) = (a =_ML a). The result is H(P, refl_a) : P(b) = (a =_ML b). So yes — Leibniz equality implies Martin-Löf equality.

**The difference in structure:** Martin-Löf's identity type carries the *path* explicitly as data — the proof term is the path itself. Leibniz equality carries a *function* — the method of "transporting" any predicate. The two are logically equivalent (each implies the other) but differ in their higher-dimensional behavior.

In HoTT, Martin-Löf's identity type is correct: the path is the fundamental datum. Leibniz equality, when internalized in the same way, would collapse (by impredicativity) to a proof-irrelevant notion of equality. Martin-Löf's version is proof-relevant — different paths are different proofs — and this is what makes HoTT possible.

The moral: how you define equality is not just a formality. It determines what higher structure is possible.

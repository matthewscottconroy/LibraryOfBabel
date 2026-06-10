# Thought Experiments: Cubical Agda and the Nature of Mathematical Computation

## Thought Experiment 1: The Stuck Proof

In axiomatic HoTT (the HoTT Book's foundation), consider this term:

```
t := transport (ua (isoToEquiv swapIso)) true : Bool
```

This term has type `Bool`. By the intended semantics of HoTT, it should equal `false` — transporting `true` across the equivalence that swaps the two booleans should give `false`. And in HoTT, we can *prove* `t ≡ false` (using the propositional computation rule for univalence).

But `t` is stuck. The normalizer cannot reduce it to `false`. It cannot reduce it to anything. It is a closed term of type `Bool` that is not `true` and not `false` in any computational sense — it is a third thing, an axiom-dependent blob.

This is the canonicity failure. Canonical forms for `Bool` are `true` and `false`. A closed term of type `Bool` that is neither is a violation.

*Questions for reflection:* Does this matter? If you can *prove* that `t ≡ false`, is that not enough? What does it mean for a proof to be "correct" if the proof object cannot be evaluated? Is there a difference between knowing that something is true and being able to compute what it is?

The canonicity debate in type theory reflects a deeper question: is mathematics about *truth* (propositions with proof-irrelevant witnesses) or about *construction* (programs with computable content)? Cubical type theory takes the constructivist position. Axiomatic HoTT takes a middle position. Classical mathematics takes the classical position. All three are consistent; which is "right" depends on what mathematics is for.

---

## Thought Experiment 2: The Geometry of Proof

In Cubical Agda, a proof that `a ≡ a` is a function `I → A`. A proof that two proofs `p q : a ≡ b` are equal is a function `I → I → A`. A proof of the pentagon identity for path concatenation is a function `I → I → I → A`.

Mathematics is being done in cubes.

This is not a metaphor. The mathematical objects are literally functions from higher-dimensional cubes (products of interval copies) to your type. The dimension of the cube you're working in corresponds to the homotopy dimension of the argument.

Intuitively: a 0-dimensional thing is a point (an element of `A`). A 1-dimensional thing is a path (a function `I → A`). A 2-dimensional thing is a square (a function `I → I → A`). A 3-dimensional thing is a cube (a function `I → I → I → A`).

When mathematicians talk about "filling in a square" or "building a cube of homotopies," cubical type theory is taking this literally. The geometric intuition is not just motivating — it is the actual mathematical content.

*Questions for reflection:* Does this make the mathematics easier to understand or harder? Is there a risk that the cubical structure is too specific — that some "natural" HoTT constructions don't fit the cubical mold? Are there other geometric shapes (simplices, globes, opetopes) that would give a different, equally valid computational foundation? What would each choice mean for the resulting type theory?

---

## Thought Experiment 3: Proof-as-Program, Deeply

In Cubical Agda, the proof that π₁(S¹) = ℤ is a program. Specifically, the function `encode : (base ≡ base) → ℤ` is the winding number algorithm. You can call it on specific loops and get specific integers.

Now consider: the Freudenthal suspension theorem says that the stable homotopy groups of spheres are eventually constant. The proof (in Cubical Agda) is a program. Is it also an algorithm for computing stable homotopy groups?

In principle, yes. The proof of Freudenthal is a term whose type guarantees certain equivalences between homotopy groups. That term, evaluated on specific inputs, computes specific group elements. This is mathematics as algorithm, generated automatically by the proof.

Now extend this: Brunerie's theorem says π₄(S³) = ℤ/2ℤ. The Brunerie number computes to 2. Is the proof of Brunerie's theorem a program that computes π₄(S³)? In principle, yes. In practice, after the 2022 optimizations, it is even tractable.

*Questions for reflection:* What would it mean to have machine-verified algorithms for computing all homotopy groups of spheres? Is this a realistic research goal? What barriers remain? Is there a sense in which HoTT could provide a new algorithmic approach to problems in algebraic topology — not just verifying known computations, but discovering new ones?

---

## Thought Experiment 4: The Boundary Between Agda and Mathematics

There is a moment in every Cubical Agda proof when you type `C-c C-n` (normalize) and wait. For simple computations, the wait is milliseconds. For the Brunerie number (in 2016), the wait was hours. For some computations, it may be longer than the age of the universe.

This raises a boundary question: where does "mathematics" end and "computation" begin?

Traditionally: mathematics is about what *exists* (or can be proved to exist), not about what can be *computed*. The axiom of choice guarantees the existence of a well-ordering of the reals; no one can exhibit one explicitly. Classically, this is fine.

In cubical type theory: every proof is a computation. The existence statement and the algorithm are the same. If the algorithm takes longer than the age of the universe, has anything been proved?

One answer: yes, because the logical correctness of the proof does not depend on the efficiency of the normalization. The term is well-typed; therefore the theorem is true. The normalization is just a check, not the proof itself.

Another answer: in the constructive tradition, a proof of existence means a construction, and a construction that cannot be performed (even in principle, even in theory) is not a construction at all. If the Brunerie number's normalization took infinite time, the proof would be empty.

A third answer: this is an engineering problem, not a philosophical one. Make the normalization faster. This is what Ljungström and Mörtberg did in 2022.

*Questions for reflection:* Is computational tractability a mathematical virtue or a practical contingency? Should type theory distinguish between "logically correct" and "computably correct"? Is there a version of HoTT where correctness requires polynomial-time normalization?

---

## Thought Experiment 5: What Would Fully Automated HoTT Look Like?

Lean 4 has `exact?`, `simp`, `ring`, and `aesop` — automation that can close large classes of goals without human guidance. Cubical Agda has much less automation.

This is partly a maturity difference: Lean 4 has been developed for longer, by a larger team, with more resources. But it is also a deeper difference: Lean 4's automation relies on the uniform structure of propositional goals (closed by SMT-style reasoning, simp, ring normalization). Cubical Agda's goals are not just propositions — they are types with computational content, and their structure is richer.

Imagine a future version of Cubical Agda with powerful automation. What would it look like?

The system would need to:
1. Recognize when a goal is a "routine" homotopy-theoretic argument (encode-decode, path induction, transport calculation)
2. Search the Cubical library for applicable equivalences and isomorphisms
3. Normalize cubical expressions efficiently
4. Automatically construct box-filling arguments (find the `hcomp` that closes the goal)

Is this achievable? Parts of it probably are. Automated proof search in dependent type theories is an active research area. Machine learning on Lean 4 proofs (the `LeanDojo` project, the `Hypertree Proof Search` approach) suggests that neural-guided proof search can be effective in typed settings.

*Questions for reflection:* Would automated Cubical Agda be the same discipline as manual Cubical Agda? Would mathematicians using it understand their proofs? Is there value in the struggle — in the hours spent at a terminal filling in cubical arguments — that would be lost if the automation did the work?

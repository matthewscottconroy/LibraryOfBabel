# Thought Experiments: Proof Theory

## 1. The Library of All Proofs

Imagine a library containing one book for every valid formal derivation of the sequent $\vdash \top$ (the provable theorem "true is true"). There are infinitely many such derivations: the trivial one-step proof, and also a proof that proves $\top$ then immediately uses it to prove $\top$ again, and a proof that proves $\top \wedge \top$ and projects out the first component, and so on infinitely. Each book in the library is a different proof object.

Classical logic says all these books contain "the same theorem" — they all establish $\top$. Proof theory says the books are genuinely different objects. Now suppose we introduce the type $\top$ — the unit type — whose only element is $\star$. The classical reading says all these books give the same element of $\top$: just $\star$. But in HoTT, could they give different elements? Could two different proofs of $\top$ be *unequal* as inhabitants of the type?

For the unit type, the answer is no: $\top$ is a proposition (an h-set with at most one element), and any two proofs are propositionally equal. But for more complex types, the analogous question is live: two different proofs of $A = B$ are not automatically equal, and the space of proofs of $A = B$ is the loop space of the type $A$.

What distinguishes the types for which "all proofs are equal" from those for which they can differ? This is the question that the h-level hierarchy in HoTT answers.

## 2. The Detour That Was Necessary

The normalization theorem says every detour can be removed. But consider this: in some cases, removing a detour makes the proof longer before it gets shorter. A sequence of reductions might first increase proof size, then decrease it. Could there be an infinite sequence of reductions that keeps increasing size without ever stabilizing?

For the simply typed lambda calculus, the answer is no — strong normalization rules this out. But for *classical* logic with continuations (call/cc), the answer is yes: certain reduction sequences in the $\lambda\mu$-calculus (Parigot's calculus for classical logic) can diverge. What does this mean for the consistency of classical logic as a computational system?

Think through the relationship: classical logic is consistent (no proof of $\bot$ exists), but its computational interpretation via continuations is not strongly normalizing. Can a system be logically consistent but computationally divergent? What does this tell us about the relationship between logical consistency and computational termination?

## 3. The Proof That Never Knew What It Was Proving

Imagine you are given a derivation tree — a complete, formal proof — but with all formula labels removed. You see only the *shape* of the tree: which rule was applied at each node, which nodes are leaves, how many premises each internal node has. Can you recover the formulas from the shape alone?

In general, no. The shape of a derivation does not determine the formulas it proves. But for proofs in *normal form* — the canonical form guaranteed by normalization — the shape constrains the formulas dramatically. Given a normal form proof shape, the subformula property says every formula must come from the conclusion and hypotheses. This means you can often recover much of the formula structure from the shape.

Now think about this computationally. The *type* of a typed lambda term determines its behavior — a term of type $A \to A$ must be the identity function if $A$ is universally quantified (Reynolds' parametricity). The *shape* of a proof is, under Curry-Howard, the *type* of the corresponding term. What does the shape of a normal form proof tell us about the computation it represents?

## 4. The Structural Rule That Changed Everything

Suppose we drop the *contraction* rule from propositional logic: each hypothesis can be used at most once. This is *affine logic*. Now ask: which theorems are no longer provable?

The theorem $A \to A \to A$ (which says "if $A$, then if $A$, then $A$") is no longer provable: using the single hypothesis twice would require contraction. But $A \to B \to A$ is still provable: the hypothesis $B$ is unused (weakening still holds), and $A$ is used exactly once.

This might seem like an odd restriction. But consider the computational interpretation: in affine logic, each function argument can be used at most once. This corresponds to *linear types* or *ownership* in programming languages. Rust's borrow checker enforces affine discipline: a value can be moved (consumed) or borrowed (used with a reference), but not duplicated without an explicit clone. The structural rule of contraction corresponds directly to the ability to copy values.

Which classical theorems fail in affine logic? Which structural properties of proofs does the absence of contraction break? And what does it mean, computationally, that the proof "uses" the hypothesis only once?

## 5. Consistency Without a Model

The standard proof of logical consistency goes: "we have a model in which all axioms are true, therefore the system is consistent (no contradiction follows, since the model would have to make $\bot$ true, which is impossible)."

Gentzen's proof of consistency is entirely different: it is syntactic. It works from the structure of derivation trees — no model, no semantics, no appeal to truth. The system is consistent because the structure of proofs in normal form rules out the existence of a proof of $\bot$.

Now ask: is the syntactic proof "more" or "less" certain than the semantic proof? The semantic proof uses a model — which presupposes the existence of that model, which presupposes the consistency of the meta-theory in which the model is constructed. The syntactic proof uses an ordinal ($\varepsilon_0$) that is beyond what Peano Arithmetic can prove — which presupposes the legitimacy of that ordinal, which presupposes... what?

This regress is precisely what Gödel's theorems predict. There is no proof of consistency that uses only the tools of the system being certified. Every consistency proof must reach outside. The interesting question is: how far outside, and what does that mean?

## 6. The Proof-as-Object Shift

In classical mathematics, we say "there exists a proof of the Pythagorean Theorem" but we do not usually ask about the internal structure of that proof, how many lines it has, or whether it is in normal form. All valid proofs of the same theorem are interchangeable.

Now consider: what would it mean for mathematics if proofs were not interchangeable? If "I proved it this way" and "I proved it that way" were genuinely different mathematical acts with genuinely different consequences?

In HoTT, this is the situation. Two proofs of $a = b$ can be genuinely unequal as elements of the type $a =_A b$. The set of proofs that $a = b$ can itself have non-trivial structure — it can form a non-trivial topological space (a higher groupoid).

What would mathematics look like if proof identity mattered everywhere — not just in HoTT, but in ordinary mathematical practice? Would the question "which proof did you use?" become as important as "what did you prove"? Could different proofs of the same result carry different mathematical information?

## 7. The Longest Cut-Free Proof

Cut elimination guarantees that every proof with cut can be transformed into a cut-free proof. But cut-free proofs can be exponentially longer than proofs with cut. This is not just a theoretical bound — it is achieved in practice: there exist families of tautologies whose shortest proofs with cut are polynomial in length, but whose shortest cut-free proofs are exponential.

This exponential blowup is related to the $\mathsf{P}$ vs $\mathsf{NP}$ question. If $\mathsf{P} = \mathsf{NP}$, then every tautology has a short proof (polynomial in length) because we could efficiently search for one. If $\mathsf{P} \neq \mathsf{NP}$, some tautologies require super-polynomial proofs.

So: the structure of proofs — whether cut-free or with cut, how long they need to be — is connected to the central question of computational complexity. Does this mean that the question "can this theorem be proved efficiently?" is meaningful? Could there be true theorems with no short proof, not because the theorem is unprovable, but because every proof is necessarily long?

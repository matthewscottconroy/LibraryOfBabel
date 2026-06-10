# Validity and Soundness

> *"A valid argument is one where the truth of the premises guarantees the truth of the conclusion — not as a matter of empirical fact, but as a matter of logical necessity."*

---

Two philosophers are arguing. One says: "Look, my conclusion must be correct. The premises are well-established facts, and the reasoning is airtight — you cannot accept the premises and reject the conclusion without contradicting yourself." The other replies: "Your reasoning may be perfectly airtight, but your premises are false. A perfectly valid argument from false premises tells us nothing about the world."

Both philosophers are right. They are articulating, from different angles, the distinction between **validity** and **soundness** — two concepts that are easy to confuse but that carve out very different logical territories.

## Validity: Truth Cannot Escape

A deductive argument is **valid** if it is *impossible* for the premises to be true and the conclusion false simultaneously. Note carefully what this says: it does not say the premises *are* true; it says that *if* they were true, the conclusion would have to be true too. Validity is about the *logical relationship* between premises and conclusion — it is a claim about necessity, not about actuality.

More formally: an argument is valid iff in every possible world (every possible state of affairs) where all the premises hold, the conclusion also holds. The conclusion cannot be false when the premises are true — in any world, at any time, for any interpretation of the terms involved.

This is why logicians say that in a valid argument, "the truth of the conclusion is contained in the truth of the premises." The conclusion does not add new factual content; it merely makes explicit what was already implicit in the premises. This is the central feature that distinguishes deductive reasoning from inductive reasoning.

**Examples of valid arguments**:

1. All humans are mortal. Socrates is human. Therefore Socrates is mortal.
2. If it rains, the ground gets wet. It is raining. Therefore the ground is wet.
3. All bachelors are unmarried. Fred is a bachelor. Therefore Fred is unmarried.
4. Either the butler did it or the gardener did it. The butler didn't do it. Therefore the gardener did it.

In each case, accepting the premises and rejecting the conclusion is a logical contradiction.

**A valid argument with false premises and a false conclusion**:

All fish can fly. Salmon are fish. Therefore salmon can fly.

This argument is *valid*: if the premises were true, the conclusion would have to be true. The fact that the premises are false — and that the conclusion is correspondingly false — does not affect the validity. Validity is about *form*, not about whether the claims happen to be true in our world.

**A valid argument with false premises but a true conclusion**:

All even numbers are prime. 4 is even. Therefore 4 is prime.

Again valid (the premises guarantee the conclusion), the premises are false, and here the conclusion happens to be false too. But consider:

All prime numbers are greater than 1. 7 is prime. Therefore 7 is greater than 1.

This is valid, the premises are true, and the conclusion is true. Also valid, and both premises true but conclusion is an independent truth.

Actually: **All multiples of 4 are multiples of 2. 12 is a multiple of 4. Therefore 12 is a multiple of 2.** Valid, both premises true, conclusion true. This is also a *sound* argument.

The key insight: a valid argument with false premises is a logical tool without factual content. It tells us what *would* follow *if* the premises were true — which can still be enormously useful if we are reasoning hypothetically, or if we are trying to derive consequences from axioms whose truth we are entertaining.

## Soundness: Validity Plus Truth

A **sound** argument is one that is valid *and* has all true premises.

If an argument is sound, its conclusion must be true. This follows from the definition: if it is valid, the premises' truth guarantees the conclusion's truth; if the premises are actually true, then the conclusion is actually true. A sound argument is as strong as a deductive argument can possibly be.

Notice the asymmetry: soundness implies validity (you cannot be sound without being valid), but validity does not imply soundness (a valid argument might have false premises).

The relationship can be mapped:

|  | Valid | Invalid |
|--|-------|---------|
| **True premises** | Sound ✓ (conclusion guaranteed true) | Useless (conclusion unknown) |
| **False premises** | Valid but unsound (conclusion unknown) | Doubly bad |

The cell "invalid with true premises" deserves emphasis: an invalid argument can have true premises and a true conclusion *by accident*, without the premises logically supporting the conclusion. Consider:

> Barack Obama was born in Hawaii. The Pacific Ocean is the largest ocean. Therefore the sum of all angles in a triangle is 180 degrees.

Both premises are true. The conclusion is true. But the argument is spectacularly invalid — the premises have nothing to do with the conclusion. Truth of premises and truth of conclusion, by themselves, guarantee nothing about the logical relationship between them.

## How Validity Is Established and Refuted

To **prove** an argument is valid, we typically:
1. Use a formal proof system (natural deduction, sequent calculus, a proof assistant) to construct an explicit derivation
2. Show that the argument form is a tautology (in propositional logic, by truth table)
3. Argue semantically that no counterexample exists

To **refute** an argument — to show it is *invalid* — we exhibit a **counterexample**: a specific interpretation in which all the premises are true but the conclusion is false. A single counterexample suffices to establish invalidity; this is one of the beauties of deductive logic's black-and-white structure.

**Example of refutation**: Is this argument valid?
> All cats are mammals. All mammals are warm-blooded. Therefore all warm-blooded things are cats.

The counterexample: dogs are mammals, hence warm-blooded, and not cats. So: domain = {dogs, cats}, both are mammals, both are warm-blooded. The premises are satisfied, but "all warm-blooded things are cats" is false. The argument is invalid.

> **Try It**: In Lean 4 and Coq, you cannot prove an invalid argument — the proof assistant will simply fail to find a proof, no matter how long you try. In Python with Z3, you can *search for a counterexample* by asking the solver to find an interpretation where the premises are true and the conclusion is false. If Z3 returns SAT with a model, that model is your counterexample. If it returns UNSAT, the argument is valid.

## The Epistemological Role of Deduction

Here is a question worth pausing on: if a valid deductive argument cannot take us beyond what is "contained in" the premises, what is the epistemological value of deduction? Why bother proving things we already know, in some sense?

The answer is that knowledge has many dimensions. A collection of axioms may implicitly contain an enormous amount of information that is not *explicitly* apparent — information that requires sophisticated deduction to extract. The Pythagorean theorem is, in some sense, already "contained" in Euclid's five axioms. But it took Euclid's proof to make that implicit content explicit and visible. Mathematical discovery is largely the activity of making the implicit explicit.

Moreover, deductive proof provides *certainty*: the kind of certainty that no other method of reasoning offers. Inductive generalizations from observed data, abductive inferences to the best explanation, probabilistic reasoning under uncertainty — all of these give us degrees of confidence, not guarantees. A valid deductive proof from true premises is the one form of reasoning that delivers an *unconditional* guarantee of truth.

This is why mathematics, uniquely among the sciences, produces permanent knowledge. Physical theories are revised as new data comes in. Mathematical theorems, once proved, are proved forever. The Pythagorean theorem will never be falsified by an experiment. That permanence is the gift of deductive validity.

---

## Tool Connections

**Python / Z3**: The Z3 SMT solver directly implements validity checking and counterexample finding. `z3.solve(Not(conclusion), *premises)` asks: is there a model where the premises hold but the conclusion fails? UNSAT means valid; SAT with a model means invalid (and gives you the counterexample).

**Lean 4**: A successful Lean proof is a machine-verified certificate of validity (actually, of soundness — since the proof was constructed under the type system's scrutiny, the premises are encoded as hypotheses, and the proof term witnesses that the conclusion follows). If you cannot construct the proof, Lean's type-checker is, in effect, looking for a counterexample.

**Tarski's World**: The "Game" feature in Tarski's World / Carnap is exactly a counterexample-search game. You and the software alternate: the software tries to show a sentence is false; you try to show it is true. Winning the game corresponds to having a valid argument.

---

*Next: We catalog the ways arguments go wrong — the formal and informal fallacies.*

# Thought Experiments: Logic and Proof

## 1. The Martian Logician

Imagine a civilization on Mars that developed mathematics independently. They have their own notation, their own proof techniques, their own history. They developed something that looks very much like what we call propositional logic. But their connectives are slightly different: they have a connective they call "nand" (not-and), and they derive everything else from it. NAND(P, Q) is true whenever it is not the case that both P and Q are true.

Question: Is their logic the same as ours, or different? If you can define ∧, ∨, ¬, →, ↔ in terms of NAND, and they can define NAND in terms of your connectives, what does that tell you about the relationship between the two systems? Is there a "canonical" set of logical connectives, or are all choices equally valid?

Now make this harder. The Martians interpret their NAND symbol in a way that is truth-functionally equivalent to ours, but they think it means something completely different — they have a very different story about what logic *is*. Does this matter? Can two people using the same formal rules while having radically different interpretations of those rules be said to be practicing "the same logic"?

This thought experiment drives toward a question the logical positivists debated: can there be alternative logics? Can we change the logical rules themselves, or are some rules (like non-contradiction) so basic that changing them would simply mean we were no longer "doing logic"?

## 2. The Proof That Takes Too Long

Suppose we discover a proof of the Riemann Hypothesis. The proof is valid — every step follows from the previous by a recognized inference rule, and the final line is the Riemann Hypothesis. But the proof is 10¹⁰⁰⁰ steps long. No human being could read it in the lifetime of the universe. No computer in any practical sense could check it. Does the existence of this proof mean the Riemann Hypothesis is "proved"?

Follow-up: suppose instead that a computer finds the proof and we verify the computer's verification program — we check that the program correctly implements the inference rules, and the program reports "valid." Is that sufficient? What if we then find a bug in the verification program?

This thought experiment is not idle: the first computer-assisted proofs of the four-color theorem (1976) and the Kepler conjecture (1998) raised exactly these questions. Is a proof that only a machine can check a proof in the intended sense? What is the epistemological difference between a proof no human can read directly and a proof a human can follow step by step?

In the context of HoTT: proof assistants like Lean and Coq generate proof terms that are checked by a small kernel. The kernel is small enough to audit by hand. This is the answer the formal mathematics community gives to this question: it is enough to trust the kernel. Is that answer satisfying?

## 3. The Liar and the Goat

The Liar paradox: "This sentence is false." If true, it's false; if false, it's true. Gödel took this paradox and made it into mathematics: a sentence in arithmetic that says "I am not provable in this system." He showed the sentence is neither provable nor disprovable (in a consistent system strong enough to express the construction).

Now consider a strengthened version. Suppose we add the Gödel sentence G as an axiom. Now we have a stronger system. That system has its own Gödel sentence G'. Add G'. And so on, transfinitely. Each system, at each stage, is consistent and proves more. Is there a "limit" system that proves everything provable? No — each extension has its own Gödel sentence, and the process never ends.

Question: what does this tell us about the relationship between truth and proof? Is there a fixed notion of "mathematical truth" that transcends any particular proof system? Or is truth always relative to a system? Gödel himself believed in mathematical Platonism — a fixed realm of mathematical truth that formal systems approximate from below. What evidence, if any, supports this? What would it mean to *refute* it?

## 4. The Curry-Howard Machine

You are handed a lambda term: `λ(f: A → B). λ(g: B → C). λ(x: A). g(f(x))`. You are also handed a natural deduction proof: "From P → Q and Q → R, derive P → R (hypothetical syllogism)." You notice they have the same structure.

Now you are handed a much longer, more complex lambda term — a function that sorts lists. What is its type? It is a type that says: "given a list, produce a sorted list with the same elements and a proof that it is sorted." So the sorting function is simultaneously a proof that sorting is possible.

Question: if proofs are programs, what are *incorrect* programs? Under Curry-Howard, an ill-typed program is an invalid proof. A type error is a logical error. A compiler rejection is a proof rejection. But what about a program that type-checks but produces wrong answers — a function that returns the "wrong" sorted list? That program is still a valid proof — of what? Of the proposition encoded by its type, which might be weaker than what we wanted.

This reveals a crucial point: the Curry-Howard correspondence is only as strong as the *specification* — the type. A function that type-checks proves its specification, nothing more. In formal verification, writing the right specification is often harder than writing the program. Is "correct" code impossible to define without a specification? What is the connection between this and the model-theoretic fact that no first-order theory can have a unique model?

## 5. Intuitionistic Logic and Lost Counterexamples

Brouwer argued that mathematics is a mental construction. A proof of "P or Q" must either exhibit a proof of P or exhibit a proof of Q — it cannot simply derive a contradiction from ¬P ∧ ¬Q and conclude "therefore P ∨ Q." The law of excluded middle (P ∨ ¬P for every P) is not valid for Brouwer, because for some P we may have neither proved P nor proved ¬P.

Consider: "Either there are infinitely many twin primes, or there are only finitely many." This is an instance of excluded middle. Classically: obviously true. Intuitionistically: not yet proved, because we have no proof of either alternative.

Now consider a specific "oracle number" α defined as: α = 0 if the twin prime conjecture is false, α = 1 if it is true. Classically, α is a well-defined real number (either 0 or 1). Intuitionistically, α is not a definite number until the twin prime conjecture is settled.

What does this imply for real analysis? For the intermediate value theorem? For constructive mathematics? If you reject excluded middle, you get a different, weaker real analysis — but one that has the advantage that every proof of existence is a proof with a witness, and every computed real number has an algorithm.

Is intuitionistic logic a rival to classical logic, or a refinement of it? Can you adopt intuitionistic logic without giving up "believing in" classical mathematics?

## 6. The Impredicative Circle

Russell's paradox arises from impredicative definition: the set R is defined using the class of all sets, which includes R itself. Russell's solution — a type hierarchy — prevents any object from being defined in terms of objects of its own type.

Impredicativity appears in logic too. The proposition "P is provable" quantifies over all proofs, including proofs of "P is provable" itself. In second-order logic, "there exists a property P such that..." quantifies over all properties, including properties defined by second-order formulas.

Martin-Löf's original type theory (1971) had an impredicative universe: Type : Type, meaning the universe of all types was itself a type. Girard showed this is inconsistent — System U, the dependent type theory with Type : Type, is inconsistent (Girard's paradox, 1972), analogous to Russell's paradox. The fix: a hierarchy of universes, just like Russell's hierarchy of types.

Question: why does allowing Type : Type cause inconsistency, intuitively? What does the self-referential construction look like? And — in HoTT — the Univalence Axiom works in an impredicative-looking way: it allows paths between types to be identified with equivalences between types, where the equivalences themselves involve types. How does HoTT avoid Girard's paradox while still being expressive enough to do mathematics?

## 7. What Does a Proof Explain?

We have two proofs of the sum formula 1 + 2 + ... + n = n(n+1)/2. The first is induction: base case, inductive step, done. The second is Gauss's method: write the sum forward and backward, add term by term, each pair sums to n+1, divide by 2. Same conclusion, completely different arguments.

Are these two proofs "the same proof"? They prove the same thing. But they offer different understanding. The induction proof verifies without explaining. The Gauss proof shows *why* the formula is true — the structure is visible.

Now: does the Curry-Howard correspondence preserve explanatory value? The induction proof corresponds to a recursive program. The Gauss proof corresponds to a different, non-recursive program. Both type-check with the same type. But one is more illuminating.

Question: in a formal system where proofs are terms and terms are proofs, is there a notion of "more explanatory proof"? How would you formalize it? This is the question of *proof relevance* in type theory — the question of whether proofs themselves carry mathematical content beyond the proposition they prove — which becomes crucial in HoTT, where different proofs of the same proposition can be genuinely different objects (different paths in a space).

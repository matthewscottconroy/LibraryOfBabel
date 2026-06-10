# Thought Experiments: Lean 4, Formalization, and the Nature of Proof

## Thought Experiment 1: The Referee's Dilemma

You are refereeing a paper for *Annals of Mathematics*. The paper proves a new theorem in algebraic number theory. The proof is 80 pages long, uses techniques from five different subfields, and contains several steps marked "the reader can verify." You have been checking it for three months and are 95% confident it is correct.

The question: Is 95% confidence acceptable for mathematical publication?

Historically, the answer was "yes" — and for most of the 20th century, this was the actual standard. Referees checked arguments, not every step. Errors were caught (sometimes) by subsequent readers. The famous 1988 error in the claimed proof of the four-color theorem by Kempe, originally accepted, was corrected — eventually. Mathematical error has traditionally been self-correcting.

But here is the pressure: your paper, if published, will be cited by other papers, whose conclusions will depend on its correctness. If it's wrong, the error propagates. The longer the chain, the harder the error is to find.

What would it take to be 100% confident? A machine-checked proof. When you submit a Lean 4 formalization, the kernel either accepts it or rejects it. There is no 95%. There is no "probably." There is only: does it type-check?

*Questions for reflection:* Is 100% certainty achievable? What does the kernel actually check — the proof, or the proof relative to the axioms? If the kernel itself contains a bug, what then? Is machine-checked proof a response to the referee's dilemma, or does it merely relocate the dilemma?

---

## Thought Experiment 2: The Language Shapes the Mathematics

Gonthier's formalization of the four-color theorem revealed errors in the informal proof. Not errors in the *theorem* — the theorem is still true — but errors in the argument that was supposed to prove it. In at least two places, the informal proof was doing something that could not be justified in the formal setting.

This raises a philosophical question: what is an informal proof, really?

One view: an informal proof is an abbreviation of a formal proof. The abbreviation omits routine steps, uses shorthand notation, and trusts the reader to fill in the gaps. The formal proof is the "real" proof; the informal one is a presentation.

Another view: informal proofs are a different kind of object. They convey *insight* — why something is true, not just that it is. Formal proofs verify correctness but can obscure the structure. A Lean 4 proof of the four-color theorem is correct but may be less illuminating than the informal argument.

A third view: the gap between formal and informal proofs is where mathematics lives. Mathematicians work in the informal mode not because formalization is too hard, but because the informal mode is richer — it allows intuitions, heuristics, and generalizations that formal systems don't yet support.

*Questions for reflection:* Which view do you find most compelling? Does formalization change what it means to understand a proof? Is there mathematical content that is expressible informally but cannot be formalized (even in principle)?

---

## Thought Experiment 3: The Lean 4 World

Imagine a world where all mathematics published after 2030 must include a machine-checked formalization. Papers are accepted only when their Lean 4 or Agda proof compiles.

What changes?

*Positive changes:* No more "proof by authority" — theorems are true or false, independent of reputation. No more errors that propagate for decades. No more refereeing uncertainty. The entire mathematical literature becomes a formally certified library.

*Negative changes:* Formalization is expensive. A 30-page paper might require 6 months of Lean 4 work. This would heavily favor results that are formalizable over results that are deep but hard to formalize. Intuition-driven work, conjectural mathematics, and exploratory research might be disadvantaged.

*Unexpected changes:* New collaborations between mathematicians and computer scientists. New tools that partially automate formalization. New mathematical results discovered during formalization (as happened with the four-color theorem). A new profession: "formalization engineer," someone who specializes in converting informal proofs to formal ones.

*Questions for reflection:* Is this world better or worse than the current one? What parts of mathematical practice would be most disrupted? Is there a middle path — where formalization is one tool among many, used selectively for high-stakes results?

---

## Thought Experiment 4: What Does Lean 4 Not Know?

Lean 4's kernel verifies proofs relative to its axioms: the Calculus of Inductive Constructions, propositional extensionality, functional extensionality, and quotient types. Mathlib adds no additional axioms beyond these (with some exceptions for `Classical.choice`).

But these axioms could, in principle, be inconsistent. If they are inconsistent, then every proposition is provable — and Lean 4 would accept proofs of `False`, making every `#check` succeed and every theorem meaningless.

As of 2025, no inconsistency has been found, and there are semantic models (in the category of sets) that show the axioms are consistent relative to ZFC. So we have relative consistency: Lean 4 is consistent if ZFC is consistent. And ZFC is consistent if... we don't know for certain, by Gödel.

The question: what is the epistemic status of a Lean 4-verified theorem?

It is: *correct, assuming the axioms of Lean 4 are consistent, assuming the kernel implementation is bug-free, assuming the hardware executes the computation correctly.*

These assumptions are all very likely to be true. But they are assumptions.

*Questions for reflection:* Does this chain of assumptions undermine the value of machine-checked proof? Or does it simply locate it more precisely than informal proof does? (Informal proofs also rest on assumptions — just implicit ones.) What would it mean to have an "absolutely certain" proof?

---

## Thought Experiment 5: Lean 4 and Mathematical Creativity

Critics of formalization sometimes worry that it will change mathematical practice in a harmful way — training a generation of mathematicians who can write Lean 4 proofs but can't see the underlying geometry, who can check correctness but can't generate new ideas.

Is this a real danger?

Consider the analogy with calculators. When calculators became ubiquitous, some educators worried that students would lose arithmetic skills and the number sense that goes with them. The worry had some validity: mental arithmetic skills did decline. But mathematicians didn't stop doing mathematics. They offloaded the routine arithmetic to the machine and focused on the structurally interesting questions.

Perhaps formalization works similarly. Lean 4 handles the routine logical bookkeeping. Mathematicians focus on the novel ideas. The proof assistant becomes an amplifier, not a replacement.

But there's a disanalogy. Arithmetic is a tool for doing mathematics; the proofs are the mathematics. If proof-writing becomes a machine task, what is left for mathematicians to do? Perhaps: identifying interesting theorems to prove, developing new mathematical structures, and exercising exactly the geometric and categorical intuition that machines lack.

*Questions for reflection:* Does formalization change the kind of mathematical intuition that's valuable? Is there a sense in which formalizing a proof teaches you something different about it than writing it informally? What kind of mathematical creativity is distinctively human?

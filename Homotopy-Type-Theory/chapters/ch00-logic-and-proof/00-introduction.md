# Chapter 0: Logic and the Art of Proof

## What This Chapter Is Really About

Mathematics is unusual among human enterprises. A well-written proof, if it is correct, is *correct forever*. No new experiment can falsify it, no future discovery can undermine it. The reason for this unusual permanence is that mathematics rests on *deduction* — the extraction of conclusions from premises through rules that are themselves beyond dispute.

Logic is the study of those rules. It asks: what does it mean for one statement to *follow from* others? When have we genuinely established something, and when are we merely making a plausible case? These questions have precise answers, and this chapter builds them from the ground up.

This is not merely background material. There is a direct line from the logical principles of this chapter to the formal systems at the heart of Homotopy Type Theory:

- The *inductive definitions* we use to build logical syntax are the same construction as *inductive types* in type theory.
- The *proof rules* we introduce (natural deduction) are, under the Curry-Howard correspondence, the *typing rules* for a programming language.
- The *quantifiers* $\forall$ and $\exists$ become the *dependent types* $\Pi$ and $\Sigma$.
- The *principle of induction* becomes the *eliminator* for the natural number type.

So when you learn to write proofs carefully in this chapter, you are not just learning a skill for this curriculum — you are internalizing the structure of formal proof systems that you will later encode in a computer.

## Why Rigor?

Students sometimes experience mathematical rigor as a kind of pedantry — a demand for excessive formality that obscures the real ideas. There's a real tension here worth acknowledging honestly.

On one hand, informal mathematical reasoning at a high level is genuinely illuminating. A short intuitive argument can convey more understanding in one paragraph than a five-page formal proof. Mathematical creativity lives at this informal level.

On the other hand, the informal level is where errors hide. History is littered with plausible-looking arguments that turned out to be wrong. Cauchy "proved" that the limit of a sequence of continuous functions is continuous — a theorem that is false without additional conditions (uniform convergence). Ramanujan made spectacular conjectures, some of which proved very hard to verify rigorously. Errors even appear in published work by excellent mathematicians.

More relevantly for this curriculum: when you write a proof in a proof assistant like Lean or Coq, there is no informal level. Every step must be justified by explicit rules. The machine is utterly unsentimental. Learning rigorous proof writing here is practice for that environment.

The goal is not to be formal for its own sake. The goal is to develop the *discipline* of being honest about every step — a discipline that ultimately makes your informal reasoning cleaner and your formal reasoning possible.

## Roadmap

- **Section 1** builds propositional logic: syntax (what formulas look like), semantics (what they mean via truth tables), and the notion of logical entailment.
- **Section 2** introduces the main proof techniques: direct proof, contrapositive, contradiction, and case analysis. Each is a logical principle made into a practical strategy.
- **Section 3** develops mathematical induction in several forms, culminating in well-founded induction — the most general version and the one most relevant to type theory.
- **Section 4** extends to predicate logic, introducing quantifiers and the proof rules that govern them.
- **Section 5** discusses standards for proof-writing: what constitutes a complete proof, and what common mistakes to avoid.

By the end, you should be able to write proofs that are rigorous by the standards demanded throughout this curriculum, and you should see the beginnings of how logical structure maps onto type-theoretic structure.

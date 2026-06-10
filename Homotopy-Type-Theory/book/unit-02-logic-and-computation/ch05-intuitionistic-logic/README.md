# Chapter 5: Intuitionistic Logic and Constructive Mathematics

## The Man Who Thought Logic Was Wrong

Luitzen Egbertus Jan Brouwer was not a modest thinker. He believed that mathematics was a fundamentally mental activity — that mathematical objects exist only in the mind of the mathematician who constructs them, not in some Platonic realm of abstract entities. He believed that the Law of Excluded Middle — the principle that every proposition is either true or false — was simply incorrect. And he believed that classical mathematicians, including some of the greatest minds of his era, were making systematic errors.

His contemporaries thought he was a crank. David Hilbert, the most powerful mathematician of the age, dismissed Brouwer's critique of classical logic as "taking away from the mathematician the tools he needs to work, the same as prohibiting the boxer the use of his fists." Hermann Weyl, initially sympathetic, eventually returned to classical mathematics. The community did not follow Brouwer.

He was not a crank.

What Brouwer discovered — though he would never have described it in these terms — was the logic of *computable functions*. His intuitionistic logic, stripped of the Law of Excluded Middle and double-negation elimination, turns out to be exactly the logic of programs that terminate, functions that compute explicit values, and proofs that carry concrete witnesses. When the computer was invented and the theory of computability was developed, the mathematics Brouwer had been doing all along came into focus.

More precisely: if you interpret a proof of $P$ as a program of type $P$, and a proof of $\exists x, P(x)$ as a program that produces a specific $x$ satisfying $P$, then the Law of Excluded Middle becomes the claim that there is a program which, for any proposition $P$, either proves $P$ or disproves it — a universal decision procedure. No such program exists. The halting problem is undecidable. The Riemann Hypothesis has no known proof or disproof. The Law of Excluded Middle asserts the existence of an oracle we do not have.

Brouwer was not discovering a restriction. He was discovering the logic of what we can actually compute and construct.

## Why Intuitionistic Logic for HoTT?

Homotopy Type Theory is built on constructive foundations. This is not a philosophical preference — it is a mathematical necessity.

The identity type $a =_A b$ in HoTT is a type whose elements are *proofs* that $a$ and $b$ are equal. Different proofs of the same equality can themselves be unequal. The space of proofs of $a = b$ can be a non-trivial topological object — a loop space, a sphere, or something more exotic. This "proof relevance" — the fact that different proofs of the same thing are genuinely different mathematical objects — is intrinsic to HoTT. It is the source of the homotopy-theoretic structure.

Classical logic destroys proof relevance. Classical logic says: any two proofs of the same proposition are equal (since all that matters is truth, and two proofs of the same true thing are "the same"). Intuitionistic logic does not say this — and HoTT exploits this to build a mathematics where the internal structure of proofs carries information.

The axiom of univalence — that equivalent types are equal — is also essentially constructive: it says that the *evidence* of an equivalence is itself a mathematical object that can be transported along equalities. This requires that the proofs of equivalence (the functions back and forth together with their homotopies) actually exist as terms, not just as abstract guarantees.

## The Structure of This Chapter

**Section 1: The BHK Interpretation.** Before formalism, intuition. The Brouwer-Heyting-Kolmogorov interpretation says exactly what it means to have a constructive proof of each kind of proposition — and why the Law of Excluded Middle cannot be constructively validated.

**Section 2: Formal Intuitionistic Logic.** We formalize the intuitionistic propositional calculus (IPC), identify which classical tautologies fail and why, and prove the Disjunction Property and Existence Property — the key properties distinguishing intuitionistic from classical logic.

**Section 3: Kripke Semantics.** Intuitionistic logic has a natural semantics in terms of possible worlds (stages of knowledge). We develop Kripke models, prove soundness and completeness, and use them to show that LEM is not constructively valid by exhibiting specific counterexample models.

**Section 4: The Double-Negation Translation.** Classical logic embeds *into* intuitionistic logic via the Gödel-Gentzen translation. Every classical theorem translates to an intuitionistic theorem. This shows that classical and intuitionistic mathematics are not as different as they appear — classical mathematics is "inside" intuitionistic mathematics, just translated.

**Section 5: Constructive Mathematics.** The broader constructive tradition: Bishop's constructivism, Markov's principle, and the relationship to MLTT. What does it mean to do analysis, algebra, and topology constructively?

**Section 6: Decidability.** The constructive notion of decidability — when $P \vee \neg P$ is actually provable for a specific $P$ — and its connection to h-levels and Hedberg's theorem in HoTT.

## What You Will Learn

After this chapter, you will understand why classical logic is not the default logic of type theory, what the constructive content of a proof is, and how the BHK interpretation is the informal version of the Curry-Howard correspondence. You will understand the Kripke semantics well enough to construct countermodels to classical principles, and you will be able to identify which parts of classical mathematics survive constructively and which require additional principles.

This understanding is essential for everything that follows.

# Chapter 5: Intuitionistic Logic and Constructive Mathematics

## The Question of Foundations

Classical mathematics operates with a simple ontology: every mathematical statement is either true or false, whether or not we know which. The sun either has an even number of electrons right now or it doesn't; there either exist infinitely many twin primes or there don't. The universe is deterministic in this sense, and logic is just the tool for discovering what's already determined.

Intuitionistic mathematics challenges this assumption at its root. The intuitionistic view, associated historically with L.E.J. Brouwer, holds that mathematical truth is not a pre-existing fact to be discovered — it is something *constructed* by mathematicians through explicit procedures. A mathematical object exists only when we can construct it; a proposition is true only when we have a proof of it.

This isn't a minor philosophical adjustment. It has profound consequences for what theorems can be stated and what proofs are acceptable.

## Why This Matters for This Curriculum

You might ask: why would a curriculum about Homotopy Type Theory spend time on intuitionistic logic? Isn't this an old philosophical debate, largely irrelevant to modern mathematics?

The answer is: HoTT is *built on* constructive/intuitionistic foundations. Homotopy type theory is a version of Martin-Löf Type Theory (MLTT), which is an explicitly constructive system. The univalence axiom, higher inductive types, and the homotopy interpretation of types all presuppose a constructive foundation. Without understanding constructive logic, HoTT cannot be properly understood.

More concretely:
- In HoTT, a proof of $\exists x : A, P(x)$ genuinely contains a witness $x$ and a proof of $P(x)$. The witness is computational data, not just an abstract existence.
- In HoTT, the logical connectives ($\wedge$, $\vee$, $\to$, $\neg$, $\forall$, $\exists$) have type-theoretic interpretations (products, coproducts, functions, etc.) that only make sense constructively.
- The Law of Excluded Middle and the Axiom of Choice are *not* assumed in HoTT; they can be added but have non-trivial effects on the homotopical structure.

## The Constructive Demand

The central demand of constructive mathematics can be stated simply: **a proof must be a construction**.

What does this mean in practice?
- A proof of $P \wedge Q$ must provide a proof of $P$ and a proof of $Q$.
- A proof of $P \vee Q$ must indicate which disjunct holds and provide a proof of that disjunct.
- A proof of $P \to Q$ must provide a procedure that transforms any proof of $P$ into a proof of $Q$.
- A proof of $\exists x, P(x)$ must exhibit a specific $x$ and provide a proof of $P(x)$.

These demands are natural and reasonable. The controversial case is negation:
- A proof of $\neg P$ (i.e., $P \to \bot$) must provide a procedure showing that any proof of $P$ leads to a contradiction.

Under these demands, the Law of Excluded Middle ($P \vee \neg P$) is not automatically valid: to prove $P \vee \neg P$ constructively, you would have to say whether $P$ holds (and prove it) or whether $P$ fails (and derive a contradiction from any proof of $P$). For many propositions $P$, we simply don't know which case holds.

## The Computational Payoff

The reward for constructive proofs is that they carry computational content. A constructive existence proof doesn't just tell you something exists — it tells you how to *find* it or *compute* it.

This is the core of the Curry-Howard correspondence (Chapter 6): constructive proofs are programs. A proof of $\forall n : \mathbb{N}, \exists m : \mathbb{N}, m > n$ is a program that, given any natural number $n$, computes a larger natural number $m$.

In HoTT, this extends to a richer computational structure: proofs of equalities are paths, homotopies between paths are higher-dimensional proofs, and the whole tower of identity types has a computational interpretation.

## The Roadmap

This chapter develops the theory of intuitionistic logic systematically.

**Section 1: The BHK Interpretation.** The Brouwer-Heyting-Kolmogorov interpretation gives an informal account of what it means to have a constructive proof of each kind of proposition. This is the foundation of the Curry-Howard correspondence.

**Section 2: Formal Systems.** We formalize intuitionistic propositional logic (IPC), identify its key properties (disjunction property, existence property), and contrast it with classical logic.

**Section 3: Kripke Semantics.** Intuitionistic logic has a natural semantics in terms of Kripke frames (partial orders of "possible worlds" representing stages of knowledge). We prove soundness and completeness.

**Section 4: The Double-Negation Translation.** Every classical theorem can be "translated" into an intuitionistic theorem via the Gödel-Gentzen translation. This shows classical and intuitionistic mathematics are more closely related than they appear.

**Section 5: Schools of Constructive Mathematics.** Different schools (Bishop, Brouwer, Markov) make different choices about which extra principles to accept beyond pure intuitionistic logic.

**Section 6: Decidability.** We examine the constructive notion of decidability and connect it to h-levels in HoTT.

Throughout, we emphasize the practical question: for a working mathematician or type theorist, when do classical arguments fail constructively, and why does it matter?

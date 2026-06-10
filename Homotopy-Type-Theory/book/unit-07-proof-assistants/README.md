# Unit 07: Proof Assistants — The Laboratory of the 21st-Century Mathematician

There is a moment in every mathematician's career when the gap between what you believe and what you can prove becomes uncomfortable. You have an argument. You have checked it. Your colleagues have checked it. It has appeared in a journal with referees. And still, quietly, you wonder: did we miss something? Was there a case we overlooked, a subtelty we glossed past, an implicit assumption we never stated?

The proof assistant is the answer to that discomfort. Not because it eliminates human creativity — it doesn't, and no one seriously claims it does — but because it makes the gap visible. When you formalize a mathematical argument in Lean 4 or Cubical Agda, you discover exactly where the informal proof is doing invisible work. You find the lemmas you didn't know you were using. You find the cases you didn't know you were skipping. You find — sometimes — errors that no human reviewer caught.

This unit teaches you to use two state-of-the-art proof assistants: Lean 4 with its library Mathlib, and Cubical Agda. These are not interchangeable tools. They address different needs, rest on different foundational choices, and excel in different domains. Understanding both is essential for anyone who wants to formalize mathematics at the level of HoTT.

## The Two Tools

**Lean 4 and Mathlib** (Chapter 21) is the tool for classical mathematics at scale. Mathlib is the largest formalized mathematics library in existence — over 150,000 theorems covering number theory, algebra, analysis, topology, and category theory. If you want to formalize the algebraic or topological background behind HoTT, Lean 4 is where you go. Its tactic system is sophisticated, its automation is powerful, and its community is large and active. The trade-off: Lean 4's foundations are the Calculus of Inductive Constructions with propositional extensionality, which forces all propositions to behave like sets. Higher inductive types, in the full HoTT sense, are not available.

**Cubical Agda** (Chapter 22) is the tool for HoTT-specific mathematics. It implements cubical type theory, where the univalence axiom is not an axiom but a theorem provable from the structure of the interval, and where higher inductive types are first-class citizens with genuine computation rules. In Cubical Agda, the circle $S^1$ is a data type. Transport along a path of types computes. The proof that $\pi_1(S^1) = \mathbb{Z}$ is not just a formal statement but a running program that computes winding numbers. The trade-off: the library is smaller, the automation less powerful, and the learning curve steeper.

Together, these two tools cover the entire mathematical terrain of this curriculum:

| Topic | Tool |
|-------|------|
| Group theory, ring theory, field theory | Lean 4 / Mathlib |
| Point-set topology, topological spaces | Lean 4 / Mathlib |
| Category theory, functors, adjunctions | Lean 4 / Mathlib |
| Homological algebra, chain complexes | Lean 4 / Mathlib |
| Identity types, path induction | Cubical Agda |
| Univalence as a theorem (computable) | Cubical Agda |
| Higher inductive types: $S^1$, $S^n$, pushouts | Cubical Agda |
| $\pi_1(S^1) = \mathbb{Z}$ (synthetic, computable) | Cubical Agda |
| Higher homotopy groups $\pi_n(S^n)$ | Cubical Agda |
| Brunerie's theorem $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ | Cubical Agda |

## What "Proof Assistant" Actually Means

A proof assistant is a program that checks mathematical proofs. The checking is done by a small kernel — a few hundred lines of code in most systems — that implements the inference rules of a type theory. When Lean 4 accepts your proof, it means the kernel has verified that every step follows from the rules of the Calculus of Inductive Constructions. There is no appealing to mathematical intuition, no handwaving about "it's clear that," no trusting the referee.

The kernel's simplicity is what makes the verification trustworthy. You don't have to trust the entire proof assistant — just the kernel. The tactics, the elaborator, the automation, the library: these are conveniences that generate proof terms for the kernel to check. The kernel is the final arbiter.

This makes proof assistants fundamentally different from computer algebra systems (which compute but don't verify) and from automated theorem provers (which find proofs for limited fragments of logic). Proof assistants are general-purpose verification systems for arbitrary mathematical arguments, with the human mathematician still in the loop at every step.

## Why This Matters for HoTT

Homotopy type theory is a new foundation for mathematics. Its basic objects — types, terms, identity types, higher inductive types — are defined by rules, and the rules have consequences that are not always obvious. The Brunerie number: the computation that $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$, presented in Brunerie's 2016 thesis, defined an integer $n$ via abstract HoTT constructions and conjectured that $n = 2$. Verifying this conjecture by hand would require an enormous amount of work. In Cubical Agda, after years of optimization work by Ljungstrom and Mortberg, the number literally computes to 2.

This is what proof assistants give HoTT: not just a way to check that the rules are consistent, but a way to extract computational content from abstract proofs. The abstract becomes concrete. The theoretical becomes runnable.

## How to Use This Unit

Each chapter stands alone — you can start with Lean 4 or Cubical Agda independently. But if you're new to proof assistants, start with Lean 4. Its tactic system is more beginner-friendly, its documentation is more extensive, and its community is larger. Once you've built intuition for interactive theorem proving, Cubical Agda's more austere style will feel natural rather than alien.

For HoTT-specific content, both chapters are necessary. Lean 4 for the classical background (group theory, topology, category theory). Cubical Agda for the HoTT-specific content (HITs, univalence, homotopy groups). The two tools are not competitors but complements.

The exercises in each chapter are designed to be done at a computer. Install Lean 4 (via elan) and Agda (via cabal or nix) before you begin. The investment in setup time is paid back immediately: there is no better way to learn a proof assistant than to use it.

---

*The proof assistant is the laboratory of the 21st-century mathematician. In it, you do not merely describe an experiment; you run it.*

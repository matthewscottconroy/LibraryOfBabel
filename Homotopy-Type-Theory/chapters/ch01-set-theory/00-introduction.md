# Chapter 1: Set Theory — The Classical Foundation and Its Discontents

## The Official Foundation

For most of the twentieth century, set theory — specifically Zermelo-Fraenkel set theory with the Axiom of Choice (ZFC) — served as the *official* foundation of mathematics. Not that most mathematicians thought about this explicitly in their daily work, but when pressed, the answer was: "everything is sets, and ZFC is the rulebook."

The claim is ambitious. Every mathematical object — the integers, the real numbers, functions, groups, topological spaces, and more — can be encoded as a set. Every mathematical theorem can be expressed as a statement about sets. Every proof can be formalized as a sequence of steps justified by the ZFC axioms. In this sense, ZFC provides a *universal language* for mathematics.

This chapter has two purposes.

**First:** Understand this foundation. What are the ZFC axioms? What do they allow, and what do they forbid? How do you build ordinary mathematical objects (numbers, functions, ordinals) from sets? This is genuine and important knowledge — the mathematics in later chapters (group theory, topology, category theory) is formally rooted here.

**Second:** Understand why set theory, despite its success, is not entirely satisfying, and why type theory offers something genuinely different. The problems are not merely aesthetic. They concern:
- **The nature of identity**: when are two mathematical objects "the same"?
- **Computational content**: do proofs carry algorithmic information?
- **Type safety**: can the foundation prevent category errors?

These questions motivate the transition from set theory to type theory that is the heart of this curriculum.

## A Philosophical Note

The question "what is mathematics ultimately about?" has several answers:
- *Formalism*: mathematics is symbol manipulation; axioms are the rules, and there's no deeper meaning.
- *Platonism*: mathematical objects (numbers, sets) exist independently of our minds; mathematics *discovers* truths about them.
- *Constructivism*: mathematics is about mental constructions; an object exists only when we can construct it.

Set theory as usually practiced is formally neutral on these questions, but its style tends toward the Platonic: the axioms assert the existence of sets without any requirement that we construct them. The Axiom of Choice, in particular, asserts the existence of functions that may have no algorithmic description.

Type theory (especially HoTT) is more constructive in spirit. This is not just philosophy — it has practical consequences for what can and cannot be proven, and for whether proofs can be extracted as programs.

Keep these perspectives in mind as we survey ZFC. The goal is not just to learn the axioms but to develop a critical eye: what does each axiom do, what would fail without it, and what kind of existence does it assert?

## Roadmap

- **Section 1** explains why naive set theory fails: Russell's paradox and the other contradictions that forced a rethinking of mathematical foundations.
- **Section 2** presents the ZFC axioms one by one, explaining the motivation and consequences of each.
- **Section 3** shows how to build ordinary mathematics — natural numbers, ordered pairs, functions, ordinals — from pure sets.
- **Section 4** goes deeper into the Axiom of Choice: its equivalent forms, its independence from ZF, and its relationship to constructive mathematics.
- **Section 5** articulates the limitations of set theory as a foundation and previews the type-theoretic alternative.

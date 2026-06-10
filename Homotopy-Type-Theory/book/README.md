# Homotopy Type Theory: A Complete Curriculum

There is a theorem that most mathematicians have never heard of, proved in 2013 by a team that included a logician, a topologist, and several computer scientists. The theorem states that the fundamental group of the circle is the integers. This is not a new result — topologists have known it for a century. What is new is that the proof was carried out entirely inside a type theory, where the circle is defined by two constructors and a handful of rules, and the integers emerge from the structure of paths around it. No coordinates. No epsilon-delta arguments. No covering space machinery. Just types, terms, and paths.

This is Homotopy Type Theory. And this book is how you get there.

---

## The Nine Units

The curriculum is organized into nine units, each building strictly on the last. There are no shortcuts: the topology comes before the type theory, the algebra before the topology, the logic before the algebra. Each unit is designed to be complete in itself — you could stop after any unit and have a coherent body of knowledge — but the full journey from propositional logic to research frontiers rewards those who make it.

| Unit | Chapters | Theme |
|------|----------|-------|
| [Unit 01](unit-01-mathematical-foundations/) | Ch 00–03 | Mathematical Foundations: logic, sets, algebra, analysis |
| [Unit 02](unit-02-logic-and-computation/) | Ch 04–07 | Logic and Computation: proof theory, intuitionistic logic, Curry-Howard, System F |
| [Unit 03](unit-03-dependent-types/) | Ch 08–09 | Dependent Types: Π and Σ types, Martin-Löf Type Theory |
| [Unit 04](unit-04-category-theory/) | Ch 10–12 | Category Theory: functors, Yoneda, adjunctions, higher categories |
| [Unit 05](unit-05-topology/) | Ch 13–15 | Topology: point-set, homotopy theory, simplicial sets |
| [Unit 06](unit-06-core-hott/) | Ch 16–20 | Core HoTT: identity types, h-levels, univalence, HITs, synthetic homotopy |
| [Unit 07](unit-07-proof-assistants/) | Ch 21–22 | Proof Assistants: Lean 4 / Mathlib, Cubical Agda |
| [Unit 08](unit-08-advanced-foundations/) | Ch 23–25 | Advanced Foundations: cubical TT, simplicial TT, modal HoTT |
| [Unit 09](unit-09-research-frontiers/) | Ch 26 | Research Frontiers: open problems, how to contribute |

---

## How Each Chapter Is Organized

Every chapter follows the same structure:

```
chXX-name/
  README.md                          ← chapter hook and roadmap
  01-topic/topic.md                  ← content sections (~1200 words each)
  02-topic/topic.md
  ...
  90-important-thinkers/             ← who built this mathematics and why it matters
  91-references/                     ← primary sources, annotated
  92-thought-experiments/            ← questions that test genuine understanding
  93-exercises/                      ← 25–35 problems, routine through proof-level
  94-applications/                   ← where these ideas do real work in the world
```

The chapter hooks are written to pull you in. The thought experiments are written to unsettle you. The exercises are written to make you do mathematics, not just read it. The applications sections are specific: they name the technology, the field, the problem, and show exactly how the mathematics inside the chapter makes a difference.

---

## Prerequisites

The curriculum is self-contained. Unit 01 assumes only mathematical maturity: comfort writing proofs, familiarity with functions and sets, some experience with algebra. Everything else is developed from first principles.

For the proof assistant chapters (Units 07 and beyond): install Lean 4 via [elan](https://github.com/leanprover/elan) and Agda via `cabal install Agda`. The chapters include working code.

---

## The Larger Argument

This book rests on a philosophical claim: that mathematics, logic, and computation are three aspects of one subject. Propositions are types. Proofs are programs. The laws of logic are the laws of type construction. This identification — the Curry-Howard correspondence — runs through the whole curriculum, from its first appearance in Chapter 06 to its mature form in the univalence axiom of Chapter 18 and the cubical interval of Chapter 23.

The claim has consequences. If proofs are programs, then type-checking is proof-checking, and the computer can verify that your proof is correct. If identity types are path spaces, then every theorem about equality has a geometric interpretation. If the universe is itself a type — a space of all spaces — then statements about mathematical structures become statements about paths in that space.

These are not analogies. They are identities. And following them to their conclusion is what this curriculum is about.

---

## Community and Further Reading

| Resource | Location |
|----------|----------|
| HoTT Book (free) | homotopytypetheory.org/book |
| HoTT Zulip (primary community) | hott.zulipchat.com |
| Cubical Agda library | github.com/agda/cubical |
| Mathlib4 | github.com/leanprover-community/mathlib4 |
| Rzk (Simplicial TT) | rzk-lang.github.io |
| nLab (reference) | ncatlab.org |

# Part I — The Language of Mathematics

## Chapters 1–3: Logic, Sets, Relations, Functions, and Cardinality

* * *

Mathematics is often described as the language of science — but that description understates the ambition. Science uses mathematics to describe regularities in the world; mathematics, at its foundation, is something more austere and more radical: a discipline of reasoning in which every claim must be either proved or acknowledged as an unproven assumption. Within this discipline, and only within it, truth is not a matter of evidence or plausibility but of logical necessity. This standard — the proof — is what makes mathematics uniquely reliable. But it is not free. It demands a language precise enough that every statement has exactly one meaning, and a method rigorous enough that every inference survives the most hostile scrutiny. Part I builds that language and that method from the ground up.

The reader approaching abstract algebra for the first time may be surprised that the book begins not with groups or rings but with propositional connectives and quantifiers. The reason is that without a precise logical foundation, every algebraic argument rests on unstated intuitions — and in algebra, intuitions learned from the integers or the real numbers routinely fail. Cantor discovered that infinite sets come in multiple, incomparable sizes, overturning the intuition that "infinite" is a single notion. Russell showed that the naive idea of "the set of all sets satisfying a property" leads to an outright contradiction. Gödel proved that in any sufficiently powerful formal system, there are true statements that cannot be proved within the system — a theorem about mathematics, proved by the very methods of mathematics itself. These are not historical curiosities; they are precise theorems that changed what mathematics means, and they are fully comprehensible only to a reader already fluent in the language Part I teaches.

Chapter 1 builds the logical vocabulary: propositions and their connectives, quantifiers and their negations, and the four dominant proof strategies — direct proof, proof by contrapositive, proof by contradiction, and mathematical induction — that together cover essentially every argument encountered in this book. The chapter closes with the axiomatic method, the organizing philosophy that makes abstract algebra possible: rather than reasoning about specific objects, we select a small set of axioms and derive everything from them by logic alone, gaining the remarkable power to prove theorems that apply simultaneously to the integers, to symmetry groups of geometric figures, and to function spaces. Chapter 2 establishes the basic ontology in which all algebraic structures live: sets and membership, the operations on sets, relations between elements, equivalence relations and the quotient-set construction (the abstract mechanism of "treating two things as the same for our purposes"), and functions together with the precise classification into injections, surjections, and bijections that will recur constantly as the distinction between maps that preserve distinct elements, maps that cover their codomain, and maps that can be reversed. Chapter 3 confronts infinity directly: Cantor's diagonal argument establishing the uncountability of the real numbers and the existence of multiple infinite sizes, the Schröder–Bernstein theorem providing a clean tool for comparing infinite cardinalities, and — at the end — Zorn's lemma, the hidden engine of algebra, which guarantees the existence of bases for vector spaces, maximal ideals in rings, and algebraic closures of fields without ever constructing any of them explicitly.

There is no shortcut through Part I. Every algebraic structure studied in Parts II–XI lives inside a set. Every homomorphism is a function. Every quotient group, quotient ring, and quotient module is a quotient set with additional structure. Every existence proof — of a basis, of a maximal ideal, of an algebraic closure, of an irreducible representation — passes through Zorn's lemma or through a piece of set-theoretic reasoning developed here. The precision built in Part I is precisely the precision the rest of the book requires.

* * *

### Internal Dependency Map

```
Chapter 1 (Logic and Proof)
    │
    ▼
Chapter 2 (Sets, Relations, Functions)
    │
    └──► Chapter 3 (Cardinality and Choice)
```

Everything in Parts II–XI depends on Part I.

* * *

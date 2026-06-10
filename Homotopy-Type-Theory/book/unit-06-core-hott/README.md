# Unit 06: Core Homotopy Type Theory

## This Is What the Whole Book Has Been Building Toward

Every chapter before this one was preparation. You learned propositional logic so you could see what proof means. You learned set theory so you could see its limits. You learned type theory — Martin-Lof's type theory, dependent types, the Curry-Howard correspondence — so you could see equality as a type. You learned category theory and homotopy theory so you could recognize the shapes when they arrived. You learned simplicial sets so you could believe the model.

Now the model is here. Now we build the real thing.

Homotopy Type Theory is not a single theorem. It is a shift in perspective so radical that, once you have made it, mathematics looks different. Not different in what it says — the theorems are the same theorems — but different in what they *mean*. The natural numbers are what they always were. The circle is what it always was. But now we know why equality is more than a relation, why a proof of A=B is a path from A to B, why paths can have homotopies between them and homotopies between those, forever upward into a tower that was always there, waiting.

Here is the structure of what we are about to discover.

---

## Chapter 16: Identity Types and Paths

The central move of HoTT is this: equality is a space. In classical mathematics, the statement "a equals b" is either true or false. There is one proof of equality or there is none. In HoTT, the identity type `a =_A b` is a genuine type — it can have many elements, each one a distinct *reason* why a equals b, a distinct *path* from a to b through the space A.

This is not a complication. It is the discovery that classical equality was impoverished all along. When we say two proofs of the same theorem are "the same proof," we are ignoring path structure. When we say two implementations of the same function are "equivalent," we are ignoring 2-paths. The structure was always there. We just didn't have the language.

Chapter 16 develops the full machinery: path induction (J), the groupoid laws, higher paths, transport, and the functorial action of functions on paths.

---

## Chapter 17: H-Levels

Not all types are equally complicated. Some types have exactly one element; the question of equality doesn't arise. Some types have multiple elements but exactly one proof of equality between any two; these are the propositions. Some have elements, paths, but only one path between any two paths; these are the sets of classical mathematics. And some — like the circle, like the universe itself — have nontrivial paths between paths between paths, with no end.

The h-level hierarchy classifies this complexity. H-level minus-two: contractible. H-level minus-one: propositions. H-level zero: sets. H-level one: groupoids. And so on.

This hierarchy is not bureaucracy. It is the precise answer to the question: when does proof matter? Propositions are where proof does not matter — all proofs are equal. Sets are where path-structure does not matter — all paths are equal. The h-level tells you what level of structure you must track and what level you may ignore.

---

## Chapter 18: The Univalence Axiom

In 2006, Vladimir Voevodsky wrote down an axiom that changed the foundations of mathematics. The Univalence Axiom says: if two types are equivalent, they are equal. Not merely isomorphic. Equal.

This sounds wrong. Two groups can be isomorphic without being identical — Z/2Z and {0,1} with mod-2 addition are isomorphic but surely not the same thing. Voevodsky's axiom says they *are* the same thing, when equality means what it means in HoTT: a path. And the path from one presentation to the other carries, as its computational content, the isomorphism itself.

The Univalence Axiom is the formal expression of a principle mathematicians have known, informally, for over a century: *isomorphic structures are interchangeable*. In HoTT, this is not a principle. It is a theorem. Or rather, an axiom from which it becomes a theorem that nothing provable in classical mathematics can distinguish isomorphic structures. The formal system is finally as good as the informal practice.

---

## Chapter 19: Higher Inductive Types

Ordinary inductive types build things from constructors. You build the natural numbers from zero and successor. You build trees from leaves and branches. These types are discrete — there are no paths between distinct elements except the paths you explicitly prove.

Higher inductive types let you build paths in along with the points. The circle S^1 has one point (base) and one path from base to itself (loop). That is the complete definition. From this two-line definition, the theory proves that the fundamental group of S^1 is Z. Not by importing classical topology. By computing, inside the type theory, with the paths you declared.

This is a new kind of definition. You are not just saying what things *are* — you are saying what things *equal*. And those equality declarations, those path constructors, give the resulting type all the homotopy-theoretic structure you intended.

---

## Chapter 20: Synthetic Homotopy Theory

The final chapter of this unit is a demonstration: here is what all of this machinery can do. We prove, inside the type theory, the fundamental group of the circle (five pages). We prove the Seifert-van Kampen theorem from the universal property of pushouts. We prove the Freudenthal Suspension Theorem. We construct the Hopf fibration and use it to compute pi_3(S^2) = Z.

These are classical theorems of algebraic topology, and their classical proofs are beautiful. But the synthetic proofs are different. They are not translations. They are new proofs that work because, in HoTT, the language and the subject matter have finally aligned. When the identity type *is* the path space, when the HIT *is* the CW complex, when transport *is* parallel transport — the proofs stop fighting the notation and become what they always were in the topologist's intuition.

---

## How to Read This Unit

These five chapters are not independent. They form a single sustained argument. Chapter 16 establishes the language. Chapter 17 classifies its objects. Chapter 18 gives the key axiom. Chapter 19 gives the key construction method. Chapter 20 shows what you can prove.

We recommend reading them in order, slowly. The ideas compound. A theorem proved in Chapter 16 will be used silently in Chapter 18. A construction from Chapter 17 will appear without comment in Chapter 20. This is how mathematics is: not a sequence of isolated results, but a growing structure where each thing you learn changes what you already knew.

The heart of the book is here. Let's begin.

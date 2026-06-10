# Chapter 1: Set Theory

## Cantor's Paradise and the Snake in It

In 1874, Georg Cantor proved something that should have been impossible. He showed that some infinite sets are bigger than others. Not just "contain more things" in some informal sense — genuinely, provably, irremediably larger, in a sense that can be made completely precise. The integers are infinite. The real numbers are infinite. And the reals are more infinite than the integers. You cannot set up a correspondence that matches each real number to exactly one integer and exhausts both collections.

The mathematical community was not immediately pleased. Kronecker called Cantor's work "a mathematical disease." Poincaré called it "a perverse pathological fantasy." Cantor himself descended into depression and spent significant periods in sanatoriums. But he was right, and by the end of his life Hilbert had given the definitive verdict: "No one shall expel us from the paradise that Cantor has created."

Then, in 1902, Russell sent Frege a letter.

Frege had just published the second volume of his *Grundgesetze der Arithmetik* — a massive formal derivation of arithmetic from logical principles. The cornerstone was Basic Law V: for any property φ, the extension {x | φ(x)} is a set. This seems entirely reasonable. If you have a property, you should be able to collect all the things with that property into a set.

Russell's letter began: "Dear Colleague, I find myself in agreement with you in all essentials... I have encountered a difficulty." The difficulty was this. Let R = {x | x ∉ x}. Is R ∈ R? If R ∈ R, then by definition of R, R ∉ R. If R ∉ R, then R satisfies the defining property, so R ∈ R. Either way, contradiction.

Frege's response was among the most honest sentences ever written by a mathematician: "Arithmetic totters." He added a hasty appendix to the volume, acknowledging the contradiction and noting that no repair was immediately available. He was right — no repair of his specific approach was ever found.

This is the world of Chapter 1. Cantor's paradise, which we will inhabit and explore, has a snake in it: unrestricted set comprehension is inconsistent. The paradise can be saved — Zermelo showed how, in 1908 — but only by disciplined restriction. We give up the most naive principle (form a set from any property) and replace it with a careful list of axioms that sanction only the constructions we actually need.

## What We Gain and What We Sacrifice

The Zermelo-Fraenkel axioms (ZFC, with Fraenkel's Replacement and the Axiom of Choice) are the standard foundation of twentieth-century mathematics. From these axioms, you can build the integers, the real numbers, Euclidean spaces, topological spaces, and groups. You can prove Cantor's theorem, the well-ordering theorem, Tychonoff's theorem. The edifice is enormous and solid.

What we sacrifice is a certain naturalness. In ZFC, every object is a set. The number 3 is a set (specifically, {∅, {∅}, {∅, {∅}}}). The ordered pair (a, b) is a set (Kuratowski's encoding: {{a}, {a, b}}). A function is a set of ordered pairs. This is technically workable — everything can be encoded — but it introduces what Benacerraf called the "identification problem": which set *is* 3? There are multiple valid encodings. The number 3 could be {∅, {∅}, {∅, {∅}}} (von Neumann) or {{∅}} (Zermelo). Both satisfy the right axioms. The choice is arbitrary.

This arbitrariness is not a small problem. It means that mathematical structures in ZFC are *implemented* rather than *specified*. The natural numbers are not *defined* by their properties — they are *constructed* by a specific encoding, and a different encoding would work just as well. Two groups can be isomorphic — structurally identical — and yet be different sets, because they happen to have different underlying sets.

Mathematicians handle this by ignoring it: we treat isomorphic structures as "the same," work up to isomorphism, and never ask which specific set is the number 3. But this informal convention is in tension with the formal foundation. The Univalence Axiom of HoTT resolves the tension by making the convention into a theorem: isomorphic structures *are* the same, in the sense that they are connected by a path in the type universe.

## The Chapter Plan

We move through five topics.

We begin with *naive set theory* — Cantor's original theory, unrestricted comprehension, and the crisis that Russell provoked. The paradox is not just a puzzle; it reveals a deep structural problem with impredicative definition.

Then the *ZFC axioms*, one by one. Each axiom has a motivation: it permits a specific construction we need, while blocking the pathological self-reference that causes paradoxes. The Axiom of Foundation says every set is well-founded — no infinite descending membership chains. The Axiom of Replacement says images of sets under definable functions are sets. These are not arbitrary restrictions; they are the minimal constraints that make mathematics work.

Then *ordinals and cardinals* — the mathematics of infinite quantity. Von Neumann ordinals give a canonical way to measure "how far along" a process has gone in the transfinite. Cantor's theorem shows there is no largest cardinality; the power set operation always produces something strictly larger. Transfinite induction extends mathematical induction into the infinite.

Then the *Axiom of Choice* — the most controversial of the ZFC axioms. Choice says: given any family of non-empty sets, you can simultaneously choose one element from each. This seems obvious (just choose!), but for infinite families there may be no rule for choosing — no algorithm, no preference, no pattern. Choice asserts the existence of choices that may have no description. Its equivalents include the well-ordering theorem (every set can be well-ordered) and Zorn's lemma (every chain-bounded partially ordered set has a maximal element). Its applications include the existence of Hamel bases for real-vector-spaces, the Hahn-Banach theorem, and the Tychonoff theorem.

Finally, *the limits of set theory*. What ZFC gets right: universality, relative consistency, a clear picture of the cumulative hierarchy. What ZFC gets wrong: the identity problem, the lack of computational content, the mismatch between formal foundation and mathematical practice. And why these failures motivate the type-theoretic alternative.

By the end, you will understand what set theory achieves — and why the next century of foundational work, leading through Gödel, Cohen, and Voevodsky, has been working to transcend it.

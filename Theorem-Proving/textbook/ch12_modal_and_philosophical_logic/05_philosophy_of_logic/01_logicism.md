# Logicism

> "My whole work is founded on the conviction that arithmetic is a branch of logic — that arithmetic requires no assumptions not required by pure logic."
> — Gottlob Frege, *Grundlagen der Arithmetik*, 1884

## The Ambition

At the end of the nineteenth century, Gottlob Frege conceived one of the most audacious projects in intellectual history: to show that all of mathematics — at least all of arithmetic — is nothing more than **pure logic**. Not applied logic, not logic with mathematical axioms added on top, but *pure logic itself*, from which mathematical truths could be derived by logical inference alone.

This project is called **logicism**: the thesis that mathematics is reducible to, or identical with, logic.

If successful, logicism would have profound consequences:
- Mathematical knowledge would be a species of *logical* knowledge — analytic, necessarily true, known through reason alone
- Mathematical truth would not depend on any special "mathematical intuition" or perception of abstract objects
- The apparent necessity of mathematical truths (that $2 + 2 = 4$ could not have been otherwise) would be explained by the necessity of logic

## Frege's Program

Frege's approach was systematic and technically brilliant. He invented **modern predicate logic** specifically for the purpose of carrying out his logicist program — a tool we still use today.

His key move: define the natural numbers *logically* using the concept of **equinumerosity**. Two concepts $F$ and $G$ are equinumerous if there is a one-to-one correspondence between the objects falling under $F$ and those falling under $G$.

**Hume's Principle**: The number of $F$s equals the number of $G$s if and only if the $F$s and $G$s are equinumerous.

$$\#F = \#G \iff F \sim G$$

From Hume's Principle alone — which Frege regarded as a logical truth — he hoped to derive all of arithmetic. **Frege's Theorem** (proved much later, by Crispin Wright in 1983): the basic laws of second-order arithmetic *do* follow from Hume's Principle in second-order logic.

The problem: Frege needed to *define* what $\#F$ is. His definition used **extensions of concepts** — the set of all objects falling under a concept. This required his notorious **Basic Law V**: the extension of concept $F$ equals the extension of concept $G$ iff $F$ and $G$ apply to exactly the same objects.

And Basic Law V is inconsistent — Russell's paradox shows this directly (the extension of "is not a member of itself" leads to contradiction).

## Russell and Whitehead: *Principia Mathematica*

Bertrand Russell, having discovered his paradox, set out with Alfred North Whitehead to repair the logicist program. Their weapon: **type theory** — a hierarchical stratification of mathematical objects that prevents the paradoxical self-reference.

The *Principia Mathematica* (1910-1913), three enormous volumes, attempted to derive mathematics from logical principles in the type-theoretic framework. The result was technically impressive and philosophically problematic:

**The axiom of reducibility**: Type theory requires that every proposition has an equivalent "predicative" version at a lower level of the type hierarchy. This axiom is needed to do mathematics, but it is not logically necessary — it is a substantive mathematical assumption.

**The axiom of infinity**: There are infinitely many individuals (objects at the lowest type level). Again, not a logical truth.

**The axiom of choice**: Needed for many mathematical results; not a logical principle.

Russell himself admitted that these axioms made the *Principia* more like "mathematical logic" than "logicism" strictly construed. The reduction to pure logic was incomplete.

## Neo-Logicism: Reviving the Program

In the 1980s, Crispin Wright and Bob Hale proposed **neo-logicism** — a revival of the Fregean program using **Hume's Principle** rather than Basic Law V.

**Key insight (Wright)**: Hume's Principle — unlike Basic Law V — is *consistent* (it does not lead to Russell's paradox). And from Hume's Principle in second-order logic, all of arithmetic can be derived.

The neo-logicist program raises the question: is Hume's Principle a logical truth? If yes, then arithmetic is a branch of logic. If no, then arithmetic might be *analytic* (true in virtue of meaning) but not purely logical.

Critics argue that Hume's Principle is more like a mathematical axiom (a claim about numbers) than a logical truth. The debate continues in contemporary philosophy of mathematics.

**Frege's Theorem revisited**: The derivation of arithmetic from Hume's Principle in second-order logic goes through without inconsistency. This is a significant technical achievement, even if it does not vindicate full logicism.

## Logicism in Lean and Coq

Modern proof assistants like Lean and Coq give us a precise formal setting in which to evaluate logicist claims. In both systems, mathematics is built from a small set of foundational principles (the type theory rules) that can be viewed as "logical" in a broad sense.

**In Lean 4**: Natural numbers are defined inductively:
```lean
inductive Nat : Type where
  | zero : Nat
  | succ : Nat → Nat
```

This is a *definition*, not a logical derivation. The natural numbers are primitive in Lean — they are not *reduced* to anything more basic. However, the definition is given in a purely formal language (the Calculus of Inductive Constructions), and all arithmetic facts are proved from it by formal reasoning.

Is this logicism? It depends on what you count as "logic." If CIC (the logical framework of Lean/Coq) counts as logic, then mathematics in Lean is indeed reducible to logic. But CIC includes inductive type definitions as a primitive — and many philosophers would say this is already doing mathematics, not pure logic.

**The practical conclusion**: Even if full-blooded logicism fails (mathematics cannot be reduced to *bare* logic), proof assistants show that mathematics can be fully formalized in a precise, machine-checkable framework. This achieves much of what the logicists hoped for — a fully explicit, rigorous foundation — even if the foundation includes mathematical primitives alongside logical ones.

## Philosophical Assessment

Logicism remains one of the great philosophical programs, even in failure:

**What it achieved:**
- Modern predicate logic (Frege's invention for the program)
- Set theory (developed partly in response to the paradoxes that threatened logicism)
- Formal foundations of mathematics (the successor to the logicist program)
- The philosophy of mathematics as a professional discipline

**Where it failed:**
- No convincing way to derive arithmetic from *pure* logic alone, without axioms that are substantive mathematical assumptions
- Russell's paradox exposed the inconsistency of naive logicism
- Gödel's incompleteness theorems showed that even if mathematics could be reduced to a formal system, that system would be incomplete — there would always be mathematical truths not provable within it

**What it taught us:**
- The distinction between *formal proof* and *mathematical truth* is sharp and permanent (Gödel)
- "Logic" and "mathematics" are not as clearly separated as the logicists hoped — they shade into each other
- The question "what is mathematics?" (its nature, its objects, its necessity) is genuinely difficult and has no easy answer

## Exercises
See [problems/ch12_modal_logic/05_philosophy_exercises.md](../../../problems/ch12_modal_logic/05_philosophy_exercises.md)

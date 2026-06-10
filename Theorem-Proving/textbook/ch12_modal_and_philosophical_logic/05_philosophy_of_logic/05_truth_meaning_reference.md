# Truth, Meaning, and Reference

## The Semantic Triangle

Language connects **signs** (words, symbols), **meanings** (concepts, senses, intensions), and **referents** (objects, extensions, what words point to). The relationships among these are the subject of the **philosophy of language** — and formal logic has contributed decisively to clarifying them.

**Frege's distinction**: For the expressions "morning star" and "evening star," both refer to the same object (Venus), but they have different *senses* (different modes of presentation). Reference (Bedeutung) is the object; sense (Sinn) is how the object is presented.

This distinction matters because "The morning star is the morning star" is trivially true, while "The morning star is the evening star" is an empirical discovery — same reference, different sense.

## Tarski's Theory of Truth

Alfred Tarski (1936) gave the first mathematically precise definition of truth for formal languages, resolving the Liar paradox by distinguishing object language from metalanguage.

**The T-schema**: For each sentence $\varphi$ of the object language:
$$T\ulcorner\varphi\urcorner \iff \varphi$$

"'Snow is white' is true if and only if snow is white."

Tarski showed this schema cannot be consistently stated within the same language — truth must be defined at a higher level. His construction: define truth for language $L$ in a richer metalanguage $L'$ that can talk about $L$'s syntax.

## Compositional Semantics

Frege's **Principle of Compositionality**: The meaning of a complex expression is determined by the meanings of its parts and the way they are combined.

This principle is what makes formal semantics possible: we define the meaning of compound formulas recursively from atomic formulas, rather than case-by-case.

In FOL: the truth of $\varphi \wedge \psi$ is determined by the truth of $\varphi$ and $\psi$ separately. The truth of $\forall x\, P(x)$ is determined by whether $P(a)$ holds for every $a$ in the domain.

## Possible Worlds Semantics for Meaning

**Modal semantics** (Kripke) extends truth-conditional semantics to modal statements. The "meaning" of a sentence in modal logic is its **intension** — a function from possible worlds to truth values.

For rigid designators (names like "Aristotle"), the reference is the same in every possible world where the name is used. For definite descriptions ("the teacher of Alexander"), the reference varies: in a world where someone else teaches Alexander, the description picks out a different person.

This distinction (Kripke 1980) has profound implications for philosophy of mind and language: natural kind terms ("water," "gold") are rigid designators — they refer to the actual kind (H₂O, Au) in all possible worlds, not just in ours.

## Connection to Formal Logic

These philosophical distinctions have direct formal counterparts:
- **Reference**: the denotation function in model theory
- **Sense/intension**: functions from possible worlds to extensions (modal semantics)
- **Compositionality**: the recursive definition of satisfaction
- **Truth**: Tarski's definition, formalized in proof assistants as the semantics of the object language

Lean 4 and Coq use type theory, which gives a computational version of these semantic notions: types are meanings, terms are proofs/programs, and the definitional equality judgment determines when expressions have the same "meaning."

## Exercises
See [problems/ch12_modal_logic/05_philosophy_exercises.md](../../../problems/ch12_modal_logic/05_philosophy_exercises.md)

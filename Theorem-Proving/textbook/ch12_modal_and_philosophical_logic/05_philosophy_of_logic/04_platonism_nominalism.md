# Mathematical Platonism and Nominalism

> "God made the integers; all else is the work of man."
> — Leopold Kronecker (1893)

> "The unreasonable effectiveness of mathematics in the natural sciences is something bordering on the mysterious."
> — Eugene Wigner (1960)

## The Central Question

Do mathematical objects — numbers, sets, functions, geometric shapes — *exist*? If so, in what sense? Are they discovered or invented? Are they real in the way physical objects are real, or are they fictions, useful conventions, or mental constructs?

These questions are not merely academic. They bear on the foundations of mathematics, the reliability of mathematical knowledge, and the stunning fact that abstract mathematics developed for its own sake — complex numbers, non-Euclidean geometries, group theory — repeatedly turns out to describe physical reality with uncanny precision.

The two major positions are **Platonism** (mathematical objects exist independently of minds) and **Nominalism** (they do not).

## Mathematical Platonism

**Platonism** (also called **realism** or **mathematical Platonism**) holds that mathematical objects exist in an abstract realm, independent of human thought, language, or convention. Mathematical truths are discovered, not invented. The number $\pi$ had the value it has before humans calculated it, and would still have that value even if no human ever existed.

**Arguments for Platonism**:

**The applicability argument (Wigner's puzzle)**: Mathematics developed purely abstractly — complex analysis invented for elegance, non-Euclidean geometry explored as a curiosity — turns out to describe quantum mechanics and general relativity with extraordinary precision. If mathematics were merely human invention or convention, why would it describe reality so perfectly? The best explanation is that mathematics is tracking objective structure in the world (or in an abstract realm).

**The indispensability argument (Quine-Putnam)**: Scientific theories we believe to be true indispensably quantify over mathematical objects. If we believe the science, we should believe in the existence of the mathematical objects. We are committed to the existence of numbers and sets as surely as we are committed to the existence of electrons.

**The reliability argument**: Mathematicians across cultures, centuries, and without communication reach the same mathematical conclusions. The fact that mathematics is intersubjectively reliable and cumulative suggests that mathematical truths are objective — not subjective like aesthetic preferences.

**Arguments against Platonism**:

**The epistemological problem**: If mathematical objects exist in an abstract, non-physical realm, how do we come to know about them? Our knowledge of the physical world comes through causal interaction — light bouncing off objects reaches our eyes. But abstract objects causally inert — they cannot affect us. How can we have knowledge of them? (Benacerraf's epistemological challenge, 1973)

**The ontological problem**: What exactly are abstract objects? Where do they "live"? The abstract realm sounds metaphysically extravagant. Occam's razor suggests we should not multiply entities unnecessarily.

## Nominalism

**Nominalism** holds that mathematical objects do not exist in the abstract. There are no numbers "out there" — only physical tokens (numerals, marks on paper), minds that use them, and patterns in the physical world.

Several varieties of nominalism:

**Fictionalism** (Hartry Field): Mathematical statements are literally false (there are no numbers), but they are *useful fictions* — like saying "the average American has 2.3 children." We use mathematical language as a convenient shorthand for complex physical facts. Field famously showed that Newtonian mechanics could be reformulated without reference to numbers — though the reformulation is cumbersome.

**Structuralism** (Benacerraf, Shapiro): Mathematical objects do not exist independently, but mathematical *structures* do (or are at least well-defined). What matters about natural numbers is not "what" they are but how they relate to each other: the pattern 0 < 1 < 2 < 3 < ... with successor, addition, and multiplication. Any structure instantiating this pattern is "the natural numbers." The natural numbers are not a specific abstract entity but a structural role.

**Nominalistic Platonism** (some positions): Mathematical objects exist, but as *ideal possibilities* or *possibilities of construction*, not as independently existing entities.

## The Benacerraf Dilemma

Paul Benacerraf (1965) posed a dilemma that has shaped the philosophy of mathematics since:

On one hand, mathematical truth should be treated like truth in general: a sentence "7 is prime" is true just in case the object denoted by "7" has the property denoted by "is prime." This requires mathematical objects to exist (or the sentence is meaninglessly false).

On the other hand, knowledge requires causal connection: we know things about the physical world because physical things causally affect us. Abstract objects, if they exist, cannot causally affect us — so how can we have mathematical knowledge?

Any satisfactory philosophy of mathematics must solve both halves:
- **Semantic requirement**: Account for mathematical truth in the same way we account for empirical truth
- **Epistemic requirement**: Explain how we have mathematical knowledge

Platonism satisfies the semantic requirement but struggles with the epistemic one. Nominalism satisfies the epistemic requirement but struggles with the semantic one.

## Formalism: A Third Way?

**Hilbert's formalism** (see section 02) offered an escape: mathematics is the study of formal symbol manipulation. "The number 7" refers to nothing — it is just a symbol we manipulate according to rules. Mathematical truth is formal derivability, not correspondence to abstract reality.

The advantage: no abstract objects, no epistemological mystery. The disadvantage: Gödel showed that formal derivability falls short of mathematical truth — there are sentences that are "true" (in any standard model) but not formally derivable (incompleteness).

## Implications for Formal Verification

From a practical standpoint, proof assistants are agnostic about Platonism vs. nominalism:
- In **Lean 4**, natural numbers are defined inductively — they are formal constructs within a type theory
- Mathematical theorems are proved as terms of dependent types
- Whether these formal objects "exist" in a Platonic sense is a philosophical question Lean cannot answer

Nevertheless, the philosophy matters for *confidence*:
- A Platonist sees Lean proofs as certifying facts about abstractly existing mathematical objects
- A formalist sees Lean proofs as showing that certain formal manipulations succeed
- A fictionalist might say Lean proofs show that certain mathematical fictions are internally consistent

All three can use and trust Lean proofs. The philosophical disagreement is about what those proofs ultimately mean, not about whether they work.

## Exercises
See [problems/ch12_modal_logic/05_philosophy_exercises.md](../../../problems/ch12_modal_logic/05_philosophy_exercises.md)

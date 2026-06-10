# The Liar's Paradox

Epimenides of Crete said: "All Cretans are liars." If true, Epimenides (a Cretan) is a liar, so the statement is false — contradiction. If false, not all Cretans are liars, so Epimenides might be truthful — but then the statement is true again.

The modern, sharper form:

> **L**: "This sentence is false."

Suppose L is true. Then what L says holds: L is false. So if L is true, L is false. Contradiction.
Suppose L is false. Then L says something false: but L says *it is false*, so it is true. Contradiction.

L is neither true nor false — or both — or something else entirely. This is the *Liar's Paradox*.

## Why It Matters

The Liar is not a curiosity. It reveals a deep tension in how we think about truth:

1. **T-schema**: For any sentence φ, "φ is true" ↔ φ. (Tarski's material adequacy condition)
2. **Self-reference**: Natural languages contain sentences that refer to themselves.

These two together yield contradiction. The Liar shows that no naive theory of truth can be consistent for a language that can refer to its own sentences.

## Formal Derivation

Let L be a sentence such that L ↔ ¬T(⌈L⌉), where T is a truth predicate and ⌈L⌉ names L.

By the T-schema: T(⌈L⌉) ↔ L.
Substituting L ↔ ¬T(⌈L⌉): T(⌈L⌉) ↔ ¬T(⌈L⌉). Contradiction.

Gödel's *diagonal lemma* shows that such self-referential sentences exist in any sufficiently strong formal system (one that can represent its own syntax). The Liar's self-reference is not just a linguistic trick — it's provably constructible in arithmetic.

## Historical Roots

The paradox goes back to antiquity. Eubulides of Miletus (4th century BCE) is credited with the *pseudomenon* (liar). Chrysippus wrote six books on it (all lost). Medieval logicians — the *insolubilia* tradition — wrestled with it for centuries.

Modern treatment began with Bertrand Russell and Alfred Tarski. Russell's type theory (1908) avoided it by stratifying language. Tarski's semantic theory of truth (1933) resolved it via hierarchy. Kripke's partial-models approach (1975) reopened the question with new tools. The debate continues: paraconsistency, revision theory, truth-value gaps — there is no consensus solution.

## The Curry Paradox

A related but distinct paradox:

> **C**: "If this sentence is true, then the Moon is made of cheese."

Suppose C is true. Then: if C is true, the Moon is made of cheese. Since C is true (assumed), the Moon is made of cheese. So C is true implies "the Moon is made of cheese" — but we can prove C is true without any contradiction! So by modus ponens, the Moon is made of cheese.

Curry's paradox shows that the problem is not negation (the Liar uses ¬) but *implication*. Any consistent self-referential use of → leads to absurdity. Paraconsistent logics must also address Curry, which requires restricting not just contradiction-tolerance but also modus ponens or conditional proof.

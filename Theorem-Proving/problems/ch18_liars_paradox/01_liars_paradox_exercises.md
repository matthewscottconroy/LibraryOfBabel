# The Liar's Paradox: Exercises

## The Basic Paradox

1. Formalize the Liar in a language with a truth predicate T:
   a. State Tarski's T-schema as an axiom schema.
   b. Show that from T-schema + self-reference, we can derive ⊥ (False).
   c. Where exactly in the derivation is the T-schema used?

2. Consider the "Truthteller": "This sentence is true." Is the Truthteller paradoxical? What truth values can it consistently have?

3. The *Yablo Paradox* (1993) achieves self-reference without self-reference:
   > Sentence n: "For all k > n, sentence k is false."
   Show that this sequence generates a paradox without any sentence referring to itself.

## Tarski's Solution

4. Implement Tarski's hierarchy for a simple formal language:
   - L₀ = propositional logic with atoms {p, q, r}
   - L₁ = L₀ + truth predicate T₀ for L₀
   - Define T₀ compositionally: T₀(⌈p⌉) ↔ p, etc.
   - Show that "T₀(⌈L₀-sentence⌉)" is always well-defined in L₁.

5. Formulate the "revenge Liar" against Tarski's hierarchy. What resources does Tarski have to respond?

## Kripke's Theory

6. Kripke's *Strong Kleene* scheme has three values: T, F, and U (undefined/gappy).
   - Define truth tables for ∧, ∨, ¬ in Strong Kleene logic.
   - Show that the Liar L gets value U in the minimal fixed point.
   - Formulate a "revenge paradox" for Kripke's theory.

## Paraconsistency

7. In Priest's LP (Logic of Paradox):
   a. Verify that modus ponens is valid: if ⊨ P and ⊨ P → Q, then ⊨ Q.
   b. Verify that explosion fails: ⊭ (P ∧ ¬P) → Q in general.
   c. Show that the Liar has value B (both) in LP.

8. The *Curry paradox* for LP: "If this sentence is true, then Q." Show that LP must also invalidate either modus ponens or contraction (A → (A → B)) ⊢ (A → B) to avoid Curry.

## Philosophical

9. Compare Tarski, Kripke, and Priest/Beall on:
   a. What is preserved from classical logic?
   b. What is given up?
   c. What is the status of the T-schema?
   d. How does each handle revenge paradoxes?

10. Is there a "right" answer to the Liar? Argue for one position.

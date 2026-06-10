# Tarski's Hierarchy of Languages

Alfred Tarski's solution to the Liar (1933/1956): a truth predicate for language L must live in a *higher* language M — no language can contain its own truth predicate.

## The Metalanguage Solution

Tarski distinguished:
- **Object language**: The language we're talking about (e.g., first-order arithmetic)
- **Metalanguage**: The language we use to talk about the object language (e.g., informal mathematics + a truth predicate T)

The truth predicate T for the object language is defined in the metalanguage. The object language *cannot* express T.

**Consequence**: Self-referential sentences like the Liar cannot be expressed — they would need to use T within the object language. The paradox is blocked by stratification.

## The Tarski Hierarchy

Formalize this into an infinite hierarchy:
- L₀ = object language (no truth predicate)
- L₁ = L₀ + truth predicate T₀ for L₀
- L₂ = L₁ + truth predicate T₁ for L₁
- ...

T_n speaks about truth in L_n. The Liar for L_n ("This sentence in L_n is false") lives in L_{n+1}, where it is perfectly well-defined and true or false.

No sentence is "the Liar" simpliciter — every Liar sentence is a Liar for a specific level, and its truth value is determined at the next level.

## Convention T (Material Adequacy)

Tarski required a truth definition to satisfy:

> **Convention T**: A truth definition for L is adequate iff it implies all instances of: "φ" is true iff φ.

For example: "'Snow is white' is true iff snow is white."

The T-schema is not a definition of truth — it is a *constraint* any adequate definition must satisfy. Tarski provides an explicit compositional definition: truth of atomic formulas is primitive; truth of compounds is defined recursively.

## Criticism

The hierarchical solution is technically clean but philosophically controversial:

1. **No unrestricted truth**: We cannot say "everything Aristotle said is true" — Aristotle said things at various levels; "everything" would need a truth predicate higher than all of them.

2. **Natural language lacks levels**: English doesn't come in stratified levels. The hierarchy is an artifact of formalization, not a description of linguistic reality.

3. **The Strengthened Liar avoids the hierarchy**: "This sentence is not true at any level of the Tarski hierarchy." This requires a response beyond Tarski's framework.

These problems motivated Kripke's alternative: allow truth gaps rather than a hierarchy.

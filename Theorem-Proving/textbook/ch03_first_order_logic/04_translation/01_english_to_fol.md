# Translating English to FOL

## Overview
The translation between natural language and first-order logic is a core skill.
It requires identifying predicates, constants, quantifiers, and scope — and resolving
the ambiguities that natural language leaves open.

## Learning Objectives
- Follow a systematic procedure for translating English sentences to FOL
- Identify common translation patterns for "every," "some," "no," "only"
- Handle relative clauses, adjectives, and possessives

## Translation Procedure
1. Identify the main claim: is it universal or existential?
2. Identify the predicates needed
3. Choose variable names and write the quantifier structure
4. Fill in the atomic sentences

## Key Patterns

| English | FOL Pattern |
|---------|-------------|
| Every F is G | ∀x(F(x) → G(x)) |
| Some F is G | ∃x(F(x) ∧ G(x)) |
| No F is G | ∀x(F(x) → ¬G(x)) ≡ ¬∃x(F(x) ∧ G(x)) |
| Only Fs are G | ∀x(G(x) → F(x)) |
| Not every F is G | ¬∀x(F(x) → G(x)) ≡ ∃x(F(x) ∧ ¬G(x)) |
| At least two Fs are G | ∃x∃y(F(x)∧G(x)∧F(y)∧G(y)∧x≠y) |
| Exactly one F is G | ∃x(F(x)∧G(x)∧∀y((F(y)∧G(y))→y=x)) |

## Common Pitfall: "Some" vs. "Every"
"Every dog loves some human" is ambiguous:
- ∀x(Dog(x) → ∃y(Human(y) ∧ Loves(x,y))) — each dog loves at least one human
- ∃y(Human(y) ∧ ∀x(Dog(x) → Loves(x,y))) — some human is loved by all dogs

## Exercises
See `problems/ch03_predicate_logic/01_translation_exercises.md`

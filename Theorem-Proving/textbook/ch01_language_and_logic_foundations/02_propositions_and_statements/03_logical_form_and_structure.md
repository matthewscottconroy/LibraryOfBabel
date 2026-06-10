# Logical Form and Structure

## Overview
The **logical form** of a sentence is its structure as seen by logic — stripped of content,
revealing the pattern of connectives, quantifiers, and predicates. Recognizing logical form
is the skill that allows us to apply general logical principles to specific arguments.

## Learning Objectives
- Extract the logical form of natural language sentences
- Distinguish surface grammatical form from logical form
- Use logical form to evaluate argument validity

## Surface Form vs. Logical Form
"The present king of France is bald" has the surface form of a simple predication, but
Russell argued its logical form is existential: ∃x(K(x) ∧ ∀y(K(y) → y=x) ∧ B(x)).
This logical form reveals why the sentence is false (not meaningless) when France has no king.

## Formal Regimentation
The process of translating natural language into formal notation is called **regimentation**.
It requires:
1. Identifying the main connective or quantifier
2. Identifying predicates, names, and variables
3. Handling scope ambiguities explicitly

## Tool Connections
- **Tarski's World / LPL**: the translation exercises in LPL directly train regimentation skill
- **Lean 4**: `∀ x : α, P x` is the regimented form of "every x has property P"
- **Coq**: `forall x : T, P x` — Coq's syntax makes logical form explicit

## Real-World Applications
- Argument mapping in critical thinking tools (e.g., Rationale, Argdown)
- Legal reasoning: identifying the logical form of statutes clarifies their scope
- AI planning: STRIPS and PDDL encode action preconditions/effects in logical form

## Exercises
See `problems/ch01_language_and_logic_foundations/01_language_exercises.md`

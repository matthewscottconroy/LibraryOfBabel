# Anatomy of an Argument

## Overview
An **argument** is a set of propositions — the **premises** — offered in support of another
proposition — the **conclusion**. Understanding argument structure is the gateway to all of
formal logic and proof theory.

## Learning Objectives
- Identify premises and conclusion in natural language arguments
- Distinguish deductive from inductive arguments
- Recognize argument indicators in prose

## Premises and Conclusion
**Premises**: the propositions assumed or asserted as evidence
**Conclusion**: the proposition the argument is intended to establish

Indicator words signal the structure:
- Conclusion indicators: "therefore," "thus," "hence," "so," "it follows that"
- Premise indicators: "since," "because," "given that," "as," "for"

## Deductive vs. Inductive Arguments
- **Deductive**: the conclusion is claimed to *follow necessarily* from the premises
- **Inductive**: the premises are claimed to make the conclusion *probable*

Formal logic (and this textbook) focuses on deductive arguments.

## Standard Form
Rewrite any argument in standard form:
```
P1. [First premise]
P2. [Second premise]
...
∴  C. [Conclusion]
```
This makes the logical structure explicit and evaluation easier.

## Tool Connections
- **Fitch (LPL)**: Fitch is a natural deduction checker — it enforces explicit premise/conclusion
  structure and checks that each step follows from previous ones by a valid rule
- **Lean 4**: a `theorem` statement specifies premises (hypotheses) and conclusion explicitly

## Exercises
See `problems/ch01_language_and_logic_foundations/02_argument_analysis_problems.md`

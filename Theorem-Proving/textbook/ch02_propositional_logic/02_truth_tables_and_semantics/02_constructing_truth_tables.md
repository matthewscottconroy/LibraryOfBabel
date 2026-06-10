# Constructing Truth Tables

## Overview
A truth table systematically evaluates a wff under every possible valuation.
It is the canonical decision procedure for propositional logic — exponential in
the number of variables, but complete and mechanical.

## Learning Objectives
- Construct truth tables for formulas with 2 and 3 variables
- Use truth tables to check validity, equivalence, and satisfiability
- Understand the computational cost of truth tables (2ⁿ rows)

## Method
1. List all 2ⁿ valuations of the n variables (one per row)
2. Add columns for each subformula (left-to-right, inside-out)
3. The final column gives the truth value of the whole formula

## Example: p → (q → p)
| p | q | q→p | p→(q→p) |
|---|---|-----|---------|
| T | T |  T  |    T    |
| T | F |  T  |    T    |
| F | T |  F  |    T    |
| F | F |  T  |    T    |

All T in the last column: this is a tautology (it is in fact an axiom of propositional logic).

## Tool Connections
See `textbook/ch02_propositional_logic/02_truth_tables_and_semantics/03_truth_tables_in_python.py`

## Exercises
See `problems/ch02_propositional_logic/01_truth_table_worksheet.md`

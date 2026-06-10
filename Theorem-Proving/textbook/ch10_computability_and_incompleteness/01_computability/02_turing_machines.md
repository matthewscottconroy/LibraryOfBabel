# Turing Machines

## Overview
A **Turing machine** (Turing 1936) is a mathematical model of computation: an idealized
machine with an infinite tape, a finite state control, and a head that reads/writes symbols.
It is the standard model for defining computability and complexity.

## Learning Objectives
- Define a Turing machine formally
- Trace the execution of simple Turing machines
- State the Church-Turing thesis

## Formal Definition
A TM is a 7-tuple M = (Q, Σ, Γ, δ, q₀, q_accept, q_reject) where:
- Q: finite set of states
- Σ ⊆ Γ \ {□}: input alphabet (□ = blank symbol)
- Γ: tape alphabet
- δ: Q × Γ → Q × Γ × {L, R}: transition function
- q₀: start state
- q_accept, q_reject: accepting/rejecting halt states

## Example: TM that accepts {0ⁿ1ⁿ | n ≥ 0}
A classic non-regular language, recognizable by a TM:
1. Mark first 0, scan right to first 1, mark it
2. Return to leftmost marked 0, repeat
3. If 0s and 1s run out simultaneously: accept
4. If mismatch: reject

## Church-Turing Thesis
Every "effectively computable" function is Turing-computable.
This is not a theorem (it connects informal and formal notions) but is universally accepted.
Evidence: all other computation models (λ-calculus, recursive functions, RAM machines,
modern CPUs) are equivalent in power to Turing machines.

## Python TM Simulator
See `textbook/ch10_computability_and_incompleteness/04_computability_in_python/01_turing_simulator.py`

## Exercises
See `problems/ch10_computability/01_turing_machine_design.md`

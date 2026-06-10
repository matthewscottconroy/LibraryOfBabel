# Linear Temporal Logic (LTL)

## Overview
**Linear Temporal Logic** extends propositional logic with operators for reasoning about
time. It treats time as a linear sequence of states (moments). LTL is the standard
language for specifying properties of reactive and concurrent systems.

## Learning Objectives
- Read and write LTL formulas
- Evaluate LTL formulas over execution traces
- Translate English temporal requirements into LTL

## LTL Operators
- **X φ** (neXt): φ holds in the next state
- **F φ** (Finally/eventually): φ holds at some future state
- **G φ** (Globally/always): φ holds at all future states
- **φ U ψ** (Until): φ holds continuously until ψ holds; ψ must eventually hold
- **φ R ψ** (Release): ψ holds until and including a point where φ holds (or ψ holds forever)

F φ = ⊤ U φ      G φ = ¬F¬φ = φ R ⊥

## Common Specification Patterns
- **Safety** "nothing bad ever happens": G¬bad
- **Liveness** "something good eventually happens": G(request → F grant)
- **Fairness** "if enabled infinitely often, executed infinitely often": GF enabled → GF executed
- **Response** "every request is eventually acknowledged": G(req → F ack)
- **Persistence** "eventually, always in good state": FG good

## Example: Mutual Exclusion
Processes P1 and P2 never both in critical section:
G ¬(in_cs_1 ∧ in_cs_2)        (safety: mutual exclusion)
G(req_1 → F in_cs_1)          (liveness: P1 eventually gets access)

## Real-World Applications
- **SPIN model checker**: verifies LTL properties of concurrent systems
- **Linux kernel**: properties like "no double-free" expressible as LTL safety
- **Protocol verification**: TLS, MPI, distributed algorithms
- **Hardware**: FIFO queues, cache coherence protocols

## Exercises
See `problems/ch12_modal_logic/01_modal_logic_exercises.md`

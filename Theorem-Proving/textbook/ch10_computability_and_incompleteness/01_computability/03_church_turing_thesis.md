# The Church-Turing Thesis

## The Thesis

**Church-Turing Thesis**: Every function that is effectively computable (by any finite, mechanical, deterministic procedure) is computable by a Turing machine (equivalently: definable in the lambda calculus, or general recursive).

This is not a mathematical theorem — it cannot be proved within mathematics, since "effectively computable" is an informal notion, not a mathematically precise one. It is a *thesis* about the relationship between informal and formal concepts.

## Evidence For the Thesis

1. **Multiple independent characterizations agree**: Turing machines, lambda calculus, general recursive functions, random access machines, cellular automata, quantum computers (for classical functions), and all known realistic models of computation compute exactly the same class of functions.

2. **No counterexample in 90 years**: Despite many attempts to define "hypercomputation" (computation beyond Turing machines), no convincing physical or mathematical counterexample has emerged.

3. **Physical intuition**: Real computers are finitely specifiable and deterministic (ignoring quantum effects, which still don't exceed Turing computability for classical functions). Any finite mechanical procedure can be simulated by a Turing machine.

## Significance

The Church-Turing Thesis allows us to prove undecidability results in *any* reasonable computational model by simply analyzing Turing machines. If we show the halting problem is undecidable for TMs, it is undecidable for Python, for C, for any other universal language.

It also clarifies what "algorithm" means formally: an algorithm is a Turing machine (or equivalent). Any problem for which there is "an algorithm" is TM-decidable.

## Exercises
See [problems/ch10_computability/](../../../problems/ch10_computability/)

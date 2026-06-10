# Reductions and the Undecidability Hierarchy

## Reductions: Connecting Hard Problems

A **many-one reduction** (mapping reduction) from problem $A$ to problem $B$, written $A \leq_m B$, is a computable function $f$ such that:
$$w \in A \iff f(w) \in B$$

If $A \leq_m B$, then:
- If $B$ is decidable, so is $A$ (decide $A$ by computing $f(w)$ then deciding $B$)
- If $A$ is undecidable, so is $B$ (contrapositive)

Reductions let us prove new problems undecidable without starting from scratch each time.

## The Standard Reductions

Starting from HALT (the halting problem), we can prove many problems undecidable by reduction:

**HALT $\leq_m$ EMPTY**: Given $\langle M, w \rangle$, construct $M'$ that on input $x$: simulates $M$ on $w$; if $M$ halts, accept $x$. Then $M'$ accepts some string iff $M$ halts on $w$. So HALT $\leq_m$ $\overline{\text{EMPTY}}$.

**HALT $\leq_m$ REGULAR**: Does TM $M$ recognize a regular language? Given $\langle M, w \rangle$, construct $M'$ that accepts $\{a^n b^n\}$ if $M$ does not halt on $w$, and accepts $\Sigma^*$ if it does. $M'$'s language is regular iff $M$ halts. So HALT $\leq_m$ REGULAR.

This generalizes to **Rice's theorem**: *any non-trivial property of TM behavior is undecidable*.

## The Arithmetical Hierarchy

Undecidable problems are not all equally hard. The **arithmetical hierarchy** stratifies them:

- $\Sigma_1^0$: semi-decidable (r.e.) — halting problem, TM acceptance
- $\Pi_1^0$: co-semi-decidable — complement of halting, TM non-acceptance
- $\Sigma_2^0$: "ask a halting oracle twice" — e.g., "TM $M$ halts on infinitely many inputs"
- $\Pi_2^0$, $\Sigma_3^0$, $\Pi_3^0$, $\ldots$: increasingly harder

Each level is strictly harder than the previous — no algorithm at level $n$ can solve all problems at level $n+1$.

## In Practice

Understanding reductions helps in software engineering and theoretical computer science:
- To show your problem is hard: reduce a known hard problem TO it
- To solve your problem: reduce IT to a known solvable problem (SAT, LP, etc.)
- Compilation: source language programs are reduced to (translated into) target language programs

## Exercises
See [problems/ch10_computability/](../../../problems/ch10_computability/)

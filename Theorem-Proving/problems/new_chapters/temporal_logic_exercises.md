# Temporal Logic Exercises

## Basic LTL

1. Write LTL formulas for:
   a. "A request is always eventually acknowledged."
   b. "Between any two acknowledgments, there was a request."
   c. "The system never enters state A and state B simultaneously."
   d. "After entering state X, the system stays in X for at least 3 steps."

2. Are these LTL formulas satisfiable? If yes, give a trace. If no, prove unsatisfiability.
   a. G p ∧ F ¬p
   b. G(p → X p)
   c. F p ∧ G ¬p

3. Which of these are tautologies?
   a. G p → F p
   b. F G p → G F p
   c. G F p → F G p  ← Is this valid?

## CTL

4. Express these properties in CTL:
   a. "There exists a computation path where p holds infinitely often."
   b. "On all paths, every state eventually satisfies q."
   c. "There is a path avoiding state s entirely."

5. Show that "F G p" (in LTL) is not expressible in CTL. (Hint: find two Kripke structures that satisfy the same CTL formulas but differ on F G p.)

## Model Checking

6. Implement a CTL model checker for the formulas EX, AF, EG. Test it on:
   - A 3-state system modeling a simple mutex protocol
   - Verify the safety property AG ¬(cs₁ ∧ cs₂)

7. **The Dining Philosophers**: Model 3 philosophers with forks as a Kripke structure. Verify:
   a. Safety: At most one philosopher holds both forks at once.
   b. Liveness: Every hungry philosopher eventually eats (may fail — find the deadlock).

## Temporal Logic and Programs

8. Express these program properties in LTL:
   a. A loop terminates (may not be expressible in LTL — why?)
   b. A variable x is always non-negative.
   c. After a write to x, the next read of x returns the written value.

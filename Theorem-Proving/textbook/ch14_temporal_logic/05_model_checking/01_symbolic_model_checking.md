# Symbolic Model Checking

Explicit model checking enumerates states. For systems with 10^50 states, that's impossible. Symbolic model checking represents sets of states as *logical formulas* and operates on them symbolically.

## Binary Decision Diagrams (BDDs)

A *Binary Decision Diagram* (BDD) is a rooted, directed acyclic graph representing a Boolean function. With variable ordering fixed, BDDs are *canonical*: two equivalent Boolean functions have identical BDDs.

BDDs enable:
- **Set representation**: A set of states (each a valuation of state variables) is a Boolean function — 1 if the state is in the set, 0 otherwise.
- **Set operations**: Union = BDD-OR, intersection = BDD-AND, complement = BDD-NOT — all polynomial in BDD size.
- **Quantification**: ∃x. f(x, ...) eliminates a variable — computes the projection.

## Symbolic CTL Model Checking

Represent the transition relation R(s, s') as a BDD over state variables {s} ∪ {s'}.

```python
# EX phi = {s | ∃s'. R(s,s') ∧ phi(s')}
def pre_image(R, phi):
    # States with a phi-successor
    return exists_quantify(state_vars_primed, R & rename(phi, prime))

# EF phi = least fixpoint: phi ∨ EX(EF phi)
def check_ef_symbolic(R, phi):
    sat = phi
    while True:
        new_sat = sat | pre_image(R, sat)
        if new_sat == sat:
            return sat
        sat = new_sat
```

Each iteration extends the set one step backward. The fixpoint converges when no new states are reachable.

## SAT-Based Model Checking (Bounded)

*Bounded model checking* (BMC): unroll the transition relation k steps and encode the search as a SAT problem.

To find a counterexample of length k for AG p:
```
SAT( init(s₀) ∧ R(s₀,s₁) ∧ ... ∧ R(sₖ₋₁,sₖ) ∧ ¬p(sᵢ) for some i )
```

If SAT returns a satisfying assignment, we have a counterexample. If UNSAT for all k up to some bound, we gain confidence (but not certainty) in the property.

BMC tools (CBMC, IC3/PDR) can handle industrial systems with millions of state variables.

## Abstraction and CEGAR

For real systems (billions of states), even symbolic methods struggle. *CounterExample-Guided Abstraction Refinement* (CEGAR):

1. **Abstract** the system (merge states that look similar)
2. **Verify** the abstraction — it's smaller
3. If the abstraction **satisfies** the property: done (abstraction is sound for safety)
4. If the abstraction has a **counterexample**: check if it's real
5. If the counterexample is **spurious** (artifact of abstraction): refine the abstraction
6. Repeat

CEGAR is the basis of tools like SLAM (Microsoft's device driver verifier) and BLAST.

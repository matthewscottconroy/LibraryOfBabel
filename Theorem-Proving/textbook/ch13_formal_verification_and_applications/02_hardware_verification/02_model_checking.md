# Model Checking

> "Model checking is the first formal verification technique to have achieved widespread industrial adoption. It does for hardware and embedded software what testing does for traditional software — but exhaustively."
> — Standard characterization

## The Problem: Hardware Bugs Are Expensive

In 1994, Intel shipped the Pentium processor with a flaw in the floating-point division unit. The bug affected only certain calculations, but its impact was enormous: Intel recalled the processor at a cost of $475 million. The Pentium FDIV bug became a defining moment that catalyzed the development of formal verification for hardware.

The problem: hardware circuits execute the same deterministic logic billions of times per second. A bug baked into the hardware cannot be patched the way software bugs can. And testing, no matter how thorough, cannot exhaustively check all possible inputs to a modern processor.

**Model checking** addresses this: given a **formal model** of a hardware circuit (or protocol or software system) and a **formal specification** of its required behavior, **automatically verify** that the model satisfies the specification — by exhaustive state-space exploration.

## The Basic Setup

A **model** in model checking is a **Kripke structure** (from modal logic, ch12):

$$\mathcal{M} = (S, S_0, R, L)$$

- $S$: finite set of states
- $S_0 \subseteq S$: initial states
- $R \subseteq S \times S$: transition relation (which states can follow which)
- $L : S \to 2^{AP}$: labeling function (which atomic propositions hold in each state)

A **specification** is a formula in a temporal logic — typically **CTL** (Computation Tree Logic) or **LTL** (Linear Temporal Logic). The model checker answers: does $\mathcal{M} \models \varphi$?

## CTL: Computation Tree Logic

CTL formulas combine propositional logic with **temporal operators** prefixed by **path quantifiers**:

- **A** (for **A**ll paths): the property holds on all paths from the current state
- **E** (there **E**xists a path): the property holds on some path

Combined with temporal operators:
- **X** (ne**X**t): the next state
- **G** (**G**lobally / always): all future states
- **F** (**F**inally / eventually): some future state
- **U** (**U**ntil): until a condition is met

**CTL formulas** (using combined operators like AG, EF, AF, EG, EU, AU):

| Formula | Meaning |
|---------|---------|
| $AG\, p$ | $p$ holds in **all** states of **every** path (globally always) |
| $EF\, p$ | $p$ holds in **some** state of **some** path (possibly eventually) |
| $AF\, p$ | $p$ holds eventually on **every** path (inevitably) |
| $EG\, p$ | $p$ holds globally on **some** path (possibly always) |
| $AG(p \to AF\, q)$ | Whenever $p$ holds, $q$ will eventually hold (response property) |

**Examples in hardware verification**:
- $AG\, \neg\text{error}$: the error flag is never set (safety property)
- $AG(\text{request} \to AF\, \text{grant})$: every request is eventually granted (liveness)
- $AG(\text{granted} \to AX(\neg\text{granted}))$: mutual exclusion — after granting, no immediate re-grant

## The Model Checking Algorithm

The classic CTL model checking algorithm (Clarke, Emerson, Sifakis — Turing Award 2007) works by **labeling** each state with the subformulas that hold there:

```
procedure CHECK(M, φ):
  match φ with
  | atom p        → {s | p ∈ L(s)}
  | ¬ψ            → S \ CHECK(M, ψ)
  | ψ₁ ∧ ψ₂       → CHECK(M, ψ₁) ∩ CHECK(M, ψ₂)
  | EX ψ          → {s | ∃t. sRt ∧ t ∈ CHECK(M, ψ)}
  | EG ψ          → greatest fixpoint of λX. CHECK(M,ψ) ∩ EX(X)
  | E[ψ₁ U ψ₂]    → least fixpoint of λX. CHECK(M,ψ₂) ∪ (CHECK(M,ψ₁) ∩ EX(X))
```

The **fixpoint computation** terminates because the state space is finite. The entire algorithm runs in time $O(|S| \cdot |\varphi|)$ — linear in the state space times the formula size.

## The State Space Explosion Problem

The catch: the state space $S$ can be astronomically large. A hardware circuit with $n$ binary state variables has $2^n$ possible states. A modern processor might have millions of state variables.

Solutions:

**Symbolic model checking** (McMillan 1993): Represent sets of states as **Binary Decision Diagrams (BDDs)** — a canonical representation of Boolean functions. BDDs can compactly represent exponentially large state sets. Symbolic model checking verifies circuits with millions of states routinely.

**SAT-based model checking (BMC)**: **Bounded Model Checking** (Clarke, Biere, et al. 1999) encodes the question "does there exist a path of length ≤ k that violates φ?" as a SAT problem. Modern SAT solvers (CDCL-based) handle these massive instances efficiently. BMC finds bugs but does not prove correctness (only checks up to bound $k$).

**Abstraction**: Build an abstract model with fewer states that conservatively approximates the concrete system. Verify the abstract model. If it satisfies the spec, so does the concrete system. If the abstract model fails, check whether the counterexample is real (counterexample-guided abstraction refinement, CEGAR).

## Industrial Success Stories

**Intel (post-Pentium FDIV)**: Intel now uses formal verification for all floating-point units and cache coherence protocols. Their proprietary model checker has found hundreds of bugs that testing missed.

**AMD Athlon 64 FPU**: Verified using formal methods.

**AWS (Amazon Web Services)**: Uses **TLA+** (Temporal Logic of Actions, Lamport) to formally specify distributed systems protocols. Found bugs in DynamoDB, S3, and other systems that years of testing had not caught.

**Microsoft Azure**: Verifies key components using model checking tools.

**seL4**: A formally verified operating system microkernel — the code is proved correct against its specification in Isabelle/HOL, which is stronger than model checking (it is a full theorem-prover proof).

## A Python Mini Model Checker

```python
from collections import defaultdict

class KripkeModel:
    def __init__(self, states, transitions, labels, initial):
        self.S = states
        self.R = transitions        # dict: state -> set of successor states
        self.L = labels             # dict: state -> set of atomic propositions
        self.S0 = initial           # set of initial states

def check_ctl(M, formula):
    match formula:
        case ('atom', p):
            return {s for s in M.S if p in M.L[s]}
        case ('neg', phi):
            return M.S - check_ctl(M, phi)
        case ('and', phi, psi):
            return check_ctl(M, phi) & check_ctl(M, psi)
        case ('EX', phi):
            sat_phi = check_ctl(M, phi)
            return {s for s in M.S if M.R.get(s, set()) & sat_phi}
        case ('EF', phi):  # EF phi = E[True U phi]
            sat = check_ctl(M, phi)
            prev = set()
            while sat != prev:
                prev = sat.copy()
                sat |= check_ctl(M, ('EX', ('setlit', sat)))
            return sat
        case ('AG', phi):  # AG phi = ~EF ~phi
            return check_ctl(M, ('neg', ('EF', ('neg', phi))))
        case ('setlit', s):  # internal: "already computed set"
            return s

# Example: traffic light model
light = KripkeModel(
    states = {'red', 'green', 'yellow'},
    transitions = {'red': {'green'}, 'green': {'yellow'}, 'yellow': {'red'}},
    labels = {'red':    {'red', 'stopped'},
              'green':  {'green', 'go'},
              'yellow': {'yellow', 'caution'}},
    initial = {'red'}
)

# Safety: AG(green -> EF red) -- after green, red is eventually reachable
# (simplified: check AG ~(green AND red simultaneously))
safety = check_ctl(light, ('AG', ('neg', ('and', ('atom', 'green'), ('atom', 'red')))))
print("No green+red simultaneously (all states):", safety == light.S)
```

## Exercises
See [problems/ch13_applications/04_model_checking.md](../../../problems/ch13_applications/04_model_checking.md)

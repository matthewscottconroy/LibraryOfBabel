# Computation Tree Logic (CTL)

Where LTL reasons about a single linear trace, CTL reasons about the *tree of all possible futures* — the computation tree. This allows specification of branching behavior: "there exists a path where...", "for all paths..."

## Syntax

CTL formulas interleave *path quantifiers* with *temporal operators*:

- **Path quantifiers**: A (for All paths), E (there Exists a path)
- **Temporal operators**: X (next), F (finally), G (globally), U (until)

In CTL, every temporal operator must be immediately preceded by a path quantifier:

```
φ ::= p | ¬φ | φ ∧ ψ | φ ∨ ψ
    | AX φ | EX φ
    | AF φ | EF φ
    | AG φ | EG φ
    | A[φ U ψ] | E[φ U ψ]
```

## Semantics

Given a Kripke structure M = (S, →, L):

```
M, s ⊨ EX φ       iff  ∃t. s→t and M, t ⊨ φ
M, s ⊨ AX φ       iff  ∀t. s→t implies M, t ⊨ φ
M, s ⊨ EF φ       iff  ∃ path s=s₀,s₁,... and ∃i. M, sᵢ ⊨ φ
M, s ⊨ AG φ       iff  ∀ paths s=s₀,s₁,... and ∀i. M, sᵢ ⊨ φ
M, s ⊨ E[φ U ψ]   iff  ∃ path s=s₀,... and ∃j. (M,sⱼ⊨ψ and ∀i<j. M,sᵢ⊨φ)
M, s ⊨ A[φ U ψ]   iff  on ALL paths, φ holds until ψ
```

## CTL Model Checking Algorithm

CTL model checking runs in **polynomial time** (P) in the size of the model — a dramatic improvement over PSPACE for LTL. The key: CTL's syntax ensures fixpoint computations can be organized bottom-up.

The algorithm labels each state with the subformulas it satisfies, bottom-up:

```python
def check_ef(M, phi):
    # EF phi = least fixpoint of: phi OR EX(EF phi)
    sat = set(s for s in M.S if M.satisfies(s, phi))
    prev = None
    while sat != prev:
        prev = sat.copy()
        sat |= {s for s in M.S if any(t in sat for t in M.successors(s))}
    return sat

def check_ag(M, phi):
    # AG phi = greatest fixpoint of: phi AND AX(AG phi)
    sat = set(M.S)  # start with all states
    prev = None
    while sat != prev:
        prev = sat.copy()
        # Keep only states satisfying phi where all successors are in sat
        sat = {s for s in sat if M.satisfies(s, phi)
               and all(t in sat for t in M.successors(s))}
    return sat
```

Least fixpoints (F, U) are computed by starting small and growing. Greatest fixpoints (G) start large and shrink. The Knaster-Tarski theorem guarantees convergence.

## CTL vs. LTL

CTL and LTL are *incomparable* in expressiveness — neither subsumes the other:

- **CTL can express, LTL cannot**: EF p ("there exists a path reaching p"). LTL can only universally quantify over paths.
- **LTL can express, CTL cannot**: F G p ("eventually, p holds forever"). CTL cannot express this global path property without the EG/AG split.

CTL* (the next chapter) subsumes both.

## Practical Impact

CTL model checking is the basis of tools like:
- **SPIN**: LTL model checker using Büchi automata
- **NuSMV/nuXMV**: CTL + LTL model checker used in industry
- **PRISM**: Probabilistic model checker (PCTL = probabilistic CTL)

The Pentium FDIV bug (1994) was found after the fact. After it, Intel adopted formal verification using model checking, discovering bugs before chips were fabricated.

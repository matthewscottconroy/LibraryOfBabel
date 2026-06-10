# Semantic Tableaux

> "The method of semantic trees does not require creativity — it is a systematic search for a model. Its power comes from the fact that if a formula is valid, the search always terminates."
> — Melvin Fitting

## A Decision Procedure for Propositional Logic

**Semantic tableaux** (also called truth trees, analytic tableaux, or the Beth method) provide a decision procedure for propositional validity: a mechanical algorithm that, given any formula, correctly determines whether it is a tautology.

The method is **refutation-based**: to prove φ is valid, assume ¬φ and try to construct a model satisfying ¬φ (which would show φ is not valid). If every branch of the construction leads to contradiction, no such model exists, and φ is indeed valid.

## The Basic Idea

A tableau is a tree whose nodes are labeled with formulas. We expand each formula according to the connective rules, branching when needed:

**Non-branching rules** (both formulas go on the same branch):
- From $\neg\neg\varphi$: add $\varphi$
- From $\varphi \wedge \psi$: add $\varphi$ and $\psi$
- From $\neg(\varphi \vee \psi)$: add $\neg\varphi$ and $\neg\psi$
- From $\neg(\varphi \to \psi)$: add $\varphi$ and $\neg\psi$

**Branching rules** (split into two branches):
- From $\varphi \vee \psi$: branch into $\varphi \mid \psi$
- From $\neg(\varphi \wedge \psi)$: branch into $\neg\varphi \mid \neg\psi$
- From $\varphi \to \psi$: branch into $\neg\varphi \mid \psi$

A branch **closes** (marked ×) if it contains both $\varphi$ and $\neg\varphi$ for some formula $\varphi$.

A tableau **closes** if every branch closes.

If the tableau closes: the original formula (or ¬formula) has no model — the formula is valid (or unsatisfiable).

## Worked Example: Proving P ∨ ¬P

We prove $P \vee \neg P$ (excluded middle) is a tautology by assuming its negation and deriving contradiction.

**Assume** $\neg(P \vee \neg P)$. Apply rule for $\neg(\varphi \vee \psi)$:
1. $\neg P$
2. $\neg\neg P$

From $\neg\neg P$ (double negation): $P$.
But we have $\neg P$ and $P$ on the same branch — **contradiction**. The branch closes.

Since every branch closes, $\neg(P \vee \neg P)$ is unsatisfiable, so $P \vee \neg P$ is a tautology. ✓

## Worked Example: Testing Modus Ponens

Is the argument $P \to Q, P \vdash Q$ valid? Equivalently, is $(P \to Q) \wedge P \to Q$ a tautology?

**Assume** $\neg((P \to Q) \wedge P \to Q)$.

By $\neg(\varphi \to \psi)$ rule:
1. $(P \to Q) \wedge P$
2. $\neg Q$

From (1) by $\wedge$ rule:
3. $P \to Q$
4. $P$

From (3) by $\to$ rule: branch into $\neg P \mid Q$.

Branch 1: $\neg P$. We have $P$ (line 4) and $\neg P$ — **closed** ×.
Branch 2: $Q$. We have $\neg Q$ (line 2) and $Q$ — **closed** ×.

All branches closed: the formula is a tautology. Modus ponens is valid. ✓

## Countermodel Extraction

If the tableau does **not** close — if some branch remains open — we can read off a **countermodel** from that branch: assign each atom $p$ the value True if $p$ appears on the branch, False if $\neg p$ appears, and either value otherwise.

**Example**: Is $P \to Q, Q \vdash P$ (affirming the consequent) valid?

**Assume** $\neg((P \to Q) \wedge Q \to P)$.
After unfolding: on some branch, $P \to Q$, $Q$, $\neg P$.
From $P \to Q$ (branch): either $\neg P$ or $Q$.
- Branch with $\neg P$: we have $\neg P$, $Q$. No contradiction! **Open branch**.
- Countermodel: $P$ = False, $Q$ = True. Check: $P \to Q$ = T → T = T, $Q$ = T, $P$ = F. The premises are satisfied but conclusion fails. Invalid. ✓

## The Role of Tableaux in Automated Reasoning

Semantic tableaux are the basis of several automated theorem provers:
- **Analytic tableaux** for first-order logic (with additional rules for quantifiers and unification)
- **DPLL algorithm** (the basis of modern SAT solvers): essentially a tableau procedure for propositional CNF formulas, with clever heuristics for branch selection
- **Free-variable tableaux**: extend propositional tableaux to first-order logic with unification

The key properties that make tableaux practical:
- **Complete**: if a formula is valid, the closed tableau is always found (for propositional logic and first-order logic via the completeness theorem)
- **Systematic**: no creativity required; follow the rules mechanically
- **Refutation-based**: looking for a model that falsifies the formula is often more natural than constructing a proof

## Python Tableau Prover (Propositional)

```python
def is_literal(formula):
    return (formula[0] == 'atom' or
            (formula[0] == 'neg' and formula[1][0] == 'atom'))

def expand(formula):
    match formula:
        case ('neg', ('neg', phi)):       return [([phi], [])]       # ¬¬φ → φ
        case ('and', phi, psi):           return [([phi, psi], [])]  # φ∧ψ → φ, ψ
        case ('neg', ('or', phi, psi)):   return [([('neg', phi), ('neg', psi)], [])]
        case ('neg', ('impl', phi, psi)): return [([phi, ('neg', psi)], [])]
        case ('or', phi, psi):            return [([phi], []), ([psi], [])]  # branch
        case ('neg', ('and', phi, psi)):  return [([('neg', phi)], []), ([('neg', psi)], [])]
        case ('impl', phi, psi):          return [([('neg', phi)], []), ([psi], [])]
        case _:                           return None

def tableau_closed(formulas):
    atoms = {f[1] for f in formulas if f[0] == 'atom'}
    neg_atoms = {f[1][1] for f in formulas if f[0] == 'neg' and f[1][0] == 'atom'}
    if atoms & neg_atoms:  # contradiction found
        return True
    # Find a non-literal to expand
    for f in formulas:
        if not is_literal(f):
            branches = expand(f)
            rest = [g for g in formulas if g != f]
            return all(tableau_closed(rest + new) for new, _ in branches)
    return False  # open branch — countermodel exists

# Test
def valid(formula):
    return tableau_closed([('neg', formula)])

p = ('atom', 'p')
q = ('atom', 'q')
em = ('or', p, ('neg', p))
mp = ('impl', ('and', ('impl', p, q), p), q)
ac = ('impl', ('and', ('impl', p, q), q), p)

print("P ∨ ¬P valid:", valid(em))   # True
print("MP valid:", valid(mp))        # True
print("AC valid:", valid(ac))        # False (invalid: affirming consequent)
```

## Exercises
See [problems/ch04_proof_systems/](../../../problems/ch04_proof_systems/)

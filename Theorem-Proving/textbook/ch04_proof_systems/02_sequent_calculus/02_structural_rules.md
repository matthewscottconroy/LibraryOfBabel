# Structural Rules in Sequent Calculus

## What are Structural Rules?

**Structural rules** in sequent calculus manipulate the *context* (list of formulas on each side of $\vdash$) without introducing or eliminating logical connectives. They govern the administrative aspects of sequents.

The three classical structural rules for the left side (and symmetrically for the right in two-sided calculi):

**Weakening (W)**: Adding unused formulas to the context:
$$\frac{\Gamma \vdash \Delta}{\Gamma, \varphi \vdash \Delta}$$

If a sequent is derivable, it remains derivable when we add more hypotheses.

**Contraction (C)**: Merging duplicate formulas:
$$\frac{\Gamma, \varphi, \varphi \vdash \Delta}{\Gamma, \varphi \vdash \Delta}$$

Using a hypothesis twice can be collapsed to using it once.

**Exchange (E)**: Permuting formulas:
$$\frac{\Gamma, \varphi, \psi, \Delta \vdash \Sigma}{\Gamma, \psi, \varphi, \Delta \vdash \Sigma}$$

The order of hypotheses does not matter (sequences are treated as multisets/sets).

## Why Structural Rules Matter

In classical logic, structural rules are unproblematic and usually implicit. But **substructural logics** restrict them, revealing deep connections:

- **Linear logic** (Girard 1987): Removes weakening and contraction. Resources are used *exactly once*. Models resource-sensitive reasoning: a linear proof is a recipe that consumes exactly the listed resources.

- **Affine logic**: Removes contraction but keeps weakening. Resources can be used *at most once*.

- **Relevant logic**: Removes weakening. Every hypothesis must be *used* in a proof.

- **Ordered logic**: Removes exchange. The order of hypotheses matters.

Linear logic models session types in programming languages, process calculi (the π-calculus), and quantum computation (no-cloning theorem: quantum states cannot be copied — analogous to linear logic's prohibition on contraction).

## Cut Elimination

The most important structural rule (often not listed as structural but equally fundamental):

**Cut**:
$$\frac{\Gamma \vdash \varphi, \Delta \qquad \Sigma, \varphi \vdash \Pi}{\Gamma, \Sigma \vdash \Delta, \Pi}$$

If $\varphi$ is provable from $\Gamma$ (with $\Delta$ on the right) and $\Pi$ follows from $\varphi$ (with $\Sigma$), then $\Pi$ follows from $\Gamma$ (without mentioning $\varphi$).

**Gentzen's Cut Elimination Theorem**: Every sequent provable with Cut can be proved without Cut.

Cut elimination is one of the most important results in proof theory:
- It shows the completeness of cut-free proof systems
- Cut-free proofs have the **subformula property**: every formula in a proof is a subformula of the conclusion
- It is the sequent calculus analog of $\beta$-normalization in lambda calculus
- It implies consistency (no proof of $\vdash$ — empty sequent — without cut, so no contradiction)

## Exercises
See [problems/ch04_proof_systems/](../../../problems/ch04_proof_systems/)

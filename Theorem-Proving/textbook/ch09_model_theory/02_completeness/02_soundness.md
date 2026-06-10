# Soundness of First-Order Logic

## Soundness: Proofs Don't Lie

**Soundness** is the property that every *provable* statement is also *true* (in every model). If there is a formal derivation of $\varphi$ from $\Gamma$, then $\varphi$ is indeed a semantic consequence of $\Gamma$.

$$\Gamma \vdash \varphi \implies \Gamma \models \varphi$$

Without soundness, formal proof would be worthless — we could "prove" false things.

## Why Soundness Holds

Soundness is proved by showing each inference rule *preserves* truth:
- **Axioms**: Every propositional tautology is true in every model. ✓
- **Modus Ponens**: If $\varphi$ and $\varphi \to \psi$ are true at $w$, then $\psi$ is true at $w$. ✓
- **∀-Introduction**: If $\varphi(x)$ is true for an arbitrary $x$ (not free in the hypotheses), then $\forall x\, \varphi(x)$ is true. ✓
- **Equality axioms**: The interpretation of $=$ in any structure is the actual identity relation. ✓

By induction on proof length, every derivation preserves truth — so conclusions of proofs are semantically valid.

## Completeness (the Other Direction)

The converse — $\Gamma \models \varphi \implies \Gamma \vdash \varphi$ — is **completeness**, proved by Gödel in 1929. Together:

$$\Gamma \vdash \varphi \iff \Gamma \models \varphi$$

Formal proof and semantic truth coincide for first-order logic. This is a remarkable and non-trivial result — it is specific to first-order logic; second-order logic is incomplete.

## Exercises
See [problems/ch09_model_theory/](../../../problems/ch09_model_theory/)

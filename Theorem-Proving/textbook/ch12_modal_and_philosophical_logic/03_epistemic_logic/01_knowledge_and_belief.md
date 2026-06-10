# Epistemic Logic: Knowledge and Belief

## Formalizing "Knowing"

**Epistemic logic** extends modal logic to reason about *knowledge* and *belief* of agents. The modal operator $K_i \varphi$ means "agent $i$ knows that $\varphi$."

The **KT45** system (also called **S5** for knowledge) captures the idealized notion of knowledge:
- **K**: $K_i(\varphi \to \psi) \to (K_i\varphi \to K_i\psi)$ — knowledge distributes over implication
- **T**: $K_i\varphi \to \varphi$ — knowledge implies truth (you can only know truths)
- **4**: $K_i\varphi \to K_iK_i\varphi$ — positive introspection: if you know, you know that you know
- **5**: $\neg K_i\varphi \to K_i\neg K_i\varphi$ — negative introspection: if you don't know, you know you don't know

For **belief** ($B_i\varphi$), we drop the T axiom (you can believe false things): **KD45** is the standard belief system.

## Applications in Computer Science

**Distributed systems**: Epistemic logic captures what each process in a distributed system knows about the global state. The **common knowledge** operator ($CK\varphi$ — everyone knows, and everyone knows everyone knows, ad infinitum) is crucial for coordination.

**Security**: Access control, information flow, and protocol analysis can be modeled epistemically. "If agent $A$ knows the password, agent $A$ can access the resource."

**AI planning**: Planning under uncertainty often involves reasoning about what an agent knows and does not know.

## The Muddy Children Puzzle

Three children are playing. Some have mud on their foreheads. Each can see the others but not themselves. The father says "At least one of you has mud."

This classic puzzle demonstrates common knowledge and iterative reasoning:
- If only one child has mud, they immediately know (they see no mud on others)
- If two have mud, after round 1 of silence (no one raised their hand), each muddy child reasons: "if only I were muddy, the other would have raised their hand, so both of us are muddy"
- Etc.

The father's announcement creates **common knowledge** of "at least one has mud" — even though everyone already believed this individually, making it common knowledge enables the reasoning cascade.

## Lean Modeling

```lean
-- Epistemic state as a Kripke model
-- Worlds represent possible global states
-- Agent i's accessibility relation = states consistent with i's observations

-- In the muddy children puzzle:
-- Each state assigns mud/no-mud to each child
-- Child i's accessibility: states that agree with what i can see (others' foreheads)
-- K_i φ at world w: φ holds at all worlds accessible to i from w
```

## Exercises
See [problems/ch12_modal_logic/03_epistemic_exercises.md](../../../problems/ch12_modal_logic/03_epistemic_exercises.md)

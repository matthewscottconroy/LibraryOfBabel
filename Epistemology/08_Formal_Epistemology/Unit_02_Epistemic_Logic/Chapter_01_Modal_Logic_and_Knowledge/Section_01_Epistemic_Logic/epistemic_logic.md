# Epistemic Logic

Epistemic logic is the formal study of reasoning about knowledge and belief. It extends classical propositional or predicate logic with modal operators representing cognitive attitudes: "it is known that P," "it is believed that P," "it is possible that P (for all the agent knows)." Epistemic logic has applications in computer science (distributed systems, multi-agent reasoning, game theory), philosophy (formal modeling of epistemic concepts), and linguistics (the semantics of epistemic modals).

## Modal Operators

The basic epistemic operators:

**Kₐ P**: Agent a knows that P.
**Bₐ P**: Agent a believes that P.
**◇ₐ P** (or ¬Kₐ¬P): P is epistemically possible for agent a — for all a knows, P might be true.

These operators are formally dual: ¬Kₐ¬P says "it is not the case that a knows that P is false" — which is the epistemic possibility of P.

In multi-agent epistemic logic, the operators are indexed by agents: Ka represents what agent a knows, Kb what agent b knows. This allows modeling of distributed knowledge ("collectively, the group knows P"), common knowledge ("everyone knows P, and everyone knows that everyone knows P, etc."), and individual uncertainty about others' knowledge.

## The S5 System

The most commonly used axiom system for knowledge is S5. S5 includes:
- **Axiom K**: Ka(P → Q) → (KaP → KaQ) — knowledge distributes over implication
- **Axiom T**: KaP → P — if a knows P, then P is true (factivity of knowledge)
- **Axiom 4**: KaP → KaKaP — if a knows P, then a knows that a knows P (positive introspection)
- **Axiom 5**: ¬KaP → Ka¬KaP — if a doesn't know P, then a knows that a doesn't know P (negative introspection)

The S5 axioms collectively characterize a very idealized agent: one whose knowledge is factive, and who has perfect introspective access to what they know and don't know.

## Formal Semantics: Possible Worlds

Epistemic logic is typically given a possible-worlds semantics. An epistemic model consists of:
- A set of possible worlds W
- For each agent a, an accessibility relation Rₐ on W: world w' is accessible from world w for agent a if everything a knows in w is true in w'
- A valuation function V assigning truth values to propositions at each world

Agent a knows P in world w iff P is true in all worlds accessible from w for a.

The accessibility relation encodes the agent's knowledge: the worlds accessible from w are the worlds the agent can't distinguish from w — the worlds consistent with everything the agent knows. If P is true in all those worlds, the agent knows P.

Different axiom systems correspond to different properties of the accessibility relation:
- Axiom T (factivity): the relation is reflexive (w is accessible from itself — you can't know falsehoods)
- Axiom 4 (positive introspection): the relation is transitive
- Axiom 5 (negative introspection): the relation is Euclidean

## Logical Omniscience

A well-known limitation of standard epistemic logic is the logical omniscience problem. In standard possible-worlds semantics, if an agent knows the axioms of set theory, they know all their consequences (since consequences are true in all possible worlds). But no real agent knows all consequences of their knowledge.

Several solutions have been proposed:
- **Impossible worlds**: Extend the model with "impossible worlds" where logical laws fail, allowing agents to be uncertain about logical truths.
- **Syntactic approaches**: Represent knowledge by sets of sentences (not possible worlds), where the closure under logical consequence is not automatic.
- **Dynamic epistemic logic**: Model knowledge as it updates step by step, rather than as a static state.

## Common Knowledge and Group Epistemology

An important application of epistemic logic is the analysis of *common knowledge*. Common knowledge that P exists among a group when everyone knows P, and everyone knows that everyone knows P, and so on ad infinitum.

Common knowledge is required for successful coordination in game theory (Aumann's agreement theorem), for the use of language (Grice's conversational maxims presuppose common knowledge of linguistic conventions), and for many social institutions (the law is common knowledge; money is commonly known to be accepted).

Interestingly, common knowledge is often *not* achieved even when all participants know P. Suppose everyone knows P, and everyone knows that everyone knows P, but it's not common knowledge that everyone knows that everyone knows P. This is the basis of Rubinstein's email game, where iterative communication fails to establish common knowledge.

## Dynamic Epistemic Logic

Dynamic Epistemic Logic (DEL, Baltag, Moss, Solecki) extends static epistemic logic to model changes in knowledge due to actions — information updates, announcements, learning events. It models not just what agents know but how their knowledge changes when information is exchanged.

Applications include modeling communication protocols, analyzing puzzles about knowledge (the "muddy children" puzzle, "knowing whether"), and formalizing the update of knowledge when evidence is received.

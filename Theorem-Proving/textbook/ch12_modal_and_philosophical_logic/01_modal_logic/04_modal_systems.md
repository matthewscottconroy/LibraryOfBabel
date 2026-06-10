# Modal Systems: K, T, S4, S5

## A Hierarchy of Systems

Adding axioms to the base system K gives a hierarchy of successively stronger modal systems, each corresponding to a restricted class of frames.

## System K (Base)

The minimal normal modal logic. Valid on all Kripke frames. Contains:
- All classical propositional tautologies
- **K**: $\square(\varphi \to \psi) \to (\square\varphi \to \square\psi)$
- Modus ponens and necessitation

K is too weak for most applications — it places no constraints on the accessibility relation.

## System T (Reflexive)

K + **T**: $\square\varphi \to \varphi$ (what is necessary is true).

Valid on reflexive frames. Natural for alethic modality: necessity should imply truth. "It is necessarily raining" entails "it is raining."

## System S4 (Reflexive + Transitive)

T + **4**: $\square\varphi \to \square\square\varphi$ (iterated necessities collapse).

Valid on preorders (reflexive, transitive). Captures:
- **Intuitionistic logic** via the Gödel translation: $\varphi$ is intuitionistically valid iff $\square\varphi$ is S4-valid
- **Provability logic** (with modification): $\square$ as "provable in T"
- **Topological semantics**: interior operator on a topological space

## System S5 (Equivalence Relation)

S4 + **5**: $\Diamond\varphi \to \square\Diamond\varphi$ (what is possible is necessarily possible).

Valid on equivalence relation frames (reflexive, symmetric, transitive). All worlds in an equivalence class see each other. Strongest standard system:
- **Alethic modality**: metaphysical necessity and possibility
- **Epistemic logic** with perfect introspection (agents know what they know and don't know)
- **Logical validity** as a modal notion

In S5: $\square\varphi \leftrightarrow \varphi$ for *valid* $\varphi$, and $\square\varphi$ is either true everywhere or nowhere (within an equivalence class). Modalities stabilize.

## Summary Table

| System | Axioms | Frame property | Applications |
|--------|--------|----------------|-------------|
| **K** | — | Arbitrary | Base logic |
| **T** | T | Reflexive | Alethic (weak) |
| **S4** | T, 4 | Preorder | Provability, topology, intuitionistic |
| **S5** | T, 4, 5 | Equivalence | Metaphysical necessity, epistemic |
| **KD** | D | Serial | Deontic (obligation) |
| **KD45** | D, 4, 5 | Serial, Euclidean | Belief |

## Exercises
See [problems/ch12_modal_logic/01_modal_logic_exercises.md](../../../problems/ch12_modal_logic/01_modal_logic_exercises.md)

# Modal Logic: Necessity and Possibility

> "Water is necessarily H₂O — not just in the actual world, but in every possible world in which water exists. 'Water is H₂O' is necessary *a posteriori*."
> — Saul Kripke, *Naming and Necessity*, 1980

## The Grammar of Modality

In everyday reasoning, we make distinctions that classical logic cannot capture:

- "It is raining" — a plain assertion about actual fact
- "It might be raining" — possible, consistent with what we know
- "It must be raining" — necessary, unavoidable given what we know
- "Necessarily, 2+2=4" — true in every possible situation, not merely in fact

Classical propositional logic has a single notion of truth: a proposition is either true or false in the actual world. But human thought constantly traffics in *modal* judgments — claims about what *could*, *must*, *should*, or *will* be the case.

**Modal logic** extends classical logic with two new operators:
- $\square\varphi$: "Necessarily $\varphi$" — $\varphi$ holds in *every* accessible state
- $\Diamond\varphi$: "Possibly $\varphi$" — $\varphi$ holds in *some* accessible state

These operators are **dual**: $\square\varphi \equiv \neg\Diamond\neg\varphi$ and $\Diamond\varphi \equiv \neg\square\neg\varphi$. "Necessarily $\varphi$" means "it is not possible that not-$\varphi$."

## A Family of Modalities

The same formal operators $\square$ and $\Diamond$ can represent many different kinds of necessity and possibility, depending on interpretation:

| Interpretation | $\square\varphi$ means | $\Diamond\varphi$ means |
|---|---|---|
| **Alethic** | $\varphi$ is metaphysically necessary | $\varphi$ is metaphysically possible |
| **Epistemic** | An agent knows $\varphi$ | $\varphi$ is consistent with agent's knowledge |
| **Deontic** | $\varphi$ is obligatory | $\varphi$ is permitted |
| **Temporal** | $\varphi$ will always hold | $\varphi$ will hold at some future time |
| **Provability** | $\varphi$ is provable in some system | $\varphi$ is consistent with some system |
| **Program** | After every execution of program $P$, $\varphi$ holds | After some execution, $\varphi$ holds |

The same formal framework — the same axioms and semantics — illuminates all of these. Modal logic achieves a remarkable level of abstraction.

## The Motivating Question: What is Necessity?

Philosophers have argued about the nature of necessity for millennia. Before modal logic was formally developed, the debate was informal and often confused. The formal tools clarify the distinctions:

**Logical necessity** ($\square\varphi$ means $\varphi$ is a tautology): $\square(P \vee \neg P)$. This is the weakest form of necessity — holding in virtue of logical form alone.

**Mathematical necessity**: $\square(2 + 2 = 4)$. True in every possible world (assuming arithmetic is necessarily true). This is stronger than logical necessity — it is substantive, not merely formal.

**Physical (nomological) necessity**: $\square(F = ma)$. True in every world with the same physical laws as ours. But is it true in *every* possible world? Philosophers disagree.

**Metaphysical necessity**: Kripke's concept — true in every possible world, period. Water is necessarily H₂O: even in worlds where water is called "phlogiston" or not called anything at all, if there is water, it is H₂O. This is necessity without qualification.

Modal logic lets us reason about these distinctions formally, even without settling the underlying philosophical debates.

## Basic Syntax

The **syntax of modal logic** extends propositional logic:
$$\varphi ::= p \mid \neg\varphi \mid \varphi \wedge \varphi \mid \varphi \vee \varphi \mid \varphi \to \varphi \mid \square\varphi \mid \Diamond\varphi$$

where $p$ ranges over propositional atoms.

**Reading examples**:
- $\square p$: "Necessarily $p$"
- $\Diamond p$: "Possibly $p$"
- $\square(p \to q)$: "Necessarily, if $p$ then $q$"
- $\square p \to p$: "What is necessary is true" (the T axiom)
- $p \to \square\Diamond p$: "What is true is necessarily possibly true" (the B axiom)
- $\Diamond\square p$: "It is possible that $p$ is necessarily true"
- $\square\Diamond p$: "It is necessarily possible that $p$" (different from the above!)

Notice: $\Diamond\square p$ and $\square\Diamond p$ are generally distinct. In temporal logic, $\square\Diamond p$ says "$p$ will hold infinitely often" (at every time, there is a future time when $p$ holds), while $\Diamond\square p$ says "$p$ will eventually hold forever" (there is a time after which $p$ always holds).

## The K Axiom: Distribution

The weakest normal modal system is **K** (named after Kripke), containing:

1. All classical propositional tautologies
2. **K axiom**: $\square(\varphi \to \psi) \to (\square\varphi \to \square\psi)$
3. **Necessitation rule**: If $\vdash \varphi$, then $\vdash \square\varphi$
4. **Modus Ponens**

The K axiom says: necessity distributes over implication. If it is necessarily the case that "$p$ implies $q$", and it is necessarily the case that $p$, then it is necessarily the case that $q$. This is the modal analog of modus ponens.

Necessitation says: logical theorems are necessary. This is uncontroversial for logical necessity; more controversial for physical necessity (the laws of logic might hold even in physically impossible worlds, for instance).

## The System Hierarchy

Adding axioms to K gives progressively stronger systems:

| Axiom | Schema | Corresponds to (in Kripke semantics) |
|-------|--------|--------------------------------------|
| T | $\square\varphi \to \varphi$ | Reflexivity: every world is accessible from itself |
| 4 | $\square\varphi \to \square\square\varphi$ | Transitivity of the accessibility relation |
| 5 | $\Diamond\varphi \to \square\Diamond\varphi$ | Euclidean: if $wRv$ and $wRu$, then $vRu$ |
| B | $\varphi \to \square\Diamond\varphi$ | Symmetry: if $wRv$ then $vRw$ |
| D | $\square\varphi \to \Diamond\varphi$ | Seriality: every world has at least one successor |

The most important combinations:
- **K + T = T**: Reflexive frames (what is necessary is true)
- **K + T + 4 = S4**: Reflexive, transitive frames (the "logic of necessity" in most contexts)
- **K + T + 4 + 5 = S5**: Equivalence relation frames (the strongest standard alethic modal logic)
- **K + D = KD**: Serial frames (used for deontic logic — every situation has a deontic successor)

## Philosophical Significance of S5

**S5** is characterized by the **Euclidean + reflexive + transitive = equivalence relation** frames. In S5, all worlds are accessible from all worlds (in the same equivalence class). This makes necessity absolute: if $\varphi$ is necessary in one world, it is necessary in all worlds. The axiom 5 captures this: $\Diamond\varphi \to \square\Diamond\varphi$ — "if something is possible, it is necessarily possible."

S5 is the standard logic for **metaphysical necessity** (Kripke), **logical necessity**, and in epistemology (when the agent has perfect introspective access to their own knowledge states).

The modal ontological argument for God's existence (Plantinga's version) can be formalized in S5:
1. It is *possible* that a maximally great being exists: $\Diamond G$
2. A maximally great being is *necessarily* great: $G \to \square G$
3. Therefore, in S5: $\Diamond\square G$ (it is possible that $G$ is necessary)
4. In S5: $\Diamond\square G \to \square G$ (a possibility of necessity is a necessity)
5. Therefore: $\square G$ — a maximally great being necessarily exists

Whether you find the argument compelling or not, the formalization clarifies exactly which assumptions are doing the work.

## Applications in Computer Science

Modal logic is not merely philosophical. It has become indispensable in computer science:

**Program logics**: Dynamic logic uses $[\pi]\varphi$ ("after every execution of program $\pi$, $\varphi$ holds") and $\langle\pi\rangle\varphi$ ("there exists an execution of $\pi$ after which $\varphi$ holds"). These are modal operators indexed by programs.

**Temporal logic**: **LTL** (Linear Temporal Logic) and **CTL** (Computation Tree Logic) extend modal logic with operators for reasoning about time sequences. These are the foundation of **model checking** — automated verification of hardware and software systems (section 03 below).

**Epistemic logic**: Models distributed systems where different agents have different knowledge. Used to analyze protocols like the Byzantine Generals problem: under what conditions do all generals agree on a decision despite possible traitors?

**Provability logic (GL)**: $\square\varphi$ means "$\varphi$ is provable in Peano Arithmetic." The system GL (Gödel-Löb) satisfies $\square(\square\varphi \to \varphi) \to \square\varphi$ (Löb's theorem) — and the second incompleteness theorem becomes a theorem of GL.

## Exercises
See [problems/ch12_modal_logic/01_modal_logic_exercises.md](../../../problems/ch12_modal_logic/01_modal_logic_exercises.md)

# Kripke Frames and Frame Validity

## Frames vs. Models

A **Kripke frame** $\mathcal{F} = (W, R)$ is the bare accessibility structure — worlds and the relation between them — without a valuation.

A **Kripke model** adds a valuation $V : \text{Atoms} \to \mathcal{P}(W)$.

**Validity on a frame**: $\mathcal{F} \models \varphi$ means $\varphi$ is valid in *every* model based on frame $\mathcal{F}$.

**Validity in a class of frames**: $\varphi$ is valid on every frame in class $\mathbf{C}$.

The **fundamental theorem of modal completeness**: Each normal modal logic $\mathbf{L}$ is complete with respect to its *corresponding class of frames* — the frames on which all of $\mathbf{L}$'s axioms are valid.

## The Correspondence Table

| Axiom | Valid exactly on frames where $R$ is... |
|-------|----------------------------------------|
| **T**: $\square\varphi \to \varphi$ | Reflexive ($wRw$ for all $w$) |
| **4**: $\square\varphi \to \square\square\varphi$ | Transitive |
| **5**: $\Diamond\varphi \to \square\Diamond\varphi$ | Euclidean |
| **B**: $\varphi \to \square\Diamond\varphi$ | Symmetric |
| **D**: $\square\varphi \to \Diamond\varphi$ | Serial ($\forall w\, \exists v,\; wRv$) |

## Bisimulation

Two Kripke models $\mathcal{M}$ and $\mathcal{N}$ are **bisimilar** (at worlds $w$ and $v$) if:
- They agree on all atoms at $w$ and $v$
- For every successor $w'$ of $w$ in $\mathcal{M}$, there is a successor $v'$ of $v$ in $\mathcal{N}$ that is bisimilar to $w'$ (and vice versa)

**Key theorem**: Bisimilar worlds satisfy exactly the same modal formulas. Modal logic cannot distinguish bisimilar models.

This makes bisimulation the "modal" notion of isomorphism — and it corresponds exactly to the indistinguishability in process calculi and concurrent programming languages (CCS, the π-calculus).

## Exercises
See [problems/ch12_modal_logic/01_modal_logic_exercises.md](../../../problems/ch12_modal_logic/01_modal_logic_exercises.md)

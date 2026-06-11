# Chapter 12 Overview: Modal Logic

---

## Central Question

"It is necessarily true that all bachelors are unmarried." "It is possible that it will rain tomorrow." "After this step, it will eventually be the case that the system terminates." These sentences involve *modalities* — operators that qualify the mode in which a proposition holds: necessarily, possibly, sometimes, always, it is known that, it is believed that, it is obligatory that.

How do we give a rigorous semantics to such sentences? And what are the structural properties of different modality types?

---

## Why This Chapter Matters

Modal logic is the common framework for reasoning about necessity and possibility (alethic logic), time (temporal logic, Chapter 14), knowledge (epistemic logic), obligation (deontic logic), programs (dynamic logic), and mathematical provability (provability logic). The key tool — Kripke semantics — gives an intuitive possible-worlds interpretation and allows the different modal systems (K, T, S4, S5) to be precisely characterised by structural properties of the accessibility relation.

---

## Key Definitions

**Modal operators.** The standard modal operators are $\Box$ ("necessarily" / "always" / "it is known that") and $\Diamond$ ("possibly" / "sometimes" / "it is consistent with what is known that"). They are duals: $\Diamond\phi \equiv \neg\Box\neg\phi$.

**Modal formulas.** The language of propositional modal logic extends propositional logic with:

$$\phi ::= p \mid \bot \mid \neg\phi \mid (\phi \land \psi) \mid (\phi \to \psi) \mid \Box\phi \mid \Diamond\phi$$

**Kripke frame.** A Kripke frame is a pair $\mathcal{F} = (W, R)$ where $W$ is a non-empty set of *worlds* (or states, situations) and $R \subseteq W \times W$ is the *accessibility relation* ($wRv$ means "from world $w$, world $v$ is accessible / possible / reachable").

**Kripke model.** A Kripke model $\mathcal{M} = (W, R, V)$ adds a *valuation* $V: \text{Prop} \to \mathcal{P}(W)$, assigning to each propositional variable the set of worlds where it is true.

**Truth at a world.** The satisfaction relation $\mathcal{M}, w \vDash \phi$:

- $\mathcal{M}, w \vDash p$ iff $w \in V(p)$
- $\mathcal{M}, w \vDash \Box\phi$ iff for all $v$ with $wRv$: $\mathcal{M}, v \vDash \phi$
- $\mathcal{M}, w \vDash \Diamond\phi$ iff there exists $v$ with $wRv$ such that $\mathcal{M}, v \vDash \phi$

**Valid in a frame.** $\phi$ is valid in frame $\mathcal{F}$ (written $\mathcal{F} \vDash \phi$) if $\mathcal{M}, w \vDash \phi$ for all models $\mathcal{M}$ on $\mathcal{F}$ and all worlds $w$.

---

## The Main Modal Systems

Different modal systems correspond to different structural properties of the accessibility relation $R$:

| System | Additional axiom schema | Property of $R$ |
|--------|------------------------|-----------------|
| K (base) | none (just $\Box(\phi \to \psi) \to (\Box\phi \to \Box\psi)$) | none |
| T | $\Box\phi \to \phi$ | reflexive ($wRw$ for all $w$) |
| K4 | $\Box\phi \to \Box\Box\phi$ | transitive |
| S4 | $\Box\phi \to \phi$ and $\Box\phi \to \Box\Box\phi$ | reflexive + transitive (preorder) |
| S5 | S4 + $\Diamond\phi \to \Box\Diamond\phi$ | equivalence relation |
| D | $\Box\phi \to \Diamond\phi$ | serial ($\forall w \exists v: wRv$) |
| B | S4 + $\phi \to \Box\Diamond\phi$ | reflexive + symmetric |

**Theorem (Soundness and Completeness of S4).** A formula $\phi$ is provable in S4 iff it is valid in all frames where $R$ is a preorder (reflexive and transitive).

**Theorem (Soundness and Completeness of S5).** A formula $\phi$ is provable in S5 iff it is valid in all frames where $R$ is an equivalence relation.

---

## Correspondence Theory

A central achievement of modal logic is the correspondence between frame properties and axiom schemas:

**Theorem (Sahlqvist 1975).** Every Sahlqvist formula (a syntactically defined class of modal formulas) corresponds to a first-order condition on frames, and the modal logic axiomatised by Sahlqvist formulas is complete with respect to its semantics.

**Key correspondences:**

- $\Box\phi \to \phi$ (T axiom) $\iff$ $R$ is reflexive: $\forall w, wRw$
- $\Box\phi \to \Box\Box\phi$ (4 axiom) $\iff$ $R$ is transitive: $\forall w,v,u. (wRv \land vRu \to wRu)$
- $\Diamond\phi \to \Box\Diamond\phi$ (5 axiom) $\iff$ $R$ is Euclidean: $\forall w,v,u. (wRv \land wRu \to vRu)$

*Proof of T iff reflexivity:* ($\to$) Let $R$ be reflexive. For any world $w$ and $\phi$ with $\mathcal{M},w \vDash \Box\phi$: since $wRw$, we have $\mathcal{M},w \vDash \phi$. So $\Box\phi \to \phi$ holds at $w$. ($\leftarrow$) Suppose $R$ is not reflexive at $w$ (no $wRw$). Let $V(p) = W \setminus \{w\}$. Then $\mathcal{M},w \vDash \Box p$ (vacuously, since there are no accessible worlds from $w$ that violate $p$ — wait, actually reflexivity failing means $w \notin R(w)$, so we need a model). Let $V(p) = \emptyset$ (so $p$ is false everywhere). Define $W = \{w\}$ and $R = \emptyset$ (no $w$-accessible worlds). Then $\mathcal{M}, w \vDash \Box p$ vacuously, but $\mathcal{M}, w \not\vDash p$. So T fails. $\square$

---

## Provability Logic (GL)

**Löb's axiom:** $\Box(\Box\phi \to \phi) \to \Box\phi$ (from S4 plus this axiom, giving the logic GL).

**Theorem (Solovay 1976).** The propositional modal logic GL is arithmetically sound and complete for provability in Peano Arithmetic: replace $\Box$ by "is provable in PA." A modal formula $\phi$ is a theorem of GL iff, for every arithmetical interpretation, the corresponding arithmetical formula is provable in PA.

This is a deep result connecting the modal logic GL to Gödel's second incompleteness theorem (which corresponds to the unprovability of $\Box\bot \to \bot$ unless $\bot$ is actually provable — i.e., unless the system is inconsistent).

---

## Epistemic Logic

In epistemic logic, $\Box$ becomes $K_i$ ("agent $i$ knows that") and $\Diamond$ becomes $\hat{K}_i$ ("it is possible for all agent $i$ knows that").

**System S5** is standardly used for epistemic logic: knowledge is reflexive (you know only truths), transitive (if you know $\phi$, you know that you know $\phi$), and Euclidean (if you don't know $\phi$, you know that you don't know $\phi$).

The **muddy children puzzle** and the **common knowledge** operator are standard examples. Common knowledge $C_G\phi$ ("all agents in group $G$ commonly know $\phi$") requires infinitely many iterations of the $K$ operator and is not finitely axiomatisable in S5 without fixed-point operators.

---

## Historical Context

**Clarence Irving Lewis (1912, 1918)** introduced modal logic in reaction to the "paradoxes of material implication" in classical propositional logic. He defined the strict conditional and developed modal axiom systems S1–S5.

**Saul Kripke (1959, 1963)** introduced possible-worlds semantics (Kripke frames) in a series of papers beginning at age 19. This provided the standard semantics for modal logic and enabled rigorous completeness proofs.

**David Lewis (1973)** used possible-worlds semantics for counterfactual conditionals ("if kangaroos had no tails, they would topple over"), arguing that possible worlds are as real as the actual world (modal realism).

**Robert Solovay (1976)** established the completeness of GL for arithmetic provability, connecting modal logic to proof theory in a precise mathematical sense.

---

## Connections to Other Chapters

- **Chapter 14** (Temporal Logic): temporal logic is modal logic with multiple accessibility relations (next, until, always, eventually).
- **Chapter 13** (Formal Verification): model checking algorithms verify modal/temporal properties of systems — the Kripke semantics is the mathematical model of the system being verified.
- **Chapter 10** (Computability): provability logic (GL) directly encodes the provability predicate of Chapter 10.

# 3.1 Kripke Semantics: Frames, Forcing, and Completeness

## Beyond Truth Tables

Classical propositional logic has a simple, elegant semantics: truth tables. Each proposition is either true or false (0 or 1), and the truth value of a compound formula is determined by the truth values of its components. This Boolean algebra semantics is sound and complete for CPC.

Intuitionistic logic requires a richer semantics. The problem is that IPC doesn't satisfy the Boolean law $A \vee \neg A$: some propositions are "neither proven nor refuted." We need a semantics that can model this "intermediate" state.

The right framework was developed by Saul Kripke in the 1960s: **Kripke semantics**, which models intuitionistic logic using *possible worlds* ordered by increasing knowledge.

## Kripke Frames and Models

**Definition.** A *Kripke frame* is a pair $(W, \leq)$ where:
- $W$ is a non-empty set of *worlds* (or *states*, or *stages of knowledge*)
- $\leq$ is a *preorder* on $W$ (reflexive and transitive)

Intuitively: $w \leq w'$ means "world $w'$ is an extension of $w$" — from $w$, knowledge can grow to $w'$. Information is monotone: once you know something, you can't "forget" it.

**Definition.** A *Kripke model* is a Kripke frame $(W, \leq)$ together with a *valuation* $V$ that assigns to each propositional variable $P$ a set $V(P) \subseteq W$ satisfying the **persistence** (monotonicity) condition:
$$\text{If } w \in V(P) \text{ and } w \leq w', \text{ then } w' \in V(P)$$

Persistence says: if you know $P$ at stage $w$, you still know $P$ at any later stage $w'$.

## The Forcing Relation

We define when a proposition is "forced at a world."

**Definition.** For a Kripke model $(W, \leq, V)$, we define the *forcing relation* $w \Vdash \varphi$ (read: "world $w$ forces $\varphi$") inductively:

$$w \Vdash P \iff w \in V(P) \quad \text{(for atomic } P)$$

$$w \Vdash \varphi \wedge \psi \iff w \Vdash \varphi \text{ and } w \Vdash \psi$$

$$w \Vdash \varphi \vee \psi \iff w \Vdash \varphi \text{ or } w \Vdash \psi$$

$$w \Vdash \varphi \to \psi \iff \forall w' \geq w:\; (w' \Vdash \varphi \Rightarrow w' \Vdash \psi)$$

$$w \Vdash \bot \iff \text{never (false at every world)}$$

$$w \Vdash \neg\varphi \iff \forall w' \geq w:\; w' \not\Vdash \varphi$$

The clauses for $\wedge$ and $\vee$ are the same as in Boolean semantics. The crucial difference is in $\to$ and $\neg$:

- **Implication** $\varphi \to \psi$ at world $w$ means: in *every* future world $w' \geq w$ where $\varphi$ holds, $\psi$ also holds. This is a universal statement about all future worlds — much stronger than just "$\varphi$ doesn't hold at $w$ or $\psi$ holds at $w$" (the classical reading).

- **Negation** $\neg\varphi$ at world $w$ means: in no future world does $\varphi$ hold. This means $\varphi$ is permanently refuted from world $w$ onwards.

**Persistence Theorem.** For any formula $\varphi$: if $w \Vdash \varphi$ and $w \leq w'$, then $w' \Vdash \varphi$.

*Proof.* By induction on $\varphi$. The atomic case is by the persistence condition on $V$. The cases for $\wedge$ and $\vee$ follow immediately. For $\to$: if $w \Vdash \varphi \to \psi$ (so all $v \geq w$ with $v \Vdash \varphi$ satisfy $v \Vdash \psi$) and $w \leq w'$, then all $v \geq w'$ also satisfy $v \geq w$, so the condition still holds. For $\neg$: similar. $\square$

Persistence mirrors the BHK idea: if you have a proof at stage $w$, you have it at all later stages too.

## LEM Fails in a Kripke Model

The key example: LEM fails in a Kripke model.

**Example.** Consider the frame $W = \{w_0, w_1\}$ with $w_0 \leq w_1$ (and $w_0 \leq w_0$, $w_1 \leq w_1$). Set $V(P) = \{w_1\}$ (P is known only at the later world).

- $w_0 \not\Vdash P$ (P is not known at $w_0$).
- $w_0 \not\Vdash \neg P$: we need all $w' \geq w_0$ to not force $P$. But $w_1 \geq w_0$ and $w_1 \Vdash P$. So $w_0 \not\Vdash \neg P$.
- Therefore $w_0 \not\Vdash P \vee \neg P$.

This is a Kripke model where LEM fails. Think of $w_0$ as the "current state of knowledge" where we haven't yet determined $P$, and $w_1$ as a "later state" where we've established $P$.

**Intuition:** LEM fails because it would require deciding $P$ at every possible state of knowledge, including states where no decision has been made.

## Soundness and Completeness

A formula $\varphi$ is *valid* in a Kripke frame $(W, \leq)$ if for every valuation $V$ and every world $w \in W$, $w \Vdash \varphi$.

**Theorem (Soundness).** If IPC $\vdash \varphi$, then $\varphi$ is valid in every Kripke frame.

*Proof.* By induction on the derivation. Each rule of IPC preserves validity in Kripke frames (verify this for each rule). The key cases:

- **Modus ponens:** If $w \Vdash \varphi \to \psi$ and $w \Vdash \varphi$, then by the definition of forcing for $\to$ (with $w' = w$), $w \Vdash \psi$.

- **$\to$-Introduction:** If $w \Vdash \varphi$ forces $w \Vdash \psi$ (for all worlds in the model), then for any $w' \geq w$, if $w' \Vdash \varphi$ then $w' \Vdash \psi$. Hence $w \Vdash \varphi \to \psi$. $\square$

**Theorem (Completeness, Kripke 1965).** If $\varphi$ is valid in every Kripke frame, then IPC $\vdash \varphi$.

*Proof sketch.* The proof uses the *canonical model*. If IPC $\not\vdash \varphi$, we build a Kripke model in which $\varphi$ fails.

The worlds of the canonical model are the *maximally consistent theories* extending IPC — sets $T$ of formulas such that IPC $+ T$ is consistent and $T$ is deductively closed. The order is inclusion: $T \leq T'$ iff $T \subseteq T'$. The valuation sets $V(P) = \{T : P \in T\}$.

One verifies that in this model, $w \Vdash \psi$ iff $\psi \in w$ (for each world $w$ = maximally consistent theory). Then $\varphi \notin$ the base world (the minimally consistent extension of IPC), so $\varphi$ fails at that world. $\square$

**Consequence.** IPC $\vdash \varphi$ iff $\varphi$ is valid in all Kripke frames. This is the semantic characterization of IPC.

## Heyting Algebras: The Algebraic Perspective

Kripke frames are a special case of a more general semantics: **Heyting algebras**.

A *Heyting algebra* is a bounded lattice $(H, \leq, \top, \bot, \wedge, \vee, \to)$ where the implication $a \to b$ is the *relative pseudocomplement*: the largest element $c$ with $a \wedge c \leq b$.

Every Kripke frame gives a Heyting algebra (the lattice of "upward closed" subsets of $W$, ordered by inclusion). The open sets of any topological space form a Heyting algebra. In this algebra, $U \to V$ is $\text{int}(U^c \cup V)$ (the interior of the set-theoretic implication).

**Soundness and completeness for Heyting algebras:** IPC $\vdash \varphi$ iff $\varphi$ evaluates to $\top$ in every Heyting algebra.

This algebraic semantics connects intuitionistic logic to:
- **Topology:** Open sets of topological spaces form Heyting algebras. IPC is the logic of topology.
- **Sheaf models:** Sheaves on a topological space are models of intuitionistic set theory and type theory.
- **Toposes:** A Grothendieck topos has an internal language that is an intuitionistic higher-order logic. HoTT can be interpreted in $\infty$-toposes.

This topological/categorical perspective is the deep reason HoTT is built on intuitionistic foundations: $\infty$-toposes naturally model the internal logic of HoTT, and their internal logic is intuitionistic.

## Kripke Semantics for First-Order Logic

The semantics extends to first-order logic. A Kripke model for first-order logic has:
- A Kripke frame $(W, \leq)$.
- For each world $w$, a domain $D_w$ of individuals.
- Domains are increasing: if $w \leq w'$ then $D_w \subseteq D_{w'}$ (as we move to later worlds, more objects might come into existence).
- Interpretations of predicates and functions that are persistent.

The forcing clauses for quantifiers:
- $w \Vdash \forall x, P(x)$ iff for all $w' \geq w$ and all $a \in D_{w'}$: $w' \Vdash P(a)$.
- $w \Vdash \exists x, P(x)$ iff there exists $a \in D_w$ with $w \Vdash P(a)$.

Note: $\exists$ is "local" (witness must exist in the current domain) while $\forall$ is "global" (must hold for all objects in all future extensions). This asymmetry reflects the BHK clauses.

**Non-constancy of domains** models the mathematical phenomenon that we might "discover" new objects as we learn more. A world where only finitely many natural numbers are known can extend to one where more are known.

## The Connection to HoTT

In HoTT, types are modeled as $\infty$-groupoids (spaces with homotopy structure), and the universe of types is an $\infty$-topos. The internal logic of this $\infty$-topos is intuitionistic — it's exactly the logic of Kripke frames, generalized to higher-dimensional homotopical structure.

Concretely:
- A type $A$ in HoTT corresponds to a "sheaf" on the "classifying $\infty$-topos."
- A proof of a proposition $P$ in HoTT is a term of the corresponding type.
- The forcing relation $w \Vdash P$ corresponds to the fiber of the sheaf at the point $w$.

The philosophical move from classical to intuitionistic logic is thus not just a restriction but a *generalization*: intuitionistic logic is the logic appropriate for reasoning about *spaces*, while classical logic is the special case appropriate for reasoning about *discrete sets*.

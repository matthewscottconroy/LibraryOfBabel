# Kripke Semantics

## The Problem of Intuitionistic Semantics

Classical propositional logic has a clean semantics: truth tables. A formula is a tautology if and only if it is true in every truth-value assignment. This works because classical logic has a two-valued semantics — every proposition is either true or false.

Intuitionistic logic cannot be captured by a finite set of truth values. This is not merely a technical inconvenience. It reflects the BHK interpretation: whether a proposition is provable depends not just on whether it "is true" but on what information is currently available, what constructions have been performed, and what further information might become available in the future.

The right semantics for intuitionistic logic is *Kripke semantics*, introduced by Saul Kripke in 1965. Kripke models capture the idea of "stages of knowledge" — partial states of information that can be extended — and the forcing relation captures what is provable at each stage.

## Kripke Frames and Models

**Definition.** A *Kripke frame* is a pair $(W, \leq)$ where $W$ is a non-empty set of *worlds* (stages of knowledge) and $\leq$ is a partial order on $W$ (the *accessibility* or *extension* relation).

Intuitively: elements of $W$ are states of information. $w \leq v$ means "state $v$ has at least as much information as state $w$" — it is an extension of $w$. The partial order captures the idea that information can accumulate but not be lost.

**Definition.** A *Kripke model* is a triple $(W, \leq, V)$ where $(W, \leq)$ is a Kripke frame and $V : \text{Prop} \times W \to \{0, 1\}$ is a *valuation* satisfying the *monotonicity condition*:

$$\text{If } V(p, w) = 1 \text{ and } w \leq v, \text{ then } V(p, v) = 1.$$

Monotonicity says: if an atomic proposition is true at a world $w$, it remains true at any more informed world $v \geq w$. Information cannot be retracted.

## The Forcing Relation

The *forcing relation* $w \Vdash \varphi$ ("world $w$ forces proposition $\varphi$" or "$\varphi$ is provable at stage $w$") is defined recursively:

- $w \Vdash p$ iff $V(p, w) = 1$ (atomic propositions).
- $w \Vdash \top$ always.
- $w \Vdash \bot$ never.
- $w \Vdash A \wedge B$ iff $w \Vdash A$ and $w \Vdash B$.
- $w \Vdash A \vee B$ iff $w \Vdash A$ or $w \Vdash B$.
- $w \Vdash A \to B$ iff for all $v \geq w$: if $v \Vdash A$, then $v \Vdash B$.
- $w \Vdash \neg A$ iff for all $v \geq w$: $v \not\Vdash A$.

The clauses for $\wedge$ and $\vee$ are as expected. The clause for $\to$ is the crucial one: $A \to B$ is forced at $w$ if, at every future world $v \geq w$ where $A$ is provable, $B$ is also provable. This captures the BHK clause: a proof of $A \to B$ is a method that works not just now but in any future state of knowledge.

Negation follows: $\neg A$ (i.e., $A \to \bot$) is forced at $w$ if $A$ is not forced at any future world. This means $A$ is permanently refuted — no future information will make $A$ provable.

**Monotonicity Lemma.** If $w \Vdash A$ and $w \leq v$, then $v \Vdash A$. (Provability is monotone: what is provable now remains provable with more information.)

*Proof.* By induction on the structure of $A$. The base case holds by the monotonicity condition on $V$. The conjunction case is immediate. The implication case: if $w \Vdash A \to B$ and $w \leq v$, then for any $u \geq v \geq w$, if $u \Vdash A$, then $u \Vdash B$ (using $w \Vdash A \to B$ and $u \geq w$). So $v \Vdash A \to B$. $\square$

## Soundness and Completeness

**Definition.** A formula $\varphi$ is *Kripke-valid* if $w \Vdash \varphi$ for all Kripke models $(W, \leq, V)$ and all worlds $w \in W$.

**Theorem (Soundness).** Every IPC theorem is Kripke-valid: if $\vdash_\text{IPC} \varphi$, then $\varphi$ is Kripke-valid.

*Proof.* By induction on the derivation of $\varphi$ in IPC. Each rule of IPC corresponds to a valid property of the forcing relation. For example, $\to$I: if assuming $A$ is forced, then $B$ is forced, then at every world where this holds, $A \to B$ is forced. $\square$

**Theorem (Completeness).** Every Kripke-valid formula is an IPC theorem: if $\varphi$ is Kripke-valid, then $\vdash_\text{IPC} \varphi$.

The completeness proof is more involved. One approach: build a *canonical Kripke model* whose worlds are consistent, prime IPC theories (maximal consistent sets with the disjunction property). If $\varphi$ is not provable in IPC, then there is a theory not containing $\varphi$, which gives a world where $\varphi$ fails. Hence $\varphi$ is not Kripke-valid.

Together: **IPC is sound and complete for Kripke semantics.**

## Counterexamples to Classical Laws

Kripke models provide *explicit counterexamples* showing that classical principles fail intuitionistically.

**Counterexample to LEM:** Let $W = \{w_0, w_1\}$ with $w_0 \leq w_1$. Let $V(p, w_0) = 0$ and $V(p, w_1) = 1$.

At world $w_0$: 
- $w_0 \not\Vdash p$ (since $V(p, w_0) = 0$).
- $w_0 \not\Vdash \neg p$: since $w_1 \geq w_0$ and $w_1 \Vdash p$, we cannot have $\neg p$ forced at $w_0$.
- Therefore $w_0 \not\Vdash p \vee \neg p$ (neither disjunct is forced, and the forcing clause for $\vee$ requires one of them to be forced).

This Kripke model falsifies LEM for the proposition $p$. The model represents a state of knowledge where we don't yet know whether $p$ holds, but where a future state ($w_1$) will establish $p$. At the present state $w_0$, we cannot commit to $p$ (not yet proved) or $\neg p$ (it's about to be refuted).

**Counterexample to DNE:** In the same model: $w_0 \Vdash \neg\neg p$? Let's check. For $\neg\neg p$ at $w_0$: for all $v \geq w_0$ (i.e., $v \in \{w_0, w_1\}$), $v \not\Vdash \neg p$. At $w_0$: $\neg p$ is not forced (as shown above). At $w_1$: since $w_1 \Vdash p$ and there's nothing above $w_1$, $\neg p$ is not forced at $w_1$. So $w_0 \Vdash \neg\neg p$. But $w_0 \not\Vdash p$. So $w_0 \Vdash \neg\neg p$ but $w_0 \not\Vdash p$. DNE fails.

**Counterexample to Peirce's Law:** For $((A \to B) \to A) \to A$. Take $W = \{w_0, w_1, w_2\}$ with $w_0 \leq w_1$, $w_0 \leq w_2$, and $w_1, w_2$ incomparable. Let $V(A, w_1) = 1$, $V(A, w_2) = 0$, $V(B, \cdot) = 0$ everywhere.

At $w_0$: $A \to B$ fails at $w_1$ (since $w_1 \Vdash A$ but $w_1 \not\Vdash B$), so $w_0 \not\Vdash A \to B$. Also $w_0 \Vdash (A \to B) \to A$ (vacuously, since the antecedent $A \to B$ fails at all future worlds reachable from $w_0$ where it might be forced). So $w_0 \Vdash (A \to B) \to A$. But $w_0 \not\Vdash A$. So $w_0 \not\Vdash ((A \to B) \to A) \to A$.

This counterexample requires care but is correct. Peirce's law fails in intuitionistic logic.

## Topological Semantics

Kripke semantics has a more geometric counterpart: *topological semantics* (also called the *open set* or *Heyting algebra* semantics), which connects intuitionistic logic to point-set topology.

Let $X$ be a topological space. Define a valuation $V(p) \subseteq X$ as an open set for each atomic proposition $p$. The forcing relation becomes set-theoretic:

- $V(A \wedge B) = V(A) \cap V(B)$ (open intersection)
- $V(A \vee B) = V(A) \cup V(B)$ (open union)
- $V(A \to B) = \text{Int}(V(A)^c \cup V(B))$ (interior of the implication)
- $V(\neg A) = \text{Int}(V(A)^c)$ (interior of the complement)
- $V(\bot) = \emptyset$, $V(\top) = X$

A formula $\varphi$ is valid if $V(\varphi) = X$ for every such valuation.

The open sets of a topological space form a *Heyting algebra* — a distributive lattice with a relative pseudo-complement operation corresponding to intuitionistic implication. IPC is complete with respect to the class of all Heyting algebras.

This topological interpretation connects intuitionistic logic to sheaf theory, topos theory, and ultimately to HoTT. The *classifying topos* of a Grothendieck topos has internal logic that is intuitionistic. A sheaf model of type theory corresponds to a forcing interpretation of constructive mathematics over a site. The connection goes deep.

## The Sheaf and HoTT Connection

In HoTT, the semantics of types in an $\infty$-topos generalizes the topological semantics of IPC. Where classical logic corresponds to the two-element Boolean algebra $\{0, 1\}$, intuitionistic logic corresponds to any Heyting algebra. And HoTT's homotopy interpretation corresponds to the $\infty$-categorical analog: the algebra of $(\infty, 1)$-categories, where the "truth values" are not just $\{0, 1\}$ but the full $\infty$-groupoid of paths.

In this sense, Kripke semantics is the shadow of a much richer structure: the forcing model where "worlds" are not just points but points in an $\infty$-dimensional space, and "accessibility" is not just a relation but a space of paths. The monotonicity condition becomes a condition that forcing is preserved along paths, which is exactly the transport operation in HoTT.

Understanding Kripke semantics is thus a first step toward understanding the $\infty$-topos semantics of HoTT — the cleanest available mathematical framework for the homotopy interpretation.

# 1.1 Judgments and Contexts

## What Is a Judgment?

When a mathematician proves a theorem, they're doing more than just writing down a sequence of symbols. They're making a *judgment*: asserting that something is true based on what they know.

Proof theory makes this explicit. A **judgment** is a formal declaration of a fact. The most basic judgment in logic is:

$$\Gamma \vdash \varphi$$

Read: "In context $\Gamma$, proposition $\varphi$ holds" or "From hypotheses $\Gamma$, $\varphi$ is derivable."

Let's unpack this.

## Contexts

A *context* $\Gamma$ is a finite list of propositions that we're treating as hypotheses — things we're temporarily assuming. Concretely:

$$\Gamma = \varphi_1, \varphi_2, \ldots, \varphi_n$$

means we're assuming all of $\varphi_1, \ldots, \varphi_n$. The order usually doesn't matter, but convention is to list them left to right.

The empty context is written $\emptyset$ or just left blank. The judgment $\vdash \varphi$ (with empty context) means $\varphi$ is derivable from no hypotheses — it's a *theorem*.

**Examples:**
- $P, Q \vdash P \wedge Q$: from hypotheses $P$ and $Q$, we can derive $P \wedge Q$.
- $\vdash P \to P$: from no hypotheses, we can derive $P \to P$ (this is a tautology).
- $P \vdash Q$: from hypothesis $P$, $Q$ is derivable. But this is not valid in general — $Q$ might have nothing to do with $P$.

## Judgments in Type Theory

In type theory, the basic judgments look different but serve the same purpose. The four fundamental judgments of Martin-Löf Type Theory (which we'll study in detail in Chapter 11) are:

1. $\Gamma \vdash A\ \mathsf{type}$ — "$A$ is a type in context $\Gamma$"
2. $\Gamma \vdash a : A$ — "$a$ is a term of type $A$ in context $\Gamma$"
3. $\Gamma \vdash A = B\ \mathsf{type}$ — "$A$ and $B$ are equal types in context $\Gamma$"
4. $\Gamma \vdash a = b : A$ — "$a$ and $b$ are equal terms of type $A$ in context $\Gamma$"

In logic, contexts contain propositions. In type theory, contexts contain *typed variables*: $\Gamma = x_1 : A_1, x_2 : A_2, \ldots$ means "variable $x_1$ has type $A_1$, variable $x_2$ has type $A_2$, ..."

The Curry-Howard correspondence (Chapter 8) bridges these: propositions correspond to types, and proofs correspond to terms.

## The Deducibility Relation

When we write $\Gamma \vdash \varphi$, we're asserting that there *exists* a proof of $\varphi$ from $\Gamma$ — a derivation in our formal system. The symbol $\vdash$ is called the *turnstile*.

This is a syntactic relation: $\Gamma \vdash \varphi$ means "there is a formal derivation," not just "it is semantically true." The distinction matters:

- **Syntactic entailment** ($\Gamma \vdash \varphi$): $\varphi$ is provable from $\Gamma$ using the formal rules.
- **Semantic entailment** ($\Gamma \models \varphi$): every valuation making all of $\Gamma$ true also makes $\varphi$ true.

The **soundness theorem** says: if $\Gamma \vdash \varphi$ then $\Gamma \models \varphi$ (provability implies truth).

The **completeness theorem** says: if $\Gamma \models \varphi$ then $\Gamma \vdash \varphi$ (truth implies provability).

Together, they say the formal system exactly captures the semantics. Soundness is usually easier to prove; completeness is deeper and often requires non-constructive methods (like the Axiom of Choice, as in Gödel's completeness theorem for first-order logic).

## Structural Rules

Before introducing rules for specific connectives, we have *structural rules* — rules that govern the basic properties of the deducibility relation $\vdash$.

**Identity (Hypothesis):*
$$\frac{}{\Gamma, \varphi \vdash \varphi}$$

Read: From a context containing $\varphi$, you can immediately derive $\varphi$. This is the most basic rule: if you're assuming $\varphi$, you can use $\varphi$.

**Weakening:**
$$\frac{\Gamma \vdash \varphi}{\Gamma, \psi \vdash \varphi}$$

If you can prove $\varphi$ from $\Gamma$, you can also prove it from $\Gamma$ with an extra, unused hypothesis $\psi$. Adding irrelevant hypotheses doesn't break proofs.

**Contraction:**
$$\frac{\Gamma, \varphi, \varphi \vdash \psi}{\Gamma, \varphi \vdash \psi}$$

If you can prove $\psi$ using hypothesis $\varphi$ twice, you can do it with it once. This says we can "reuse" hypotheses freely — in classical logic, using a hypothesis once or many times is the same.

**Exchange:**
$$\frac{\Gamma, \varphi, \psi, \Delta \vdash \chi}{\Gamma, \psi, \varphi, \Delta \vdash \chi}$$

The order of hypotheses in the context doesn't matter.

These three structural rules (weakening, contraction, exchange) are often taken for granted in classical logic. But they can be restricted:
- *Affine logic* drops contraction (each hypothesis can be used at most once).
- *Linear logic* (Girard) drops both weakening and contraction (each hypothesis must be used exactly once).
- *Ordered logic* drops exchange (hypotheses are ordered and must be used in order).

These resource-sensitive logics have applications in computer science (linear types for memory safety, session types for concurrency).

## The Cut Rule

One more structural rule deserves special mention:

**Cut:**
$$\frac{\Gamma \vdash \varphi \qquad \Delta, \varphi \vdash \psi}{\Gamma, \Delta \vdash \psi}$$

If you can prove $\varphi$ from $\Gamma$, and can prove $\psi$ from $\Delta$ plus $\varphi$, then you can prove $\psi$ from $\Gamma, \Delta$ (substituting the proof of $\varphi$ into the proof of $\psi$).

The cut rule formalized the intuitive idea of "using a lemma." To prove a theorem, we often prove an intermediate result (a lemma) and then use it. Cut says: you don't need to keep the intermediate result around — you can "cut it out" by substituting the proof of the lemma into the proof of the theorem.

The profound question: is the cut rule *necessary*? If you remove cut, can you still prove everything provable with cut?

**Gentzen's Hauptsatz (Cut Elimination):** In sequent calculus, the cut rule is eliminable — every proof using cut can be transformed into a proof without cut.

This is not an obvious fact. It says that "lemmas" are a convenience, not a necessity: in principle, every argument can be made without any intermediate results. But cut-free proofs can be *exponentially longer*. (This exponential blowup is related to the complexity of proof verification in NP.)

We'll prove this in Section 4.

## Derivations as Trees

A derivation of $\Gamma \vdash \varphi$ is a finite *tree* whose:
- Root is the sequent $\Gamma \vdash \varphi$ (the conclusion).
- Leaves are axioms (instances of the Identity rule).
- Internal nodes are applications of inference rules.

Each inference rule takes the judgments at the premises (the children in the tree) and derives the judgment at the conclusion (the parent in the tree).

**Example derivation of $P, Q \vdash P \wedge Q$:**
$$\frac{\dfrac{}{P, Q \vdash P} \quad \dfrac{}{P, Q \vdash Q}}{P, Q \vdash P \wedge Q} \wedge\text{I}$$

The two leaves are instances of the Identity rule (using hypothesis $P$ and hypothesis $Q$ respectively). The root is derived by the conjunction-introduction rule $\wedge$I.

This tree is the formal proof. In natural deduction, derivations are exactly such trees. The rules we'll study in the next section tell us what trees are valid.

## Hypothetical Reasoning

A key feature of formal proof systems is *hypothetical reasoning* — the ability to temporarily assume a proposition and derive consequences.

In natural deduction, this corresponds to the $\to$-introduction rule (implication introduction). To prove $\varphi \to \psi$, you:
1. Assume $\varphi$ (add it to the context).
2. Under this assumption, derive $\psi$.
3. Discharge the assumption: the result is a proof of $\varphi \to \psi$ from the *original* context, not the extended one.

Formally: if $\Gamma, \varphi \vdash \psi$, then $\Gamma \vdash \varphi \to \psi$.

Hypothetical reasoning is what makes implication meaningful: to prove "if $P$ then $Q$," you assume $P$ and derive $Q$. When you discharge the assumption, you've proved $P \to Q$ without $P$ in your context.

In type theory, this becomes $\lambda$-abstraction: a proof of $\varphi \to \psi$ under hypothesis $\varphi$ is a function that takes a proof of $\varphi$ and produces a proof of $\psi$.

## The Subformula Property

Once we have the formal system, we can ask: what does a proof "look like"? Is there a restricted class of proofs that suffices?

The *subformula property* says: in a normal form proof (a proof with no detours), every formula that appears in the derivation tree is a subformula of the conclusion or one of the hypotheses.

This is a dramatic restriction. It means proofs in normal form can't "go outside" the vocabulary of what you're proving. You can't prove something about arithmetic using facts about real analysis, if those real analysis facts don't appear as subformulas of your conclusion.

The subformula property is what makes cut elimination so powerful: it gives a syntactic characterization of what "minimal proofs" look like, and this has direct consequences for consistency and decidability.

We'll develop this carefully in the normalization section.

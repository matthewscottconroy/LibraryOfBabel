# Judgments and Derivations

## The Gap Between Knowing and Proving

There is a gap between knowing something to be true and having a proof of it. Most of us believe that there are infinitely many prime numbers, and most of us are right. But knowing this and having a proof of it are different activities. A proof is a structured demonstration that compels conviction not through authority or appeal to intuition but through explicit, checkable steps.

Proof theory makes this gap precise by insisting that we say exactly what a proof *is* — not just what it establishes. A proof is a formal derivation in a formal system: a finite tree of inference steps, each licensed by one of the system's rules, with the conclusion of each rule matching the premises of the steps that use it. The rules are explicit. The derivation is explicit. Anyone can check it.

This is the starting point.

## What Is a Judgment?

A **judgment** is a formal declaration of a fact. In logic, the basic judgment has the form:

$$\Gamma \vdash A$$

Read: "From context $\Gamma$, proposition $A$ is provable." The symbol $\vdash$ is called the *turnstile*, and you should read it as "proves" or "yields."

The judgment has two parts. The *context* $\Gamma$ is a finite list of propositions that we are treating as hypotheses — temporary assumptions that we are permitted to use. The *conclusion* $A$ is the proposition we claim to be derivable from those assumptions.

Some examples to build intuition:
- $P, Q \vdash P \wedge Q$: from hypotheses $P$ and $Q$, the conjunction $P \wedge Q$ is provable.
- $\vdash P \to P$: from *no* hypotheses, $P$ implies $P$ is provable. This is a theorem.
- $P \to Q, P \vdash Q$: from $P \to Q$ and $P$, we can derive $Q$. This is modus ponens.

The empty context — no hypotheses — is written $\emptyset$ or simply left blank. A judgment $\vdash A$ with empty context asserts that $A$ is a theorem of the formal system: provable from scratch.

## The Syntax-Semantics Distinction

There are two kinds of "truth" in logic, and conflating them is a perpetual source of confusion.

**Syntactic provability** ($\Gamma \vdash A$): there exists a formal derivation of $A$ from $\Gamma$ using the rules of our formal system. This is a purely symbolic claim — it says something about the manipulation of symbols, not about any mathematical reality.

**Semantic truth** ($\Gamma \models A$): in every interpretation where all formulas in $\Gamma$ are true, $A$ is also true. This is a claim about meaning — about what happens when we assign truth values to atomic propositions.

These are not the same notion, but they are related by two fundamental theorems:

The **Soundness Theorem** says: if $\Gamma \vdash A$ then $\Gamma \models A$. Provability implies truth. If our formal system derives $A$ from $\Gamma$, then any interpretation making $\Gamma$ true also makes $A$ true. Soundness says the rules are not lying to us — they only derive genuinely valid consequences.

The **Completeness Theorem** (Gödel, 1930) says: if $\Gamma \models A$ then $\Gamma \vdash A$. Truth implies provability. If $A$ is true in every interpretation making $\Gamma$ true, then our formal system can derive $A$ from $\Gamma$. Completeness says the rules are powerful enough — they can prove everything that is semantically valid.

Together, soundness and completeness say that our formal system *exactly* captures semantic validity: the things it can prove are exactly the things that are semantically true. For propositional and first-order classical logic, both theorems hold. For intuitionistic logic, soundness holds and completeness holds relative to Kripke semantics. For full higher-order logic, completeness fails.

## Formal Systems as Inductive Definitions

A formal system is, at its core, an *inductive definition* of a set of valid derivations. This is the right way to think about it.

An inductive definition specifies a set by giving:
1. A collection of *base cases* — elements that are in the set by definition.
2. A collection of *inductive cases* — rules that produce new elements from existing ones.

For a proof system, the base cases are the axioms: judgments that are valid without any premises. The inductive cases are the inference rules: if certain judgments are valid (the premises), then a new judgment is valid (the conclusion).

A **derivation** of $\Gamma \vdash A$ is a finite tree whose nodes are judgments, whose root is $\Gamma \vdash A$, whose leaves are axioms, and whose internal nodes are applications of inference rules. The root is the conclusion, and each internal node is licensed by a rule whose premises are its children.

This makes derivations into well-defined mathematical objects — not sequences of symbols or informal arguments, but finite trees with a precise recursive structure. We can reason about derivations using mathematical induction on their structure. We can ask: what is the depth of this derivation? How many times is a given rule used? What formulas appear in it? These are questions about mathematical objects, and they have mathematical answers.

## Derivation Trees: Notation and Examples

We write derivation trees with the conclusion at the bottom, premises above, and a horizontal rule separating them. Each rule is labeled on the right.

The most basic derivation: from the hypothesis $P$, derive $P$. This is the *Identity* (or *Hypothesis*) rule:

$$\frac{}{P \vdash P} \quad \text{(Hyp)}$$

No premises; this holds directly because $P$ is in the context.

A slightly richer example. From $P$ and $Q$, derive $P \wedge Q$:

$$\frac{\dfrac{}{P, Q \vdash P} \quad \dfrac{}{P, Q \vdash Q}}{P, Q \vdash P \wedge Q} \quad (\wedge\text{I})$$

Two hypothesis applications (leaves), combined by the conjunction introduction rule. This is a complete, formal proof of $P, Q \vdash P \wedge Q$.

A theorem — derivable from no hypotheses. From $A$ and $B \to C$ and $A \to B$, we can prove $C$. Let's build the derivation:

$$\frac{
  \dfrac{\dfrac{}{A, A \to B, B \to C \vdash A \to B} \quad \dfrac{}{A, A\to B, B \to C \vdash A}}{A, A \to B, B \to C \vdash B} \;\; (\to\text{E}) \quad \dfrac{}{A, A\to B, B\to C \vdash B\to C}
}{A, A\to B, B\to C \vdash C} \;\; (\to\text{E})$$

Each leaf is a hypothesis application; each internal node is a rule application. The tree is the proof.

## Structural Rules

Before any logical rules, there are *structural rules* that govern the basic behavior of contexts. These rules are usually invisible in informal mathematics — we apply them without noticing. Making them explicit is one of proof theory's contributions.

**Identity (Hypothesis):**
$$\frac{}{\Gamma, A \vdash A}$$
If $A$ is among your hypotheses, you can immediately conclude $A$. This is the base case of derivation.

**Weakening:**
$$\frac{\Gamma \vdash A}{\Gamma, B \vdash A}$$
If you can prove $A$ from $\Gamma$, you can also prove $A$ from $\Gamma$ with an extra irrelevant hypothesis $B$ added. Unused hypotheses don't hurt.

**Contraction:**
$$\frac{\Gamma, A, A \vdash B}{\Gamma, A \vdash B}$$
If you can prove $B$ using hypothesis $A$ twice, you can prove it using $A$ once — hypotheses can be reused for free.

**Exchange:**
$$\frac{\Gamma, A, B, \Delta \vdash C}{\Gamma, B, A, \Delta \vdash C}$$
The order of hypotheses in the context doesn't matter.

These seem trivial. They are not. Restricting these rules gives rise to *substructural logics* — systems where resources matter:

- *Linear logic* (Girard, 1987) drops both weakening and contraction: each hypothesis must be used exactly once. This models resources that are consumed by use — memory in a program, energy in a physical process, money in a transaction.
- *Affine logic* drops only contraction: hypotheses can be discarded but not duplicated.
- *Ordered logic* drops exchange: hypotheses must be used in the order they were introduced.

These logics have real applications. Rust's ownership system is an implementation of affine type theory. Session types for concurrent protocols use ordered or linear structure. The structural rules that seem like bureaucratic trivialities are actually load-bearing assumptions with deep computational significance.

## The Cut Rule

One more structural rule stands apart:

**Cut:**
$$\frac{\Gamma \vdash A \quad \Delta, A \vdash B}{\Gamma, \Delta \vdash B}$$

Read: if you can prove $A$ from $\Gamma$, and can prove $B$ from $\Delta$ together with $A$, then you can prove $B$ from $\Gamma$ and $\Delta$ together — eliminating the intermediate $A$.

This formalizes "using a lemma." To prove a theorem, we often prove an intermediate result and then use it. Cut says: the proof of the lemma can be substituted into the proof of the theorem, eliminating the lemma as a visible intermediate step.

The cut rule is clearly sound: if you have a proof of $A$ and a proof of $B$ from $A$, composing them gives a proof of $B$. The deep question is: is it *necessary*? Can you prove everything provable-with-cut without using cut at all?

Gentzen's answer — yes, always — is the *Hauptsatz* (main theorem), which we prove in Section 4. Cut elimination is one of the great results of 20th-century logic. It says that the modular style of proof — prove a lemma, use it elsewhere — is a convenience that can always be eliminated, at potential cost in proof length but no cost in logical power.

## Multiple Judgment Forms

In type theory, the basic judgment $\Gamma \vdash A$ bifurcates into several related forms. Martin-Löf Type Theory uses four fundamental judgments:

1. $\Gamma \vdash A \;\mathsf{type}$: "$A$ is a well-formed type in context $\Gamma$."
2. $\Gamma \vdash a : A$: "$a$ is a term of type $A$ in context $\Gamma$."
3. $\Gamma \vdash A = B \;\mathsf{type}$: "$A$ and $B$ are definitionally equal types."
4. $\Gamma \vdash a = b : A$: "$a$ and $b$ are definitionally equal terms of type $A$."

These four judgments are not independent — they are related by rules that say, for instance, if $\Gamma \vdash a : A$ and $\Gamma \vdash A = B \;\mathsf{type}$, then $\Gamma \vdash a : B$. The whole framework of dependent type theory is built from interactions among these judgment forms.

In HoTT specifically, the fourth judgment — definitional equality — is the one that gets enriched. Two terms are definitionally equal if one reduces to the other by computation (beta and eta reduction). But there is a separate, richer notion: *propositional equality*, the identity type $a =_A b$. In classical type theory, these two notions are kept strictly separate. In HoTT, their relationship is the central subject.

Understanding this requires understanding the basic judgment $\Gamma \vdash a : A$ — which is just the Curry-Howard reading of the logical judgment $\Gamma \vdash A$, with proof $a$ made explicit. We develop this correspondence in Chapter 6. For now, the important point is that the judgment structure of this section is the foundation that everything else builds on.

## Formal Systems Compute

We have been describing formal systems as inductive definitions of valid derivations. There is another way to see them: as specifications of computation.

An inference rule is a *transformation*: given proofs of the premises, produce a proof of the conclusion. A derivation is a *composition* of such transformations. Normalization — reducing a derivation to normal form — is applying transformations until no more apply. This is computation.

The identification of proof normalization with computation is the central insight of the Curry-Howard correspondence. It is not just an analogy. In the Simply Typed Lambda Calculus (Chapter 7), programs *are* proofs, and executing a program *is* normalizing a derivation. Type checking a program *is* checking that a derivation is valid.

We will develop this correspondence fully. But even here, at the level of pure proof theory, we can see its shadow: formal systems are not just about truth, but about transformation. The question "what is a proof of $A$?" is not separable from the question "what does it compute?"

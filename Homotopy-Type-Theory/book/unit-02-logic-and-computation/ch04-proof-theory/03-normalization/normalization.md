# Normalization

## The Proof That Did Too Much Work

Here is a proof of $A$ from the hypothesis $A \wedge B$:

1. From $A \wedge B$, derive $A$ (by $\wedge$E$_1$).

Done. One step. Now here is another proof of the same conclusion from the same hypothesis:

1. From $A \wedge B$, derive $A$ (by $\wedge$E$_1$).
2. From $A \wedge B$, derive $B$ (by $\wedge$E$_2$).
3. From $A$ and $B$, derive $A \wedge B$ (by $\wedge$I).
4. From $A \wedge B$, derive $A$ (by $\wedge$E$_1$).

Also a valid proof. But it does three steps of unnecessary work: it extracts $B$, pairs $A$ and $B$ back together, then extracts $A$ again. This is a *detour* — the proof introduces the connective $\wedge$ at step 3 only to immediately eliminate it at step 4. The detour contributes nothing to the conclusion.

Normalization is the process of removing detours. The Normalization Theorem says that every proof can be systematically simplified into one with no detours — a *normal form* — by a finite process of detour elimination. This is not obvious. Removing one detour might create another. The process might loop. The theorem says it never does.

This matters for two reasons: logical and computational. Logically, normal form proofs have the subformula property, which implies consistency. Computationally, normalization *is* computation — every step of simplification corresponds to a step of program execution.

## Redexes: The Anatomy of a Detour

A **redex** (reducible expression) is a pair of adjacent inference rules where an introduction is immediately followed by the corresponding elimination. Removing a redex — the corresponding **reduction** — produces a simpler proof.

Each connective has its own reduction.

**Conjunction reduction** ($\wedge$-redex): introduce $A \wedge B$ from proofs of $A$ and $B$, then immediately project out $A$.

$$\frac{\Gamma \vdash A \quad \Gamma \vdash B}{\Gamma \vdash A \wedge B} \xrightarrow{\wedge\text{E}_1} \Gamma \vdash A$$

Reduces to: just use the proof of $A$ directly. The detour is deleted.

**Implication reduction** ($\to$-redex): prove $A \to B$ by assuming $A$ and deriving $B$, then apply the result to a proof of $A$.

$$\frac{\dfrac{[A]^u \;\cdots\; B}{A \to B}\quad A}{B}$$

Reduces to: substitute the actual proof of $A$ for every use of the hypothesis $[A]^u$ in the derivation of $B$. The assumption is replaced by its witness; the implication introduction-elimination pair disappears.

This reduction is *substitution*. This is not a coincidence. Under the Curry-Howard correspondence, it is exactly the $\beta$-reduction of the $\lambda$-calculus: $(\lambda x.\, t)\, a \to_\beta t[a/x]$.

**Disjunction reduction** ($\vee$-redex): prove $A \vee B$ by left injection, then case-split on $A \vee B$.

$$\frac{\dfrac{A}{A \vee B}\quad [A]^u \;\cdots\; C \quad [B]^v \;\cdots\; C}{C}$$

Reduces to: since we know we're in the left case, use the derivation of $C$ from $A$ directly, substituting our proof of $A$.

**Universal quantifier reduction** ($\forall$-redex): prove $\forall x, B(x)$ by proving $B(a)$ for fresh $a$, then instantiate at $t$.

$$\frac{\dfrac{\;\cdots B(a)\;}{\forall x, B(x)}}{\; B(t) \;}$$

Reduces to: substitute $t$ for $a$ throughout the derivation of $B(a)$.

## Eta Reductions: The Other Kind of Redundancy

There is a dual kind of redundancy. An **eta-redex** occurs when we *eliminate* a connective and then immediately *re-introduce* it. The most common example:

**Implication eta-reduction**: if $f : A \to B$ is a function, then $\lambda x.\, f\, x$ is a function that takes $x$ and applies $f$ to it. But this is just $f$ — the lambda wraps $f$ in a redundant function shell.

$$\lambda x.\, f\, x \;\to_\eta\; f$$

Eta-reductions enforce *extensionality*: two functions that agree on all inputs are equal. In type theory with eta-rules, $f$ and $\lambda x.\, f\, x$ are not just logically equivalent but *definitionally equal*. This has important consequences in HoTT, where function extensionality (funext) states that pointwise equal functions are propositionally equal — a non-trivial theorem that becomes the defining feature of function types from a homotopy perspective.

## The Normalization Theorem

**Definition.** A derivation is in *normal form* if it contains no $\beta$-redexes.

**Theorem (Weak Normalization, Prawitz 1965).** Every derivation in intuitionistic natural deduction has at least one reduction sequence that terminates in a normal form.

**Theorem (Strong Normalization).** Every reduction sequence from any derivation in intuitionistic natural deduction terminates in a normal form.

Weak normalization says: there exists a way to normalize. Strong normalization says: every way to normalize terminates. Strong normalization is the more powerful and more useful result — it says the simplification process cannot loop regardless of the order in which we choose to eliminate redexes.

This is not obvious. Here is why one might worry: eliminating the $\to$-redex for $(\lambda x.\, t)\, a$ substitutes $a$ into $t$. If $t$ mentions $x$ many times, then $a$ gets duplicated many times in the result. If $a$ contains other redexes, we have created new redexes by duplication. Could this process diverge?

For the *untyped* $\lambda$-calculus, the answer is yes: $(\lambda x.\, x\, x)(\lambda x.\, x\, x) \to (\lambda x.\, x\, x)(\lambda x.\, x\, x)$, the Omega combinator. It reduces to itself and never terminates.

Types prevent this. The typing rules do not allow self-application (a term of type $A \to B$ cannot be applied to itself unless $A = A \to B$, which is impossible in a simple type system with no recursive types). The structure imposed by types rules out the patterns that cause non-termination.

## Proof of Strong Normalization: Tait's Method

The standard proof of strong normalization uses a technique called *logical relations* or *reducibility candidates*, due to William Tait (1967).

The idea: define a predicate $\text{Red}(A)$ — "reducibility at type $A$" — that captures the terms that compute well, and prove that all well-typed terms belong to their reducibility predicate. The definition is *logical* in that it follows the structure of types.

**Definition of reducibility**:
- $\text{Red}(\alpha)$ (for base types $\alpha$): the set of strongly normalizing terms of type $\alpha$.
- $\text{Red}(A \to B)$: the set of terms $f$ of type $A \to B$ such that for every $a \in \text{Red}(A)$, we have $f\, a \in \text{Red}(B)$.

**Key properties** (CR1–CR3):
- CR1: every term in $\text{Red}(A)$ is strongly normalizing.
- CR2: if $t \in \text{Red}(A)$ and $t \to t'$, then $t' \in \text{Red}(A)$.
- CR3: if $t$ is in normal form and $t'$ is in $\text{Red}(A)$ whenever $t \to^* t'$, then $t \in \text{Red}(A)$.

One proves by induction on the structure of $A$ that $\text{Red}(A)$ satisfies CR1–CR3 for every type $A$.

**Main Lemma**: every well-typed term belongs to $\text{Red}$ at its type.

*Proof*: by induction on the typing derivation. The critical case is lambda abstraction: if $\Gamma, x : A \vdash t : B$ and for all $a \in \text{Red}(A)$ the substituted term $t[a/x] \in \text{Red}(B)$, then $\lambda x.\, t \in \text{Red}(A \to B)$.

**Corollary (Strong Normalization)**: every well-typed term is strongly normalizing (by CR1).

The logical relations method is general and powerful. Variants of it prove normalization for System F (Girard 1971), for MLTT (Martin-Löf 1975), and — more recently — for fragments of HoTT (Coquand and others). Understanding this proof method is essential for research in type theory.

## The Subformula Property

**Theorem (Subformula Property).** In a normal form derivation, every formula appearing in the derivation is a subformula of the conclusion or of one of the undischarged hypotheses.

A *subformula* of $\varphi$ is $\varphi$ itself or any component of it: $A$ and $B$ are subformulas of $A \wedge B$; $A$, $B(x)$, and every instance $B(t)$ are subformulas of $\forall x, B(x)$.

Why is this true? In a normal form derivation there are no $\beta$-redexes. The proof has a clean "diamond" structure:
- An *elimination phase*: starting from undischarged hypotheses, apply elimination rules, breaking down compound formulas into smaller ones.
- An *introduction phase*: starting from small formulas, apply introduction rules, building up toward the conclusion.

In the elimination phase, every formula is a subformula of some hypothesis. In the introduction phase, every formula is a subformula of the conclusion. No formula is ever "created from outside" — the proof is analytic. It works only with the vocabulary already present in its statement.

This is remarkable. It means that to prove $A \to B$, you never need to introduce an auxiliary proposition $C$ that appears neither in $A$ nor in $B$. In classical mathematics, lemmas introduce such auxiliary propositions constantly. But in *cut-free* (normal form) proofs, this is impossible. Lemmas are a *convenience* — a way of making proofs shorter and more readable — but not a logical *necessity*.

## Consistency as a Corollary

**Theorem (Consistency).** The system of intuitionistic natural deduction is consistent: $\not\vdash \bot$ (bottom is not provable from no hypotheses).

*Proof*. Suppose $\vdash \bot$. By the Normalization Theorem, this proof has a normal form. By the Subformula Property, every formula in the normal form derivation is a subformula of $\bot$. But $\bot$ has no subformulas — it is atomic. And the only rule that could discharge $\bot$ without assumptions would have to have a leaf producing $\bot$ without hypotheses. But there is no introduction rule for $\bot$ (by definition — $\bot$ is the proposition that has no proof). Contradiction. $\square$

This proof is notable for what it does not use. It does not appeal to the semantics of logic. It does not invoke a model. It is purely syntactic: a fact about the structure of derivation trees. Consistency follows from the internal architecture of the proof system.

## Normalization and Type Checking

Strong normalization has an immediate computational corollary: *type checking terminates*.

To check whether a term $t$ has type $A$ in a type theory based on natural deduction, we reduce $t$ to normal form and then check whether the normal form has the right structure. If the reduction process terminates — which it does, by strong normalization — then type checking terminates.

This is why strongly normalizing type theories make good foundations for proof assistants. Every proof that is submitted can be checked mechanically in finite time. There is no risk of the checker looping forever. And because normal forms have the subformula property, the checking process is bounded: it only examines formulas in the conclusion and hypotheses.

When we add classical axioms, things become more complicated. Call/cc (call-with-current-continuation), which gives classical logic its computational interpretation, can cause normalization to diverge in certain settings. This is why proof assistants based on classical logic (like some modes of Isabelle/HOL) use different proof-checking strategies.

## The Connection to HoTT

In HoTT, the situation is richer. The identity type $a =_A b$ is itself a type, and its elements are *proofs of equality* — paths in the homotopy interpretation. Two proofs of the same equality can themselves be unequal (connected by a homotopy), and this tower of higher equalities has no classical analog.

The computational content of HoTT — what it means to normalize a proof of equality — is captured by *path computation*: path composition, inversion, transport, and the higher groupoid laws. Normalization in this richer setting is the subject of current research, including *cubical type theory*, which provides computational rules for the univalence axiom.

The normalization theorem of this section — the clean, classical result about intuitionistic natural deduction — is the foundation. Everything that comes after is an extension of the insight that proofs have internal structure, that structure can be simplified, and that simplification is computation.

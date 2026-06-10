# 2.1 Natural Deduction: Rules for Connectives

## The Design Philosophy

Natural deduction was designed by Gerhard Gentzen with one guiding principle: the logical rules should mirror how mathematicians actually reason. Each connective ($\wedge$, $\vee$, $\to$, $\neg$, $\forall$, $\exists$) has exactly two kinds of rules:

- **Introduction rules (I):** How to *prove* a proposition with that connective as its main connective. These tell you how to "build" a proof.
- **Elimination rules (E):** How to *use* a proved proposition with that connective. These tell you what you can "extract" from a proof.

The introduction-elimination duality is the fundamental organizing principle of proof theory. When we prove $A \wedge B$, we do so by proving $A$ and $B$ separately — and the only thing we can do with a proof of $A \wedge B$ is extract the proof of $A$ or the proof of $B$. The introduction and elimination rules are inverses of each other in this sense.

We'll see this "what you can introduce, you can later eliminate" principle made precise in the normalization theorem.

## Conjunction (AND)

**Introduction rule** ($\wedge$I):
$$\frac{\Gamma \vdash A \qquad \Gamma \vdash B}{\Gamma \vdash A \wedge B}$$

To prove $A \wedge B$: prove $A$ and prove $B$.

**Elimination rules** ($\wedge$E$_1$ and $\wedge$E$_2$):
$$\frac{\Gamma \vdash A \wedge B}{\Gamma \vdash A} \qquad \frac{\Gamma \vdash A \wedge B}{\Gamma \vdash B}$$

From a proof of $A \wedge B$: extract either the proof of $A$ or the proof of $B$.

**In programming.** Under the Curry-Howard correspondence:
- $A \wedge B$ corresponds to the *product type* $A \times B$.
- A proof of $A \wedge B$ is a *pair* $(a, b)$ where $a$ proves $A$ and $b$ proves $B$.
- The elimination rules are the *projections* $\text{fst}$ and $\text{snd}$.

## Implication (IF-THEN)

**Introduction rule** ($\to$I):
$$\frac{\Gamma, A \vdash B}{\Gamma \vdash A \to B}$$

To prove $A \to B$: assume $A$ (add it to the context) and derive $B$ under that assumption. Then discharge the assumption.

This rule is sometimes written with a "hypothetical" style, using a named assumption:
$$\frac{[A]^u \quad \vdots \quad B}{A \to B} \to\text{I}^u$$

The superscript $u$ labels the assumption that is being discharged.

**Elimination rule** ($\to$E, also called *modus ponens*):
$$\frac{\Gamma \vdash A \to B \qquad \Gamma \vdash A}{\Gamma \vdash B}$$

To use a proof of $A \to B$: apply it to a proof of $A$ to get a proof of $B$.

**In programming.** $A \to B$ is the *function type*. A proof of $A \to B$ is a function from proofs of $A$ to proofs of $B$. The introduction rule is $\lambda$-abstraction; the elimination rule is function application.

## Disjunction (OR)

**Introduction rules** ($\vee$I$_1$ and $\vee$I$_2$):
$$\frac{\Gamma \vdash A}{\Gamma \vdash A \vee B} \qquad \frac{\Gamma \vdash B}{\Gamma \vdash A \vee B}$$

To prove $A \vee B$: either prove $A$ or prove $B$.

**Elimination rule** ($\vee$E, case analysis):
$$\frac{\Gamma \vdash A \vee B \qquad \Gamma, A \vdash C \qquad \Gamma, B \vdash C}{\Gamma \vdash C}$$

From a proof of $A \vee B$: to prove $C$, prove $C$ under the assumption $A$ and also prove $C$ under the assumption $B$. This is proof by cases.

**The constructive content.** The introduction rules say which disjunct holds. The elimination rule is the logic analog of a case split: you know one of two things holds, and you derive your conclusion in either case.

**In programming.** $A \vee B$ is the *sum type* (tagged union). A proof is either `Left a` (a proof $a$ of $A$) or `Right b` (a proof $b$ of $B$). The elimination rule is pattern matching.

## Negation and Bottom

**False** ($\bot$, "bottom"): a proposition that is always false — there is no proof of $\bot$.

**Elimination rule for $\bot$** (*ex falso quodlibet*, EFQ):
$$\frac{\Gamma \vdash \bot}{\Gamma \vdash A}$$

From a proof of $\bot$, you can derive anything. If you prove a contradiction, the logical system collapses to triviality. This rule says: false implies everything.

There is *no introduction rule* for $\bot$ — you can't prove $\bot$ directly. (If you could, the system would be inconsistent.)

**Negation** ($\neg A$) is defined as $A \to \bot$: negation is "implies false."

Under this definition, the rules for negation are just the rules for implication with $B = \bot$:
- To prove $\neg A$: assume $A$ and derive $\bot$ (a contradiction).
- To use $\neg A$ with a proof of $A$: apply modus ponens to get $\bot$, then use EFQ.

**In programming.** $\bot$ is the empty type (no values). $\neg A$ is the type of functions $A \to \bot$ — functions that could never be called, since there are no values of type $\bot$ to return. The EFQ rule says: if you have an element of the empty type, you can produce an element of any type (by pattern matching on the empty type, which has no cases).

## Double Negation and Classical Logic

So far, our rules are *intuitionistic* — they don't assume classical logic. In particular, we cannot prove:

**Law of Excluded Middle (LEM):** $\vdash A \vee \neg A$ for every $A$.

**Double Negation Elimination (DNE):** $\neg\neg A \vdash A$.

**Peirce's Law:** $\vdash ((A \to B) \to A) \to A$.

These three are equivalent and each gives classical logic when added to intuitionistic logic.

**To get classical logic:** Add one of the above as an axiom, or add the following rule:
$$\frac{\Gamma, \neg A \vdash \bot}{\Gamma \vdash A}$$

This says: to prove $A$, it suffices to derive a contradiction from $\neg A$. This is *proof by contradiction* in the strong sense: assuming $\neg A$ and deriving $\bot$ gives us $A$, not just $\neg\neg A$.

**The Curry-Howard view.** DNE and LEM do not have natural computational interpretations. There's no algorithm that, given a proof of $\neg\neg A$, produces a proof of $A$ — because a proof of $\neg\neg A$ is a function of type $(A \to \bot) \to \bot$, and calling it requires providing an "escape continuation" that can never be used. Classical axioms can be interpreted using *continuations* (Parigot's λ-μ calculus), but these are non-standard computational objects. This is why constructive logic has cleaner connections to computation.

## Universal Quantification

For first-order logic, we add rules for quantifiers.

**Introduction rule** ($\forall$I):
$$\frac{\Gamma \vdash A[x/a]}{\Gamma \vdash \forall x, A}$$

where $a$ is a *fresh variable* not free in $\Gamma$ or $\forall x, A$ (an "arbitrary" variable).

To prove $\forall x, A(x)$: prove $A(a)$ for an arbitrary $a$ not mentioned elsewhere. The freshness condition ensures $a$ is truly arbitrary — the proof doesn't use any specific properties of $a$.

**Elimination rule** ($\forall$E, universal instantiation):
$$\frac{\Gamma \vdash \forall x, A}{\Gamma \vdash A[x/t]}$$

for any term $t$. From $\forall x, A(x)$, substitute any specific term $t$ to get $A(t)$.

**In programming/type theory.** $\forall x : A, B(x)$ is the *dependent product type* $\Pi_{x:A} B(x)$. A proof is a function that takes any $a : A$ and returns a proof of $B(a)$.

## Existential Quantification

**Introduction rule** ($\exists$I):
$$\frac{\Gamma \vdash A[x/t]}{\Gamma \vdash \exists x, A}$$

To prove $\exists x, A(x)$: exhibit a specific term $t$ and prove $A(t)$.

**This is the constructive content of existential statements.** A constructive proof of $\exists x, A(x)$ contains an explicit *witness* $t$ and a proof $A(t)$. You can't just argue "such a thing must exist" — you have to produce one.

**Elimination rule** ($\exists$E):
$$\frac{\Gamma \vdash \exists x, A \qquad \Gamma, A[x/a] \vdash C}{\Gamma \vdash C}$$

where $a$ is fresh (not free in $C$ or $\Gamma$).

To use a proof of $\exists x, A(x)$: give a name $a$ to the witness (treating it as an arbitrary element with property $A$) and derive $C$. The freshness condition ensures $C$ doesn't depend on which specific witness was chosen.

**In programming/type theory.** $\exists x : A, B(x)$ is the *dependent sum type* $\Sigma_{x:A} B(x)$. A proof is a pair $(t, b)$ where $t : A$ and $b : B(t)$. The elimination rule is pattern matching on pairs.

## Putting It Together: A Sample Derivation

Let's derive $\vdash \forall x, (P(x) \to P(x))$ — every property implies itself.

$$\frac{\dfrac{}{P(a) \vdash P(a)}}{\vdash P(a) \to P(a)} \to\text{I} \quad \vdash \forall x, P(x) \to P(x) \quad \forall\text{I}$$

Step by step:
1. By the Identity rule: $P(a) \vdash P(a)$.
2. By $\to$I (discharging the hypothesis $P(a)$): $\vdash P(a) \to P(a)$.
3. By $\forall$I (universalizing over fresh variable $a$): $\vdash \forall x, P(x) \to P(x)$.

This is a theorem — provable from no hypotheses.

## The Introduction-Elimination Balance

The design of natural deduction maintains a balance: each connective's introduction and elimination rules are "inverses" of each other.

- To introduce $A \wedge B$: need proofs of $A$ and $B$.
- To eliminate $A \wedge B$: get back proofs of $A$ and $B$.

This means: if you introduce a connective and immediately eliminate it, you've gone around in a circle. The roundtrip:
$$\frac{\Gamma \vdash A \quad \Gamma \vdash B}{\Gamma \vdash A \wedge B} \wedge\text{I} \quad \vdash A \quad \wedge\text{E}_1$$

is a *detour* — you proved $A \wedge B$ just to extract $A$, when you already had $A$. Normalization removes these detours, and this removal corresponds directly to computation in the Curry-Howard correspondence.

The elimination-introduction balance is also why we can ask: for which connectives do the rules "uniquely characterize" the connective? This is the question of *local completeness* and *local soundness*, which we'll examine in the normalization section.

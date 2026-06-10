# 1.1 The Syntax of Propositional Logic

## Language Before Meaning

There's a useful two-stage approach to any formal system: first understand the *syntax* (what strings of symbols are legal), then understand the *semantics* (what those strings mean). The separation is not pedantic — it reflects a genuine distinction between form and interpretation.

In propositional logic, the syntax asks: what counts as a well-formed formula? The answer is given by an *inductive definition*, which specifies the formulas recursively. This pattern — defining a set of objects by specifying how to build them from simpler ones — recurs throughout mathematics and type theory. Get comfortable with it now.

## Atomic Propositions

We start with the simplest possible things: *atomic propositions* (also called *propositional variables* or *atoms*). These are the basic, unanalyzed statements:
- "$P$": "it is raining"
- "$Q$": "the ground is wet"
- "$R$": "there is a rainbow"

Individually, these are either true or false — but propositional logic doesn't analyze *why* they're true or what they *mean*. It only cares about how their truth values combine.

We use letters $P, Q, R, \ldots$ (or $p, q, r, \ldots$, or $A, B, C, \ldots$ depending on convention) to denote atomic propositions. There are infinitely many of them available.

## Connectives

From atoms, we build complex propositions using *logical connectives*. Each connective takes one or more propositions and builds a new one:

| Symbol | Name | Meaning | Arity |
|--------|------|---------|-------|
| $\neg$ | Negation | "not $\varphi$" | 1 (unary) |
| $\wedge$ | Conjunction | "$\varphi$ and $\psi$" | 2 (binary) |
| $\vee$ | Disjunction | "$\varphi$ or $\psi$" | 2 (binary) |
| $\to$ | Implication | "if $\varphi$ then $\psi$" | 2 (binary) |
| $\leftrightarrow$ | Biconditional | "$\varphi$ if and only if $\psi$" | 2 (binary) |

**Notation:** In everyday speech, "or" is sometimes exclusive (either one or the other but not both). In logic, $\vee$ is *inclusive* or: "$\varphi \vee \psi$" is true if at least one of $\varphi$, $\psi$ is true, including when both are.

## The Inductive Definition of Formulas

**Definition (Well-Formed Formula).** The set $\mathsf{Form}$ of *well-formed formulas* (wff) of propositional logic is defined inductively:

**Base case:**
- Every atomic proposition $P$ is a wff.

**Recursive cases:**
- If $\varphi$ is a wff, then $\neg\varphi$ is a wff.
- If $\varphi$ and $\psi$ are wffs, then $(\varphi \wedge \psi)$ is a wff.
- If $\varphi$ and $\psi$ are wffs, then $(\varphi \vee \psi)$ is a wff.
- If $\varphi$ and $\psi$ are wffs, then $(\varphi \to \psi)$ is a wff.
- If $\varphi$ and $\psi$ are wffs, then $(\varphi \leftrightarrow \psi)$ is a wff.

**Closure:** Nothing else is a wff.

The last clause is crucial: it says the definition is exhaustive. A string of symbols is a wff only if it can be built by the above rules, not if it merely looks formula-like.

## Reading Inductive Definitions

Let's think carefully about what this definition does.

It defines a set $\mathsf{Form}$ as the *smallest* set satisfying the clauses above. "Smallest" means: if anything satisfies those clauses, the set we defined is contained in it. This is the right reading — we want the formulas to be exactly what the rules generate, no more.

Equivalently, $\mathsf{Form}$ is the *intersection* of all sets closed under the above rules. This interpretation lets you prove things about all formulas by showing they hold for atoms (base case) and are preserved by each construction step (inductive cases).

This is *structural induction*, which we'll see in Section 3.

## Precedence Conventions

Strict adherence to the definition requires parentheses around every binary connective: $((P \wedge Q) \to R)$ rather than $P \wedge Q \to R$. The parentheses uniquely determine how the formula is parsed.

In practice, we use precedence conventions to reduce parentheses:
1. $\neg$ binds most tightly (prefix, applied first)
2. $\wedge$ (left-associative)
3. $\vee$ (left-associative)
4. $\to$ (right-associative)
5. $\leftrightarrow$ (left-associative, binds least tightly)

So $P \vee Q \to R \wedge S$ parses as $(P \vee Q) \to (R \wedge S)$, and $P \to Q \to R$ parses as $P \to (Q \to R)$.

**Warning:** Different texts use different conventions. Always check which ones apply. When in doubt, add parentheses.

## Examples of Formulas

Well-formed formulas:
- $P$ (just an atom)
- $\neg P$
- $P \wedge Q$
- $(P \wedge Q) \to R$
- $\neg(P \vee \neg Q)$
- $P \to (Q \to P)$ (reads: "P implies that Q implies P")
- $(P \to Q) \wedge (Q \to P)$ (reads: "P implies Q and Q implies P," equivalent to $P \leftrightarrow Q$)

Not well-formed:
- $P \wedge$ (incomplete — $\wedge$ requires two arguments)
- $\neg \neg$ (no argument for the inner $\neg$)
- $P Q$ (no connective between them)
- $\to P Q$ (wrong syntax for $\to$)

## Subformulas

**Definition (Subformula).** The *subformulas* of a formula are defined recursively:
- The only subformula of an atom $P$ is $P$ itself.
- The subformulas of $\neg\varphi$ are $\neg\varphi$ and all subformulas of $\varphi$.
- The subformulas of $(\varphi \star \psi)$ (for any binary connective $\star$) are $(\varphi \star \psi)$ and all subformulas of $\varphi$ and $\psi$.

Every formula is a subformula of itself. The subformula relation is *strict* when we require a proper subformula (a subformula that is not the formula itself).

The subformula relation is well-founded: every formula has only finitely many subformulas, all strictly smaller. This is why structural induction on formulas always terminates.

## Why Syntax Matters for Type Theory

In type theory, *types* are defined by very similar inductive rules:
- Base types (like the type of booleans `Bool`, or natural numbers `ℕ`)
- Type-forming operations (function types `A → B`, dependent product types `Π(x:A). B(x)`, etc.)

The *terms* inhabiting types are also defined inductively. And proving properties of all terms or all types uses *structural induction* — the same principle we apply to propositional formulas.

Propositional logic, in this sense, is a warm-up. The syntax of propositional logic is very simple. The syntax of a dependent type theory like Martin-Löf Type Theory (Chapter 9) is much more complex, but the *style* of definition — inductive, built from base cases and construction rules — is identical.

Learning to read and work with inductive definitions here makes everything later more transparent.

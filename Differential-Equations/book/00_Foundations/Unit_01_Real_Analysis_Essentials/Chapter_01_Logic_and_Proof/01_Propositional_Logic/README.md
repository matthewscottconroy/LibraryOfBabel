# Propositional Logic

A proposition is any declarative sentence that has a definite truth value — true or false, with no ambiguity and no dependence on unspecified variables. "The integer 7 is prime" is a proposition (it is true). "Every continuous function is differentiable" is a proposition (it is false; $f(x) = |x|$ is a counterexample). "Solve for $x$" is not a proposition. Mathematical proofs are built from propositions connected by logical operators, and the rules governing those connections are the subject of propositional logic.

## Logical Connectives

Given propositions $P$ and $Q$, the standard connectives produce new propositions:

- **Negation**: $\neg P$ ("not $P$") is true exactly when $P$ is false.
- **Conjunction**: $P \land Q$ ("$P$ and $Q$") is true exactly when both $P$ and $Q$ are true.
- **Disjunction**: $P \lor Q$ ("$P$ or $Q$") is true when at least one of $P$, $Q$ is true. This is the inclusive or.
- **Implication**: $P \Rightarrow Q$ ("if $P$ then $Q$", or "$P$ implies $Q$") is false only when $P$ is true and $Q$ is false. It is vacuously true when $P$ is false.
- **Biconditional**: $P \Leftrightarrow Q$ ("$P$ if and only if $Q$") is true exactly when $P$ and $Q$ have the same truth value.

A truth table displays all possible combinations of truth values for the component propositions and the resulting truth value of a compound proposition. For the implication $P \Rightarrow Q$:

| $P$ | $Q$ | $P \Rightarrow Q$ |
|-----|-----|-------------------|
| T   | T   | T                 |
| T   | F   | F                 |
| F   | T   | T                 |
| F   | F   | T                 |

The vacuous truth — an implication with a false hypothesis — is initially counterintuitive but is essential for the system to be consistent. The statement "if $0 = 1$, then the moon is made of cheese" is true as a proposition, because the hypothesis is false.

## Tautologies and Logical Equivalence

A **tautology** is a compound proposition that is true for every possible combination of truth values of its components. The law of excluded middle, $P \lor \neg P$, is a tautology. So is $\neg(P \land \neg P)$ (the law of non-contradiction).

Two propositions are **logically equivalent**, written $P \equiv Q$, when they have identical truth tables. Several equivalences are used constantly in analysis proofs:

**De Morgan's Laws:**
$$\neg(P \land Q) \equiv \neg P \lor \neg Q, \qquad \neg(P \lor Q) \equiv \neg P \land \neg Q.$$

**Contrapositive:**
$$(P \Rightarrow Q) \equiv (\neg Q \Rightarrow \neg P).$$

**Double Negation:**
$$\neg \neg P \equiv P.$$

**Implication as Disjunction:**
$$(P \Rightarrow Q) \equiv (\neg P \lor Q).$$

The contrapositive equivalence is particularly important. To prove $P \Rightarrow Q$, one may instead prove $\neg Q \Rightarrow \neg P$, which is logically the same statement. Which form is easier depends on the specific theorem.

## Converse and Contrapositive

Given $P \Rightarrow Q$:
- The **converse** is $Q \Rightarrow P$. This is not equivalent to the original.
- The **inverse** is $\neg P \Rightarrow \neg Q$. This is also not equivalent to the original.
- The **contrapositive** is $\neg Q \Rightarrow \neg P$. This is equivalent to the original.

A common error in mathematical reasoning is to prove the converse when only the original implication was required, or to assume that because $P \Rightarrow Q$ is true, so is $Q \Rightarrow P$. For instance, "if $f$ is differentiable at $a$, then $f$ is continuous at $a$" is true, but its converse is false.

## Compound Propositions and Parsing

In more complex logical expressions, precedence rules govern parsing. The standard convention, from highest to lowest precedence, is: $\neg$, then $\land$, then $\lor$, then $\Rightarrow$, then $\Leftrightarrow$. So $P \lor Q \Rightarrow R \land S$ parses as $(P \lor Q) \Rightarrow (R \land S)$.

**Example.** Determine the truth value of $(P \land Q) \Rightarrow (P \lor R)$ when $P$ is true, $Q$ is false, and $R$ is false.

Since $Q$ is false, $P \land Q$ is false. The implication has a false hypothesis, so it is vacuously true regardless of the truth value of $P \lor R$.

## Common Pitfalls

**Confusing implication and biconditional.** The statement "$f$ has a local maximum at $a$ if $f'(a) = 0$" uses "if" to denote the converse: $f'(a) = 0 \Rightarrow$ local max. This is false (consider $f(x) = x^3$ at $x = 0$). The correct statement is that a local max implies $f'(a) = 0$ (Fermat's theorem), not the other way around.

**Asserting the converse.** In a proof that $P \Rightarrow Q$, it is not valid to conclude $Q \Rightarrow P$ without a separate argument.

**Misapplying De Morgan.** The negation of "$P$ and $Q$" is "not $P$ or not $Q$", not "not $P$ and not $Q$". This error appears frequently when students try to negate definitions involving conjunctions or disjunctions.

## Connection to Proof Structure

Every proof, at its logical core, establishes an implication. Understanding propositional logic tells you what it means to assume the hypothesis and derive the conclusion, and gives you the freedom to switch to the contrapositive form when that is more convenient. De Morgan's laws are the mechanism by which the negations of definitions are formed, and those negations appear in proofs by contradiction and in the precise characterization of failure (for example, the negation of "sequence converges" is the statement that the sequence diverges). The two subsequent sections of this chapter build directly on the propositional framework to handle the quantified statements that define limits, continuity, and convergence.

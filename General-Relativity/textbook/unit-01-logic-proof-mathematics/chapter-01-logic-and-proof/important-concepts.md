# Chapter 1: Important Concepts

---

A precise glossary of the key terms introduced in Chapter 1. Definitions are given rigorously; intuitive commentary follows where helpful.

---

**Proposition** (also: statement, sentence)  
A declarative sentence with a definite, unambiguous truth value — either True or False, but not both and not neither. Propositions are the atomic units of propositional logic.

**Truth value**  
One of the two elements {True, False} (alternatively written {T, F}, {1, 0}, or {⊤, ⊥}). Every proposition has exactly one truth value.

**Law of the Excluded Middle**  
For any proposition P: P ∨ ¬P is a tautology. Every proposition is either true or false; there is no middle ground. Rejected by intuitionistic logicians.

**Law of Non-Contradiction**  
For any proposition P: ¬(P ∧ ¬P) is a tautology. No proposition is simultaneously true and false.

**Logical Connective**  
An operation that combines one or more propositions to form a new proposition. The standard connectives are: negation (¬), conjunction (∧), disjunction (∨), implication (→), biconditional (↔).

**Negation (¬P)**  
True iff P is False.

**Conjunction (P ∧ Q)**  
True iff both P and Q are True.

**Disjunction (P ∨ Q)**  
True iff at least one of P, Q is True. (Inclusive OR.)

**Implication (P → Q)**  
False iff P is True and Q is False; True otherwise. The antecedent is P; the consequent is Q.

**Biconditional (P ↔ Q)**  
True iff P and Q have the same truth value. Equivalent to (P → Q) ∧ (Q → P).

**Truth Table**  
A complete tabulation of the truth values of a compound proposition for all possible assignments of truth values to its atomic components.

**Tautology**  
A proposition that is True for every possible assignment of truth values to its components. Examples: P ∨ ¬P, (P → Q) ↔ (¬Q → ¬P), ((P → Q) ∧ P) → Q.

**Contradiction**  
A proposition that is False for every possible assignment of truth values. Example: P ∧ ¬P.

**Contingent proposition**  
A proposition that is True for some assignments and False for others — neither a tautology nor a contradiction.

**Logical Equivalence (P ≡ Q)**  
Two propositions P and Q are logically equivalent if they have the same truth value for every possible assignment of truth values to their components; equivalently, if P ↔ Q is a tautology.

**De Morgan's Laws**  
The equivalences: ¬(P ∧ Q) ≡ ¬P ∨ ¬Q and ¬(P ∨ Q) ≡ ¬P ∧ ¬Q. Used to distribute negation across conjunctions and disjunctions.

**Contrapositive**  
The contrapositive of P → Q is ¬Q → ¬P. They are logically equivalent.

**Converse**  
The converse of P → Q is Q → P. Not logically equivalent to the original in general.

**Modus Ponens**  
The inference rule: from P and P → Q, conclude Q. The tautology ((P → Q) ∧ P) → Q.

**Modus Tollens**  
The inference rule: from ¬Q and P → Q, conclude ¬P. Equivalent to modus ponens applied to the contrapositive.

**Boolean Algebra**  
An algebraic structure with two operations (AND, OR) and a complement (NOT) satisfying commutativity, associativity, distributivity, identity, and complement laws. Propositional logic has the structure of a Boolean algebra on the set {T, F}.

**Predicate (propositional function)**  
An expression with one or more free variables that becomes a proposition when values are assigned to those variables. Example: P(x) = "x is prime."

**Domain of Discourse**  
The set of objects over which variables range in a given context. Must always be specified for predicate logic statements to be meaningful.

**Universal Quantifier (∀x P(x))**  
"For all x in the domain, P(x) is true." False iff there exists a counterexample.

**Existential Quantifier (∃x P(x))**  
"There exists at least one x in the domain for which P(x) is true." True iff at least one witness exists.

**Uniqueness Quantifier (∃! x P(x))**  
"There exists exactly one x such that P(x)." Equivalent to ∃x P(x) ∧ ∀x ∀y (P(x) ∧ P(y) → x = y).

**Counterexample**  
A specific element of the domain for which the predicate P fails, thereby disproving a universal statement ∀x P(x).

**Witness**  
A specific element of the domain for which P holds, thereby proving an existential statement ∃x P(x).

**Free variable**  
A variable in a formula that is not bound by any quantifier. A formula with free variables is an open formula (predicate), not a proposition.

**Bound variable**  
A variable captured by a quantifier (∀x or ∃x) within its scope.

**Scope**  
The subformula to which a quantifier applies.

**Alpha-equivalence**  
The equivalence of two formulas that differ only in the names of their bound variables (e.g., ∀x P(x) ≡ ∀z P(z)).

**Direct Proof**  
A proof of P → Q by assuming P and deriving Q through a chain of valid logical steps.

**Proof by Contrapositive**  
A proof of P → Q by instead proving ¬Q → ¬P, which is logically equivalent.

**Proof by Contradiction (Reductio ad Absurdum)**  
A proof of P by assuming ¬P and deriving a contradiction, establishing that ¬P is false and therefore P is true.

**Mathematical Induction**  
A proof technique for ∀n ∈ ℕ P(n): prove the base case P(0) and the inductive step ∀k (P(k) → P(k+1)).

**Strong Induction**  
A variant of mathematical induction in which the inductive step assumes P(0), P(1), ..., P(k) and derives P(k+1). Logically equivalent to ordinary induction.

**Induction Hypothesis**  
The assumption P(k) made in the inductive step of an induction proof.

**Gödel's Incompleteness Theorems**  
(1) Any consistent formal system powerful enough to express arithmetic contains true statements that cannot be proved within the system. (2) Such a system cannot prove its own consistency. Proved by Kurt Gödel (1931).

**Proof by Cases**  
A proof technique that exhausts all possibilities (cases) and proves the conclusion in each case separately.

**Axiom**  
A statement accepted without proof as the starting point for a formal system. All theorems are derived from the axioms by rules of inference.

**Theorem**  
A statement proved from the axioms by a finite sequence of valid logical steps (a proof).

**Lemma**  
A theorem proved specifically to be used in the proof of a larger theorem.

**Corollary**  
A theorem that follows immediately and with little additional work from a previously proved theorem.

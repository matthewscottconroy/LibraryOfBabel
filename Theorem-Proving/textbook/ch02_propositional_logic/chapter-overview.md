# Chapter 2 Overview: Propositional Logic

---

## Central Question

How do we reason about statements that can be combined by "and," "or," "not," and "if ... then ..."? Can we reduce reasoning about these combinations to a decision procedure — an algorithm that always gives a yes/no answer?

Propositional logic is the simplest non-trivial formal system: powerful enough to model significant mathematical reasoning, yet simple enough to have completely algorithmic decision procedures. It is the essential foundation before quantifiers, before dependent types, before anything more expressive.

---

## Why This Chapter Matters

Propositional logic is directly implemented in hardware (Boolean circuits), in programming languages (type systems, condition evaluation), in proof assistants (Lean 4's `Prop`, Coq's `Prop`), and in automated theorem provers (SAT solvers, which are the industrial workhorse of formal verification). Understanding its structure — both the semantic (truth-table) and syntactic (proof-system) levels — is essential for everything that follows.

---

## Key Definitions

**Propositional language.** The language $\mathcal{L}_{prop}$ consists of:
- Propositional variables: $p_0, p_1, p_2, \ldots$
- Logical constants: $\top$ (truth), $\bot$ (falsity)
- Connectives: $\neg$ (negation), $\land$ (conjunction), $\lor$ (disjunction), $\to$ (conditional/implication), $\leftrightarrow$ (biconditional)
- Punctuation: parentheses $(, )$

**Truth assignment.** A truth assignment is a function $v: \text{Var} \to \{0, 1\}$ mapping propositional variables to truth values. It extends to all formulas by the standard recursive clauses (e.g., $v(\phi \land \psi) = 1$ iff $v(\phi) = 1$ and $v(\psi) = 1$).

**Model.** A model for a formula is any truth assignment; propositional logic has no "domain" or "interpretation" beyond the truth assignment.

**Tautology.** A formula $\phi$ is a tautology (written $\vDash \phi$) if $v(\phi) = 1$ for every truth assignment $v$.

**Satisfiable.** A formula is satisfiable if there exists some truth assignment making it true.

**Contradiction.** A formula is a contradiction (or unsatisfiable) if no truth assignment makes it true.

**Logical consequence.** $\phi \vDash \psi$ if every truth assignment satisfying $\phi$ also satisfies $\psi$.

**Conjunctive Normal Form (CNF).** A formula is in CNF if it is a conjunction of *clauses*, where each clause is a disjunction of *literals* (variables or their negations). Example: $(p \lor \neg q) \land (\neg p \lor r \lor s)$.

**Disjunctive Normal Form (DNF).** A formula is in DNF if it is a disjunction of *conjunctions* of literals.

**Resolution.** Given clauses $C_1 = A \lor \ell$ and $C_2 = B \lor \neg\ell$, the *resolvent* is $A \lor B$. Resolution is a single inference rule that is complete for propositional refutation.

---

## Main Theorems

### Theorem: Every Formula Has a CNF (and DNF) Equivalent

**Statement.** For every propositional formula $\phi$, there exist formulas $\phi_{CNF}$ in CNF and $\phi_{DNF}$ in DNF such that $\phi \equiv \phi_{CNF} \equiv \phi_{DNF}$ (logically equivalent, meaning same truth table).

**Proof sketch.** By induction on formula structure, using De Morgan's laws and distributivity:
- $\neg(\phi \land \psi) \equiv (\neg\phi \lor \neg\psi)$   (De Morgan)
- $\neg(\phi \lor \psi) \equiv (\neg\phi \land \neg\psi)$   (De Morgan)
- $\phi \lor (\psi \land \chi) \equiv (\phi \lor \psi) \land (\phi \lor \chi)$   (distributivity, for CNF)
- $\phi \land (\psi \lor \chi) \equiv (\phi \land \psi) \lor (\phi \land \chi)$   (distributivity, for DNF)

Push all negations inward (to literals) using De Morgan, then distribute. Every formula has a CNF. $\square$

**Warning:** The CNF equivalent can be exponentially larger than the original formula. This is not a flaw in the proof — it is a fundamental feature.

### Theorem: Compactness for Propositional Logic

**Statement.** An infinite set of propositional formulas $\Gamma$ is satisfiable if and only if every finite subset of $\Gamma$ is satisfiable.

**Proof sketch.** The "only if" direction is immediate (any satisfying assignment for $\Gamma$ satisfies any subset). For the "if" direction: enumerate the variables as $p_0, p_1, p_2, \ldots$. Define a satisfying assignment by a König's lemma argument on the binary tree of partial assignments, extending consistently at each step. If every finite subset of $\Gamma$ is satisfiable, we can always extend any partial consistent assignment to include the next variable. $\square$

This theorem has profound consequences, including the possibility of non-standard models (Chapter 9).

### Theorem: Resolution Completeness

**Statement.** A set of clauses $\Gamma$ is unsatisfiable if and only if the empty clause $\bot$ can be derived from $\Gamma$ by finitely many resolution steps.

**Proof sketch.** *Soundness* is easy: each resolution step preserves satisfiability (the resolvent is a logical consequence of the two parent clauses). *Completeness* is the hard direction, proved by showing that if $\Gamma$ is unsatisfiable, there is a variable $p$ such that both $\Gamma[p:=\top]$ and $\Gamma[p:=\bot]$ (with $p$ set to true/false) lead to smaller unsatisfiable sets, and by induction the empty clause is derivable. $\square$

### Completeness of Propositional Proof Systems

**Statement.** For any proof system sound for propositional logic (natural deduction, Hilbert system), if $\Gamma \vDash \phi$ then $\Gamma \vdash \phi$.

**Proof sketch.** The key lemma is that any consistent set of formulas is satisfiable (Lindenbaum-Tarski construction). If $\Gamma \not\vdash \phi$, then $\Gamma \cup \{\neg\phi\}$ is consistent, hence satisfiable, hence $\Gamma \not\vDash \phi$. The contrapositive gives completeness. $\square$

---

## Truth Tables as Decision Procedures

For a formula with $n$ propositional variables, the truth table has $2^n$ rows. The following algorithm decides any propositional question:

1. Identify all variables $p_1, \ldots, p_n$ in the formula(s).
2. Enumerate all $2^n$ truth assignments.
3. For each assignment, evaluate the formula recursively (in time $O(|\phi|)$).

**Total time:** $O(2^n \cdot |\phi|)$ — exponential in the number of variables, but *always terminating*. Propositional logic is **decidable**. (First-order logic, Chapter 3, is only semi-decidable.)

However, the best known algorithms for propositional satisfiability (SAT solvers using DPLL + clause learning) run in worst-case exponential time. Whether SAT can be decided in polynomial time is equivalent to the P = NP problem.

---

## Normal Forms: Worked Examples

**Example: Convert to CNF.**

$\phi = \neg(p \to q) \lor r$

Step 1: Eliminate $\to$: $\phi = \neg(\neg p \lor q) \lor r$

Step 2: Push $\neg$ inward: $\phi = (p \land \neg q) \lor r$

Step 3: Distribute $\lor$ over $\land$: $\phi = (p \lor r) \land (\neg q \lor r)$

This is now in CNF. $\square$

---

## Connections to Other Chapters

- **Chapter 1** introduced the distinction between syntax and semantics; this chapter instantiates it fully for propositional logic.
- **Chapter 4** develops natural deduction and sequent calculus, proving soundness and completeness for these proof systems.
- **Chapter 10** discusses the limits of decision procedures: Gödel's incompleteness theorems apply to systems far more expressive than propositional logic, but their proof technique (arithmetisation) has propositional analogues (circuit lower bounds).
- **Chapter 13** (Formal Verification): SAT solvers and BDD-based model checkers are the industrial applications of propositional decision procedures. Propositional Horn clause satisfiability underlies Prolog.

---

## Historical Context

**George Boole (1847, 1854)** created the algebra of logic: 0 and 1 represent false and true, AND is multiplication, OR is addition modulo 2 (or bounded addition). Boole showed that syllogistic and beyond could be reduced to algebra.

**Charles Sanders Peirce (1880s)** independently developed the propositional calculus with a complete set of connectives. He also discovered the Sheffer stroke (NAND), showing that all connectives can be defined from one.

**Emil Post (1921)** proved that any two-valued propositional logic that is a subset of tautologies is contained in one of a finite number of closed classes — the *Post lattice*, a complete classification of propositional clones.

**Stephen Cook (1971)** proved that SAT (the satisfiability problem for propositional logic in CNF) is NP-complete. This is perhaps the most important single result in computational complexity theory.

**Davis and Putnam (1960), Davis, Logemann, and Loveland (1962)** introduced the DPLL algorithm (Splitting + Unit Propagation), the ancestor of all modern SAT solvers.

---

## Common Confusions

**$\to$ is not causation.** "$p \to q$" is false only when $p$ is true and $q$ is false. It does not assert any causal or temporal relationship. "$p \to q$" is vacuously true when $p$ is false, regardless of $q$.

**$\vDash$ vs. $\vdash$.** $\vDash$ is semantic (truth in all models); $\vdash$ is syntactic (derivable in a proof system). Soundness says $\vdash$ implies $\vDash$; completeness says $\vDash$ implies $\vdash$. They are different until proved equivalent.

**CNF vs. DNF complexity.** Every formula has both CNF and DNF equivalents, but converting from formula to CNF and from formula to DNF have different worst-case sizes. CNF conversion can be exponential; DNF conversion can also be exponential. The succinct representations (circuits, BDDs) avoid this but add their own complexity.

---

## Tool Connections

- **Lean 4:** `p ∧ q`, `p ∨ q`, `¬p`, `p → q`, `p ↔ q` are first-class propositions. Tautologies are proved by `tauto` or `decide`.
- **Coq:** `Prop` is the type of propositions. `auto` and `tauto` handle propositional goals.
- **Python/SymPy:** `from sympy.logic import satisfiable, tautology`
- **SAT solvers:** Minisat, CaDiCaL, Kissat accept CNF in DIMACS format.

# Normal Forms: Canonical Shapes for Logical Formulas

> *"In a well-developed theory, everything is either obvious or can be made obvious by a systematic transformation."*

---

Mathematics has a recurring pattern: take an arbitrary object from a messy, complicated space, and transform it into a **canonical form** — a standard, normalized representative of its equivalence class. Fractions are reduced to lowest terms. Polynomials are written with terms in decreasing degree. Matrices are brought to row-echelon form. The canonical form strips away incidental variation and exposes essential structure.

Propositional logic has its own canonical forms, and they are not merely aesthetic conveniences. They are the foundations of automated reasoning — the shapes that SAT solvers require, that circuit synthesis tools produce, and that proof systems use internally to simplify search.

## What Is a Normal Form?

A **normal form** is a syntactically specified format to which every formula can be converted by a truth-preserving transformation (actually, an equivalence-preserving one — the resulting formula is logically equivalent to the original). Two formulas in the same normal form that are logically equivalent must be syntactically identical, making equivalence checking trivial.

We focus on two forms: CNF and DNF.

## Literals and Clauses: The Building Blocks

Before defining the forms, we need vocabulary.

A **literal** is either an atomic formula (a positive literal, like p or q) or the negation of an atomic formula (a negative literal, like ¬p or ¬q). Literals are the simplest components from which CNF and DNF are assembled.

A **clause** (for CNF) is a disjunction of literals:
$$l_1 \vee l_2 \vee \cdots \vee l_k$$

A **term** (or *monom*; for DNF) is a conjunction of literals:
$$l_1 \wedge l_2 \wedge \cdots \wedge l_k$$

## Conjunctive Normal Form (CNF)

A formula is in **Conjunctive Normal Form (CNF)** if it is a conjunction of clauses:
$$(l_{11} \vee \cdots \vee l_{1k_1}) \wedge (l_{21} \vee \cdots \vee l_{2k_2}) \wedge \cdots$$

*Every formula is equivalent to a formula in CNF.*

Think of CNF as a list of requirements that must all be satisfied. Each clause is a "soft constraint" — at least one literal in it must be true. The entire formula is satisfied when every clause is satisfied. This is exactly the format that SAT solvers expect.

**Example**: `(p ∨ ¬q) ∧ (¬p ∨ r) ∧ (q ∨ r)` is in CNF. Three clauses, each a disjunction of literals.

## Disjunctive Normal Form (DNF)

A formula is in **Disjunctive Normal Form (DNF)** if it is a disjunction of terms:
$$(l_{11} \wedge \cdots \wedge l_{1k_1}) \vee (l_{21} \wedge \cdots \wedge l_{2k_2}) \vee \cdots$$

*Every formula is also equivalent to a formula in DNF.*

Think of DNF as a list of complete scenarios — conditions under which the formula is satisfied. Each term specifies a complete valuation for a subset of the variables. The formula is satisfied when at least one term is fully satisfied. DNF is the format of truth tables (each row where the formula is true contributes a term).

## Converting to CNF: The Algorithm

The conversion algorithm for CNF has four steps:

**Step 1: Eliminate ↔.**
$$\phi \leftrightarrow \psi \quad\Rightarrow\quad (\phi \rightarrow \psi) \wedge (\psi \rightarrow \phi)$$

**Step 2: Eliminate →.**
$$\phi \rightarrow \psi \quad\Rightarrow\quad \neg\phi \vee \psi$$

**Step 3: Push ¬ inward** using De Morgan's laws and double negation elimination.
$$\neg(\phi \wedge \psi) \;\Rightarrow\; \neg\phi \vee \neg\psi$$
$$\neg(\phi \vee \psi) \;\Rightarrow\; \neg\phi \wedge \neg\psi$$
$$\neg\neg\phi \;\Rightarrow\; \phi$$
Repeat until negations appear only in front of atoms.

**Step 4: Distribute ∧ over ∨.**
$$(A \vee (B \wedge C)) \;\Rightarrow\; (A \vee B) \wedge (A \vee C)$$
Repeat until ∧ never appears inside a ∨.

After these steps, the formula is in CNF.

**Example**: Convert ¬(p → q) to CNF.
1. Eliminate →: ¬(¬p ∨ q)
2. Push ¬ inward (De Morgan): ¬¬p ∧ ¬q
3. Eliminate double negation: p ∧ ¬q
4. Already in CNF (each conjunct is a single literal, which is a clause).

Result: `p ∧ ¬q`. This makes intuitive sense: p → q fails exactly when p is true and q is false.

## The Tseitin Transformation: CNF Without Exponential Blowup

There is a subtle problem with the straightforward CNF conversion: the distribution step (Step 4) can cause exponential blowup. For example, distributing ∧ over ∨ in:
$$(p_1 \wedge q_1) \vee (p_2 \wedge q_2) \vee \cdots \vee (p_n \wedge q_n)$$
produces a formula with 2ⁿ clauses.

The **Tseitin transformation** (1968) avoids this by introducing fresh auxiliary variables for subformulas. For each subformula ψᵢ, introduce a new variable xᵢ and add equivalences xᵢ ↔ ψᵢ (converted to clauses). The top-level formula is then expressed as a conjunction of these definitions. The result is in CNF, is satisfiability-equivalent (not logically equivalent) to the original, and has size *linear* in the original formula.

SAT solvers use Tseitin-style encoding internally. When you ask Z3 to solve a non-CNF formula, it applies Tseitin transformation before running the CDCL search algorithm.

## CNF and the Resolution Rule

CNF is the natural format for the **resolution proof system** (Chapter 4). The resolution rule says: from clauses (A ∨ l) and (B ∨ ¬l), derive (A ∨ B). Applied to CNF formulas, resolution can derive the empty clause (a contradiction), proving the original formula unsatisfiable.

The connection between CNF and resolution means that SAT solvers are, in a deep sense, automated theorem provers: they either find a satisfying assignment or produce a resolution refutation proving unsatisfiability.

## Real-World Significance: Industrial SAT Solving

The conversion to CNF enables the \$1.2 billion industry of **formal hardware verification**. Modern CPU designs contain billions of transistors and are too complex to test exhaustively. Hardware verification tools convert correctness properties into SAT instances (in CNF) and use SAT solvers to either find bugs or prove correctness.

After Intel's Pentium FDIV bug in 1994 — a flaw in the floating-point division unit that cost Intel \$475 million in recalls — the company invested heavily in formal verification. Today, every Intel and AMD processor goes through formal equivalence checking (a form of SAT) before manufacture. The mathematical content of that verification is, at its core, the question: is this CNF formula satisfiable?

---

*Next: The resolution proof system — how to prove theorems by finding contradictions in CNF.*

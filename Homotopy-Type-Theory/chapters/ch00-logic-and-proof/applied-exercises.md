# Applied Exercises

The logical structures introduced in this chapter — propositional connectives, quantifiers, inference rules, inductive definitions, proof by contradiction and contrapositive — are not confined to pure mathematics. They are the invisible skeleton of reliable systems across every technical domain. A circuit is a physical implementation of Boolean logic. A type-checker is a mechanical proof verifier. An SQL query is a restricted first-order formula evaluated over a finite relational model. The discipline of writing proofs carefully, introduced here as a mathematical skill, is identical to the discipline required to reason correctly about any formal system. These exercises make that connection concrete.

---

## Exercise A.1: Logic Gates and Boolean Completeness
*Domain: Digital Circuit Design / Computer Architecture*

**Setup:** Digital circuits are built from logic gates — physical devices that compute Boolean functions. A NAND gate computes $\neg(P \wedge Q)$: it outputs 0 (false) only when both inputs are 1 (true). The claim is that NAND gates alone can implement any Boolean function — a property called *functional completeness*. This is why real hardware can be (and historically was) built entirely from a single gate type.

**Questions:**
1. Using only NAND gates (i.e., only the connective $\mathrm{NAND}(P,Q) = \neg(P \wedge Q)$), express $\neg P$, $(P \wedge Q)$, and $(P \vee Q)$ as formulas built from NAND. Verify each using truth tables.
2. Using the connectives $\{\neg, \wedge, \vee\}$ as primitives, construct the *exclusive or* (XOR) connective $P \oplus Q$, defined as: $P \oplus Q$ is true iff exactly one of $P$, $Q$ is true. Write a truth table and give a formula.
3. Prove by structural induction that every Boolean function $f : \{0,1\}^n \to \{0,1\}$ can be expressed as a propositional formula using only $\{\neg, \wedge, \vee\}$. (*Hint:* the disjunctive normal form argument — what is the base case? What is the inductive step when you add a new variable?)

*Abstract concept illustrated: Functional completeness of connective sets; structural induction over the syntax of formulas (Section 1 and Section 3 of this chapter).*

---

## Exercise A.2: Prolog Queries as First-Order Logic
*Domain: Logic Programming / Artificial Intelligence*

**Setup:** Prolog is a programming language based on first-order logic. A Prolog *fact* such as `parent(tom, bob).` encodes the atomic sentence $\mathsf{parent}(\mathsf{tom}, \mathsf{bob})$. A *rule* such as `ancestor(X, Z) :- parent(X, Y), ancestor(Y, Z).` encodes the universal sentence $\forall X \forall Z \,[\exists Y \,(\mathsf{parent}(X,Y) \wedge \mathsf{ancestor}(Y,Z)) \to \mathsf{ancestor}(X,Z)]$. A *query* is an existential statement whose witness Prolog attempts to find.

**Questions:**
1. Translate the following Prolog program into first-order logic sentences using $\forall$, $\exists$, $\wedge$, $\to$, and predicate notation. Then draw the formal proof (derivation tree) that Prolog implicitly constructs when answering the query `?- ancestor(tom, pat).`
   ```
   parent(tom, bob).
   parent(bob, pat).
   ancestor(X, Y) :- parent(X, Y).
   ancestor(X, Z) :- parent(X, Y), ancestor(Y, Z).
   ```
2. Prolog's resolution strategy is refutation-complete for Horn clauses: if the negation of a query follows from the program, Prolog will eventually find a refutation. Relate this to proof by contradiction (Section 2.2): what does it mean for Prolog to "fail" to find a proof, and when is that failure a sound negative answer?
3. Write a Prolog program (or describe its logical structure in FOL) that encodes transitivity and symmetry of a relation, then argue using the predicate logic rules from Section 4 that your encoding correctly captures the intended meaning.

*Abstract concept illustrated: Translation between informal mathematical statements and formal first-order sentences; use of universal and existential quantifiers (Section 4).*

---

## Exercise A.3: SQL Queries as Restricted Predicate Logic
*Domain: Database Systems / Software Engineering*

**Setup:** A relational database table corresponds to a predicate: a table `Employee(id, name, dept, salary)` is a finite relation over four domains, and each row is a tuple making the predicate true. SQL `SELECT` queries are restricted first-order formulas — specifically, they correspond to positive existential queries (no universal quantifiers in the naive case; universal quantification requires tricks like double negation).

**Questions:**
1. Translate the following SQL query into a first-order formula. Identify each clause (SELECT, FROM, WHERE, JOIN) with its logical counterpart.
   ```sql
   SELECT e.name
   FROM Employee e
   JOIN Department d ON e.dept = d.id
   WHERE d.name = 'Engineering' AND e.salary > 80000;
   ```
   The formula should be of the form $\{x \mid \varphi(x)\}$ — a set comprehension — where $\varphi$ is a formula in the language of predicates $\mathsf{Employee}$ and $\mathsf{Department}$.

2. SQL's `NOT EXISTS` subquery is used to express universal conditions. For instance: "find all departments with no employees earning less than 50000." Write this in SQL and translate it into a first-order formula using universal quantification. Does your formula use $\forall$ directly, or as $\neg\exists\neg$? Show they are equivalent using the duality rules from Section 4.

3. Relational algebra has a direct correspondence with a fragment of first-order logic: selection ($\sigma$) corresponds to restriction with a formula; projection ($\pi$) corresponds to existential quantification over dropped columns; join ($\bowtie$) corresponds to conjunction. Formalize this correspondence precisely for at least two of these operators. Does SQL's `DISTINCT` keyword have a logical analogue?

*Abstract concept illustrated: Set comprehension notation $\{x \mid \varphi(x)\}$; duality between $\forall$ and $\exists$; predicate logic as a query language (Section 4).*

---

## Exercise A.4: Program Correctness via Pre- and Postconditions
*Domain: Software Verification / Formal Methods*

**Setup:** Hoare logic provides a proof system for reasoning about program correctness. A *Hoare triple* $\{P\}\, C\, \{Q\}$ asserts: if precondition $P$ holds before executing command $C$, then postcondition $Q$ holds afterward. This is a logical statement, and proving it requires applying inference rules — exactly the kind of structured proof writing in this chapter. Consider the following program fragment in pseudocode:

```
// Precondition: x >= 0
y := 0;
i := 0;
while i < x:
    y := y + 2*i + 1
    i := i + 1
// Postcondition: y = x^2
```

**Questions:**
1. The loop invariant is a property that holds before and after every iteration. Propose a loop invariant for this program. Your invariant should be a predicate on `i` and `y` that (a) holds when the loop begins (`i = 0, y = 0`), (b) is preserved by each iteration, and (c) together with the loop termination condition (`i = x`) implies the postcondition `y = x^2`.

2. Prove that your invariant is preserved by each iteration using the proof technique of mathematical induction from Section 3. Explicitly identify the base case, the inductive hypothesis, and the inductive step.

3. Hoare's assignment rule says: to establish $\{P[e/x]\}\; x := e\; \{P\}$, where $P[e/x]$ is $P$ with every free occurrence of $x$ replaced by $e$. This is a *backwards* proof rule — you derive the precondition from the postcondition. How does this relate to proof by contrapositive (Section 2.1)? Is there a logical duality between forward and backward reasoning about programs?

*Abstract concept illustrated: Mathematical induction as a proof technique (Section 3); the relationship between loop invariants and inductive hypotheses; structured formal reasoning (Section 5).*

---

## Exercise A.5: Type-Checking as Proof-Checking
*Domain: Programming Language Theory / Compiler Design*

**Setup:** The Curry-Howard correspondence states that types are propositions and typed programs are proofs. A type-checker — the component of a compiler that verifies type annotations — is therefore a proof-checker: it verifies that a program (proof term) has its claimed type (proposition). This exercise makes that correspondence concrete in a simple typed lambda calculus.

Define a *simple type system* with base types $\mathsf{Bool}$ and $\mathsf{Nat}$, and a function type constructor $\to$. Types are defined inductively:
- $\mathsf{Bool}$ and $\mathsf{Nat}$ are types.
- If $A$ and $B$ are types, then $A \to B$ is a type.

A *typing judgment* $\Gamma \vdash t : A$ says: "in context $\Gamma$ (a list of variable-type pairs), term $t$ has type $A$." The typing rules include:
- **Var:** If $(x : A) \in \Gamma$, then $\Gamma \vdash x : A$.
- **Abs:** If $\Gamma, x : A \vdash t : B$, then $\Gamma \vdash (\lambda x{:}A.\, t) : A \to B$.
- **App:** If $\Gamma \vdash f : A \to B$ and $\Gamma \vdash a : A$, then $\Gamma \vdash (f\, a) : B$.

**Questions:**
1. Compare the typing rules Var, Abs, App with the natural deduction rules for propositional logic from Section 1 (specifically: the assumption rule, the $\to$-introduction rule, and the $\to$-elimination rule / modus ponens). Write each rule in both the type-theoretic and the logical presentation and identify the exact correspondence.

2. Consider the term $\lambda f{:}(A \to B).\, \lambda x{:}A.\, f\, x$. Derive its type using the typing rules above (write out the full derivation tree). Then read the same derivation as a natural deduction proof. What logical proposition does this proof prove?

3. A *type error* in a program corresponds, under Curry-Howard, to a proof of a false proposition. Find a simple program (term) that would be well-typed only if $A \to (B \to A)$ were provable for any $A$ and $B$ (it is), and another that would be well-typed only if $A \to \neg A \to B$ were provable (the *ex falso* principle). Are both provable in the natural deduction system of this chapter?

*Abstract concept illustrated: The Curry-Howard correspondence between proofs and programs; natural deduction rules as typing rules (Section 1 and the connection to type theory previewed in the introduction).*

---

## Exercise A.6: Contradiction and Diagnosis in Safety-Critical Systems
*Domain: Systems Engineering / Formal Specification*

**Setup:** In safety-critical systems (avionics, medical devices, nuclear plant control), engineers use formal specification languages (e.g., Z, TLA+, Alloy) to describe system requirements and check them for consistency. A set of requirements is *consistent* if no contradiction can be derived from them — if a contradiction can be derived, the system cannot possibly satisfy all requirements simultaneously. This is precisely the notion of consistency from Section 2.2.

Consider the following (simplified) set of requirements for a train control system:
- **R1:** The train shall not move unless the door is closed.
- **R2:** The door shall be closed whenever the train is at speed above 5 km/h.
- **R3:** The train shall always be able to reach its destination.
- **R4:** The door shall open if and only if the train is at a designated station.
- **R5:** There exists a route between every pair of stations with no intermediate designated station.

**Questions:**
1. Formalize R1–R5 as first-order sentences over the predicates $\mathsf{Moving}(t)$, $\mathsf{DoorClosed}(t)$, $\mathsf{Speed}(t, v)$, $\mathsf{AtStation}(t)$, $\mathsf{CanReach}(a, b)$, and $\mathsf{Designated}(s)$. Try to be precise about the domain of discourse (what do variables range over?).

2. Derive a contradiction from this set of requirements. Use the logical proof techniques from this chapter (direct deduction, case analysis) to show which requirements are jointly inconsistent. Identify the minimal inconsistent subset.

3. Propose a revision to one requirement that restores consistency. Prove (informally but rigorously) that the revised set is consistent by exhibiting a model — a concrete scenario in which all revised requirements are satisfied simultaneously. (Exhibiting a model is the standard method for proving consistency, as Section 1 explains in the context of truth valuations.)

*Abstract concept illustrated: Proof by contradiction (Section 2.2); the relationship between consistency and model existence; formal specification as applied predicate logic (Sections 1 and 4).*

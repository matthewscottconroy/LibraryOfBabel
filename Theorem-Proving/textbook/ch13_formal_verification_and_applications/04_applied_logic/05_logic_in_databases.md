# Logic in Databases

## Relational Databases as Logic

The connection between databases and logic is not accidental — it is foundational. The **relational model** of databases (Codd, 1970) is directly inspired by mathematical relations from set theory and first-order logic.

A **relational database** is a collection of relations (tables). A **SQL query** is essentially a logical formula — it specifies which tuples satisfy certain conditions. The query engine evaluates the formula against the database as a model.

## Datalog: Pure Logic as a Database Language

**Datalog** is a subset of Prolog/logic programming used for database queries. A Datalog program consists of:
- **Facts**: Base relations (the "database")
- **Rules**: Horn clauses of the form `head :- body` (if body holds, conclude head)

```datalog
% Facts (database)
parent(alice, bob).
parent(bob, charlie).
parent(charlie, dave).

% Rules
ancestor(X, Y) :- parent(X, Y).
ancestor(X, Y) :- parent(X, Z), ancestor(Z, Y).

% Query
?- ancestor(alice, dave).
% Answer: yes
```

Datalog is:
- **Syntactically restricted FOL**: no function symbols, no negation in recursive rules
- **Decidable**: evaluation always terminates (unlike full Prolog)
- **Equivalent to relational algebra**: can express the same queries as SQL (for non-aggregate queries)

## SQL as FOL

Every SQL SELECT query corresponds to a formula in FOL (with database relations as predicates):

```sql
SELECT e.name, d.dept_name
FROM employee e, department d
WHERE e.dept_id = d.dept_id AND e.salary > 50000
```

Corresponds to:
$$\exists d.\; \text{Employee}(\text{name}, d.\text{dept\_id}, \text{salary}) \wedge \text{Department}(d.\text{dept\_id}, d.\text{dept\_name}) \wedge \text{salary} > 50000$$

The **relational algebra** (projection, selection, join, union, difference) is the algebraic equivalent of these logical operations.

## Conjunctive Query Containment

A fundamental database theory problem: does every answer to query $Q_1$ also satisfy query $Q_2$? This is **query containment**: $Q_1 \subseteq Q_2$.

For **conjunctive queries** (no negation, no disjunction — just joins and selections), containment is NP-complete and decidable. For full SQL (with negation), containment is undecidable.

Applications: query optimization (can we replace $Q_1$ with equivalent but faster $Q_2$?), view maintenance, data integration.

## Constraint Checking and Integrity

Databases enforce **integrity constraints** — logical statements that must be true of the data:
- **Primary key**: each tuple is uniquely identified
- **Foreign key**: references must exist
- **Check constraints**: $\forall t.\; P(t)$ for some predicate $P$

These are exactly first-order formulas checked by the database engine on every update. If an update would violate a constraint, it is rejected.

## Deductive Databases and Knowledge Graphs

**Deductive databases** add rule-based inference: given facts, derive new facts using logical rules. Datalog is the canonical language. Applications:
- **Semantic web**: OWL (Web Ontology Language) is a description logic for knowledge representation
- **Knowledge graphs**: RDF/SPARQL databases with inferencing
- **Prolog databases**: Medical diagnosis, legal reasoning systems

**SPARQL and OWL**: The W3C stack for the Semantic Web uses description logics (fragments of FOL) for ontologies and SPARQL (itself a logic-based query language) for querying RDF graphs.

## Exercises
See [problems/ch13_applications/](../../../problems/ch13_applications/)

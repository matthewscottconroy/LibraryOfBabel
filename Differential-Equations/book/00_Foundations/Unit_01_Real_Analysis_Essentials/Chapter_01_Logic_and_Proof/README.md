# Chapter 01: Logic and Proof

Mathematics is distinguished from other disciplines not only by its subject matter but by its method: the proof. A proof is a finite sequence of statements, each one either an axiom or a logical consequence of earlier statements, culminating in the assertion to be proved. Before any theorem about real numbers, sequences, or differential equations can be stated precisely — let alone proved — a shared language of logic is required. This chapter establishes that language.

## Logical Structure and Proof Strategies

The chapter opens with propositional logic: the study of statements that are either true or false, and of the connectives — negation, conjunction, disjunction, implication, biconditional — that combine them. The key object is the implication $P \Rightarrow Q$, read "if $P$ then $Q$." Almost every theorem in analysis has this form, where $P$ is the hypothesis and $Q$ is the conclusion. Understanding what makes such a statement true, and what it means to prove one, is the entry point to all subsequent work.

From propositional logic, the chapter moves to predicate logic, where statements depend on variables: "$x > 0$" is neither true nor false until $x$ is specified. Universal quantifiers ("for all $x$, $P(x)$") and existential quantifiers ("there exists $x$ such that $P(x)$") allow precise statements about entire collections of objects at once. The epsilon-delta definition of a limit — $\forall \varepsilon > 0, \exists \delta > 0$ such that... — is one of the most important quantified statements in analysis, and Chapter 3 returns to it in full.

## Proof Techniques

The four core proof strategies are introduced and practiced here.

A **direct proof** of $P \Rightarrow Q$ assumes $P$ and derives $Q$ through a chain of valid inferences. Most proofs encountered early in analysis are direct proofs, and skill at constructing them comes from practice on concrete examples.

A **proof by contrapositive** observes that $P \Rightarrow Q$ is logically equivalent to $\neg Q \Rightarrow \neg P$. When the negation of the conclusion provides more useful information than the hypothesis itself, the contrapositive form is often easier to work with. For instance, showing that a function with $f'(x) \neq 0$ everywhere is injective is more naturally handled as a contrapositive.

A **proof by contradiction** assumes $\neg Q$ (in addition to $P$) and derives a logical impossibility. The assumption $\neg Q$ is then rejected, leaving $Q$ as the only consistent option. The proof that $\sqrt{2}$ is irrational is the canonical example.

**Mathematical induction** proves statements of the form $\forall n \in \mathbb{N}, P(n)$. The base case verifies $P(1)$ (or $P(0)$); the inductive step shows that $P(k) \Rightarrow P(k+1)$ for arbitrary $k$. Strong induction allows the assumption that $P(j)$ holds for all $j \leq k$, not just for $j = k$.

## The Three Sections

**Section 1 — Propositional Logic** covers the syntax and semantics of logical connectives, truth tables, tautologies, and logical equivalences. Special attention is given to the equivalences that are most used in proofs: De Morgan's laws ($\neg(P \land Q) \equiv \neg P \lor \neg Q$), the equivalence of $P \Rightarrow Q$ with $\neg P \lor Q$, and the contrapositive equivalence.

**Section 2 — Quantifiers and Predicates** introduces first-order logic, develops the rules for negating quantified statements ($\neg \forall x, P(x)$ is $\exists x, \neg P(x)$), and builds skill in parsing complex quantifier strings. The ability to negate the statement "$\forall \varepsilon > 0, \exists N$ such that for all $n > N$, $|a_n - L| < \varepsilon$" correctly — which yields "$\exists \varepsilon > 0$ such that $\forall N$, there exists $n > N$ with $|a_n - L| \geq \varepsilon$" — is directly needed in Chapter 3 when divergence of sequences is characterized.

**Section 3 — Proof Techniques** puts the strategies to work on a collection of concrete propositions drawn from number theory, combinatorics, and elementary real analysis. By the end of the section, students should be comfortable selecting a proof strategy, carrying it through, and writing a clear, complete proof in the accepted mathematical style.

## Connection to the Rest of the Unit

Every subsequent chapter depends on this one. The statement of the Completeness Axiom in Chapter 2 uses quantifiers; so does every epsilon-delta argument in Chapter 3. The convergence tests of Chapter 4 are theorems with hypotheses and conclusions that must be applied correctly. The Mean Value Theorem and the Fundamental Theorem of Calculus are implications whose careful application requires understanding exactly what the hypotheses demand. Students who treat this chapter as optional do so at their peril: the precise mathematical language introduced here is the medium in which all subsequent reasoning takes place.

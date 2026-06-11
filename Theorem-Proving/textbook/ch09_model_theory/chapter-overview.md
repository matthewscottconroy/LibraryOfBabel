# Chapter 9 Overview: Model Theory

---

## Central Question

What is the relationship between a theory (a set of axioms) and its models (the structures that satisfy those axioms)? Can a theory have many different models? Can it have only one (up to isomorphism)? What do the models of a theory tell us about the theory's provability?

---

## Why This Chapter Matters

Model theory is the branch of logic that studies structures directly, using logic to compare and classify them. Its tools — compactness, the Löwenheim-Skolem theorems, and the notion of elementary equivalence — have deep consequences for the limits of axiomatisation, for the existence of non-standard models of arithmetic, and for understanding what "meaning" means in formal systems.

---

## Key Definitions

**Structure.** A structure $\mathcal{M}$ for a signature $\sigma$ consists of a non-empty domain $M$ together with interpretations of all symbols in $\sigma$ (as defined in Chapter 3).

**Theory.** A *theory* is a set of sentences. A structure $\mathcal{M}$ is a *model* of theory $T$ (written $\mathcal{M} \vDash T$) if $\mathcal{M}$ satisfies every sentence in $T$.

**Elementary equivalence.** Two structures $\mathcal{M}$ and $\mathcal{N}$ are *elementarily equivalent* (written $\mathcal{M} \equiv \mathcal{N}$) if they satisfy exactly the same first-order sentences: $\mathcal{M} \vDash \phi \iff \mathcal{N} \vDash \phi$ for all sentences $\phi$.

**Isomorphism.** $\mathcal{M}$ and $\mathcal{N}$ are *isomorphic* (written $\mathcal{M} \cong \mathcal{N}$) if there is a bijection $h: M \to N$ preserving all operations and relations.

**Elementary embedding.** A function $h: M \to N$ is an *elementary embedding* if for every first-order formula $\phi(x_1, \ldots, x_n)$ and all $a_1, \ldots, a_n \in M$: $\mathcal{M} \vDash \phi(a_1, \ldots, a_n) \iff \mathcal{N} \vDash \phi(h(a_1), \ldots, h(a_n))$.

**Complete theory.** A theory $T$ is *complete* if for every sentence $\phi$, either $T \vDash \phi$ or $T \vDash \neg\phi$ (but not both).

**Categorical theory.** A theory $T$ is $\kappa$-categorical if it has exactly one model of cardinality $\kappa$ up to isomorphism.

---

## Main Theorems

### Compactness Theorem

**Theorem.** A set of sentences $T$ is satisfiable if and only if every finite subset of $T$ is satisfiable.

**Proof sketch.** The "only if" direction is trivial. For the "if" direction: by completeness (Gödel's Completeness Theorem), $T$ is unsatisfiable iff $T \vdash \bot$. But any proof is finite and uses only finitely many premises; so if $T \vdash \bot$, some finite subset of $T$ proves $\bot$, contradicting finite satisfiability. $\square$

**Applications:**

*Non-standard models of arithmetic:* Add to Peano arithmetic $T_{PA}$ the constants $c_0, c_1, c_2, \ldots$ and the sentences $\{c_0 < c_1, c_1 < c_2, c_2 < c_3, \ldots\} \cup \{\overline{n} < c_0 : n \in \mathbb{N}\}$ (where $\overline{n}$ is the numeral for $n$). Every finite subset of this extended theory has a model (take the standard $\mathbb{N}$ with $c_i$ interpreted as large enough naturals). By compactness, the whole theory has a model — an *non-standard* model of arithmetic containing infinite natural numbers.

*Upward closure:* If $T$ has infinite models of all finite sizes, it has infinite models of all infinite cardinalities. (Add $n$ distinct constants for each finite $n$; compactness gives an infinite model.)

### Löwenheim-Skolem Theorem (Downward Direction)

**Theorem (Löwenheim 1915, Skolem 1920).** If a countable theory $T$ has an infinite model, then it has a countable model.

**Proof sketch.** Given an infinite model $\mathcal{M}$, construct a countable elementary substructure by the Skolem-Henkin construction: starting from any countable set $A_0 \subseteq M$, iteratively close under Skolem functions (functions selecting witnesses for existential formulas). The result $\bigcup_n A_n$ is countable and is the domain of an elementary substructure. $\square$

### Löwenheim-Skolem Theorem (Upward Direction, Tarski)

**Theorem.** If a countable theory $T$ has an infinite model of cardinality $\kappa$, then it has models of all infinite cardinalities $\lambda \geq \kappa$.

**Proof sketch.** Add $\lambda$ many new constants to the language and sentences asserting they are distinct. By compactness, this extended theory has a model. $\square$

**Skolem's Paradox:** ZFC set theory (which is a first-order theory) has a countable model if it has any model at all. But ZFC proves "there are uncountably many real numbers." In the countable model, "the real numbers" is a set in the model's universe, but this set is countable from *outside* the model. Within the model, it appears uncountable because the model lacks the bijection that the outside observer can see.

This is not a contradiction; it reveals that "uncountable" is a relative concept, defined within the model.

### Vaught's Theorem (Completeness from Categoricity)

**Theorem (Vaught 1954).** If a countable theory $T$ with no finite models is $\kappa$-categorical for some infinite $\kappa$, then $T$ is complete.

**Proof sketch.** Suppose $T$ is $\kappa$-categorical and let $\phi$ be any sentence. We show $T \vDash \phi$ or $T \vDash \neg\phi$. Assume not: then $T \cup \{\phi\}$ and $T \cup \{\neg\phi\}$ are both consistent. By the Löwenheim-Skolem theorem, both have models of cardinality $\kappa$. But these two models disagree on $\phi$, contradicting $\kappa$-categoricity ($T$ has only one model of cardinality $\kappa$ up to isomorphism, hence they would have to agree on all sentences). $\square$

**Application:** The theory of dense linear orders without endpoints (DLO) is $\aleph_0$-categorical (every two countable such orders are isomorphic to $\mathbb{Q}$), hence complete.

---

## Non-Standard Models and Their Consequences

The existence of non-standard models has several important consequences:

**Non-standard natural numbers.** Any model of PA (Peano Arithmetic) that is not isomorphic to $\mathbb{N}$ contains "infinite" numbers — elements greater than all standard naturals $\overline{0}, \overline{1}, \overline{2}, \ldots$. Importantly, these satisfy all the first-order properties of natural numbers that PA can express.

**Non-standard analysis.** Abraham Robinson (1961) used compactness to construct hyperreal numbers containing actual infinitesimals. In this framework, the intuitive "infinitely small" $\varepsilon$ from Leibniz's calculus is made rigorous.

**Definability and undefinability.** Model theory is a tool for proving that certain properties *cannot* be expressed in first-order logic. For example, "the set of all even natural numbers" is not definable in the first-order language of arithmetic with just the successor function (by an Ehrenfeucht-Fraïssé argument).

---

## Historical Context

**Leopold Löwenheim (1915)** proved the downward theorem for countable signatures, launching model theory as a discipline distinct from proof theory.

**Thoralf Skolem (1920)** strengthened Löwenheim's result and — in a now-famous move — argued that set theory therefore had unintended (non-standard) countable models, raising deep questions about the meaning of mathematical foundations. "Skolem's paradox" is named for this observation.

**Alfred Tarski (1935, 1936)** developed the rigorous definition of truth in a structure (semantic satisfaction) and the notion of elementary equivalence. He also proved the upward Löwenheim-Skolem theorem and the undefinability of truth (Chapter 10).

**Abraham Robinson (1961)** invented nonstandard analysis using model-theoretic compactness, giving the first rigorous foundation for infinitesimals since Newton and Leibniz.

**Michael Morley (1965)** proved the Morley categoricity theorem: if a countable theory is $\kappa$-categorical for any uncountable $\kappa$, it is $\kappa$-categorical for all uncountable $\kappa$. This launched a deep classification programme for theories.

---

## Connections to Other Chapters

- **Chapter 3** provides the FOL syntax and semantics that model theory studies.
- **Chapter 6** provides the set-theoretic framework for structures.
- **Chapter 10** uses model-theoretic ideas (truth in models, definability) in the proof of incompleteness.

# Chapter 3 Overview: First-Order Logic

---

## Central Question

Propositional logic can express "it is raining" but not "all humans are mortal." To reason about *all* elements of a domain, about *existence*, and about *relations between objects*, we need first-order logic (FOL) — the language in which virtually all of modern mathematics is written.

---

## Why This Chapter Matters

First-order logic is the *lingua franca* of formal mathematics. Peano arithmetic, Zermelo-Fraenkel set theory, and every axiom system used in practice are first-order theories. Understanding FOL's syntax, semantics, and proof theory is prerequisite for all of Chapter 4 (proof systems), Chapter 6 (set theory), Chapter 9 (model theory), and Chapter 10 (computability).

---

## Key Definitions

**Signature / Vocabulary.** A first-order signature $\sigma$ consists of:
- *Constant symbols*: $c_0, c_1, \ldots$ (zero-argument functions)
- *Function symbols*: $f^{(n)}, g^{(n)}, \ldots$ each with a specified arity $n \geq 1$
- *Relation/Predicate symbols*: $R^{(n)}, P^{(n)}, \ldots$ each with a specified arity $n \geq 0$

**First-order terms** over $\sigma$ with variables $x_0, x_1, \ldots$:
- Any variable $x_i$ is a term.
- If $t_1, \ldots, t_n$ are terms and $f$ is an $n$-ary function symbol, then $f(t_1, \ldots, t_n)$ is a term.
- Constants are 0-ary function symbols: $c$ is a term.

**First-order formulas** (wffs) over $\sigma$:
- If $t_1, \ldots, t_n$ are terms and $R$ is an $n$-ary relation symbol, then $R(t_1, \ldots, t_n)$ is an *atomic formula*.
- If $t_1, t_2$ are terms, $t_1 = t_2$ is an atomic formula (if equality is present).
- $\bot$, $\top$ are atomic formulas.
- If $\phi, \psi$ are formulas, so are $\neg\phi$, $(\phi \land \psi)$, $(\phi \lor \psi)$, $(\phi \to \psi)$, $(\phi \leftrightarrow \psi)$.
- If $\phi$ is a formula and $x$ is a variable, then $\forall x\, \phi$ and $\exists x\, \phi$ are formulas.

**Free vs. bound variables.** A variable occurrence is *bound* if it is within the scope of a quantifier binding it; otherwise it is *free*. A *sentence* is a formula with no free variables.

**Structure / Model.** A structure $\mathcal{M} = (M, \sigma^{\mathcal{M}})$ for signature $\sigma$ consists of:
- A non-empty domain $M$ (the "universe")
- An interpretation of each constant as an element of $M$, each function symbol as a function $M^n \to M$, and each relation symbol as a subset of $M^n$

**Satisfaction.** Given a structure $\mathcal{M}$, a variable assignment $s: \text{Var} \to M$, and a formula $\phi$, the satisfaction relation $\mathcal{M}, s \vDash \phi$ is defined recursively. A sentence $\phi$ is true in $\mathcal{M}$ (written $\mathcal{M} \vDash \phi$) if $\mathcal{M}, s \vDash \phi$ for any (equivalently, every) assignment $s$.

**Theory.** A first-order theory is a set of sentences closed under logical consequence. Examples: the theory of groups, Peano arithmetic, ZFC set theory.

---

## Herbrand's Theorem

**Theorem (Herbrand 1930).** A set of sentences $\Gamma$ (in prenex form without equality) is unsatisfiable if and only if there is a finite unsatisfiable set of ground instances of $\Gamma$ — instances obtained by substituting terms from the Herbrand universe (terms built from constants and function symbols of $\Gamma$) for variables.

**Why it matters:** Herbrand's theorem provides the theoretical foundation for automated theorem proving. To refute $\Gamma$, it suffices to search for a finite propositional unsatisfiable set of ground instances. This search may not terminate (FOL is only semi-decidable), but if $\Gamma$ is unsatisfiable, it will eventually succeed.

*Proof sketch.* The key idea is the Herbrand universe: the set of all ground terms over the signature. The *Herbrand model* assigns each ground atom its truth value based on whether it is derivable. If $\Gamma$ has no finite propositional refutation, the Herbrand model satisfies $\Gamma$. Contrapositive: if $\Gamma$ is unsatisfiable, there must be a finite propositional refutation.

---

## Unification

**Unification problem.** Given terms $s$ and $t$ (possibly with variables), find a *most general unifier* (MGU) — a substitution $\sigma$ such that $\sigma(s) = \sigma(t)$, if one exists.

**Algorithm (Robinson 1965).** Unification is decidable and the MGU is unique (up to renaming) when it exists:

1. If $s$ and $t$ are both variables and equal: return identity.
2. If $s$ is a variable not occurring in $t$: return $\{s \mapsto t\}$ (after occurs check).
3. If $t$ is a variable not occurring in $s$: return $\{t \mapsto s\}$.
4. If $s = f(s_1, \ldots, s_n)$ and $t = f(t_1, \ldots, t_n)$: recursively unify pairs $(s_i, t_i)$.
5. Otherwise: fail.

**The occurs check.** The substitution $\{x \mapsto f(x)\}$ is not a valid unifier because it would create an infinite term. The occurs check prevents this. (Many Prolog implementations skip it for efficiency, leading to unsound behaviour.)

**Application to proof search.** In resolution-based FOL provers, unification determines when two clauses can be resolved: the positive literal $P(s_1, \ldots, s_n)$ in one clause and the negative literal $\neg P(t_1, \ldots, t_n)$ in another can be resolved if $s_i$ and $t_i$ unify.

---

## Decidability and Semi-Decidability

**Theorem (Church, Turing 1936).** First-order logic is undecidable: there is no algorithm that, given a sentence $\phi$, decides whether $\phi$ is a tautology (valid in all structures).

**Theorem (semi-decidability of validity).** First-order logic is semi-decidable: there is an algorithm that, given a valid sentence $\phi$, eventually outputs "yes," but may run forever on non-valid sentences.

**Theorem (semi-decidability of unsatisfiability).** There is an algorithm (e.g., resolution with Herbrand's theorem) that, given an unsatisfiable set $\Gamma$, eventually derives the empty clause.

*Proof sketch for undecidability:* Turing machines can be simulated in first-order logic (encoding halting configurations as finite structures). Whether a Turing machine halts is undecidable; therefore whether the corresponding FOL sentence is satisfiable is undecidable.

---

## Decidable Fragments

Not all of FOL is undecidable. Important decidable fragments:

- **Monadic predicates (no function symbols):** Decidable (the "monadic" class).
- **Universal fragment without function symbols ($\forall^* \exists^0$):** Decidable (the AE class).
- **Two-variable logic $L^2$:** Decidable (Mortimer 1975; key for description logics).
- **Guarded fragment:** Decidable, captures many modal and description logics.

Undecidable by standard reduction:

- **Three or more variables:** Undecidable.
- **One binary relation:** Undecidable (if non-trivially used).

---

## Historical Context

**Gottlob Frege (1879)** introduced quantifiers in the *Begriffsschrift*, giving the first complete formal system for FOL (though in awkward 2D notation). Frege's system was essentially equivalent to what we today call first-order logic with equality.

**Charles Sanders Peirce and Ernst Schröder (1880s–1890s)** independently developed algebraic treatments of quantification. Their notation was more accessible than Frege's but less carefully axiomatised.

**Giuseppe Peano (1889)** introduced modern mathematical notation (∀, ∃, ∈) and the axioms for arithmetic that bear his name, formulated in a language equivalent to FOL.

**Leopold Löwenheim (1915)** proved the first model-theoretic theorem: if a sentence in a finite or countable signature has a model, it has a countable model. This initiated model theory (Chapter 9).

**Thoralf Skolem (1920)** strengthened Löwenheim's result and introduced *Skolemisation* — eliminating existential quantifiers by introducing new function symbols. The Löwenheim-Skolem theorem (Chapter 9) bears their names.

**Jacques Herbrand (1930)** proved his fundamental theorem reducing FOL satisfiability to propositional satisfiability, enabling automated proof search.

**Alan Robinson (1965)** introduced *resolution with unification*, the foundation of logic programming (Prolog) and automated theorem proving.

---

## Connections to Other Chapters

- **Chapter 2** is propositional logic; Chapter 3 extends it with quantifiers and terms.
- **Chapter 4** develops proof systems for FOL (natural deduction for FOL, sequent calculus).
- **Chapter 9** studies the model theory of FOL: which theories have unique models? Which have many?
- **Chapter 10** uses FOL to encode computability theory: the halting problem and Rice's theorem have natural FOL formulations.
- **Chapter 11** (Type Theory): the Curry-Howard correspondence relates FOL to intuitionistic logic and simply typed λ-calculus.

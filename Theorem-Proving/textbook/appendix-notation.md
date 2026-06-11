# Appendix: Notation Reference

This appendix provides a comprehensive reference for all symbols and notation used across the chapters. Entries are grouped by topic; each entry gives the symbol, its name, its meaning, the chapter(s) where it is introduced, and relevant notes.

---

## 1. Propositional Connectives

| Symbol | Name | Meaning | Chapter |
|--------|------|---------|---------|
| $\neg\phi$ | Negation / not | "It is not the case that $\phi$" | 2 |
| $\phi \land \psi$ | Conjunction / and | "$\phi$ and $\psi$ are both true" | 2 |
| $\phi \lor \psi$ | Disjunction / or | "At least one of $\phi$, $\psi$ is true" | 2 |
| $\phi \to \psi$ | Conditional / implies | "If $\phi$ then $\psi$"; false only when $\phi$ true and $\psi$ false | 2 |
| $\phi \leftrightarrow \psi$ | Biconditional / iff | "$\phi$ if and only if $\psi$"; true when both have the same truth value | 2 |
| $\top$ | Truth / verum | The constantly true formula | 2 |
| $\bot$ | Falsity / falsum | The constantly false formula; also used as the absurdity constant | 2 |
| $\phi \mid \psi$ | Sheffer stroke / NAND | "Not both $\phi$ and $\psi$"; functionally complete alone | 2 |
| $\phi \downarrow \psi$ | Peirce arrow / NOR | "Neither $\phi$ nor $\psi$"; functionally complete alone | 2 |

**Precedence (high to low):** $\neg > \land > \lor > \to > \leftrightarrow$

So $\neg p \land q \lor r \to s$ is parsed as $((\neg p) \land q) \lor r) \to s$.

---

## 2. First-Order Quantifiers and Variables

| Symbol | Name | Meaning | Chapter |
|--------|------|---------|---------|
| $\forall x\, \phi(x)$ | Universal quantifier | "For all $x$, $\phi(x)$" | 3 |
| $\exists x\, \phi(x)$ | Existential quantifier | "There exists an $x$ such that $\phi(x)$" | 3 |
| $\exists! x\, \phi(x)$ | Unique existence | "There exists exactly one $x$ such that $\phi(x)$" | 3 |
| $\iota x\, \phi(x)$ | Definite description | "The unique $x$ such that $\phi(x)$" (undefined if not unique) | 3 |
| $x, y, z, \ldots$ | Individual variables | Singular first-order variables ranging over the domain | 3 |
| $f, g, h$ | Function symbols | Denote functions from $M^n \to M$ | 3 |
| $P, Q, R$ | Relation symbols | Denote subsets of $M^n$ | 3 |
| $c, d$ | Constant symbols | Denote elements of $M$ | 3 |
| $\phi[t/x]$ | Substitution | Formula $\phi$ with term $t$ substituted for all free occurrences of $x$ | 3 |
| $\text{FV}(\phi)$ | Free variables | The set of variables with free occurrences in $\phi$ | 3 |

**Note.** $\forall x\, \phi \to \psi$ is ambiguous; conventions differ. We use $(\forall x\, \phi) \to \psi$ vs. $\forall x\, (\phi \to \psi)$ with explicit parentheses when ambiguous.

---

## 3. Proof-Theoretic Notation

| Symbol | Name | Meaning | Chapter |
|--------|------|---------|---------|
| $\Gamma \vdash \phi$ | Provability / syntactic consequence | "$\phi$ is provable from hypotheses $\Gamma$ in the proof system" | 4 |
| $\vdash \phi$ | Theorem | "$\phi$ is provable from no hypotheses" | 4 |
| $\Gamma \vDash \phi$ | Semantic consequence / entailment | "$\phi$ is true in every model of $\Gamma$" | 2, 4 |
| $\vDash \phi$ | Tautology / validity | "$\phi$ is true in every model" | 2 |
| $\mathcal{M} \vDash \phi$ | Satisfaction | "Structure $\mathcal{M}$ satisfies formula $\phi$" | 3, 9 |
| $\{P\}\ C\ \{Q\}$ | Hoare triple | "If $P$ holds before $C$ and $C$ terminates, then $Q$ holds after" | 13 |
| $[P]\ C\ [Q]$ | Total correctness triple | "If $P$ holds before $C$, then $C$ terminates and $Q$ holds after" | 13 |
| $\frac{\phi_1 \quad \phi_2}{\psi}$ Rule | Inference rule | From premises $\phi_1, \phi_2$, conclude $\psi$ | 4 |
| $[\phi]$ | Discharged hypothesis | Hypothesis $\phi$ introduced and cancelled by a rule | 4 |

**Sequent calculus notation:**

| Symbol | Meaning |
|--------|---------|
| $\Gamma \vdash \Delta$ | From $\Gamma$ (conjunction), derive $\Delta$ (disjunction) |
| $\Gamma, \phi$ | $\Gamma$ extended with $\phi$ on the left |
| $\Delta, \phi$ | $\Delta$ extended with $\phi$ on the right |

---

## 4. Set Notation

| Symbol | Name | Meaning | Chapter |
|--------|------|---------|---------|
| $a \in A$ | Membership | "$a$ is an element of $A$" | 6 |
| $a \notin A$ | Non-membership | "$a$ is not an element of $A$" | 6 |
| $A \subseteq B$ | Subset | Every element of $A$ is in $B$ | 6 |
| $A \subsetneq B$ | Proper subset | $A \subseteq B$ and $A \neq B$ | 6 |
| $A = B$ | Set equality | $A \subseteq B$ and $B \subseteq A$ | 6 |
| $\emptyset$ or $\varnothing$ | Empty set | The set with no elements | 6 |
| $\{a, b, c\}$ | Finite set | The set containing exactly $a$, $b$, $c$ | 6 |
| $\{x : \phi(x)\}$ or $\{x \mid \phi(x)\}$ | Set comprehension | The set of all $x$ satisfying $\phi$ (requires separation axiom) | 6 |
| $\{x \in A : \phi(x)\}$ | Restricted comprehension | The set of elements of $A$ satisfying $\phi$ | 6 |
| $A \cup B$ | Union | $\{x : x \in A \lor x \in B\}$ | 6 |
| $A \cap B$ | Intersection | $\{x : x \in A \land x \in B\}$ | 6 |
| $A \setminus B$ | Set difference | $\{x \in A : x \notin B\}$ | 6 |
| $A^c$ or $\overline{A}$ | Complement | $\{x : x \notin A\}$ (relative to a fixed universe) | 6 |
| $\mathcal{P}(A)$ or $2^A$ | Power set | The set of all subsets of $A$ | 6 |
| $\bigcup \mathcal{F}$ | Generalised union | $\{x : \exists A \in \mathcal{F}, x \in A\}$ | 6 |
| $\bigcap \mathcal{F}$ | Generalised intersection | $\{x : \forall A \in \mathcal{F}, x \in A\}$ | 6 |
| $A \times B$ | Cartesian product | $\{(a, b) : a \in A \land b \in B\}$ | 6 |
| $A^n$ | $n$-fold product | $A \times A \times \cdots \times A$ ($n$ times) | 6 |
| $(a, b)$ | Ordered pair | Represented set-theoretically as $\{\{a\}, \{a, b\}\}$ (Kuratowski) | 6 |
| $|A|$ or $\#A$ | Cardinality | The cardinality (size) of $A$ | 6 |
| $\omega$ | Omega (first infinite ordinal) | The set of natural numbers $\{0, 1, 2, \ldots\}$ as an ordinal | 6 |
| $\aleph_0$ | Aleph-null | Cardinality of countably infinite sets ($|\omega|$) | 6 |
| $\aleph_1$ | Aleph-one | The first uncountable cardinal | 6 |
| $\mathfrak{c}$ | Continuum | Cardinality of the real numbers $|\mathbb{R}| = |\mathcal{P}(\mathbb{N})|$ | 6 |
| $\alpha, \beta, \gamma$ | Ordinal variables | Convention: lowercase Greek for ordinals | 6 |
| $\kappa, \lambda, \mu$ | Cardinal variables | Convention: lowercase Greek for cardinals | 6 |

---

## 5. Type-Theoretic Notation

| Symbol | Name | Meaning | Chapter |
|--------|------|---------|---------|
| $t : A$ | Typing judgement | "Term $t$ has type $A$" | 11 |
| $\Gamma \vdash t : A$ | Typing in context | "In context $\Gamma$, term $t$ has type $A$" | 11 |
| $\lambda x : A.\, t$ | Lambda abstraction | A function mapping $x : A$ to $t$ | 11 |
| $t_1\ t_2$ | Application | Apply $t_1$ to $t_2$ | 11 |
| $A \to B$ | Function type | Type of functions from $A$ to $B$ | 11 |
| $\Pi x : A.\, B(x)$ | Dependent product | Type of functions $f$ with $f(a) : B(a)$ for all $a : A$; also written $(x : A) \to B(x)$ | 11 |
| $\Sigma x : A.\, B(x)$ | Dependent sum | Type of pairs $(a, b)$ with $a : A$ and $b : B(a)$; also written $\exists x : A, B(x)$ in Lean | 11 |
| $\text{Prop}$ | Proposition universe | The type of propositions (proof-irrelevant in Lean 4 / Coq) | 11 |
| $\text{Type}_i$ | Type universe | The $i$-th type universe; $\text{Type}_i : \text{Type}_{i+1}$ | 11 |
| $\text{Sort}$ | Sort universe | Overarching universe; in Lean 4, $\text{Prop}$ and $\text{Type}$ are sorts | 11 |
| $\Lambda\alpha.\, t$ | Type abstraction | System F universal type abstraction | 11 |
| $\forall\alpha.\, A$ | Universal type | System F: for all types $\alpha$, type $A$ | 11 |
| $\text{rec}$ or $\text{elim}$ | Recursor / eliminator | The elimination principle for an inductive type | 11 |
| $a \equiv b$ | Definitional equality | $a$ and $b$ reduce to the same normal form | 11 |
| $a = b$ | Propositional equality | There is a proof that $a$ equals $b$ (the equality type) | 11 |
| $\text{refl}$ | Reflexivity proof | $\text{refl} : a = a$ | 11 |

---

## 6. Modal and Temporal Operators

| Symbol | Name | Meaning | Chapter |
|--------|------|---------|---------|
| $\Box\phi$ | Box / Necessarily | "$\phi$ is necessary" / "in all accessible worlds, $\phi$" | 12 |
| $\Diamond\phi$ | Diamond / Possibly | "$\phi$ is possible" / "in some accessible world, $\phi$" | 12 |
| $K_i\phi$ | Knowledge | "Agent $i$ knows that $\phi$" (epistemic logic) | 12 |
| $B_i\phi$ | Belief | "Agent $i$ believes that $\phi$" (doxastic logic) | 12 |
| $O\phi$ | Obligation | "$\phi$ is obligatory" (deontic logic) | 12 |
| $(W, R)$ | Kripke frame | Set of worlds $W$ with accessibility relation $R$ | 12 |
| $wRv$ | Accessibility | "World $v$ is accessible from world $w$" | 12 |
| $X\phi$ | Next | "In the next state, $\phi$" (LTL) | 14 |
| $F\phi$ | Finally / Eventually | "At some future state, $\phi$" ($F\phi \equiv \top U \phi$) | 14 |
| $G\phi$ | Globally / Always | "At all future states, $\phi$" ($G\phi \equiv \neg F\neg\phi$) | 14 |
| $\phi\ U\ \psi$ | Until | "$\phi$ holds until $\psi$ eventually holds" | 14 |
| $\phi\ W\ \psi$ | Weak until | "$\phi$ holds until $\psi$, or $\phi$ holds forever" | 14 |
| $\phi\ R\ \psi$ | Release | Dual of Until: $\neg(\neg\phi\ U\ \neg\psi)$ | 14 |
| $AX\phi$ | All-next | "In all immediate next states, $\phi$" (CTL) | 14 |
| $EX\phi$ | Exists-next | "In some immediate next state, $\phi$" (CTL) | 14 |
| $AF\phi$ | All-finally | "On all paths, eventually $\phi$" (CTL) | 14 |
| $EF\phi$ | Exists-finally | "On some path, eventually $\phi$" (CTL) | 14 |
| $AG\phi$ | All-globally | "On all paths, always $\phi$" (CTL) | 14 |
| $EG\phi$ | Exists-globally | "On some path, always $\phi$" (CTL) | 14 |
| $A[\phi\ U\ \psi]$ | All-until | "On all paths, $\phi$ until $\psi$" (CTL) | 14 |
| $E[\phi\ U\ \psi]$ | Exists-until | "On some path, $\phi$ until $\psi$" (CTL) | 14 |

---

## 7. Plural Logic Notation

| Symbol | Name | Meaning | Chapter |
|--------|------|---------|---------|
| $xx, yy, zz$ | Plural variables | Variables ranging over pluralities (one or more objects) | 15 |
| $x \prec xx$ | Inclusion | "$x$ is one of the $xx$" | 15 |
| $\exists xx\, \phi$ | Plural existential | "There are some things $xx$ such that $\phi$" | 15 |
| $\forall xx\, \phi$ | Plural universal | "For any things $xx$, $\phi$" | 15 |
| $XX, YY$ | Superplural variables | Variables ranging over pluralities of pluralities | 15 |

---

## 8. Number-Theoretic Notation

| Symbol | Name | Meaning | Chapter |
|--------|------|---------|---------|
| $a \mid b$ | Divides | "$a$ divides $b$"; $\exists k \in \mathbb{Z}, b = ak$ | 8 |
| $\gcd(a, b)$ | Greatest common divisor | Largest positive integer dividing both $a$ and $b$ | 8 |
| $\text{lcm}(a, b)$ | Least common multiple | Smallest positive integer divisible by both | 8 |
| $a \equiv b \pmod{m}$ | Congruence modulo $m$ | $m \mid (a - b)$ | 8 |
| $\mathbb{Z}/m\mathbb{Z}$ | Integers modulo $m$ | The ring of congruence classes mod $m$ | 8 |
| $\varphi(n)$ | Euler's totient | Number of integers in $\{1, \ldots, n\}$ coprime to $n$ | 8 |
| $[a]_m$ | Congruence class | The congruence class of $a$ modulo $m$ | 8 |
| $(\mathbb{Z}/m\mathbb{Z})^*$ | Unit group | Multiplicative group of units (coprime to $m$) in $\mathbb{Z}/m\mathbb{Z}$ | 8 |

---

## 9. Computability Notation

| Symbol | Name | Meaning | Chapter |
|--------|------|---------|---------|
| $\phi_e$ | $e$-th partial computable function | The partial function computed by TM with index $e$ | 10 |
| $\phi_e(n)\downarrow$ | Convergence | Machine $e$ halts on input $n$ | 10 |
| $\phi_e(n)\uparrow$ | Divergence | Machine $e$ does not halt on input $n$ | 10 |
| $W_e$ | Domain of $\phi_e$ | $W_e = \{n : \phi_e(n)\downarrow\}$ | 10 |
| $K$ | Halting set | $K = \{e : \phi_e(e)\downarrow\}$ | 10 |
| $\ulcorner\phi\urcorner$ | Gödel number | The Gödel number encoding of formula (or proof) $\phi$ | 10 |
| $Pr_T(x)$ | Provability predicate | "The formula with Gödel number $x$ is provable in $T$" | 10 |
| $Con(T)$ | Consistency statement | $\neg Pr_T(\ulcorner\bot\urcorner)$ — "$T$ is consistent" | 10 |
| $G_T$ | Gödel sentence | The self-referential sentence asserting its own unprovability | 10 |
| $A \leq_m B$ | Many-one reducibility | $A$ reduces to $B$: there is a total computable $f$ with $n \in A \iff f(n) \in B$ | 10 |
| $A \equiv_m B$ | Many-one equivalence | $A \leq_m B$ and $B \leq_m A$ | 10 |

---

## 10. Conventions Across the Text

**Metavariables.**
- $\phi, \psi, \chi$: propositional or first-order formulas
- $\Gamma, \Delta, \Pi$: sets (or sequences) of formulas (contexts)
- $A, B, C$: types (in type theory chapters)
- $t, s, u, v$: terms or programs
- $\mathcal{M}, \mathcal{N}$: structures/models
- $M, N$: domains of structures

**Abbreviations used in proofs.**
- "iff" = "if and only if" (biconditional)
- "w.l.o.g." = "without loss of generality"
- "a.e." = "almost everywhere" (measure zero exception)
- "t.f.a.e." = "the following are equivalent"
- "$\square$" or "QED" ends a proof
- "$\square$" inside a proof marks the end of a subproof

**Lean 4 / Coq notation.**
- `→` (Unicode) or `->` (ASCII) for the function type / implication
- `∀` or `forall` for dependent products
- `∃` or `exists` for dependent sums
- `:=` for definitions
- `by` introduces a tactic block
- `#check t` displays the type of term `t`
- `#eval t` evaluates `t` as a computation

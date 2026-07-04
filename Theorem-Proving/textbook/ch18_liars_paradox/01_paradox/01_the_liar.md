# The Liar's Paradox

Epimenides of Crete said: "All Cretans are liars." Eubulides of Miletus (4th century BCE) sharpened the puzzle into the *pseudomenon*; Chrysippus wrote six books on it (all lost); the medieval *insolubilia* tradition wrestled with it for centuries. The modern, sharpest form is:

> **L**: "This sentence is false."

Suppose $L$ is true. Then what $L$ says holds: $L$ is false. Suppose $L$ is false. Then $L$ says something false — but $L$ says *it is false*, so it is true. Either way, contradiction. $L$ is neither true nor false — or both — or something else entirely. This chapter is about making that "something else" precise.

The Liar is not a curiosity. It rests on exactly two assumptions, each individually innocent:

1. **The T-schema**: for every sentence $\phi$, $T(\ulcorner\phi\urcorner) \leftrightarrow \phi$ — "$\phi$ is true" if and only if $\phi$.
2. **Self-reference**: the language contains sentences that talk about their own truth.

Together they are inconsistent. The rest of this section shows that (2) is not an eliminable trick of natural language but a *theorem* about formal systems, and derives the consequence: no sufficiently expressive language can define its own truth predicate.

## Making Self-Reference Precise: Gödel Numbering

Fix a first-order theory $T$ extending Robinson arithmetic $\mathsf{Q}$ (Chapter 10). Assign to each formula $\phi$ a natural number $\ulcorner\phi\urcorner$, its **Gödel number**, injectively and computably. Syntactic operations — substitution, concatenation — become computable functions on codes, and every computable function is representable in $T$. In particular, the **diagonalization function**
$$d(\ulcorner\phi(x)\urcorner) \;=\; \ulcorner\phi(\overline{\ulcorner\phi(x)\urcorner})\urcorner$$
which maps the code of a formula $\phi(x)$ to the code of the sentence obtained by substituting the numeral of $\phi$'s own code for $x$, is computable, hence representable.

**Lemma (Diagonal Lemma; Gödel 1931, Carnap 1934).** For every formula $\psi(x)$ with one free variable there is a sentence $\lambda$ such that
$$T \vdash \lambda \leftrightarrow \psi(\ulcorner\lambda\urcorner).$$

*Proof sketch.* Let $\theta(x)$ be the formula $\psi(d(x))$ (using the formula representing $d$), and let
$$\lambda \;:=\; \theta(\overline{\ulcorner\theta(x)\urcorner}) \;=\; \psi\bigl(d(\overline{\ulcorner\theta(x)\urcorner})\bigr).$$
By definition of $d$, $\;d(\ulcorner\theta(x)\urcorner) = \ulcorner\theta(\overline{\ulcorner\theta(x)\urcorner})\urcorner = \ulcorner\lambda\urcorner$, and since $T$ represents $d$, $T$ proves this identity. Hence $T \vdash \lambda \leftrightarrow \psi(\ulcorner\lambda\urcorner)$. $\square$

The construction is the same self-application move as the fixed-point combinator in lambda calculus and the recursion theorem in computability: apply a description to its own code. Self-reference is thus *provably constructible* in any theory that can represent its own syntax. There is no syntactic firewall to hide behind.

## Tarski's Undefinability Theorem

Call a formula $Tr(x)$ a **truth predicate** for $T$ if $T \vdash Tr(\ulcorner\phi\urcorner) \leftrightarrow \phi$ for every sentence $\phi$ (every instance of the T-schema is provable).

**Theorem (Tarski's Undefinability of Truth, 1933).** No consistent theory $T$ extending $\mathsf{Q}$ has a truth predicate for itself.

*Proof.* Suppose $Tr(x)$ were such a predicate. Apply the Diagonal Lemma to the formula $\neg Tr(x)$: there is a sentence $\lambda$ with
$$T \vdash \lambda \leftrightarrow \neg Tr(\ulcorner\lambda\urcorner).$$
This $\lambda$ is the formal Liar: it says of itself that it is not true. The T-schema instance for $\lambda$ gives
$$T \vdash Tr(\ulcorner\lambda\urcorner) \leftrightarrow \lambda.$$
Chaining the two biconditionals, $T \vdash Tr(\ulcorner\lambda\urcorner) \leftrightarrow \neg Tr(\ulcorner\lambda\urcorner)$. But $\chi \leftrightarrow \neg\chi$ is propositionally refutable, so $T$ is inconsistent — contradiction. $\square$

**Corollary (semantic form).** The set $\{\ulcorner\phi\urcorner : \mathbb{N} \models \phi\}$ of codes of true arithmetic sentences is not definable by any arithmetic formula.

Contrast this with provability: the set of codes of *provable* sentences **is** definable (by Gödel's $\mathrm{Prov}(x)$, Chapter 10). Truth outruns provability in any consistent theory — this gap is exactly Gödel's first incompleteness theorem, and the Liar is its engine: Gödel's sentence "I am not provable" is the Liar with the definable predicate $\mathrm{Prov}$ substituted for the undefinable $Tr$.

## Variants: Mapping the Terrain

**The Truthteller.** Apply the Diagonal Lemma to $Tr(x)$ itself: $\tau \leftrightarrow T(\ulcorner\tau\urcorner)$ — "this sentence is true." No contradiction follows: assuming $\tau$ true is stable, and so is assuming it false. The Truthteller is consistent but *underdetermined* — nothing settles which value it has. Pathological self-reference thus comes in two flavors, overdetermined (Liar) and underdetermined (Truthteller); Kripke's construction in Section 3 makes this distinction mathematically exact.

**Curry's paradox.** Diagonalize on $T(x) \to (0 = 1)$ to get $\kappa$ with $\kappa \leftrightarrow (T(\ulcorner\kappa\urcorner) \to 0=1)$ — "if this sentence is true, then $0=1$." Now derive:

1. Assume $T(\ulcorner\kappa\urcorner)$. $\quad$ [for $\to$I]
2. $\kappa$. $\quad$ [1, T-schema]
3. $T(\ulcorner\kappa\urcorner) \to 0=1$. $\quad$ [2, the biconditional]
4. $0 = 1$. $\quad$ [1, 3, MP — assumption 1 used a *second* time: contraction]
5. $T(\ulcorner\kappa\urcorner) \to 0=1$. $\quad$ [$\to$I, discharging 1]
6. $\kappa$. $\quad$ [5, the biconditional]
7. $T(\ulcorner\kappa\urcorner)$. $\quad$ [6, T-schema]
8. $0 = 1$. $\quad$ [5, 7, MP]

The derivation uses only the T-schema, $\to$-introduction, modus ponens, and **contraction** (the reuse of assumption 1 at steps 2 and 4). *No negation appears.* Curry shows the paradox is not about negation or falsity: any logic with self-reference, a detachable conditional, and contraction proves everything. This will haunt the paraconsistent solutions of Section 5.

**Yablo's paradox (1993).** Consider an infinite sequence of sentences where $S_n$ says: "for every $k > n$, $S_k$ is not true." Suppose some $S_n$ is true. Then (a) $S_{n+1}$ is not true, and (b) every $S_k$ with $k > n+1$ is not true. But (b) is exactly what $S_{n+1}$ asserts, so $S_{n+1}$ *is* true, contradicting (a). Hence no $S_n$ is true. But then, for any $n$, every $S_k$ with $k > n$ is not true — which is what $S_n$ says — so $S_n$ is true. Contradiction. $\square$ No sentence in the sequence refers to itself; each speaks only of its successors. Whether the paradox is genuinely free of self-reference is disputed (Priest argues the fixed point hides in the uniform predicate "$S_x$ is true" used to define the sequence), but it shows at minimum that banning *sentence-level circularity* does not suffice.

**Contingent liars.** Kripke (1975) observed that paradoxicality can depend on empirical fact. Suppose Dean says "most of Nixon's assertions about Watergate are false," while Nixon's Watergate assertions, apart from responses to Dean, happen to split exactly evenly between true and false. Then Dean's utterance is true iff it is false — a Liar assembled not by syntax but by the world. There is no syntactic test that quarantines the "risky" sentences: as Kripke put it, many ordinary assertions about truth would exhibit paradoxical features if the empirical facts were sufficiently unfavorable. Any adequate theory must handle sentences whose paradoxicality is *a posteriori*.

This distinction — between **semantic paradoxes** (Liar, Curry, Yablo, Grelling), which involve truth, satisfaction, and reference, and set-theoretic paradoxes (Russell, Burali-Forti), which involve membership — is due to Ramsey (1925). Both families share the diagonal engine; the responses surveyed in the following sections (hierarchy, gaps, gluts) all target the semantic family, and each has a set-theoretic cousin.

## Exercises
See [problems/ch18_liars_paradox/](../../../problems/ch18_liars_paradox/)

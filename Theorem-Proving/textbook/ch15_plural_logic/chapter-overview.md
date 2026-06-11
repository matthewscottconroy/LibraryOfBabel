# Chapter 15 Overview: Plural Logic

---

## Central Question

When we say "Russell and Whitehead wrote *Principia Mathematica*," are we quantifying over a set of people, or over the people themselves — plurally? Does plural quantification — quantifying over *many things at once* rather than over a set — provide a logically simpler foundation for mathematics? And can we go further still, to "superplurals" — things of things?

---

## Why This Chapter Matters

Plural logic challenges the assumption that every quantification over many things must be mediated by a set. It has applications in the philosophy of mathematics (avoiding the paradoxes of naive set comprehension), in linguistics (natural language quantification is often irreducibly plural), and in the foundations of set theory itself (where plural comprehension may avoid Russell's paradox more naturally than type theory). Understanding plural logic illuminates the logical structure underlying set theory and helps clarify what is primitive vs. derived in foundations.

---

## Key Definitions

**Plural terms.** A plural term refers to many things at once — not a single set, but the things themselves plurally. "Alice and Bob" is a plural term referring to two people; "the natural numbers" is a plural term referring to all of them.

**Plural variables.** Plural logic introduces plural variables $xx, yy, zz, \ldots$ ranging over *pluralities* (one or more things). The singular variables $x, y, z$ range over individual objects as usual.

**Plural quantification.** $\exists xx\, \phi(xx)$ — "there are some things $xx$ such that $\phi$." $\forall xx\, \phi(xx)$ — "for any things $xx$, $\phi$."

**Inclusion.** $x \prec xx$ — "x is one of the xx" (the basic predicate of plural logic). This is *not* the set-membership relation $\in$; no set is posited.

**Plural comprehension.** For any condition $\phi(x)$ (with $x$ singular and $\phi$ not mentioning $yy$): $\exists xx\, \forall x(x \prec xx \leftrightarrow \phi(x))$ — there are some things, namely all the things satisfying $\phi$.

Note: this looks like the naive comprehension axiom that generated Russell's paradox! The key difference: "the things satisfying $\phi$" is a *plurality*, not a *set*. Pluralities are not objects; "the plurality of all self-membered pluralities" makes no sense, so the Russell paradox analogue does not arise.

---

## Syntax of Plural Logic

**Full plural logic ($PFO^+$).** Extend first-order logic with:

*Plural terms:* $xx, yy, zz, \ldots$ (schematic plural variables)

*Formation rules:* If $\phi$ is a formula and $x$ is a singular variable, then:
- $x \prec xx$ is an atomic formula ("$x$ is one of the $xx$")
- $\exists xx\, \phi$ and $\forall xx\, \phi$ are formulas (binding $xx$ in $\phi$)

*Comprehension schema:* $\exists y\, \phi(y) \to \exists xx\, \forall y(y \prec xx \leftrightarrow \phi(y))$ (for any formula $\phi$ not free in $xx$)

*Plural descriptions:* $\iota x\, \phi(x)$ is the standard singular description; plural logic adds $\iota x\!x\, \phi(xx)$ for plural descriptions.

---

## Main Results

### Theorem: Plural Logic is Ontologically Innocent

**Claim (Boolos 1984).** Plural quantification is *ontologically innocent*: saying "there are some sets that contain all ordinals" does not commit us to the existence of a set of those sets; it commits us only to the ordinals and sets we already believed in.

**Argument.** Singular quantification $\exists x\, \phi(x)$ posits an *object* satisfying $\phi$. Plural quantification $\exists xx\, \phi(xx)$ posits the *things* satisfying $\phi$, but no new object. The plurality of natural numbers is not an object — it is simply all the natural numbers, referred to plurally.

### Theorem: Expressibility Beyond First-Order Logic

**Theorem (Boolos 1985).** Second-order logic is interpretable in plural logic, in the sense that each second-order sentence can be translated into a plural sentence with the same models (over domains of sets).

**Proof sketch.** Second-order variables range over subsets of the domain. Replace each second-order variable $X$ with a plural variable $xx$ and replace $X(a)$ with $a \prec xx$. Comprehension for sets becomes plural comprehension. $\square$

**Significance.** Plural logic thus inherits the expressibility of second-order logic — including the ability to characterise $\mathbb{N}$ up to isomorphism (via the second-order Peano axioms). First-order logic cannot do this (Chapter 9, Löwenheim-Skolem).

### Plural Comprehension and Paradox Avoidance

**Russell's paradox attempt in plural logic.** Can we form the "plurality of all non-self-including pluralities"?

Let $\phi(xx)$ mean "$\neg(xx \prec xx)$." But "$xx \prec xx$" is not well-typed in standard plural logic: $\prec$ relates a *singular* term to a *plural* term ($x \prec yy$), not a plural term to itself. So the Russell-like formula is not even grammatical. The type-distinction between singular and plural terms blocks the paradox at the syntactic level.

**Alternative route (Hazen 1997).** One can extend plural logic to *superplural logic* (plural of plurals) without incurring paradox, by the same grammatical type-distinction trick applied one level up.

---

## Superplural Logic

**Superplural quantification.** $\exists X\!X\, \phi(X\!X)$ — "there are some pluralities $XX$" — where $XX$ ranges over *pluralities of pluralities*.

**Applications:**
- Modelling plural predication in natural language ("the students each submitted their assignments")
- Set-theoretic foundations: sets of sets correspond naturally to pluralities of pluralities
- Cumulative type hierarchies: each level of the hierarchy is a plurality of lower-level objects

**Theorem (Hazen 1997, Rayo 2006).** Third-order logic is interpretable in superplural logic, and the hierarchy can be continued.

---

## Applications in Foundations of Set Theory

**Boolos (1989)** used plural logic to give a new reading of the Zermelo-Fraenkel axioms. The axiom "for any condition $\phi$, there exists a set of all things satisfying $\phi$ that are in a given set $a$" becomes: "for any condition $\phi$, there are some things — namely the things satisfying $\phi$ that are in $a$ — and they form a set." The plural reading makes the existential commitment of comprehension precise.

**Consistency of NF (New Foundations).** Quine's New Foundations set theory has been notoriously difficult to analyse. Recent work (Holmes 2024, announced) uses plural-like machinery to show NF consistent relative to ZFC. Whether this succeeds is an active research topic.

---

## Natural Language Motivation

**The problem.** "The rocks collapsed together." This sentence predicates something of the rocks collectively, not individually. If we regiment it as "for each rock $x$, $x$ collapsed," we misrepresent its meaning. The collective reading requires predicating of the rocks *as a group*.

**Plural logic solution.** Introduce a collective predicate: $C(xx)$ means "the $xx$ collapsed together." No reduction to singular predication is attempted or needed.

**Distributive vs. collective.** "The students passed the exam" is typically *distributive* (each student passed). "The students surrounded the building" is *collective* (they surrounded it together, though no individual did). Plural logic provides the framework to formalise both.

---

## Historical Context

**George Boolos (1984)** wrote "To Be Is to Be a Value of a Variable (Or to Be Some Values of Some Variables)," introducing plural quantification to the philosophical logic literature. This paper single-handedly launched plural logic as a research programme.

**George Boolos (1985)** used plural logic to reformulate second-order logic in the paper "Nominalist Platonism," arguing that second-order logic commits us to no more objects than first-order logic if interpreted plurally.

**Alex Oliver and Timothy Smiley (2001, 2016)** developed a systematic formal treatment of plural logic, including plural terms, plural predicates, and plural descriptions.

**Agustín Rayo (2002, 2006)** extended plural logic to higher orders and studied its connections to set theory.

**The linguistic tradition (Link 1983, Landman 1989, Schwarzschild 1996)** developed plural semantics for natural language independently of the philosophical tradition, providing linguistic motivation for the formal systems.

---

## Connections to Other Chapters

- **Chapter 3** (FOL): plural logic extends FOL with plural variables and comprehension.
- **Chapter 6** (Set Theory): plural logic provides an alternative way to understand the ontological commitments of set comprehension.
- **Chapter 11** (Type Theory): the type-theoretic approach to avoiding Russell's paradox (types) and the plural-logic approach (plurality/object distinction) are two different solutions to the same problem.
- **Chapter 19** (Abstract Algebra): plural quantification over algebraic structures provides a natural way to state universal properties.

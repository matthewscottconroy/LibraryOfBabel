# Chapter 1 Overview: Language and Logic Foundations

---

## Central Question

What is the difference between a *rigorous* argument and merely a persuasive one, and why does that distinction require a new kind of language?

This question is not merely academic. Every proof assistant — Lean 4, Coq, Isabelle — is built on the answer: natural language is too ambiguous, too context-dependent, and too subject to misinterpretation to serve as the medium of verified reasoning. Formal languages replace it not because they are more expressive, but because they are *transparent*: every step is explicit, every rule is mechanical, every inference is checkable.

---

## Why This Chapter Matters

Logic has a peculiar history: it was considered essentially complete from Aristotle until the mid-nineteenth century, then shattered and rebuilt from scratch in a generation. The rebuilding — by Boole, Frege, Peano, Russell, Hilbert, and Gödel — gave mathematics its current foundations and simultaneously gave computer science its theoretical core. Understanding why that rebuilding was necessary, and what it produced, is essential background for every chapter that follows.

---

## Key Definitions

**Formal language.** A formal language over an alphabet Σ is a set of finite strings over Σ, specified by a grammar that determines exactly which strings are *well-formed formulas* (wffs). The grammar is compositional and recursive: complex formulas are built from simpler ones by explicit formation rules.

**Syntax vs. semantics.** Syntax concerns the shape of expressions — which strings are well-formed — without reference to meaning. Semantics assigns meanings to well-formed expressions. The same syntactic formula can receive different semantic interpretations under different models.

**Argument.** An argument is a finite sequence of sentences $P_1, P_2, \ldots, P_n, C$ where $P_1, \ldots, P_n$ are the *premises* and $C$ is the *conclusion*.

**Validity.** An argument is *valid* if the conclusion is true in every model in which all premises are true. Validity is a syntactic/semantic property of the argument's form, not its content.

**Soundness (of an argument).** An argument is *sound* if it is valid and all its premises are true.

**Proof.** A formal proof is a finite sequence of formulas in which each formula is either an axiom or follows from earlier formulas by an explicit inference rule. A proof establishes a *syntactic consequence* relation $\Gamma \vdash \phi$ ("from $\Gamma$, prove $\phi$").

**Logical truth / tautology.** A formula $\phi$ is a *logical truth* (or tautology) if it is true under every possible interpretation. We write $\vDash \phi$.

---

## Main Results

### The Relationship Between Syntax and Semantics

The most important results in logic relate the syntactic derivability relation $\vdash$ (provable) to the semantic consequence relation $\vDash$ (true in all models):

**Soundness:** If $\Gamma \vdash \phi$, then $\Gamma \vDash \phi$. (Provable things are true in all models.)

**Completeness:** If $\Gamma \vDash \phi$, then $\Gamma \vdash \phi$. (Things true in all models are provable.)

These are proved in later chapters for specific proof systems, but the *conceptual* gap between them is already visible at this stage: soundness says the proof system does not prove falsehoods; completeness says it proves everything that is semantically forced.

### Unique Readability Theorem

**Theorem.** Every well-formed formula has a unique parse tree.

*Proof sketch.* By induction on formula complexity. The parenthesisation rules in standard syntax ensure that the main connective of any compound formula is unique (it is the connective whose removal results in exactly two well-formed parts, and the only such connective). This guarantees that the grammar is unambiguous: there is exactly one way to parse any wff.

*Why it matters:* Unique readability ensures that every formula has a determinate meaning — its semantics is a function of its syntax in a well-defined way.

### Argument Form vs. Argument Instance

The distinction between valid *forms* and valid *instances* is subtle. The argument form "if P then Q; P; therefore Q" (modus ponens) is valid. The instance "If 2+2=4 then snow is white; 2+2=4; therefore snow is white" is valid. The instance "If pigs fly then money grows on trees; pigs fly; therefore money grows on trees" is also valid (because the conditional is vacuously true) — even though the conclusion is false and the premises are false.

This illustrates a key point: *validity is about form, not truth*.

---

## Proof Sketches

### Proof that Valid + True Premises = True Conclusion (Soundness for Arguments)

**Claim.** If argument $\{P_1, \ldots, P_n\} \vDash C$ is valid and $P_1, \ldots, P_n$ are all true, then $C$ is true.

**Proof.** By definition of validity, $C$ is true in every model making all $P_i$ true. The actual world is one such model (since all $P_i$ are true by assumption). Therefore $C$ is true in the actual world. $\square$

This is trivial but pedagogically important: it shows that the two properties (validity and true premises) work together to guarantee the truth of the conclusion. Neither alone suffices.

---

## Historical Context

**Aristotle (384–322 BCE)** developed the first systematic theory of valid inference, the *syllogistic*. A syllogism consists of two premises and a conclusion, each of the form "All A are B," "Some A are B," "No A are B," or "Some A are not B." Aristotle identified the valid syllogistic forms and distinguished them from invalid ones. This remained the dominant framework for formal reasoning for two millennia.

**Leibniz (1646–1716)** envisioned a *calculus ratiocinator* — a symbolic calculus for reasoning — that would reduce any dispute to calculation. He developed an early algebraic notation for logic, though he did not fully realise his vision. His dream directly inspired Boole and Frege.

**Boole (1815–1864)** published *The Mathematical Analysis of Logic* (1847) and *An Investigation of the Laws of Thought* (1854). Boole showed that logical reasoning could be expressed algebraically: propositions as variables, conjunction as multiplication, disjunction as addition (in the two-element Boolean algebra). This was the first system to treat logic as mathematics.

**Frege (1848–1925)** published the *Begriffsschrift* ("Concept-Script," 1879), widely regarded as the founding document of modern logic. Frege introduced quantifiers (∀ and ∃), function-argument notation, and a complete formal proof system. He showed that the entire logical structure of mathematical reasoning could be made explicit — a revolutionary step. His *Grundgesetze der Arithmetik* (1893, 1903) attempted to derive arithmetic from logic, but was undermined by Russell's paradox (1902).

**Russell and Whitehead** responded to the paradox with *Principia Mathematica* (1910–1913), a monumental formal system built on type theory that they claimed (but could not fully establish) could express all of mathematics.

**Hilbert (1862–1943)** posed the *Entscheidungsproblem* (decision problem, 1928): find an algorithm that decides, for any mathematical statement, whether it is provable. The negative answer — given independently by Church (1936) and Turing (1936) — defined the limits of formal proof, and thereby laid the foundations for computability theory.

---

## Connections to Other Chapters

- **Chapter 2** formalises propositional logic: the simplest formal language in which syntax, semantics, and proof systems can all be studied together.
- **Chapter 3** extends to first-order logic, adding quantifiers and making Frege's full vision precise.
- **Chapter 4** develops proof systems (natural deduction, sequent calculus) and proves soundness and completeness.
- **Chapter 10** returns to Hilbert's program and its fate: Gödel's incompleteness theorems show that no consistent formal system rich enough for arithmetic can be both complete and have a decidable proof relation.
- **Chapter 13** connects formal languages to proof assistants: Lean 4 and Coq are, at their core, formal languages in which proofs are programs and type-checking is proof verification.

---

## Common Confusions

**Validity vs. soundness:** Validity is a property of argument *form*; soundness additionally requires true premises. An argument can be valid but unsound (valid form, false premise). It cannot be sound but invalid.

**"Formal" does not mean "mechanical":** Formal proof systems require creative discovery of proofs; the formalism only guarantees that *checking* a proof is mechanical, not that *finding* one is.

**The naturalistic fallacy in logic:** "It is natural to reason this way, so it is valid." Naturalness is irrelevant to validity. Many natural inference patterns (e.g., "most A are B; most B are C; therefore most A are C") are logically invalid.

---

## Further Reading

- Frege, G. (1879). *Begriffsschrift*. (Translated by van Heijenoort in *From Frege to Gödel*, 1967.)
- Boole, G. (1854). *An Investigation of the Laws of Thought*. Macmillan. (Free online.)
- van Heijenoort, J., ed. (1967). *From Frege to Gödel: A Source Book in Mathematical Logic*. Harvard University Press.
- Enderton, H. B. (2001). *A Mathematical Introduction to Logic* (2nd ed.). Chapter 1. Academic Press.

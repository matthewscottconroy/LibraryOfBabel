# Language and Logic Foundations: Exercises

## Section 1: Ambiguity (★)

**1.** Identify the type of ambiguity (lexical, syntactic, or pragmatic) in each sentence:
  a. "He saw the man with the telescope."
  b. "I need to get a new bank."
  c. "Can you open the window?"
  d. "Flying planes can be dangerous."

**2.** Disambiguate each sentence by rewriting it in two different unambiguous ways.

**3.** Formalize the two readings of "Every professor likes a student" in FOL.
Which reading is true in a department where each professor likes at least one student,
but different professors may like different students?

## Section 2: Formal Languages (★)

**4.** Using the BNF grammar for propositional logic, determine which strings are wffs:
  a. `¬p ∧ q`
  b. `(p ∧ (q ∨))`
  c. `((p → q) ↔ (¬q → ¬p))`
  d. `p q ∧`

**5.** Draw the parse tree for `(p ∧ ¬q) → (r ∨ p)`. What is the main connective?

**6.** How many distinct wffs can be formed using exactly the atoms {p, q} and exactly
one connective (negation counts as one occurrence)? List them all.

## Section 3: Syntax and Semantics (★★)

**7.** Give an example of a wff that is:
  a. Syntactically well-formed but semantically undefined (hint: consider division by zero analog)
  b. Syntactically simple but semantically powerful
  c. Syntactically complex but semantically trivial (a tautology)

**8.** In Tarski's World, the sentence `Cube(a)` is syntactically a simple atomic formula.
But its truth value depends entirely on the semantic interpretation (which block is a, what
shapes it has). Explain this syntax/semantics distinction in your own words.

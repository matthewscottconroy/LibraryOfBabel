# Formal Languages: Building Precision from Nothing

> *"The intention of the Begriffsschrift is to provide logic with the sharpest possible instrument."*
> — Gottlob Frege, *Begriffsschrift* (1879)

---

Imagine you have been tasked with building a machine that can check mathematical proofs. Your machine cannot rely on intuition, background knowledge, or common sense. It can only follow explicit rules. What do you give it?

The answer, as we will develop over this chapter and the next several, is a **formal language**: a precisely specified system in which every legal expression can be recognized by a mechanical procedure, and every meaning is determined entirely by structure, with no appeal to context.

The development of formal languages is one of the great achievements of nineteenth and twentieth century thought. It emerged from the convergence of three streams: Frege's project of grounding arithmetic in logic, Boole's algebraic treatment of reasoning, and Cantor's set-theoretic revolution. What they collectively realized is that the messiness of mathematical language — the imprecision, the gaps, the reliance on intuition — was not an inevitable feature of human thought but an engineering problem, and engineering problems have engineering solutions.

## The Anatomy of a Formal Language

A formal language has two components: **syntax** and **semantics**. We will spend this section on syntax — the rules for forming legal expressions — and take up semantics in the next.

**Syntax** is purely a matter of shape. It asks: which strings of symbols are well-formed? The syntax does not care what anything *means*, only whether the arrangement of symbols follows the rules. This might seem oddly restrictive, but it is essential: a machine cannot evaluate meaning, but it can check shape.

A formal language is specified by three things:

1. **Alphabet** (Σ): A finite set of basic symbols. These are the atoms from which everything is built. For propositional logic, a typical alphabet includes propositional variables (p, q, r, ...), connective symbols (¬, ∧, ∨, →, ↔), and punctuation (parentheses).

2. **Grammar**: A finite set of formation rules that specify exactly which strings of symbols from Σ are **well-formed formulas** (wffs). The grammar is the recipe; the wffs are the legal dishes.

3. **Semantics** (covered in the next section): An interpretation scheme that assigns meanings — specifically, truth conditions — to the well-formed formulas.

## Grammars: The Recursive Structure

What makes formal grammars so powerful is that they are **recursive**: complex expressions are built from simpler ones, which are built from still simpler ones, all the way down to atomic elements. This recursive structure is what gives formal languages their expressive reach from so few basic ingredients.

Here is the **Backus-Naur Form (BNF)** grammar for propositional logic:

```
φ ::= p₀ | p₁ | p₂ | ...       (propositional variables: atoms)
    | ⊤                          (truth)
    | ⊥                          (falsity)
    | ¬φ                         (negation of a formula)
    | (φ ∧ φ)                    (conjunction of two formulas)
    | (φ ∨ φ)                    (disjunction of two formulas)
    | (φ → φ)                    (conditional)
    | (φ ↔ φ)                    (biconditional)
```

Read the `|` as "or alternatively." Read `φ` as "any well-formed formula." So the rule `¬φ` says: if φ is a well-formed formula, then ¬φ is also a well-formed formula. The rule `(φ ∧ φ)` says: if φ and ψ are both well-formed formulas, then (φ ∧ ψ) is a well-formed formula.

The recursive structure means that formulas can be nested to arbitrary depth. From the atoms p and q, we can build:
- `¬p`
- `(p ∧ q)`
- `(¬p ∨ q)`
- `((p → q) ∧ (¬q → ¬p))`
- `(((p ∧ q) → r) ↔ (p → (q → r)))`

And so on, without limit. A finite grammar generates an infinite language — this is the miracle of recursive definition.

> **Stop and Think**: How many distinct well-formed formulas can be built from a single propositional variable p and the connective ¬? Think carefully before you answer. You can form ¬p, ¬¬p, ¬¬¬p, ... This infinite sequence is already contained in a grammar with just one atom and one connective. Formal languages are inexhaustibly productive.

## The Unique Readability Theorem

Here is a crucial fact that distinguishes formal languages from natural language: every well-formed formula has exactly **one** parse tree. This is the **Unique Readability Theorem**, and it guarantees that there is no syntactic ambiguity in propositional logic.

Why does this matter? Recall that in natural language, "I saw the man with the telescope" admits two parse trees and therefore two meanings. If our formal language admitted this, our truth tables would be undefined — the same formula could have different truth values under different parsings. Unique readability guarantees that every formula has a single, well-defined meaning (once we fix an interpretation).

The proof of unique readability is a nice exercise in structural induction (which we will study in Chapter 7). The key ingredient is the use of balanced parentheses: they uniquely identify the main connective of every non-atomic formula, which determines how the formula was built.

In practice, we often drop outermost parentheses and apply precedence conventions (¬ binds tightest, then ∧, then ∨, then →, then ↔, and → is right-associative) to reduce clutter. But these are conveniences: the underlying language with full parenthesization is unambiguous.

## Object Language and Metalanguage: The Crucial Distinction

Here is a distinction that will save you considerable confusion. When we *study* a formal language, we use a different language to talk *about* it. The language being studied is the **object language**; the language in which we conduct our study is the **metalanguage**.

In this textbook:
- The **object language** is (at various points) propositional logic, first-order logic, or Lean 4 code.
- The **metalanguage** is English, augmented with mathematical notation.

When we write things like "*the formula* p → q *is true under the valuation v*," the formula p → q is an expression of the object language, and the rest of the sentence is metalanguage.

Why does this matter? Because confusing the two levels leads to paradoxes. The Liar Paradox — "This sentence is false" — is paradoxical precisely because it is a natural language sentence that talks about *itself*, blurring the object/metalanguage boundary. Alfred Tarski showed in the 1930s that a formally correct theory of truth must be *stratified*: truth predicates belong to the metalanguage, never to the object language. This insight — for which Tarski is rightly celebrated — prevents the Liar and other self-referential paradoxes from arising.

## Why Formal Languages? The Historical Pressure

To understand why formal languages were developed, it helps to know what pressure they were responding to.

By the 1870s, mathematics had grown enormously complex and surprisingly shaky. The calculus, invented by Newton and Leibniz in the seventeenth century and expanded by Euler and others in the eighteenth, rested on a foundation of intuitive but imprecise notions: infinitesimals, limits, continuity. Cauchy and Weierstrass had done heroic work in the early nineteenth century to make these rigorous using the ε-δ formalism — but their proofs were still written in a combination of mathematical notation and natural language prose, and, as we noted in the previous section, errors were creeping in undetected.

Meanwhile, Frege was asking an even more fundamental question: what, exactly, are numbers? Not in a physical or psychological sense — not what they *feel* like or how we come to know them — but in a purely logical sense: what are the *logical* objects that arithmetic is about? His answer, developed over twenty years in the *Begriffsschrift* (1879) and *Grundgesetze der Arithmetik* (1893, 1903), was that numbers are logical objects definable within a formal system. The formal system he constructed was, in its essentials, what we now call second-order logic.

Frege's notation was famously difficult to read — he wrote formulas in a two-dimensional tree-like format that his contemporaries found impenetrable. But his *idea* was revolutionary: that a mathematical proof could be, in principle, a purely formal manipulation of symbols, subject to mechanical verification. He did not achieve this goal (Russell's paradox destroyed his foundational system), but he laid the conceptual groundwork for everything that followed.

## Formal Languages in Your Tools

**Lean 4** is, among other things, a formal language. Its grammar — the rules for forming valid Lean expressions — is defined precisely in its specification. When Lean's elaborator rejects your code, it is doing exactly what a grammar-checker would do: determining that your string of symbols does not conform to the formation rules. The type system is, in a deep sense, the grammar of Lean's formal language of proofs.

**Coq** is similar. Gallina, the term language of Coq, is a formal language with a precise BNF grammar. The `Proof.` and `Qed.` keywords delineate a formal proof — a string that must conform to the grammar and whose meaning is checked by the kernel.

**Haskell** is also a formal language, and its type system enforces a kind of syntactic constraint that catches semantic errors before they reach the runtime. When GHC gives you a type error, it is rejecting a term because its type — in the formal grammar of types — does not match what was expected.

---

## Real-World Applications

**Compiler design**: A compiler translates source code (one formal language) into machine code (another formal language). The first phase — lexical analysis and parsing — is entirely about determining whether the source code is a well-formed formula of the programming language's grammar. The grammar of every major programming language is specified in a BNF or EBNF grammar, usually found in the language's reference manual.

**Protocol specification**: Network protocols like TCP/IP are specified in documents (RFCs) that describe message formats using formal grammars. A malformed packet — one that does not conform to the grammar — is a common vector for security vulnerabilities. Formally verified protocol implementations (like those produced with tools like Coq or F*) can guarantee that they only produce and accept grammatically correct messages.

**Bioinformatics**: DNA and RNA sequences are strings over the alphabet {A, T, G, C} (or {A, U, G, C}). Stochastic context-free grammars model the secondary structure of RNA molecules. The formal language of RNA structure is not just a metaphor — it is the actual computational object that secondary structure prediction algorithms work with.

---

*Next: We complete the picture by adding semantics — the systematic assignment of meaning to well-formed formulas.*

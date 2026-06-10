# References and Primary Sources

## Foundational Texts

**Gottlob Frege.** *Begriffsschrift, eine der arithmetischen nachgebildete Formelsprache des reinen Denkens.* Halle: Louis Nebert, 1879.
The founding document of modern mathematical logic. Frege introduced the first complete formal system for predicate logic, including quantifiers and inference rules, and laid the groundwork for logicism — the program of reducing mathematics to logic. Reading it alongside later treatments reveals how much of what we now take for granted was invented here.

**Alfred North Whitehead and Bertrand Russell.** *Principia Mathematica.* 3 vols. Cambridge University Press, 1910–1913.
The monumental attempt to ground all of mathematics in a formal logical system built on type theory (to avoid Russell's paradox). Though superseded as a foundation by ZFC and later by type theory in a different form, it remains essential for understanding what rigorous foundational work looks like, and Russell's type-theoretic solution to the paradoxes is a direct ancestor of modern type theory.

**David Hilbert and Wilhelm Ackermann.** *Grundzüge der theoretischen Logik* (Principles of Mathematical Logic). Berlin: Springer, 1928.
The canonical reference for first-order logic from the formalist program. Establishes the standard notions of logical consequence, completeness, and decidability questions, and contains the formulation of the *Entscheidungsproblem* that drove Church and Turing's work.

**Gerhard Gentzen.** *Untersuchungen über das logische Schließen* (Investigations into Logical Deduction). *Mathematische Zeitschrift* 39, 1935.
Introduced both natural deduction and sequent calculus — the two most important formalisms for proof theory. The system presented in this chapter is essentially Gentzen's system NJ (natural deduction for intuitionistic logic). His cut-elimination theorem (Hauptsatz) is among the most significant results in proof theory.

**Sara Negri and Jan von Plato.** *Structural Proof Theory.* Cambridge University Press, 2001.
A modern, accessible treatment of Gentzen-style proof theory that connects classical material to contemporary research. Excellent for readers who want to see how the rules of natural deduction are not arbitrary choices but arise from deep structural considerations.

---

## Seminal Papers

**Gerhard Gentzen.** "Untersuchungen über das logische Schließen I–II." *Mathematische Zeitschrift* 39 (1935): 176–210, 405–431.
Introduced natural deduction and the sequent calculus. The introduction–elimination duality that structures this chapter's proof rules is Gentzen's invention. His normalization theorem shows that every proof can be put in a canonical "direct" form, which under the Curry-Howard correspondence corresponds to program normalization (beta-reduction).

**Kurt Gödel.** "Die Vollständigkeit der Axiome des logischen Funktionenkalküls." *Monatshefte für Mathematik und Physik* 37 (1930): 349–360.
Gödel's completeness theorem for first-order logic: every logically valid formula has a formal proof. This result gives meaning to the claim that the inference rules of first-order logic are adequate — they capture all valid consequence. (The incompleteness theorems are a different matter, addressed in a later chapter.)

**Alfred Church.** "A Note on the Entscheidungsproblem." *Journal of Symbolic Logic* 1 (1936): 40–41, 101–102.
Established, via the lambda calculus, that the decision problem for first-order logic is unsolvable. The lambda calculus developed here is the logical ancestor of every functional programming language, and Church's proof is the first use of what became the Church-Turing thesis. Directly relevant because the Curry-Howard correspondence connects lambda calculus to natural deduction.

**William Alvin Howard.** "The Formulae-as-Types Notion of Construction." Manuscript 1969; published in Hindley and Seldin (eds.), *To H. B. Curry: Essays on Combinatory Logic, Lambda Calculus and Formalism*. Academic Press, 1980.
The paper that formalized the Curry-Howard correspondence: propositions are types, proofs are programs, and normalization is computation. Though circulated as a manuscript for over a decade before publication, this is the conceptual bridge from the logic in this chapter to the type theory in the core HoTT curriculum.

**Per Martin-Löf.** "Intuitionistic Type Theory." Lecture notes, Padova 1980. Published by Bibliopolis, Naples, 1984.
Martin-Löf's mature formulation of dependent type theory, which is the direct foundation of Homotopy Type Theory. The propositions-as-types principle is applied here not just as a correspondence but as a design principle for the foundation of mathematics. Understanding the logic in this chapter is prerequisite to understanding why Martin-Löf's system looks the way it does.

**Haskell B. Curry and Robert Feys.** *Combinatory Logic*, Vol. 1. North-Holland, 1958.
The half of the Curry-Howard correspondence from Curry's side: noticing that the types of the basic combinators $K$ and $S$ correspond to the axioms of implicational logic. The connection was implicit in Curry's work before Howard made it explicit.

**Dana Scott.** "Domains for Denotational Semantics." *Lecture Notes in Computer Science* 140 (1982): 577–613.
Laid the foundation for domain-theoretic semantics of programming languages, providing a mathematical model for the kind of "proofs as programs" intuition that Curry-Howard makes precise. Understanding that proofs have computational content — not just abstract meaning — starts here.

---

## Textbooks and Modern Treatments

**Herbert B. Enderton.** *A Mathematical Introduction to Logic.* 2nd ed. Academic Press, 2001.
The standard rigorous undergraduate textbook for mathematical logic. Clear, complete, and mathematically honest. Covers propositional logic, first-order logic, and the completeness and incompleteness theorems at a level appropriate for mathematics and computer science students who want to understand the foundations rather than just use them.

**Dirk van Dalen.** *Logic and Structure.* 5th ed. Springer, 2013.
Covers propositional and predicate logic with an eye toward proof theory and the relationship between classical and intuitionistic logic. More philosophically aware than Enderton; good for readers who want to understand why intuitionistic logic matters (as it does throughout this curriculum).

**Ian Chiswell and Wilfrid Hodges.** *Mathematical Logic.* Oxford University Press, 2007.
An unusually clean introduction that emphasizes the formal syntax and semantics of first-order logic without sacrificing mathematical rigor. The natural deduction calculus is presented carefully and connects well to later proof-theoretic material.

**Simon Thompson.** *Type Theory and Functional Programming.* Addison-Wesley, 1991. (Available free online.)
Bridges from basic logic and proof theory to dependent type theory. Especially good at making the Curry-Howard correspondence concrete through programming examples. The type-theoretic perspective illuminates why the proof rules in this chapter take the forms they do.

**Jean-Yves Girard, Yves Lafont, and Paul Taylor.** *Proofs and Types.* Cambridge University Press, 1989. (Available free online.)
A concise and elegant treatment of the Curry-Howard correspondence, the lambda calculus, and linear logic. Appropriate after mastering the basics; it shows where proof theory goes when you take the computational interpretation seriously.

---

## Online Resources and Lecture Notes

**The HoTT Book (Homotopy Type Theory: Univalent Foundations of Mathematics).** Institute for Advanced Study, 2013. Available at https://homotopytypetheory.org/book/
The primary text for this curriculum. Appendix A contains a detailed formulation of the type-theoretic foundations that the logic in this chapter informs. Chapters 1 and 3 clarify what "proof" means in a type-theoretic setting.

**Lean 4 / Mathlib Documentation.** https://leanprover-community.github.io/
The Mathlib library for Lean 4 contains thousands of formally verified proofs. Browsing the logic-related files (e.g., `Mathlib.Logic.Basic`) shows exactly how the proof rules of this chapter are implemented in a modern proof assistant.

**Stanford Encyclopedia of Philosophy: Classical Logic / Natural Deduction.** https://plato.stanford.edu/entries/logic-classical/
Philosophically careful and mathematically accurate overview of classical and intuitionistic logic, with historical context and pointers to primary sources.

**Frank Pfenning.** "Lecture Notes on Natural Deduction." Carnegie Mellon University, various years. Available online.
Pfenning's course notes on natural deduction and sequent calculus are among the clearest treatments available. They directly address the relationship between the two formalisms and include the proof-theoretic perspective (normalization, analyticity) that is essential for understanding why natural deduction is the right system for type theory.

**Robert Harper.** *Practical Foundations of Mathematics for Programming Languages.* Cambridge University Press, 2016. Draft available online.
An advanced treatment that takes the computational interpretation of logic as its starting point, developing type theory from the ground up. The chapters on propositional logic and natural deduction are excellent supplementary reading for understanding where this chapter's material goes.

---

## Historical Context

The systematic study of logic as a mathematical discipline begins in the mid-nineteenth century with the work of George Boole and Augustus De Morgan, who showed that the laws of reasoning could be treated algebraically. But the decisive transformation came with Frege's *Begriffsschrift* in 1879. Frege introduced quantifiers, variables bound by quantifiers, and a complete formal system of inference — essentially creating first-order predicate logic from scratch. His motivation was foundational: he wanted to show that the truths of arithmetic are analytic (logically necessary) rather than synthetic (dependent on intuition), a project he called *logicism*.

Frege's program ran into serious trouble when, in 1902, Russell wrote to him pointing out the paradox that bears Russell's name: the set (or in Frege's terms, the *extension*) of all sets that do not contain themselves is both a member and not a member of itself. This was not a minor technical flaw — it was a contradiction at the heart of Frege's system. Frege's response, in the appendix to the second volume of his *Grundgesetze*, is one of the most poignant passages in the history of mathematics: he acknowledges the collapse of his lifework with characteristic honesty. Russell's own response was the theory of types: a hierarchy of logical types designed to block the self-referential constructions that generate paradoxes. This is the direct ancestor, through a long line of development, of the type theories at the heart of this curriculum.

The period from 1900 to 1935 saw several competing programs for securing the foundations of mathematics: Russell's logicism, Hilbert's formalism (reduce mathematics to a finite set of axioms, then prove the axioms consistent using only "finitary" means), and Brouwer's intuitionism (mathematics consists of mental constructions; a statement is only true when constructed, not merely when its negation leads to contradiction). Hilbert's program was dealt a severe blow by Gödel's 1931 incompleteness theorems, though the completeness theorem (1930) stands as a positive achievement. Gentzen's 1935 work on natural deduction unified proof-theoretic intuitions from all these programs and gave proof theory its modern technical shape. The natural deduction rules that appear in this chapter are essentially Gentzen's, and their underlying philosophy — that logical constants are defined by their introduction and elimination rules — was articulated by Gentzen himself and later made precise by Prawitz and Martin-Löf. It is this proof-theoretic perspective, more than any other single idea, that underlies the connection from classical logic through type theory to Homotopy Type Theory.

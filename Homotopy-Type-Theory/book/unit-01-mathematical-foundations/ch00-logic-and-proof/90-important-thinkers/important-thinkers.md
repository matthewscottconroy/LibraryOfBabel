# Important Thinkers in Logic and Proof

## Aristotle (384–322 BCE)

Aristotle invented the discipline of logic. He had no predecessors — he began with nothing and produced a systematic theory of valid inference that dominated the subject for twenty-two centuries. His logical writings, collected under the title *Organon* ("instrument"), identify the syllogism as the fundamental unit of deductive reasoning: a three-proposition argument whose conclusion follows from two premises sharing a middle term. He catalogued valid syllogistic forms, articulated principles of non-contradiction and excluded middle, and recognized — this is the crucial insight — that validity is a *formal* property of argument structure, independent of the specific content.

The limitation of Aristotle's logic is equally instructive: it cannot express nested quantification, function application, or relational predicates with multiple arguments. "Every student of some teacher knows some fact" is beyond the reach of Aristotelian syllogistic. Overcoming this limitation required waiting until Frege.

## Gottlob Frege (1848–1925)

Frege invented modern logic. His *Begriffsschrift* ("concept-script") of 1879 introduced quantifiers, variables, formal inference rules, and the distinction between object and concept — in 83 pages that permanently changed the subject. Before the *Begriffsschrift*, logic was largely philosophical. Afterward, it was mathematical.

Frege's motivation was logicism: the thesis that arithmetic is a branch of pure logic. His *Grundgesetze der Arithmetik* attempted to derive all of arithmetic from logical principles. The project was destroyed by Russell's paradox, discovered in 1902 just as the second volume was going to press. Frege's honest response — he acknowledged the failure, proposed a patch that did not work, and spent his remaining years in philosophical frustration — is one of the more poignant stories in intellectual history.

What survived is everything: quantifiers, the notion of logical consequence, formal proof systems, the distinction between sense and reference. Every inference rule in modern natural deduction ultimately descends from Frege's original system.

## David Hilbert (1862–1943)

Hilbert was not primarily a logician, but he defined the agenda for twentieth-century logic. His *Grundlagen der Geometrie* (1899) showed that geometry could be completely axiomatized — every theorem follows from the axioms by pure logic, with no appeal to intuition about space. His program for foundations of mathematics, articulated in the 1920s, asked for a complete, consistent, decidable formal system for all of mathematics.

Gödel showed in 1931 that this program is unrealizable: no consistent system capable of expressing arithmetic can prove its own consistency (second incompleteness theorem), and no such system is complete (first incompleteness theorem). But Hilbert's questions — consistency, completeness, decidability — are the permanent framework for evaluating formal systems. We still ask them today, and they are the right questions to ask.

Hilbert's positive contribution to this chapter: the clarity of his formalist vision. Proofs are finite syntactic objects. Inference rules are explicit. Every step is checkable by a machine that does not understand anything. This is exactly the vision that proof assistants implement.

## Kurt Gödel (1906–1978)

Gödel proved the completeness theorem (1930): every logically valid first-order sentence is provable in the standard proof calculus. This is the positive result — the proof system is *enough*. Every valid inference can be captured.

Then, in 1931, he proved the incompleteness theorems. No consistent formal system extending Peano arithmetic is complete: there are true sentences of arithmetic not provable in the system. The proof uses a self-referential sentence that says "I am not provable in this system" — a formalized version of the Liar Paradox made precise by coding syntax as numbers.

For this chapter, the completeness theorem is the main result: soundness plus completeness say that the proof system captures exactly the valid inferences. For HoTT more broadly, the incompleteness theorems demonstrate that no single formal system can capture all mathematical truth — motivating the study of what specific systems like MLTT can and cannot prove.

## Gerhard Gentzen (1909–1945)

Gentzen invented natural deduction and the sequent calculus in 1935, dying at 35 in a Prague internment camp after the Second World War. His proof systems are the foundation of all modern proof theory.

Natural deduction's key design principle: introduction rules define the meaning of each connective, and elimination rules express what you can do with a connective once you have proved it. The introduction-elimination duality ensures that proofs can always be normalized: any proof can be converted to a *normal form* with no "detours" (the Hauptsatz, or cut-elimination theorem). Under Curry-Howard, normal proofs correspond to normal-form programs, and cut-elimination corresponds to computation.

Gentzen also proved the consistency of Peano arithmetic by transfinite induction up to ε₀ — partially rehabilitating Hilbert's program by showing that consistency can be proved, but only using stronger methods than those formalized within arithmetic.

## Dag Prawitz (born 1936)

Prawitz proved the normalization theorem for natural deduction: every derivation can be reduced to a normal form, without detours. This is the proof-theoretic version of termination: proofs, like programs, eventually reach a canonical form.

Prawitz's work on *proof-theoretic semantics* — the idea that the meaning of a logical connective is given by its introduction rule, not by truth-table semantics — is philosophically fundamental. It gives the justification for intuitionistic logic: you are entitled to assert P ∧ Q only when you have a proof of P and a proof of Q. Not when P ∧ Q "would be true" in some hypothetical world. The constructive, evidence-based reading of logic that HoTT adopts traces directly to Prawitz's framework.

## Per Martin-Löf (born 1942)

Martin-Löf developed Intuitionistic Type Theory (MLTT) through a series of papers in the 1970s and 1980s. His system unifies the Curry-Howard correspondence (proofs are programs) with dependent types (types that depend on values), producing a foundation for constructive mathematics that is also a programming language.

MLTT is the direct ancestor of every modern proof assistant: Lean, Coq, Agda, and Isabelle's Pure all descend from Martin-Löf's framework. The natural deduction rules in this chapter — particularly the quantifier rules and the induction principles — are the propositional shadows of Martin-Löf's dependent type rules. When we study MLTT in later chapters, we will see that the machinery is already implicit in what we have built here.

Martin-Löf's philosophical contribution is equally important: he gave a precise, constructive account of what it means to understand a proposition — to know what a proof of it looks like — and built his type theory on this epistemological foundation rather than on abstract set-theoretic semantics. This philosophical stance informs the entire HoTT program.

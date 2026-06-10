# Important Figures

## Aristotle (384–322 BCE)
*The first systematic logician; identified the syllogism as the core form of deductive inference.*

Aristotle was born in Stagira in northern Greece and studied at Plato's Academy before founding his own school, the Lyceum, in Athens. Though his philosophy ranges across metaphysics, biology, ethics, and rhetoric, his logical writings — collected under the title *Organon* — represent the first systematic attempt to describe the rules of valid reasoning as an independent subject of study. He had no predecessors to build on in this; he was, as he himself acknowledged in the *Sophistical Refutations*, starting from nothing.

Aristotle's central logical invention is the *syllogism*: a three-part argument in which two premises sharing a middle term yield a conclusion about the outer terms. The canonical example — "All men are mortal; Socrates is a man; therefore Socrates is mortal" — is in *Barbara*, one of the valid syllogistic moods Aristotle identified. He catalogued 14 valid moods across four figures, and this classification dominated logic for nearly two thousand years. More fundamentally, he identified the principle of non-contradiction ("it is impossible for the same thing to belong and not belong to the same thing at the same time") and the excluded middle as foundational laws of thought.

Aristotle's lasting contribution to this chapter is the recognition that validity is a *formal* property: what makes an argument valid is the *form* of its steps, not the truth of its specific content. This insight — that we can study correctness by studying patterns rather than particulars — is the animating idea behind every formal proof system, including the natural deduction calculus in this chapter. The limitations of his system (no function symbols, no quantifier nesting, no hypothetical reasoning) were overcome only by Frege two millennia later. But the project Aristotle defined — logic as the science of correct inference — remains the project this chapter addresses.

---

## Gottlob Frege (1848–1925)
*Founder of modern mathematical logic; inventor of predicate logic, quantifiers, and formal proof systems.*

Frege was born in Wismar, Germany, and spent virtually his entire academic career at the University of Jena, where he was a professor of mathematics largely ignored by his contemporaries. He trained in mathematics rather than philosophy, which gave him both the precision and the ambition to do something philosophers of logic had not done: actually write down, in complete formal detail, the inference rules for a logical system powerful enough to express all of arithmetic. His 1879 *Begriffsschrift* ("concept-script") is the result.

The *Begriffsschrift* introduced quantifiers over arbitrary domains, variables bound by quantifiers, a formal notation for functions and predicates, and a complete axiom system with explicit rules of inference (including what we now call modus ponens and universal instantiation). This is the direct ancestor of every formal proof system in use today. Frege's notation, presented in an unusual two-dimensional form, was never widely adopted, but the underlying system was translated into modern notation and became the standard. He followed this with the *Grundlagen der Arithmetik* (1884), which argued philosophically that arithmetic truths are logical truths, and the *Grundgesetze der Arithmetik* (1893–1903), which attempted to carry out the formal derivation.

The *Grundgesetze* was undone by Russell's paradox, discovered by Russell in 1902 just as the second volume was going to press. Frege's Basic Law V — which asserts that two concepts have the same extension if and only if they apply to exactly the same objects — is inconsistent, as Russell showed. Frege's response was honest and tragic: he acknowledged the failure and was unable to repair it. He spent the rest of his career exploring other foundational approaches without completing any of them. Despite this collapse, his technical achievements stand. The notion of *logical consequence*, the distinction between first-order and second-order logic, the analysis of quantifiers as operators binding variables, and the very idea of a formal proof system — all of these were Frege's inventions. Every inference rule in this chapter's natural deduction system works with the concepts Frege defined.

---

## Bertrand Russell (1872–1970)
*Logicist philosopher and mathematician; discoverer of Russell's paradox, architect of type theory as a foundational repair.*

Russell was born into British aristocracy and educated at Trinity College, Cambridge, where he studied mathematics and moral sciences. His early career was devoted to the foundations of mathematics: influenced by Frege's logicism, he set out to demonstrate that all of mathematics could be derived from logical principles. This project was partly collaborative, producing *Principia Mathematica* (1910–1913) with Alfred North Whitehead — one of the most technically demanding and philosophically ambitious works in the history of logic.

Russell's paradox, which he discovered in 1902 while studying Frege's system, is strikingly simple: let $R = \{ x \mid x \notin x \}$. Then $R \in R$ if and only if $R \notin R$, a contradiction. The paradox is fatal for any system that permits unrestricted set comprehension (forming a set $\{x \mid \varphi(x)\}$ for any formula $\varphi$). Russell's response was the theory of types: objects are assigned to a hierarchy of types (individuals, sets of individuals, sets of sets of individuals, etc.), and self-referential constructions like $R$ are ruled out as type-incorrect. *Principia Mathematica* is built on a ramified version of this type theory, which is syntactically complex but avoids the paradoxes.

The direct relevance to this chapter is Russell's analysis of how logical syntax can generate contradictions, and how type restrictions prevent them. The type-checking intuition — that a well-formed formula must satisfy constraints on the types of its subexpressions — is present here and reappears throughout type theory. The *Principia* also introduces the notation $\vdash$ for provability and establishes the convention of writing proof systems as formal calculi, which is the standard for natural deduction in this chapter. Russell's broader influence on logic, philosophy of language, and the analytic tradition is enormous, though this chapter focuses on the foundational work.

---

## David Hilbert (1862–1943)
*Leader of the formalist program; posed the fundamental problems of completeness and decidability that shaped twentieth-century logic.*

Hilbert was born in Königsberg (now Kaliningrad) and became the most influential mathematician of the late nineteenth and early twentieth centuries, making foundational contributions to invariant theory, algebraic number theory, the axiomatization of geometry, and mathematical physics. In logic, his contribution was programmatic: he asked, with great precision, what we should want from a formal axiomatic system, and pursued those questions relentlessly.

Hilbert's *Grundlagen der Geometrie* (1899) demonstrated that geometry could be axiomatized without any appeal to intuition — every theorem followed from the axioms by pure logic. This model of axiomatic rigor was Hilbert's template for all of mathematics. His program for foundations, articulated in the 1920s, had two components: first, formalize all of mathematics as a complete, consistent formal system; second, prove the consistency of that system using only elementary "finitary" reasoning (to avoid circularity). The *Hilbert–Ackermann* textbook (1928) is the clearest statement of what first-order logic looks like as a formal system, and contains the explicit formulation of the completeness problem (proved by Gödel in 1930) and the decision problem (shown undecidable by Church and Turing in 1936).

Hilbert's positive contribution to this chapter is the clarity of the questions he asked: what does it mean for a system of axioms to be *complete* (every truth provable), *consistent* (no contradiction provable), and *decidable* (existence of an algorithm for theoremhood)? These are the standard properties against which any formal system — including the natural deduction calculus in this chapter — is evaluated. The proof-theoretic framework he promoted, in which inference rules are explicit and proofs are finite syntactic objects, is exactly the framework natural deduction inhabits. That Gödel and Turing ultimately showed his broader program to be unrealizable does not diminish the precision and importance of the questions.

---

## Gerhard Gentzen (1909–1945)
*Inventor of natural deduction and the sequent calculus; founder of proof theory as a mathematical discipline.*

Gentzen was born in Greifswald, Germany, and studied mathematics at several German universities before completing his doctoral thesis at Göttingen in 1933 under Paul Bernays. His 1935 paper "Untersuchungen über das logische Schließen" (Investigations into Logical Deduction) is the founding document of modern proof theory. Despite dying at age 35 in a Prague internment camp after the Second World War, Gentzen's technical output permanently shaped the field.

The 1935 paper introduces two formal proof systems: *natural deduction* (system N) and the *sequent calculus* (system L). Natural deduction is designed to reflect how mathematicians actually reason: each logical connective has introduction rules (how to prove it) and elimination rules (how to use it once proved). This introduction–elimination duality is not arbitrary; Gentzen designed it so that the introduction rule for each connective gives the meaning of that connective, and the elimination rule is the inverse. This design principle — later called *proof-theoretic semantics* by Prawitz and Dummett — is philosophically fundamental and corresponds exactly to the type-introduction and type-elimination rules in Martin-Löf type theory. The proof rules in this chapter are Gentzen's rules, presented in the standard notation that descended from his original paper.

Gentzen's most celebrated result is the *Hauptsatz* (cut-elimination theorem): in the sequent calculus, any proof can be transformed into a *cut-free* proof, in which no lemma is introduced and then used. This theorem implies the *subformula property*: every formula in a cut-free proof is a subformula of the conclusion. Cut-elimination is the proof-theoretic analogue of beta-reduction in the lambda calculus — under Curry-Howard, they correspond exactly. Gentzen also proved the consistency of Peano arithmetic by transfinite induction up to $\varepsilon_0$, partially rehabilitating Hilbert's program within its means.

---

## Kurt Gödel (1906–1978)
*Proved the completeness of first-order logic and the consistency of the axiom of choice; the preeminent mathematical logician of the twentieth century.*

Gödel was born in Brünn (now Brno, Czech Republic) and studied mathematics and philosophy in Vienna, becoming associated with the Vienna Circle while maintaining an independence from its positivist philosophy. His doctoral dissertation, completed in 1929 and published in 1930, proved the *completeness theorem* for first-order predicate logic: every logically valid first-order sentence is provable in the Hilbert–Ackermann calculus. This is the positive counterpart to the incompleteness theorems: the inference rules for first-order logic are, in a precise sense, *enough* — they do not miss any logical truth.

For this chapter, the completeness theorem is the central result bearing Gödel's name. It gives the formal proof systems of Section 4 (predicate logic) their significance: proving something in the formal calculus is equivalent to it being true in every interpretation. The proof proceeds by a constructive argument: given a consistent set of axioms, Gödel builds a model (the *Henkin model*) in which all those axioms are true, by adding "witness constants" for every existential statement. This Henkin construction is reused throughout model theory and has a type-theoretic analogue in the construction of term models.

Gödel's other foundational contributions — the constructible universe $L$, the relative consistency of the Axiom of Choice and the Generalized Continuum Hypothesis with ZF, and, yes, the incompleteness theorems — are addressed in later chapters. His work on intuitionistic logic (the Gödel translation, showing intuitionistic logic embeds into classical modal logic S4) is directly relevant to the relationship between classical and constructive proof that runs through this entire curriculum. Gödel himself was deeply sympathetic to Platonism and believed that mathematical objects exist independently of formal systems — a philosophical position in productive tension with the constructivist leanings of type theory.

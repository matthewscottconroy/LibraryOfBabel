# References and Primary Sources

## Foundational Texts

**David Hilbert and Wilhelm Ackermann, *Grundzüge der theoretischen Logik* (Principles of Theoretical Logic, 1928).** The textbook that formalized first-order logic and posed the *Entscheidungsproblem* (decision problem): is there an algorithm to determine, for any first-order sentence, whether it is a logical tautology? Church and Turing's independent negative answers to this question in 1936 defined the limits of formal proof and inaugurated computability theory.

**Kurt Gödel, *Über formal unentscheidbare Sätze der Principia Mathematica und verwandter Systeme I* (On Formally Undecidable Propositions of Principia Mathematica and Related Systems I, 1931).** The paper containing both incompleteness theorems. Gödel shows that in any consistent formal system strong enough to encode arithmetic, there is a sentence $G$ — the Gödel sentence, which asserts "I am not provable" — that is true but unprovable. This single paper ended Hilbert's program and reshaped the philosophy of mathematics.

**Gerhard Gentzen, *Untersuchungen über das logische Schließen* (Investigations into Logical Deduction, 1935).** The founding document of structural proof theory. In a single paper Gentzen introduces both natural deduction and sequent calculus, proves the normalization theorem for natural deduction, and proves the *Hauptsatz* (cut elimination theorem) for sequent calculus. He then uses cut elimination to give a syntactic proof of the consistency of predicate logic and goes on to prove the consistency of Peano Arithmetic using transfinite induction up to $\varepsilon_0$.

**Dag Prawitz, *Natural Deduction: A Proof-Theoretical Study* (Almqvist & Wiksell, 1965).** The definitive modern treatment of natural deduction, proving the normalization theorem in full generality. Prawitz introduces the concept of *proof reduction* (the removal of detours, corresponding to $\beta$-reduction in the $\lambda$-calculus) and establishes the strong normalization theorem. This book laid the groundwork for the Curry-Howard correspondence as it is understood today.

**Jean-Yves Girard, Paul Taylor, and Yves Lafont, *Proofs and Types* (Cambridge University Press, 1989; freely available online).** A concise and elegant treatment of the Curry-Howard correspondence and the proof theory of the simply-typed $\lambda$-calculus. Covers natural deduction, sequent calculus, cut elimination, strong normalization, and the second-order $\lambda$-calculus (System F). Serves as the bridge from proof theory to type theory in this curriculum.

---

## Seminal Papers

**David Hilbert, "Über das Unendliche" (On the Infinite), *Mathematische Annalen* 95 (1926), pp. 161–190.** Hilbert's definitive statement of his *Formalist Program*: the goal of finding finite, combinatorial proofs of the consistency of all of mathematics. This paper introduces "ideal elements" (infinite objects used in mathematics) and the *finitist* epistemology meant to justify them. Gödel's incompleteness theorems showed that the program as stated is unrealizable.

**Kurt Gödel, "Über formal unentscheidbare Sätze der Principia Mathematica und verwandter Systeme I," *Monatshefte für Mathematik und Physik* 38 (1931), pp. 173–198.** See above. The paper is technically precise and still readable today. Gödel introduces the coding of formulas by natural numbers (Gödel numbering), constructs the provability predicate $\mathrm{Prov}(n)$, and builds the Gödel sentence $G \equiv \neg\mathrm{Prov}(\ulcorner G \urcorner)$ using the diagonal lemma.

**Gerhard Gentzen, "Untersuchungen über das logische Schließen I–II," *Mathematische Zeitschrift* 39 (1935), pp. 176–210 and 405–431.** See above. The cut elimination proof is in Part I (pp. 187–210); the consistency of predicate logic follows immediately. Part II extends the analysis to intuitionistic logic and first-order logic with identity.

**Alonzo Church, "An Unsolvable Problem of Elementary Number Theory," *American Journal of Mathematics* 58 (1936), pp. 345–363.** Church proves that the $\lambda$-calculus has an undecidable equality problem, establishing what is now called *Church's thesis*: the computable functions are exactly the $\lambda$-definable functions. This paper, together with Turing's, settles the Entscheidungsproblem negatively.

**Alan Turing, "On Computable Numbers, with an Application to the Entscheidungsproblem," *Proceedings of the London Mathematical Society* 42 (1936), pp. 230–265.** Introduces the Turing machine and proves the halting problem undecidable, directly solving the Entscheidungsproblem negatively. The paper is extraordinarily readable for its depth. Turing's proof uses diagonalization to show that no Turing machine can correctly decide, for all machine-input pairs $(M, w)$, whether $M$ halts on $w$.

**William Howard, "The Formulae-as-Types Notion of Construction," circulated 1969, published in: *To H.B. Curry: Essays on Combinatory Logic, Lambda Calculus, and Formalism* (Academic Press, 1980), pp. 479–490.** Howard's letter (unpublished for 11 years) explicitly identifies the correspondence between natural deduction proofs and simply-typed $\lambda$-terms, formalizing the informal connection Curry had observed between combinators and axioms of propositional logic. This is the Curry-Howard correspondence.

**Per Martin-Löf, "An Intuitionistic Theory of Types: Predicative Part," in: *Logic Colloquium '73* (North-Holland, 1975), pp. 73–118.** Martin-Löf's first published presentation of Intuitionistic Type Theory, extending the Curry-Howard correspondence from propositional to predicate logic using dependent types. This paper bridges proof theory and type theory: dependent function types correspond to universal quantifiers, dependent pair types to existential quantifiers.

---

## Textbooks and Modern Treatments

**Sara Negri and Jan von Plato, *Structural Proof Theory* (Cambridge University Press, 2001).** A modern treatment of sequent calculus, cut elimination, and proof-theoretic methods, with an emphasis on the computational interpretation. Covers classical and intuitionistic logic, modal logic, and substructural logics. Particularly good on the details of cut reduction procedures and the subformula property.

**Samuel Buss (ed.), *Handbook of Proof Theory* (Elsevier, 1998).** The comprehensive reference for proof theory as a research discipline. Chapters cover ordinal analysis, proof complexity, subsystems of arithmetic, and the proof theory of set theory. The introductory chapters by Buss on propositional proof complexity and by Troelstra on basic proof theory are accessible to graduate students.

**Helmut Schwichtenberg and Stanley Wainer, *Proof Theory: The First Step into Impredicativity* (Springer, 2012).** A rigorous treatment of ordinal proof theory, covering the consistency of Peano Arithmetic via Gentzen's method, the ordinal $\varepsilon_0$, and more. A valuable companion to the material on Gentzen's consistency proof and ordinal analysis in Section 4 of this chapter.

**Anne Sjerp Troelstra and Helmut Schwichtenberg, *Basic Proof Theory*, 2nd ed. (Cambridge University Press, 2000).** The standard graduate textbook for proof theory, covering natural deduction, sequent calculus, normalization, and cut elimination in detail. More accessible than the Handbook and suitable as a primary text. The treatment of the intuitionistic/classical distinction via sequent calculus is particularly clear.

---

## Online Resources and Lecture Notes

**Frank Pfenning, "Lecture Notes on Proof Theory" (Carnegie Mellon University, various years).** Pfenning's course notes on proof theory are among the best freely available treatments. They cover natural deduction, sequent calculus, cut elimination, and the Curry-Howard correspondence with an emphasis on the computational interpretation. Available at Pfenning's CMU course pages.

**Peter Smith, *An Introduction to Gödel's Theorems*, 2nd ed. (Cambridge University Press, 2013; extensive excerpts available online).** A philosophical and technical introduction to the incompleteness theorems at the level of an advanced undergraduate. Very careful about distinguishing syntax from semantics, and about exactly which assumptions go into the proofs. The discussion of Gödel numbering, the diagonal lemma, and the second incompleteness theorem is especially clear.

**Jean-Yves Girard, *Proofs and Types* (Cambridge, 1989; available free at Paul Taylor's website).** See above. Freely downloadable.

**Stanford Encyclopedia of Philosophy, entries on "Proof Theory," "Gödel's Incompleteness Theorems," "Natural Deduction," and "The Curry-Howard Correspondence."** Authoritative philosophical overviews with substantial technical content and extensive bibliographies. Freely available at plato.stanford.edu.

**Luca Paolini and Simona Ronchi Della Rocca, "Introduction to Proof Theory" (lecture notes, Università di Torino).** A clear and self-contained introduction to natural deduction, sequent calculus, and the Curry-Howard isomorphism, accessible to students with a logic background. Covers both classical and intuitionistic systems.

---

## Historical Context

Proof theory as a discipline was born from Hilbert's Formalist Program of the 1920s. Hilbert believed that all of mathematics could be formalized in a single consistent axiomatic system, and that the consistency of this system could be established by *finitist* means — combinatorial arguments about the formal symbols of the system, without any appeal to infinite objects. This was not mere formalism for its own sake: Hilbert viewed it as the way to secure the foundations of mathematics against the paradoxes (Russell's paradox, Burali-Forti's paradox) that had emerged at the turn of the century. His 1926 lecture "Über das Unendliche" is the most eloquent statement of this vision.

Gödel's 1931 incompleteness theorems ended Hilbert's program in its original form. The First Incompleteness Theorem showed that any consistent, recursively axiomatizable theory strong enough to encode arithmetic contains a sentence that is neither provable nor disprovable. The Second Incompleteness Theorem showed that the consistency of such a theory cannot be proved within the theory itself — blocking the specifically *finitist* consistency proofs Hilbert sought. The philosophical shock was enormous: not only could mathematics not prove its own consistency, but there are mathematical truths that lie forever beyond the reach of any fixed formal system.

What emerged from the ruins of Hilbert's program was not despair but a richer discipline. Gentzen's 1935 paper showed that the consistency of arithmetic could still be proved — just not by the finitist means Hilbert had hoped for. Gentzen used transfinite induction up to the ordinal $\varepsilon_0$ (which exceeds all the ordinals finitists accept) to prove that no formal contradiction can appear in Peano Arithmetic. Ordinal proof theory — using ordinals as a measure of the "strength" of formal theories — became a central tool: the proof-theoretic ordinal of a system quantifies exactly how much transfinite induction is needed to prove its consistency. At the same time, Church and Turing (working independently in 1936) resolved the Entscheidungsproblem by showing that no algorithm can decide all first-order tautologies. Their work connected proof theory to computability theory, a connection that deepened with the Curry-Howard correspondence: the computation of a program and the normalization of a proof are the same phenomenon.

---

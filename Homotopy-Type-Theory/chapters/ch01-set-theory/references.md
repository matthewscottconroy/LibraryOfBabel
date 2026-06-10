# References and Primary Sources

## Foundational Texts

**Paul Halmos.** *Naive Set Theory.* Van Nostrand, 1960. (Reprinted by Springer.)
The classic short introduction to set theory without the heavy formalism of axiomatic treatments. Halmos writes with unusual clarity, making this the best starting point for mathematicians who want to understand set theory as a working tool rather than as a formal system. Its "naive" title is honest: it develops the essential material (relations, functions, cardinality, ordinals, the Axiom of Choice) at a level that most mathematicians actually operate at, before the full ZFC formalism is needed.

**Thomas Jech.** *Set Theory: The Third Millennium Edition.* Springer, 2003.
The definitive comprehensive reference for modern set theory. Covers ZFC axioms, ordinals and cardinals, the constructible universe, forcing, large cardinals, and more. The first ten chapters — covering the material in this chapter — are a rigorous but readable treatment. The later chapters document twentieth-century research that extends far beyond the curriculum but provides essential context for understanding what set theory is and is not capable of.

**Kenneth Kunen.** *Set Theory: An Introduction to Independence Proofs.* North-Holland, 1980. (New edition: College Publications, 2011.)
The standard graduate text for forcing and independence results. Kunen's exposition of the method of forcing — developed by Paul Cohen to prove the independence of the Continuum Hypothesis — is clearer than Cohen's original presentation. The first three chapters provide an exceptionally careful treatment of ZFC, ordinals, and cardinals that rewards close reading.

**Yiannis Moschovakis.** *Notes on Set Theory.* 2nd ed. Springer, 2006.
A concise undergraduate-level text that emphasizes the mathematical content of set theory rather than its foundational role. Particularly good on the development of ordinals and cardinals from first principles, and on careful statements of the Axiom of Choice and its equivalents. The tone is engaged and mathematically honest about what set theory does and does not accomplish.

**Michael Hallett.** *Cantorian Set Theory and Limitation of Size.* Oxford University Press, 1984.
A historical and philosophical study of Cantor's original development of set theory and the paradoxes that forced its axiomatization. Essential for understanding why the ZFC axioms look the way they do, and for appreciating that the axioms are not arbitrary but were chosen to capture Cantor's informal theory while blocking the known paradoxes.

---

## Seminal Papers

**Georg Cantor.** "Über eine Eigenschaft des Inbegriffes aller reellen algebraischen Zahlen." *Journal für die reine und angewandte Mathematik* 77 (1874): 258–262.
Cantor's first proof that the real numbers are uncountable, using the method of nested intervals. This is the paper that introduced the concept of *different sizes of infinity* into mathematics. The argument is more constructive than the later diagonal argument and works by showing the reals cannot be listed even in a bounded interval.

**Georg Cantor.** "Über eine elementare Frage der Mannigfaltigkeitslehre." *Jahresbericht der Deutschen Mathematiker-Vereinigung* 1 (1891): 75–78.
Cantor's diagonal argument: the second and more famous proof that $\mathbb{R}$ is uncountable, generalized to show that for any set $A$, the power set $\mathcal{P}(A)$ has strictly greater cardinality than $A$. This is Cantor's theorem in its general form. The diagonalization technique introduced here is one of the most productive methods in all of logic and theoretical computer science, appearing in Gödel's incompleteness proofs, the halting problem, and Cantor's own later work.

**Ernst Zermelo.** "Untersuchungen über die Grundlagen der Mengenlehre I." *Mathematische Annalen* 65 (1908): 261–281.
The first axiomatization of set theory. Zermelo proposed seven axioms — Extensionality, Empty Set, Pairing, Union, Power Set, Separation (Aussonderung), and Infinity — designed to capture the set-theoretic reasoning mathematicians actually used while avoiding the paradoxes. This paper initiated the axiomatic tradition that produced ZFC, the standard foundation used today.

**Abraham Fraenkel.** "Zu den Grundlagen der Cantor-Zermeloschen Mengenlehre." *Mathematische Annalen* 86 (1922): 230–237. See also Skolem's 1922 address.
Fraenkel (and independently Skolem) identified a weakness in Zermelo's original system: it could not prove the existence of $\{\omega, \mathcal{P}(\omega), \mathcal{P}(\mathcal{P}(\omega)), \ldots\}$ as a set. The Replacement Axiom — which says the image of a set under any definable function is a set — fills this gap and makes the system strong enough for all of ordinary mathematics. The addition of Replacement gives ZF; adding the Axiom of Choice gives ZFC.

**Paul Cohen.** "The Independence of the Continuum Hypothesis." *Proceedings of the National Academy of Sciences* 50 (1963): 1143–1148; 51 (1964): 105–110.
Cohen's proof that the Continuum Hypothesis ($2^{\aleph_0} = \aleph_1$) cannot be proved in ZFC — complementing Gödel's 1938 result that it cannot be disproved. The method, called *forcing*, constructs models of ZFC in which $2^{\aleph_0}$ can be any cardinal of uncountable cofinality. This is arguably the most technically difficult result in twentieth-century mathematical logic, and it established forcing as the central method of modern set theory.

**Kurt Gödel.** "The Consistency of the Axiom of Choice and of the Generalized Continuum Hypothesis with the Axioms of Set Theory." *Proceedings of the National Academy of Sciences* 24 (1938): 556–557. (Full account: Princeton University Press, 1940.)
Gödel proved that if ZF is consistent, then ZFC is consistent, and the Generalized Continuum Hypothesis is consistent with ZFC. His method was the *constructible universe* $L$: the smallest model of ZF, built by an explicit transfinite construction, in which both Choice and GCH hold. Together with Cohen's result, this shows that CH is *independent* of ZFC — neither provable nor disprovable.

**Bertrand Russell.** "Letter to Frege," June 16, 1902. Published in van Heijenoort (ed.), *From Frege to Gödel.* Harvard University Press, 1967.
The letter in which Russell communicated the paradox to Frege. Short and devastating. The paradox — the set of all sets that do not contain themselves — requires only the axiom of unrestricted comprehension, which Frege's system allows. Russell's proposed resolution in this letter (what becomes the theory of types) is the first draft of a type-theoretic solution to the paradoxes.

---

## Textbooks and Modern Treatments

**Herbert B. Enderton.** *Elements of Set Theory.* Academic Press, 1977.
The clearest undergraduate textbook on axiomatic set theory. Covers the ZFC axioms, ordinals, cardinals, and the Axiom of Choice in a mathematically rigorous but accessible way. Enderton is careful to distinguish what requires which axioms, which makes this book especially useful for understanding the logical structure of set theory.

**Azriel Lévy.** *Basic Set Theory.* Springer, 1979. (Reprinted by Dover.)
A mathematically thorough treatment of ZF, ordinals, cardinals, and models of set theory. Lévy's book is more formally careful than Halmos and covers more ground. Particularly good on the relationship between first-order logic and set theory, and on what can and cannot be formalized.

**Patrick Suppes.** *Axiomatic Set Theory.* Van Nostrand, 1960. (Reprinted by Dover.)
Develops set theory from an axiomatic starting point that is simpler and more philosophically explicit than Zermelo's original. Good for readers who want to see the axioms introduced one at a time with careful justification, and who want explicit discussion of the philosophical issues (what is a set? what do the axioms assert?).

**Michael D. Potter.** *Set Theory and Its Philosophy.* Oxford University Press, 2004.
Combines a rigorous mathematical development of set theory with sustained philosophical commentary. Potter uses a set theory based on levels (cumulative hierarchy) rather than the standard ZFC presentation, which makes the cumulative conception of set explicit. Very good for understanding *why* the axioms are what they are.

**Winfried Just and Martin Weese.** *Discovering Modern Set Theory.* 2 vols. American Mathematical Society, 1996–1997.
A graduate-level text that builds from ZFC to independence results with careful attention to mathematical detail. Volume I covers ZFC and basic combinatorics; Volume II develops forcing. The exercises are excellent and the exposition is unusually honest about the difficulties.

---

## Online Resources and Lecture Notes

**Stanford Encyclopedia of Philosophy: Set Theory.** https://plato.stanford.edu/entries/set-theory/
A philosophically careful and mathematically accurate overview of the foundations and history of set theory. Covers the paradoxes, ZFC, independence results, and alternative foundations (NF, NBG, MK) at a level appropriate for someone who wants context and significance rather than technical detail.

**Lean 4 / Mathlib: Set Theory Files.** https://leanprover-community.github.io/mathlib4_docs/Mathlib/SetTheory/
Mathlib formalizes substantial parts of set theory in Lean 4, including cardinals, ordinals, and the Axiom of Choice. Browsing these files shows exactly how abstract set-theoretic definitions are encoded in a dependent type system — a preview of the relationship between set theory and type theory developed in this curriculum.

**Timothy Chow.** "A Beginner's Guide to Forcing." *Communicating Mathematics,* Contemporary Mathematics 479 (2009): 25–40. Available on arXiv.
The clearest non-technical introduction to Cohen's method of forcing. Chow explains the key ideas (generic extensions, names for sets, the forcing relation) without requiring graduate-level background. Essential for understanding the independence of CH discussed in Section 5.

**Thoralf Skolem.** "Some Remarks on Axiomatized Set Theory." Address to the Fifth Congress of Scandinavian Mathematicians, 1922. Published in van Heijenoort (ed.), *From Frege to Gödel.*
Skolem's paper simultaneously introduced the Replacement Axiom (independent of Fraenkel) and proved the Löwenheim-Skolem theorem for set theory, yielding the "Skolem paradox": ZFC has a countable model, yet ZFC proves the existence of uncountable sets. This paradox illuminates the relativity of cardinality to models, a point central to understanding the limitations of set theory discussed in Section 5.

**Daniel E. Cunningham.** *Set Theory: A First Course.* Cambridge University Press, 2016.
A modern introductory text available in many university libraries. Covers the ZFC axioms, relations, functions, ordinals, and cardinals in a self-contained way. Particularly useful as a bridge between an undergraduate discrete mathematics background and the more sophisticated treatments in Jech or Kunen.

---

## Historical Context

The history of set theory is inseparable from the history of infinity in mathematics. Before Cantor, infinity was treated with extreme caution or outright hostility by most mathematicians — Gauss famously declared that "infinity is merely a way of speaking." Cantor changed this permanently. His work in the 1870s on trigonometric series led him to study accumulation points of sets of real numbers and, from there, to the realization that infinite sets can have different sizes. The 1874 paper that introduced uncountability was initially met with skepticism; his colleague Kronecker called his work a "corruption of youth." Cantor persisted, and by the 1880s had developed a rich theory of transfinite ordinals and cardinals, including the diagonal argument and the theorem that $|{\mathcal P}(A)| > |A|$ for every set $A$.

The contradiction between Cantor's rich theory and the foundations of logic became apparent around 1900. Russell's paradox (1902) was the most famous, but Burali-Forti (1897) had already identified a paradox in the theory of ordinals, and Cantor himself knew by 1899 that the class of all ordinals (and of all cardinals) cannot be a set. The community's response was to axiomatize — to identify explicitly which constructions are permitted, rather than assuming all "comprehensible" collections are sets. Zermelo's 1908 axioms were the first successful attempt. The system was strengthened by Fraenkel's Replacement Axiom (1922), clarified by Skolem's first-order reformulation (1922), and supplemented by the regularity (Foundation) axiom (von Neumann, 1925) to produce the ZFC system we use today.

The deep questions about ZFC's power — what it can and cannot prove — occupied logicians for decades after the axioms were set down. Gödel's constructible universe (1938) showed that both the Axiom of Choice and the Continuum Hypothesis are consistent with ZF; Cohen's forcing method (1963) showed that their negations are also consistent. The resulting picture — that set theory has a rich landscape of models, and that many natural questions (not just artificial independence results) are undecidable in ZFC — fundamentally changed how mathematicians think about foundations. It is this picture of foundations as something to be understood critically, rather than taken as an unquestionable ground, that motivates this chapter's journey from naive set theory through ZFC to an examination of set theory's limits — and a preview of why type theory offers a different approach to the same foundational questions.

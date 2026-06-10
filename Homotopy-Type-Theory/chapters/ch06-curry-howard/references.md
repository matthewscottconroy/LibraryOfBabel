# References and Primary Sources

## Foundational Texts

**Haskell B. Curry and Robert Feys.** *Combinatory Logic, Volume I.* North-Holland, 1958.
The foundational text on combinatory logic that contains early seeds of the correspondence: Curry had observed by 1934 that the types of the basic combinators $K$ and $S$ match the axioms of implicational logic, but this book develops that observation in full formal context.

**William A. Howard.** "The Formulae-as-Types Notion of Construction." In *To H.B. Curry: Essays on Combinatory Logic, Lambda Calculus and Formalism*, eds. J.P. Seldin and J.R. Hindley. Academic Press, 1980. (Manuscript circulated 1969.)
The paper that named and fully articulated the correspondence between natural deduction and typed lambda calculus — that proof rules and typing rules are formally identical, and that proof normalization is beta reduction. Famously circulated as an unpublished manuscript for eleven years before publication.

**Per Martin-Löf.** *Intuitionistic Type Theory.* Bibliopolis, Naples, 1984. (Notes by Giovanni Sambin from the 1980 Padova lectures.)
The founding text of Martin-Löf Type Theory, extending the Curry-Howard correspondence from propositional to predicate logic. Introduces dependent product types ($\Pi$), dependent sum types ($\Sigma$), identity types, and the whole apparatus of intensional type theory.

**Jean-Yves Girard, Paul Taylor, and Yves Lafont.** *Proofs and Types.* Cambridge University Press, 1989.
A lucid account of the proofs-as-programs correspondence at the level of System F, covering the reducibility method for strong normalization, the Curry-Howard isomorphism for second-order logic, and the connections to category theory.

**Morten Heine Sørensen and Pawel Urzyczyn.** *Lectures on the Curry-Howard Isomorphism.* Studies in Logic and the Foundations of Mathematics, Vol. 149. Elsevier, 2006.
The most comprehensive technical monograph on the subject, covering STLC, System F, classical logic and control operators, linear logic, and the extensions toward HoTT. An essential reference for the technically inclined reader.

---

## Seminal Papers

**Haskell B. Curry.** "Functionality in Combinatory Logic." *Proceedings of the National Academy of Sciences* 20(11): 584–590, 1934.
The original observation — that the types of the $K$ and $S$ combinators correspond to axioms of implicational logic — that started the entire correspondence. Curry did not have the full picture yet (natural deduction had not yet been formulated by Gentzen), but this is the seed.

**Gerhard Gentzen.** "Untersuchungen über das logische Schließen" (Investigations into Logical Deduction). *Mathematische Zeitschrift* 35: 176–210, 405–431, 1935.
While not about the Curry-Howard correspondence per se, Gentzen's introduction of natural deduction is the logical half of the correspondence. Howard's observation was that *this* proof system is isomorphic to the typed lambda calculus — which required knowing natural deduction first.

**William A. Howard.** "The Formulae-as-Types Notion of Construction." In *To H.B. Curry: Essays on Combinatory Logic*, 1980 (ms. 1969).
See above. The direct statement of the correspondence: propositions as types, proofs as terms, proof normalization as beta reduction, with a precise translation table for all connectives.

**Joachim Lambek.** "The Mathematics of Sentence Structure." *The American Mathematical Monthly* 65(3): 154–170, 1958. And: "Deductive Systems and Categories I–III." *Mathematical Systems Theory* 2(4): 287–318, 1969; *Lecture Notes in Mathematics* 86, 1969; *Lecture Notes in Mathematics* 274, 1972.
Lambek discovered the categorical counterpart: the internal language of a Cartesian closed category is the simply typed lambda calculus, and the morphisms of a Cartesian closed category model both natural deduction proofs and typed lambda terms. This completes the Curry-Howard-Lambek triple correspondence.

**Jean-Yves Girard.** "Interprétation Fonctionnelle et Élimination des Coupures de l'Arithmétique d'Ordre Supérieur." PhD thesis, Université Paris VII, 1972.
Girard's thesis introducing System F (polymorphic lambda calculus) and establishing its correspondence with second-order logic. Strong normalization for System F — proved by a spectacular semantic argument — is also contained here.

**Philip Wadler.** "Propositions as Types." *Communications of the ACM* 58(12): 75–84, 2015.
A superb expository account of the history and significance of the correspondence, written for a broad computer science audience. Introduced the "holy trinity" framing (logic, type theory, category theory) and is the standard modern introduction to the topic.

**Jean-Louis Krivine.** "Lambda-Calculus, Types and Models." Ellis Horwood / Masson, 1993. (Originally in French, 1990.)
Develops the denotational semantics of typed lambda calculus through domain theory and Scott models, providing the semantic completeness results that complement the syntactic correspondence.

---

## Textbooks and Modern Treatments

**Benjamin C. Pierce.** *Types and Programming Languages.* MIT Press, 2002.
The standard graduate textbook for type theory in computer science. Part III covers STLC in full detail, with clean proofs of type safety, decidability, and the substitution lemma. Treats the Curry-Howard correspondence in Chapter 9. Appropriate for readers with programming background; prerequisites are light.

**Simon Thompson.** *Type Theory and Functional Programming.* Addison-Wesley, 1991. (Available free online.)
A pedagogically careful introduction to Martin-Löf type theory and the Curry-Howard correspondence for functional programmers. Develops the translation between propositions and types with many worked examples in a Pascal-like pseudocode. More accessible than Martin-Löf's original notes.

**Rob Nederpelt and Herman Geuvers.** *Type Theory and Formal Proof: An Introduction.* Cambridge University Press, 2014.
A modern undergraduate/graduate introduction covering the lambda cube — from STLC through System F and System $F_\omega$ to the Calculus of Constructions — with careful attention to the logical interpretation of each system.

**The Univalent Foundations Program.** *Homotopy Type Theory: Univalent Foundations of Mathematics.* Institute for Advanced Study, 2013. (Available free at homotopytypetheory.org)
Chapter 1 develops dependent type theory from the propositions-as-types perspective, and the introduction gives an excellent overview of the Curry-Howard philosophy as it extends to HoTT.

**Bengt Nordström, Kent Petersson, and Jan M. Smith.** *Programming in Martin-Löf's Type Theory: An Introduction.* Oxford University Press, 1990. (Available free online.)
Develops the full Curry-Howard correspondence in the Martin-Löf type theory setting, with programming examples in a type-theoretic functional language. Excellent for seeing how $\Pi$ and $\Sigma$ types correspond to universal and existential quantification.

---

## Online Resources and Lecture Notes

**Philip Wadler.** "Propositions as Types" (lecture video and slides). POPL 2015, and various conference talks. Available at homepages.inf.ed.ac.uk/wadler/
Wadler's entertaining and lucid talk on the history of the Curry-Howard correspondence, suitable as a first introduction. The slides alone are highly informative and include historical photos and context.

**Frank Pfenning.** "Lecture Notes on Proof Theory." Carnegie Mellon University. Available at www.cs.cmu.edu/~fp/courses/15317/
Comprehensive lecture notes on proof theory, natural deduction, and the Curry-Howard correspondence from one of the leading researchers in proof theory and type theory.

**Andrej Bauer.** "Propositions as Types" lecture series and blog posts at math.andrej.com
Bauer's accessible explanations of the correspondence, with attention to the mathematical structures (category theory, realizability) underlying it.

**Robert Harper.** *Practical Foundations of Mathematics and Computer Science* (draft). Available at www.cs.cmu.edu/~rwh/pfp/
Harper's comprehensive treatment of programming languages from a type-theoretic foundation, covering the Curry-Howard correspondence thoroughly with an eye toward proof assistants.

**Lean 4 Theorem Proving.** *Theorem Proving in Lean 4.* leanprover.github.io/theorem_proving_in_lean4/
The official introduction to Lean 4, which is structured entirely around the Curry-Howard correspondence — proofs are literally terms of the appropriate type. Chapter 2 ("Dependent Type Theory") and Chapter 3 ("Propositions and Proofs") are directly relevant.

---

## Historical Context

The Curry-Howard correspondence was not discovered in a single moment but emerged gradually across three decades, connecting research traditions that were initially unaware of each other. Haskell Curry's 1934 observation — made while studying the axioms of combinatory logic — predates Gentzen's natural deduction by a year. Curry noticed a pattern without the full context to explain it. When Gentzen introduced natural deduction in 1935 and Church published the typed lambda calculus in 1940, the essential pieces were in place, but no one had yet drawn the connection explicitly. Howard's 1969 manuscript was the critical synthesis: he wrote down the precise translation table, showing that every natural deduction proof corresponds to a typed lambda term, that proof normalization (cutting detours from proofs) corresponds exactly to beta reduction (substituting arguments into function bodies), and that the structural rules of logic correspond to structural operations on terms.

The broader appreciation of the correspondence grew slowly in the 1970s and 1980s. Girard's 1972 thesis, and independently Reynolds's 1974 paper, showed that the correspondence extended to System F and second-order logic. Lambek's categorical work in the late 1960s revealed the third corner of the "trinity": cartesian closed categories, typed lambda calculi, and intuitionistic logic are three presentations of the same mathematical structure. Martin-Löf's work in the 1970s and 1980s extended the correspondence to predicate logic via dependent types, making it possible to express full mathematical proofs within the type-theoretic framework. Philip Wadler's 2015 CACM article, "Propositions as Types," brought the story to a wide audience and coined the phrase "holy trinity" for the three-way correspondence. Today, every major proof assistant — Lean, Coq, Agda, Isabelle/HOL — is built on the correspondence, and it forms the conceptual backbone of formal verification in both industry and academia.
